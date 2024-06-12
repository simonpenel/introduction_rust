// Pour comprendre l'organisation en modules:
//https://larouille.github.io/modules/#mettre-notre-module-dans-un-dossier-dedie

pub mod agenda;

use std::env;
use std::process;
use crate::agenda::premier_jour;
use crate::agenda::read_agenda;
use crate::agenda::programme_du_jour_map_fold;

fn main() {
    // CLI tres rudimentaire
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        eprintln!("Entrer 1 argument s'il vous plait!");
        process::exit(1);

    }
    let file_path = &arguments[1];
    let mon_agenda  = match read_agenda(String::from(file_path)){
    	Ok(agenda) => agenda,
    	Err(e) => {        
    		match e {
    			1 => eprintln!("Le fichier n'existe pas."), 
    			2 => eprintln!("Le fichier ne semble pas au bon format."), 
    			3 => eprintln!("Le jour du mois doit être un entier."), 
    			_ => eprintln!("Une erreur s'est produite."), 
    		}
        	process::exit(1);}
    }
    ;
    println!("Les 160 premiers jours : ");
    for i in premier_jour().take(160) {
        println!("> {}", i);
        match programme_du_jour_map_fold(&i,&mon_agenda) {    
            Some(s) => { println!("\nAttention aujourd'hui {} :\n{}", i,s)},
            None => {},
        }

    }
}


