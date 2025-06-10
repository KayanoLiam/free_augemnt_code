use std::collections::HashMap;

use crate::utils::paths::get_db_path;

/// 清理Augment相关数据 / Clean Augment related data / Augment関連データをクリーンアップ
///
/// 从SQLite数据库中删除所有包含'augment'关键字的记录，并创建备份
/// Delete all records containing 'augment' keyword from SQLite database and create backup
/// SQLiteデータベースから'augment'キーワードを含むすべてのレコードを削除し、バックアップを作成
///
/// 返回值 / Returns / 戻り値:
///     HashMap<String, String>: 包含备份路径和删除行数的结果 / Result containing backup path and deleted rows count / バックアップパスと削除行数を含む結果
pub fn clean_augment_data() -> HashMap<String, String> {
    // 获取数据库路径 / Get database path / データベースパスを取得
    let db_path = get_db_path();

    // 在修改前创建备份 / Create backup before modification / 変更前にバックアップを作成
    let db_backup_path = create_buckup(&db_path);

    // 连接到数据库 / Connect to database / データベースに接続
    let conn = rusqlite::Connection::open(&db_path).expect("Failed to open database");

    // 执行删除操作 / Execute delete operation / 削除操作を実行
    let deleted_rows = {
        // 准备SQL语句 / Prepare SQL statement / SQL文を準備
        let mut stmt = conn.prepare("DELETE FROM ItemTable WHERE key LIKE '%augment%'").expect("Failed to prepare SQL statement");
        // 执行删除查询 / Execute delete query / 削除クエリを実行
        stmt.execute([]).expect("Failed to execute SQL statement")
    };

    // 显式关闭数据库连接 / Explicitly close database connection / データベース接続を明示的に閉じる
    drop(conn);

    // 构建返回结果 / Build return result / 戻り値を構築
    let mut result = HashMap::new();
    result.insert("db_backup_path".to_string(), db_backup_path);
    result.insert("deleted_rows".to_string(), deleted_rows.to_string());
    result
}

/// 创建文件备份 / Create file backup / ファイルバックアップを作成
///
/// 使用时间戳创建文件的备份副本
/// Create a backup copy of the file using timestamp
/// タイムスタンプを使用してファイルのバックアップコピーを作成
///
/// 参数 / Parameters / パラメータ:
///     file_path: &str - 要备份的文件路径 / File path to backup / バックアップするファイルパス
///
/// 返回值 / Returns / 戻り値:
///     String: 备份文件的路径 / Path of the backup file / バックアップファイルのパス
fn create_buckup(file_path: &str) -> String {
    use std::fs::copy;
    use std::time::SystemTime;

    // 获取当前时间戳 / Get current timestamp / 現在のタイムスタンプを取得
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    // 构建备份文件路径 / Build backup file path / バックアップファイルパスを構築
    let backup_path = format!("{}.bak.{}", file_path, timestamp);

    // 复制文件到备份位置 / Copy file to backup location / ファイルをバックアップ場所にコピー
    copy(file_path, &backup_path).expect("Failed to create backup file");

    backup_path
}