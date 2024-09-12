mod commands {pub mod dot; pub mod angle; pub mod scalar;}
mod utils;

use std::io::{stdin, stdout, Write};
use utils::{get_vector, vector_input};
use commands::dot::dot;
use commands::angle::angle;
use commands::scalar::scalar;

#[derive(Debug, Clone, Copy)]
struct Vector3 {
    x: f64, y: f64, z: f64
}
impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 {x, y, z}
    }
    fn zero() -> Self {
        Vector3 {x: 0.0, y: 0.0, z: 0.0}
    }
    fn from_array(arr: &[f64; 3]) -> Self {
        Vector3::new(arr[0], arr[1], arr[2])
    }
    pub fn as_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
    pub fn to_str(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }
}

fn magnitude(v: &Vector3) -> f64 {
    let mut res: f64 = 0.0;
    for axis in 0..v.as_array().len() {
        res += v.as_array()[axis as usize] * v.as_array()[axis as usize];
    }
    res.sqrt()
}

fn normalize(v: &mut Vector3) {
    *v = {
        let mut res: [f64; 3] = [0.0, 0.0, 0.0];
        for axis in 0..v.as_array().len() {
            res[axis] = v.as_array()[axis] / magnitude(v);
        }

        Vector3::new(res[0], res[0], res[0])
    }
}

//Main Function
#[allow(unused)]
fn main() {
    println!("My Vector Calculator App");

    let mut vectors: Vec<Vector3> = Vec::new();
    
    loop {
        print!("Enter command: ");
        stdout().flush().expect("Failed to flush output");
        let mut menu_input: String = String::new();
        stdin().read_line(&mut menu_input).expect("Failed to process input");
        match menu_input.trim() {
            "help" => {
                println!("The help menu is WIP");
                continue;
            },
            "add" => {
                let mut new_vector: Vector3 = Vector3::zero();
                vector_input(&mut new_vector);

                vectors.push(new_vector);
                println!("Added new vector");
                continue;
            },
            "view" => {
                println!("Current size of vectors: {}", vectors.len());

                for v in 0..vectors.len() {
                    println!("Vector {v}: {} ({})", 
                    vectors[v].to_str(),
                    if (magnitude(&vectors[v])-1.0).abs() < f64::EPSILON {"normalized"} else {"not normalized"});
                }

                continue;                
            },
            "normalize" => {
                let selection = get_vector(&vectors, "");
                if selection.1 == false {
                    normalize(&mut vectors[selection.2 as usize]);
                    println!("Normalized vector {}: {}", selection.2, vectors[selection.2 as usize].to_str());
                    continue;
                }else {continue;}
            },
            "magnitude" => {
                let selection = get_vector(&vectors, "");
                if selection.1 == false {
                    let mag = magnitude(&selection.0);
                    println!("The magnitude of {}: {mag}", selection.0.to_str());
                    continue;
                }else {continue;}
            },
            "dot" => {
                dot(&vectors);
                continue;
            },
            "angle" => {
                angle(&vectors);
                continue;
            },
            "scalar" => {
                scalar(&vectors);
                continue;
            },
            "exit" => {
                println!("Goodbye");
                break;
            },
            _ => {
                println!("Unknown command");
                continue;
            }
        }
    }
}