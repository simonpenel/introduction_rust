 #[allow(unused)]

fn main() {
	println!("Hello, world!");

	// ==================================
	// Le "borowing" et le "ownership"
	// https://doc.rust-lang.org/rust-by-example/scope/move.html
	// En  Rust  chaque variable est propriétaire d’un emplacement mémoire.
	// Il ne peut y avoir qu’un seul propriétaire à la fois pour cet emplacement.
	// Lorsqu’un emplacement est assigné à une autre variable,
	// il est transféré de la variable source à la variable cible. La variable source
	// n’a plus accès à l'emplacement mémoire et la variable cible devient le nouveau propriétaire.
	// Dans quel but:  maintenir la mémoire propre:
	// - éviter les comportements imprévisibles avec les opérations de lectures/écritures
	// - nettoyer rapidement les données inutilisées.
	// ==================================

	println!("Ex 2.1 ");
	let hello = String::from("Hello, world!");
	println!("{}",hello);
	let coucou = hello;
	println!("{}",coucou);
	// hello n'est plus accessible
	// cette instruction engendre une erreur:
	// println!("{}",hello);

	// Cas des variables "simples"
	// --------------------------
	// Le borrowing ne pose pas de probleme avec les variable de type entier.
	// Dans ces cas la, la variable est en fait copiée automatiquement, car cela ne coute
	// pas cher, contrairement  à une variable  complexe (allocation dynamique, 
	// taille variable, structures,…).
	
	println!("Ex 2.2 ");
	let hello = 1;
	println!("{}",hello);
	let coucou = hello;
	println!("{}",coucou);
	// cette instruction marche:
	println!("{}",hello);


	// ==================================
	// Les references
	// Une réference est type de données qui représente un emplacement mémoire.
	// L'opérateur & permet d'obtenir l'emplacement mémoire assigné à une variable.
	// &x est l'emplacement mémoire  de x
	// ==================================
	
	println!("Ex 2.3 ");
	let coucou = String::from("Hello, world!");
	let bonjour = &coucou;
	// Pour récuperer la valeur à l'emplacement mémoire décrit par la réference on utilise l'opérateur *
	println!("{}",*bonjour);
	// Pour afficher la réference:
	println!("{:p}",bonjour);
	// Note on utilise {:p} pour forcer println! a ecrire l'adresse et non la valeur qui s'y trouve ce qui est le comportement par défaut
    // La variable coucou est toujours disponible:
	println!("{}",coucou);



	// Rappel sur le borowing sur les entiers 
	// ======================================
	println!("Ex 2.4 ");

    let variable_1 = 3;
	let variable_2 = variable_1;


	println!("variable_1 = {}",variable_1);
	println!("variable_2 = {}",variable_2);


	// Rappel sur le borowing sur les String
	// ======================================


    let variable_1 = String::from("Hello, world!");;
	let variable_2 = variable_1;


	// println!("variable_1 = {}",variable_1); // Ce code genere une erreur!
	println!("variable_2 = {}",variable_2);


	// Rappel sur le borowing sur les entiers mutables
	// ===============================================
	println!("Ex 2.5 ");

    let  variable_1 = 3;
	let mut variable_2 = variable_1;
	variable_2 = variable_2 + 1;


	println!("variable_1 = {}",variable_1);
	println!("variable_2 = {}",variable_2);


	// Les references non mutables
	// ======================================
    println!("Ex 2.6 ");
	let  variable = 3;

    let ref1_variable = & variable;

	println!("variable = {}",variable);
    println!("ref1_variable = {}",ref1_variable);

	// Plusieur references non mutables : ok
	// ----------------------------------------------------------------
	let ref2_variable = & variable;
	let ref3_variable = & variable;
	println!("variable = {}",variable);	
	println!("ref2_variable = {}",ref2_variable);
	println!("ref3_variable = {}",ref3_variable);
	// variable est toujours  accessible
	println!("variable = {}",variable);	

	// Les references mutables
	// =======================

	//  Dans ces exemples, on a un peu l'impression que le compilateur comprend  le code.
	//  Dans le contexte de references vers des variables mutables, il va generer une erreur
	//  seulement si nous essayons d'utiliser une variable  que nous devrions  plus utiliser.

    // une seule reference mutable : ok
	// ---------------------------------
	println!("Ex 2.8 ");
	let mut variable = 3;

	println!("variable = {}",variable);	

    let ref_variable = &mut variable;

	*ref_variable = 4;

	println!("variable = {}",variable);	


	// plusieur references mutables : erreur
	// -------------------------------------

	//  ici on ne fait pas de println! des variables  pour eviter de complexifier

	println!("Ex 2.9 ");
	let mut variable = 3;
    let ref1_variable = &mut variable;
	let ref2_variable = &mut variable;

	// *ref1_variable = 4; // <- Cette instruction va déclencher une erreur
	//  L'erreur est generee si on essaye d'utiliser la variable et non par la declaration des variables

	*ref2_variable = 4; // <- Cette instruction ne va pas déclencher une erreur

	// 1 reference mutable et une reference non mutable 
	// ------------------------------------------------

	//  ceci va marcher

	println!("Ex 2.10 ");
	let mut variable = 3;
    let ref1_variable = &variable;     // immutable borrow 
	let ref2_variable = &mut variable; // mutable borrow 

	*ref2_variable = 4; // mutable borrow  <- Cette instruction ne  déclenche pas d'erreur
	
	let test =  ref2_variable;
	// let test =  ref1_variable;; // Remarque : et cette instruction  déclenche une d'erreur

    
	// Mais Ceci ne va pas marcher   : on a juste inversé l'ordre de déclaration de ref2_variable et ref1_variable
	
	println!("Ex 2.11 ");
	let mut variable = 3;
	let ref2_variable = &mut variable; // mutable borrow 
    let ref1_variable = &variable;     // immutable borrow 

	// *ref2_variable = 4; // mutable borrow  <- Cette instruction   déclenche une erreur 
	// C'est logique : une fois que la variable a ete empruntée de maniere immutable, 
	// on ne peut plus l'emprunter de manière mutable 
	// 


    // Pourquoi println! fait des trucs bizares  
	// ========================================

    println!("Ex 2.11 ");
	let mut variable = 3;
	println!("variable = {}",variable);	

    let ref_variable = &mut variable;
	*ref_variable = 6;

	println!("ref_variable = {}",ref_variable); // <- cette instruction  marche

	println!("variable = {}",variable);	

	// println!("ref_variable = {}",ref_variable); // <- cette meme instruction ne marche plus  

    // parce que println! emprunte comme immutable puis comme mutable


	//  L'explication du compilateur:
	// 	A variable already borrowed with a certain mutability (either mutable or immutable) was borrowed again with a different
	// mutability.

	// Erroneous code example:

	// fn bar(x: &mut i32) {}
	// fn foo(a: &mut i32) {
	//     let y = &a; // a is borrowed as immutable.
	//     bar(a); // error: cannot borrow `*a` as mutable because `a` is also borrowed
	//             //        as immutable
	//     println!("{}", y);
	// }

	// To fix this error, ensure that you don't have any other references to the variable before trying to access it with a different
	// mutability:

	// fn bar(x: &mut i32) {}
	// fn foo(a: &mut i32) {
	//     bar(a);
	//     let y = &a; // ok!
	//     println!("{}", y);
	// }


	// testons ca

	fn bar(x: &mut i32) {
		if *x  == 0 {
			println!("Zero!")
		}
		else {
			println!("Pas zero!")
		}
		*x = 10;
	}


	//  Fonction qui marche
	fn foo(a: &mut i32) {
		bar(a);                // a est modifiée
		let y = &a; // ok!
		println!("{}", y);
	}
	//  Fonction qui ne marche pas 
	fn foo2(a: &mut i32) {
		let y = &a; // a is borrowed as immutable.
		bar(a); 	// error: cannot borrow `*a` as mutable because `a` is also borrowed
					//        as immutable
		// println!("{}", y); // <- Ceci genere une erreur
		// on comprend bien que y qui est immutable ne peut plus être utilise puisque la fonction bar l'a modifiee

	}


	let mut test = 0; 
	foo(&mut test);
	test = 1;
	foo(&mut test);

	//  Pour resumer:
	//  On peut faire le parallele avec les fichiers en lecture ou en lecture/ecriture
	//  Un fichier en lecture seule peut etre accedé en même temps par differentes parties du code
	//  Un fichier en lecture/ecriture ne devrait  être accédé en même temps que par une seule
	//  partie du code  pour eviter des erreurs potentielles.
	//  
	//  De la meme manière Rust interdit d'emprunter plusieur fois de maniere mutable une variable, ou d'emprunter
	//  de maniere mutable une variable  empruntee avant de maniere immutable



	// plus d'info : voir  https://dhghomon.github.io/easy_rust/Chapter_17.html

}
