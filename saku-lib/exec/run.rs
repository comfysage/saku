use crate::prelude::*;
use crate::util::constants;

use std::collections::HashMap;
use std::io::{Read, stdout, Write, IoSlice};
use std::process::{Command, Stdio};
use std::thread;

/// Pipe streams are blocking, we need separate threads to monitor them without blocking the primary thread.
fn child_stream_to_vec<R>(mut stream: R) -> Result<()>
where
    R: Read + Send + 'static,
{
    let out = stdout();
    thread::Builder::new()
        .name("child_stream_to_vec".into())
        .spawn(move || loop {
            let mut buf = [0];
            match stream.read(&mut buf) {
                Err(err) => {
                    println!("{}] Error reading from stream: {}", line!(), err);
                    break;
                }
                Ok(got) => {
                    if got == 0 {
                        break;
                    } else if got == 1 {
                        let value = buf[0];
                        out.lock().write_vectored(&[IoSlice::new(&[value])]).unwrap();
                    } else {
                        println!("{}] Unexpected number of bytes: {}", line!(), got);
                        break;
                    }
                }
            }
        })?;
    Ok(())
}

pub fn run(cmd: Vec<String>, pwd: &str) -> Result<()> {
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
        cmd.envs(env_vars());

        let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
        child_stream_to_vec(child.stdout.take().expect("!stdout"))?;
        child_stream_to_vec(child.stderr.take().expect("!stderr"))?;
        child.wait()?;
    }
    Ok(())
}

pub fn run_one(cmd: String, pwd: &str) -> Result<()> {
    run(vec![cmd], pwd)
}

fn env_vars() -> HashMap<String, String> {
    let mut vars = HashMap::new();
    vars.insert("SAKUPATH".to_string(), (&*constants::SAKU_DIR).into());
    let log_env = ::std::env::var("RUST_LOG").unwrap_or("info".to_string());
    vars.insert("RUST_LOG".to_string(), log_env);
    vars
}
