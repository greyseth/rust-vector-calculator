use std::io::{stdin, stdout, Write};

use crate::{utils::get_vector, Vector3};

pub fn scalar(vectors: &Vec<Vector3>) {
    let vec_input = get_vector(vectors, "");
    if vec_input.1 == false {
        loop {
            print!("Enter scale: ");
            stdout().flush().expect("Failed to flush output");
            
            let mut scale_input = String::new();
            stdin().read_line(&mut scale_input).expect("Failed to process input");
            match scale_input.trim().parse::<u32>() {
                Ok(value) => {
                    let mut vec_arr = vec_input.0.as_array();
                    for axis in 0..vec_arr.len() {
                        vec_arr[axis] *= value as f64;
                    }

                    println!("{} scaled by {value}: {}", vec_input.0.to_str(), Vector3::from_array(&vec_arr).to_str());
                    break;
                }
                Err(_) => {
                    println!("Expected a number");
                    continue;
                }
            }
        }
    }
}