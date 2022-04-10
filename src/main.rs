use std::{io, env, process};


pub const MAX: usize = 9;

#[derive(Debug)]
pub struct Candidate {
    name: String,
    votes: u32
}


fn main() {
    let mut candidates: Vec<Candidate> = Vec::new();
    // Makes an array of candidates of length MAX
    let candidate_count: usize;

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
        let candidate = Candidate {
            name: args[i + 1].clone(),
            votes: 0
        };
        candidates.push(candidate);
    }

    let voter_count = string_to_int(&input("Number of voters : "));

    #[allow(unused_variables)]
    for i in 0..voter_count {
        let name = input("Vote : ").replace("\n", "");

        // Check for invalid vote
        if !vote(&name, &mut candidates) {
            println!("Invalid vote.");
        }
    }

    print_winner(&mut candidates);
}


fn vote(name: &str, candidates: &mut Vec<Candidate>) -> bool {
    for candidate in candidates{
        // println!("{:?}", candidate.name);
        // println!("{:?}", name);
        if candidate.name == name {
            candidate.votes += 1;
            return true
        }
    }
    false
}

fn print_winner(candidates: &mut Vec<Candidate>) {
    let mut best_candidates: Vec<String> = Vec::new();
    let mut best_score = 0;
    for candidate in candidates {
        if candidate.votes >= best_score {
            best_candidates.push(candidate.name.clone());
            best_score = candidate.votes;
        }
    }
    for candidate in best_candidates {
        println!("{}", candidate);
    }
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
