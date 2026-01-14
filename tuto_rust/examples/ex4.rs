// =========================================================
// FUNCTIONS (CONTINUED), PATTERN MATCHING, TYPE OPTION
// =========================================================
fn main() {
    // =========================================================================
    // Function that takes a string as a parameter and returns a variable of
    // type integer u32
    // =========================================================================
    fn calcule_nb_o(x: &String) -> u32 {
        let mut nb = 0;
        for c in x.chars() {
            println!("charactere {}", c);
            if c == 'o' {
                nb = nb + 1;
            }
        }
        nb // Please note, there is no final semicolon here!
    }
    let hello = String::from("Hello, world!");
    let nb_de_o = calcule_nb_o(&hello);
    println!("Number of o in {} = {}", hello, nb_de_o);

    // The function can return different types of variables.
    // Here, the ‘tuple’ type: a list of variables that can be of different types.
    // Use of pattern matching
    // =======================
    fn calcule_nb_voy(x: &String) -> (u32, u32, u32, u32, u32) {
        let mut nb_a = 0;
        let mut nb_e = 0;
        let mut nb_i = 0;
        let mut nb_o = 0;
        let mut nb_u = 0;
        for c in x.chars() {
            match c {
                'a' => nb_a += 1, // cas ou c est égal à 'a'
                'e' => nb_e += 1, // ...
                'i' => nb_i += 1,
                'o' => nb_o += 1,
                'u' => nb_u += 1,
                _ => {} // <- tous les autre cas
            }
        }
        (nb_a, nb_e, nb_i, nb_o, nb_u)
    }

    let nb_de_voyelles = calcule_nb_voy(&hello);
    //(use {:?} to display complex variables)
    println!("Nb of a, e, i, o, u in {} : {:?}", hello, nb_de_voyelles);

    // On peut aussi écrire comme ça
    let (a, e, i, o, u) = calcule_nb_voy(&hello);
    println!(
        "Nb of a : {} , of e : {}, of i : {}, of o : {}, of u : {}",
        a, e, i, o, u
    );

    // Using the Option type
    // ==========================
    // Let's imagine we want to retrieve a string of characters containing numbers from some text.
    //
    // Without the Option type (or equivalent):
    // We define the function f1: f1(‘AZER0989TY’) -> ‘0989’
    // ok!
    // but f1(‘AZERTYUIOP’) -> ?
    // we can ask the function to return an error, or we can do te following
    // f1(‘AZERTYUIOP’) -> ‘’
    // This is annoying: an empty string is not the absence of a string.
    // And if we wants to sum the digits in this string:
    // f2(‘0989’) -> 26
    // f2(‘’) -> ?
    // We can do the following :  f2(‘’) -> 0
    // But f2(‘00000’) -> 0 too
    // We have f2(f1(‘AZER0000TYUI000OP’)) == f2(f1(‘AZERTYUIOP’))
    // We can instead do the following: f2(‘’) -> -1, as -1 will never be found in the
    // string, which serves as a signal.
    // We must therefore systematically check whether the value is -1 before any
    // processing. If we are distracted and calculate the square of f2, we get
    // (f2(f1(‘AZERTYUIOP’)))²  = (-1)² = 1
    // And also  (f2(f1(‘AZERT1YUIOP’)))²  = (1)² = 1
    // We can see that checking the entered value is a burden
    // and that the risk of undetected errors is real.
    //
    // The Option type solves this problem. The Option type allows
    // the use of an optional value.
    // Option can be:
    //      - either a value of a given type
    //      - either nothing
    //
    // In this example, we are only looking for the numbers 0-4. We return the
    // string composed of these numbers.
    fn extrait_chaine(x: &String) -> Option<String> {
        let mut chaine_0123 = String::new(); // Create an empty string
        for c in x.chars() { // loop over characters in  x
            match c {
                '0' => chaine_0123.push_str("0"),
                '1' => chaine_0123.push_str("1"),
                '2' => chaine_0123.push_str("2"),
                '3' => chaine_0123.push_str("3"),
                _ => {}
            }
        }
        match chaine_0123.len() > 0 {
            true => Some(chaine_0123), // return a string
            false => None,             // return  None
        }
    }
    let hello = String::from("Hello, world!");
    println!("Processing string {}", hello);
    println!("The function returns {:?}", extrait_chaine(&hello));
    let hello = String::from("He12llo000, wo3455rl12d!");
    println!("Processing string {}", hello);
    println!("The function returns {:?}", extrait_chaine(&hello));

    // Pattern matching on the returned variable:
    let hello = String::from("Hello, world!");
    println!("Processing string {}", hello);
    match extrait_chaine(&hello) {
        Some(chaine_de_chiffres) => println!("Found string : {}.", chaine_de_chiffres),
        None => println!("Nothing found."),
    }
    let hello = String::from("Hel12076lo, w21orld!");
    println!("Processing string {}", hello);
    match extrait_chaine(&hello) {
        Some(chaine_de_chiffres) => println!("Found string : {}.", chaine_de_chiffres),
        None => println!("Nothing found."),
    }

    // Store the option variable for further processing
    let _resu = extrait_chaine(&hello);

    //  A more general function, which returns the sum
    fn extrait_chaine_somme(x: &String) -> Option<u32> {
        let mut somme = 0;
        let mut flag = false;
        for c in x.chars() {
            let valeur_de_c = c.to_digit(10);
            // "to_digit" is a  méthod  of the  'char' type which returns  an Option: the numerical value  of the char, or nothing
            match valeur_de_c {
                Some(valeur) => {
                    somme = somme + valeur;
                    flag = true;
                }
                None => {}
            }
        }
        match flag {
            true => Some(somme),
            false => None,
        }
    }

    let hello = String::from("Hello, world!");
    println!("Sum of {} = {:?}", hello, extrait_chaine_somme(&hello));
    let hello = String::from("H3ll0, w0r1d!");
    println!("Sum of {} = {:?}", hello, extrait_chaine_somme(&hello));

    // Info on Option type
    // pub enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // https://doc.rust-lang.org/std/option/enum.Option.html

    // A similar type : Results
    // https://doc.rust-lang.org/std/result/
}
