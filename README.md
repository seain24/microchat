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
sea-orm-cli generate entity -u mysql://root:123456@127.0.0.1:3306/mircochat -o src/data/entity/mircochat
```
