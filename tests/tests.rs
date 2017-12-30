extern crate cli_test_dir;

use cli_test_dir::*;

#[test]
fn analyze_wasm_file() {
    let testdir = TestDir::new("wasm-bloat", "analyze_wasm_file");
    let output = testdir.cmd()
        .arg(testdir.src_path("fixtures/double.wasm"))
        .output()
        .expect_success();
    assert!(output.stdout_str().contains("double"));
}
