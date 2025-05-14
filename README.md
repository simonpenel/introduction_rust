# introduction_rust

## Installation de Rust

Installer cargo sous linux ou Mac:

`curl https://sh.rustup.rs -sSf | sh`

pour  Windows voir  https://doc.rust-lang.org/cargo/getting-started/installation.html

## Documentation

La documentation Rust est accessible ici : https://doc.rust-lang.org/book/title-page.html

## Exercices et exemples

Les fichiers se trouvent  à  l'adresse  https://github.com/simonpenel/introduction_rust

Récupérer les fichiers nécessaires et tester l'installation:

```
git clone https://github.com/simonpenel/introduction_rust.git
cd introduction_rust
cd tuto_rust
cargo run --example ex1
```





## Premiers pas
Dans cette introduction à Rust, on  va aborder des aspects importants du langage Rust.
Dans un premier temmp on va executer differents codes Rust. On verra ensuite comment créer un binaire executable.

Tout d'abord on va tester les code proposés dans les  fichiers ex1.rs, ex2.rs ex3.rs et ex4.rs dans le répertoire tuto_rust/examples/


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

Les codes proposés contiennent des commentaires, signalés par '//'

```
// Ceci est un commentaire
```

qui présentent le but de l'exercice.

Certains lignes sont comentées pour vous permettre de tester des modifications du code.
Pour décommenter une ligne, il suffit de supprimer le // initial.


### Exercice 1 : variables mutables et non mutables
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/ex1.rs

Exécuter le code:

`cargo run --example ex1`

Décommenter la ligne  14 et exécuter le code à nouveau.

Observer  le message d'erreur.

Même chose avec la ligne 17


### Exercice 2 : l'ownership et le borrowing, les références
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/ex2.rs

Exécuter le code:

`cargo run --example ex2`

Décommenter la ligne  24 et exécuter le code à nouveau.

Observer  le message d'erreur.


### Exercice 3 : inference de type et fonctions simples
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/ex3.rs

Exécuter le code:

`cargo run --example ex3`

Décommenter la ligne  12 et exécuter le code à nouveau.

Observer  le message d'erreur.

Décommenter la ligne  42 et exécuter le code à nouveau.

Observer  le message d'erreur. Il s'agit d'un problème de _borrowing_.

Expliquer pourquoi la fonction ma_fonction_3 n'a pas modifié la variable.




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

On aborde un peu le problème de concaténation de chaînes de caractères.

On aborde la documentation du code. La commande

`cargo doc --open`

permet de générer la documentation sous la forme html.
