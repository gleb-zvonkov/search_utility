use clap::Parser; //used to parse arguments
use std::fs; //used to open file
use std::io::BufRead; //use to read the file line by line

#[derive(Parser)] //macro derives an implementation of Parser trait for the struct
struct Args {
    /// The search term
    search_term: String,
    /// The file path to search in
    file_path: String,
}

fn main() {
    let args = Args::parse(); //function is provided by Parser, which clap macro derives for the Args struct
    println!(
        "Searching for '{}' in file '{}'",
        args.search_term,
        args.file_path // ask for the two argument
    );

    let file = fs::File::open(&args.file_path).unwrap(); //open file    unwrap cause panic if err and returns file object if ok
    let reader = std::io::BufReader::new(file); //open bufreader

    let mut found = false; //boolean for if search term is found

    for (index, line) in reader.lines().enumerate() {
        //go through all the lines in, enumarte returns index of line and the line itself, BufRead is used here
        let line = line.unwrap(); //unwrap cause panic if err and returns line if ok
        if line.contains(&args.search_term) {
            //check if the line contains the search term
            println!("{}", line);
            //println!("Found on line {}: {}", index + 1, line); //print the line
            found = true; //have found the search term
        }
    }
    if !found {
        // if its not found
        println!("\nSearch term '{}' not found in the file", args.search_term);
    }
}
