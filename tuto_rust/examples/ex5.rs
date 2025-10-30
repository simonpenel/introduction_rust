#[allow(dead_code)]
#[allow(unused)]
// =========================
//  LES TRAITS COPY ET CLONE
// =========================


//  Une fonction simple avec une chaîne
pub fn ma_fonction_string_int(x: &u32) {
    println!("entrée = {}", x);
    let y = *x; //ok
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

    let hello = 12;
    ma_fonction_string_int(&hello);




}
