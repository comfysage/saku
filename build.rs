use std::io::Read;
use std::process::{Command, Stdio};

fn capture_command(command: &mut Command) -> String {
    let command = command
        .stdout(Stdio::piped()) // Capture stdout
        .spawn();

    // Check if the command was successfully created
    let mut child = match command {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            std::process::exit(1);
        }
    };

    // Read the captured stdout
    let mut output = String::new();
    child
        .stdout
        .take()
        .unwrap()
        .read_to_string(&mut output)
        .unwrap();

    // Wait for the command to finish and check its exit status
    let status = child.wait().expect("Failed to wait for child process");
    if status.success() {
        println!("Command executed successfully. Output:\n{}", output);
    } else {
        eprintln!("Command failed with exit code: {}", status);
    }

    output
}

struct Vars {
    repo_branch: String,
    commit_hash: String,
}

impl Vars {
    pub fn new() -> Self {
        let mut repo_branch_bind = Command::new("git");
        let repo_branch_cmd = repo_branch_bind.arg("branch").arg("--show-current");
        let mut commit_hash_bind = Command::new("git");
        let commit_hash_cmd = commit_hash_bind.arg("rev-parse").arg("HEAD");
        let mut repo_branch = capture_command(repo_branch_cmd);
        repo_branch = repo_branch.replace('\n', "");
        let mut commit_hash = capture_command(commit_hash_cmd);
        commit_hash = commit_hash.chars().take(7).collect();

        Self {
            repo_branch,
            commit_hash,
        }
    }
}

fn main() {
    let vars = Vars::new();
    println!("cargo:rustc-env=REPO_BRANCH={}", vars.repo_branch);
    println!("cargo:rustc-env=COMMIT_HASH={}", vars.commit_hash);
    let mut pkg_version = format!("{}-{}", vars.repo_branch, vars.commit_hash);
    if std::env::var("PROFILE").unwrap() == "debug" {
        pkg_version = format!("{pkg_version}-debug");
    }
    println!("cargo:rustc-env=PKG_VERSION={}", pkg_version);
}
