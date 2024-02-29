use saku_lib::prelude::*;

use saku_lib::exec;
use saku_lib::util::msg;
use saku_lib::util::path;

pub fn changelog(name: &str) -> Result<()> {
    msg::changelog(name, &path::repo(name));

    exec::Cmd::Log{ name: name.to_string() }.exec()?;

    Ok(())
}
