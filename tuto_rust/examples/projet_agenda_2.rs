// Utiliser des structures
// =======================

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

fn main() {
    let ajd = Jour { jour_sem: JourSemaine::Vendredi, jour_mois: 13, mois: Mois::Octobre};
    println!("Aujourd'hui c'est {:?}",ajd);
	println!("Aujourd'hui c'est {:?}, le {} du mois d'{:?}",ajd.jour_sem,ajd.jour_mois,ajd.mois);
}
