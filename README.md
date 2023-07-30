# Hello Axum 

## 项目介绍
这是一个axum web模版工程，方便用户快速在axum上快速进行web开发，为了帮助java、go开发者找到web开发的感觉。因此整个项目的划分结构和传统的项目结构保持一一致。

## 项目目录介绍
controller层主要放置处理器handler。路由部分暂时统一定义在main.rs中。

middleware层主要定义中间件，比如auth认证、session、链路跟踪等，对应spring中的aop面向切面编程。

service层，这层主要处理服务，当然项目较小，可以直接controller到底。

database层对应的是dao、mapper，这个目录下主要放对db的增删改查操作。

state文件主要定义项目所需上下文，例如redis链接、db链接等。





## 项目技术栈

- axum
- sqlx
- swagger-ui
- postgres
- redis
- 待完善
