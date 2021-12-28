fn main() {
    let v = vec![1.0, 2.0, 4.0, 3.2, 0.5, 4.2, 7.7, 8.3];

    let result = signal_change(v);

    for element in result {
        println!("The value is {}", element)
    }
}

fn signal_change(v: Vec<f64>) -> Vec<f64> {
    let mut result_vec: Vec<f64> = Vec::new();
    let n = v.len();
    let last_element = v[n - 1]; // v.last().unwrap().clone();       // last() borrows v.  last_element owns reference to the last element of v
    println!("The last value is {}", last_element);
    for element in v {
        result_vec.push(last_element - element);
    }
    result_vec // Return value
}
