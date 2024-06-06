
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

}
