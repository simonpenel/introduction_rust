// ======================================================================
// VARIABLES MUTABLES ET NON MUTABLES, LE SHADOWING,  l'INFERENCE DE TYPE
// ...
// MUTABLE AND IMMUTABLE VARIABLES, SHADOWING,  TYPE INFERENCE
// ======================================================================

fn main() {
    // ==================================
    // variables mutables et non mutables
    // ==================================
    println!("Hello, world!"); // <- une instruction se termine par ';'.
    let a = 32; // <- déclaration d'une variable avec let
    println!("{}", a); // <- println! est une macro rust dédiée à l'affichage

    // cette instruction engendre une erreur:
    // a = 42;

    // cette instruction engendre une erreur:
    // a = a + 10;

    let mut b = 32;
    println!("{}", b);

    // cette instruction est correcte:
    b = 42;
    println!("{}", b);

    // cette instruction est correcte:
    b = b + 10;
    println!("{}", b);

    // ==================================
    // L'inférence de type
    // ==================================
    let mut a = 21;
    println!("{}", a);
    a = a + 5;
    println!("{}", a);
    // Génére une erreur
    // a = a + 12.0;

    let mut b = 52.0;
    b = b + 10.0;
    println!("{}", b);

    // Spécifier le type: ici un entier positif ("unsigned") codé sur 32 bit : "u32"
    let a: u32 = 12;
    println!("{}", a);

    // Quelques types : i8,u8, i16, u16, i32, u32,f32,f64 ...
    // https://doc.rust-lang.org/book/ch03-02-data-types.html

    // ============
    // Le shadowing
    // ============
    let a = 10;
    //  a = a + 10 ; // Erreur
    let a = a + 10; // "shadowing"
    println!("{}", a);
}
