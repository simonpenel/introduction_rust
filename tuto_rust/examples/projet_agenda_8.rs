// PLUS LOIN AVEC LES ITERATORS, LECTURE DE FICHIER, METHODES EXPECT ET UNWRAP
// ===========================================================================

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

/// Enum JourSemaine
#[derive(Debug)]
pub enum JourSemaine {
    Lundi,
    Mardi,
    Mercredi,
    Jeudi,
    Vendredi,
    Samedi,
    Dimanche,
}

/// Enum Mois
#[derive(Debug, PartialEq)]
pub enum Mois {
    Octobre,
    Novembre,
    Decembre,
    Juin,
    Juillet,
    Aout,
}

/// Structure Jour
#[derive(Debug)]
pub struct Jour {
    /// jour de la semaine
    jour_sem: JourSemaine,
    /// jour du mois
    jour_mois: u32,
    /// mois
    mois: Mois,
}
impl std::fmt::Display for Jour {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "le {:?} {} {:?}",
            self.jour_sem, self.jour_mois, self.mois
        )
    }
}

/// Structure Date
#[derive(Debug)]
pub struct Date {
    /// jour du mois
    jour_mois: u32,
    /// mois
    mois: Mois,
}
impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.mois {
            Mois::Octobre => write!(f, "le {} du mois d'{:?}", self.jour_mois, self.mois),
            Mois::Aout => write!(f, "le {} du mois d'{:?}", self.jour_mois, self.mois),
            _ => write!(f, "le {} du mois de {:?}", self.jour_mois, self.mois),
        }
    }
}
// Implémente des fonctions pour la structure Date
impl Date {
    pub fn new(j: u32, m: Mois) -> Self {
        Self {
            jour_mois: j,
            mois: m,
        }
    }
    pub fn premier_jour() -> Self {
        Self {
            jour_mois: 1,
            mois: Mois::Octobre,
        }
    }
    pub fn affiche(&self) {
        println!("------ {:?}/{} --------", self.mois, self.jour_mois)
    }
}

/// Structure Evenement
#[derive(Debug)]
pub struct Evenement {
    description: String,
    date: Date,
}

fn main() {
    println!("\nprojet_agenda_8.1");
    let mut agenda_anniv: std::vec::Vec<Evenement> = Vec::new();
    // On veut remplir un agenda à partir d'un fichier date_naissance.txt
    // Ouverture d'un fichier:
    let file = File::open("date_naissance.txt");
    // file est du type Results, qui ressemble un peu au type Option
    // https://doc.rust-lang.org/std/result/enum.Result.html
    println!("File = {:?}", file);
    // On peut utiliser le matching
    let file_resu = match file {
        Ok(file_ok) => file_ok,
        Err(erreur) => {
            eprintln!("Erreur : {:?}", erreur);
            panic!("Erreur a l'ouverture du fichier")
        }
    };
    println!("File : {:?}", file_resu);

    println!("\nprojet_agenda_8.2");
    // On pourait aussi ecrire tout simplement avec le meme nom de variable
    let file = File::open("date_naissance.txt");
    let file = match file {
        Ok(file) => file,
        Err(erreur) => {
            eprintln!("Erreur : {:?}", erreur);
            panic!("Erreur a l'ouverture du fichier")
        }
    };
    println!("File : {:?}", file);

    // Test sur un fichier bidon
    // let file = File::open("bidon.txt");
    // let file = match file {
    //     Ok(file) => file,
    //     Err(erreur) => {
    //         eprintln!("Erreur : {:?}",erreur);
    //         panic!("Erreur a l'ouverture du fichier")
    //         },
    // };
    // println!("File : {:?}",file);

    println!("\nprojet_agenda_8.3");
    // C'est un peu lourd, on peut simplifier:
    let file = match File::open("date_naissance.txt") {
        Ok(file) => file,
        Err(erreur) => {
            eprintln!("Erreur : {:?}", erreur);
            panic!("Erreur a l'ouverture du fichier")
        }
    };
    println!("File : {:?}", file);

    // Mais c'est toujours un peu lourd d'ecrire ce code à chaque fois.
    // Si on veut un code plus compact on peut utiliser expect() (ou unwrap())
    // https://learning-rust.github.io/docs/unwrap-and-expect/
    // Unwrap in Rust returns the result of the operation for Option and Result enums.
    // If unwrap encounters an error Err or a None, it will panic and stop the program execution.
    // Unwrap method is defined on both Option and Result type.
    println!("\nprojet_agenda_8.4");
    let file = File::open("date_naissance.txt").expect("Erreur à l'ouverture");
    println!("File : {:?}", file);

    // Lecture d'un fichier
    println!("\nprojet_agenda_8.5");
    let reader = BufReader::new(file);
    println!("\nLecture:");
    for line in reader.lines() {
        // line est du type Results
        println!("line = {:?}", line);
        let line = line.expect("Probleme de lecture");
        // line est maintenant du type String
        println!("line = {:?}", line);
        // Ici on utilise split() qui renvoie  un iterateur
        // La methode collect() place tous les elements dans une collection
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
        let split_line: Vec<&str> = line.split(',').collect();
        println!("split_line = {:?}", split_line);
        // On s'assure que l'on a 5 elements
        assert_eq!(split_line.len(), 5); // assert_eq va renvoyer un erreur si les deux termes split_line.len() et 5 sont differents
                                         // On recupere le prenom
        let prenom = split_line[1];
        println!("prenom = {}", prenom);
        // Virer les prenoms multiples:
        // split est un iterator, la methode next renvoie le prochain element sous forme
        // de Option: soit il y a un prochain element ( Some(x)) soit il n'y en a pas (None)
        let prenom = prenom.split(' ').next();
        println!("first element of splited prenom = {:?}", prenom);
        // on recuper ce qu'il y a dans Option en utilisant unwrap
        let prenom = prenom.unwrap();
        println!("first element of splited prenom = {:?}", prenom);
        // On recupere le jour du mois
        let jour_mois = split_line[2];
        // C'est une chaine de caracteres
        // On veut etre sur que le jour du mois est un entier
        let jour_mois = match jour_mois.parse::<u32>() {
            Ok(valeur) => valeur,
            Err(_err) => {
                eprintln!("Erreur, le jour du mois doit etre un entier");
                process::exit(1);
            }
        };
        println!("Jour du mois = {}", jour_mois);
        // On pourrait s'assurer que le jour du mois est <=31

        // On recupere le mois avec le pattern matching
        let mois = match split_line[3] {
            "June" => Mois::Juin,
            "July" => Mois::Juillet,
            "August" => Mois::Aout,
            "October" => Mois::Octobre,
            "November" => Mois::Novembre,
            "December" => Mois::Decembre,
            _ => {
                eprintln!("Erreur, je connais pas ce mois.");
                process::exit(1);
            }
        };
        println!("Mois = {:?}", mois);
        // On definit une date a partir du mois et du jour du mois
        let date = Date {
            jour_mois: jour_mois,
            mois: mois,
        };
        println!("Date = {:?}", date);
        // On definit un evenement a partir du prenom
        let evt = Evenement {
            description: prenom.to_string() + "'s birthday",
            date: date,
        };
        agenda_anniv.push(evt);
    }
    println!("\nAGENDA {:?}", agenda_anniv);
}
