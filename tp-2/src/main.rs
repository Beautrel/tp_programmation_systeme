/*programme 1
Ton code ne compile pas à cause du principe d’ownership en Rust.

 Quand on fais :

let v2 = v;
 La propriété de v est déplacée (move) vers v2
Donc v n’est plus utilisable après ça

correction en utilisant clone() pour créer une copie de v au lieu de déplacer sa propriété :
 */
fn main() {
    let v = vec![1, 2, 3];
    let v2 = v.clone();
    println!("Longueur : {}", v.len());
}

//Programme 2:  Corrigez sans utiliser clone() 

fn somme(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}

fn main() {
    let nombres = vec![1, 2, 3, 4, 5];
    let s = somme(&nombres); // on passe une référence
    println!("Somme : {}, Vecteur : {:?}", s, nombres); // nombres reste utilisable
}

//Partie B: Implémentez un gestionnaire de tâches simple (sans références) : 

// Création d'une tâche
fn creer_tache(titre: String, priorite: u8) -> (String, u8, bool) {
    (titre, priorite, false) // bool = false signifie non terminée
}

// Afficher une tâche
fn afficher_tache(tache: (String, u8, bool)) {
    let (titre, priorite, complete) = tache; // move des valeurs
    println!(
        "Tâche : {}, Priorité : {}, Complète : {}",
        titre, priorite, complete
    );
    // après ce point, tache n'existe plus
}

// Marquer une tâche comme complète
fn marquer_complete(tache: (String, u8, bool)) -> (String, u8, bool) {
    let (titre, priorite, _) = tache; // on prend les valeurs, ignore l'ancien bool
    (titre, priorite, true) // nouvelle tâche complète
}

// Extraire le titre d'une tâche
fn extraire_titre(tache: (String, u8, bool)) -> String {
    let (titre, _priorite, _complete) = tache;
    titre // move du titre, tache n'existe plus
}

fn main() {
    // Créer une tâche
    let t1 = creer_tache(String::from("Apprendre Rust"), 1);

    // Marquer comme complète
    let t1 = marquer_complete(t1);

    // Afficher la tâche
    afficher_tache(t1); // t1 est déplacé ici et n'existe plus après

    // Créer une autre tâche
    let t2 = creer_tache(String::from("Faire les exercices"), 2);

    // Extraire le titre
    let titre = extraire_titre(t2); // t2 est déplacé ici
    println!("Titre extrait : {}", titre);
}
