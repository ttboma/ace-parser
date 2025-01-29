use std::fs::File;
use std::io::Read;

use ace_parser::grammar::*;

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
    let query = parse_tree.query(line, character);
    println!("query line: {line} character: {character}\n{:#?}", query);
    println!("show_completions: {:?}", query.show_completions());

    let line = 1;
    let character = 14;
    let query = parse_tree.query(line, character);
    println!("query line: {line} character: {character}\n{:#?}", query);
    println!("show_completions: {:?}", query.show_completions());

    let line = 4;
    let character = 0;
    let query = parse_tree.query(line, character);
    println!("query line: {line} character: {character}\n{:#?}", query);
    println!("show_completions: {:?}", query.show_completions());

    // The position is not in the parse tree (there is one character '\n' in line 4),
    // Still get the result of the last query
    let line = 4;
    let character = 2;
    let query = parse_tree.query(line, character);
    println!("query line: {line} character: {character}\n{:#?}", query);
    println!("show_completions: {:?}", query.show_completions());

    // Error: End of the file
    let line = 7;
    let character = 2;
    let query = parse_tree.query(line, character);
    println!("query line: {line} character: {character}\n{:#?}", query);
    println!("show_completions: {:?}", query.show_completions());

    let cpu_attributes = match &parse_tree.statements()[0] {
        Statement::Cpu(cpu) => {
            cpu.attributes()
        },
        _ => panic!("Not a CPU statement"),
    };
    let name = match &cpu_attributes[0] {
        statement::CpuAttribute::Name(name) => name.identifier().token().fragment(),
        _ => panic!("Not a Name attribute"),
    };
    println!("cpu name: {:#?}", name);
}
