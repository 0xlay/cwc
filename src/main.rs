use std::env;
mod ops;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 0 {
        if let Err(err) = parse_args(args) {
            eprintln!("{err}");
        }
    } else {
        print_help()
    }
}

fn print_help() {
    println!("cwc - print newline, word, character and byte counts for each file\n");
    println!("-c, --bytes\n\tprint the byte counts\n");
    println!("-m, --chars\n\tprint the character counts\n");
    println!("-l, --lines\n\tprint the newline counts\n");
    println!("-w, --words\n\tprint the word counts\n");
    println!("Example: cwc -l test.txt");
}

fn parse_args(args: Vec<String>) -> Result<(), String> {
    if args[0] == "-c" || args[0] == "--bytes" {
        println!(
            "\t{}\t{}",
            ops::calculate_bytes(args[1].as_str())?,
            args[1].as_str()
        );
    } else if args[0] == "-m" || args[0] == "--chars" {
        println!(
            "\t{}\t{}",
            ops::calculate_chars(args[1].as_str())?,
            args[1].as_str()
        );
    } else if args[0] == "-l" || args[0] == "--lines" {
        println!(
            "\t{}\t{}",
            ops::calculate_lines(args[1].as_str())?,
            args[1].as_str()
        );
    } else if args[0] == "-w" || args[0] == "--words" {
        println!(
            "\t{}\t{}",
            ops::calculate_words(args[1].as_str())?,
            args[1].as_str()
        );
    } else {
        println!(
            "\t{}\t{}\t{}\t{}",
            ops::calculate_lines(args[0].as_str())?,
            ops::calculate_words(args[0].as_str())?,
            ops::calculate_chars(args[0].as_str())?,
            args[0].as_str()
        );
    }
    Ok(())
}
