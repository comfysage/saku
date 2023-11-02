use saku_lib::pkg::data;
use saku_lib::pkg::flask::Flask;

use saku_lib::Result;

pub fn add_with_name(name: &str, url: &str) -> Result<()> {
    let flask = Flask::new(name, url)?;

    // install flask to flask/flasks/name
    data::save_pkg(flask.pkg())?;

    // install seeds to flask/name
    flask.link()?;

    Ok(())
}

pub fn add(url: &str) -> Result<()> {
    let flask = Flask::from_url(url)?;

    // install flask to flask/flasks/owner.repo
    data::save_pkg(flask.pkg())?;

    // install seeds to flask/owner.repo
    flask.link()?;

    Ok(())
}

pub fn update(url: &str) -> Result<()> {
    let flask = data::get_flask(url)?;

    crate::update::update_flask(&flask)?;

    Ok(())
}
