use lazy_static::lazy_static;

lazy_static! {
    pub static ref HOME: String = std::env::var("HOME").unwrap();
    pub static ref SAKU_PATH: String = std::env::var("SAKUPATH").unwrap_or(format!("{}{}", *HOME, "/.saku"));

    pub static ref SAKU_DIR: String = SAKU_PATH.to_string();

    pub static ref CONFIG_NAME: String = "saku.toml".to_string();
    pub static ref STORE_NAME: String = ".store.yaml".to_string();

    pub static ref REPO_DIR: String = format!("{}{}", *SAKU_DIR, "/repo");
    pub static ref PKG_DIR: String = format!("{}{}", *SAKU_DIR, "/flask");
    pub static ref ROOT_DIR: String = format!("{}{}", *SAKU_DIR, "/root");
    pub static ref STORE_DIR: String = format!("{}{}", *SAKU_DIR, "/store");
    pub static ref LIB_DIR_NAME: String = "lib".to_string();
    pub static ref LIB_DIR: String = format!("{}/{}", *SAKU_DIR, *LIB_DIR_NAME);

    pub static ref FLASK_DIR_NAME: String = "flasks".to_string();
    pub static ref FLASK_DIR: String = format!("{}/{}", *PKG_DIR, *FLASK_DIR_NAME);

    pub static ref INIT_FILE_NAME: String = "init.fl".to_string();
    pub static ref INIT_FILE: String = format!("{}/{}", *LIB_DIR, *INIT_FILE_NAME);

    pub static ref REPO_SEED: String = "pkg.fl".to_string();
}
