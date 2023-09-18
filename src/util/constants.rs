pub static HOME: &str = std::env::var("HOME").unwrap().as_str();

pub static HAYASHI_ROOT: &str = format!("{}{}", HOME, "/.hayashi").as_str();
pub static CONFIG_NAME: &str = ".hayashi.yaml";
pub static STORE_NAME: &str = ".store.yaml";

pub static REPO_ROOT: &str = format!("{}{}", HAYASHI_ROOT, "/repo").as_str();
pub static PKG_ROOT: &str = format!("{}{}", HAYASHI_ROOT, "/pkg").as_str();
pub static PACK_ROOT: &str = format!("{}{}", HAYASHI_ROOT, "/pack").as_str();

pub static REPO_PKG: &str = "yuki.yaml";
