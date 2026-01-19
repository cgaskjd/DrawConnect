const express = require('express');
const router = express.Router();
const multer = require('multer');
const path = require('path');
const fs = require('fs');
const jwt = require('jsonwebtoken');
const Plugin = require('../models/Plugin');
const PluginLike = require('../models/PluginLike');

// Ensure uploads/plugins directory exists
const pluginsDir = path.join(__dirname, '../uploads/plugins');
if (!fs.existsSync(pluginsDir)) {
    fs.mkdirSync(pluginsDir, { recursive: true });
}

// Configure multer for plugin file uploads
const storage = multer.diskStorage({
    destination: (req, file, cb) => {
        cb(null, pluginsDir);
    },
    filename: (req, file, cb) => {
        const uniqueSuffix = Date.now() + '-' + Math.round(Math.random() * 1E9);
        cb(null, uniqueSuffix + path.extname(file.originalname));
    }
});

const fileFilter = (req, file, cb) => {
    // Only allow .zip files
    if (file.mimetype === 'application/zip' ||
        file.mimetype === 'application/x-zip-compressed' ||
        path.extname(file.originalname).toLowerCase() === '.zip') {
        cb(null, true);
    } else {
        cb(new Error('Only .zip files are allowed'), false);
    }
};

const upload = multer({
    storage,
    fileFilter,
    limits: {
        fileSize: 50 * 1024 * 1024 // 50MB limit
    }
});

// Middleware to verify JWT token
const authenticateToken = (req, res, next) => {
    const authHeader = req.headers['authorization'];
    const token = authHeader && authHeader.split(' ')[1];

    if (!token) {
        return res.status(401).json({
            success: false,
            message: 'Authentication required'
        });
    }

    jwt.verify(token, process.env.JWT_SECRET || 'default_secret', (err, user) => {
        if (err) {
            return res.status(403).json({
                success: false,
                message: 'Invalid authentication token'
            });
        }
        req.user = user;
        next();
    });
};

// Optional authentication - populates req.user if token is valid, but doesn't require it
const optionalAuth = (req, res, next) => {
    const authHeader = req.headers['authorization'];
    const token = authHeader && authHeader.split(' ')[1];

    if (token) {
        jwt.verify(token, process.env.JWT_SECRET || 'default_secret', (err, user) => {
            if (!err) {
                req.user = user;
            }
            next();
        });
    } else {
        next();
    }
};

// Helper to format plugin response
const formatPlugin = (plugin, userId = null, liked = false) => ({
    id: plugin._id,
    name: plugin.name,
    slug: plugin.slug,
    description: plugin.description,
    shortDescription: plugin.shortDescription,
    version: plugin.version,
    category: plugin.category,
    tags: plugin.tags,
    authorId: plugin.authorId,
    authorName: plugin.authorName,
    fileUrl: plugin.fileUrl,
    fileName: plugin.fileName,
    fileSize: plugin.fileSize,
    thumbnailUrl: plugin.thumbnailUrl,
    screenshotUrls: plugin.screenshotUrls,
    downloadCount: plugin.downloadCount,
    likeCount: plugin.likeCount,
    status: plugin.status,
    liked: liked,
    createdAt: plugin.createdAt,
    updatedAt: plugin.updatedAt
});

// GET /plugins - Get plugin list with pagination, search, and filtering
router.get('/', optionalAuth, async (req, res) => {
    try {
        const {
            page = 1,
            limit = 20,
            search,
            category,
            sort = 'newest',
            status = 'approved'
        } = req.query;

        const pageNum = Math.max(1, parseInt(page));
        const limitNum = Math.min(50, Math.max(1, parseInt(limit)));
        const skip = (pageNum - 1) * limitNum;

        // Build query
        const query = {};

        // Only show approved plugins by default (unless user is querying their own)
        if (status === 'approved') {
            query.status = 'approved';
        }

        // Category filter
        if (category && category !== 'all') {
            query.category = category;
        }

        // Search
        if (search) {
            query.$or = [
                { name: { $regex: search, $options: 'i' } },
                { description: { $regex: search, $options: 'i' } },
                { tags: { $in: [new RegExp(search, 'i')] } }
            ];
        }

        // Sort options
        let sortOption = {};
        switch (sort) {
            case 'newest':
                sortOption = { createdAt: -1 };
                break;
            case 'oldest':
                sortOption = { createdAt: 1 };
                break;
            case 'popular':
                sortOption = { downloadCount: -1 };
                break;
            case 'liked':
                sortOption = { likeCount: -1 };
                break;
            default:
                sortOption = { createdAt: -1 };
        }

        // Execute query
        const [plugins, total] = await Promise.all([
            Plugin.find(query)
                .sort(sortOption)
                .skip(skip)
                .limit(limitNum)
                .populate('authorId', 'username avatarUrl'),
            Plugin.countDocuments(query)
        ]);

        // Get like status if user is authenticated
        let likedPluginIds = new Set();
        if (req.user) {
            const likes = await PluginLike.find({
                userId: req.user.id,
                pluginId: { $in: plugins.map(p => p._id) }
            });
            likedPluginIds = new Set(likes.map(l => l.pluginId.toString()));
        }

        res.json({
            success: true,
            plugins: plugins.map(p => formatPlugin(p, req.user?.id, likedPluginIds.has(p._id.toString()))),
            pagination: {
                page: pageNum,
                limit: limitNum,
                total,
                totalPages: Math.ceil(total / limitNum)
            }
        });
    } catch (error) {
        console.error('Get plugins error:', error);
        res.status(500).json({
            success: false,
            message: 'Failed to fetch plugins',
            error: error.message
        });
    }
});

