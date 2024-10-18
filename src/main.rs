//Gleb Zvonkov
//October 17, 2024
//ECE1724

use clap::Parser; // Parser used to parse arguments, ArgGroup for grouping args
use colored::Colorize; //used to color text
use std::fs; // Used to open file
use std::io::BufRead; // Used to read the file line by line
use std::path::PathBuf; //owned, mutable path
use walkdir::WalkDir;

#[derive(Parser)] // Macro derives an implementation of Parser trait for the struct
#[clap(disable_help_flag = true)] //disaple the default help flag because we implement a custom one
struct Args {
    search_term: String,     // The search term
    file_paths: Vec<String>, // The file paths to begin sesrch
    #[clap(short = 'i')] // case-insensitive search
    case_insensitive: bool,
    #[clap(short = 'n')] // print line numbers
    print_line_numbers: bool,
    #[clap(short = 'v')] // Only include lines that dont contain pattern
    invert_match: bool,
    #[clap(short = 'r')] // recursive directory search
    recursive: bool,
    #[clap(short = 'f')] // print path names
    print_filenames: bool,
    #[clap(short = 'c')] // colored output of the search term
    colored_output: bool,
    #[clap(short = 'h', long = "help")] // help menu
    display_help: bool,
}

fn main() {
    let args = Args::parse(); // Parse command-line arguments

    if args.display_help {
        display_help(); //display the help message
        return; // exit program
    }

    for input_path in &args.file_paths {
        //for each file listed take the file path as a refrence, so can use arg later
        let paths = get_file_entries(input_path, args.recursive); //get all the path names recursively if necessary
        for path in paths {
            search_in_file(&path, &args);
        }
    }
}

// Function to display custom help message
// No parameters
// No return
fn display_help() {
    println!("Usage: grep [OPTIONS] <pattern> <files...>");
    println!("Options:");
    println!("  -i                Case-insensitive search");
    println!("  -n                Print line numbers");
    println!("  -v                Invert match (exclude lines that match the pattern)");
    println!("  -r                Recursive directory search");
    println!("  -f                Print filenames");
    println!("  -c                Enable colored output");
    println!("  -h, --help        Show help information");
}

// Function to get file entrie paths from the specified path recursively
// paramaters are the file path and a boolean if we want to recurse into subdirectories
// return is a vector of PathBuf containing the paths to files
fn get_file_entries(input_path: &str, recursive: bool) -> Vec<PathBuf> {
    WalkDir::new(input_path) //use walkdir object on the input path, it deals with recursion and special characters
        .max_depth(if recursive { usize::MAX } else { 1 }) // If recursive, go as deep as possible; if not, only search at depth level 1
        .into_iter() // Make it an iterator
        .filter_map(|e| e.ok()) // Take out all the errors
        .filter(|e| e.path().is_file()) // Filter out the directories
        .map(|e| e.path().to_path_buf()) // Transform each entry into file path
        .collect() // Collect file paths into vector
}

// Function to search for input term in single file
// parameters are the path which is of type PathBuf and a refrence to all the input arguments
// return is nothing
fn search_in_file(path: &PathBuf, args: &Args) {
    let file = fs::File::open(path).unwrap(); // Open the file
    let reader = std::io::BufReader::new(file); // Create a buffer reader
    for (index, line) in reader.lines().enumerate() {
        //for each line
        let line = line.unwrap(); // Read the line
        let matched = is_line_matched(&line, args); //check if it matches inputted pattern
        if matched {
            //if it does print it
            print_line(&line, index, args, path);
        }
    }
}

// Function to check if a line matches the search term
// paramaters are the line and the input arguments
// return is true or false
fn is_line_matched(line: &str, args: &Args) -> bool {
    let contains = if args.case_insensitive {
        line.to_lowercase()
            .contains(&args.search_term.to_lowercase()) // if case insensitive make it all lowercase and compare
    } else {
        line.contains(&args.search_term) //just compare
    };
    if args.invert_match {
        !contains // we want this to be false when the line does contain the search term so we print lines without search term
    } else {
        contains //otherwise we just want it remain the same so we print lines with the search terms
    }
}

// Function to print the line according to the specified argument
// parameters are the line, its index (line number), the arguments, and the path
//
fn print_line(line: &str, index: usize, args: &Args, path: &PathBuf) {
    let line_to_print = if args.colored_output {
        line.replace(&args.search_term, &args.search_term.red().to_string()) // Replace the search term with a red search term
    } else {
        line.to_string() // Otherwise, just return the regular line
    };
    if args.print_filenames {
        print!("{}: ", path.display()); // Print the file name
    }
    if args.print_line_numbers {
        println!("{}: {}", index + 1, line_to_print); // Index starts at 0, so +1 then print the line
    } else {
        println!("{}", line_to_print); // Print it without the index
    }
}
