use saku_lib::pkg::data;
use saku_lib::pkg::flask::Flask;
use saku_lib::util::msg;
use saku_lib::Result;

pub fn update_flask(flask: &Flask) -> Result<()> {
    msg::fetch(&flask.name(), &flask.url());

    flask.update()?;

    Ok(())
}

pub fn update_flask_from_url(url: &String) -> Result<()> {
    let flask = data::get_flask(&url)?;
    update_flask(&flask)?;
    Ok(())
}

fn update_flask_from_name(name: &String) -> Result<()> {
    let flask = data::get_flask_from_name(&name)?;
    update_flask(&flask)?;
    Ok(())
}

pub fn update() -> Result<()> {
    let flasks = data::get_flasks()?;
    flasks
        .iter()
        .map::<Result<()>, fn(&String) -> Result<()>>(update_flask_from_name)
        .collect::<Result<()>>()?;

    Ok(())
}
