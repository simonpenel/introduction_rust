// IMPLEMENTER UN TRAIT POUR UNE STRUCTURE
// =======================================

// Un trait décrit une fonctionnalité qu'a un type particulier et qu'il peut
// partager avec d'autres types. Cette fonctionalité fournit un ensemble de
// méthodes aux types qui possèdent se trait. Par exemple il existe le trait
// qui permet de savoir si une variable est supérieure ou inférieure à une
// autre. Ce trait est possédé par le type entier et par le type chaîne de
// caractères. Si on crée une structure, on peut avoir besoin de les classer.
// Il faudra alors lui associer ce trait, et éventuellement l'implémenter.

// Ici on a spécifié que l'enum a le trait Debug : #[derive(Debug)]
// Ce trait permet d'utiliser le format {:?} dans println!("{:?}",toto) pour afficher l'enum toto.
// Mais on voudrait que notre structure ait le trait Display  qui permet de l'afficher avec println!("{}",toto).
// La plupart des  types ont le trait Display, mais ici il faut que nous définissions comment la
// stucture doit être affichée. Pour cela il faut que le trait Display soit implémenté.

// All types which want to use std::fmt formatting traits require an implementation to be printable.
// Automatic implementations are only provided for types such as in the std library.
// All others must be manually implemented somehow.
// The fmt::Debug trait makes this very straightforward. All types can derive (automatically create) the fmt::Debug implementation.
// This is not true for fmt::Display which must be manually implemented.

// More info on Debug and Display https://practice.course.rs/formatted-output/debug-display.html

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

#[derive(Debug)]
pub enum Mois {
    Octobre,
    Novembre,
    Decembre,
    Juillet,
    Aout,
}

// On définit la structure Jour
// ----------------------------
#[derive(Debug)]
pub struct Jour {
    /// jour de la semaine
    jour_sem: JourSemaine,
    /// jour du mois
    jour_mois: u32,
    /// mois
    mois: Mois,
}

// Le trait "Display" (nécessaire pour utiliser println!) n'existe pas pour
// notre structure. Nous allons l'implémenter.
// voir https://doc.rust-lang.org/std/fmt/trait.Display.html
impl std::fmt::Display for Jour {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "le {:?} {} {:?}",
            self.jour_sem, self.jour_mois, self.mois
        ) //<- définit comment println! va écrire Jour
    }
}

fn main() {
    let ajd = Jour {
        jour_sem: JourSemaine::Vendredi,
        jour_mois: 13,
        mois: Mois::Octobre,
    };
    println!("Aujourd'hui c'est {:?}", ajd);
    println!(
        "Aujourd'hui c'est {:?}, le {} du mois d'{:?}",
        ajd.jour_sem, ajd.jour_mois, ajd.mois
    );
    println!("Aujourd'hui c'est {}", ajd); // <- on peut utiliser println!

}
