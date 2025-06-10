use crate::utils::paths::{get_machine_id_path, get_storage_dir};
use crate::utils::device_codes::{generate_device_id, generate_machine_id};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};


pub fn modify_telemetry_ids() -> HashMap<String, String> {
    /*
    python:
    storage_path = get_storage_path()
    machine_id_path = get_machine_id_path()

    if not os.path.exists(storage_path):
        raise FileNotFoundError(f"Storage file not found at: {storage_path}")

    # Create backups before modification
    storage_backup_path = _create_backup(storage_path)
    machine_id_backup_path = None
    if os.path.exists(machine_id_path):
        machine_id_backup_path = _create_backup(machine_id_path)

    # Read the current JSON content
    with open(storage_path, 'r', encoding='utf-8') as f:
        data = json.load(f)

    # Store old values
    old_machine_id = data.get('telemetry.machineId', '')
    old_device_id = data.get('telemetry.devDeviceId', '')

    # Generate new IDs
    new_machine_id = generate_machine_id()
    new_device_id = generate_device_id()

    # Update the values in storage.json
    data['telemetry.machineId'] = new_machine_id
    data['telemetry.devDeviceId'] = new_device_id

    # Write the modified content back to storage.json
    with open(storage_path, 'w', encoding='utf-8') as f:
        json.dump(data, f, indent=4)

    # Write the new machine ID to the machine ID file
    with open(machine_id_path, 'w', encoding='utf-8') as f:
        f.write(new_device_id)

    return {
        'old_machine_id': old_machine_id,
        'new_machine_id': new_machine_id,
        'old_device_id': old_device_id,
        'new_device_id': new_device_id,
        'storage_backup_path': storage_backup_path,
        'machine_id_backup_path': machine_id_backup_path
    }
     */
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
    //Read the current JSON content
    let mut file = File::open(&storage_path).expect("无法打开存储文件");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("无法读取文件内容");
    let mut data: serde_json::Value = serde_json::from_str(&contents).expect("无法解析 JSON");

    let old_machine_id = data.get("telemetry.machineId").unwrap().as_str().unwrap().to_string();
    let old_device_id = data.get("telemetry.devDeviceId").unwrap().as_str().unwrap().to_string();

    //update the values in storage.json
    let new_machine_id = generate_machine_id();
    let new_device_id = generate_device_id();

    data["telemetry.machineId"] = serde_json::Value::String(new_machine_id.clone());
    data["telemetry.devDeviceId"] = serde_json::Value::String(new_device_id.clone());

    //Write the modified content back to storage.json
    let updated_json = serde_json::to_string_pretty(&data).expect("无法序列化JSON");
    std::fs::write(&storage_path, updated_json).expect("无法写入存储文件");

    //Write the new machine ID to the machine ID file
    let mut file = File::create(&machine_id_path).expect("无法创建machine ID文件");
    file.write_all(new_device_id.as_bytes())
        .expect("无法写入machine ID");

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

/// 创建文件备份
/// Create file backup
/// ファイルバックアップを作成
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
    copy(file_path, &backup_path).expect("无法创建备份文件");

    backup_path
}
