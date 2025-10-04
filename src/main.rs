use std::env;
use termlogo::ascii_font::render_text;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 || args[1] == "--help" || args[1] == "-h" {
        print_help();
        return;
    }

    let input_text = args[1..].join(" ");
    let rendered = render_text(&input_text);
    println!("{}", rendered);
}

fn print_help() {
    eprintln!("TermLogo - Simple ASCII Logo Output Tool\n");
    eprintln!("Usage:");
    eprintln!("  termlogo.exe \"YOUR TEXT HERE\"");
    eprintln!("  termlogo \"Another Message\"");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -h, --help     Show this help message");
}
