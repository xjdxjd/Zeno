# Zeno

轻量级 Windows 桌面凭据管理工具。纯本地、离线运行，定位是「打开即用、快速复制粘贴」——记录工作中的账号密码、数据库连接、服务器信息，需要时几秒钟复制出来。

![tech](https://img.shields.io/badge/Tauri-2-blue) ![tech](https://img.shields.io/badge/Svelte-5-orange) ![tech](https://img.shields.io/badge/Rust-stable-dea584)

## 功能特性

- **记录管理**：四类记录模板（用户名/密码、数据库、服务器、网站），创建/编辑/删除，编辑时类别只读
- **分组列表**：按类别固定顺序分组展示，组内按更新时间倒序，卡片内直接展示全部字段
- **字段级复制**：每个字段一键复制；密码默认掩码显示，可切换明文
- **搜索**：实时过滤，匹配标题和所有非密码字段（支持按 IP 等数字内容搜索），大小写不敏感
- **全局快速搜索**：任意界面按 `Alt+Q` 唤起置顶迷你搜索弹窗（屏幕左上角），`Ctrl+数字` 直选复制字段，复制后自动回到原窗口粘贴即可
- **密码生成器**：长度 8–64 可调，四类字符集开关，默认 16 位全字符集
- **系统托盘**：关闭窗口不退出，托盘常驻；菜单支持显示主窗口/退出
- **剪贴板保护**：复制密码 30 秒后自动清空剪贴板（若期间未被其他内容覆盖）

## 核心设计原则（请先读这段）

**免密解锁是刻意的设计，不是遗漏。** 应用只在首次启动时要求设置主密码（用于派生加密密钥），之后每次启动直接解锁，永不要求输入主密码。这个工具的价值在于「打开即用」，每次输主密码就失去了意义。

由此明确安全边界：

- 加密（Argon2id + AES-256-GCM）保护的是 **vault 文件本身**——单独拷走 `vault.enc` 无法读出内容
- 派生密钥保存在本机应用数据目录（`config.json`），**不防御能接触本机磁盘的人**
- 便利性优先于安全强度，这是本工具明确的产品取舍

## 技术栈

- 前端：Svelte 5 + TypeScript + Tailwind CSS 4
- 后端：Rust（Tauri 2，含托盘、全局快捷键）
- 构建：Vite

## 项目结构

```
zeno-frontend/            # 应用主体
├── src/
│   ├── App.svelte        # 入口与页面切换
│   ├── lib/              # 各页面组件、类型、剪贴板工具
│   └── quick.ts          # 全局快速搜索弹窗入口
├── src-tauri/
│   ├── src/main.rs       # 窗口/托盘/全局热键
│   ├── src/commands.rs   # Tauri 命令（CRUD/搜索/生成器）
│   ├── src/crypto.rs     # 加密与 vault 存储
│   └── capabilities/     # Tauri 2 能力授权
├── quick.html            # 快速搜索弹窗页面
└── vite.config.ts        # 多页构建（主窗口 + 快速弹窗）
```

数据文件（不在仓库内）：`%APPDATA%/zeno/` 下的 `vault.enc`（加密数据）与 `config.json`（密钥）。备份 `vault.enc` 即可手动备份全部数据。

## 开发

环境要求：Node.js、Rust（MSVC 工具链）、Visual Studio Build Tools。

```bash
cd zeno-frontend
npm install

# 开发模式（需先初始化 MSVC 环境）
# call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" x64
npm run tauri dev
```

## 构建

```bash
cd zeno-frontend
npx tauri build --no-bundle
```

产物为绿色单文件 `src-tauri/target/release/zeno.exe`（前端资源已嵌入，放到任意位置双击即用，无需安装）。

> 注意：`cargo build` 不会嵌入前端资源，必须用 `tauri build`；如需 NSIS 安装包，用 `npx tauri build -b nsis`。

## 安全说明

- 主密码仅首次设置时通过 Argon2id 派生密钥，此后不参与解锁流程
- 数据使用 AES-256-GCM 加密，每次保存生成新随机 nonce
- 应用不发起任何网络请求，不收集遥测数据
