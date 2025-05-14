
fn main() {
	println!("Hello, world!");

	// ==================================
	// Le "borowing" et le "ownership"
	// https://doc.rust-lang.org/rust-by-example/scope/move.html
	// En  Rust  chaque variable est propriétaire d’une valeur.
	// Il ne peut y avoir qu’un seul propriétaire à la fois pour cette valeur.
	// Lorsqu’une valeur est assignée à une autre variable,
	// elle est transférée de la variable source à la variable cible. La variable source
	// n’a plus accès à la valeur et la variable cible devient le nouveau propriétaire.
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
	// ==================================

	let bonjour = &coucou;
	println!("{}",bonjour);
	// println! recupere automatiquement
	println!("{}",*bonjour);
	println!("{}",coucou);

	// Le borrowing ne pose pas de probleme avec les variable de type entier.
	// La variable est copiée, cela ne coute pas cher, contrairement à une chaine de caracteres.

	let hello = 1;
	println!("{}",hello);
	let coucou = hello;
	println!("{}",coucou);
	// cette instruction marche:
	println!("{}",hello);
}
