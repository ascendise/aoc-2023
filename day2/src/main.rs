use std::{io::{BufReader, BufRead}, fs::File};

use game::Game;

use crate::game::CubeSet;

mod game;

fn main() {
    let file = File::open("games.txt")
        .expect("Failed to open games.txt");
    let content = CubeSet::new(12, 13, 14);
    let games: Vec<Game> = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|l| Game::from(&l))
        .collect();
    //Part1
    let game_id_sum: u32 = games.iter()
        .map(|g| if g.is_content(&content) { g.id } else { 0 } )
        .sum();
    println!("Sum of all game ids with content is {game_id_sum}");

    //Part2
    let set_powers: u32 = games.iter()
        .map(|g| g.get_minimum_content().power())
        .sum();
    println!("Combined power of all minimum contents is {set_powers}");

}
 