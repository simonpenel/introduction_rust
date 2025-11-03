#[allow(dead_code)]
#[allow(unused)]
// =========================
//  LES TRAITS COPY ET CLONE
// =========================
// https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions
//  Une fonction simple avec une chaîne
pub fn ma_fonction_int_ref(x: &u32) {
    println!("entrée = {}", x);
    let y = *x; //ok
    let z = *x; //ok
}

//  Une fonction simple avec une chaîne
pub fn ma_fonction_string_ref(x: &String) {
    println!("entrée = {}", x);
    // let y = *x;
    let y = x.clone(); // ok
}
fn main() {
    let hello = String::from("Hello");
    ma_fonction_string_ref(&hello);
    println!("{}",hello);

    let hello = 12;
    ma_fonction_int_ref(&hello);
    println!("{}",hello);
}
