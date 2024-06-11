pub mod agenda;

use std::env;
use crate::agenda::premier_jour;
use crate::agenda::read_agenda;
use crate::agenda::programme_du_jour_map_fold;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(),2,"Entrer un argument!");
    let file_path = &args[1];

    let mon_agenda = read_agenda(String::from(file_path));
    println!("Les 160 premiers jours : ");
    for i in premier_jour().take(160) {
        println!("> {}", i);
        match programme_du_jour_map_fold(&i,&mon_agenda) {    
            Some(s) => { println!("\nAttention aujourd'hui {} :\n{}", i,s)},
            None => {},
        }

    }
}


