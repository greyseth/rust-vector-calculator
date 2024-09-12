use crate::{magnitude, utils::get_vector, Vector3};

use super::dot::calc_dot;

pub fn angle(vectors: &Vec<Vector3>) {
    let input1 = get_vector(vectors, "Select first vector: ");
    if input1.1 == false {
        let input2 = get_vector(vectors, "Select second vector index: ");
        if input2.1 == false {
            let mag1 = magnitude(&input1.0);
            let mag2 = magnitude(&input2.0);
            let dot = calc_dot(&input1.0, &input2.0);

            println!("The angle between {} and {} is {}", input1.0.to_str(), input2.0.to_str(),
            dot / mag1*mag2);
        }
    }
}