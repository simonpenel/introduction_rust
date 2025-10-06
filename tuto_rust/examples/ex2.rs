
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


	// Rappel sur le borowing sur les entiers mutables
	// ===============================================
	println!("Ex 2.5 ");

    let mut variable_1 = 3;
	let variable_2 = variable_1;
	variable_1 = variable_1 + 1;


	println!("variable_1 = {}",variable_1);
	println!("variable_2 = {}",variable_2);


	// References mutables


	let mut variable_1 = 3;

    let ref_variable_1 = & variable_1;

	//  *ref_variable_1 = 4;

	// println!("{}",variable_1);
    println!("{}",ref_variable_1);
	println!("{}",variable_1);
    println!("{}",ref_variable_1);
	// voir  https://dhghomon.github.io/easy_rust/Chapter_17.html

}
