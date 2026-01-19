const express = require('express');
const cors = require('cors');
const mongoose = require('mongoose');
require('dotenv').config();

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware - 使用 express 内置解析器，移除 body-parser 依赖
app.use(cors());
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// 路由懒加载 - 提升启动速度
let authRoutes, cloudRoutes, pluginRoutes;
const loadRoutes = () => {
    if (!authRoutes) authRoutes = require('./routes/auth');
    if (!cloudRoutes) cloudRoutes = require('./routes/cloud');
    if (!pluginRoutes) pluginRoutes = require('./routes/plugins');
};

// 延迟路由注册
app.use('/auth', (req, res, next) => { loadRoutes(); authRoutes(req, res, next); });
app.use('/cloud', (req, res, next) => { loadRoutes(); cloudRoutes(req, res, next); });
app.use('/plugins', (req, res, next) => { loadRoutes(); pluginRoutes(req, res, next); });

// Serve static files for plugin uploads
app.use('/uploads/plugins', express.static('uploads/plugins'));

// MongoDB 异步连接 - 不阻塞服务器启动
mongoose.connect(process.env.MONGODB_URI || 'mongodb://localhost:27017/drawconnect', {
    maxPoolSize: 10,
    serverSelectionTimeoutMS: 5000,
    socketTimeoutMS: 45000,
})
.then(() => console.log('MongoDB connected successfully'))
.catch(err => console.error('MongoDB connection error:', err));

// Health check endpoint
app.get('/health', (req, res) => {
    res.json({ status: 'ok', message: 'DrawConnect API is running' });
});

// Error handling middleware
app.use((err, req, res, next) => {
    console.error(err.stack);
    res.status(500).json({
        success: false,
        message: 'Internal server error',
        error: err.message
    });
});

app.listen(PORT, () => {
    console.log(`DrawConnect API server is running on port ${PORT}`);
});