// GET /plugins/user/me - Get current user's plugins
router.get('/user/me', authenticateToken, async (req, res) => {
    try {
        const plugins = await Plugin.find({ authorId: req.user.id })
            .sort({ createdAt: -1 })
            .populate('authorId', 'username avatarUrl');

        res.json({
            success: true,
            plugins: plugins.map(p => formatPlugin(p, req.user.id, false))
        });
    } catch (error) {
        console.error('Get user plugins error:', error);
        res.status(500).json({
            success: false,
            message: 'Failed to fetch user plugins',
            error: error.message
        });
    }
});

// GET /plugins/:id - Get plugin details
router.get('/:id', optionalAuth, async (req, res) => {
    try {
        const plugin = await Plugin.findById(req.params.id)
            .populate('authorId', 'username avatarUrl');

        if (!plugin) {
            return res.status(404).json({
                success: false,
                message: 'Plugin not found'
            });
        }

        // Check if user has liked this plugin
        let liked = false;
        if (req.user) {
            const like = await PluginLike.findOne({
                pluginId: plugin._id,
                userId: req.user.id
            });
            liked = !!like;
        }

        res.json({
            success: true,
            plugin: formatPlugin(plugin, req.user?.id, liked)
        });
    } catch (error) {
        console.error('Get plugin error:', error);
        res.status(500).json({
            success: false,
            message: 'Failed to fetch plugin',
            error: error.message
        });
    }
});

// POST /plugins - Upload a new plugin
router.post('/', authenticateToken, upload.single('file'), async (req, res) => {
    try {
        const file = req.file;

        if (!file) {
            return res.status(400).json({
                success: false,
                message: 'No plugin file uploaded'
            });
        }

        const {
            name,
            description,
            shortDescription,
            version,
            category,
            tags
        } = req.body;

        if (!name || !description) {
            // Clean up uploaded file
            fs.unlinkSync(file.path);
            return res.status(400).json({
                success: false,
                message: 'Name and description are required'
            });
        }

        // Parse tags if provided as JSON string
        let parsedTags = [];
        if (tags) {
            try {
                parsedTags = typeof tags === 'string' ? JSON.parse(tags) : tags;
            } catch (e) {
                parsedTags = tags.split(',').map(t => t.trim());
            }
        }

        const plugin = new Plugin({
            name,
            description,
            shortDescription: shortDescription || description.substring(0, 200),
            version: version || '1.0.0',
            category: category || 'other',
            tags: parsedTags,
            authorId: req.user.id,
            fileUrl: `/uploads/plugins/${file.filename}`,
            fileName: file.originalname,
            fileSize: file.size,
            status: 'pending'
        });

        await plugin.save();

        res.status(201).json({
            success: true,
            message: 'Plugin uploaded successfully. It will be reviewed before publishing.',
            plugin: formatPlugin(plugin, req.user.id, false)
        });
    } catch (error) {
        console.error('Upload plugin error:', error);
        // Clean up uploaded file on error
        if (req.file) {
            try {
                fs.unlinkSync(req.file.path);
            } catch (e) {}
        }
        res.status(500).json({
            success: false,
            message: 'Failed to upload plugin',
            error: error.message
        });
    }
});

// PUT /plugins/:id - Update plugin (author only)
router.put('/:id', authenticateToken, async (req, res) => {
    try {
        const plugin = await Plugin.findById(req.params.id);

        if (!plugin) {
            return res.status(404).json({
                success: false,
                message: 'Plugin not found'
            });
        }

        if (plugin.authorId.toString() !== req.user.id) {
            return res.status(403).json({
                success: false,
                message: 'You can only update your own plugins'
            });
        }

        const {
            name,
            description,
            shortDescription,
            version,
            category,
            tags
        } = req.body;

        // Update fields
        if (name) plugin.name = name;
        if (description) plugin.description = description;
        if (shortDescription) plugin.shortDescription = shortDescription;
        if (version) plugin.version = version;
        if (category) plugin.category = category;
        if (tags) {
            try {
                plugin.tags = typeof tags === 'string' ? JSON.parse(tags) : tags;
            } catch (e) {
                plugin.tags = tags.split(',').map(t => t.trim());
            }
        }

        // If plugin was rejected and user updates, set back to pending
        if (plugin.status === 'rejected') {
            plugin.status = 'pending';
            plugin.rejectReason = null;
        }

        await plugin.save();

        res.json({
            success: true,
            message: 'Plugin updated successfully',
            plugin: formatPlugin(plugin, req.user.id, false)
        });
    } catch (error) {
        console.error('Update plugin error:', error);
        res.status(500).json({
            success: false,
            message: 'Failed to update plugin',
            error: error.message
        });
    }
});

