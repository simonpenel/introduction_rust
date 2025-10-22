//  LES ENUMERATIONS(ENUM)
//  ======================

// On définit l'enum "JourSemaine"
#[derive(Debug)] // <- JourSemaine doit implementer le "trait" Debug pour pouvoir etre affiche avec {:?}. La notion de Trait sera abordé plus tard
pub enum JourSemaine { // <- le mot-clef  pub ("publique") permet de rendre cette  énumeration visible par le reste du code.
	Lundi,
	Mardi,
	Mercredi,
	Jeudi,
	Vendredi,
	Samedi,
	Dimanche
}

fn main() {
    let ajd = JourSemaine::Lundi;
    println!("Aujourd'hui c'est {:?}",ajd);

// Dans le contexte d'une enumeration, le matching verifie que tous les cas sont testés.
// C'est une sécurité.
// Ceci va generer un erreur:
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
    println!("Demain c'est {:?}",demain);    
}
