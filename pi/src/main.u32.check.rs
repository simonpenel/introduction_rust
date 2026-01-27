use std::env;
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut i  = 0;
    let mut somme = 0.0;
    let imax = match arguments[1].parse::<u32>() {
        Ok(valeur) => valeur,
        Err(_e) => { panic!("Please enter an integer") }
    }; 
    while i < imax {
        // let val = (4 * i + 1) * (4 * i + 3);

        let val1 = i.checked_mul(4).and_then(|v| v. checked_add(1)).unwrap();
        let val2 = i.checked_mul(4).and_then(|v| v. checked_add(3)).unwrap();
        let val3 = val1.checked_mul(val2).expect("Overflow!");
        somme = somme + 1.0 / val3 as f32;
        i += 1;
    }
    println!("{}",somme * 8.0);
}
