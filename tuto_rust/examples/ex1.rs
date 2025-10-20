
// =======================================================
// VARIABLES MUTABLES ET NON MUTABLES, l'INFERENCE DE TYPE
// =======================================================
    
fn main() {
    // ==================================
    // variables mutables et non mutables
    // ==================================
    println!("Hello, world!");
    let a = 32;
    println!("{}",a);

    // cette instruction engendre une erreur:
    // a = 42;

    // cette instruction engendre une erreur:
    // a = a + 10;

    let mut b = 32;
    println!("{}",b);

    // cette instruction est correcte:
    b = 42;
    println!("{}",b);

    // cette instruction est correcte:
    b = b + 10;
    println!("{}",b);

    // ==================================
    // L'inference de type
    // ==================================
    let mut a = 21;
    println!("{}",a);
    a = a + 5;
    println!("{}",a);
    // Genere une erreur
    // a = a + 12.0;


    let mut b = 52.0;
    b = b + 10.0;
    println!("{}",b);


    // Specifier le type: ici un entier positif codé sur 32 bit : "u32"
    let a: u32 = 12;    
    println!("{}",a);

}
