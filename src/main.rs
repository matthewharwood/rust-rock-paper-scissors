use std::io;
use std::cmp::PartialEq;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
}; 
use std::fmt;
use dialoguer:: {
    Select
};


#[derive(Debug)]
struct OddNumber(u32);

impl OddNumber {
    pub fn create(val: u32) -> Option<OddNumber> {
        if val % 2 == 0 {
            None
        } else {
            Some(OddNumber(val))
        }
    }
}

impl PartialEq for OddNumber {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}




#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Tie
}

impl PartialEq for Outcome {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl fmt::Display for RPS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
   
    }
}

impl Distribution<RPS> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RPS {
        match rng.gen_range(0, 3) {
            0 => RPS::Rock,
            1 => RPS::Paper,
            2 => RPS::Scissors,
            _ => RPS::Scissors,
        }
    }
}
fn rps_match(val: usize) -> RPS {
    match val {
        0 => RPS::Rock,
        1 => RPS::Paper,
        2 => RPS::Scissors,
        _ => RPS::Scissors,
    }
}

#[derive(Debug)]
enum Player {
    Enemy,
    You,
}
type GameSet = (Player, RPS, Player, RPS);
#[derive(Debug)]
struct Game {
    num_of_sets: Option<OddNumber>,
    sets: Vec<GameSet>,
}
fn calc_winner(round: &GameSet) -> Outcome { 
    let outcome = match (&round.1, &round.3) {
        (RPS::Rock, RPS::Paper) => (&round.0, Outcome::Lose),
        (RPS::Rock, RPS::Scissors) => (&round.0, Outcome::Win),
        (RPS::Paper, RPS::Rock) => (&round.0, Outcome::Win),
        (RPS::Paper, RPS::Scissors) => (&round.0, Outcome::Lose),
        (RPS::Scissors, RPS::Rock) => (&round.0, Outcome::Lose),
        (RPS::Scissors, RPS::Paper) => (&round.0, Outcome::Win),
        _ => (&round.0, Outcome::Tie),
    };
    println!("{:?}: {:?}", outcome, round);
    outcome.1
}
fn add_up_wins(acc: u32, v: Outcome) -> u32 {
    if let Outcome::Win = v {
        acc + 1
    } else {
        acc
    }
}

fn main() {
    let mut buffer = String::new();
    let mut game = Game{num_of_sets: None, sets: Vec::new()};
    println!("How many matches?");
    io::stdin().read_line(&mut buffer).expect("not a value");

    game.num_of_sets = OddNumber::create(buffer.trim().parse().unwrap());
    let mut count = 0;

    loop {
        if game.num_of_sets == OddNumber::create(count) {
            break;
        }
        

        println!("Please choose {:?}, {:?}, {:?}", RPS::Rock, RPS::Paper, RPS::Scissors);
        let mut select = Select::new();
        select.item(&RPS::Rock.to_string());
        select.item(&RPS::Paper.to_string());
        select.item(&RPS::Scissors.to_string());
        let player_select = select.interact();  
        let rps: RPS = rand::random();
    
        game.sets.push((Player::You, rps_match(player_select.ok().unwrap()), Player::Enemy, rps ));
        calc_winner(game.sets.get(count as usize).unwrap());
        count += 1;

    }
    let res = game.sets.iter().map(calc_winner).fold(0, add_up_wins);
    if(res > 2) {

        println!("you've won! {:?}", res);
    } else {
        println!("you've lost :( {:?}", res);
    }
    
}

