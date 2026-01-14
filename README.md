

# introduction_rust

## Installing Rust

Install Cargo on Linux or Mac:

`curl https://sh.rustup.rs -sSf | sh`

For Windows, see https://doc.rust-lang.org/cargo/getting-started/installation.html

## Documentation

The Rust documentation is available here: https://doc.rust-lang.org/book/title-page.html

## Exercises and examples

The files can be found at https://github.com/simonpenel/introduction_rust

Download the necessary files and test the installation:

```
git clone https://github.com/simonpenel/introduction_rust.git
cd introduction_rust
cd tuto_rust
cargo run --example ex1
```





## Getting started
In this introduction to Rust, we will cover important aspects of the Rust language.
We will run different Rust codes, then we will see how to create an executable binary.

First, we will test the code provided in the files ex1.rs, ex2.rs, ex3.rs, and ex4.rs in the directory _tuto_rust/examples/_.


To run the code in one of these files, for example ‘ex1.rs’, once in the _tuto_rust_ directory, use the command:

`cargo run --example ex1`

The examples ex1, ex2, ex3 and ex4 cover:

- mutable and immutable variables
- borrowing and ownership
- type inference
- references (a sort of pointer)
- functions
- pattern matching
- the Option type

The code provided contains comments, indicated by “//”


```
// This is a comment
```

which explain the purpose of the exercise.

Some lines are commented out to allow you to test changes to the code. 
Theses lines are preceded by the comment _the following statement causes an error:_

For example:

```
// the following statement causes an error:
// erroneous statement
```

To uncomment a line, simply delete the  // in front of 'erroneous statement'.


### Exercise 1: mutable and immutable variables, type inference 
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/ex1.rs

Run the code:

`cargo run --example ex1`

Uncomment the different erroneous statements  and run the code again.

Observe  the error messages.



### Exercise 2: ownership and borrowing, references
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/ex2.rs

Run the code:

`cargo run --example ex2`

Uncomment the different erroneous statements  and run the code again.

Observe  the error messages.

### Exercise 3: simple functions, call by reference
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/ex3.rs

Run the code:

`cargo run --example ex3`

Uncomment the different erroneous statements  and run the code again.

Observe  the error messages.



### Exercise 4: functions, pattern matching and use of the Option type 
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/ex4.rs

Run the code:

`cargo run --example ex4`

Uncomment the different erroneous statements  and run the code again.

Observe  the error messages.

Understand the use of functions, pattern matching, and the Option type.
Understand the meaning of the absence of a ‘;’ in a function  to return a value.


## Agenda Project

In the project_agenda_[1-11].rs files in the tuto_rust/examples/ directory, we will gradually write code that allows us to explore a calendar and check if an event is scheduled in an agenda.

To execute the code in one of these files, for example ‘project_agenda_7.rs’, once in the tuto_rust directory, use the command:

`cargo run --example project_agenda_7`

These examples cover:

- enumerations (‘enum’)
- the concept of ‘Trait’
- structures (‘struct’)
- functions implemented for a structure (and methods)
- the Vector structure, its construction, and its exploration via an iterator (‘iter’)
- iterators (Iterator) and certain associated methods (‘filter’, ‘enumerate’, ‘collect’, “map”, ‘fold’)
- chaining the ‘expect()’ and ‘unwrap’ methods after the Option or Result type
- implementing an iterator dedicated to a structure


### project agenda 1: enumerations (_enum_) and  their use with  pattern matching 
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_1.rs

Run the code:

`cargo run --example projet_agenda_1`

Uncomment the different erroneous statements  and run the code again.

Observe  the error messages.



### Project Agenda 2: Creating Structures (_struct_)
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_2.rs

Run the code:

`cargo run --example project_agenda_2`

Uncomment the different erroneous statements  and run the code again.

Observe  the error messages.

### Project Agenda 3: Implement a trait to associate with a structure
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_3.rs

Learn more about traits: https://doc.rust-lang.org/rust-by-example/trait.html

An example of a trait is the Display trait. If a structure has this trait, it can be displayed using _println!(‘{}’)_.
(The Debug trait allows ‘verbose’ display using _println!(‘{:?}’)_, this trait does not need to be implemented) 
Here we have created a new structure, for which this trait is of course not defined.
It is up to us to implement it if we want to display this structure, and this will allow us to define the form in which the structure is displayed.

Run the code:

`cargo run --example project_agenda_3`





### Project Agenda 4: Implementing a trait to associate with a structure
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_4.rs

Another example of implementing the Display trait for a structure we have defined, in this case the _Date_ structure.
We use pattern matching in the trait implementation.
Run the code:

`cargo run --example project_agenda_4`

### project agenda 5: implement a function to associate with a structure
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_5.rs

Example of function creation. Difference between functions and methods.

Run the code:

`cargo run --example project_agenda_5`

### project agenda 6: create a vector of structures
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_6.rs

Run the code:

`cargo run --example project_agenda_6`

Different ways to instantiate a structure.
Uncomment the different erroneous statements  and run the code again.

Observe  the error messages.


### Project Agenda 7: Using iterators to process vectors, method chaining
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_7.rs

Run the code:

`cargo run --example projet_agenda_7`

Uncomment the different erroneous statements  and run the code again.

Observe  the error messages.

### Project Agenda 8: Further with iterators, file reading, expect and unwrap methods
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_8.rs

Run the code:

`cargo run --example project_agenda_8`



### project agenda 9: defining several functions to manage dates 
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_9.rs

Run the code:

`cargo run --example projet_agenda_9`

Check what happens if we remove the Copy trait from Month.


### Agenda project 10: using methods related to iterators 
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_10.rs

We will discover the power of iterator-related methods: filter, map, fold.

Run the code:

`cargo run --example project_agenda_10`



### Project Agenda 11: Implement an iterator for a structure
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_11.rs

Run the code:

`cargo run --example projet_agenda_11`

Exercise: define the last element of the iterator implemented for the structure.
See https://doc.rust-lang.org/std/iter/index.html

## Final project

All these elements are grouped together in the tuto_rust/src directory to write a programme.

https://github.com/simonpenel/introduction_rust/tree/master/tuto_rust/src


In the tuto_rust directory, the command


`cargo build`

will create the executable binary

`tuto_rust/target/debug/tuto_rust`

We can see how the code is organised into modules, in this case a single module ‘agenda.rs’ and the main programme in ‘main.rs’.

We use the Result type to handle errors.

We briefly discuss the problem of concatenating character strings.

Execute the code:

`target/debug/tuto_rust date_naissance.txt`

Check error handling by passing incorrect files as parameters.

We discuss code documentation. The command

`cargo doc --open`

command generates documentation in HTML format.

Exercise: 

Write a programme that sorts a birthday calendar  with different options: alphabetically by surname, first name, or by date (day/month, day/month/year). You will need to implement a trait for the Date structure that allows comparison.
