// Les Iterators
// =============


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
#[derive(Debug,PartialEq)]
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
	
// Le trait "Display" (necessaire pour utiliser println!) n'existe pas pour notre structure.
// Nous allons le definir	
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
	
	// Ceci genere une erreur: la variable hpl a ete donnee au vecteur agenda_anniv, elle n'est plus accessible.
	// println!("{:?}",hpl);
    // On la voit ici
	println!("{:?}",agenda_anniv[0]);
	println!("{} c'est {}",agenda_anniv[0].date, agenda_anniv[0].description);
	
	
	// Tester ceci
	/*
	for evenement in agenda_anniv {
		println!("{} c'est {}",evenement.date, evenement.description);
	}
	
	// et maintenant cette commande ne marche plus
	println!("{:?}",agenda_anniv);
	*/
	// Que s'est  t il passé ?
	// Le compilateur l'explique

	// Le compilateur propose:
	for evenement in &agenda_anniv {
		println!("{} c'est {}",evenement.date, evenement.description);
	}
	
	// et maintenant ca marche
	println!("{:?}",agenda_anniv);	
	
	// Utilisation d'un iterateur
	// La boucle for utilise implicitement un iterateur into_iter
	// On peut le faire explicitement
	let iterateur = agenda_anniv.iter();
	// Et pas into_iter
	
	for evenement in iterateur {
	    println!("Et encore, {} c'est toujours {}",evenement.date, evenement.description);
    }	

    // ca marche toujours
	println!("{:?}",agenda_anniv);	

	// les methodes associees aux  iterateurs sont nombreuses
	// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map

	// quelques exemples	
	
	let iterateur = agenda_anniv.iter().enumerate();
	for (i, eve) in iterateur {
		println!("Evnement numero {} : {:?}",i,eve);
	}
	
	// necessite l'ajout de 
	// #[derive(PartialEq)]
	// a la struture Mois
	let iterateur = agenda_anniv.iter().filter(|x| x.date.mois == Mois::Aout).enumerate();
	for (i, eve) in iterateur {
		println!("Evnement en Aout numero {} : {:?}",i,eve);
	}
		

//    }

	
}

