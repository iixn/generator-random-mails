use rand::Rng;

fn main() {

    let abecedario = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    let domains = vec!["gmail", "yahoo", "outlook", "proton"];
    let mut word = String::new();

    while word.len() < 10 {
        let random_letter = rand::thread_rng().gen_range(1..26);
        word.push_str(abecedario[random_letter]);
    }

    let select_domain = rand::thread_rng().gen_range(1..4);
    word.push_str("@");
    word.push_str(domains[select_domain]);
    word.push_str(".com");

    println!("\nYour new electronic mail account : {word}");
}
