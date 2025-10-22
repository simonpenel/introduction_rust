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

Certains lignes sont commentées pour vous permettre de tester des modifications du code.
Pour décommenter une ligne, il suffit de supprimer le // initial.


### Exercice 1 : variables mutables et non mutables, l'inférence de type 
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/ex1.rs

Exécuter le code:

`cargo run --example ex1`

Décommenter la ligne  15 et exécuter le code à nouveau.

Observer  le message d'erreur.

Même chose avec la ligne 18

Même chose avec la ligne 39



### Exercice 2 : l'ownership et le borrowing, les références
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/ex2.rs

Exécuter le code:

`cargo run --example ex2`

Décommenter la ligne  33 et exécuter le code à nouveau.

Observer  le message d'erreur.

Décommenter la ligne  89 et exécuter le code à nouveau.

Observer  le message d'erreur.

Décommenter la ligne  148 et exécuter le code à nouveau.

Observer  le message d'erreur.

Décommenter la ligne  165 et exécuter le code à nouveau.

Observer  le message d'erreur.


### Exercice 3 : fonctions simples, l'appel par référence
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/ex3.rs

Exécuter le code:

`cargo run --example ex3`

Décommenter la ligne  22 et exécuter le code à nouveau.

Observer  le message d'erreur.

Décommenter la ligne  42 et exécuter le code à nouveau.

Observer  le message d'erreur. Il s'agit d'un problème de _borrowing_.

Expliquer pourquoi la fonction ma_fonction_3 n'a pas modifié la variable.

Comprendre ce qu'il se passe avec les variable de type String.

Décommenter la ligne  111 et exécuter le code à nouveau.

### Exercice 4 : fonctions, pattern matching et utilisation du  type Option 
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/ex4.rs

Exécuter le code:

`cargo run --example ex4`

Comprendre l'utilisation des fonctions, du pattern matching et du type Option.


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


### projet agenda 1 : les énumérations (_enum_) et  leur utilisation avec le  pattern matching 
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_1.rs

Exécuter le code:

`cargo run --example projet_agenda_1`

Décommenter les lignes  23,24,25 et exécuter le code à nouveau. 

### projet agenda 2 : créer ses structures (_struct_)
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_2.rs

Exécuter le code:

`cargo run --example projet_agenda_2`

Décommenter la ligne  40 et exécuter le code à nouveau.

### projet agenda 3 : implémenter un trait à associer à une structure
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_3.rs

En savoir plus sur les traits: http...

Un exemple de trait et le trait Display. Si une structure a ce trait, elle peut être affichée, par _println!()_ par exemple.
Ici on a crée une nouvelle structure, pour laquelle ce trait n'est bien sur pas défini.
C'est à nous de l'implémenter si on veut afficher cette structure.

Exécuter le code:

`cargo run --example projet_agenda_3`


### projet agenda 4 : implémenter un trait à associer à une structure
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_4.rs

Un autre exemple d'implémentation du trait Display pour une structure que nous avons définie, ici la structure _Date_.
On utilise le pattern matching dans l'implémentation du trait.
Exécuter le code:

`cargo run --example projet_agenda_4`

### projet agenda 5 : implémenter une fonction à associer à une structure
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_5.rs

Exemple de création de fonction. Différence entre les fonctions et les méthodes.

Exécuter le code:

`cargo run --example projet_agenda_5`

### projet agenda 6 : créer un vecteur de structures
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_6.rs

Exécuter le code:

`cargo run --example projet_agenda_6`

Différentes manière d'instancier une structure.
Décommenter la ligne  114 et exécuter le code à nouveau.


Décommenter la ligne  117 et exécuter le code à nouveau.

### projet agenda 7 : utiliser les itérateurs pour traiter des vecteurs, chaînage de méthode
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_7.rs

Exécuter le code:

`cargo run --example projet_agenda_7`

Décommenter les lignes  118,119,120 et exécuter le code à nouveau.

### projet agenda 8 : plus loin avec les itérateurs, lecture de fichier, methodes expect et unwrap
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_8.rs

Exécuter le code:

`cargo run --example projet_agenda_8`



### projet agenda 9 : définition de plusieurs fonctions pour gérer les dates 
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_9.rs

Exécuter le code:

`cargo run --example projet_agenda_9`

Vérfier ce qui se passe si on supprime le trait Copy de Mois.


### projet agenda 10 : utilisation des methodes liées aux iterateurs 
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_10.rs

Exécuter le code:

`cargo run --example projet_agenda_10`



### projet agenda 11 : implementer un iterateur pour une structure
https://github.com/simonpenel/introduction_rust/blob/master/tuto_rust/examples/projet_agenda_11.rs

Exécuter le code:

`cargo run --example projet_agenda_11`


## Projet final

Tous ces éléments sont regroupés dans le répertoire tuto_rust/src pour écrire un programme.

https://github.com/simonpenel/introduction_rust/tree/master/tuto_rust/src


Dans  le répertoire tuto_rust, la commande


`cargo build`

va créer le binaire exécutable

`tuto_rust/target/debug/tuto_rust`

On peut voir l'organisation du code en modules, ici un seul module "agenda.rs" et le programme principal dans "main.rs".

On utilise le type Result pour gérer les erreur.

On aborde un peu le problème de concaténation de chaînes de caractères.

Exécuter le code:

`target/debug/tuto_rust date_naissance.txt`

On aborde la documentation du code. La commande

`cargo doc --open`

permet de générer la documentation sous la forme html.
