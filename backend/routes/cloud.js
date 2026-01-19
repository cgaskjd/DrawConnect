const express = require('express');
const router = express.Router();
const multer = require('multer');
const path = require('path');
const fs = require('fs');
const Artwork = require('../models/Artwork');
const jwt = require('jsonwebtoken');

// Ensure uploads directory exists
const uploadsDir = path.join(__dirname, '../uploads');
if (!fs.existsSync(uploadsDir)) {
    fs.mkdirSync(uploadsDir, { recursive: true });
}

// Configure multer for file uploads
const storage = multer.diskStorage({
    destination: (req, file, cb) => {
        cb(null, uploadsDir);
    },
    filename: (req, file, cb) => {
        const uniqueSuffix = Date.now() + '-' + Math.round(Math.random() * 1E9);
        cb(null, uniqueSuffix + path.extname(file.originalname));
    }
});

const upload = multer({ storage });

// Middleware to verify JWT token
const authenticateToken = (req, res, next) => {
    const authHeader = req.headers['authorization'];
    const token = authHeader && authHeader.split(' ')[1];

    if (!token) {
        return res.status(401).json({
            success: false,
            message: '未提供认证令牌'
        });
    }

    jwt.verify(token, process.env.JWT_SECRET || 'default_secret', (err, user) => {
        if (err) {
            return res.status(403).json({
                success: false,
                message: '无效的认证令牌'
            });
        }
        req.user = user;
        next();
    });
};

// Upload artwork
router.post('/upload', upload.single('file'), async (req, res) => {
    try {
        const { userId, title } = req.body;
        const file = req.file;

        if (!file) {
            return res.status(400).json({
                success: false,
                message: '未上传文件'
            });
        }

        // Create artwork record
        const artwork = new Artwork({
            userId,
            title: title || '未命名作品',
            dataUrl: `/uploads/${file.filename}`,
            thumbnailUrl: `/uploads/${file.filename}`,
            width: 1080,
            height: 1920
        });

        await artwork.save();

        res.json({
            success: true,
            message: '上传成功',
            artwork: {
                id: artwork._id,
                userId: artwork.userId,
                title: artwork.title,
                thumbnailUrl: artwork.thumbnailUrl,
                dataUrl: artwork.dataUrl,
                width: artwork.width,
                height: artwork.height,
                createdAt: artwork.createdAt.getTime(),
                updatedAt: artwork.updatedAt.getTime()
            }
        });
    } catch (error) {
        console.error('Upload error:', error);
        res.status(500).json({
            success: false,
            message: '上传失败',
            error: error.message
        });
    }
});

// Get artworks by user
router.get('/artworks/:userId', async (req, res) => {
    try {
        const { userId } = req.params;
        const artworks = await Artwork.find({ userId }).sort({ updatedAt: -1 });

        res.json({
            success: true,
            artworks: artworks.map(artwork => ({
                id: artwork._id,
                userId: artwork.userId,
                title: artwork.title,
                thumbnailUrl: artwork.thumbnailUrl,
                dataUrl: artwork.dataUrl,
                width: artwork.width,
                height: artwork.height,
                createdAt: artwork.createdAt.getTime(),
                updatedAt: artwork.updatedAt.getTime()
            }))
        });
    } catch (error) {
        console.error('Get artworks error:', error);
        res.status(500).json({
            success: false,
            message: '获取作品列表失败',
            error: error.message
        });
    }
});

// Get single artwork
router.get('/artwork/:id', async (req, res) => {
    try {
        const { id } = req.params;
        const artwork = await Artwork.findById(id);

        if (!artwork) {
            return res.status(404).json({
                success: false,
                message: '作品不存在'
            });
        }

        res.json({
            success: true,
            artwork: {
                id: artwork._id,
                userId: artwork.userId,
                title: artwork.title,
                thumbnailUrl: artwork.thumbnailUrl,
                dataUrl: artwork.dataUrl,
                width: artwork.width,
                height: artwork.height,
                createdAt: artwork.createdAt.getTime(),
                updatedAt: artwork.updatedAt.getTime()
            }
        });
    } catch (error) {
        console.error('Get artwork error:', error);
        res.status(500).json({
            success: false,
            message: '获取作品失败',
            error: error.message
        });
    }
});

// Delete artwork
router.delete('/artwork/:id', async (req, res) => {
    try {
        const { id } = req.params;
        const artwork = await Artwork.findById(id);

        if (!artwork) {
            return res.status(404).json({
                success: false,
                message: '作品不存在'
            });
        }

        // Delete file from filesystem
        const filePath = path.join(__dirname, '..', artwork.dataUrl);
        if (fs.existsSync(filePath)) {
            fs.unlinkSync(filePath);
        }

        await Artwork.findByIdAndDelete(id);

        res.json({
            success: true,
            message: '删除成功'
        });
    } catch (error) {
        console.error('Delete artwork error:', error);
        res.status(500).json({
            success: false,
            message: '删除失败',
            error: error.message
        });
    }
});

module.exports = router;