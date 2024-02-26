use log::info;
use super::colors::{COLOR_BLUE, COLOR_CYAN, COLOR_MAGENTA, COLOR_RESET, COLOR_YELLOW};

pub mod general {
    use super::{COLOR_BLUE, COLOR_CYAN, COLOR_MAGENTA, COLOR_RESET, COLOR_YELLOW};
    pub fn name_f(name: &str) -> String {
        format!("{COLOR_MAGENTA}{}{COLOR_RESET}", name)
    }

    pub fn present_name(name: &str, group: &str) -> String {
        format!("{COLOR_CYAN}{}/{}", group, name_f(name))
    }

    pub fn url_f(url: &str) -> String {
        format!("{COLOR_YELLOW}{url}{COLOR_RESET}")
    }

    pub fn path_f(path: &str) -> String {
        format!("{COLOR_BLUE}{path}{COLOR_RESET}")
    }
}

pub fn clone(name: &str, url: &str) {
    info!(
        "{}",
        format!(
            "cloning {} from {} ...",
            general::name_f(name),
            general::url_f(url)
        )
    );
}

pub fn build(name: &str, path: &str) {
    info!(
        "{}",
        format!(
            "building {} at {} ...",
            general::name_f(name),
            general::path_f(path)
        )
    );
}

pub fn fetch(name: &str, url: &str) {
    info!(
        "{}",
        format!(
            "fetching {} from {} ...",
            general::name_f(name),
            general::url_f(url)
        )
    )
}

pub fn create_config(path: &str) {
    info!(
        "{}",
        format!(
            "creating config file at {} ...",
            general::path_f(path)
        )
    )
}

pub fn add(name: &str) {
    info!("{}", format!("adding  {}", general::name_f(name)))
}

pub fn remove(name: &str) {
    info!("{}", format!("removing  {}", general::name_f(name)))
}

pub fn add_flask(name: &str, url: &str) {
    info!("{}", format!("adding flask {} from {}", general::name_f(name), general::url_f(url)))
}

pub fn changelog(name: &str, path: &str) {
    info!("{}", format!("showing changes for {} at {}", general::name_f(name), general::path_f(path)))
}

pub fn link(target: &str, path: &str) {
    info!("{}", format!("linking {} to {}", general::path_f(path), general::path_f(target)))
}

pub fn remove_file(path: &str) {
    info!("{}", format!("removing {}", general::path_f(path)))
}
