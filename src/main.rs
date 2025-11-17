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
    let arguments: Vec<String> = std::env::args().collect();

    let command = match arguments.get(1) {

        None => panic!("No command provided"),
        Some(command) => command,
    };

    match command.as_str() {
        
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
