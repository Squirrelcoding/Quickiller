use std::{
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

    fn get_exe(&self) -> Vec<String> {
        let file = fs::read_to_string(self.config.to_string().to_string()).unwrap();
        let exe_files = file.split(' ').into_iter();
        let mut exe_files_vec = Vec::new();
        for i in exe_files {
            exe_files_vec.push(i.to_string());
        }
        exe_files_vec
    }

    pub fn kill(&self) {
        let files = self.get_exe();
        let mut sys = System::new_all();
        sys.refresh_all();
        for (_, process) in sys.processes() {
            if files.contains(&process.name().to_string()) {
                process.kill(Signal::Kill);
            }
        }
    }

    pub fn add(&self, exe_name: String) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.config.to_string())
            .unwrap();
        write!(file, " {}", exe_name).unwrap();
        println!("Successfully added {}", exe_name);
    }

    pub fn remove(&self, exe_name: String) {
        let mut file = fs::read_to_string(self.config.to_string()).unwrap();
        file = file.replace(&exe_name, "");
        fs::write(self.config.to_string(), file).unwrap();
        println!("Successfully removed {}", exe_name);
    }

    pub fn list(&self) {
        let file = fs::read_to_string(self.config.to_string()).unwrap();
        println!("{}", file);
    }

    pub fn list_all(&self) {
        let mut sys = System::new_all();
        sys.refresh_all();
        for (pid, process) in sys.processes() {
            println!("[{}] {}", pid, process.name());
        }
    }
}
