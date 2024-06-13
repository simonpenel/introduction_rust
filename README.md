# introduction_rust

## Installation de Rust

Installer cargo:

`curl https://sh.rustup.rs -sSf | sh`

pour  Windows voir  https://doc.rust-lang.org/cargo/getting-started/installation.html

## Introduction

La documentation Rust est accessible ici : https://doc.rust-lang.org/book/title-page.html

## Premiers pas
Dans cette introduction à Rust, on  va aborder des aspects importants du langage Rust.
Tout d'abord dans les fichiers ex1.rs, ex2.rs ex3.rs et ex4.rs dans le répertoire tuto_rust/examples/

Pour executer le code de l'un ces fichiers, par exemple "ex1.rs", une fois dans le répertoire tuto_rust, on utilise la commande:

`cargo run --example ex1`

Les exemples ex1, ex2, ex3 et ex4 abordent:

- les variables mutables et non mutables
- le "borrowing" et le "ownership"
- les références (sortes de pointeur)
- l'inférence de type
- les fonctions en Rust
- le matching
- le type Option

## Projet Agenda

Dans les fichiers projet_agenda_[1-11].rs dans le répertoire tuto_rust/examples/ on va peu à peu écrire un code qui permet d'explorer un calendrier et vérifier si un événement est prévu dans un agenda.

Pour éxecuter le code de l'un de ces fichiers, par exemple "projet_agenda_7.rs", une fois dans le répertoire tuto_rust, on utilise la commande:

`cargo run --example projet_agenda_7`

Ces exemples abordent :

- les énumerations ("enum")
- la notion de "Trait"
- les structures ("struct")
- les fonctions implementées pour une structure (méthodes)
- la structure Vector, sa construction, son exploration via un itérateur ("iter")
- les itérateurs (Iterator) et certaines méthodes associées ("filter", "enumerate", "collect", "map", "fold")
- le chainage des méthodes "expect()" et "unwrap" après le type Option ou Result
- l'implémentation d'un itérateur dédiée à une structure


## Projet final

Tous ces éléments sont regroupés dans le répertoire tuto_rust/src pour écrire un programme.

Dans  le répertoire tuto_rust, la commande


`cargo build`

va créer le binaire exécutable

`tuto_rust/target/debug/tuto_rust`

On peut voir l'organisation du code en modules, ici un seul module "agenda.rs" et le programme principal dans "main.rs".

On utilise le type Result pour gérer les erreur.

On aborde la documentation du code. La commande

`cargo doc --open`

permet de générer la documentation sous la forme html.
