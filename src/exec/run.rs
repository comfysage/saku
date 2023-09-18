use std::process::Stdio;
use crate::Error;
use std::process::Command;

pub fn run(cmd: Vec<String>, pwd: String) -> Result<(), Error> {
    let mut cwd = pwd;
    drop(pwd);
    for c in cmd {
        let el: Vec<&str> = c.splitn(2, ' ').collect();
        if el[0] == "cd" {
            // el[0] = cwd;
            cwd = el[1].to_string();
            continue;
        }
        drop(el);
        let cmd = Command::new("bash").arg("-c").arg(c);
        cmd.current_dir(cwd);

        let child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    }
    Ok(())
}

pub fn run_one(cmd: String, pwd: String) -> Result<(), Error> {
    run(vec![cmd], pwd)
}

