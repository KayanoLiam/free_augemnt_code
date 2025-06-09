
mod utils;

use utils::paths::get_home_dir;

fn main() {
    println!("System Paths:");
    println!("Home Directory:{}",get_home_dir());//用户的主目录
    println!("App data Directory:{}",utils::paths::get_app_data_dir());
    println!("Storage Directory:{}",utils::paths::get_storage_dir());
    println!("DB Path:{}",utils::paths::get_db_path());
    println!("Machine ID Path:{}",utils::paths::get_machine_id_path());
    println!("Workspace Storage Path: {}",utils::paths::get_workspace_storage_path());
}
