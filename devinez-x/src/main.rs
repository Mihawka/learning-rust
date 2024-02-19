use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new(); // Contiendra les entrées utilisateurs
    let mut health_left = 5;
    let num_to_guess = (rand::random::<f32>() * 10.0) as i32; // On génère un nombre à deviner entre 0 et 10

    while health_left != 0 {
        print!("Vie restante : {health_left} > ");
        stdout().flush().unwrap(); // Force l'affichage de notre print! sinon il ne s'affichera pas et restera en cache

        input.clear(); // On supprime l'entrée utilisateur précédent
        stdin().read_line(&mut input).unwrap(); // Lecture des entrées utilisateur

        match input.trim_end().parse::<i32>() {
            // On trim_end pour enlever \n qui vient s'ajouter quand on appuie sur Enter
            // Si l'entrée utilisateur est valide alors on récupère sa valeur
            Ok(value) => {
                if value < 0 || value > 10 {
                    println!("Ce nombre n'est pas compris entre 0 et 10 !");
                    continue;
                } else if game_logic(value, &num_to_guess) {
                    // Si tu utilises un IDE passe ton curseur sur game_logic pour voir le commentaire écrit
                    break;
                }
            }
            // Si l'utilisateur n'entre pas un nombre entre 0 et 10 ou autre
            // alors on renvoi un message d'erreur et on relance la boucle
            Err(_) => {
                // Ici le underscore sert à dire qu'on ne veut pas utiliser les informations de l'erreur
                println!("Ceci n'est pas un nombre !");
                continue;
            }
        }

        health_left -= 1; // En rust l'incrémentation et la décrémentation n'existe pas
    }

    println!("Tu as perdu !"); // Message de fin
}

/// Exemple de commentaire apparaissant dans l'IntelliSense.\
/// Renvoi :
/// - `vrai` si l'utilisateur à gagné
/// - `faux` s'il a perdu
fn game_logic(guess: i32, num_to_guess: &i32) -> bool {
    if guess == *num_to_guess {
        println!("Tu as gagné !");
        return true;
    } else if guess < *num_to_guess {
        println!("C'est plus grand !");
    } else if guess > *num_to_guess {
        println!("C'est plus petit !");
    }
    false
}
