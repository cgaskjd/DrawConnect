/**
 * Seed script to add sample plugins to the database
 * Run with: node scripts/seedPlugins.js
 */

const mongoose = require('mongoose');
require('dotenv').config();

// Plugin Schema (same as models/Plugin.js)
const pluginSchema = new mongoose.Schema({
    name: { type: String, required: true, trim: true, maxlength: 100 },
    slug: { type: String, required: true, unique: true, lowercase: true, trim: true },
    description: { type: String, required: true, maxlength: 5000 },
    shortDescription: { type: String, maxlength: 200 },
    version: { type: String, required: true, default: '1.0.0' },
    category: {
        type: String, required: true,
        enum: ['brushes', 'filters', 'tools', 'panels', 'themes', 'automation', 'other'],
        default: 'other'
    },
    tags: [{ type: String, trim: true, lowercase: true }],
    authorId: { type: mongoose.Schema.Types.ObjectId, ref: 'User', default: null },
    authorName: { type: String, default: 'DrawConnect Team' },
    fileUrl: { type: String, required: true },
    fileName: { type: String, required: true },
    fileSize: { type: Number, required: true },
    thumbnailUrl: { type: String, default: null },
    screenshotUrls: [{ type: String }],
    downloadCount: { type: Number, default: 0 },
    likeCount: { type: Number, default: 0 },
    status: {
        type: String,
        enum: ['pending', 'approved', 'rejected', 'unpublished'],
        default: 'pending'
    },
    rejectReason: { type: String, default: null }
}, { timestamps: true });

const Plugin = mongoose.model('Plugin', pluginSchema);

