fn main() {
    println!("Hello, world!");
    let v = vec![1.0, 2.0, 4.0];

    let avg = vector_average(v);
    println!("The average is {}", avg)
}

fn vector_average(v: Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    let n = v.len();
    for element in v {
        sum += element;
    }
    sum / (n as f64)
}
