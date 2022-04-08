use std::process::Command;

fn main() {
    // creating some basic command bases to work with later
    let mut sleep_command = Command::new("sleep");
    let mut list_command = Command::new("ls");

    // adding an argument to the command
    sleep_command.arg("5s");

    // changing the cwd of the list command
    list_command.current_dir("/home/ocelot/");
    list_command.arg("-l");

    // running those commands
    // spawn() runs the command, but doesn't wait for it to finish
    let _blank = sleep_command.spawn().expect("sleep Failed");

    // output() runs the command, waits for it to finish, then returns the output
    // output is stored in .stdout
    let output = list_command.output().expect("ls Failed");

    // status() runs the command, waits for it to finish, then returns the status

    println!("{}", String::from_utf8(output.stdout).unwrap());
}
