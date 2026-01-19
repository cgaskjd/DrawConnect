# DrawConnect Backend API

## 安装依赖

```bash
cd backend
npm install
```

## 配置环境变量

复制 `.env.example` 为 `.env` 并修改配置:

```bash
cp .env.example .env
```

## 启动MongoDB

确保MongoDB已安装并运行:

```bash
mongod
```

## 启动服务器

开发模式(自动重启):
```bash
npm run dev
```

生产模式:
```bash
npm start
```

## API端点

### 认证相关

- `POST /auth/register` - 用户注册
- `POST /auth/login` - 用户登录

### 云端存储

- `POST /cloud/upload` - 上传作品
- `GET /cloud/artworks/:userId` - 获取用户作品列表
- `GET /cloud/artwork/:id` - 获取单个作品
- `DELETE /cloud/artwork/:id` - 删除作品

### 健康检查

- `GET /health` - 服务器状态检查