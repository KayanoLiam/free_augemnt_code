/// 获取用户的主目录路径
///
/// 返回值:
///     String: 用户主目录的完整路径
///
/// 示例:
///     - Windows: C:\Users\username
///     - macOS: /Users/username
///     - Linux: /home/username
pub fn get_home_dir() -> String {
    // 使用 dirs crate 获取主目录路径
    let home_dir = dirs::home_dir().unwrap();
    // 将路径转换为字符串格式
    home_dir.to_str().unwrap().to_string()
}

/// 获取跨平台的应用程序数据目录
///
/// 返回值:
///     String: 应用程序数据目录的完整路径
///
/// 不同平台的具体路径:
///     - Windows: %APPDATA% (通常是 C:\Users\<用户名>\AppData\Roaming)
///     - macOS: ~/Library/Application Support
///     - Linux: ~/.local/share
///
/// # Panics
/// 如果运行在不支持的平台上会触发 panic
pub fn get_app_data_dir() -> String {
    if cfg!(target_os = "windows") {
        // Windows 系统：使用本地数据目录
        dirs::data_local_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    } else if cfg!(target_os = "macos") {
        // macOS 系统：使用应用程序支持目录
        dirs::data_dir().unwrap().to_str().unwrap().to_string()
    } else if cfg!(target_os = "linux") {
        // Linux 系统：使用用户数据目录
        dirs::data_dir().unwrap().to_str().unwrap().to_string()
    } else {
        // 不支持的平台
        panic!("不支持的操作系统平台");
    }
}

pub fn get_storage_dir() -> String {
    if cfg!(target_os = "windows") {
        // Windows
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
        // macOS
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
        // Linux 和其他类 Unix 系统
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

pub fn get_db_path() -> String {
    /*
    python:
    if sys.platform == "win32":
        # Windows
        base_path = os.getenv("APPDATA", "")
        return os.path.join(base_path, "Code", "User", "globalStorage", "state.vscdb")
    elif sys.platform == "darwin":
        # macOS
        return os.path.join(str(Path.home()), "Library", "Application Support", "Code", "User", "globalStorage", "state.vscdb")
    else:
        # Linux and other Unix-like systems
        return os.path.join(str(Path.home()), ".config", "Code", "User", "globalStorage", "state.vscdb")
     */
    if cfg!(target_os = "windows") {
        // Windows
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
        // macOS
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
        // Linux 和其他类 Unix 系统
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

pub fn get_machine_id_path() -> String{
    /*
    python:
    if sys.platform == "win32":
        # Windows
        base_path = os.getenv("APPDATA", "")
        return os.path.join(base_path, "Code", "User", "machineid")
    elif sys.platform == "darwin":
        # macOS
        return os.path.join(str(Path.home()), "Library", "Application Support", "Code", "machineid")
    else:
        # Linux and other Unix-like systems
        return os.path.join(str(Path.home()), ".config", "Code", "machineid")
     */
    if cfg!(target_os = "windows") {
        // Windows
        let base_path = std::env::var("APPDATA").unwrap_or_default();
        std::path::Path::new(&base_path)
            .join("Code")
            .join("User")
            .join("machineid")
            .to_str()
            .unwrap()
            .to_string()
    } else if cfg!(target_os = "macos") {
        // macOS
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
        // Linux 和其他类 Unix 系统
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

pub fn get_workspace_storage_path() -> String{
    /*
    python:
    if sys.platform == "win32":
        # Windows
        base_path = os.getenv("APPDATA", "")
        return os.path.join(base_path, "Code", "User", "workspaceStorage")
    elif sys.platform == "darwin":
        # macOS
        return os.path.join(str(Path.home()), "Library", "Application Support", "Code", "User", "workspaceStorage")
    else:
        # Linux and other Unix-like systems
        return os.path.join(str(Path.home()), ".config", "Code", "User", "workspaceStorage") 
     */
    if cfg!(target_os = "windows") {
        // Windows
        let base_path = std::env::var("APPDATA").unwrap_or_default();
        std::path::Path::new(&base_path)
            .join("Code")
            .join("User")
            .join("workspaceStorage")
            .to_str()
            .unwrap()
            .to_string()
    } else if cfg!(target_os = "macos") {
        // macOS
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
        // Linux 和其他类 Unix 系统
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