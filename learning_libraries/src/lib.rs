///This library provides utility functions for reading input from stdin. 
/// The main function is `read_stdin` which reads a line of input from the user and returns it as a String.
/// The function uses `BufReader` to efficiently read from stdin and handles any potential errors that may occur during the reading process. 
/// The input is trimmed to remove any leading or trailing whitespace before being returned. 
/// This library can be used in various applications where user input is required, such as command-line tools or interactive programs.

use std::io::{BufReader, BufRead};


///this function reads a line from stdin and returns a String
///# Examples:
///```
///let input = read_stdin();
/// ```
pub fn read_stdin()-> String{
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read input");
    line.trim().to_string();
}