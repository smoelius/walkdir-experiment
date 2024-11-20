use std::process::Command;
use walkdir::WalkDir;

fn main() {
    let status = Command::new("git")
        .args(["config", "--global", "core.protectNTFS", "false"])
        .status()
        .unwrap();
    assert!(status.success());
    let status = Command::new("git")
        .args(["clone", "https://github.com/g-plane/pretty_yaml.git"])
        .status()
        .unwrap();
    assert!(status.success());
    let mut count = 0;
    for result in WalkDir::new("pretty_yaml") {
        let entry = result.unwrap();
        let path = entry.path();
        println!("{}", path.display());
        count += 1;
    }
    println!("count = {count}");
}
