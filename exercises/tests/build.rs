//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.


fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let your_command = format!("TEST_FOO={}", timestamp);
    println!("cargo:rustc-env={}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = "cargo:rustc-cfg=feature=\"pass\"";
    println!("{}", your_command);
}
