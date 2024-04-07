// SPDX-License-Identifier: GPL-2.0

//! Rust completion device
use core::cell::UnsafeCell;
use core::ops::Deref;

use kernel::prelude::*;
use kernel::sync::Mutex;
use kernel::task::Task;
use kernel::{bindings, chrdev, file};

module! {
    type: CompletionDev,
    name: "completion",
    author: "allenli178",
    description: "r4l_experiment completion device",
    license: "GPL",
}

struct Completion(UnsafeCell<bindings::completion>);
unsafe impl Send for Completion {}

impl Deref for Completion {
    type Target = UnsafeCell<bindings::completion>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

static COMPLETION: Mutex<Option<Pin<Box<Completion>>>> = unsafe { Mutex::new(None) };

const GLOBALMEM_SIZE: usize = 0x1000;

static GLOBALMEM_BUF: Mutex<[u8; GLOBALMEM_SIZE]> = unsafe { Mutex::new([0u8; GLOBALMEM_SIZE]) };

struct CompletionFile {
    #[allow(dead_code)]
    inner: &'static Mutex<[u8; GLOBALMEM_SIZE]>,
}

#[vtable]
impl file::Operations for CompletionFile {
    type Data = Box<Self>;

    fn open(_shared: &(), _file: &file::File) -> Result<Box<Self>> {
        pr_info!("open is invoked\n");
        Ok(Box::try_new(Self {
            inner: &GLOBALMEM_BUF,
        })?)
    }

    fn write(
        this: &Self,
        _file: &file::File,
        reader: &mut impl kernel::io_buffer::IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("write is invoked\n");
        pr_info!("process {} awakening the readers\n", Task::current().pid());

        let mut len = reader.len();
        let buffer = &mut *this.inner.lock();
        if len > GLOBALMEM_SIZE {
            len = GLOBALMEM_SIZE;
        }
        pr_info!("data.len is {}\n", len);
        reader.read_slice(&mut buffer[..len])?;

        let lock = COMPLETION.lock();
        let completion = lock.deref().as_ref().unwrap().get();
        drop(lock);
        unsafe { bindings::complete(completion) };
        Ok(len)
    }

    fn read(
        this: &Self,
        _file: &file::File,
        writer: &mut impl kernel::io_buffer::IoBufferWriter,
        offset: u64,
    ) -> Result<usize> {
        pr_info!("read is invoked\n");
        if writer.is_empty() || offset as usize > GLOBALMEM_SIZE {
            return Ok(0);
        }
        pr_info!("process {} is going to sleep\n", Task::current().pid());
        let lock = COMPLETION.lock();
        let completion = lock.deref().as_ref().unwrap().get();
        drop(lock);
        unsafe { bindings::wait_for_completion(completion) };

        let buffer = &*this.inner.lock();
        let data = &buffer[offset as usize..];
        writer.write_slice(data)?;
        Ok(data.len())
    }
}

struct CompletionDev {
    _dev: Pin<Box<chrdev::Registration<2>>>,
}

impl kernel::Module for CompletionDev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("completion device (init)\n");
        let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?;

        let mut completion = COMPLETION.lock();
        let inner = Pin::new(Box::try_new(Completion(UnsafeCell::new(
            bindings::completion::default(),
        )))?);
        unsafe { bindings::init_completion(inner.get()) };
        *completion = Some(inner);

        chrdev_reg.as_mut().register::<CompletionFile>()?;
        Ok(Self { _dev: chrdev_reg })
    }
}

impl Drop for CompletionDev {
    fn drop(&mut self) {
        pr_info!("completion device (exit)\n");
    }
}
