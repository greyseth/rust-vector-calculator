use std::io::{stdin, stdout, Write};

use crate::Vector3;

pub fn vector_input(vec: &mut Vector3) {
    *vec = loop {
        println!("Input X, Y, and Z values for new vector:");

        let mut x1 = String::new();
        let mut y1 = String::new();
        let mut z1 = String::new();

        let mut ret_v1 = Vector3::zero();

        print!("Value X: ");
        stdout().flush().expect("Failed to flush output");
        stdin().read_line(&mut x1).expect("Failed to receive input");
        match x1.trim().parse() {
            Ok(value) => ret_v1.x = value,
            Err(_) => {
                println!("Expected a number");
                continue;
            }
        }

        print!("Value Y: ");
        stdout().flush().expect("Failed to flush output");
        stdin().read_line(&mut y1).expect("Failed to receive input");
        match y1.trim().parse() {
            Ok(value) => ret_v1.y = value,
            Err(_) => {
                println!("Expected a number");
                continue;
            }
        }

        print!("Value Z: ");
        stdout().flush().expect("Failed to flush output");
        stdin().read_line(&mut z1).expect("Failed to receive input");
        match z1.trim().parse() {
            Ok(value) => ret_v1.z = value,
            Err(_) => {
                println!("Expected a number");
                continue;
            }
        }

        break ret_v1
    };
}

pub fn get_vector(vectors: &Vec<Vector3>, msg: &str) -> (Vector3, bool, u32) {
    let pmsg = if msg == "" {"Enter vector index: "} else {&msg};
    
    loop {
        print!("{pmsg}");
        stdout().flush().expect("Failed to flush output");
        let mut vector_input = String::new();
        stdin().read_line(&mut vector_input).expect("Failed to process input");
        if vector_input.trim() == "cancel" {break (Vector3::zero(), true, 0)}
        
        match vector_input.trim().parse::<u32>() {
            Ok(value) => {
                let index = value as usize;
                if index >= vectors.len() {
                    println!("Vector index not found");
                    continue;
                }else {break (vectors[index], false, value)}
            }
            Err(_) => {
                println!("Expected a number (or cancel)");
                continue;
            }
        }     
    }
}