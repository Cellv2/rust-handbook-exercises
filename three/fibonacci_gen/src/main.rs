fn main() {
    println!("binet's: {}", binets_formula(250));
}

fn binets_formula(num: i32) -> i128 {
    // Fn =(Φn - (1-Φ)n)/√5
    // Φ =~ 1.618034
    const PHI: f64 = 1.618034;
    let sqrt_five: f64 = 5_f64.sqrt();

    let res = ((PHI).powf(num.into()) - (1.0 - PHI).powf(num.into())) / sqrt_five;
    let res = res.round() as i128;

    if res == i128::MAX {
        println!("We hit the i128 max! The number will be incorrect 😞")
    }

    res
}

