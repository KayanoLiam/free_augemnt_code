# Free Augment Code

一个用 Rust 编写的 VS Code Augment 数据清理工具，专门用于清理和重置 VS Code 中与 Augment 扩展相关的遥测数据、机器标识符和工作区存储。

*A Rust-based VS Code Augment data cleaning tool designed to clean and reset telemetry data, machine identifiers, and workspace storage related to the Augment extension in VS Code.*

*VS CodeのAugment拡張機能に関連するテレメトリデータ、マシン識別子、ワークスペースストレージをクリーンアップおよびリセットするためのRustベースのツール。*

## 🚀 功能特性 / Features / 機能

- **🔄 遥测ID重置 / Telemetry ID Reset / テレメトリID リセット**: 自动生成新的机器ID和设备ID / Automatically generate new machine and device IDs / 新しいマシンIDとデバイスIDを自動生成
- **🗄️ SQLite数据清理 / SQLite Data Cleaning / SQLiteデータクリーニング**: 从VS Code状态数据库中删除Augment相关记录 / Remove Augment-related records from VS Code state database / VS Code状態データベースからAugment関連レコードを削除
- **📁 工作区存储清理 / Workspace Storage Cleaning / ワークスペースストレージクリーニング**: 清理VS Code工作区存储目录 / Clean VS Code workspace storage directory / VS Codeワークスペースストレージディレクトリをクリーンアップ
- **💾 自动备份 / Automatic Backup / 自動バックアップ**: 在修改前自动创建备份文件 / Automatically create backup files before modifications / 変更前にバックアップファイルを自動作成
- **🌐 跨平台支持 / Cross-platform Support / クロスプラットフォーム対応**: 支持 Windows、macOS 和 Linux / Support for Windows, macOS, and Linux / Windows、macOS、Linuxをサポート
- **🛡️ 类型安全 / Type Safety / 型安全**: 利用 Rust 的类型系统确保代码安全性 / Leverage Rust's type system for code safety / Rustの型システムを活用したコードの安全性
- **📝 三语注释 / Trilingual Comments / 三言語コメント**: 包含中文、英文和日文注释 / Contains Chinese, English, and Japanese comments / 中国語、英語、日本語のコメントを含む

## ⚠️ 重要说明 / Important Notice / 重要な注意事項

**此工具会修改VS Code的配置文件和数据库。使用前请确保：**
- 已关闭所有VS Code实例
- 已备份重要的VS Code配置
- 理解此操作的影响

**This tool modifies VS Code configuration files and databases. Before use, ensure:**
- All VS Code instances are closed
- Important VS Code configurations are backed up
- You understand the impact of this operation

**このツールはVS Codeの設定ファイルとデータベースを変更します。使用前に以下を確認してください：**
- すべてのVS Codeインスタンスが閉じられている
- 重要なVS Code設定がバックアップされている
- この操作の影響を理解している

## 📦 安装 / Installation / インストール

### 前提条件 / Prerequisites / 前提条件

