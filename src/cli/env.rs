use crate::util::path;
use crate::Result;

pub fn env() -> Result<()> {
    let script = format!(
        "\
export PATH=\"$PATH:{}\"
export XDG_DATA_DIRS=\"${{XDG_DATA_DIRS:-/usr/share:/usr/local/share}}:{}\"",
        path::root_dir("bin"),
        path::root_dir("share")
    );
    println!("{script}");
    Ok(())
}
