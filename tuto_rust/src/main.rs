// Pour comprendre l'organisation en modules:
//https://larouille.github.io/modules/#mettre-notre-module-dans-un-dossier-dedie

pub mod agenda;

use std::env;
use std::process;

// Crates internes
use crate::agenda::premier_jour;
use crate::agenda::programme_du_jour_map_fold;
use crate::agenda::read_agenda;
use crate::agenda::{Date, Mois}; // Utile pour la fin du code (utilisation du crate local)

// Crates externes : attention meme structure Date
use gregorian::{Date as GregorianDate, Month::*, YearMonth};

fn main() {
    // CLI tres rudimentaire
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        eprintln!("Entrer 1 argument s'il vous plait!");
        process::exit(1);
    }
    // Decoration les characteres en rust.
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    let file_path = &arguments[1];
    // Lecture de l'agenda
    let mon_agenda = match read_agenda(String::from(file_path)) {
        Ok(agenda) => agenda,
        Err(e) => {
            match e {
                1 => eprintln!("Le fichier n'existe pas ou n'est pas lisible."),
                2 => eprintln!("Le fichier ne semble pas au bon format."),
                3 => eprintln!("Le jour du mois doit être un entier."),
                _ => eprintln!("Une erreur s'est produite."),
            }
            process::exit(1);
        }
    };
    // Explore les 160 premiers jours du calendrier
    println!("Les 160 premiers jours : ");
    for i in premier_jour().take(190) {
        println!("> {}", i);
        match programme_du_jour_map_fold(&i, &mon_agenda) {
            // Ceci ne marche pas: le champs jour_sem est privé
            //Some(s) => { println!("\nAttention aujourd'hui {} :\n{}", i.jour_sem,s)},
            Some(s) => {
                println!("\nAttention aujourd'hui {} {}:\n{}", i, sparkle_heart, s)
            }
            None => {}
        }
    }
    // Example de connflit entre crates interne et externe
    // Utilisation du crates local
    let date = Date::new(2, Mois::Aout);
    println!("Tuto_rust date {:?}", date);
    // Utilise le crates externe  calendrier gregorien
    assert!(YearMonth::new(1900, February).total_days() == 28);
    assert!(YearMonth::new(2000, February).total_days() == 29);
    let date = GregorianDate::new(2020, 1, 31).unwrap();
    println!("Gregorian date : {:?}", date);
}
