
extern crate dirs;
//
// use std::path::{Path, PathBuf};

const VERSION: &'static str = "1.16.3-1635814179000";



pub struct Driver {
    pub drive_directory: &str,
    pub version: &str,
    pub drive_binary_location: &str,
    pub skip_install_browsers: bool,
    pub browsers: Vec<&str>,
    pub verbose: bool,
}

impl Driver {
    /// default starts a Playwright instance with the default configuration
    fn default() -> Driver {
        let HOME_DIR = dirs::config_dir().unwrap();
        let drive_directory = HOME_DIR.join("playwright-rust");
        // drive_directory = drive_directory.join(VERSION.clone());
        let _drive_directory = drive_directory.join(VERSION.clone()).to_str().unwrap();
        Driver {
            drive_directory: _drive_directory.clone(),
            version: "0.0.1",
            drive_binary_location: _drive_directory.clone(),
            skip_install_browsers: false,
            browsers: vec![
                "chrome",
                "firefox",
                "safari",
                "opera",
                "edge",
            ],
            verbose: false,
        }
    }
}