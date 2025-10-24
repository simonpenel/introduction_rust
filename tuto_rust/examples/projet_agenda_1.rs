//  LES ENUMERATIONS (ENUM)
//  =======================

// On définit l'enum "JourSemaine"
#[derive(Debug)] // JourSemaine doit avoir le "trait" Debug pour pouvoir être
                 // affiché avec {:?} La notion de Trait sera abordée plus tard
                 // le mot-clef pub permet de rendre l'énumeration visible par le reste du code.
pub enum JourSemaine {
    Lundi,
    Mardi,
    Mercredi,
    Jeudi,
    Vendredi,
    Samedi,
    Dimanche,
}

fn main() {
    let ajd = JourSemaine::Lundi;
    println!("Aujourd'hui c'est {:?}", ajd);

    // Dans le contexte d'une enumération, le matching vérifie que tous les cas
    // sont testés. C'est une sécurité.
    // Ceci va générer un erreur:
    //    let demain = match ajd {
    //    	JourSemaine::Lundi => JourSemaine::Mardi,
    //    };

    let demain = match ajd {
        JourSemaine::Lundi => JourSemaine::Mardi,
        JourSemaine::Mardi => JourSemaine::Mercredi,
        JourSemaine::Mercredi => JourSemaine::Jeudi,
        JourSemaine::Jeudi => JourSemaine::Vendredi,
        JourSemaine::Vendredi => JourSemaine::Samedi,
        JourSemaine::Samedi => JourSemaine::Dimanche,
        JourSemaine::Dimanche => JourSemaine::Lundi,
    };
    println!("Demain c'est {:?}", demain);
}
