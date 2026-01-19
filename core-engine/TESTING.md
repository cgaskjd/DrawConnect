# DrawConnect Core Engine 测试指南

## 快速开始

### 1. 环境准备

确保已安装 Rust 工具链：

```bash
# 安装 Rust (如果尚未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 验证安装
rustc --version
cargo --version
```

### 2. 运行测试

```bash
cd core-engine

# 运行所有单元测试
cargo test

# 运行特定模块的测试
cargo test brush::      # 笔刷模块测试
cargo test layer::      # 图层模块测试
cargo test color::      # 颜色模块测试
cargo test canvas::     # 画布模块测试

# 显示测试输出
cargo test -- --nocapture

# 运行特定测试
cargo test test_brush_creation

# 并行测试 (默认)
cargo test -- --test-threads=4
```

### 3. 运行基准测试

```bash
# 运行所有基准测试
cargo bench

# 运行特定基准测试
cargo bench brush_benchmark
cargo bench layer_benchmark
```

### 4. 运行示例程序

```bash
# 运行基础示例
cargo run --example basic_drawing

# 运行压力测试示例
cargo run --example stress_test

# 运行渲染示例
cargo run --example render_test
```

## 测试类型

### 单元测试 (已内置)

每个模块都包含单元测试，位于各模块文件底部的 `#[cfg(test)]` 块中：

- `brush/mod.rs` - 笔刷创建、压感计算、印章生成
- `layer/mod.rs` - 图层操作、混合模式
- `layer/blend.rs` - 所有混合模式算法
- `color/mod.rs` - 颜色转换、HSB/HSL
- `canvas/mod.rs` - 画布操作、撤销/重做
- `canvas/tile.rs` - 瓦片管理
- `stroke/mod.rs` - 笔触构建、平滑
- `geometry/mod.rs` - 几何计算

### 集成测试

位于 `tests/` 目录，测试模块间的交互。

### 基准测试

位于 `benches/` 目录，测试性能关键路径。

## 代码覆盖率

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html

# 查看报告
open tarpaulin-report.html
```

## 常见测试场景

### 测试笔刷渲染

```rust
use drawconnect_core::*;

#[test]
fn test_brush_stroke() {
    let mut engine = DrawEngine::new().unwrap();

    // 创建画布和图层
    let layer_id = engine.layer_manager().write().add_layer("Test Layer");

    // 创建笔触
    let mut stroke = Stroke::new();
    stroke.add_point(StrokePoint::new(100.0, 100.0, 1.0));
    stroke.add_point(StrokePoint::new(150.0, 150.0, 0.8));
    stroke.add_point(StrokePoint::new(200.0, 200.0, 0.5));

    // 渲染笔触
    engine.process_stroke(&stroke).unwrap();

    // 验证像素被修改
    let canvas = engine.canvas();
    let pixel = canvas.read().get_pixel(150, 150);
    assert!(pixel.is_some());
}
```

### 测试图层混合

```rust
use drawconnect_core::*;

#[test]
fn test_layer_blending() {
    let base = Color::from_rgb(1.0, 0.0, 0.0);  // 红色
    let blend = Color::from_rgb(0.0, 0.0, 1.0); // 蓝色

    // 测试正片叠底
    let result = BlendMode::Multiply.blend(base, blend);
    assert!(result.r < 0.1);  // 红*蓝 ≈ 0

    // 测试滤色
    let result = BlendMode::Screen.blend(base, blend);
    assert!(result.r > 0.9);  // 接近白色的红色分量
}
```

## 调试技巧

```bash
# 启用调试日志
RUST_LOG=debug cargo test

# 使用 backtrace
RUST_BACKTRACE=1 cargo test

# 检查内存问题
cargo test --release
```
