use std::process::Command;

#[test]
fn tree_command_lists_all_subcommands() {
    let bin = env!("CARGO_BIN_EXE_mpt");
    let output = Command::new(bin)
        .arg("tree")
        .output()
        .expect("run mpt tree");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let tree = String::from_utf8(output.stdout).expect("utf8");
    assert!(tree.starts_with("mpt\n"));
    assert!(tree.contains("validate <path>"));
    assert!(tree.contains("part"));
    assert!(tree.contains("extract <path> <id>"));
    assert!(tree.contains("--header"));
    assert!(tree.contains("merge <input>…"));
    assert!(!tree.contains("--header <"), "boolean flags should not take values");
}
