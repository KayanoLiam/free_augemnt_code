
mod utils;
mod augment_utils;

use utils::paths::get_home_dir;
use augment_utils::json_modifier::modify_telemetry_ids;
use augment_utils::sqlite_modifier::clean_augment_data;
use augment_utils::workspace_cleaner::clean_workspace_storage;

fn main() {
    println!("System Paths:");
    println!("Home Directory:{}",get_home_dir());//用户的主目录
    println!("App data Directory:{}",utils::paths::get_app_data_dir());
    println!("Storage Directory:{}",utils::paths::get_storage_dir());
    println!("DB Path:{}",utils::paths::get_db_path());
    println!("Machine ID Path:{}",utils::paths::get_machine_id_path());
    println!("Workspace Storage Path: {}",utils::paths::get_workspace_storage_path());

    let result = modify_telemetry_ids();
    if !result["machine_id_backup_path"].is_empty(){
        println!("Machine ID backup created at: {}", result["machine_id_backup_path"]);
    }

    println!("Old IDs:");
    println!("Old Machine ID: {}", result["old_machine_id"]);
    println!("Old Device ID: {}", result["old_device_id"]);

    println!("New IDs:");
    println!("New Machine ID: {}", result["new_machine_id"]);
    println!("New Device ID: {}", result["new_device_id"]);

    println!("Cleaning SQLite Database:");
    let db_result = clean_augment_data();
    println!("Database backup created at: {}", db_result["db_backup_path"]);
    println!("Deleted rows: {}", db_result["deleted_rows"]);

    println!("Cleaning Workspace Storage:");
    let ws_result = clean_workspace_storage();
    println!("Workspace backup created at: {}", ws_result["backup_path"]);
    println!("Deleted files: {}", ws_result["deleted_files_count"]);

}

