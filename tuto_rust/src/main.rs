
use std::fs;
use std::fs::File;
use std::process;
use std::io::BufReader;
use std::io::BufRead;

//voir https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
#[derive(Debug,Copy,Clone)] 
//#[derive(PartialEq)]
pub enum JourSemaine {
	Lundi,
	Mardi,
	Mercredi,
	Jeudi,
	Vendredi,
	Samedi,
	Dimanche
}

#[derive(Debug,Copy,Clone,PartialEq)]
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
impl std::fmt::Display for Jour {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "le {:?} {} {:?}",self.jour_sem, self.jour_mois, self.mois)
    }
 }
 
 /*pub struct JourIter {
	curr: Jour,
	next: Jour,
 }*/
 
 // Implement `Iterator` for `Jour`.
impl Iterator for Jour {
    type Item = Jour;
	fn next(&mut self) -> Option<Self::Item> {
		let jsem = &self.jour_sem;
		let jmois = &self.jour_mois;
		let mois = &self.mois;
		let jsem_prochain = renvoie_demain_semaine(jsem);
		let jmois_prochain = match *jmois < 30 {
			true => jmois + 1,
			false  => 1,
		};	

		let mois_prochain = match  *jmois < 30 {
			true => *mois,
			false => renvoie_mois_suivant(mois)
		};

		self.jour_mois = jmois_prochain;
		self.jour_sem = jsem_prochain;
		self.mois = mois_prochain;
		let dem = Jour{jour_sem:self.jour_sem,jour_mois: self.jour_mois, mois:self.mois};
		//let dem = Jour{jour_sem:jsem_prochain,jour_mois: jmois_prochain, mois:mois_prochain};
		Some(dem)
	 }
}
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
    	Mois::Octobre => write!(f, "Le  {} du mois d'{:?})", self.jour_mois, self.mois),
    	Mois::Aout => write!(f, "Le  {} du mois d'{:?})", self.jour_mois, self.mois),
    	_ =>  write!(f, "Le  {} du mois de {:?})", self.jour_mois, self.mois),
    	}
    }
}

#[derive(Debug)] 
pub struct Evenement {
	description:String,
	date:Date
}

