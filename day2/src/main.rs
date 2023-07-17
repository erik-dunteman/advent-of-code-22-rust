use std::fs;
use std::collections::HashMap;

fn main() {
    // read file into string
    let input: String = fs::read_to_string("src/input.txt").unwrap();

    // create map of string to map of string to int
    let mut scores: HashMap<String, HashMap<String, i32>> = HashMap::new();
    // init my moves
    scores.insert("X".to_string(), HashMap::new());
    scores.insert("Y".to_string(), HashMap::new());
    scores.insert("Z".to_string(), HashMap::new());
    
    // init scores for each of their responses
    // yes this is poorly coded
    scores.get_mut("X").unwrap().insert("A".to_string(), 1 + 3); // Rock - Rock
    scores.get_mut("X").unwrap().insert("B".to_string(), 1 + 0); // Rock - Paper
    scores.get_mut("X").unwrap().insert("C".to_string(), 1 + 6); // Rock - Scissors
    
    scores.get_mut("Y").unwrap().insert("A".to_string(), 2 + 6); // Paper - Rock
    scores.get_mut("Y").unwrap().insert("B".to_string(), 2 + 3); // Paper - Paper
    scores.get_mut("Y").unwrap().insert("C".to_string(), 2 + 0); // Paper - Scissors
    
    scores.get_mut("Z").unwrap().insert("A".to_string(), 3 + 0); // Scissors - Rock
    scores.get_mut("Z").unwrap().insert("B".to_string(), 3 + 6); // Scissors - Paper
    scores.get_mut("Z").unwrap().insert("C".to_string(), 3 + 3); // Scissors - Scissors

    // PART 1

    // split by lines
    let lines = input.split("\n");
    let mut cum_score = 0;
    for line in lines {
        println!("{}", line);
        // split by space
        let moves: Vec<&str> = line.split(" ").collect();
        if moves.len() != 2 {
            continue;
        }
        // get moves
        let their_move = moves[0];
        if !(their_move == "A" || their_move == "B" || their_move == "C") {
            continue;
        }

        let my_move = moves[1];
        if !(my_move == "X" || my_move == "Y" || my_move == "Z") {
            continue;
        }

        // get score
        let score = scores.get(my_move).unwrap().get(their_move).unwrap();
        println!("me {} them {}, score {}", my_move, their_move, score);
        cum_score += score;
    }

    println!("cumulative score {}", cum_score);

    // PART 2

    // split by lines
    let lines = input.split("\n");
    let mut cum_score = 0;
    for line in lines {
        println!("{}", line);
        // split by space
        let moves: Vec<&str> = line.split(" ").collect();
        if moves.len() != 2 {
            continue;
        }
        // get moves
        let their_move = moves[0];
        if !(their_move == "A" || their_move == "B" || their_move == "C") {
            continue;
        }

        let instruction = moves[1];
        if !(instruction == "X" || instruction == "Y" || instruction == "Z") {
            continue;
        }

        // match instruction
        let my_move = match instruction {
            "X" => {
                // lose
                match their_move {
                    "A" => "Z",
                    "B" => "X",
                    "C" => "Y",
                    _ => panic!("invalid move")
                }
            },
            "Y" => {
                // tie
                match their_move {
                    "A" => "X",
                    "B" => "Y",
                    "C" => "Z",
                    _ => panic!("invalid move")
                }
            },
            "Z" => {
                // win
                match their_move {
                    "A" => "Y",
                    "B" => "Z",
                    "C" => "X",
                    _ => panic!("invalid move")
                }
            },
            _ => panic!("invalid instruction")
        };

        // get score
        let score = scores.get(my_move).unwrap().get(their_move).unwrap();
        println!("me {} them {}, score {}", my_move, their_move, score);
        cum_score += score;
    }
    println!("cumulative score {}", cum_score);
}
