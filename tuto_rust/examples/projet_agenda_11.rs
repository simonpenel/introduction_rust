// IMPLEMENTER UN ITERATEUR POUR UNE STRUCTURE
// ===========================================

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

/// Enum JourSemaine
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, PartialEq, Clone, Copy)]
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

// Le trait "Display" (necessaire pour utiliser println!) n'existe pas pour notre structure.
// Nous allons le definir
impl std::fmt::Display for Jour {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "le {:?} {} {:?}",
            self.jour_sem, self.jour_mois, self.mois
        )
    }
}

// Implemente le trait "Iterator" pour  `Jour`.
impl Iterator for Jour {
    type Item = Jour;
    // La methode next renvoie le prochain et met a jour le courant
    fn next(&mut self) -> Option<Self::Item> {
        let jsem = &self.jour_sem;
        let jmois = &self.jour_mois;
        let mois = &self.mois;
        let jsem_prochain = renvoie_demain_semaine(jsem);
        let jmois_prochain = match *jmois < 30 {
            true => jmois + 1,
            false => 1,
        };
        let mois_prochain = match *jmois < 30 {
            true => *mois,
            false => renvoie_mois_suivant(mois),
        };
        self.jour_mois = jmois_prochain;
        self.jour_sem = jsem_prochain;
        self.mois = mois_prochain;
        // Pour faire ça, il faut que  JourSemaine ait le trait Copy/Clone
        // Les perfomances du programme s'en ressentiront
        let dem = Jour {
            jour_sem: self.jour_sem,
            jour_mois: self.jour_mois,
            mois: self.mois,
        };
        // Cet iterateur n'a pas de dernier element. (Exercice : a faire)
        Some(dem)
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
// Implemente des fonctions pour la structure Date
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
    // On utilise une fonction pour lire l'agenda
    let mon_agenda = read_agenda(String::from("date_naissance.txt"));
    println!("Les 160 premiers jours : ");
    // premier_jour renvoie une varaible de type Jour.
    // On a défini un iterateur pour ce type.
    // On peut donc faire une boucle dessus, et utiliser les méthodes liées au trait Iterator.
    // On utilise la methode take qui renvoie un iterateur des 160 premières iterations.
    for i in premier_jour().take(160) {
        println!("> {}", i);
        match programme_du_jour_map_fold(&i, &mon_agenda) {
            Some(s) => {
                println!("\nAttention aujourd'hui {} :\n{}", i, s)
            }
            None => {}
        }
    }
}

// Fonction premier jour (Different de la fonction Date::premier_jour())
fn premier_jour() -> Jour {
    Jour {
        jour_sem: JourSemaine::Lundi,
        jour_mois: 1,
        mois: Mois::Octobre,
    }
}

// Fonction qui remplit l'agenda
fn read_agenda(filename: String) -> Vec<Evenement> {
    let mut agenda_anniv: std::vec::Vec<Evenement> = Vec::new();
    let file = File::open(filename).expect("Erreur a l'ouverture");
    let reader = BufReader::new(file);
    println!("\nLecture:");
    for line in reader.lines() {
        // Line est du type Results
        let line = line.expect("Probleme de lecture");
        // Ici on utilise split() qui renvoie  un iterateur
        // La methode collect() place tous les elements dans une collection
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
        let split_line: Vec<&str> = line.split(',').collect();
        // On s'assure que l'on a 5 elements
        assert_eq!(split_line.len(), 5);

        // On recupere le prenom
        let prenom = split_line[1];
        // Virer les prenome multiples:
        // split est un iterator, la methode next renvoie le prochain element sous forme
        // de Option: soit il y a un prochain element ( Some(x)) soit il n'y en a pas (None)
        let prenom = prenom.split(' ').next();
        // on recuper ce qu'il y a dans Option en utilisant unwrap
        let prenom = prenom.unwrap();

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
        // On pourrait s'assurer que le jour du mois est <=31

        // On recupere le mois
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
        // On definit une date a partir du mois et du jour du mois
        let date = Date {
            jour_mois: jour_mois,
            mois: mois,
        };
        // On definit un evenement a partir du prenom
        let evt = Evenement {
            description: prenom.to_string() + "'s birthday",
            date: date,
        };
        agenda_anniv.push(evt);
    }
    agenda_anniv
}

// Fonction qui renvoie un variable de type Jour qui est
// le lendemain du jour donne en entree par reference
// ------------------------------------------------------
pub fn renvoie_demain(jour: &Jour) -> Jour {
    let jsem = &jour.jour_sem;
    let jmois = &jour.jour_mois;
    let mois = &jour.mois;

    let jsem_prochain = renvoie_demain_semaine(jsem);

    let jmois_prochain = match *jmois < 30 {
        true => jmois + 1,
        false => 1,
    };

    let mois_prochain = match *jmois < 30 {
        //true => *mois,
        true => *mois, // pour faire ça, On doit donner le trait Copy a Mois!
        false => renvoie_mois_suivant(mois),
    };

    let demain = Jour {
        jour_sem: jsem_prochain,
        jour_mois: jmois_prochain,
        mois: mois_prochain,
    };

    demain
}

// Fonction qui renvoie un variable de type JourSemaine qui est
// le lendemain du jour  de la semaine donne en entree par reference
// ------------------------------------------------------------------
pub fn renvoie_demain_semaine(jour: &JourSemaine) -> JourSemaine {
    let demain = match *jour {
        JourSemaine::Lundi => JourSemaine::Mardi,
        JourSemaine::Mardi => JourSemaine::Mercredi,
        JourSemaine::Mercredi => JourSemaine::Jeudi,
        JourSemaine::Jeudi => JourSemaine::Vendredi,
        JourSemaine::Vendredi => JourSemaine::Samedi,
        JourSemaine::Samedi => JourSemaine::Dimanche,
        JourSemaine::Dimanche => JourSemaine::Lundi,
    };
    demain
}

// Fonction qui renvoie un variable de type Mois qui est
// le mois suivant le mois en entree par reference
// ------------------------------------------------------
pub fn renvoie_mois_suivant(mois: &Mois) -> Mois {
    let mois_prochain = match *mois {
        Mois::Octobre => Mois::Novembre,
        Mois::Novembre => Mois::Decembre,
        Mois::Decembre => Mois::Juillet,
        Mois::Juin => Mois::Juillet,
        Mois::Juillet => Mois::Aout,
        Mois::Aout => Mois::Octobre,
    };
    mois_prochain
}

// Fonction qui renvoie les evenements dans l'agenda lies au jour.
// Utilise les methodes filter map et fold
// --------------------------------------------------------------
pub fn programme_du_jour_map_fold(jour: &Jour, agenda: &Vec<Evenement>) -> Option<String> {
    let iterateur = agenda.iter();
    let res = iterateur
        .filter(|e| evenement(jour, &e.date))
        .map(|e| &e.description)
        .fold(String::new(), |a, b| a + b + "\n");
    match res.len() {
        0 => None,
        _ => Some(res),
    }
}

// Fonction qui renvoie un boolean indiquany si une date et un jour correspondent
// ------------------------------------------------------------------------------
pub fn evenement(jour: &Jour, date: &Date) -> bool {
    match jour.jour_mois == date.jour_mois {
        true => jour.mois == date.mois,
        false => false,
    }
}
