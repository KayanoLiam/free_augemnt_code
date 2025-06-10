use rand::Rng;

/// 生成机器ID / Generate machine ID / マシンIDを生成
///
/// 生成一个随机的64字符十六进制字符串作为机器ID
/// Generate a random 64-character hexadecimal string as machine ID
/// マシンIDとしてランダムな64文字の16進文字列を生成
///
/// 类似于在bash中使用/dev/urandom，但使用Rust的随机数生成器
/// Similar to using /dev/urandom in bash, but uses Rust's random number generator
/// bashで/dev/urandomを使用するのと似ているが、Rustの乱数生成器を使用
///
/// 返回值 / Returns / 戻り値:
///     String: 64字符的十六进制字符串 / 64-character hexadecimal string / 64文字の16進文字列
pub fn generate_machine_id() -> String {
    // 创建随机数生成器 / Create random number generator / 乱数生成器を作成
    let mut rng = rand::thread_rng();
    // 生成32字节的随机数据 / Generate 32 bytes of random data / 32バイトのランダムデータを生成
    let mut random_bytes = [0u8; 32];
    rng.fill(&mut random_bytes);
    // 转换为十六进制字符串 / Convert to hexadecimal string / 16進文字列に変換
    hex::encode(random_bytes)
}

/// 生成设备ID / Generate device ID / デバイスIDを生成
///
/// 生成一个随机的64字符十六进制字符串作为设备ID
/// Generate a random 64-character hexadecimal string as device ID
/// デバイスIDとしてランダムな64文字の16進文字列を生成
///
/// 返回值 / Returns / 戻り値:
///     String: 64字符的十六进制字符串 / 64-character hexadecimal string / 64文字の16進文字列
pub fn generate_device_id() -> String {
    // 创建随机数生成器 / Create random number generator / 乱数生成器を作成
    let mut rng = rand::thread_rng();
    // 生成32字节的随机数据 / Generate 32 bytes of random data / 32バイトのランダムデータを生成
    let mut random_bytes = [0u8; 32];
    rng.fill(&mut random_bytes);
    // 转换为十六进制字符串 / Convert to hexadecimal string / 16進文字列に変換
    hex::encode(random_bytes)
}