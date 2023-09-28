use lazy_static::lazy_static;

lazy_static! {
    pub static ref HOME: String = std::env::var("HOME").unwrap();

    pub static ref HAYASHI_DIR: String = format!("{}{}", *HOME, "/.saku");

    pub static ref CONFIG_NAME: String = "saku.yaml".to_string();
    pub static ref STORE_NAME: String = ".store.yaml".to_string();

    pub static ref REPO_DIR: String = format!("{}{}", *HAYASHI_DIR, "/repo");
    pub static ref PKG_DIR: String = format!("{}{}", *HAYASHI_DIR, "/flask");
    pub static ref ROOT_DIR: String = format!("{}{}", *HAYASHI_DIR, "/root");

    pub static ref REPO_SEED: String = "saku.yaml".to_string();
}
