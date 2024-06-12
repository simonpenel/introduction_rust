# introduction_rust

## Installation de Rust

Installer cargo:

`curl https://sh.rustup.rs -sSf | sh`

pour  Windows voir  https://doc.rust-lang.org/cargo/getting-started/installation.html

## Introduction

La documentation Rust est accessible ici : https://doc.rust-lang.org/book/title-page.html

Dans cette introduction à Rust on  va aborder des aspects importants du langage Rust.
Tout d'abord  dans les fichiers ex1.rs, ex2.rs ex3.rs et ex4.rs dans le repertoire tuto_rust/examples/

Pour executer un de ces fichiers, par exemple "ex1.rs", une fois dans le répertoire tuto_rust on utilise la commande:

`cargo run --example ex1`


Les exemples ex1, ex2, ex3 et ex4 abordent:

- les variables mutables et non mutables
- le "borrowing"
- les références (sortes de pointeur)
- l'inférence de type
- les fonctions en Rust
- le matching
- le type Option
