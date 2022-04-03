use std::io;
use std::env;
use std::process;


pub const MAX: usize = 9;

pub struct Candidate {
    pub name: String,
    pub votes: u32
}

fn main() {
    // Makes an array of candidates of length MAX
    let mut candidates: Vec<Candidate> = Vec::new();
    let mut candidate_count: usize;

    // Put the command line arguments in var
    // and check if there is at least 2 of them
    let args: Vec<String> = env::args().collect();
    let argc = args.len();
    if argc < 3 {
        println!("Usage: cargo run cadidate1 candicate2....");
        process::exit(1); 
    }

    // Check if there isn't to many candidates
    candidate_count = argc - 1;
    if candidate_count > MAX {
        println!("Too many candidates ! Max is {}", MAX);
        process::exit(2); 
    }


    for i in 0..candidate_count {
        candidates[i].name = args[i + 1].clone();
        candidates[i].votes = 0;
    }

    let voter_count = string_to_int(&input("Number of voters : "));

    for i in 0..voter_count {
        let name = input("Vote : ");

        // Check for invalid vote
        if !vote(&name) {
            println!("Invalid vote.");
        }
    }

    print_winner();
}


fn vote(name: &str) -> bool {
    // TODO
    // On voit qu'il ne prend pas le vec en paramatre.
    // Ce qui veut dire que si je veux respecter l'énoncer, je dois trouver
    // le moyen de declarer `candidates` en variable globale.
    false
}

fn print_winner() {
    // TODO
    // Même problématique, `candidates` a besoin d'être en variable globale
    // Ensuite je n'ai plus qu'à comparer les résultats
    println!("");
    process::exit(0)
}







fn input(message: &str) -> String {
    // This function displays an informative string to the user, and return his input
    println!("{}", message);
    let mut user_input = String::from("");
    io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input.");
    user_input
}

fn string_to_int(nb_str: &str) -> u8 {
    // Transform a string into an unsigned int, or exit if failed.
    let nb: u8 = match nb_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please provide a correct number.");
            process::exit(1);
        },
    };
    nb
}