// LES ITERATEURS (ITERATORS), LE CHAINAGE DE METHODES
// ===================================================

// use std::collections::binary_heap::Iter;

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
#[derive(Debug, PartialEq)] // Ici on ajoute le Trait "PartialEq" aux traits de Mois, afin de pouvoir utiliser l'opérateur "==" ligne 179
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
            mois: Mois::Novembre,
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
    // Instanciation d'1 évènement à partir d'une date
    let date_hpl = Date {
        jour_mois: 20,
        mois: Mois::Aout,
    };
    let hpl = Evenement {
        description: "Anniversaire de Howard".to_string(),
        date: date_hpl,
    };

    // Instanciation directe
    let ms = Evenement {
        description: "Anniversaire de Mary".to_string(),
        date: Date {
            jour_mois: 30,
            mois: Mois::Aout,
        },
    };

    // Instanciation en utilisant la fonction new de Date
    let ulg = Evenement {
        description: "Anniversaire de Ursula".to_string(),
        date: Date::new(21, Mois::Octobre),
    };
    let bs = Evenement {
        description: "Anniversaire de Bram".to_string(),
        date: Date::new(8, Mois::Novembre),
    };
    let jp = Evenement {
        description: "Anniversaire de John".to_string(),
        date: Date::new(30, Mois::Aout),
    };

    // Création d'un vecteur d'évènements vide
    let mut agenda_anniv: Vec<Evenement> = Vec::new();
    agenda_anniv.push(hpl);
    agenda_anniv.push(ms);
    agenda_anniv.push(ulg);
    agenda_anniv.push(bs);
    agenda_anniv.push(jp);

    // agenda_anniv est un vecteur.
    // Les vecteurs ont le trait "Iterator" qui permet d'itérer sur chaque élément du vecteur
    // Le trait Iterator fournit un grand nombre de méthodes qui peuvent être utilisée sur les
    // types quit possèdent ce trait, par exemple les vecteurs.

    // Tester ceci

    // for evenement in agenda_anniv {
    // println!("{} c'est {}",evenement.date, evenement.description);
    // }
    println!("\nprojet_agenda_7.1");
    // et maintenant cette commande ne marche plus
    println!("{:?}", agenda_anniv);

    // Que s'est  t il passé ?
    // Le compilateur l'explique

    // Le compilateur propose:
    for evenement in &agenda_anniv {
        println!("{} c'est {}", evenement.date, evenement.description);
    }
    // et maintenant ca marche
    println!("{:?}", agenda_anniv);

    println!("\nprojet_agenda_7.2");
    // Utilisation d'un itérateur
    // La boucle "for" utilise implicitement  la methode "into_iter" fournie par trait Iterator
    // la commande exécutée est en fait "for evenement in agenda_anniv.into_iter()"
    // la méthode into_iter() renvoie un itérateur à partir d'une collection (par exemple un vecteur)
    // into_iter() transmet l'owwnership du vecteur à l'itérateur.
    // Il existe une autre méthode, iter() qui emprunte (borrowing) le vecteur
    // https://blog.coolhead.in/difference-between-intoiter-iter-and-itermut-in-rust
    // A la place de la boucle sur le vecteur, on peut utiliser explicitement l'itérateur:
    let iterateur = agenda_anniv.iter(); // Et pas into_iter() ! 
    // Ensuite on fait la boucle sur l'itérateur
    for evenement in iterateur {
        println!(
            "Et encore, {} c'est toujours {}",
            evenement.date, evenement.description
        );
    }

    // ca marche toujours
    println!("{:?}", agenda_anniv);

    // les méthodes associées aux  itérateurs sont nombreuses
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
    // Certaines méthodes prennent un itérateur comme arguments et renvoient
    // un nouvel itérateur.

    // "Functions which take an Iterator and return another Iterator are often
    // called ‘iterator adapters’, as they’re a form of the ‘adapter pattern’.
    // Common iterator adapters include map, take, and filter."

    // quelques exemples
    // ------------------

    // La méthode enumerate
    println!("\nprojet_agenda_7.3");
    let iterateur = agenda_anniv.iter();
    let iterateur_avec_index = iterateur.enumerate();
    for (i, eve) in iterateur_avec_index {
        println!("Evenement numero {} : {:?}", i, eve);
    }

    println!("\nprojet_agenda_7.4");
    // On peut "chaîner" les méthodes:
    let iterateur = agenda_anniv.iter().enumerate();
    for (i, eve) in iterateur {
        println!("Evnement numero {} : {:?}", i, eve);
    }

    // La méthode filter
    println!("\nprojet_agenda_7.5");
    // nécessite l'ajout de
    // #[derive(PartialEq)]
    // à la struture Mois
    let iterateur = agenda_anniv
        .iter()
        .filter(|x| x.date.mois == Mois::Aout) // <-  nécessite  #[derive(PartialEq)]
        .enumerate();
    for (i, eve) in iterateur {
        println!("Evenement en Aout numero {} : {:?}", i, eve);
    }
}
