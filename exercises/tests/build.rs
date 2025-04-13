//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

/* fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    let your_command = format!(
        "Your command here with {}, please checkout exercises/tests/build.rs",
        timestamp
    );
    println!("cargo:{}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = "Your command here, please checkout exercises/tests/build.rs";
    println!("cargo:{}", your_command);
}
 */
/* use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("获取系统时间失败")
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
} */
// build.rs
fn main() {
    // 告诉 Cargo 在编译时启用 feature = "pass" 的条件编译标志
    println!("cargo:rustc-cfg=feature=\"pass\"");
}