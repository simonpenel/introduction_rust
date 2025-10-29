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
        // Cet iterateur n'a pas de dernier element. (Exercic : a faire)
        if jmois_prochain == 1 && mois_prochain == Mois::Octobre {
            println!("Fin de l'année atteint  demain : {}", dem);
            None
        } else {
            Some(dem)
        }
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
    // premier_jour renvoie une varaible de type Jour.
    // On a défini un iterateur pour ce type.
    // On peut donc faire une boucle dessus, et utiliser les méthodes liées au trait Iterator.
    // On utilise la methode take qui renvoie un iterateur des 160 premières iterations.
    for i in premier_jour() {
        println!("> {}", i);
    }
}

// Fonction premier jour (Different de la fonction Date::premier_jour())
fn premier_jour() -> Jour {
    Jour {
        jour_sem: JourSemaine::Lundi,
        jour_mois: 25,
        mois: Mois::Juillet,
    }
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

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
