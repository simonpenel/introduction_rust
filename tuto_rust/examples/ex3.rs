
fn main() {
     println!("Hello, world!"); 

    // ==================================
    // Variables mutables et non mutables
    // ==================================
    
    let a = 32;
    println!("{}",a);

    // cette instruction engendre une erreur:
    // a = 42;

    let mut b = 42;
    println!("{}",b);

    // cette instruction est correcte:
    b = 42;
    println!("{}",b);


    // ==================================
    // Le "borowing"
    // ==================================

    let hello = String::from("Hello, world!");

    println!("{}",hello);

    let coucou = hello;

    println!("{}",coucou);     
    // cette instruction engendre une erreur:
    // println!("{}",hello);

    // ==================================
    // Les references
    // ==================================

    let bonjour = &coucou;
    println!("{}",bonjour);
    // println! recupere automatiquement
    println!("{}",*bonjour);
    println!("{}",coucou);     

    // Le borowing ne pose pas de probleme avec les variable de type entier.
    // La variable est copiée, cela ne coute pas cher, contrairement à une chaine de caracteres.

    let hello = 1;
    println!("{}",hello);
    let coucou = hello;
    println!("{}",coucou);     
    // cette instruction marche:
    println!("{}",hello);


    // ==================================
    // Inference de type
    // ==================================    
    let mut a = 21;
    println!("{}",a);

    // Genere une erreur
    // a = a + 12.0;


    let mut b = 52.0;
    b = b + 10.0;
    println!("{}",b);

    let mut b = 32.5;
    b = b + 10.0;
    println!("{}",b);

    // Specifier le type: ici un entier positif codé sur 32 bit : "u32"
    let a: u32 = 12;


    // ==================================
    // Les fonctions
    // ==================================
    let a: u32 = 12;
    ma_fonction(a);

    let hello = String::from("Hello, world!");
    println!(" {}",hello);
    ma_fonction_string(hello);
    // Genere une erreur:
    // println!(" {}",hello);

    println!("avant {}",a);
    ma_fonction_3(a);
    println!("apres {}",a);
    // on constate que rien n'a changé

    let  a: &mut u32 = &mut 12;
    println!("avant {}",a);
    *a = 24;
    println!("apres {}",a);
    ma_fonction_ref(a);
    println!("apres {}",a);
    // on constate que ca a changé


}

fn ma_fonction(x:u32) {
    println!("entree = {}", x)
}

fn ma_fonction_string(x:String) {
    println!("entree = {}", x)
}


fn ma_fonction_2(x:u32) {
    println!("entree = {}", x);
    // Genere une erreur
    //x = x + 5;

}

fn ma_fonction_3(mut x:u32) {
    println!("entree = {}", x);
    x = x + 5;
    println!("nouvelle entree = {}", x);

}

fn ma_fonction_ref( x: &mut u32) {
    println!("entree = {}", x);
    *x = 54;
    println!("nouvelle entree = {}", x);

}
