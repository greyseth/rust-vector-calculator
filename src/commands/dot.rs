use crate::Vector3;

pub fn dot(vectors: &Vec<Vector3>) {
    let input1 = crate::utils::get_vector(vectors, "Select first vector: ");
    if input1.1 == false {
        let input2 = crate::utils::get_vector(vectors, "Select second vector: ");
        if input2.1 == false {
            let dot = calc_dot(&input1.0, &input2.0);
            println!("The dot product of {} and {}: {}", input1.0.to_str(), input2.0.to_str(), dot);
        }
    }
}

pub fn calc_dot(v1: &Vector3, v2: &Vector3) -> f64 {
    let mut vec1 = v1.as_array();
    let vec2 = v2;
    let mut dot = 0.0;
    for axis in 0..vec1.len() {
        vec1[axis] *= vec2.as_array()[axis];
        dot += vec1[axis];
    }

    dot
}