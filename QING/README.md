# 轻画 - 智能云端绘画应用

## 项目概述

轻画是一款面向全平台用户的专业级绘画软件,采用**三端互通**架构（Android、iOS、Web），集成云端存储、社交分享和AI辅助功能,为用户提供随时随地的跨平台创作体验。

## 第一阶段 MVP 功能

本项目已完成第一阶段(MVP版本)的开发,包含以下核心功能:

### ✅ 已实现功能

#### 1. 基础画布系统
- 单图层绘图引擎
- 基于 Jetpack Compose Canvas 的高性能绘图
- 支持触摸手势绘画
- 撤销/重做功能

#### 2. 五种基础绘画工具
- **画笔工具**: 支持自定义颜色和笔刷大小
- **橡皮擦工具**: 可调节橡皮擦大小
- **填充工具**: 颜色填充(UI已实现,逻辑待完善)
- **选择工具**: 选区功能(UI已实现,逻辑待完善)
- **移动工具**: 移动画布内容(UI已实现,逻辑待完善)

#### 3. 基础色彩选择器
- 36种预设颜色
- 直观的颜色选择界面
- 当前颜色实时显示

#### 4. 本地保存/加载
- 基于 Room 数据库的本地存储
- JSON 格式保存绘画数据
- 支持作品标题命名
- 完整的路径和点数据保存

#### 5. 用户注册/登录
- 邮箱注册登录系统
- JWT Token 认证
- 离线模式支持
- 用户信息管理

#### 6. 基础云端备份
- RESTful API 后端服务
- 作品上传到云端
- 用户作品列表管理
- MongoDB 数据存储

## 技术架构

### 三端互通架构

轻画采用统一后端 + 多端客户端的架构，支持：
- **Android 客户端**: 原生 Kotlin 开发
- **iOS 客户端**: Swift/SwiftUI 开发
- **Web 客户端**: React/Vue 前端应用

### Android 客户端
- **开发语言**: Kotlin
- **UI框架**: Jetpack Compose
- **架构模式**: MVVM + Clean Architecture
- **依赖注入**: Hilt
- **数据库**: Room + SQLite
- **网络请求**: Retrofit + OkHttp
- **响应式编程**: Kotlin Coroutines + Flow

### iOS 客户端 (规划中)
- **开发语言**: Swift
- **UI框架**: SwiftUI
- **架构模式**: MVVM + Clean Architecture
- **数据库**: Core Data / SQLite
- **网络请求**: URLSession / Alamofire
- **响应式编程**: Combine

### Web 客户端 (规划中)
- **前端框架**: React / Vue.js
- **绘图引擎**: Canvas API / WebGL
- **状态管理**: Redux / Vuex
- **网络请求**: Axios
- **UI组件库**: Ant Design / Element UI

### 服务端 (Backend)
- **后端框架**: Node.js + Express
- **数据库**: MongoDB
- **认证**: JWT (JSON Web Tokens)
- **文件上传**: Multer
- **密码加密**: bcryptjs

## 项目结构

```
轻画/
├── app/                                    # Android 应用
│   ├── src/main/
│   │   ├── java/com/qinghua/              # 轻画 Android 代码
│   │   │   ├── data/                      # 数据层
│   │   │   │   ├── local/                 # 本地数据库
│   │   │   │   │   ├── dao/              # DAO 接口
│   │   │   │   │   ├── entity/           # 数据库实体
│   │   │   │   │   └── QinghuaDatabase.kt
│   │   │   │   ├── remote/                # 远程API
│   │   │   │   │   ├── api/              # API 接口定义
│   │   │   │   │   └── dto/              # 数据传输对象
│   │   │   │   └── repository/            # 仓库层
│   │   │   ├── domain/                    # 领域层
│   │   │   │   └── model/                # 领域模型
│   │   │   ├── ui/                        # UI层
│   │   │   │   ├── auth/                 # 认证界面
│   │   │   │   ├── canvas/               # 绘图界面
│   │   │   │   ├── navigation/           # 导航
│   │   │   │   └── theme/                # 主题
│   │   │   ├── di/                        # 依赖注入
│   │   │   ├── QinghuaApplication.kt
│   │   │   └── MainActivity.kt
│   │   ├── res/                           # 资源文件
│   │   └── AndroidManifest.xml
│   └── build.gradle.kts
├── ios/                                    # iOS 应用 (规划中)
│   ├── QingHua/                           # iOS 源代码
│   └── QingHua.xcodeproj
├── web/                                    # Web 应用 (规划中)
│   ├── src/
│   ├── public/
│   └── package.json
├── backend/                                # 后端服务 (三端共用)
│   ├── models/                            # 数据模型
│   │   ├── User.js
│   │   └── Artwork.js
│   ├── routes/                            # 路由
│   │   ├── auth.js
│   │   └── cloud.js
│   ├── uploads/                           # 上传文件目录
│   ├── server.js                          # 服务器入口
│   ├── package.json
│   └── .env.example
├── build.gradle.kts
├── settings.gradle.kts
└── README.md
```

