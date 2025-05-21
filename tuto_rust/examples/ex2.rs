
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
	// ==================================

	let hello = String::from("Hello, world!");

	println!("{}",hello);

	let coucou = hello;

	println!("{}",coucou);
	// hello n'est plus accessible
	// cette instruction engendre une erreur:
	// println!("{}",hello);

	// ==================================
	// Les references
	// Une réference est type de données qui représente un emplacement mémoire.
	// L'opérateur & permet d'obtenir l'emplacement mémoire assigné à une variable.
	// &x est l'emplacement mémoire  de x
	// ==================================

	let bonjour = &coucou;


	// Pour récuperer la valeur à l'emplacement mémoire décrit par la réference on utilise l'opérateur *
	println!("{}",*bonjour);

	// Pour afficher la réference:
	println!("{:p}",bonjour);
	// Note on utilise {:p} pour forcer println! a ecrire l'adresse et non la valeur qui s'y trouve ce qui est le comportement par défaut

    // La variable coucou est toujours disponible:
	println!("{}",coucou);

	// Le borrowing ne pose pas de probleme avec les variable de type entier.
	// La variable est copiée automatiquement, car cela ne coute pas cher contrairement à une chaine de caracteres.

	let hello = 1;
	println!("{}",hello);
	let coucou = hello;
	println!("{}",coucou);
	// cette instruction marche:
	println!("{}",hello);
}
