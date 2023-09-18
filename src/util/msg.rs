use super::colors::{COLOR_BLUE, COLOR_YELLOW, COLOR_RESET, COLOR_MAGENTA};

pub fn clone(name: &str, url: &str) {
    println!("{}", format!(
        "cloning {COLOR_MAGENTA}{}{COLOR_RESET} from {COLOR_YELLOW}{}{COLOR_RESET} ...",
        name, url
    ));
}

pub fn build(name: &str, path: &str) {
    println!("{}", format!(
        "building {COLOR_MAGENTA}{}{COLOR_RESET} at {COLOR_BLUE}{}{COLOR_RESET} ...",
        name, path
    ));
}
