// Implementer un trait pour une structure
// =======================================

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
	
// Le trait "Display" (necessaire pour utiliser println!) n'existe pas pour notre structure.
// Nous allons le definir	
impl std::fmt::Display for Jour {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "le {:?} {} {:?}",self.jour_sem, self.jour_mois, self.mois)
    }
 }
 
fn main() {
    let ajd = Jour { jour_sem: JourSemaine::Vendredi, jour_mois: 13, mois: Mois::Octobre};
    println!("Aujourd'hui c'est {:?}",ajd);
	println!("Aujourd'hui c'est {:?}, le {} du mois d'{:?}",ajd.jour_sem,ajd.jour_mois,ajd.mois);
	println!("Aujourd'hui c'est {}",ajd);
}
