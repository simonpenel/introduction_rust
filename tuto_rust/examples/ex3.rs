
fn main() {
    println!("Hello, world!"); 

    // ==================================
    // Les differentes variables
    // ==================================

    // Inference de type
    let mut a = 21;
    println!("{}",a);
    print_type_of(&a);
    a = a + 10;
    println!("{}",a);

    // Genere une erreur
    // a = a + 12.0;

    // Genere une erreur
    //let a = 3 + 12.0;

    let mut b = 52.0;
    print_type_of(&b);
    b = b + 10.0;
    println!("{}",b);

    let mut b = 32.5;
    b = b + 10.0;
    println!("{}",b);

    // Specifier le type

    let a: u32 = 12;
    let b = a * 2;
    let c: u64 = 54;
    // Genere une erreur    
    //let d = a + c;

    let chaine = "Bonjour chez vous";
    println!("{}",chaine);
    print_type_of(&chaine);

    let chaine = String::from("Bonjour chez vous");
    println!("{}",chaine);
    print_type_of(&chaine);


    let chaine = "Bonjour chez vous".to_string();
    println!("{}",chaine);
    print_type_of(&chaine);

}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}