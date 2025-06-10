
// 工具模块 / Utilities module / ユーティリティモジュール
mod utils;
// 扩展工具模块 / Augment utilities module / 拡張ユーティリティモジュール
mod augment_utils;

// 导入路径工具函数 / Import path utility functions / パスユーティリティ関数をインポート
use utils::paths::get_home_dir;
// 导入JSON修改器 / Import JSON modifier / JSON修正器をインポート
use augment_utils::json_modifier::modify_telemetry_ids;
// 导入SQLite清理器 / Import SQLite cleaner / SQLiteクリーナーをインポート
use augment_utils::sqlite_modifier::clean_augment_data;
// 导入工作区清理器 / Import workspace cleaner / ワークスペースクリーナーをインポート
use augment_utils::workspace_cleaner::clean_workspace_storage;

/// 主函数 / Main function / メイン関数
///
/// 程序的入口点，执行系统路径获取和数据清理操作
/// Entry point of the program, performs system path retrieval and data cleaning operations
/// プログラムのエントリーポイント、システムパス取得とデータクリーニング操作を実行
fn main() {
    // 打印系统路径信息 / Print system path information / システムパス情報を出力
    println!("System Paths:");
    // 用户主目录 / User home directory / ユーザーホームディレクトリ
    println!("Home Directory: {}", get_home_dir());
    // 应用数据目录 / Application data directory / アプリケーションデータディレクトリ
    println!("App data Directory: {}", utils::paths::get_app_data_dir());
    // 存储目录 / Storage directory / ストレージディレクトリ
    println!("Storage Directory: {}", utils::paths::get_storage_dir());
    // 数据库路径 / Database path / データベースパス
    println!("DB Path: {}", utils::paths::get_db_path());
    // 机器ID路径 / Machine ID path / マシンIDパス
    println!("Machine ID Path: {}", utils::paths::get_machine_id_path());
    // 工作区存储路径 / Workspace storage path / ワークスペースストレージパス
    println!("Workspace Storage Path: {}", utils::paths::get_workspace_storage_path());

    // 修改遥测ID / Modify telemetry IDs / テレメトリIDを変更
    let result = modify_telemetry_ids();
    if !result["machine_id_backup_path"].is_empty() {
        println!("Machine ID backup created at: {}", result["machine_id_backup_path"]);
    }

    // 打印旧ID信息 / Print old ID information / 古いID情報を出力
    println!("Old IDs:");
    println!("Old Machine ID: {}", result["old_machine_id"]);
    println!("Old Device ID: {}", result["old_device_id"]);

    // 打印新ID信息 / Print new ID information / 新しいID情報を出力
    println!("New IDs:");
    println!("New Machine ID: {}", result["new_machine_id"]);
    println!("New Device ID: {}", result["new_device_id"]);

    // 清理SQLite数据库 / Clean SQLite database / SQLiteデータベースをクリーンアップ
    println!("Cleaning SQLite Database:");
    let db_result = clean_augment_data();
    println!("Database backup created at: {}", db_result["db_backup_path"]);
    println!("Deleted rows: {}", db_result["deleted_rows"]);

    // 清理工作区存储 / Clean workspace storage / ワークスペースストレージをクリーンアップ
    println!("Cleaning Workspace Storage:");
    let ws_result = clean_workspace_storage();
    println!("Workspace backup created at: {}", ws_result["backup_path"]);
    println!("Deleted files: {}", ws_result["deleted_files_count"]);
}