fn main() {
    println!("Hello, world!");
    let mut agenda_anniv:std::vec::Vec<Evenement> = Vec::new();
    
    let date_hpl = Date{jour_mois: 20, mois: Mois::Aout};
	let hpl = Evenement{description:"Anniversaire de Howard".to_string(),date:date_hpl};
	let ms = Evenement{description:"Anniversaire de Mary".to_string(),date:Date{jour_mois: 30, mois: Mois::Aout}};
	let ulg = Evenement{description:"Anniversaire de Ursula".to_string(),date:Date{jour_mois: 21, mois: Mois::Octobre}};
	let bs = Evenement{description:"Anniversaire de Bram".to_string(),date:Date{jour_mois: 8, mois: Mois::Novembre}};
	let jp = Evenement{description:"Anniversaire de John".to_string(),date:Date{jour_mois: 30, mois: Mois::Aout}};

	agenda_anniv.push(hpl);
	

	
	println!("{:?}",agenda_anniv);
	// ne marche pas
	//println!("{:?}",hpl);
	println!("{:?}",agenda_anniv[0]);
	agenda_anniv.push(ms);
	agenda_anniv.push(ulg);
	agenda_anniv.push(bs);
	agenda_anniv.push(jp);
	println!("{:?}",agenda_anniv);

	/*for evenement in agenda_anniv {
		println!("Le {:?} c'est {:?}",evenement.date, evenement.description);
	}*/
	
	// et maintenant ca ne marche plus
	//println!("{:?}",agenda_anniv);
	
	for evenement in &agenda_anniv {
		println!("Le {:?} c'est {:?}",evenement.date, evenement.description);
	}
	
	// et maintenant ca marche
	println!("{:?}",agenda_anniv);
	
	
	
	for evenement in agenda_anniv.iter() {
    println!("ET AUSSI Le {:?} c'est {:?}",evenement.date, evenement.description);
    }
	println!("{:?}",agenda_anniv);
	
// Utilistaion iteratoeir
	let iterateur = agenda_anniv.iter();
	println!("{:?}",agenda_anniv);
	for evenement in iterateur {
    println!("ET ENCORE Le {:?} c'est {:?}",evenement.date, evenement.description);
    }
    	println!("{:?}",agenda_anniv);
// Utilistaion iteratoeir
//	let iterateur = agenda_anniv.into_iter();
	//println!("{:?}",agenda_anniv);
//	for evenement in iterateur {
//    println!("ET LA  {} c'est {}",evenement.date, evenement.description);
    //println!("{:?}",agenda_anniv);
//    }
				
		
	
	
	let ajd = Jour{jour_sem: JourSemaine::Lundi, jour_mois: 29, mois: Mois::Octobre};
    println!("Ajourd'hui c'est {:?}",ajd);
    let demain = renvoie_demain(&ajd);
    
 /*   if evenement(&ajd,&agenda_anniv[0].date) {
    	println!("*** {:?}",&agenda_anniv[0])
    	};*/
    println!("Demain c'est {:?}",demain);
    let demain = renvoie_demain(&demain);
    println!("Apres demain c'est {:?}",demain);
    let demain = renvoie_demain(&demain);
    println!("Apres demain c'est {:?}",demain);
    let ajd = Jour{jour_sem: JourSemaine::Lundi, jour_mois: 29, mois: Mois::Aout};
    println!("Ajourd'hui c'est {:?}",ajd);
    let prog = programme_map_fold(&ajd,&agenda_anniv);
    match prog {
    	Some(s) => { println!("Attention ajourd'hui:\n{}",s)},
    	None => {println!("Rien ajourd'hui...")},
    }
    let demain = renvoie_demain(&ajd);
    let prog = programme_map_fold(&demain,&agenda_anniv);
    match prog {
    	Some(s) => { println!("Attention :\n{}",s)},
    	None => {},
    }
   let ajd = Jour{jour_sem: JourSemaine::Lundi, jour_mois: 29, mois: Mois::Aout};
   println!("Ajourd'hui c'est {}",ajd);
   let demain = renvoie_demain(&ajd);
   let prog = programme_map_fold(&demain,&agenda_anniv);
    match prog {
    	Some(s) => { println!("Attention demain {} :\n{}",demain,s)},
    	None => {},
    }    
	let mut count = 0u32;


    // Infinite loop
    let mut ajd = Jour{jour_sem: JourSemaine::Lundi, jour_mois: 1, mois: Mois::Aout};

    loop {
        count += 1;
   		println!("Ajourd'hui c'est {}",ajd);
   		let demain = renvoie_demain(&ajd);
    	match programme_map_fold(&demain,&agenda_anniv) {
    		Some(s) => { println!("Attention demain {} :\n{}",demain,s)},
    		None => {},
    	}
    	ajd = demain;
    	        
		if count == 80 {
            println!("OK, that's enough");
            // Exit this loop
            break;
        }
    }
    
    println!("Les 42 premiers jours : ");
    for i in jour().take(42) {
        println!("> {}", i);
    }
    
    //open file
     let contents = fs::read_to_string("date_naissance.txt");
    // .expect("Something went wrong reading the config file");
    let contents = match contents {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Il y a eu un probleme lors de la lecture du fichier.\nL'erreur en question : [--> {} <--]",err);
            process::exit(1);
        },
      };
      println!("{}",contents);
      
      
     //open file
     let contents = fs::read_to_string("date_naissance.txt");
     println!("{:?}",contents);
     let contents = fs::read_to_string("date_naissance.txt").expect("Ca a plante pendant la lecture");
     let conf = contents.split('\n');
    for line in conf {
    	println!("-> {}",line);
    	// Split renvoie un iterateur
    	let test = line.split(',');
    	println!("-> {:?}",test);
    	let test: Vec<&str> = line.split(',').collect();
    	    	println!("-> {:?}",test);
    	    	//plante a la fin
    	    	//assert_eq!(test.len(),5);
    }
    
    // open file 2
    let file = File::open("date_naissance.txt").expect("Ca a plante pendant l'ouverture");
    let reader = BufReader::new(file);
	println!("\n\n\nLecture 2");
    for line in reader.lines() {

        // Non:
        // let test: Vec<&str> = line.split(',').collect();
        // Non
        // let test: Vec<&str> = line.expect("REASON").split(',').collect();
        let line = line.expect("Probleme de lecture");
        let split_line: Vec<&str> = line.split(',').collect();
    	println!("-> {:?}",split_line);
        assert_eq!(split_line.len(),5);
        
        // cf expect, wrap, etc
        //Unwrap in Rust returns the result of the operation for Option and Result enums.
        //If unwrap encounters an error Err or a None, it will panic and stop the program execution.
		//Unwrap method is defined on both Option and Result type.
        
        let prenom = split_line[1];
        //let prenom = split_line[1].expect("Erreur durant le split");
        println!("{}",prenom);
        // Virer les prenome multiples
        let prenom = prenom.split(' ').next();
        // on recuper ce qu'il y a dans Option
        //let prenom = prenom.split(' ').next();
        println!("{:?}",prenom);
        println!("{:?}",prenom.unwrap());
        let prenom  = prenom.unwrap();
    	let jour_mois = split_line[2];   
    	
    	 let jour_mois = match jour_mois.parse::<u32>(){
                            Ok(valeur) => valeur,
                            Err(_err) => {
                                eprintln!("Erreur, le jour du mois doit etre un entier");
                                process::exit(1);
                            },
    	};
    	 println!("{}",jour_mois);
    	let mois = match split_line[3] {
    		"July" => Mois::Juillet,
    		"August" => Mois::Aout, 
     		"October" => Mois::Octobre,  
     		"November" => Mois::Novembre,   
      		"December" => Mois::Decembre,       		    		  		   		
    		_ => {
    			eprintln!("Erreur, je connais pas ce mois.");
                process::exit(1);
                },
    	};
        let date = Date{jour_mois: jour_mois, mois: mois};
        let evt =  Evenement{description:prenom.to_string()+"'s birthday",date:date};
        agenda_anniv.push(evt);
		//let hpl = Evenement{description:"Anniversaire de Howard".to_string(),date:date_hpl};

	}
    // Infinite loop
    let mut ajd = Jour{jour_sem: JourSemaine::Lundi, jour_mois: 1, mois: Mois::Juillet};

	let mut count = 0;
    loop {
        count += 1;
   		println!("Ajourd'hui c'est {}",ajd);
   		let demain = renvoie_demain(&ajd);
    	match programme_map_fold(&demain,&agenda_anniv) {
    		Some(s) => { println!("\nAttention demain {} :\n{}",demain,s)},
    		None => {},
    	}
    	ajd = demain;
    	        
		if count == 110 {
            println!("OK, that's enough");
            // Exit this loop
            break;
        }
    }
   


        

    
}
// Returns a Fibonacci sequence generator
//fn jour() -> JourIter {
//    JourIter {curr: Jour{jour_sem: JourSemaine::Lundi, jour_mois: 1, mois: Mois::Octobre}, next : Jour{jour_sem: JourSemaine::Mardi, jour_mois: 2, mois: Mois::Octobre}}
//}
fn jour() -> Jour {
    Jour{jour_sem: JourSemaine::Lundi, jour_mois: 1, mois: Mois::Octobre}
}


