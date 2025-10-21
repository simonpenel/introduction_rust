//  LES ENUMERATIONS(ENUM)
//  ======================

// On définit l'enum "JourSemaine"
#[derive(Debug)] // <- JourSemaine doit implementer le "trait" Debug pour pouvoir etre affiche avec {:?}. La notion de Trait sera abordé plus tard
pub enum JourSemaine {
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

// Le matching verifie que tous les cas sont testes.
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
