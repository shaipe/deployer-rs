# Deployer 应用程序部署器

**实现原理：** 在服务器上搭建一个服务对外提供http接口接口如下：
1. 接收客户端上传的文件
2. 接收客户端发送的指令，此指令可以在服务端转换为shell命令在服务器上进行执行
3. 客户端程序需要执行命令来进行应用的打包

## 应用程序发布基础流程

```mermaid
graph TB
Start(开始) --> cli1(客户应用打包) --> up(应用上传)--> s1(应用接收)--> s2(部署)--> End(结束)
cli1 --> c2(应用程序编译) --> c3(应用打包为zip) -->cli1
s2 --> d1(程序解压更新) --> d2(服务重启) -->s2
```

### 程序框架

```mermaid
graph LR
deployer --> server --> 上传服务
server --> 命令执行服务
deployer --> cli --> 文件上传
cli --> 发送命令
```

### 技术栈

- 开发语言： Rust
- 框架： Actix


### examples

添加
a add core https://www.google.com

打开
a open core

### Server 

服务端,根据类型进行不同的业务处理
action: install, update