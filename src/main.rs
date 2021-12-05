use std::env;
use std::fs::File;
use std::process;
use std::{thread, time::Duration};

fn main() {
   let file = File::open("quickillerconfig.txt");
   if let Err(err) = file {
        File::create("quickillerconfig.txt");
        println!("Set up complete, you may now close this or wait a few seconds");
        thread::sleep(Duration::from_millis(5000));
        process::exit(0);
   } else {
    let args: Vec<String> = env::args().collect();
    let option = args[1].clone();
    
    let killer = quickiller::Quickiller::new("quickillerconfig.txt");
    match option.as_str() {
        "kill" => { killer.kill() },
        "add" => { 
            let second_option: String = args[2].clone();
            killer.add(second_option);
        },
        "remove" => {
            let second_option: String = args[2].clone();
            killer.remove(second_option);
        },
        "list" => {
            killer.list();
        },
        "list_all" => {
            killer.list_all();
        },
        "help" => {
            println!("Quickiller - Softsquirrel Studios");
            println!("Quickiller is a CLI tool which can help you kill multiple processes at once for whatever reason, maybe it could be so you can free up memory or CPU usage!");
            println!("=== COMMANDS ===");
            println!("quickiller kill - Kills all processes");
            println!("quickiller add <exe_name> - Add a process to kill when the kill command is ran. To see the name of an exe, run list_all");
            println!("quickuller remove <exe_name> - Removes a process from the list of processes that are killed when the kill command is ran.");
            println!("quickiller list - Lists all processes that are killed when the kill command is ran");
            println!("quickiller list_all - Lists all system processes, this can help you when adding a new process to kill");
            println!("quickiller help - Opens this menu");
            println!("============================================================================================");
            println!("https://github.com/Squirrelcoding/Quickiller");
        }
        _ => { println!("unknown option, use quickiller help for info") }
    }
  }

}