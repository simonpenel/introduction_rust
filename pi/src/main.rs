fn main() {
    let mut i : i64 = 0;
    let mut somme = 0.0;
    let imax = 100000000;
    while i < imax {
        let val = (4 * i + 1) * (4 * i + 3);
        somme = somme + 1.0 / val as f64;
        i += 1;
    }
    println!("{}",somme * 8.0);
}
