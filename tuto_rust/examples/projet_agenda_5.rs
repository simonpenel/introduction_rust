// IMPLEMENTER UNE FONCTION POUR UNE STRUCTURE
// ============================================

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

#[derive(Debug)]
pub enum Mois {
	Octobre,
	Novembre,
	Decembre,
	Juin,
	Juillet,
	Aout,
}

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

// Implémente des fonctions et des méthodes pour la structure Date
// ----------------------------------------------------------------
impl Date {
	// Fonction qui crée une nouvelle date à partir du jour et du mois. La
	// fonction renvoie donc une Date
    pub fn new(j: u32, m: Mois) -> Self { // <- Self designe la structure pour laquelle on implémente la fonction. 
        Self {
            jour_mois: j,
            mois: m,
        }
    }
	// Fonction sans paramètre qui crée une nouvelle date que l'on veut
	// considérer comme le premier jour de l'année, par exemple le 1er Novembre 
    pub fn premier_jour() -> Self {
        Self {
            jour_mois: 1,
            mois: Mois::Novembre,
        }
    }
	// Méthode  associée à la variable. Dans la défintion d'une méthode, le 
	// premier argument est toujours &self 
    pub fn affiche(&self) {
    	println!("------ {:?}/{} --------",self.mois, self.jour_mois) 	
   }
}
fn main() {
	let p = Date::premier_jour();  // fonction
	println!("Le premier jour de l'annee c'est {}",p);
	let anniv = Date::new(2,Mois::Aout); // fonction
	println!("L'anniversaire de Wes c'est {}",anniv);
	anniv.affiche(); // méthode
	let anniv = Date::new(17,Mois::Juillet); // fonction
	println!("L'anniversaire de Nancy c'est {}",anniv);
	anniv.affiche(); // méthode
}

