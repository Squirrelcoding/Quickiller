use std::{fs::{OpenOptions, self}, io::{Write}};

use sysinfo::{System, Signal, SystemExt, ProcessExt};


pub struct Quickiller {
  config: &'static str
}

impl Quickiller {
  pub fn new(config: &'static str) -> Quickiller {
    Quickiller { config }
  }

  fn get_exe(&self) -> Vec<String> {
    let file = fs::read_to_string(self.config).unwrap();
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
    for (pid, process) in sys.processes() {
        if files.contains(&process.name().to_string()) {
            process.kill(Signal::Kill);
        }

    }
  }

  pub fn add(&self, exe_name: String)  {
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(self.config)
    .unwrap();
    write!(file, " {}", exe_name);
    println!("Successfully added {}", exe_name);
  }

  pub fn remove(&self, exe_name: String) {
    let mut file = fs::read_to_string(self.config).unwrap();
    file = file.replace(&exe_name, "");
    fs::write(self.config, file);
    println!("Successfully removed {}", exe_name);
  }

  pub fn list(&self) {
    let mut file = fs::read_to_string(self.config).unwrap();
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