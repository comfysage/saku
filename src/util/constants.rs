use lazy_static::lazy_static;

lazy_static! {
    pub static ref HOME: String = std::env::var("HOME").unwrap();

    pub static ref HAYASHI_ROOT: String = format!("{}{}", *HOME, "/.hayashi");

    pub static ref CONFIG_NAME: String = ".hayashi.yaml".to_string();
    pub static ref STORE_NAME: String = ".store.yaml".to_string();

    pub static ref REPO_ROOT: String = format!("{}{}", *HAYASHI_ROOT, "/repo");
    pub static ref PKG_ROOT: String = format!("{}{}", *HAYASHI_ROOT, "/pkg");
    pub static ref PACK_ROOT: String = format!("{}{}", *HAYASHI_ROOT, "/pack");

    pub static ref REPO_PKG: String = "yuki.yaml".to_string();
}
