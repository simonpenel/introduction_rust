// =========================================================
//  LES FONCTIONS (SUITE) ,LE PATTERN MATCHING, LE TYPE OPTION
// =========================================================
fn main() {
    // =========================================================================
    // Fonction qui prend une chaîne comme paramètre et renvoie une variable de
    // type entier u32 
    // =========================================================================
    fn calcule_nb_o( x: &String)-> u32 {
        let mut nb = 0;
        for c in x.chars() {
            println!("charactere {}",c);
            if c == 'o' {
                nb = nb + 1 ;
            }
        };
        nb // Attention, ici pas de ";" final ! 
    }
    let hello = String::from("Hello, world!");
    let nb_de_o = calcule_nb_o(&hello);
    println!("Nombre de o dans {} = {}", hello, nb_de_o);

    // La fonction peut renvoyer differents type de variables.
    // ici le type "tupple" : une liste de variables qui peuvent être de type
    // differents.
    // Utilisation du matching
    // =======================
    fn calcule_nb_voy( x: &String)-> (u32,u32,u32,u32,u32) {
        let mut nb_a = 0;
        let mut nb_e = 0;
        let mut nb_i = 0;
        let mut nb_o = 0;
        let mut nb_u = 0;
        for c in x.chars() {
            match c {
                'a' => nb_a +=1, // cas ou c est égal à 'a'
                'e' => nb_e +=1, // ...
                'i' => nb_i +=1,
                'o' => nb_o +=1,
                'u' => nb_u +=1,
                _ => {} // <- tous les autre cas
            }
        };
        (nb_a,nb_e,nb_i,nb_o,nb_u)
    }

    let nb_de_voyelles = calcule_nb_voy(&hello);
    //(on utilise {:?} pour afficher des variables complexes)
    println!("Nb de a, e, i, o, u dans {} : {:?}",hello, nb_de_voyelles); 

    // On peut aussi écrire comme ça
    let (a, e, i, o, u)  = calcule_nb_voy(&hello);
    println!("Nb de a : {} , de e : {}, de i : {}, de o : {}, de u : {}",a,e,i,o,u);

    // Utilisation du type Option
    // ==========================
    // Imaginons que l'on veuille récupérer dans du texte une chaîne de
    // caractères contenant des chiffres.
    // 
    // Sans le type Option (ou équivalent):
    // On définit la fonction f1 : f1("AZER0989TY") -> "0989"
    // ok!
    // mais f1("AZERTYUIOP") -> ?
    // on peut demander à la fonction de renvoyer une erreur , ou alors on dit
    // f1("AZERTYUIOP") -> ""
    // C'est génant: une chaine vide n'est pas l'absence de chaine.
    // Et si ob veut sommer les chiffres de cette chaine:
    // f2("0989") -> 26
    // f2("") -> ?
    // On peut dire f2("") -> 0
    // Mais f2("00000") -> 0 aussi
    // On a f2(f1("AZER0000TYUI000OP")) == f2(f1("AZERTYUIOP"))
    // On peut plutot dire f2("") -> -1 comme -1 ne sera jamais trouvé dans la
    // chaine, cela sert de signal.
    // Il faut alors vérifier systematiquement si la valeur est -1 avant tout
    // traitement. Si on est distrait et qu'on calcule le carré de f2 on a 
    // (f2(f1("AZERTYUIOP")))²  = (-1)² = 1
    // Et aussi  (f2(f1("AZERT1YUIOP")))²  = (1)² = 1
    // On voit qu'on traîne comme un boulet la vérification de la valeur entrée
    // et que le risque d'erreur indetectée est réel.
    // 
    // Le type Option permet de resoudre ce probleme. Le type Option permet
    // d'utiliser une valeur optionelle.
    // Option peut être  :
    //      - soit une valeur d'un type donnée 
    //      - soit rien
    // 

    // Dans cet exemple on ne cherche que les chiffres 0-4. On renvoie la
    // chaîne de cararactères composée de ces chiffres.
    // Utilisation de Option
    // =====================
    fn extrait_chaine( x: &String)-> Option<String> {
        let mut chaine_0123 = String::new(); // Crée une chaîne vide
        for c in x.chars() { // boucle sur les caractères de la chaîne entrée
            match c {
                '0' => chaine_0123.push_str("0"),
                '1' => chaine_0123.push_str("1"),
                '2' => chaine_0123.push_str("2"),
                '3' => chaine_0123.push_str("3"),
                _   => {}
                }
        };
        match   chaine_0123.len() > 0 {
            true => Some(chaine_0123),  // On renvoie une chaîne
            false => None,              // On renvoie None
        }
    }
    let hello = String::from("Hello, world!");
    println!("traite la chaîne {}",hello);
    println!("La fonction renvoie {:?}",extrait_chaine(&hello));
    let hello = String::from("He12llo000, wo3455rl12d!");
    println!("traite la chaîne {}",hello);
    println!("La fonction renvoie {:?}",extrait_chaine(&hello));

    // On peut faire un matching sur la variable Option que l'on recupère:
    let hello = String::from("Hello, world!");
    println!("traite la chaine {}",hello);
    match extrait_chaine(&hello) {
        Some(chaine_de_chiffres) => println!("On a trouvé cette chaine: {}.",chaine_de_chiffres),
        None  => println!("On n'a rien trouvé."),
    }
    let hello = String::from("Hel12076lo, w21orld!");
    println!("traite la chaine {}",hello);
    match extrait_chaine(&hello) {
        Some(chaine_de_chiffres) => println!("On a trouvé cette chaine: {}.",chaine_de_chiffres),
        None  => println!("On n'a rien trouvé."),
    }

    // On peut aussi recuperer la variable option pour la traiter ulterieuremnt
    let _resu = extrait_chaine(&hello);

    //  Une fonction plus generale, qui donne la somme
    fn extrait_chaine_somme( x: &String)-> Option<u32> {
        let mut somme = 0;
        let mut flag = false;
        for c in x.chars() {
            let valeur_de_c = c.to_digit(10);
            // to_digit est une méthode du type char qui renvoie le type Option: la valeur numerique du char, ou rien
            match valeur_de_c {
                Some(valeur) => { 
                    somme = somme + valeur;
                    flag = true; 
                    },
                None => {}
            }
        };
        match flag {
            true => Some(somme),
            false => None,
        }
    }

    let hello = String::from("Hello, world!");
    println!("Somme de {} = {:?}",hello, extrait_chaine_somme(&hello));
    let hello = String::from("H3ll0, w0r1d!");
    println!("Somme de {} = {:?}",hello, extrait_chaine_somme(&hello));

    // Info sur le type Option
    // pub enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // https://doc.rust-lang.org/std/option/enum.Option.html

    // Il existe un type similaire : Results
    // https://doc.rust-lang.org/std/result/
}
