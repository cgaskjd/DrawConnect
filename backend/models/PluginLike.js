const mongoose = require('mongoose');

const pluginLikeSchema = new mongoose.Schema({
    pluginId: {
        type: mongoose.Schema.Types.ObjectId,
        ref: 'Plugin',
        required: true
    },
    userId: {
        type: mongoose.Schema.Types.ObjectId,
        ref: 'User',
        required: true
    }
}, {
    timestamps: true
});

// Ensure one user can only like a plugin once
pluginLikeSchema.index({ pluginId: 1, userId: 1 }, { unique: true });
pluginLikeSchema.index({ pluginId: 1 });
pluginLikeSchema.index({ userId: 1 });

module.exports = mongoose.model('PluginLike', pluginLikeSchema);
