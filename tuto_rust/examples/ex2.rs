#[allow(unused)]

// ========================================
//  L'OWNERSHIP ET LE BORROWING, LE CLONAGE
// ========================================

fn main() {
    println!("Hello, world!");

    // ===============
    // L'"ownership"
    // ===============
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
    // https://doc.rust-lang.org/rust-by-example/scope/move.html
    //
    // Rêgle de l'"ownership":
    // ----------------------
    // En  Rust  chaque variable est propriétaire d’un emplacement mémoire.
    // Il ne peut y avoir qu’un seul propriétaire à la fois pour cet
    // emplacement.
    // Lorsqu’un emplacement est assigné à une autre variable, il est transféré
    // de la variable source à la variable cible. La variable source n’a plus
    // accès à l'emplacement mémoire et la variable cible devient le nouveau
    // propriétaire.
    // Dans quel but:  maintenir la mémoire propre:
    // - éviter les comportements imprévisibles avec les opérations de
    //   lectures/écritures
    // - nettoyer rapidement les données inutilisées.

    println!("Ex 2.1 ");
    let hello = String::from("Hello, world!");
    let coucou = hello;
    // hello n'est plus accessible
    // cette instruction engendre une erreur:
    // let salut  = hello;

    // Cas des variables "simples"
    // --------------------------
    // L'ownsership ne pose pas de problême avec les variables de type entier.
    // Dans ces cas-là, la variable est en fait copiée automatiquement, car
    // cela ne coûte pas cher, contrairement  à une variable  complexe
    // (allocation dynamique, taille variable, structures,…).

    println!("Ex 2.2 ");
    let hello = 1;
    let coucou = hello;
    // hello est tojours accessible
    // cette instruction marche:
    let salut = hello;

    // ==================================
    // Les références et le borrowing
    // ==================================
    // Si on veut garder l'accès à un emplacement mémoire, on utilise les références.
    // Une réference est un type de données qui représente un emplacement mémoire.
    // L'opérateur & permet d'obtenir l'emplacement mémoire assigné à une variable.
    // &x est l'emplacement mémoire  de x
    // L'action de créer une référence est appelée le "borrowing"
    println!("Ex 2.3 ");
    let coucou = String::from("Hello, world!");
    let bonjour = &coucou;
    let salut = &coucou;
    // Pour récupérer la valeur à l'emplacement mémoire décrit par la réference on utilise l'opérateur *
    println!("{}", *bonjour);
    // Pour afficher la réference:
    println!("{:p}", bonjour);
    // Note on utilise {:p} pour forcer println! à ecrire l'adresse et non la valeur qui s'y trouve
    // ce qui est le comportement par défaut de println!.
    // La variable coucou est toujours disponible:
    println!("{}", coucou);
    println!("{}", *salut);

    // Rappel sur l'ownership avec les entiers  (mutables ou non)
    // ===========================================================
    println!("Ex 2.4 ");
    let variable_1 = 3;
    let variable_2 = variable_1;
    let variable_3 = variable_1; // Cette ligne ne génere pas d'erreur

    // Rappel sur l'ownership avec les String
    // ======================================
    println!("Ex 2.5 ");
    let variable_1 = String::from("Hello, world!");
    let variable_2 = variable_1;
    //let variable_3 = variable_1; // Cette ligne  génere une erreur

    // Les réferences non mutables
    // ============================
    println!("Ex 2.6 ");
    let variable = 3;
    let ref1_variable = &variable;
    println!("variable = {}", variable);
    println!("ref1_variable = {}", ref1_variable);

    // Plusieures références non mutables : ok
    // --------------------------------------
    let ref2_variable = &variable;
    let ref3_variable = &variable;
    println!("variable = {}", variable);
    println!("ref2_variable = {}", ref2_variable);
    println!("ref3_variable = {}", ref3_variable);
    // variable est toujours  accessible
    println!("variable = {}", variable);

    // Les références mutables
    // =======================

    // ATTENTION:
    // Lorsque l'on déclare une reference mutable
    // ref_variable_toto = &mut variable_toto
    // &mut représente un accès exclusif, et interdit tout usage de la variable
    // qui ne passe pas par la référence.
    // Ici , ref_variable_toto a l'exclusivité de l'accès à variable_toto,
    // jusqu'a sa dernière utilisation.

    // Une règle simple:
    //  * une variable non mutable peut être empruntée par autant de références
    //    non mutables que l'on veut
    //  * une variable  mutable ne peut être empruntée que par une référence
    //    mutable unique.
    //  Pourquoi ? On peut faire le parallèle avec les fichiers en lecture ou
    //  en lecture/écriture:
    //  Un fichier en lecture seule peut etre accedé en même temps par
    //  différentes parties du code
    //  Un fichier en lecture/ecriture ne devrait  être accédé en même temps
    //  que par une seule partie du code  pour éviter des erreurs potentielles.

    // une seule référence mutable : ok
    // ---------------------------------
    println!("Ex 2.8 ");
    let mut variable = 3;
    let ref_variable = &mut variable;
    *ref_variable = 4;
    println!("variable = {}", variable);

    // plusieures références mutables : erreur
    // -------------------------------------

    println!("Ex 2.9 ");

    let mut variable = 3;
    let ref1_variable = &mut variable;
    let ref2_variable = &mut variable;

    // *ref1_variable = 4; // <- Cette instruction va déclencher une erreur
    // En effet la nouvelle référence "ref2_variable" emprunte la variable
    // "variable", ce qui entraîne la disparition  de la référence précédente
    // "ref1_variable".

    *ref2_variable = 4;
    let test = ref2_variable;

    // 1 référence mutable et une référence non mutable  : erreur aussi
    // ----------------------------------------------------------------

    println!("Ex 2.10 ");
    let mut variable = 3;
    let ref1_variable = &variable; // immutable borrow
    let ref2_variable = &mut variable; // mutable borrow

    // let test =  ref1_variable; // <- Cette instruction va déclencher une erreur
    // En effet la nouvelle référence "ref2_variable" emprunte la variable
    // "variable", ce qui entraine la disparition  de la référence précédente
    // "ref1_variable".

    *ref2_variable = 4; // mutable borrow  <- Cette instruction ne  déclenche pas d'erreur

    let test = ref2_variable; // Remarque : après ça, ref2_variable n'existe plus. (à cause de l'ownership!)
    println!("variable = {}", variable);

    // Plus d'info
    // https://medium.com/@manikandan96372/rust-for-beginners-part-7-borrowing-reference-mutable-borrow-immutable-borrow-5c0e5c84e1ef
    // https://dhghomon.github.io/easy_rust/Chapter_17.html
    // https://users.rust-lang.org/t/why-is-this-println-s-treated-as-an-immutable-borrow/78870

    // Le clonage: 
    // ===========
    // Cela permet de dupliquer une variable, mais cela a un coût en terme de performance

    println!("Ex 2.11 ");
    let hello = String::from("Hello, world!");
    //  On duplique la variable:
    let coucou = hello.clone();
    // hello est toujours accessible
    print!("{}",hello);
    print!("{}",coucou);



}
