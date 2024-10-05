use dinocore::{Battle, Team};

fn random_battle() {
    let battle = Battle::new(Team::random_team(), Team::random_team());
    println!("{:?}", battle.battle());
}

fn main() {
    random_battle();
}
