<a name="Z8yAt"></a>
# 作业一
![image.png](https://cdn.nlark.com/yuque/0/2024/png/21805625/1711937751368-18092a2b-7ea8-4220-9ef5-15fb2db6f3e3.png#averageHue=%23d6d6d6&clientId=u412c1e19-1a01-4&from=paste&height=149&id=u77252fff&originHeight=298&originWidth=1530&originalType=binary&ratio=2&rotation=0&showTitle=false&size=69529&status=done&style=none&taskId=u5e395d42-da26-453c-9333-9d920ada849&title=&width=765)

1. 安装1.62.0版本的rust，修改rustup源和cargo源（这里使用的rsproxy源）
2. 进入linux目录进行config文件修改和make编译，最后生成一个vmlinux文件

![image.png](https://cdn.nlark.com/yuque/0/2024/png/21805625/1711939298845-1294ea49-d495-433d-9366-d8913d6f3a23.png#averageHue=%23282828&clientId=u412c1e19-1a01-4&from=paste&height=256&id=u0b5ecfb4&originHeight=512&originWidth=1704&originalType=binary&ratio=2&rotation=0&showTitle=false&size=206174&status=done&style=none&taskId=u6c3a8a4b-50fa-4212-9ae4-b152107abd7&title=&width=852)
<a name="DMVwm"></a>
# 作业二

- Q1：在Kuild的 obj-m := r4l_e1000_demo.o 使用-M参数，决定编译成内核模块。
- Q2：在Makefile中的KDIR ?= ../linux实现与编译内核的目录一致。

![image.png](https://cdn.nlark.com/yuque/0/2024/png/21805625/1711939445375-834c38ec-9eac-49e6-8ee1-f603ad066f6d.png#averageHue=%23cfcfcf&clientId=u412c1e19-1a01-4&from=paste&height=108&id=u10e3ca51&originHeight=216&originWidth=1194&originalType=binary&ratio=2&rotation=0&showTitle=false&size=76008&status=done&style=none&taskId=u0f04dbf5-fd47-44d6-975d-be3b57fae7f&title=&width=597)<br />![截屏2024-03-28 17.58.27.png](https://cdn.nlark.com/yuque/0/2024/png/21805625/1711687127825-434235af-ac90-4bfc-92b7-9ad35f9683c9.png#averageHue=%23313131&clientId=ua2d287dc-12fe-4&from=drop&id=uca10730a&originHeight=998&originWidth=1666&originalType=binary&ratio=2&rotation=0&showTitle=false&size=425359&status=done&style=none&taskId=u89dc11f5-852d-450e-930c-7a5e4531964&title=)![截屏2024-03-28 17.58.07.png](https://cdn.nlark.com/yuque/0/2024/png/21805625/1711687127377-dbb2ce0e-3dc0-4f61-a32a-2528e477893f.png#averageHue=%23292929&clientId=ua2d287dc-12fe-4&from=drop&id=uce355b8d&originHeight=374&originWidth=1778&originalType=binary&ratio=2&rotation=0&showTitle=false&size=130897&status=done&style=none&taskId=uac971ad0-a210-44aa-8e56-31fcc43470e&title=)
<a name="KIyJ0"></a>
# 作业三

1. 修改配置文件

![image.png](https://cdn.nlark.com/yuque/0/2024/png/21805625/1711940649142-846457ef-e32d-4939-9ee3-432607f4b00c.png#averageHue=%23242423&clientId=u412c1e19-1a01-4&from=paste&height=128&id=u1d0f4163&originHeight=256&originWidth=1626&originalType=binary&ratio=2&rotation=0&showTitle=false&size=100578&status=done&style=none&taskId=ua56a1697-ca50-4ae4-8721-72222b5606c&title=&width=813)<br />![image.png](https://cdn.nlark.com/yuque/0/2024/png/21805625/1711939954498-ee091aae-5fda-4604-90c3-9322119dc701.png#averageHue=%23272525&clientId=u412c1e19-1a01-4&from=paste&height=232&id=u7d81e50d&originHeight=464&originWidth=1446&originalType=binary&ratio=2&rotation=0&showTitle=false&size=93049&status=done&style=none&taskId=uad7c83e4-47e9-4613-b377-ccc01b1f439&title=&width=723)

2. 添加模块并编译

![image.png](https://cdn.nlark.com/yuque/0/2024/png/21805625/1711940150418-90e6cbb4-fba4-4e3e-9516-803990b89d16.png#averageHue=%23d5d5d5&clientId=u412c1e19-1a01-4&from=paste&height=257&id=u892484f1&originHeight=514&originWidth=662&originalType=binary&ratio=2&rotation=0&showTitle=false&size=83164&status=done&style=none&taskId=ue1961882-0341-4a57-a2b5-ca73a76f2d8&title=&width=331)<br />![image.png](https://cdn.nlark.com/yuque/0/2024/png/21805625/1711941044968-c10be71b-c00e-491b-ad1c-59df8096c6f1.png#averageHue=%231b1b1a&clientId=u412c1e19-1a01-4&from=paste&height=253&id=ubad67851&originHeight=506&originWidth=824&originalType=binary&ratio=2&rotation=0&showTitle=false&size=35628&status=done&style=none&taskId=u507a79f7-dfe8-48b0-b34c-601c099fd53&title=&width=412)<br />![image.png](https://cdn.nlark.com/yuque/0/2024/png/21805625/1711949447977-b9925b40-e865-4643-9d35-92938101ef12.png#averageHue=%23252424&clientId=uf510350a-9b55-4&from=paste&height=118&id=u240bf0b6&originHeight=236&originWidth=878&originalType=binary&ratio=1&rotation=0&showTitle=false&size=46347&status=done&style=none&taskId=u037f59fe-7383-414a-8840-4e38e2670b3&title=&width=439)
<a name="tqVNC"></a>
# 作业四

- 在remove中对pci的设备进行释放（在pci.rs中添加释放函数）
- 在stop中添加对irq的释放

![image.png](https://cdn.nlark.com/yuque/0/2024/png/21805625/1712199646679-f5c44381-8268-4e49-816d-95ec647e4914.png#averageHue=%232c2c2c&clientId=u7c974338-848e-4&from=paste&height=240&id=u8a6d766b&originHeight=240&originWidth=636&originalType=binary&ratio=1&rotation=0&showTitle=false&size=46868&status=done&style=none&taskId=u477141e7-6010-405c-971e-5d7b9b31ea9&title=&width=636)
<a name="SIu7S"></a>
# 作业五
![image.png](https://cdn.nlark.com/yuque/0/2024/png/21805625/1711967197845-4a2097b8-c78d-4b44-b8ef-6f0155bbc875.png#averageHue=%23252525&clientId=uf510350a-9b55-4&from=paste&height=77&id=u2ad4b564&originHeight=154&originWidth=694&originalType=binary&ratio=1&rotation=0&showTitle=false&size=20723&status=done&style=none&taskId=ueb898f0b-9e26-4665-809d-9749c591c16&title=&width=347)<br />Q1：build_image.sh中有`echo "mknod /dev/cicv c 248 0" >> etc/init.d/rcS`，所以设备号码是：248
<a name="eEXql"></a>
# 项目小实验

1. 添加配置文件和启动脚本，验证 telnet server（把之前卸载的网卡重新加上，然后编译内核）
2. 查看c文件结构，并用rust实现。

本地terminal<br />![image.png](https://cdn.nlark.com/yuque/0/2024/png/21805625/1712543159474-5387ee68-4244-417e-a9c5-1dd7f0b0f142.png#averageHue=%23242424&clientId=u39b035cf-e03b-4&from=paste&height=114&id=u0be14815&originHeight=228&originWidth=1428&originalType=binary&ratio=2&rotation=0&showTitle=false&size=54404&status=done&style=none&taskId=uc7821ae9-7ccb-4d8e-8c19-1d1c9c16775&title=&width=714)<br />qemu terminal<br />![image.png](https://cdn.nlark.com/yuque/0/2024/png/21805625/1712543170415-74bfcc15-12c0-4f62-b4f1-1bbfc8831045.png#averageHue=%232e2e2e&clientId=u39b035cf-e03b-4&from=paste&height=190&id=u417046b0&originHeight=380&originWidth=994&originalType=binary&ratio=2&rotation=0&showTitle=false&size=99819&status=done&style=none&taskId=uc0368a0a-8ebb-4a8e-bf27-d04878b68d9&title=&width=497)
