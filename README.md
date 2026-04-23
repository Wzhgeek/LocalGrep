# LocalGrep

**LocalGrep** 是一款基于 **Rust + Tauri 2** 的跨平台本地文件搜索桌面应用：在本地完成目录扫描、全文索引与检索，数据不出本机。

## 功能概览

- 文件名、路径与全文内容检索（由 Tantivy 驱动）
- 本地元数据与任务状态（SQLite）
- 增量与后台索引任务（规划中持续完善）
- 面向 Windows、macOS、Linux 的桌面打包

## 技术栈

| 层次 | 技术 |
| --- | --- |
| 桌面壳 | Tauri 2 |
| 界面 | React 18 + TypeScript + Vite |
| 核心逻辑 | Rust（Tokio 异步） |
| 全文索引 | Tantivy |
| 元数据 / 状态 | SQLite（rusqlite，bundled） |

## 仓库结构

```text
.
├── src/                 # 前端（React + Vite）
├── src-tauri/           # 后端与 Tauri 配置（Rust）
├── docs/                # 项目文档（架构、发布、QA 等）
├── .github/workflows/  # GitHub Actions（CI、Release 构建）
├── scripts/             # 辅助脚本
└── .data/               # 本地开发时数据库与索引目录（已加入 .gitignore，勿提交）
```

## GitHub 与自动构建

- 推送到 **`main`** 会运行 CI：`.github/workflows/ci.yml`（前端 + Rust 检查）。
- 推送 **`v*` 标签**（如 `v0.1.0`）会运行发布：`.github/workflows/release.yml`，用 [tauri-action](https://github.com/tauri-apps/tauri-action) 构建多平台安装包并上传到 **GitHub Releases**。

详细命令与注意事项见 [docs/release.md](docs/release.md)。

## 环境要求

- **Node.js** 建议 20 LTS 或更高
- **Rust** 稳定版（`rustup` 安装 `stable`），用于 `src-tauri` 编译
- 开发 Tauri 桌面端时，需按 [Tauri 官方文档](https://v2.tauri.app/start/prerequisites/) 准备各平台依赖

## 开发

```bash
# 安装依赖
npm install

# 仅前端（浏览器调试 UI）
npm run dev

# 在已安装 Tauri CLI 的前提下，于项目根目录运行桌面应用
# npx 方式（推荐，无需全局安装）:
npx tauri dev
```

### 代码质量

```bash
# 前端：Lint / 测试 / 生产构建
npm run lint
npm run test
npm run build

# 后端：格式化与静态检查（需在 src-tauri 目录下且已安装 Rust 工具链）
cd src-tauri
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## 数据存储说明

- 开发运行时会在项目根目录下生成 **`.data/`**（含 SQLite 与 Tantivy 索引），已默认忽略，请勿将个人数据误提交到仓库。

## 作者

- **ZiHan Wang**
- 邮箱: [wangzh011031@163.com](mailto:wangzh011031@163.com)

## 许可证

（待项目确定后补充，例如 MIT / Apache-2.0。）
