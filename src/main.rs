pub mod convert;

use std::{env::args, path::Path};

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() <= 1 {
        println!("Not enough parameters");
        return;
    }

    if args[1] == "all".to_string() {
        convert::all();
        return;
    }

    for argument in args {
        if argument.contains(".scss") {
            let path = Path::new(&argument);
            convert::file(path);
        } else {
            println!("Invalid parameter");
            return;
        }
    }
}
