#[allow(unused)]

// ========================================
//  OWNERSHIP AND BORROWING, CLONING
// ========================================

fn main() {
    println!("Hello, world!");

    // ===============
    // Ownership
    // ===============
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
    // https://doc.rust-lang.org/rust-by-example/scope/move.html
    //
    // Ownership rule:
    // ----------------------
    // In Rust, each variable owns a memory location.
    // There can only be one owner at a time for this location.
    // When a location is assigned to another variable, it is transferred
    // from the source variable to the target variable. The source variable no longer
    // has access to the memory location, and the target variable becomes the new
    // owner.
    // Purpose:  to keep memory clean:
    // - avoid unpredictable behaviour with read/write operations
    // - quickly clean up unused data.


    println!("Ex 2.1 ");
    let hello = String::from("Hello, world!");
    let coucou = hello;
    // the variable 'hello' is no longer accessible
    // the following statement causes an error:
    // let salut  = hello;

    // Case of ‘simple’ variables
    // --------------------------
    // Ownership is not an issue with integer variables.
    // In these cases, the variable is actually copied automatically, as
    // this is inexpensive, unlike  copying a  complex variable
    // (dynamic allocation, variable size, structures, etc.).

    println!("Ex 2.2 ");
    let hello = 1;
    let coucou = hello;
    // the variable 'hello' is still  accessible
    // the following statement is correct:
    let salut = hello;

    // ==================================
    // References and borrowing
    // ==================================
    // In order to keep access to a memory location, Rust  langage allows to use  'references'.
    // A reference is a data type that represents a memory location.
    // The & operator allows to obtain the memory location assigned to a variable.
    // &x is the memory location of x.
    // The action of creating a reference is called ‘borrowing’.


    println!("Ex 2.3 ");
    let coucou = String::from("Hello, world!");
    let bonjour = &coucou;
    let salut = &coucou;
    // To retrieve the value at the memory location described by the reference, use the * operator.
    println!("{}", *bonjour);
    // To display the reference:
    println!("{:p}", bonjour);
    // Note that we use {:p} to force println! to write the address and not the value found there,
    // which is the default behaviour of println!.
    // the variable 'coucou' is still  accessible
    println!("{}", coucou);
    println!("{}", *salut);

    // Using the * operator
    let variable_1 = 12;
    let ref_variable_1 = &variable_1;
    let variable_2 = 2 * ref_variable_1;
    println!("variable_1 = {}",variable_1);
    println!("variable_2 = {}",variable_2);


    // Reminder concerning ownership on integers (mutable or not)
    // ==========================================================
    println!("Ex 2.4 ");
    let variable_1 = 3;
    let variable_2 = variable_1;
    // the following statement is correct:
    let variable_3 = variable_1;

    // Reminder concerning ownership on Strings
    // ========================================
    println!("Ex 2.5 ");
    let variable_1 = String::from("Hello, world!");
    let variable_2 = variable_1;
    // the following statement causes an error:
    //let variable_3 = variable_1; 

    // Immutable references
    // =====================
    println!("Ex 2.6 ");
    let variable = 3;
    let ref1_variable = &variable;
    println!("variable = {}", variable);
    println!("ref1_variable = {}", ref1_variable);

    // It is allowed to use many immutable references to the same variable 
    // --------------------------------------------------------------------
    let ref2_variable = &variable;
    let ref3_variable = &variable;
    println!("variable = {}", variable);
    println!("ref2_variable = {}", ref2_variable);
    println!("ref3_variable = {}", ref3_variable);
    // variable 'variable' is still  accessible
    println!("variable = {}", variable);

    // Mutable references
    // ==================
    // WARNING:
    // --------
    // When declaring a mutable reference :
    // ref_variable_toto = &mut variable_toto
    // &mut represents exclusive access and prohibits any use of the variable
    // that does not go through the reference.
    // Here, ref_variable_toto has exclusive access to variable_toto until it is last used.

    // A simple rule:
    //  * a non-mutable variable can be borrowed by as many non-mutable references
    //    as desired
    //  * a mutable variable can only be borrowed by a single mutable reference
    //  
    //  Why?  draw a parallel with read-only or read/write files:
    //  A read-only file can be accessed at the same time by
    //  different parts of the code.
    //  A read/write file should only be accessed at the same time
    //  by a single part of the code to avoid potential errors.


    // A single mutable reference : ok
    // ---------------------------------
    println!("Ex 2.8 ");
    let mut variable = 3;
    let ref_variable = &mut variable;
    *ref_variable = 4;
    println!("variable = {}", variable);

    // Multiple  mutable references : error
    // ------------------------------------
    println!("Ex 2.9 ");
    let mut variable = 3;
    let ref1_variable = &mut variable;
    let ref2_variable = &mut variable;
    // the following statement causes an error:
    // *ref1_variable = 4;
    // Indeed, the new reference ‘ref2_variable’ borrows the variable
    // ‘variable’, which causes the previous reference
    // ‘ref1_variable’ to disappear.
    *ref2_variable = 4;
    println!("variable = {}", variable);

    // A mutable reference and a immutable reference  : error as well
    // ----------------------------------------------------------------
    println!("Ex 2.10 ");
    let mut variable = 3;
    let ref1_variable = &variable; // immutable borrow
    let ref2_variable = &mut variable; // mutable borrow
    // the following statement causes an error:
    // let test =  ref1_variable; 
    // (Same reason)
    *ref2_variable = 4;
    println!("variable = {}", variable);



    // a immutable reference to a mutable variable (?)
    // --------------------------------------------
    println!("Ex 2.11 ");
    let mut variable = 3;
    let ref1_variable = &variable; // immutable borrow
    variable = 5;
    // the following statement causes an error:
    // let test =  ref1_variable; 
    let mut variable = "Salut";
    let ref1_variable = &variable; // immutable borrow
    variable = "Bonjour";
    // the following statement causes an error:
    // let test =  ref1_variable; 

    // rustc --explain E0506

    // More info
    // https://medium.com/@manikandan96372/rust-for-beginners-part-7-borrowing-reference-mutable-borrow-immutable-borrow-5c0e5c84e1ef
    // https://dhghomon.github.io/easy_rust/Chapter_17.html
    // https://users.rust-lang.org/t/why-is-this-println-s-treated-as-an-immutable-borrow/78870

    // Cloning:
    // ===========
    // This allows a variable to be duplicated, but it comes at a cost in terms of performance.
    println!("Ex 2.12 ");
    let hello = String::from("Hello, world!");
    //  Variable 'hello' is duplicated:
    let coucou = hello.clone();
    // hello is still accessible
    print!("{}", hello);
    print!("{}", coucou);
}
