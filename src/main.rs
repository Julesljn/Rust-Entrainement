fn main() {

// -- 1 --
// Afficher tous les nombres pairs entre 30 et 75 (inclus)

// for i in 30..75 {
//     if i % 2 == 0 {
//         println!("Nombre pair : {}", i);
//     }
// }

// -- 2 --
// Afficher la table de multiplication de 1 à 10

// for i in 1..10 {
//     println!("Voici la table de {}", i);
//     for t in 1..10 {
//         println!("{t} x {i} = {}", i*t);
//     }
// }

// -- 3 --
// Afficher toutes les tables de multiplication des nombres pairs uniquement entre 10 et 20 (inclus)

for i in 10..20 {
    if i % 2 == 0 {
        println!("Voici la table de {}", i);
    for t in 1..10 {
        println!("{t} x {i} = {}", i*t);
    }
    }
}

// -- 4 --
// Calculer l'addition des nombres pairs entre 10 et 30




// -- 5 --
// Créer une fonction qui convertit n'importe quelle monnaie (avec le taux de change en paramètre) en Euros



// -- 6 --
// Calculer la moyenne de chiffres provenant d'un array



// -- 7 --
// Créer une fonction qui prend une string en paramètre et la renvoie à l'envers



// -- 8 --
// Créer une fonction qui compte le nombre de mots dans un texte



// -- 9 --
// Créer une fonction qui renvoie le plus petit chiffre d'un array



// -- 10 --
// Créer une fonction qui met la première lettre de chaque mot d'un texte en majuscules



// -- 11 --
// Créer une fonction qui compte le nombre de fois un caractère passé en argument est trouvé dans une string lui aussi passé en argument
// Ex : countChar("Hello World!", "l") => 3



// -- 12 --
// j'ai un array qui contient les années de naissance de mes clients
// je veux créer un array qui contient leurs âges (on ne tient pas compte des mois ni des jours) et afficher la liste des âges



// -- 13 --
// Créer une fonction qui demande à l'utilisateur son age et lui indique dans combien d'années il aura 100 ans



// -- 14 --
// Créez un tableau qui contient les prénoms des élèves d'EDEN School Paris (au moins 10)
// Affichez dans la page la liste des élèves.
// Ajoutez un bouton "Ajouter un élève" qui permet d'ajouter un élève à la liste



// -- 15 --
// J'ai 103€ sur mon compte bancaire. J'économise pour financer mon voyage autour du monde. Tous les 2 mois, je mets 55€ de côté. Combien de temps faut-il pour que j'ai 13400€ sur mon compte ?



// Afficher le résultat en détaillant les 4 informations suivantes :
// (1) nombre de mois nécessaires
// (2) nombre d'années' nécessaires
// (3) nombre de semaines nécessaires
// (4) nombre de jours nécessaires
}