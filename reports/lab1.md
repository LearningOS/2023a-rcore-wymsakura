# 实验：
rcore的栈不符合elf文件规范 顺序不一样并且规范多了一个argc的个数
![Alt text](image.png)
![Alt text](image-1.png)
按顺序申请空间，先将argv的数据存到argv_addr,再赋值，最后再申请空间，写入argc的个数
![Alt text](image-2.png)
# 问答：
elf文件删除 ELF header 符号得到纯二进制的镜像就是bin文件

