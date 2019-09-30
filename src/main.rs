fn print_help() {
    let text = "
    alias_extractor

    Usage:
        <stdin> | alias_extractor

        where <stdin> is in the form of

        <string>=<command>

        just like one would use alias <string>=<command>

        Example:

        alias kaf='kubectl apply -f'
        alias | head -1 | alias_extractor
        > kaf
        ";

    println!("{}", text);

    std::process::exit(0);
}

fn main() {
    {
        let help = "--help".to_string();
        match std::env::args().nth(1) {
            Some(help) => print_help(),
            _ => {}
        }
    }

    let stdin = {
        let mut buffer = String::new();
        use std::io::Read;

        let size = std::io::stdin().read_to_string(&mut buffer).unwrap();

        if size == 0 {
            std::process::exit(0);
        }

        buffer
    };

    if let Some(index) = stdin.find('=') {
        let result = &stdin[0..index];

        print!("{}", result);
    } else {
        eprintln!("No alias command found");
        std::process::exit(1);
    }
}
