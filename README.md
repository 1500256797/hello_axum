# Hello Axum

## 项目介绍

这是一个 axum web 模版工程，方便用户快速在 axum 上快速进行 web 开发，为了帮助 java、go 开发者找到 web 开发的感觉。因此整个项目的划分结构和传统的项目结构保持一一致。

## 项目目录介绍

controller 层主要放置处理器 handler。路由部分暂时统一定义在 main.rs 中。

middleware 层主要定义中间件，比如 auth 认证、session、链路跟踪等，对应 spring 中的 aop 面向切面编程。

service 层，这层主要处理服务，当然项目较小，可以直接 controller 到底。

database 层对应的是 dao、mapper，这个目录下主要放对 db 的增删改查操作。

state 文件主要定义项目所需上下文，例如 redis 链接、db 链接等。

## 项目技术栈

-   axum
-   sqlx
-   swagger-ui
-   postgres
-   redis
-   待完善

## 子模块管理

### 在 git 中新增 A 仓库和 B 仓库

如果想在 B 仓库中，使用 A 仓库的代码，除了直接将 A 仓库的代码下载后复制粘贴到 B 中，还可以直接使用 git 的子模块命令。

使用方式如下

```
git submodule add <A模块的仓库链接>
git submodule add git@github.com:1500256797/foundry_contract_hello.git
```

执行结果如下：

```
~/Documents/rust_hello[master] git submodule add git@github.com:1500256797/foundry_contract_hello.git
Cloning into '/Users/ouhuang/Documents/rust_hello/foundry_contract_hello'...
remote: Enumerating objects: 18, done.
remote: Counting objects: 100% (18/18), done.
remote: Compressing objects: 100% (11/11), done.
remote: Total 18 (delta 1), reused 18 (delta 1), pack-reused 0
Receiving objects: 100% (18/18), done.
Resolving deltas: 100% (1/1), done.
```

这样会在 B 项目中克隆一个 A 项目。

### 在 B 仓库中对 A 仓库的代码进行更新

因为 A 仓库的代码是嵌套在 B 仓库中的，如果 A 仓库中的代码有更新，则提交时，应先提交 A 仓库，然后再提交 B 仓库。拉取的时候也是同理。先让 A 仓库保持最新后，
然后再提交 B 仓库的代码。

## sqlx 创建数据表结构及常用命令

### 1、创建数据库

在 env 文件中

```
 sqlx database create
```

创建本地 sql

```
sqlx migrate add customers
```

迁移 sql 到数据库

```
 sqlx migrate run
```

## docker-compose up -d

docker-compose 如何每次都重新 build 镜像
要求 docker-compose 每次重新构建镜像，可以使用 --build 标志来强制重新构建镜像。你可以在运行 docker-compose up 或 docker-compose build 命令时使用该标志。例如：

```
docker-compose up --build
```

这将使用 docker-compose.yml 文件中指定的构建上下文重新构建所有服务的镜像。如果你只想重新构建某个服务的镜像，可以使用以下命令：

```
docker-compose build --no-cache axum_app

docker-compose build  axum_app
```

在这个命令中，--no-cache 标志指示构建过程中不使用缓存，以便强制重新构建镜像。<service-name> 是要构建镜像的服务名称。

### docker-compose 最佳实践

```
docker-compose build  axum_app && docker-compose up -d
```
