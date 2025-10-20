#[allow(dead_code)]
#[allow(unused)]
// ===============
//  LES FONCTIONS 
// ===============
//  Voir https://doc.rust-lang.org/book/ch03-03-how-functions-work.htmlhttps://doc.rust-lang.org/book/ch03-03-how-functions-work.html

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
    // let coucou = hello;  // Genere une erreur: la chaine a ete empruntéepar la fonction: elle n'est plus accessible.

    // Dans les examples précédents la fonction devient propriétaire de la variable, 
    // il s'agit de ownership comme dans l'instruction coucou = hello. (qui ne pose pas de problème avec les entiers)
    // C'est la raison pour laquelle, dans le cas des entiers la variable n'est pas modifiée car il sagit en fait
    // d'une copie de la variable, et dans le cas des String il n'est plus possible d'acceder à la variable après qu'elle
    // soit devenue la propriété de la fonction.
    // La solution : l'appel par reference

    // APPEL PAR REFERENCE
    // ===================

    // On va utiliser une fonction qui prend la reference vers la variable plutot que la variable elle meme.
    // Rappel sur les references : https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

    fn ma_fonction_ref( x: &mut u32) {
        println!("entree = {}", x);
        *x = 54;
        println!("nouvelle entree = {}", x);

    }
    let  mut a: u32 =  12;
    println!("avant {}",a);
    ma_fonction_ref(&mut a);
    println!("apres {}",a);
    // on constate que la variable a bien  change

	// alternative : a est ici une reference vers une valeur mutable
    let  a: &mut u32 = &mut 12;
    println!("a =  {}",a);
    *a = 24;
    println!("avant {}",a);
    ma_fonction_ref(a);
    println!("apres {}",a);

    //  Une fonction simple avec une chaine
    fn ma_fonction_string_ref( x: &String) {
        println!("entree = {}", x);

    }
    let hello = String::from("Hello");
    ma_fonction_string_ref(&hello);
    println!(" {}",hello);   // Ne genere plus d'erreur:

    // Modification de la chaine donnee en entree
    fn ma_fonction_string_ref_mut( x: &mut String) {
	x.push_str(", world!");
    }
    let mut hello = String::from("Hello");
    ma_fonction_string_ref_mut(&mut hello);
    println!(" {}",hello);

    // Le cas de  println!
    // println! est une macro. Elle fonctionna un peu comme une fonction.
    // Cependnant
    let hello = String::from("Hello");
    println!(" {}",hello);
    let coucou = hello; // Cette instruction marche. Alors  qu'on lui fournit un variablle sans passer par une reference.                      
    // On s'attend à ce que la variabke "hello" soit  possédée par println! et donc ne soit plus accessible. (cf ligne 43)
    // En fait println! fait automatiquement un appel par référence, pour des raisons pratiques. 
    // Mais cela a des conséquuences:
	let mut variable = 3;
	println!("variable = {}",variable);	
    let ref_variable = &mut variable;
	*ref_variable = 6;
	println!("variable = {}",variable);	
	// println!("ref_variable = {}",ref_variable); // <- cette instruction ne marche pas
    // parce que println! emprunte "variable" en creant une reference mutable vers "variable".
    // Cette action a supprimée  la reference mutable précédente "ref_variable", 

    // Pas de problème si on a une variable non mutable: 
	let variable = 3;
	println!("variable = {}",variable);	
    let ref_variable = & variable;
	let test = *ref_variable + 6;
	println!("variable = {}",variable);	
	println!("ref_variable = {}",ref_variable); // <- cette instruction marche 

}
