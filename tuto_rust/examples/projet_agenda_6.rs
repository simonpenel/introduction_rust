// CREER UN VECTEUR DE STRUCTURES 
// ==============================


/// Enum JourSemaine
#[derive(Debug)]
pub enum JourSemaine {
	Lundi,
	Mardi,
	Mercredi,
	Jeudi,
	Vendredi,
	Samedi,
	Dimanche
}

/// Enum Mois
#[derive(Debug)]
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
	mois: Mois	
	}
impl std::fmt::Display for Jour {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "le {:?} {} {:?}",self.jour_sem, self.jour_mois, self.mois)
    }
 }
 
/// Structure Date 
#[derive(Debug)] 
pub struct Date {
	/// jour du mois
	jour_mois: u32,
	/// mois
	mois: Mois	
	}
impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    	match self.mois {
    	Mois::Octobre => write!(f, "le {} du mois d'{:?}", self.jour_mois, self.mois),
    	Mois::Aout => write!(f, "le {} du mois d'{:?}", self.jour_mois, self.mois),
    	_ =>  write!(f, "le {} du mois de {:?}", self.jour_mois, self.mois),
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
    	println!("------ {:?}/{} --------",self.mois, self.jour_mois) 	
   }   
}   
 
/// Structure Evenement
#[derive(Debug)] 
pub struct Evenement {
	description:String,
	date:Date
}
   

fn main() {

	// Creation d'1 evenement a partir d'une date
    let date_hpl = Date{jour_mois: 20, mois: Mois::Aout};
	let hpl = Evenement{description:"Anniversaire de Howard".to_string(),date:date_hpl};

	// Creation directe
	let ms = Evenement{description:"Anniversaire de Mary".to_string(),date:Date{jour_mois: 30, mois: Mois::Aout}};

	// Creation en utilisant la fonction new de Date
	let ulg = Evenement{description:"Anniversaire de Ursula".to_string(),date:Date::new(21,Mois::Octobre)};
	let bs = Evenement{description:"Anniversaire de Bram".to_string(),date:Date::new(8,Mois::Novembre)};
	let jp = Evenement{description:"Anniversaire de John".to_string(),date:Date::new(30,Mois::Aout)};
 
 	// Creation d'un vecteur d'evenements vide
 	let mut agenda_anniv:std::vec::Vec<Evenement> = Vec::new();
	println!("Avant : {:?}",agenda_anniv); 	
 	agenda_anniv.push(hpl);
	agenda_anniv.push(ms);
	agenda_anniv.push(ulg);
	agenda_anniv.push(bs);
	agenda_anniv.push(jp);

	println!("Apres : {:?}",agenda_anniv);
	
	// Laligne suivante genere une erreur: la variable hpl a ete donnee au vecteur agenda_anniv, elle n'est plus accessible.
	// println!("{:?}",hpl);
    // On la voit ici
	println!("{:?}",agenda_anniv[0]);
	// println!("{}",agenda_anniv[0]); // Ceci genere une erreur car la structure Evenement n'a pas le trait Display,
	//                                    seulement le trait Debug
	println!("{} c'est {}",agenda_anniv[0].date, agenda_anniv[0].description);
}