确保你已经安装了 Rust 工具链。如果没有，请访问 [rustup.rs](https://rustup.rs/) 进行安装。

*Ensure you have the Rust toolchain installed. If not, visit [rustup.rs](https://rustup.rs/) for installation.*

*Rustツールチェーンがインストールされていることを確認してください。インストールされていない場合は、[rustup.rs](https://rustup.rs/)にアクセスしてインストールしてください。*

### 构建和运行 / Build and Run / ビルドと実行

```bash
# 克隆项目 / Clone the project / プロジェクトをクローン
git clone https://github.com/KayanoLiam/free_augemnt_code.git
cd free_augemnt_code

# 构建项目 / Build the project / プロジェクトをビルド
cargo build --release

# 运行项目 / Run the project / プロジェクトを実行
cargo run
```

## 🛠️ 使用方法 / Usage / 使用方法

### 基本使用 / Basic Usage / 基本的な使用方法

直接运行程序即可执行所有清理操作：

*Simply run the program to execute all cleaning operations:*

*プログラムを実行するだけで、すべてのクリーニング操作が実行されます：*

```bash
cargo run
```

### 程序执行流程 / Program Execution Flow / プログラム実行フロー

1. **显示系统路径 / Display System Paths / システムパス表示**
   - 用户主目录 / User home directory / ユーザーホームディレクトリ
   - VS Code配置目录 / VS Code configuration directory / VS Code設定ディレクトリ
   - 存储文件路径 / Storage file paths / ストレージファイルパス

2. **修改遥测ID / Modify Telemetry IDs / テレメトリID変更**
   - 生成新的机器ID和设备ID / Generate new machine and device IDs / 新しいマシンIDとデバイスIDを生成
   - 更新storage.json文件 / Update storage.json file / storage.jsonファイルを更新
   - 更新machineid文件 / Update machineid file / machineidファイルを更新

3. **清理SQLite数据库 / Clean SQLite Database / SQLiteデータベースクリーニング**
   - 删除包含'augment'关键字的记录 / Delete records containing 'augment' keyword / 'augment'キーワードを含むレコードを削除
   - 创建数据库备份 / Create database backup / データベースバックアップを作成

4. **清理工作区存储 / Clean Workspace Storage / ワークスペースストレージクリーニング**
   - 创建工作区备份 / Create workspace backup / ワークスペースバックアップを作成
   - 删除工作区文件 / Delete workspace files / ワークスペースファイルを削除

### 示例输出 / Example Output / 出力例

```
System Paths:
Home Directory: /Users/username
App data Directory: /Users/username/Library/Application Support
Storage Directory: /Users/username/Library/Application Support/Code/User/globalStorage/storage.json
DB Path: /Users/username/Library/Application Support/Code/User/globalStorage/state.vscdb
Machine ID Path: /Users/username/Library/Application Support/Code/machineid
Workspace Storage Path: /Users/username/Library/Application Support/Code/User/workspaceStorage

Machine ID backup created at: /Users/username/Library/Application Support/Code/machineid.bak.1234567890
Old IDs:
Old Machine ID: abc123...
Old Device ID: def456...
New IDs:
New Machine ID: xyz789...
New Device ID: uvw012...

Cleaning SQLite Database:
Database backup created at: /Users/username/Library/Application Support/Code/User/globalStorage/state.vscdb.bak.1234567890
Deleted rows: 5

Cleaning Workspace Storage:
Workspace backup created at: /Users/username/Library/Application Support/Code/User/workspaceStorage_backup_1234567890.zip
Deleted files: 42
```

## 📁 项目结构 / Project Structure / プロジェクト構造

```
free_augemnt_code/
├── src/
│   ├── main.rs                    # 主程序入口 / Main program entry / メインプログラムエントリ
│   ├── utils/                     # 工具模块 / Utility modules / ユーティリティモジュール
│   │   ├── mod.rs                # 模块声明 / Module declarations / モジュール宣言
│   │   ├── paths.rs              # 路径工具 / Path utilities / パスユーティリティ
│   │   └── device_codes.rs       # 设备代码生成 / Device code generation / デバイスコード生成
│   └── augment_utils/            # Augment工具模块 / Augment utility modules / Augmentユーティリティモジュール
│       ├── mod.rs                # 模块声明 / Module declarations / モジュール宣言
│       ├── json_modifier.rs      # JSON修改器 / JSON modifier / JSON修正器
│       ├── sqlite_modifier.rs    # SQLite修改器 / SQLite modifier / SQLite修正器
│       └── workspace_cleaner.rs  # 工作区清理器 / Workspace cleaner / ワークスペースクリーナー
├── Cargo.toml                    # 项目配置 / Project configuration / プロジェクト設定
├── .gitignore                    # Git忽略文件 / Git ignore file / Git無視ファイル
└── README.md                     # 项目文档 / Project documentation / プロジェクトドキュメント
```

## 🔧 依赖项 / Dependencies / 依存関係

- **dirs**: `6.0.0` - 跨平台目录路径获取 / Cross-platform directory path retrieval / クロスプラットフォームディレクトリパス取得
- **serde_json**: `1.0` - JSON序列化和反序列化 / JSON serialization and deserialization / JSONシリアライゼーションとデシリアライゼーション
- **rand**: `0.8.5` - 随机数生成 / Random number generation / 乱数生成
- **hex**: `0.4.3` - 十六进制编码 / Hexadecimal encoding / 16進エンコーディング
- **rusqlite**: `0.29.0` - SQLite数据库操作 / SQLite database operations / SQLiteデータベース操作

## 🚦 开发指南 / Development Guide / 開発ガイド

### 环境要求 / Requirements / 要件

- Rust 1.70.0 或更高版本 / Rust 1.70.0 or higher / Rust 1.70.0以上
- Cargo (随 Rust 一起安装) / Cargo (comes with Rust) / Cargo（Rustに付属）

### 开发命令 / Development Commands / 開発コマンド

```bash
# 检查代码 / Check code / コードチェック
cargo check

# 运行测试 / Run tests / テスト実行
cargo test

# 格式化代码 / Format code / コードフォーマット
cargo fmt

# 代码检查 / Code linting / コードリンティング
cargo clippy

# 生成文档 / Generate documentation / ドキュメント生成
cargo doc --open

# 发布构建 / Release build / リリースビルド
cargo build --release
```

## 📝 核心API文档 / Core API Documentation / コアAPI ドキュメント

### 路径工具 / Path Utilities / パスユーティリティ

#### `get_home_dir() -> String`
获取用户主目录路径 / Get user home directory path / ユーザーホームディレクトリパスを取得

#### `get_storage_dir() -> String`
获取VS Code存储文件路径 / Get VS Code storage file path / VS Codeストレージファイルパスを取得

#### `get_db_path() -> String`
获取VS Code数据库文件路径 / Get VS Code database file path / VS Codeデータベースファイルパスを取得

#### `get_machine_id_path() -> String`
获取机器ID文件路径 / Get machine ID file path / マシンIDファイルパスを取得

#### `get_workspace_storage_path() -> String`
获取工作区存储目录路径 / Get workspace storage directory path / ワークスペースストレージディレクトリパスを取得

### 设备代码生成 / Device Code Generation / デバイスコード生成

#### `generate_machine_id() -> String`
生成64字符的随机机器ID / Generate 64-character random machine ID / 64文字のランダムマシンIDを生成

#### `generate_device_id() -> String`
生成64字符的随机设备ID / Generate 64-character random device ID / 64文字のランダムデバイスIDを生成

### Augment工具 / Augment Utilities / Augmentユーティリティ

#### `modify_telemetry_ids() -> HashMap<String, String>`
修改遥测ID并创建备份 / Modify telemetry IDs and create backups / テレメトリIDを変更してバックアップを作成

#### `clean_augment_data() -> HashMap<String, String>`
清理SQLite数据库中的Augment数据 / Clean Augment data from SQLite database / SQLiteデータベースからAugmentデータをクリーンアップ

#### `clean_workspace_storage() -> HashMap<String, String>`
清理工作区存储并创建备份 / Clean workspace storage and create backup / ワークスペースストレージをクリーンアップしてバックアップを作成

## ⚠️ 注意事项 / Precautions / 注意事項

1. **数据安全 / Data Safety / データ安全性**: 程序会自动创建备份，但建议手动备份重要数据 / The program automatically creates backups, but manual backup of important data is recommended / プログラムは自動的にバックアップを作成しますが、重要なデータの手動バックアップを推奨します

2. **VS Code状态 / VS Code Status / VS Code状態**: 运行前请关闭所有VS Code实例 / Close all VS Code instances before running / 実行前にすべてのVS Codeインスタンスを閉じてください

3. **权限要求 / Permission Requirements / 権限要件**: 程序需要读写VS Code配置目录的权限 / The program requires read/write permissions to VS Code configuration directories / プログラムはVS Code設定ディレクトリへの読み書き権限が必要です

## 🐛 问题报告 / Issue Reporting / 問題報告

如果你发现了 bug 或有功能建议，请在 [GitHub Issues](https://github.com/KayanoLiam/free_augemnt_code/issues) 中提交。

*If you find bugs or have feature suggestions, please submit them in [GitHub Issues](https://github.com/KayanoLiam/free_augemnt_code/issues).*

*バグを発見したり機能提案がある場合は、[GitHub Issues](https://github.com/KayanoLiam/free_augemnt_code/issues)で報告してください。*

## 📄 许可证 / License / ライセンス

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

*This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.*

*このプロジェクトはMITライセンスの下でライセンスされています - 詳細は[LICENSE](LICENSE)ファイルを参照してください。*

## 🙏 致谢 / Acknowledgments / 謝辞

- [dirs](https://crates.io/crates/dirs) - 跨平台目录路径支持 / Cross-platform directory path support / クロスプラットフォームディレクトリパスサポート
- [serde_json](https://crates.io/crates/serde_json) - JSON处理库 / JSON processing library / JSON処理ライブラリ
- [rusqlite](https://crates.io/crates/rusqlite) - SQLite数据库接口 / SQLite database interface / SQLiteデータベースインターフェース
- Rust 社区 - 提供优秀的工具和生态系统 / Providing excellent tools and ecosystem / 優れたツールとエコシステムの提供

## 📊 项目状态 / Project Status / プロジェクト状況

- ✅ 核心功能完成 / Core functionality completed / コア機能完了
- ✅ 跨平台支持 / Cross-platform support / クロスプラットフォーム対応
- ✅ 三语文档 / Trilingual documentation / 三言語ドキュメント
- ✅ 自动备份功能 / Automatic backup functionality / 自動バックアップ機能
- ✅ 类型安全保证 / Type safety guarantee / 型安全保証
- 🔄 持续维护中 / Under continuous maintenance / 継続的メンテナンス中

---

**作者 / Author / 作者**: KayanoLiam
**项目链接 / Project Link / プロジェクトリンク**: https://github.com/KayanoLiam/free_augemnt_code
