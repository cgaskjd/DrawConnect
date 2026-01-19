const mongoose = require('mongoose');

const pluginSchema = new mongoose.Schema({
    name: {
        type: String,
        required: true,
        trim: true,
        maxlength: 100
    },
    slug: {
        type: String,
        required: true,
        unique: true,
        lowercase: true,
        trim: true
    },
    description: {
        type: String,
        required: true,
        maxlength: 5000
    },
    shortDescription: {
        type: String,
        maxlength: 200
    },
    version: {
        type: String,
        required: true,
        default: '1.0.0'
    },
    category: {
        type: String,
        required: true,
        enum: ['brushes', 'filters', 'tools', 'panels', 'themes', 'automation', 'other'],
        default: 'other'
    },
    tags: [{
        type: String,
        trim: true,
        lowercase: true
    }],
    authorId: {
        type: mongoose.Schema.Types.ObjectId,
        ref: 'User',
        default: null
    },
    authorName: {
        type: String,
        default: 'Anonymous'
    },
    fileUrl: {
        type: String,
        required: true
    },
    fileName: {
        type: String,
        required: true
    },
    fileSize: {
        type: Number,
        required: true
    },
    thumbnailUrl: {
        type: String,
        default: null
    },
    screenshotUrls: [{
        type: String
    }],
    downloadCount: {
        type: Number,
        default: 0
    },
    likeCount: {
        type: Number,
        default: 0
    },
    status: {
        type: String,
        enum: ['pending', 'approved', 'rejected', 'unpublished'],
        default: 'pending'
    },
    rejectReason: {
        type: String,
        default: null
    }
}, {
    timestamps: true
});

// Index for search and filtering
pluginSchema.index({ name: 'text', description: 'text', tags: 'text' });
pluginSchema.index({ category: 1, status: 1 });
pluginSchema.index({ authorId: 1 });
pluginSchema.index({ slug: 1 }, { unique: true });
pluginSchema.index({ downloadCount: -1 });
pluginSchema.index({ likeCount: -1 });
pluginSchema.index({ createdAt: -1 });

// Generate slug from name
pluginSchema.pre('validate', function(next) {
    if (this.name && !this.slug) {
        this.slug = this.name
            .toLowerCase()
            .replace(/[^a-z0-9\u4e00-\u9fa5]+/g, '-')
            .replace(/^-+|-+$/g, '')
            + '-' + Date.now().toString(36);
    }
    next();
});

module.exports = mongoose.model('Plugin', pluginSchema);
