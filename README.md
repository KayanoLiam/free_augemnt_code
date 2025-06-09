# Free Augment Code

一个用 Rust 编写的跨平台系统路径工具库，提供简单易用的 API 来获取各种系统目录路径。

## 🚀 功能特性

- **跨平台支持**: 支持 Windows、macOS 和 Linux 系统
- **简单易用**: 提供直观的 API 接口
- **完整文档**: 包含详细的中文注释和使用说明
- **类型安全**: 利用 Rust 的类型系统确保代码安全性
- **零配置**: 开箱即用，无需复杂配置

## 📦 安装

确保你已经安装了 Rust 工具链。如果没有，请访问 [rustup.rs](https://rustup.rs/) 进行安装。

```bash
# 克隆项目
git clone https://github.com/KayanoLiam/free_augemnt_code.git
cd free_augemnt_code

# 构建项目
cargo build

# 运行项目
cargo run
```

## 🛠️ 使用方法

### 基本用法

```rust
use utils::paths::{get_home_dir, get_app_data_dir};

fn main() {
    // 获取用户主目录
    let home = get_home_dir();
    println!("用户主目录: {}", home);

    // 获取应用数据目录
    let app_data = get_app_data_dir();
    println!("应用数据目录: {}", app_data);
}
```

### 支持的路径类型

| 函数 | 描述 | Windows 示例 | macOS 示例 | Linux 示例 |
|------|------|-------------|------------|------------|
| `get_home_dir()` | 用户主目录 | `C:\Users\username` | `/Users/username` | `/home/username` |
| `get_app_data_dir()` | 应用数据目录 | `C:\Users\username\AppData\Roaming` | `~/Library/Application Support` | `~/.local/share` |

## 📁 项目结构

```
free_augemnt_code/
├── src/
│   ├── main.rs              # 主程序入口
│   ├── utils/               # 工具模块
│   │   ├── mod.rs          # 模块声明
│   │   └── paths.rs        # 路径相关功能
│   └── augmentUilts/       # 扩展工具模块
│       └── mod.rs
├── Cargo.toml              # 项目配置文件
├── Cargo.lock              # 依赖锁定文件
├── .gitignore              # Git 忽略文件
└── README.md               # 项目说明文档
```

## 🔧 依赖项

- **dirs**: `6.0.0` - 用于获取系统标准目录路径的跨平台库

## 🚦 开发指南

### 环境要求

- Rust 1.70.0 或更高版本
- Cargo (随 Rust 一起安装)

### 开发命令

```bash
# 检查代码
cargo check

# 运行测试
cargo test

# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 生成文档
cargo doc --open
```

### 贡献指南

1. Fork 本项目
2. 创建你的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交你的更改 (`git commit -m 'feat: 添加某个很棒的功能'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开一个 Pull Request

## 📝 API 文档

### `get_home_dir() -> String`

获取用户的主目录路径。

**返回值**: 用户主目录的完整路径字符串

**示例**:
```rust
let home = get_home_dir();
println!("主目录: {}", home);
```

### `get_app_data_dir() -> String`

获取跨平台的应用程序数据目录。

**返回值**: 应用程序数据目录的完整路径字符串

**平台差异**:
- **Windows**: 使用 `%APPDATA%` 目录
- **macOS**: 使用 `~/Library/Application Support` 目录
- **Linux**: 使用 `~/.local/share` 目录

**异常**: 如果运行在不支持的平台上会触发 panic

**示例**:
```rust
let app_data = get_app_data_dir();
println!("应用数据目录: {}", app_data);
```

## 🐛 问题报告

如果你发现了 bug 或有功能建议，请在 [GitHub Issues](https://github.com/KayanoLiam/free_augemnt_code/issues) 中提交。

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [dirs](https://crates.io/crates/dirs) - 提供跨平台目录路径支持
- Rust 社区 - 提供优秀的工具和生态系统

## 📊 项目状态

- ✅ 基础功能完成
- ✅ 跨平台支持
- ✅ 中文文档
- 🔄 持续开发中

---

**作者**: KayanoLiam
**邮箱**: [你的邮箱]
**项目链接**: https://github.com/KayanoLiam/free_augemnt_code
