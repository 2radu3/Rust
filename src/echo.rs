pub(crate) fn echo(arguments: &[String]) {
    let mut start_index = 0;
    if let Some(argument) = arguments.get(0) {
        if argument == "-n" || argument == "--no-newline" {
            start_index = 1;
        }
    }

    for argument in arguments[start_index..].iter() {
        print!("{}", argument);
        if start_index == 0{
            println!();
        }
    }
}
