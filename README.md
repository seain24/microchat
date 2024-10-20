## IM系统的架构设计

IM其本质是一套消息发送与投递系统，或者说是一套网络通信系统，归根结底就是两个词：存储与转发。IM系统的主要需求：包括账号、关系链、在线状态显示、消息交互（文本、图片、语音）、实时视频电话
要处理好上述需求，我们通常需要从以下方面进行考量从而设计出合适的架构：

    如果采用可靠传输协议TCP，需要考虑到负载问题：短连接实现账号、关系链相关业务，长连接实现上线、信息推送；
    后台架构的灵活性、可扩展性：支持分布式部署——把网络层、业务逻辑层、数据层分离，网络层和业务层支持负载均衡策略、数据层支持分布式存储；
    客户端SDK的易用性：把网络层、数据层分离、业务逻辑层分离。

### DDD架构

![img.png](img.png)

### 所用技术

#### 1. xxx

#### 2. task_local_extensions

task_local_extensions 是一个 Rust 库，它提供了一种在异步任务之间共享数据的方法。这个库的主要作用是在异步任务中存储和访问线程局部变量（thread-local
variables），这些变量对于每个线程都是唯一的。

使用场景为：

1. 在异步任务中共享配置信息，例如数据库连接池、缓存实例等。
2. 在异步任务中传递上下文信息，例如用户身份、请求ID等。
3. 在异步任务中实现性能监控，例如记录任务执行时间、计数器等。

### 一. 数据库准备

1. 创建mysql容器

```shell
  docker run --name mysql -p 3306:3306 -e MYSQL_ROOT_PASSWORD=123456 -d mysql:latest
  ```

2. 使用navicat连接, 建立一个数据库

```shell
    数据库名：mircochat
    字符集：utf8mb4   
    编码集：utf8mb4_0900_ai_ci
```

3. 安装sea-orm-cli工具

``` shell
cargo install sea-orm-cli=0.12.12
```

4. 生成orm-mapping

```shell
sea-orm-cli generate entity -u mysql://root:123456@127.0.0.1:3306/LeChat -o src/data/entity/LeChat
```

### 二、功能实现

#### 2.1 注册登录

**问题一： 业务系统如何正确实现防重名功能**

[业务系统如何正确实现防重名功能](/home/xingbin/Workspaces/RustProjects/micro-chat/业务系统如何正确实现防重名功能.md)
