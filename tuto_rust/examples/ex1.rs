// ======================================================================
// MUTABLE AND IMMUTABLE VARIABLES, SHADOWING,  TYPE INFERENCE
// ======================================================================

fn main() {
    // ==================================
    // mutable and immutable variables
    // ==================================
    println!("Hello, world!");  // <-  an instruction ends with ';'.
    let a = 32;                 // <- declaration of a  variable with 'let'
    println!("{}", a);          // <- println! is a rust  macro dedicated to displaying text

    // this instruction causes an error:
    // a = 42;

    // this instruction causes an error:
    // a = a + 10;

    let mut b = 32;
    println!("{}", b);

    // this instruction is correct:
    b = 42;
    println!("{}", b);

    // this instruction is correct:
    b = b + 10;
    println!("{}", b);

    // ==================================
    // Type inference
    // ==================================
    let mut a = 21;
    println!("{}", a);
    a = a + 5;
    println!("{}", a);
    // this instruction causes an error:
    // a = a + 12.0;

    let mut b = 52.0;
    b = b + 10.0;
    println!("{}", b);

    // Specifying type: here a unsigned  integer encoded on 32 bits : "u32"
    let a: u32 = 12;
    println!("{}", a);

    // Some types types : i8,u8, i16, u16, i32, u32,f32,f64 ...
    // https://doc.rust-lang.org/book/ch03-02-data-types.html

    // ============
    // Shadowing
    // ============
    let a = 10;
    //  a = a + 10 ; // Error
    let a = a + 10; // "shadowing"
    println!("{}", a);
}
