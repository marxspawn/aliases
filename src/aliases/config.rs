use std::env;
use std::path::{PathBuf};
use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json;
use std::process::Command;

#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct Config {
    pub shim_directory: String,
    alias_directories: Vec<String>,
    users: Vec<String>,
}

impl Config {

    pub fn load() -> Config {
        if Config::config_file_path().exists() {
            Config::load_file(&Config::config_file_path())
        } else {
            Config::create(&Config::config_file_path())
        }
    }

    pub fn shim_path(&self) -> PathBuf {
        let mut command = String::from("echo \"");
        command.push_str(&self.shim_directory);
        command.push_str("\"");
        let output = Command::new("bash")
            .arg("-c")
            .arg(&command)
            .output()
            .unwrap_or_else(|e| { panic!("failed to execute child: {}", e) });

        PathBuf::from(String::from_utf8(output.stdout).unwrap().trim())
    }

    pub fn alias_paths(&self) -> Vec<PathBuf> {
        self.alias_directories.iter().map(|dir| PathBuf::from(dir)).collect()
    }

    pub fn users(&self) -> Vec<String> {
        self.users.clone()
    }

    pub fn add_alias_directory(&mut self, directory: &PathBuf) {
        let string = String::from(directory.to_str().unwrap());
        self.alias_directories.push(string);
        self.alias_directories.dedup();
        self.update_file();
    }

    // ------- private methods --------//

    fn config_file_path() -> PathBuf {
        match env::var("HOME") {
            Ok(home_dir) => {
                PathBuf::from(home_dir).join(".aliases_cfg")
            },
            Err(_) => {
                PathBuf::new() // TODO need to handle this better
            },
        }
    }

    fn create(path: &PathBuf) -> Config {
        let mut file = File::create(path).unwrap(); //TODO handle the error case
        let default_config = TemplateRepository::config_template();
        file.write_all(default_config.as_bytes()).unwrap();
        let config: Config = json::decode(&default_config).unwrap();
        config
    }

    fn load_file(path: &PathBuf) -> Config {
        let mut file = File::open(path).unwrap(); //TODO handle the error case
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);
        let config: Config = json::decode(&content).unwrap();
        config
    }

    fn update_file(&self) {
        let mut file = File::create(Config::config_file_path()).unwrap(); //TODO handle the error case
        let encoded = json::encode(&self).unwrap();
        file.write_all(encoded.as_bytes()).unwrap();
    }
}

struct TemplateRepository;

impl TemplateRepository {

    pub fn config_template() -> String {
        "{
            \"shim_directory\" : \"${HOME}/.aliases.d/shims\",
            \"alias_directories\" : [],
            \"users\" : [\"default\"]
        }
        ".to_string()
    }
}
