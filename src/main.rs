extern crate clap;

use clap::{Arg, App};
use std::process::exit;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let matches = App::new("brainfuck")
        .version("1.0")
        .about("Brainfuck interpreter")
        .author("mvukic")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE_PATH")
            .help("Reads program from file")
            .takes_value(true)
        )
        .arg(Arg::with_name("program")
            .short("r")
            .long("raw")
            .value_name("PROGRAM")
            .help("Source code of a program")
            .takes_value(true)
        )
        .arg(Arg::with_name("memory")
            .short("m")
            .long("memory")
            .value_name("SIZE")
            .help("Memory size in bytes")
            .takes_value(true)
            .required(false)
        )
        .get_matches();

    // Parse memory size from arguments or use default value
    let mut memory_size: usize = 65535;
    if matches.is_present("memory") {
        match matches.value_of("memory") {
            Some(s) =>
                memory_size = match s.to_string().parse() {
                    Ok(size) => size,
                    Err(_) => 65535
                },
            None => println!("Using default memory size")
        }
    }

    // Initialize memory
    let mut memory: Vec<u8> = vec![0; memory_size];

    //Parse file path or source code
    if matches.is_present("file") {
        let path = matches.value_of("file").unwrap().to_string();
        let program = read_from_file(path);
        interpret(program, &mut memory);
    }else if matches.is_present("program") {
        let program = matches.value_of("program").unwrap().to_string();
        interpret(program, &mut memory);
    } else {
        println!("Program must be provided from either a file or source.");
        exit(1);
    }

}

fn read_from_file(path: String) -> String {
    println!("Opening file {}", path);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content.trim().to_string()
}

fn interpret(program: String, memory: &mut Vec<u8>){
    let program = program.as_bytes();
    let mut brackets = Vec::<(usize,usize)>::new();
    let mut program_counter = 0;
    let mut memory_pointer = 0;

    for (index,&token) in program.iter().enumerate() {
        if token == b'[' {
            let mut end_pos = index;
            let mut counter = 1;
            while counter > 0 {
                end_pos += 1;
                match program[end_pos] {
                    b'[' => counter += 1,
                    b']' => counter -= 1,
                    _ => {}
                };
            };
            brackets.push((index, end_pos));
        };
    }

    loop {
        match program[program_counter] {
            b'<' => {
                memory_pointer -= 1;
                program_counter += 1;
            },
            b'>' => {
                memory_pointer += 1;
                program_counter += 1;
            },
            b'[' => {
                if memory[memory_pointer] == 0 {
                    // skip past matching ending bracket
                    let &(_, position) = brackets
                        .iter()
                        .filter(|x| x.0 == program_counter)
                        .next()
                        .unwrap();
                    program_counter = position;
                }
                program_counter += 1;
            },
            b']' => {
                if memory[memory_pointer] != 0 {
                    // jump to the begining of the loop
                    let &(position,_) = brackets
                        .iter()
                        .filter(|x| x.1 == program_counter)
                        .next()
                        .unwrap();
                    program_counter = position;
                } else {
                    program_counter += 1;
                };
            },
            b'+' => {
                memory[memory_pointer] += 1;
                program_counter += 1;
            },
            b'-' => {
                memory[memory_pointer] -= 1;
                program_counter += 1;
            },
            b'.' => {
                print!("{}", memory[memory_pointer] as char);
                program_counter += 1;
            },
            b',' => {
                memory[memory_pointer] = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u8)
                    .unwrap();
                program_counter += 1;
            },
            _ => {
                break;
            }
        }

        if program_counter >= program.len(){
            break;
        }

    }
    println!("");
}