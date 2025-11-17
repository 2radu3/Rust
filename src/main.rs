// Declare the echo module used to implement echo command
mod echo;
mod cat;
mod head;
mod date;
mod mkdir;
mod pwd;
mod rmdir;
mod env;
mod ln;
mod mv;
mod stat;

fn main() {
    // Retrieve the list of arguments
    let arguments: Vec<String> = std::env::args().collect();

    // Check if a command was specified
    let command = match arguments.get(1) {
        // Instead of panicking, you can display a help message with the list of all possible
        // commands.
        None => panic!("No command provided"),
        Some(command) => command,
    };

    // Check what command the user specified
    match command.as_str() {
        // Call the appropiate function. Since echo() function is defined in another module, we have
        // to use the full path.
        //
        // Also, the echo argument takes some arguments. Pass them to the command.
        "echo" => echo::echo(&arguments[2..]),
        "cat" => cat::cat(&arguments[2..]),
        "head" => head::head(&arguments[2..]),
        "date" => date::date(&arguments[2..]),
        "mkdir" => mkdir::mkdir(&arguments[2..]),
        "pwd" => pwd::pwd(&arguments[2..]),
        "rmdir" => rmdir::rmdir(&arguments[2..]),
        "env" => env::env(&arguments[2..]),
        "ln" => ln::ln(&arguments[2..]),
        "mv" => mv::mv(&arguments[2..]),
        "stat" => stat::stat(&arguments[2..]),
        _ => panic!("Unrecognized command"),
    }
}