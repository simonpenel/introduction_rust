
fn main() {
    println!("Hello, world!"); 

    // ==================================
    // Variables mutables et non mutables
    // ==================================


    let a = 32;
    println!("{}",a);

    // cette instruction engendre une erreur:
    // a = 42;

    // cette instruction engendre une erreur:
    // a = a + 10;

    let mut b = 42;
    println!("{}",b);

    // cette instruction est correcte:
    b = 42;
    println!("{}",b);

    // cette instruction est correcte:
    b = b + 10;
    println!("{}",b);    
        
}