// DELETE /plugins/:id - Delete plugin (author only)
router.delete('/:id', authenticateToken, async (req, res) => {
    try {
        const plugin = await Plugin.findById(req.params.id);

        if (!plugin) {
            return res.status(404).json({
                success: false,
                message: 'Plugin not found'
            });
        }

        if (plugin.authorId.toString() !== req.user.id) {
            return res.status(403).json({
                success: false,
                message: 'You can only delete your own plugins'
            });
        }

        // Delete the plugin file
        const filePath = path.join(__dirname, '..', plugin.fileUrl);
        if (fs.existsSync(filePath)) {
            fs.unlinkSync(filePath);
        }

        // Delete all likes for this plugin
        await PluginLike.deleteMany({ pluginId: plugin._id });

        // Delete the plugin
        await Plugin.findByIdAndDelete(req.params.id);

        res.json({
            success: true,
            message: 'Plugin deleted successfully'
        });
    } catch (error) {
        console.error('Delete plugin error:', error);
        res.status(500).json({
            success: false,
            message: 'Failed to delete plugin',
            error: error.message
        });
    }
});

// POST /plugins/:id/like - Like a plugin
router.post('/:id/like', authenticateToken, async (req, res) => {
    try {
        const plugin = await Plugin.findById(req.params.id);

        if (!plugin) {
            return res.status(404).json({
                success: false,
                message: 'Plugin not found'
            });
        }

        // Check if already liked
        const existingLike = await PluginLike.findOne({
            pluginId: plugin._id,
            userId: req.user.id
        });

        if (existingLike) {
            return res.status(400).json({
                success: false,
                message: 'Plugin already liked'
            });
        }

        // Create like record
        await PluginLike.create({
            pluginId: plugin._id,
            userId: req.user.id
        });

        // Increment like count
        plugin.likeCount += 1;
        await plugin.save();

        res.json({
            success: true,
            message: 'Plugin liked',
            likeCount: plugin.likeCount
        });
    } catch (error) {
        console.error('Like plugin error:', error);
        res.status(500).json({
            success: false,
            message: 'Failed to like plugin',
            error: error.message
        });
    }
});

// DELETE /plugins/:id/like - Unlike a plugin
router.delete('/:id/like', authenticateToken, async (req, res) => {
    try {
        const plugin = await Plugin.findById(req.params.id);

        if (!plugin) {
            return res.status(404).json({
                success: false,
                message: 'Plugin not found'
            });
        }

        // Find and delete the like
        const like = await PluginLike.findOneAndDelete({
            pluginId: plugin._id,
            userId: req.user.id
        });

        if (!like) {
            return res.status(400).json({
                success: false,
                message: 'Plugin not liked'
            });
        }

        // Decrement like count
        plugin.likeCount = Math.max(0, plugin.likeCount - 1);
        await plugin.save();

        res.json({
            success: true,
            message: 'Plugin unliked',
            likeCount: plugin.likeCount
        });
    } catch (error) {
        console.error('Unlike plugin error:', error);
        res.status(500).json({
            success: false,
            message: 'Failed to unlike plugin',
            error: error.message
        });
    }
});

// GET /plugins/:id/download - Download plugin file
router.get('/:id/download', async (req, res) => {
    try {
        const plugin = await Plugin.findById(req.params.id);

        if (!plugin) {
            return res.status(404).json({
                success: false,
                message: 'Plugin not found'
            });
        }

        // Only allow download of approved plugins
        if (plugin.status !== 'approved') {
            return res.status(403).json({
                success: false,
                message: 'Plugin is not available for download'
            });
        }

        const filePath = path.join(__dirname, '..', plugin.fileUrl);

        if (!fs.existsSync(filePath)) {
            return res.status(404).json({
                success: false,
                message: 'Plugin file not found'
            });
        }

        // Increment download count
        plugin.downloadCount += 1;
        await plugin.save();

        // Send file
        res.download(filePath, plugin.fileName);
    } catch (error) {
        console.error('Download plugin error:', error);
        res.status(500).json({
            success: false,
            message: 'Failed to download plugin',
            error: error.message
        });
    }
});

module.exports = router;
