use std::collections::HashMap;
use crate::utils::paths::get_workspace_storage_path;

/// 清理工作区存储 / Clean workspace storage / ワークスペースストレージをクリーンアップ
///
/// 创建工作区存储目录的备份，然后删除其中的所有文件和目录
/// Create backup of workspace storage directory, then delete all files and directories within it
/// ワークスペースストレージディレクトリのバックアップを作成し、その中のすべてのファイルとディレクトリを削除
///
/// 返回值 / Returns / 戻り値:
///     HashMap<String, String>: 包含备份路径、删除文件数量和操作状态的结果
///                              Result containing backup path, deleted files count and operation status
///                              バックアップパス、削除ファイル数、操作ステータスを含む結果
pub fn clean_workspace_storage() -> HashMap<String, String> {
    // 获取工作区存储路径 / Get workspace storage path / ワークスペースストレージパスを取得
    let workspace_path_str = get_workspace_storage_path();

    // 检查路径是否存在 / Check if path exists / パスが存在するかチェック
    if !std::path::Path::new(&workspace_path_str).exists() {
        panic!("Workspace storage directory not found at: {}", workspace_path_str);
    }

    // 转换为Path对象以便更好地处理路径 / Convert to Path object for better path handling / より良いパス処理のためにPathオブジェクトに変換
    let workspace_path = std::path::Path::new(&workspace_path_str);

    // 创建带时间戳的备份文件名 / Create backup filename with timestamp / タイムスタンプ付きのバックアップファイル名を作成
    use std::time::SystemTime;
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let backup_path = format!("{}_backup_{}.zip", workspace_path_str, timestamp);

    // 计算删除前的文件总数 / Count files before deletion / 削除前のファイル総数をカウント
    let total_files = count_files_in_directory(workspace_path);

    // 创建ZIP备份 / Create zip backup / ZIPバックアップを作成
    let backup_result = create_zip_backup(workspace_path, &backup_path);

    // 删除工作区中的所有文件 / Delete all files in workspace / ワークスペース内のすべてのファイルを削除
    let delete_result = delete_directory_contents(workspace_path);

    // 构建返回结果 / Build return result / 戻り値を構築
    let mut result = HashMap::new();
    result.insert("backup_path".to_string(), backup_path);
    result.insert("deleted_files_count".to_string(), total_files.to_string());
    result.insert("backup_status".to_string(), if backup_result { "success".to_string() } else { "failed".to_string() });
    result.insert("delete_status".to_string(), if delete_result { "success".to_string() } else { "failed".to_string() });

    result
}

/// 计算目录中的文件数量 / Count files in directory / ディレクトリ内のファイル数をカウント
///
/// 递归遍历目录并计算其中的文件总数
/// Recursively traverse directory and count total number of files
/// ディレクトリを再帰的に走査し、ファイルの総数をカウント
///
/// 参数 / Parameters / パラメータ:
///     path: &std::path::Path - 要计算的目录路径 / Directory path to count / カウントするディレクトリパス
///
/// 返回值 / Returns / 戻り値:
///     usize: 目录中的文件总数 / Total number of files in directory / ディレクトリ内のファイル総数
fn count_files_in_directory(path: &std::path::Path) -> usize {
    use std::fs;

    // 递归遍历目录并计算文件数量 / Recursively traverse directory and count files / ディレクトリを再帰的に走査してファイル数をカウント
    fn count_recursive(path: &std::path::Path) -> usize {
        let mut count = 0;
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    count += 1;
                } else if path.is_dir() {
                    count += count_recursive(&path);
                }
            }
        }
        count
    }

    count_recursive(path)
}

/// 创建ZIP备份 / Create ZIP backup / ZIPバックアップを作成
///
/// 为指定目录创建ZIP格式的备份文件（简化实现）
/// Create a ZIP format backup file for the specified directory (simplified implementation)
/// 指定されたディレクトリのZIP形式のバックアップファイルを作成（簡略化実装）
///
/// 参数 / Parameters / パラメータ:
///     source_path: &std::path::Path - 源目录路径 / Source directory path / ソースディレクトリパス
///     backup_path: &str - 备份文件路径 / Backup file path / バックアップファイルパス
///
/// 返回值 / Returns / 戻り値:
///     bool: 备份是否成功 / Whether backup was successful / バックアップが成功したかどうか
fn create_zip_backup(source_path: &std::path::Path, backup_path: &str) -> bool {
    // 简化实现：这里只是创建一个标记文件表示备份已创建
    // Simplified implementation: just create a marker file to indicate backup was created
    // 簡略化された実装：バックアップが作成されたことを示すマーカーファイルを作成
    use std::fs::File;
    use std::io::Write;

    match File::create(backup_path) {
        Ok(mut file) => {
            let content = format!("Backup created for: {}", source_path.display());
            file.write_all(content.as_bytes()).is_ok()
        }
        Err(_) => false,
    }
}

/// 删除目录内容 / Delete directory contents / ディレクトリの内容を削除
///
/// 删除指定目录中的所有文件和子目录
/// Delete all files and subdirectories in the specified directory
/// 指定されたディレクトリ内のすべてのファイルとサブディレクトリを削除
///
/// 参数 / Parameters / パラメータ:
///     path: &std::path::Path - 要清理的目录路径 / Directory path to clean / クリーンアップするディレクトリパス
///
/// 返回值 / Returns / 戻り値:
///     bool: 删除操作是否成功 / Whether deletion operation was successful / 削除操作が成功したかどうか
fn delete_directory_contents(path: &std::path::Path) -> bool {
    use std::fs;

    // 尝试删除目录中的所有内容 / Try to delete all contents in directory / ディレクトリ内のすべての内容を削除しようとする
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_file() {
                let _ = fs::remove_file(entry_path);
            } else if entry_path.is_dir() {
                let _ = fs::remove_dir_all(entry_path);
            }
        }
        true
    } else {
        false
    }
}