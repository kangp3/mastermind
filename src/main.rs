use std::cmp;
use std::io;
use std::iter;
use rand::Rng;

const MAX_GUESSES: u8 = 10;
const EMOJIS: [char; 6] = ['🟥', '🟦', '🟧', '🟨', '🟩', '🟪'];

fn main() {
    println!("MASTERMIND");

    let mut rng = rand::thread_rng();
    let mut solution: Vec<u8> = vec![];
    for _ in 0..4 {
      solution.push(rng.gen_range(0..6))
    }

    println!("Input 4 guesses separated by spaces!");
    println!(
        "1) Red
2) Blue
3) Orange
4) Yellow
5) Green
6) Purple"
    );

    let mut num_guesses = 0;
    'a: loop {
        let stdin = io::stdin();
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        let mut guesses: Vec<u8> = vec![];
        for guess in buf.split_whitespace() {
            match guess.parse::<u8>() {
                Ok(v) => {
                    if v < 1 || v > 6 {
                        println!("YA MESSED UP: Guesses should be between 1 and 6");
                        continue 'a;
                    }

                    guesses.push(v-1);
                }
                Err(e) => {
                    println!("YA MESSED UP: {:?}", e);
                    continue 'a;
                }
            }
        }
        if guesses.len() != 4 {
            println!("YA MESSED UP: Must have exactly 4 guesses");
            continue;
        }

        let filtered_guesses: Vec<(u8, u8)> = iter::zip(solution.clone(), guesses.clone())
            .filter(|(x, y)| x != y)
            .collect();
        let black_pegs = 4 - filtered_guesses.len();
        let mut sol_color_map = vec![0; 6];
        for (sol, _) in &filtered_guesses {
            sol_color_map[*sol as usize] += 1;
        }
        let mut guess_color_map = vec![0; 6];
        for (_, guess) in &filtered_guesses {
            guess_color_map[*guess as usize] += 1;
        }
        let white_pegs: u8 = iter::zip(sol_color_map, guess_color_map)
            .map(|(x, y)| cmp::min(x, y))
            .sum();
        println!(
            "{}{}{}{} {black_pegs} black pegs, {white_pegs} white pegs",
            EMOJIS[guesses[0] as usize],
            EMOJIS[guesses[1] as usize],
            EMOJIS[guesses[2] as usize],
            EMOJIS[guesses[3] as usize],
        );

        if black_pegs == 4 {
            println!("YOU WIN. YOU ARE THE GREATEST.");
            break;
        }

        num_guesses += 1;
        if num_guesses == MAX_GUESSES {
            println!("YOU LOSE.");
            break;
        }
    }
}

