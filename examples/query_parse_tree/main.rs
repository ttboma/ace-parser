use std::fs::File;
use std::io::Read;

use ace_parser::*;

fn main() {
    let mut file = File::open("examples/query_parse_tree/test.ace")
        .expect("open file examples/query_parse_tree/test.ace");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let parse_tree = ace(&contents);
    println!("{:#?}", parse_tree);

    // First character of the file 
    let line = 0;
    let character = 0;
    let non_terminal = parse_tree.query(Position::new(line, character)).unwrap();
    println!(
        "query line: {line} character: {character}\n{:#?}",
        non_terminal
    );

    let line = 1;
    let character = 14;
    let non_terminal = parse_tree.query(Position::new(line, character)).unwrap();
    println!(
        "query line: {line} character: {character}\n{:#?}",
        non_terminal
    );

    let line = 4;
    let character = 0;
    let non_terminal = parse_tree.query(Position::new(line, character)).unwrap();
    println!(
        "query line: {line} character: {character}\n{:#?}",
        non_terminal
    );

    // The position is not in the parse tree (there is one character '\n' in line 4), 
    // Still get the result of the last non_terminal
    let line = 4;
    let character = 2;
    let non_terminal = parse_tree.query(Position::new(line, character));
    println!(
        "query line: {line} character: {character}\n{:#?}",
        non_terminal
    );

    // Error: End of the file
    let line = 7;
    let character = 2;
    let non_terminal = parse_tree.query(Position::new(line, character));
    println!(
        "query line: {line} character: {character}\n{:#?}",
        non_terminal
    );
}
