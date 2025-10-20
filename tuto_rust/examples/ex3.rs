// ===============
//  LES FONCTIONS 
// ===============


fn main() {

    // Une fonction tres simple sur un entier
    fn ma_fonction(x:u32) {
        println!("entree = {}", x)
    }
    let a: u32 = 12;
    ma_fonction(a);
    println!(" {}",a);

    // Si on veut modifier la variable:
    fn ma_fonction_2(x:u32) {
        println!("entree = {}", x);
        // x = x + 5; // Genere une erreur

    }
    // La variable doit etre mutable
    fn ma_fonction_3(mut x:u32) {
        println!("entree = {}", x);
        x = x + 5;
        println!("nouvelle entree = {}", x);
    }
    println!("avant {}",a);
    ma_fonction_3(a);
    println!("apres {}",a);
    // On constate que la variable  n'a pas changé


    // La meme fonction tres simple sur  une chaine
    fn ma_fonction_string(x:String) {
        println!("entree = {}", x)
    }

    let hello = String::from("world");
    println!("Hello  {}!",hello);
    ma_fonction_string(hello);
    // println!(" {}",hello); // Genere une erreur: la chaine a ete empruntéepar la fonction: elle n'est plus accessible.


    // Dans les examples précédents la fonction devient propriétaire de la variable, 
    // il s'agit de ownership comme dans l'instruction coucou = hello.


    // APPEL PAR REFERENCE
    // ===================

    // On va utiliser une fonction qui prend la reference vers la variable plutot que la variable elle meme.
    // Rappel sur les references : https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
    let  mut a: u32 =  12;
    println!("avant {}",a);
    ma_fonction_ref(&mut a);
    println!("apres {}",a);
    // on constate que la variable a bien  change

	// alternative : a est ici une reference vers une valeur mutable
    let  a: &mut u32 = &mut 12;
    println!("avant {}",a);
    *a = 24;
    println!("apres {}",a);
    ma_fonction_ref(a);
    println!("apres {}",a);


    // * La meme chose avec une chaine
    let hello = String::from("Hello");
    ma_fonction_string_ref(&hello);
    // Ne genere plus d'erreur:
    println!(" {}",hello);

    // * Modification de la chaine donnee en entree
    let mut hello = String::from("Hello");
    ma_fonction_string_ref_mut(&mut hello);
    println!(" {}",hello);

	// * Autre maniere de modifier une chaine
    let mut hello = String::from("Hello");
    ma_fonction_string_ref_mut_2(&mut hello);
    println!(" {}",hello);

}

// Definition des fonctions
// ========================
// Voir https://doc.rust-lang.org/book/ch03-03-how-functions-work.htmlhttps://doc.rust-lang.org/book/ch03-03-how-functions-work.html




fn ma_fonction_ref( x: &mut u32) {
    println!("entree = {}", x);
    *x = 54;
    println!("nouvelle entree = {}", x);

}

fn ma_fonction_string_ref( x: &String) {
    println!("entree = {}", x);

}
fn ma_fonction_string_ref_mut( x: &mut String) {
	x.push_str(", world!");
}

fn ma_fonction_string_ref_mut_2( x: &mut String) {
	*x = x.to_owned() + " ,world!"
}

// Plus d'info sur String et &str

// String is the dynamic heap string type, like Vec:
// use it when you need to own or modify your string data.

// str is an immutable sequence of UTF-8 bytes of dynamic length somewhere in memory.
// Since the size is unknown, one can only handle it behind a pointer.
// This means that str most commonly appears as &str: a reference to some UTF-8 data,
// normally called a "string slice" or just a "slice"
//
// More info on https://blog.logrocket.com/understanding-rust-string-str/