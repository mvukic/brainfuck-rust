#![allow(usused_must_use)]

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

    println!("Memory size is {} bytes", memory_size);

    // Initialize memory
    let memory: Vec<i8> = vec![0; memory_size];
    let mut program = String::new();

    //Parse file path or source code
    if matches.is_present("file") {
        let path = matches.value_of("file").unwrap().to_string();
        program = read_from_file(path);
    }else if matches.is_present("program") {
        program = matches.value_of("program").unwrap().to_string();
    } else {
        println!("Program must be provided from either a file or source.");
        exit(1);
    }

    interpret(program, &memory);

}

fn read_from_file(path: String) -> String {
    println!("Opening file {}", path);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content);
    
    content.trim().to_string()
}


fn exit_with_message(message: &str){
    println!("{}",message);
    exit(1);
}

fn interpret(program: String, memory: &Vec<i8>){
    let bytes = program.as_bytes();
    let mut counter = 0;
    for i in 0..bytes.len() {
        match bytes[i] {
            b'<' => println!("<"),
            b'>' => println!(">"),
            b'[' => println!("["),
            b']' => println!("]"),
            b'+' => println!("+"),
            b'-' => println!("-"),
            b'.' => println!("."),
            b',' => println!(","),
            _ => {}
        }
    }
}