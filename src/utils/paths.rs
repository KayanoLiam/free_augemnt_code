/// 获取用户的主目录路径 / Get user's home directory path / ユーザーのホームディレクトリパスを取得
///
/// 返回值 / Returns / 戻り値:
///     String: 用户主目录的完整路径 / Complete path to user's home directory / ユーザーホームディレクトリの完全パス
///
/// 示例 / Examples / 例:
///     - Windows: C:\Users\username
///     - macOS: /Users/username
///     - Linux: /home/username
pub fn get_home_dir() -> String {
    // 使用 dirs crate 获取主目录路径 / Use dirs crate to get home directory path / dirs crateを使用してホームディレクトリパスを取得
    let home_dir = dirs::home_dir().unwrap();
    // 将路径转换为字符串格式 / Convert path to string format / パスを文字列形式に変換
    home_dir.to_str().unwrap().to_string()
}

/// 获取跨平台的应用程序数据目录 / Get cross-platform application data directory / クロスプラットフォームのアプリケーションデータディレクトリを取得
///
/// 返回值 / Returns / 戻り値:
///     String: 应用程序数据目录的完整路径 / Complete path to application data directory / アプリケーションデータディレクトリの完全パス
///
/// 不同平台的具体路径 / Platform-specific paths / プラットフォーム固有のパス:
///     - Windows: %APPDATA% (通常是 C:\Users\<用户名>\AppData\Roaming / Usually C:\Users\<username>\AppData\Roaming / 通常 C:\Users\<ユーザー名>\AppData\Roaming)
///     - macOS: ~/Library/Application Support
///     - Linux: ~/.local/share
///
/// # Panics
/// 如果运行在不支持的平台上会触发 panic / Panics if running on unsupported platform / サポートされていないプラットフォームで実行するとパニック
pub fn get_app_data_dir() -> String {
    if cfg!(target_os = "windows") {
        // Windows 系统：使用本地数据目录 / Windows system: use local data directory / Windowsシステム：ローカルデータディレクトリを使用
        dirs::data_local_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    } else if cfg!(target_os = "macos") {
        // macOS 系统：使用应用程序支持目录 / macOS system: use application support directory / macOSシステム：アプリケーションサポートディレクトリを使用
        dirs::data_dir().unwrap().to_str().unwrap().to_string()
    } else if cfg!(target_os = "linux") {
        // Linux 系统：使用用户数据目录 / Linux system: use user data directory / Linuxシステム：ユーザーデータディレクトリを使用
        dirs::data_dir().unwrap().to_str().unwrap().to_string()
    } else {
        // 不支持的平台 / Unsupported platform / サポートされていないプラットフォーム
        panic!("Unsupported operating system platform");
    }
}

/// 获取存储文件路径 / Get storage file path / ストレージファイルパスを取得
///
/// 获取VS Code全局存储配置文件的路径
/// Get the path to VS Code global storage configuration file
/// VS Codeグローバルストレージ設定ファイルのパスを取得
///
/// 返回值 / Returns / 戻り値:
///     String: storage.json文件的完整路径 / Complete path to storage.json file / storage.jsonファイルの完全パス
pub fn get_storage_dir() -> String {
    if cfg!(target_os = "windows") {
        // Windows 系统 / Windows system / Windowsシステム
        let base_path = std::env::var("APPDATA").unwrap_or_default();
        std::path::Path::new(&base_path)
            .join("Code")
            .join("User")
            .join("globalStorage")
            .join("storage.json")
            .to_str()
            .unwrap()
            .to_string()
    } else if cfg!(target_os = "macos") {
        // macOS 系统 / macOS system / macOSシステム
        dirs::home_dir()
            .unwrap()
            .join("Library")
            .join("Application Support")
            .join("Code")
            .join("User")
            .join("globalStorage")
            .join("storage.json")
            .to_str()
            .unwrap()
            .to_string()
    } else {
        // Linux 和其他类 Unix 系统 / Linux and other Unix-like systems / Linuxおよびその他のUnix系システム
        dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("Code")
            .join("User")
            .join("globalStorage")
            .join("storage.json")
            .to_str()
            .unwrap()
            .to_string()
    }
}

