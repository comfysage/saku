use super::colors::{COLOR_BLUE, COLOR_CYAN, COLOR_MAGENTA, COLOR_RESET, COLOR_YELLOW};

pub mod general {
    use crate::util::msg::{COLOR_CYAN, COLOR_MAGENTA, COLOR_RESET, COLOR_YELLOW};
    pub fn name_f(name: &str) -> String {
        format!("{COLOR_MAGENTA}{}{COLOR_RESET}", name)
    }

    pub fn present_name(name: &str, group: &str) -> String {
        format!("{COLOR_CYAN}{}/{}", group, name_f(name))
    }

    pub fn url_f(url: &str) -> String {
        format!("{COLOR_YELLOW}{url}{COLOR_RESET}")
    }
}

pub fn clone(name: &str, url: &str) {
    println!(
        "{}",
        format!(
            "cloning {} from {} ...",
            general::name_f(name),
            general::url_f(url)
        )
    );
}

pub fn build(name: &str, path: &str) {
    println!(
        "{}",
        format!(
            "building {} at {COLOR_BLUE}{}{COLOR_RESET} ...",
            general::name_f(name),
            path
        )
    );
}

pub fn fetch(name: &str, url: &str) {
    println!(
        "{}",
        format!(
            "fetching {} from {} ...",
            general::name_f(name),
            general::url_f(url)
        )
    )
}

pub fn create_config(path: &str) {
    println!(
        "{}",
        format!(
            "creating config file at {COLOR_BLUE}{}{COLOR_RESET} ...",
            path
        )
    )
}

pub fn add(name: &str) {
    println!("{}", format!("adding  {}", general::name_f(name)))
}

pub fn remove(name: &str) {
    println!("{}", format!("removing  {}", general::name_f(name)))
}

pub fn add_flask(name: &str, url: &str) {
    println!("{}", format!("adding flask {} from {}", general::name_f(name), general::url_f(url)))
}

pub fn changelog(name: &str, path: &str) {
    println!("{}", format!("showing changes for {} at {COLOR_BLUE}{}{COLOR_RESET}", general::name_f(name), path))
}
