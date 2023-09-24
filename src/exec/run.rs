use std::process::Stdio;
use crate::Error;
use std::process::Command;

pub fn run(cmd: Vec<String>, pwd: &str) -> Result<(), Error> {
    let mut cwd: String = pwd.to_string();
    for c in cmd {
        let line = c.clone();
        let elements = line.splitn(2, ' ');
        let el: Vec<&str> = elements.collect();
        if el[0] == "cd" {
            cwd = el[1].to_string();
            continue;
        }
        drop(el);
        let cmd: &mut Command = &mut Command::new("bash");
        cmd.arg("-c").arg(line);
        cmd.current_dir(cwd.as_str());

        let _ = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    }
    Ok(())
}

pub fn run_one(cmd: String, pwd: &str) -> Result<(), Error> {
    run(vec![cmd], pwd)
}

