# Rust Bevy 生态系统模拟游戏

这是一个使用 Rust 和 Bevy 引擎开发的生态系统模拟游戏项目。游戏旨在通过可视化的方式展示生物之间的相互作用和生态平衡，适用于教育和娱乐场景。

## 项目特点

- **生态系统模拟**：模拟生物之间的食物链关系和能量传递。
- **模块化架构**：采用 Bevy 的 ECS 架构，模块化设计便于扩展和维护。
- **数据驱动**：通过配置文件定义游戏内容，支持热重载。
- **可视化 UI**：提供直观的用户界面，展示生物状态和游戏信息。
- **空间分区**：优化性能，提高大规模实体交互效率。

## 技术栈

- **Rust**：高性能系统编程语言。
- **Bevy**：开源的 ECS 游戏引擎，支持跨平台开发。
- **WGSL**：用于编写着色器代码，实现图形渲染。

## 目录结构

- `minigame/src/core`：核心游戏逻辑，包括组件、系统、状态机等。
- `minigame/src/level`：关卡系统，负责加载和管理游戏关卡。
- `minigame/src/ui`：用户界面模块，包括 HUD 和实体卡片。
- `minigame/src/sprite`：精灵图管理模块，负责加载和管理游戏资源。
- `minigame/src/scenes`：场景管理模块，处理主菜单、游戏场景切换。
- `minigame/src/ai`：AI 行为树模块，控制生物行为逻辑。
- `minigame/assets`：游戏资源文件，如着色器、纹理等。

## 安装与运行

### 环境要求

- Rust 1.60+
- Bevy 引擎0.16.1
- Cargo 构建工具

### 构建项目

```bash
git clone https://gitee.com/xeroin/rust_bevy_demo.git
cd rust_bevy_demo
cargo build --release
```

### 运行游戏

```bash
cargo run
```

## 使用说明

1. **启动游戏**：运行后会进入主菜单界面，选择关卡开始游戏。
2. **交互操作**：
   - 鼠标点击选择生物，查看详细信息。
   - 使用键盘控制摄像机移动（WASD）。
   - 按 `ESC` 键退出游戏。
3. **观察生态系统**：游戏会模拟生物之间的互动，包括觅食、移动、繁殖等行为。

## 贡献指南

欢迎贡献代码和建议！请遵循以下步骤：

1. Fork 项目
2. 创建新分支 (`git checkout -b feature/new-feature`)
3. 提交更改 (`git commit -am 'Add new feature'`)
4. 推送分支 (`git push origin feature/new-feature`)
5. 创建 Pull Request

## 许可证

本项目采用 MIT 许可证。详情请查看 [LICENSE](LICENSE) 文件。