use crate::utils::paths::{get_machine_id_path, get_storage_dir};
use crate::utils::device_codes::{generate_device_id, generate_machine_id};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

/// 修改遥测ID / Modify telemetry IDs / テレメトリIDを変更
///
/// 修改VS Code存储文件中的遥测机器ID和设备ID，并创建备份
/// Modify telemetry machine ID and device ID in VS Code storage file and create backups
/// VS Codeストレージファイル内のテレメトリマシンIDとデバイスIDを変更し、バックアップを作成
///
/// 返回值 / Returns / 戻り値:
///     HashMap<String, String>: 包含旧ID、新ID和备份路径的结果 / Result containing old IDs, new IDs and backup paths / 古いID、新しいID、バックアップパスを含む結果
pub fn modify_telemetry_ids() -> HashMap<String, String> {
    // 获取存储路径和机器ID路径 / Get storage path and machine ID path / ストレージパスとマシンIDパスを取得
    let storage_path = get_storage_dir();
    let machine_id_path = get_machine_id_path();

    // 检查storage_path是否存在 / Check if storage_path exists / storage_pathが存在するかチェック
    if !std::path::Path::new(&storage_path).exists() {
        panic!("Storage file not found at: {}", storage_path);
    }

    // 在修改前创建备份 / Create backups before modification / 変更前にバックアップを作成
    let storage_backup_path = create_backup(&storage_path);
    let machine_id_backup_path = if std::path::Path::new(&machine_id_path).exists() {
        Some(create_backup(&machine_id_path))
    } else {
        None
    };

    // 读取当前JSON内容 / Read the current JSON content / 現在のJSONコンテンツを読み取り
    let mut file = File::open(&storage_path).expect("Failed to open storage file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file contents");
    let mut data: serde_json::Value = serde_json::from_str(&contents).expect("Failed to parse JSON");

    // 获取旧的ID值 / Get old ID values / 古いID値を取得
    let old_machine_id = data.get("telemetry.machineId").unwrap().as_str().unwrap().to_string();
    let old_device_id = data.get("telemetry.devDeviceId").unwrap().as_str().unwrap().to_string();

    // 更新storage.json中的值 / Update the values in storage.json / storage.json内の値を更新
    let new_machine_id = generate_machine_id();
    let new_device_id = generate_device_id();

    data["telemetry.machineId"] = serde_json::Value::String(new_machine_id.clone());
    data["telemetry.devDeviceId"] = serde_json::Value::String(new_device_id.clone());

    // 将修改后的内容写回storage.json / Write the modified content back to storage.json / 変更されたコンテンツをstorage.jsonに書き戻し
    let updated_json = serde_json::to_string_pretty(&data).expect("Failed to serialize JSON");
    std::fs::write(&storage_path, updated_json).expect("Failed to write storage file");

    // 将新的机器ID写入机器ID文件 / Write the new machine ID to the machine ID file / 新しいマシンIDをマシンIDファイルに書き込み
    let mut file = File::create(&machine_id_path).expect("Failed to create machine ID file");
    file.write_all(new_device_id.as_bytes())
        .expect("Failed to write machine ID");

    // 构建返回结果 / Build return result / 戻り値を構築
    let mut result = HashMap::new();
    result.insert("old_machine_id".to_string(), old_machine_id);
    result.insert("new_machine_id".to_string(), new_machine_id);
    result.insert("old_device_id".to_string(), old_device_id);
    result.insert("new_device_id".to_string(), new_device_id);
    result.insert("storage_backup_path".to_string(), storage_backup_path);
    result.insert("machine_id_backup_path".to_string(),
        machine_id_backup_path.unwrap_or_else(|| "".to_string()));
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
fn create_backup(file_path: &str) -> String {
    use std::fs::copy;
    use std::time::SystemTime;

    // 获取当前时间戳 / Get current timestamp / 現在のタイムスタンプを取得
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 构建备份文件路径 / Build backup file path / バックアップファイルパスを構築
    let backup_path = format!("{}.bak.{}", file_path, timestamp);

    // 复制文件到备份位置 / Copy file to backup location / ファイルをバックアップ場所にコピー
    copy(file_path, &backup_path).expect("Failed to create backup file");

    backup_path
}
