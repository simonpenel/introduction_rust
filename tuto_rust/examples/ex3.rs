#[allow(dead_code)]
#[allow(unused)]
// ===============
//  FUNCTIONS
// ===============
//  See https://doc.rust-lang.org/book/ch03-03-how-functions-work.htmlhttps://doc.rust-lang.org/book/ch03-03-how-functions-work.html

fn main() {
    // A very simple function that takes an u32 integer as a parameter
    // (for clarity, we declare the functions here, but it would be better
    // to declare them outside  the main function)
    // A function is declared with ‘fn’ and  the type of the argument is defined:
    fn ma_fonction(x: u32) {
        println!("entrée = {}", x)
    }
    ma_fonction(66); // Ok
    // the following statement causes an error:
    // ma_fonction(3.5);  

    let a: u32 = 12;
    println!("value of 'a' before : {}", a);
    ma_fonction(a);
    println!("value of 'a' after : {}", a);

    // The same very simple function with a string as a parameter
    fn ma_fonction_string(mut x: String) {
        println!("x = {}", x)
    }
    let hello = String::from("world");
    println!("Hello  {}!", hello);
    ma_fonction_string(hello);
    // the following statement causes an error:
    // let coucou = hello; 
    // the variable has been borrowed by the function and is threfeore not accessible anymore   


    // In the previous examples, the function becomes the owner of the
    // variable given as input; this is ownership as in the statement 
    // “coucou = hello”. This does not pose a problem with integers.
    // This is why, in the case of integers, it is still  possible to access the variable after calling
    // the fuction  because the variable given in argument is actually copied. In the case of strings,
    // it is no longer possible to access the variable after it has become the property of the function.
    // The solution: the call by reference
    //               ---------------------

    
    // We want to modify the variable  given as argument:
    fn ma_fonction_2(x: u32) {
        println!("x = {}", x);
        // the following statement causes an error:
        // x = x + 5; 
    }
    // the variable must be mutable:
    fn ma_fonction_3(mut x: u32) {
        println!("x = {}", x);
        x = x + 5;
        println!("modified  x = {}", x);
    }
    println!("value of 'a' before : {}", a);
    ma_fonction_3(a);
    println!("value of 'a' after : {}", a);
    // The variable  has not changed after the function  
    // The solution: the call by reference 
    //               ---------------------

    // CALL BY REFERENCE
    // =================
    // The function takes the reference to the variable rather than the variable itself.
    // more info on references :
    // https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

    fn ma_fonction_ref(x: &mut u32) {
        println!("x = {}", x);
        *x = *x + 5;
        println!("modified x  = {}", x);
    }
    let mut a: u32 = 12;
    println!("value of 'a' before : {}", a);
    ma_fonction_ref(&mut a);
    println!("value of 'a' after : {}", a);
    // the variable  has changed after the function  

    // alternative
    let a: &mut u32 = &mut 12;
    println!("a =  {}", a);
    *a = 24;
    println!("avant {}", a);
    ma_fonction_ref(a);
    println!("après {}", a);

    //  A  simple function  with a string as argument
    fn ma_fonction_string_ref(x: &String) {
        println!("x = {}", x);
    }
    let hello = String::from("Hello");
    ma_fonction_string_ref(&hello);
    // the following statement is correct:
    let coucou = hello; 

    // Modification de la chaîne donnée en entree
    fn ma_fonction_string_ref_mut(x: &mut String) {
        x.push_str(", world!");
    }
    let mut hello = String::from("Hello");
    println!("value of 'hello' before : {}", hello);
    ma_fonction_string_ref_mut(&mut hello);
    println!("value of 'hello' after : {}", hello);

    // About  println!
    // ---------------
    // println! is a macro. It works somewhat like a fonction.
    // However: 
    let hello = String::from("Hello");
    println!(" {}", hello);
    let coucou = hello; // This instruction works although  we provided
                        // println!  with a String type variable without going through a reference.
                        // We expect the variable ‘hello’ to be  owned by println! and
                        // therefore no longer accessible (see line 33). In fact, println!
                        // automatically makes a call by reference, for practical reasons. But
                        // this has consequences:
    let mut variable = 3;
    println!("variable = {}", variable);
    let ref_variable = &mut variable;
    *ref_variable = 6;
    println!("variable = {}", variable);
    // the following statement causes an error:
    // *ref_variable = 12; 
    // That's because println! borrows ‘variable’ by creating a mutable reference to ‘variable’.
    // This action removed the previous mutable reference ‘ref_variable’.

    // No problem with a immutable variable
    let variable = 3;
    println!("variable = {}", variable);
    let ref_variable = &variable;
    println!("variable = {}", variable);
    // the following statement is correct:
    let test = *ref_variable + 6;
    println!("ref_variable = {}", ref_variable); 
    println!("test = {}", test); 
}