pub fn demain(jour: &JourSemaine) { 
	println!("Aujourdhui c'est bien {:?}",jour);
	//if *jour == JourSemaine::Lundi {println!("On est demain Mardi")}
	match *jour {
		JourSemaine::Lundi => {println!("On est demain Mardi")},
		_ => {},
	}
}

pub fn programme(jour: &Jour, agenda: &Vec<Evenement>) -> Option<String> {
	let iterateur = agenda.iter();
	let mut events = String::new();
	let mut nb_events =0; 
	for eve in iterateur {
	if evenement(jour, &eve.date) {
		println!("Trouve {}",&eve.description);
		events.push_str(&eve.description);
		events.push_str("\n");
		nb_events += 1;
		}
	}
	println!("Trouve {} evenement(s)",nb_events);
	match nb_events {
	0 => None,
	_ => Some(events)
	}
}


pub fn programme_map(jour: &Jour, agenda: &Vec<Evenement>) -> Option<String> {
	let iterateur = agenda.iter();
	let mut events = String::new();
	let mut nb_events =0; 	
	for eve in iterateur.filter(|e| {evenement(jour, &e.date)}).map(|e|{&e.description}){
		events.push_str(eve);
		events.push_str("\n");
		nb_events += 1;
		};
	println!("Trouve {} evenement(s)",nb_events);
	match nb_events {
	0 => None,
	_ => Some(events)
	}
}
pub fn programme_map_fold(jour: &Jour, agenda: &Vec<Evenement>) -> Option<String> {
	let iterateur = agenda.iter();
	let res = iterateur.filter(|e| {evenement(jour, &e.date)}).map(|e|{&e.description}).fold(String::new(), |a, b| a + b + "\n");
	match res.len() {
	0 => None,
	_ => Some(res)
	}
}



pub fn evenement(jour: &Jour, date: &Date) -> bool {
match jour.jour_mois == date.jour_mois {
	true => {
		 jour.mois == date.mois 
		},
	false => false
}
}


pub fn renvoie_demain(jour: &Jour)->Jour {

let jsem = &jour.jour_sem;
//println!("Jour de la semaine = {:?}",jsem);
let jmois = &jour.jour_mois;
let mois = &jour.mois;

let jsem_prochain = renvoie_demain_semaine(jsem);

let jmois_prochain = match *jmois < 30 {
	true => jmois + 1,
	false  => 1,
	};	

let mois_prochain = match  *jmois < 30 {
	true => *mois,
	false => renvoie_mois_suivant(mois)
	};

let dem = Jour{jour_sem:jsem_prochain,jour_mois: jmois_prochain, mois:mois_prochain};

dem
} 


pub fn renvoie_demain_semaine(jour: &JourSemaine)->JourSemaine { 
	let demain = match *jour {
	JourSemaine::Lundi => JourSemaine::Mardi,
	JourSemaine::Mardi => JourSemaine::Mercredi,
	JourSemaine::Mercredi => JourSemaine::Jeudi,
	JourSemaine::Jeudi => JourSemaine::Vendredi,
	JourSemaine::Vendredi => JourSemaine::Samedi,
	JourSemaine::Samedi => JourSemaine::Dimanche,
	JourSemaine::Dimanche => JourSemaine::Lundi,			
	};
	demain
}
pub fn renvoie_mois_suivant(mois: &Mois)->Mois { 
	let mois_prochain = match *mois {
	Mois::Octobre => Mois::Novembre,
	Mois::Novembre => Mois::Decembre,
	Mois::Decembre => Mois::Juillet,
	Mois::Juillet => Mois::Aout,	
	Mois::Aout => Mois::Octobre,				
	};
	mois_prochain
}