## 快速开始

### 前置要求

- Android Studio (最新版本)
- JDK 17+
- Node.js 16+
- MongoDB 4.4+

### 安装步骤

#### 1. 克隆项目

```bash
git clone <repository-url>
cd QingHua
```

#### 2. 启动后端服务

```bash
cd backend
npm install
cp .env.example .env
# 编辑 .env 文件配置数据库连接
npm run dev
```

后端服务将在 `http://localhost:3000` 运行

#### 3. 配置Android应用

在 `app/src/main/java/com/qinghua/di/NetworkModule.kt` 中修改 BASE_URL:

```kotlin
private const val BASE_URL = "http://10.0.2.2:3000/" // Android模拟器
// 或
private const val BASE_URL = "http://your-ip:3000/" // 真机测试
```

#### 4. 运行Android应用

1. 在 Android Studio 中打开项目
2. 等待 Gradle 同步完成
3. 连接设备或启动模拟器
4. 点击 Run 按钮

## API 文档

### 认证接口

#### 注册
```
POST /auth/register
Content-Type: application/json

{
  "username": "用户名",
  "email": "email@example.com",
  "password": "密码"
}
```

#### 登录
```
POST /auth/login
Content-Type: application/json

{
  "email": "email@example.com",
  "password": "密码"
}
```

### 云端存储接口

#### 上传作品
```
POST /cloud/upload
Content-Type: multipart/form-data

file: <文件>
userId: <用户ID>
title: <作品标题>
```

#### 获取用户作品列表
```
GET /cloud/artworks/:userId
```

#### 获取单个作品
```
GET /cloud/artwork/:id
```

#### 删除作品
```
DELETE /cloud/artwork/:id
```

## 核心功能说明

### 绘图引擎

绘图引擎基于 Jetpack Compose 的 Canvas API 实现,支持:
- 实时路径绘制
- 平滑的笔触效果
- 压感支持(预留接口)
- 高性能渲染

### 数据持久化

- **本地存储**: 使用 Room 数据库存储作品元数据,绘图数据以 JSON 格式保存到文件系统
- **云端同步**: 通过 RESTful API 上传作品到服务器,支持跨设备访问

### 状态管理

使用 Kotlin Flow 和 StateFlow 实现响应式状态管理:
- `CanvasState`: 画布状态(路径、背景色等)
- `DrawingSettings`: 绘图设置(工具、颜色、笔刷大小)
- `AuthState`: 认证状态
- `SaveState`: 保存状态

## 后续开发计划

### 第二阶段: 三端互通基础
- iOS 客户端开发（基础绘画功能）
- Web 客户端开发（基础绘画功能）
- 统一 API 接口规范
- 跨平台数据同步机制
- 账号系统三端互通

### 第三阶段: 完整绘画功能
- 多图层系统(最多20层)
- 完整工具集(15+种工具)
- 笔刷系统(50+预设笔刷)
- 图层混合模式
- 导入导出(PNG/JPG)

### 第四阶段: 社区与协作
- 作品发布与展示
- 社区互动功能
- 素材市场
- 协作绘画

### 第五阶段: AI与高级功能
- AI辅助功能
- 实时协作优化
- PSD格式支持
- 高级笔刷系统

## 已知问题

1. 填充工具、选择工具、移动工具的核心逻辑尚未实现
2. 云端备份功能需要配置实际的服务器地址
3. 缺少图片导出功能
4. 缺少作品缩略图生成
5. iOS 和 Web 客户端尚在规划中

## 贡献指南

欢迎提交 Issue 和 Pull Request!

## 许可证

[待定]

## 联系方式

[待定]

---

**项目名称**: 轻画 (QingHua)
**版本**: v1.0.0 (MVP)
**架构**: 三端互通 (Android / iOS / Web)
**最后更新**: 2026-01-15