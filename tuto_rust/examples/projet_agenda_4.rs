// IMPLEMENTER UN TRAIT POUR UNE STRUCTURE SUITE
// =============================================

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

// Le trait "Display" (nécessaire pour utiliser println!) n'existe pas pour
// notre structure. Nous allons l'implémenter :
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
// Le trait "Display" (nécessaire pour utiliser println!) n'existe pas pour
// notre structure. Nous allons l'implémenter :	
impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    	match self.mois {
    		Mois::Octobre => write!(f, "le {} du mois d'{:?}", self.jour_mois, self.mois),
    		Mois::Aout => write!(f, "le {} du mois d'{:?}", self.jour_mois, self.mois),
    		_ =>  write!(f, "le {} du mois de {:?}", self.jour_mois, self.mois),
    	}
    }
}

fn main() {
    let ajd = Jour { jour_sem: JourSemaine::Vendredi, jour_mois: 13, mois: Mois::Octobre};
    println!("Aujourd'hui c'est {:?}",ajd);
	println!("Aujourd'hui c'est {:?}, le {} du mois d'{:?}",ajd.jour_sem,ajd.jour_mois,ajd.mois);
	println!("Aujourd'hui c'est {}",ajd);

	let anniv = Date {jour_mois: 13, mois:Mois::Juin};
	println!("L'anniversaire de Jason c'est {}",anniv);
	let anniv = Date {jour_mois: 19, mois:Mois::Octobre};
	println!("L'anniversaire de Michael c'est {}",anniv);
	let anniv = Date {jour_mois: 2, mois:Mois::Aout};
	println!("L'anniversaire de Wes c'est {}",anniv);
	let anniv = Date {jour_mois: 17, mois:Mois::Juillet};
	println!("L'anniversaire de Nancy c'est {}",anniv);
}
