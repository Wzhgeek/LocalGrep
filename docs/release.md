# 发布说明

- macOS: `.app` / `.dmg`
- Windows: `.exe` / `.msi`（由 Tauri bundler 配置决定）
- Linux: `.deb` 等

## GitHub：首次推送仓库

在本地项目根目录执行（将 `YOUR_TOKEN` 换为个人访问令牌，或使用 SSH 远程地址）：

```bash
git init
git add .
git commit -m "chore: initial LocalGrep import"
git branch -M main
git remote add origin https://github.com/Wzhgeek/LocalGrep.git
git push -u origin main
```

若已存在 `origin`，可改为：

```bash
git remote set-url origin https://github.com/Wzhgeek/LocalGrep.git
git push -u origin main
```

## 自动 CI（推送到 `main`）

工作流：`.github/workflows/ci.yml`  
在每次 **push 到 `main`** 或 **Pull Request** 时执行：前端 lint / 测试 / 构建，以及 Rust `fmt`、`clippy`、`test`。

## 自动发布 Release（打 tag）

工作流：`.github/workflows/release.yml`  
在推送 **符合 `v*` 的标签**（例如 `v0.1.0`）时触发，使用 [tauri-apps/tauri-action](https://github.com/tauri-apps/tauri-action) 在 **macOS（ARM + x64）、Ubuntu、Windows** 上构建并上传安装包到 **GitHub Releases**。

```bash
# 确保 package.json 与 src-tauri/tauri.conf.json 中 version 与本次发布一致
git tag v0.1.0
git push origin v0.1.0
```

标签名中的版本号应与 Tauri 应用版本一致；`v__VERSION__` 中的 `__VERSION__` 会由 Action 替换为 `tauri.conf.json` 里的 `version`。

## 签名与公证（生产环境建议）

- **macOS**：在 Apple 开发者环境对应用 **代码签名** 与 **Notarization**（CI 中需配置证书与密钥，见 [Tauri 分发文档](https://v2.tauri.app/distribute/)）。
- **Windows**：Authenticode 签名（可选但推荐）。

当前默认 workflow **未** 配置签名密钥；公开发布前请在仓库 **Settings → Secrets and variables → Actions** 中按 Tauri 文档添加对应环境变量与步骤。

## 手动检查清单

1. 各平台安装包可安装、可启动
2. 核心检索流程可用
3. Release 说明中写明已知问题与升级注意点
