use saku_lib::pkg::data;
use saku_lib::pkg::flask::Flask;
use saku_lib::prelude::*;
use saku_lib::util::msg;

pub fn list() -> Result<()> {
    let flask_files = data::get_flasks()?;

    let flasks: Vec<Result<Flask>> = flask_files
        .iter()
        .map(|f| data::get_flask_from_name(f))
        .collect();

    for f in &flasks {
        if let Ok(flask) = f {
            println!(" - {} from {}", msg::general::name_f(&flask.name()), msg::general::url_f(&flask.url()));
        } else if let Err(err) = f {
            warn!("error while listing flask: {err}");
        }
    }

    Ok(())
}