/// 获取数据库文件路径 / Get database file path / データベースファイルパスを取得
///
/// 获取VS Code状态数据库文件的路径
/// Get the path to VS Code state database file
/// VS Code状態データベースファイルのパスを取得
///
/// 返回值 / Returns / 戻り値:
///     String: state.vscdb文件的完整路径 / Complete path to state.vscdb file / state.vscdbファイルの完全パス
pub fn get_db_path() -> String {
    if cfg!(target_os = "windows") {
        // Windows 系统 / Windows system / Windowsシステム
        let base_path = std::env::var("APPDATA").unwrap_or_default();
        std::path::Path::new(&base_path)
            .join("Code")
            .join("User")
            .join("globalStorage")
            .join("state.vscdb")
            .to_str()
            .unwrap()
            .to_string()
    } else if cfg!(target_os = "macos") {
        // macOS 系统 / macOS system / macOSシステム
        dirs::home_dir()
            .unwrap()
            .join("Library")
            .join("Application Support")
            .join("Code")
            .join("User")
            .join("globalStorage")
            .join("state.vscdb")
            .to_str()
            .unwrap()
            .to_string()
    } else {
        // Linux 和其他类 Unix 系统 / Linux and other Unix-like systems / Linuxおよびその他のUnix系システム
        dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("Code")
            .join("User")
            .join("globalStorage")
            .join("state.vscdb")
            .to_str()
            .unwrap()
            .to_string()
    }
}

/// 获取机器ID文件路径 / Get machine ID file path / マシンIDファイルパスを取得
///
/// 获取VS Code机器ID文件的路径
/// Get the path to VS Code machine ID file
/// VS CodeマシンIDファイルのパスを取得
///
/// 返回值 / Returns / 戻り値:
///     String: machineid文件的完整路径 / Complete path to machineid file / machineidファイルの完全パス
pub fn get_machine_id_path() -> String {
    if cfg!(target_os = "windows") {
        // Windows 系统 / Windows system / Windowsシステム
        let base_path = std::env::var("APPDATA").unwrap_or_default();
        std::path::Path::new(&base_path)
            .join("Code")
            .join("User")
            .join("machineid")
            .to_str()
            .unwrap()
            .to_string()
    } else if cfg!(target_os = "macos") {
        // macOS 系统 / macOS system / macOSシステム
        dirs::home_dir()
            .unwrap()
            .join("Library")
            .join("Application Support")
            .join("Code")
            .join("machineid")
            .to_str()
            .unwrap()
            .to_string()
    } else {
        // Linux 和其他类 Unix 系统 / Linux and other Unix-like systems / Linuxおよびその他のUnix系システム
        dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("Code")
            .join("machineid")
            .to_str()
            .unwrap()
            .to_string()
    }
}

/// 获取工作区存储目录路径 / Get workspace storage directory path / ワークスペースストレージディレクトリパスを取得
///
/// 获取VS Code工作区存储目录的路径
/// Get the path to VS Code workspace storage directory
/// VS Codeワークスペースストレージディレクトリのパスを取得
///
/// 返回值 / Returns / 戻り値:
///     String: workspaceStorage目录的完整路径 / Complete path to workspaceStorage directory / workspaceStorageディレクトリの完全パス
pub fn get_workspace_storage_path() -> String {
    if cfg!(target_os = "windows") {
        // Windows 系统 / Windows system / Windowsシステム
        let base_path = std::env::var("APPDATA").unwrap_or_default();
        std::path::Path::new(&base_path)
            .join("Code")
            .join("User")
            .join("workspaceStorage")
            .to_str()
            .unwrap()
            .to_string()
    } else if cfg!(target_os = "macos") {
        // macOS 系统 / macOS system / macOSシステム
        dirs::home_dir()
            .unwrap()
            .join("Library")
            .join("Application Support")
            .join("Code")
            .join("User")
            .join("workspaceStorage")
            .to_str()
            .unwrap()
            .to_string()
    } else {
        // Linux 和其他类 Unix 系统 / Linux and other Unix-like systems / Linuxおよびその他のUnix系システム
        dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("Code")
            .join("User")
            .join("workspaceStorage")
            .to_str()
            .unwrap()
            .to_string()
    }
}