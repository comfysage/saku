use crate::pkg::data;
use crate::Result;
use crate::pkg::flask::Flask;
use crate::util::msg;

pub fn update_flask(flask: &Flask) -> Result<()> {
    msg::fetch(&flask.name(), &flask.url());

    flask.update()?;

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
