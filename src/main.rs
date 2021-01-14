extern crate rand;
use rand::Rng;
use std::io;

fn main() {
    println!("How many times are we going to play the game? (recommendation: 100 to 100000)");
    let num_games = test_input();
    println!("How many doors are going to be in the game? (recommendation: 3 to 5)");
    let num_doors = test_input();

    let mut wins:f64 = 0.;
    if num_games>=1&&num_doors>=1 {
        println!("Alright! The game will be played {} times with {} doors:\n", num_games, num_doors);

        //First we're going to simulate the game if the player never switched doors:
        for i in 0..(num_games) {
            let mut doors = vec![0;num_doors as usize];
            //The prize is placed behind a random door:
            let prize = rand::thread_rng().gen_range(0..(num_doors));
            doors[prize as usize] = 1;
            //And the player chooses a random door:
            let player_choice = rand::thread_rng().gen_range(0..(num_doors));
            //If they're right, they win. The winrate should just be "1/(number of doors)"
            if doors[player_choice as usize] == 1 {
                wins+=1.;
            }
        }
        println!("Winrate if the player NEVER switched doors: {}%", (100.*wins)/num_games as f64);
        wins = 0.;
        if num_doors>=3{
            /*The second part of the simulation is extremely similar to the first one except the player
            will always choose to switch doors after the host opens a door revealing no prizes behind it:*/
            for i in 0..(num_games) {
                let mut doors = vec![0;num_doors as usize];
                let prize = rand::thread_rng().gen_range(0..(num_doors));
                doors[prize as usize] = 1;
                let mut player_choice = rand::thread_rng().gen_range(0..(num_doors));
                //The host will pick a random closed door with no prizes behind it:
                let mut open_door = rand::thread_rng().gen_range(0..(num_doors));
                while (open_door==player_choice) || (open_door==prize){
                    open_door = rand::thread_rng().gen_range(0..(num_doors));
                }
                //The player then will switch to a different door:
                let old_choice = player_choice;
                player_choice = rand::thread_rng().gen_range(0..(num_doors));
                while (player_choice==open_door) || (player_choice==old_choice){
                    player_choice = rand::thread_rng().gen_range(0..(num_doors));
                }
                //Counterintuitively the winrate won't be "1/(number of doors - 1)":
                if doors[player_choice as usize] == 1 {
                    wins+=1.;
                }
            }
            println!("Winrate if the player ALWAYS switched doors: {}%",(100.*wins)/num_games as f64);
        } else {
            println!("There weren't enough closed doors for the host to open and ask the player if they'd like to switch doors.")
        }
    } else if num_games==0||num_doors==0 {
        println!("There's no point in running this simulation if you're going to pick 0, you know...")
    } else {
        println!("We can't have a negative number of doors or games.")
    }

}

fn test_input() -> i32 {
    loop {
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Não foi possível ler o input.");
        let teste = num.trim().parse::<i32>();
        match teste {
        Ok(ok) => return ok,
        Err(e) => println!("{}", e), 
        }
    } 
}