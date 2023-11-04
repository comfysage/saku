use std::env;

use crate::prelude::*;

pub fn get_cwd() -> Result<String> {
    let working_dir = env::current_dir();
    Ok(working_dir.map_err(|_| make_err!())?.to_str().unwrap().to_owned())
}
