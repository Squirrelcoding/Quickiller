use std::env;
use std::fs;
use std::process;
use std::{thread, time::Duration};
fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();
    if len <= 1 {
        error();
    }
    if let Err(_) = fs::File::open(format!(
        "{}/quickillerconfig.txt",
        env::var("USERPROFILE").unwrap()
    )) {
        match fs::File::create(format!(
            "{}/quickillerconfig.txt",
            env::var("USERPROFILE").unwrap()
        )) {
            Ok(_) => {
                fs::create_dir(format!(
                    "{}/quickiller_profiles",
                    env::var("USERPROFILE").unwrap()
                ))
                .unwrap();
                println!("Set up complete, you may now close this or wait a few seconds");
                thread::sleep(Duration::from_millis(5000));
                process::exit(0);
            }
            Err(e) => {
                println!("failed to create file");
                println!("L");
                println!("{}", e);
            }
        }
    } else {
        let option = args[1].clone();

        let killer = quickiller::Quickiller::new(format!(
            "{}/quickillerconfig.txt",
            env::var("USERPROFILE").unwrap()
        ));
        match option.as_str() {
            "kill" => {
                if len == 2 {
                    killer.kill(None);
                } else if len == 3 {
                    let second_option = args[2].clone();
                    killer.kill(Some(second_option));
                }
            }
            "add" => {
                if len == 3 {
                    let program: String = args[2].clone();
                    killer.add(program, None);
                } else if len == 4 {
                    let profile: String = args[2].clone();
                    let program: String = args[3].clone();
                    killer.add(program, Some(profile));
                }
            }
            "remove" => {
                if len == 3 {
                    let program: String = args[2].clone();
                    killer.remove(program, None);
                } else if len == 4 {
                    let profile: String = args[2].clone();
                    let program: String = args[3].clone();
                    killer.remove(program, Some(profile));
                }
            }
            "list" => {
                if len == 2 {
                    killer.list(None);
                } else if len == 3 {
                    let second_option = args[2].clone();
                    killer.list(Some(second_option));
                }
            }
            "list_os_current" => {
                killer.list_all();
            }
            "new_profile" => {
                if len <= 2 {
                    error();
                } else if len >= 3 {
                    let second_option = args[2].clone();
                    killer.new_profile(second_option);
                }
            }
            "del_profile" => {
                if len <= 2 {
                    error();
                } else if len >= 3 {
                    let second_option = args[2].clone();
                    killer.del_profile(second_option);
                }
            }
            "list_profiles" => {
                killer.list_profiles();
            }
            "help" => {
                println!("Quickiller - Softsquirrel Studios");
                println!("Quickiller is a CLI tool which can help you kill multiple processes at once for whatever reason, maybe it could be so you can free up memory or CPU usage!");
                println!("============================================================================================");
                println!("Visit the link below for information, help, and a list of commands.");
                println!("https://github.com/Squirrelcoding/Quickiller");
                println!("v1.2");
            }
            _ => {
                println!("unknown option, use quickiller help for info")
            }
        }
    }
}

fn error() {
    println!("Invalid amount of arguments, use quickiller help for info");
    process::exit(0);
}
