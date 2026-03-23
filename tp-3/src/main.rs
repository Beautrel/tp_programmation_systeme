// src/main.rs
use std::collections::HashMap;

// ----------------- Fonctions de l'analyseur -----------------

fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().filter(|mot| !mot.is_empty()).count()
}

fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte.split_whitespace().max_by_key(|mot| mot.len()).unwrap_or("")
}

fn est_palindrome(texte: &str) -> bool {
    let nettoye: String = texte
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
    nettoye.chars().eq(nettoye.chars().rev())
}

fn premiers_mots(texte: &str, n: usize) -> Vec<&str> {
    texte.split_whitespace().take(n).collect()
}

fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

// Structure pour les statistiques
struct Statistiques {
    nb_mots: usize,
    nb_caracteres: usize,
    nb_phrases: usize,
    mot_le_plus_long: String,
    frequence_chars: [(char, usize); 5], // top 5
}

// Analyse globale
fn analyser(texte: &str) -> Statistiques {
    let nb_mots = compter_mots(texte);
    let nb_caracteres = texte.chars().count();
    let nb_phrases = texte.matches(|c| c == '.' || c == '!' || c == '?').count();
    let mot_le_plus_long = mot_le_plus_long(texte).to_string();

    let mut freqs = HashMap::new();
    for c in texte.chars().filter(|c| c.is_alphabetic()) {
        *freqs.entry(c.to_lowercase().next().unwrap()).or_insert(0) += 1;
    }

    let mut top5: Vec<(char, usize)> = freqs.into_iter().collect();
    top5.sort_by(|a, b| b.1.cmp(&a.1));
    let mut top5_array = [(' ', 0); 5];
    for i in 0..5.min(top5.len()) {
        top5_array[i] = top5[i];
    }

    Statistiques {
        nb_mots,
        nb_caracteres,
        nb_phrases,
        mot_le_plus_long,
        frequence_chars: top5_array,
    }
}

// ----------------- Fonction main -----------------

fn main() {
    let texte = "A man a plan a canal Panama. Rust est génial!";

    println!("Texte : {}", texte);
    println!("Nombre de mots : {}", compter_mots(texte));
    println!("Mot le plus long : {}", mot_le_plus_long(texte));
    println!("Palindrome ? {}", est_palindrome(texte));
    println!("Premiers 3 mots : {:?}", premiers_mots(texte, 3));
    println!("Remplacer 'Rust' par 'Python' : {}", remplacer(texte, "Rust", "Python"));

    let stats = analyser(texte);
    println!("\n--- Statistiques globales ---");
    println!("Nombre de mots : {}", stats.nb_mots);
    println!("Nombre de caractères : {}", stats.nb_caracteres);
    println!("Nombre de phrases : {}", stats.nb_phrases);
    println!("Mot le plus long : {}", stats.mot_le_plus_long);
    println!("Top 5 caractères : {:?}", stats.frequence_chars);
}

// ----------------- Tests -----------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compter_mots() {
        assert_eq!(compter_mots("hello world"), 2);
        assert_eq!(compter_mots(""), 0);
        assert_eq!(compter_mots("  espaces  "), 1);
    }

    #[test]
    fn test_palindrome() {
        assert!(est_palindrome("kayak"));
        assert!(est_palindrome("A man a plan a canal Panama"));
        assert!(!est_palindrome("Rust"));
    }

    #[test]
    fn test_mot_le_plus_long() {
        assert_eq!(mot_le_plus_long("le chat est mignon"), "mignon");
        assert_eq!(mot_le_plus_long(""), "");
    }

    #[test]
    fn test_premiers_mots() {
        assert_eq!(premiers_mots("un deux trois quatre", 2), vec!["un", "deux"]);
    }

    #[test]
    fn test_remplacer() {
        assert_eq!(remplacer("bonjour le monde", "le", "la"), "bonjour la monde");
    }

    #[test]
    fn test_analyser() {
        let texte = "Rust est génial. Oui, vraiment!";
        let stats = analyser(texte);
        assert_eq!(stats.nb_mots, 5);
        assert_eq!(stats.nb_caracteres, texte.chars().count());
        assert_eq!(stats.nb_phrases, 2);
        assert_eq!(stats.mot_le_plus_long, "génial.");
    }
}