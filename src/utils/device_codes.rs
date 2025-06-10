use rand::Rng;

/// 生成机器ID
///
/// 生成一个随机的64字符十六进制字符串作为机器ID。
/// 类似于在bash中使用/dev/urandom，但使用Rust的加密函数。
///
/// 返回值:
///     String: 64字符的十六进制字符串
///
/// 对应的Python代码:
/// ```python
/// random_bytes = secrets.token_bytes(32)
/// # 转换为十六进制字符串
/// return random_bytes.hex()
/// ```
pub fn generate_machine_id() -> String {
    // 创建随机数生成器
    let mut rng = rand::thread_rng();
    // 生成32字节的随机数据
    let mut random_bytes = [0u8; 32];
    rng.fill(&mut random_bytes);
    // 转换为十六进制字符串
    hex::encode(random_bytes)
}

pub fn generate_device_id() -> String {
    // 创建随机数生成器
    let mut rng = rand::thread_rng();
    // 生成32字节的随机数据
    let mut random_bytes = [0u8; 32];
    rng.fill(&mut random_bytes);
    // 转换为十六进制字符串
    hex::encode(random_bytes)
}