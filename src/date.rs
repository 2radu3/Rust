use chrono::Local;
use chrono::Utc;

pub fn date(arguments: &[String]){
    let local_time=Local::now();
    println!("{}", local_time);

    let mut start_index = 0;
    if let Some(argument) = arguments.get(0) {
                if argument == "-u" || argument == "--utc"{
                    start_index=2;
            }
    }

    if start_index == 0{
        let local_time = Local::now();
        println!("{}", local_time)
    }
    else{
        let local_time=Utc::now();
        println!("{}", local_time);
    }

}
