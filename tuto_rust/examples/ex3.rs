#[allow(dead_code)]
#[allow(unused)]
// ===============
//  LES FONCTIONS
// ===============
//  Voir https://doc.rust-lang.org/book/ch03-03-how-functions-work.htmlhttps://doc.rust-lang.org/book/ch03-03-how-functions-work.html

fn main() {
    // Une fonction très simple qui prend un entier u32 comme paramètre
    // (pour plus de clarté on déclare les fonctions ici, mais il serait mieux
    // de les déclarer en dehors  de la fonction  main)
    // Une fonction se déclare avec "fn" et on définit le type de l'argument :
    fn ma_fonction(x: u32) {
        println!("entrée = {}", x)
    }
    let a: u32 = 12;
    ma_fonction(a);
    println!(" {}", a);

    ma_fonction(66); // Ok
                     // ma_fonction(3.5);   // erreur

    // Si on veut modifier la variable donnée comme paramètre:
    fn ma_fonction_2(x: u32) {
        println!("entrée = {}", x);
        // x = x + 5; // Génére une erreur
    }
    // La variable doit etre mutable
    fn ma_fonction_3(mut x: u32) {
        println!("entrée = {}", x);
        x = x + 5;
        println!("nouvelle entrée = {}", x);
    }
    println!("avant {}", a);
    ma_fonction_3(a);
    println!("après {}", a);
    // On constate que la variable  n'a pas changé après la fonction

    // La même fonction très simple avec une chaine comme paramètre
    fn ma_fonction_string(x: String) {
        println!("entrée = {}", x)
    }
    let hello = String::from("world");
    println!("Hello  {}!", hello);
    ma_fonction_string(hello);
    // let coucou = hello;  // Genere une erreur: la chaîne a été empruntée par
    // la fonction: elle n'est plus accessible.

    // Dans les examples précédents la fonction devient propriétaire de la
    // variable donnée en entrée, il s'agit de ownership comme dans
    // l'instruction 'coucou = hello'. (qui ne pose pas de problème avec les
    // entiers). C'est la raison pour laquelle, dans le cas des entiers la
    // variable n'est pas modifiée car il s'agit en fait d'une copie de la
    // variable, et dans le cas des String il n'est plus possible d'accéder à
    // la variable après qu'elle soit devenue la propriété de la fonction.
    //
    // La solution : l'appel par référence

    // APPEL PAR REFERENCE
    // ===================

    // On va utiliser une fonction qui prend la référence vers la variable
    // plutot que la variable elle meme.
    // Info sur les références :
    // https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

    fn ma_fonction_ref(x: &mut u32) {
        println!("entrée = {}", x);
        *x = 54;
        println!("nouvelle entrée = {}", x);
    }
    let mut a: u32 = 12;
    println!("avant {}", a);
    ma_fonction_ref(&mut a);
    println!("après {}", a);
    // on constate que la variable a bien  changé

    // alternative : a est ici une référence vers une valeur mutable
    let a: &mut u32 = &mut 12;
    println!("a =  {}", a);
    *a = 24;
    println!("avant {}", a);
    ma_fonction_ref(a);
    println!("après {}", a);

    //  Une fonction simple avec une chaîne
    fn ma_fonction_string_ref(x: &String) {
        println!("entrée = {}", x);
    }
    let hello = String::from("Hello");
    ma_fonction_string_ref(&hello);
    println!(" {}", hello); // Ne génère plus d'erreur:

    // Modification de la chaîne donnée en entree
    fn ma_fonction_string_ref_mut(x: &mut String) {
        x.push_str(", world!");
    }
    let mut hello = String::from("Hello");
    ma_fonction_string_ref_mut(&mut hello);
    println!(" {}", hello);

    // Le cas de  println!
    // -------------------
    // println! est une macro. Elle fonctionne un peu comme une fonction.
    // Cependant
    let hello = String::from("Hello");
    println!(" {}", hello);
    let coucou = hello; // Cette instruction marche. Alors  qu'on a fourni à
                        // println!  une variable de type String sans passer par une référence. On
                        // s'attend à ce que la variable "hello" soit  possédée par println! et
                        // donc ne soit plus accessible. (cf ligne 47). En fait println! fait
                        // automatiquement un appel par référence, pour des raisons pratiques. Mais
                        // cela a des conséquuences:
    let mut variable = 3;
    println!("variable = {}", variable);
    let ref_variable = &mut variable;
    *ref_variable = 6;
    println!("variable = {}", variable);
    // *ref_variable = 12; //<- cette instruction ne marche pas
    // parce que println! emprunte "variable" en créant une référence mutable vers "variable".
    // Cette action a supprimé  la référence mutable précédente "ref_variable".

    // Pas de problème si on a une variable non mutable:
    let variable = 3;
    println!("variable = {}", variable);
    let ref_variable = &variable;
    let test = *ref_variable + 6;
    println!("variable = {}", variable);
    println!("ref_variable = {}", ref_variable); // <- cette instruction marche
}
