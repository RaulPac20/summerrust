//Raul Pacheco
use std::process::Command;

fn executing_os_commands_linux(command_full:&str) {
    
    let parts: Vec<&str> = command_full.split_whitespace().collect();
    let actual_command = parts[0];
    let arg1 = parts[1];

    let output = Command::new(actual_command)
        .arg(arg1)
        .output()
        .unwrap();
        //.expect("Failed to execute command");
        

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}


use std::io::{self, Read, Write};

fn accept_linux_command_from_user()-> String{
    let mut buffer = String::new();
    print!("What linux command you want to execute? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}


fn main(){

     let full_command:String = accept_linux_command_from_user();
     executing_os_commands_linux("full_command");
     //executing_os_commands_linux("echo hello");
    // executing_os_commands_linux("ls -ls");
}