// Sample plugins data
const samplePlugins = [
    {
        name: 'Skin Smoothing Filter',
        slug: 'skin-smoothing-filter',
        description: `Professional skin smoothing and beautification filter for portrait photos.

Features:
• Automatic Skin Detection - Uses YCbCr color space analysis to accurately detect skin tones
• Edge-Preserving Smoothing - Bilateral filter technology preserves important details while smoothing
• Multiple Skin Tone Presets - Supports light, medium, dark, and automatic skin tone detection
• Adjustable Strength - Fine-tune the smoothing intensity from 0-100%
• Detail Preservation - Option to blend with original for natural results

Usage:
1. Open a portrait image in DrawConnect
2. Go to Filters > Portrait > Skin Smoothing
3. Adjust settings and click Apply

Keyboard Shortcut: Ctrl+Shift+S`,
        shortDescription: 'Professional portrait skin smoothing filter with automatic skin detection',
        version: '1.0.0',
        category: 'filters',
        tags: ['portrait', 'skin', 'beauty', 'smoothing', 'retouch', 'filter'],
        authorName: 'DrawConnect Team',
        fileUrl: '/uploads/plugins/skin-smoothing-filter.zip',
        fileName: 'skin-smoothing-filter.zip',
        fileSize: 3981,
        downloadCount: 128,
        likeCount: 45,
        status: 'approved'
    },
    {
        name: 'Watercolor Brush Pack',
        slug: 'watercolor-brush-pack',
        description: `A collection of 20 realistic watercolor brushes for digital painting.

Includes:
• Wet Wash - Soft, flowing strokes with natural color blending
• Dry Brush - Textured strokes for rough effects
• Splatter - Random ink splatter patterns
• Grunge - Distressed watercolor textures
• Round Soft - Smooth gradients for backgrounds
• And 15 more unique brushes!

Each brush features:
• Pressure sensitivity support
• Tilt recognition for natural strokes
• Color mixing simulation
• Paper texture interaction`,
        shortDescription: '20 realistic watercolor brushes for digital painting',
        version: '2.1.0',
        category: 'brushes',
        tags: ['watercolor', 'brush', 'painting', 'artistic', 'texture'],
        authorName: 'ArtistPro',
        fileUrl: '/uploads/plugins/watercolor-brush-pack.zip',
        fileName: 'watercolor-brush-pack.zip',
        fileSize: 15240,
        downloadCount: 356,
        likeCount: 89,
        status: 'approved'
    },
    {
        name: 'Color Harmony Panel',
        slug: 'color-harmony-panel',
        description: `An advanced color theory panel that helps you create harmonious color palettes.

Features:
• Complementary Colors - Find perfect opposite colors
• Analogous Scheme - Colors next to each other on the wheel
• Triadic Colors - Three colors equally spaced
• Split-Complementary - Variation of complementary
• Tetradic (Square) - Four colors forming a square

Additional Tools:
• Color temperature adjustment
• Saturation/Brightness variations
• Export palettes as .aco or .ase files
• Quick apply to current brush`,
        shortDescription: 'Advanced color theory panel for creating harmonious palettes',
        version: '1.5.0',
        category: 'panels',
        tags: ['color', 'palette', 'harmony', 'theory', 'design'],
        authorName: 'ColorMaster',
        fileUrl: '/uploads/plugins/color-harmony-panel.zip',
        fileName: 'color-harmony-panel.zip',
        fileSize: 8920,
        downloadCount: 234,
        likeCount: 67,
        status: 'approved'
    },
    {
        name: 'Dark Studio Theme',
        slug: 'dark-studio-theme',
        description: `A professional dark theme optimized for long drawing sessions.

Design Features:
• Ultra-dark background reduces eye strain
• High contrast UI elements for visibility
• Accent colors carefully chosen for accessibility
• Smooth transitions and animations

Customization:
• 5 accent color presets (Orange, Blue, Purple, Green, Pink)
• Adjustable panel opacity
• Custom icon set included`,
        shortDescription: 'Professional dark theme optimized for long drawing sessions',
        version: '1.2.0',
        category: 'themes',
        tags: ['theme', 'dark', 'ui', 'interface', 'design'],
        authorName: 'ThemeWorks',
        fileUrl: '/uploads/plugins/dark-studio-theme.zip',
        fileName: 'dark-studio-theme.zip',
        fileSize: 2450,
        downloadCount: 512,
        likeCount: 134,
        status: 'approved'
    },
    {
        name: 'Auto Save & Backup',
        slug: 'auto-save-backup',
        description: `Automatically save your work and create backups at regular intervals.

Features:
• Configurable auto-save interval (1-60 minutes)
• Automatic backup creation
• Keep up to 10 backup versions
• Crash recovery support
• Cloud sync integration (optional)

Never lose your work again!`,
        shortDescription: 'Automatic saving and backup system for your projects',
        version: '1.0.2',
        category: 'automation',
        tags: ['autosave', 'backup', 'safety', 'recovery', 'automation'],
        authorName: 'SafetyFirst',
        fileUrl: '/uploads/plugins/auto-save-backup.zip',
        fileName: 'auto-save-backup.zip',
        fileSize: 4120,
        downloadCount: 678,
        likeCount: 201,
        status: 'approved'
    },
    {
        name: 'Perspective Grid Tool',
        slug: 'perspective-grid-tool',
        description: `Professional perspective drawing assistance tool.

Grid Types:
• 1-Point Perspective
• 2-Point Perspective
• 3-Point Perspective
• Isometric Grid
• Custom Vanishing Points

Features:
• Snap to grid option
• Adjustable grid opacity
• Multiple grid colors
• Save/Load grid presets
• Keyboard shortcuts for quick toggling`,
        shortDescription: 'Professional perspective grid tool for accurate drawings',
        version: '2.0.0',
        category: 'tools',
        tags: ['perspective', 'grid', 'drawing', 'guide', 'architecture'],
        authorName: 'PerspectivePro',
        fileUrl: '/uploads/plugins/perspective-grid-tool.zip',
        fileName: 'perspective-grid-tool.zip',
        fileSize: 6780,
        downloadCount: 445,
        likeCount: 112,
        status: 'approved'
    }
];

async function seedPlugins() {
    try {
        // Connect to MongoDB
        const mongoUri = process.env.MONGODB_URI || 'mongodb://localhost:27017/drawconnect';
        console.log('Connecting to MongoDB...');
        await mongoose.connect(mongoUri);
        console.log('Connected to MongoDB');

        // Clear existing plugins (optional - comment out if you want to keep existing)
        // await Plugin.deleteMany({});
        // console.log('Cleared existing plugins');

        // Insert sample plugins
        for (const pluginData of samplePlugins) {
            const existing = await Plugin.findOne({ slug: pluginData.slug });
            if (existing) {
                console.log(`Plugin "${pluginData.name}" already exists, skipping...`);
                continue;
            }

            const plugin = new Plugin(pluginData);
            await plugin.save();
            console.log(`Added plugin: ${pluginData.name}`);
        }

        console.log('\nSeed completed successfully!');
        console.log(`Total plugins in database: ${await Plugin.countDocuments()}`);

    } catch (error) {
        console.error('Seed failed:', error);
    } finally {
        await mongoose.disconnect();
        console.log('Disconnected from MongoDB');
    }
}

// Run the seed
seedPlugins();
