use std::{
    env::var,
    fs::{self, OpenOptions},
    io::Write,
};

use sysinfo::{ProcessExt, Signal, System, SystemExt};
pub struct Quickiller {
    config: String,
}

impl Quickiller {
    pub fn new(config: String) -> Quickiller {
        Quickiller { config }
    }

    fn get_exe(&self, profile: Option<String>) -> Vec<String> {
        let config_file = match profile {
            Some(profile) => {
                format!(
                    "{}/quickiller_profiles/{}.txt",
                    var("USERPROFILE").unwrap(),
                    profile
                )
            }
            None => self.config.to_string(),
        };
        let file = fs::read_to_string(config_file).unwrap();
        let exe_files = file.split(' ').into_iter();
        let mut exe_files_vec = Vec::new();
        for i in exe_files {
            exe_files_vec.push(i.to_string());
        }
        exe_files_vec
    }

    pub fn kill(&self, profile: Option<String>) {
        let files = self.get_exe(profile);
        let mut sys = System::new_all();
        sys.refresh_all();
        for (_, process) in sys.processes() {
            if files.contains(&process.name().to_string()) {
                process.kill(Signal::Kill);
            }
        }
    }

    pub fn add(&self, exe_name: String, profile: Option<String>) {
        let config_file = match profile {
            Some(profile) => {
                format!(
                    "{}/quickiller_profiles/{}.txt",
                    var("USERPROFILE").unwrap(),
                    profile
                )
            }
            None => self.config.to_string(),
        };
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(config_file)
            .unwrap();
        write!(file, " {}", exe_name).unwrap();
        println!("Successfully added {}", exe_name);
    }

    pub fn remove(&self, exe_name: String, profile: Option<String>) {
        let config_file = match profile {
            Some(profile) => {
                format!(
                    "{}/quickiller_profiles/{}.txt",
                    var("USERPROFILE").unwrap(),
                    profile
                )
            }
            None => self.config.to_string(),
        };
        let mut file = fs::read_to_string(&config_file).unwrap();
        file = file.replace(&exe_name, "");
        fs::write(config_file, file).unwrap();
        println!("Successfully removed {}", exe_name);
    }

    pub fn list(&self, path: Option<String>) {
        let path = match path {
            Some(path) => {
                format!(
                    "{}/quickiller_profiles/{}.txt",
                    var("USERPROFILE").unwrap(),
                    path
                )
            }
            None => self.config.to_string(),
        };
        println!("{}", fs::read_to_string(path).unwrap());
    }

    pub fn list_all(&self) {
        let mut sys = System::new_all();
        sys.refresh_all();
        for (pid, process) in sys.processes() {
            println!("[{}] {}", pid, process.name());
        }
    }

    pub fn new_profile(&self, name: String) {
        match fs::File::create(format!(
            "{}/{}.txt",
            format!("{}/quickiller_profiles", var("USERPROFILE").unwrap()),
            &name
        )) {
            Ok(_) => {
                println!("Successfully created {}", name);
            }
            Err(err) => {
                println!("{}", err)
            }
        }
    }

    pub fn del_profile(&self, name: String) {
        match fs::remove_file(format!(
            "{}/{}.txt",
            format!("{}/quickiller_profiles", var("USERPROFILE").unwrap()),
            &name
        )) {
            Ok(_) => {
                println!("Successfully deleted {}", name);
            }
            Err(err) => {
                println!("{}", err)
            }
        }
    }

    pub fn list_profiles(&self) {
        let paths = fs::read_dir(format!(
            "{}/quickiller_profiles",
            var("USERPROFILE").unwrap()
        ))
        .unwrap();

        for entry in paths {
            let entry = entry.unwrap();
            let path = entry.path();
            println!("{}", path.display());
        }
    }
}
