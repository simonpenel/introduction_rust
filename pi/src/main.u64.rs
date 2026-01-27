use std::env;
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut i  = 0;
    let mut somme = 0.0;
    let imax = match arguments[1].parse::<u64>() {
        Ok(valeur) => valeur,
        Err(_e) => { panic!("Please enter an integer") }
    }; 
    while i < imax {
        let val = (4 * i + 1) * (4 * i + 3);
        somme = somme + 1.0 / val as f64;
        i += 1;
    }
    println!("{}",somme * 8.0);
}
