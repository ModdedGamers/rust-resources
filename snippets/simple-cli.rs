// No-Library command line arguments (example for "rblox")
    match args().collect::<Vec<_>>().as_slice() {
        // rblox
        [_] => repl(),
        // rblox FILE
        [_, file] => run(file),
        // Unknown usage, show correct usage
        [exe, ..] => println!("Usage: {} [file]", exe),
        _ => (),
    }
