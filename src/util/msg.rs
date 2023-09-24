use super::colors::{COLOR_BLUE, COLOR_CYAN, COLOR_MAGENTA, COLOR_RESET, COLOR_YELLOW};

pub fn clone(name: &str, url: &str) {
    println!(
        "{}",
        format!(
            "cloning {COLOR_MAGENTA}{}{COLOR_RESET} from {COLOR_YELLOW}{}{COLOR_RESET} ...",
            name, url
        )
    );
}

pub fn build(name: &str, path: &str) {
    println!(
        "{}",
        format!(
            "building {COLOR_MAGENTA}{}{COLOR_RESET} at {COLOR_BLUE}{}{COLOR_RESET} ...",
            name, path
        )
    );
}

pub fn fetch(name: &str, url: &str) {
    println!(
        "{}",
        format!(
            "fetching {COLOR_MAGENTA}{}{COLOR_RESET} from {COLOR_YELLOW}{}{COLOR_RESET} ...",
            name, url
        )
    )
}

pub fn create_config(path: &str) {
    println!(
        "{}",
        format!(
            "creating config file at {COLOR_CYAN}{}{COLOR_RESET} ...",
            path
        )
    )
}
