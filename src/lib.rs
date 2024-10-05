use rand::Rng;
use rand::distributions::{Distribution, Standard};
use std::cmp::max;
use std::fmt::Display;

const TEAM_CAP: usize = 8;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Species {
    Tyrannosaurus, // high health and attack, infight
    Velociraptor,  // high attack, low health and defense
    Triceratops,   // high defense, low attack, moderate health
    Brachiosaurus, // high health, low attack and defense
    Pteranodon,    // low everything, heals teammates
    Dilophosaurus, // low everything, splash damage
                   // STEGOSAURUS, ANKLYOSAURUS, IGUANODON, PLESIOSAURUS
                   // SPINOSAURUS, ALLOSAURUS, PARASAUROLOPHUS, UTAHRAPTOR...
}
impl Distribution<Species> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Species {
        match rng.gen_range(0..6) {
            0 => Species::Tyrannosaurus,
            1 => Species::Velociraptor,
            2 => Species::Triceratops,
            3 => Species::Brachiosaurus,
            4 => Species::Pteranodon,
            5 => Species::Dilophosaurus,
            _ => panic!("Invalid species"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Dino {
    species: Species,
    health: i8,
    attack: i8,
    defense: i8,
    heal: i8,
    splash: i8,
    infight: i8,
}

impl Display for Dino {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} ({})", self.species, self.health)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Team {
    dinos: [Option<Dino>; TEAM_CAP],
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.dinos
                .iter()
                .filter(|dino| dino.is_some())
                .map(|dino| dino.unwrap().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

pub struct Battle {
    left: Team,
    right: Team,
}

impl Dino {
    fn damage(self, damage: i8) -> Dino {
        let actual_damage = max(1, damage - self.defense);
        println!("{:?} was hit for {} damage", self.species, actual_damage);
        if actual_damage >= self.health {
            println!("{:?} died!", self.species);
        }
        Dino {
            health: self.health - actual_damage,
            ..self
        }
    }

    fn heal(self, heal: i8) -> Dino {
        if heal > 0 {
            println!("{:?} was healed for {} damage", self.species, heal);
        }
        Dino {
            health: self.health + heal,
            ..self
        }
    }

    fn is_dead(self) -> bool {
        self.health <= 0
    }

    pub fn tyrannosaurus() -> Dino {
        Dino {
            species: Species::Tyrannosaurus,
            health: 4,
            attack: 5,
            defense: 1,
            heal: 0,
            splash: 0,
            infight: 2,
        }
    }

    pub fn velociraptor() -> Dino {
        Dino {
            species: Species::Velociraptor,
            health: 2,
            attack: 4,
            defense: 0,
            heal: 0,
            splash: 0,
            infight: 0,
        }
    }

    pub fn triceratops() -> Dino {
        Dino {
            species: Species::Triceratops,
            health: 4,
            attack: 3,
            defense: 2,
            heal: 0,
            splash: 0,
            infight: 0,
        }
    }

    pub fn brachiosaurus() -> Dino {
        Dino {
            species: Species::Brachiosaurus,
            health: 7,
            attack: 1,
            defense: 0,
            heal: 0,
            splash: 0,
            infight: 0,
        }
    }

    pub fn pteranodon() -> Dino {
        Dino {
            species: Species::Pteranodon,
            health: 1,
            attack: 1,
            defense: 0,
            heal: 1,
            splash: 0,
            infight: 0,
        }
    }

    pub fn dilophosaurus() -> Dino {
        Dino {
            species: Species::Dilophosaurus,
            health: 1,
            attack: 0,
            defense: 0,
            heal: 0,
            splash: 1,
            infight: 0,
        }
    }

    pub fn to_damage(self) -> [i8; TEAM_CAP] {
        let mut damage = [self.splash; TEAM_CAP];
        damage[0] += self.attack;
        damage
    }

    pub fn to_healing(self) -> [i8; TEAM_CAP] {
        let mut healing = [self.heal; TEAM_CAP];
        healing[0] = 0;
        healing
    }

    pub fn from_species(species: Species) -> Dino {
        match species {
            Species::Tyrannosaurus => Dino::tyrannosaurus(),
            Species::Velociraptor => Dino::velociraptor(),
            Species::Triceratops => Dino::triceratops(),
            Species::Brachiosaurus => Dino::brachiosaurus(),
            Species::Pteranodon => Dino::pteranodon(),
            Species::Dilophosaurus => Dino::dilophosaurus(),
        }
    }
}

impl Team {
    pub fn random_team() -> Team {
        let mut dinos = [None; TEAM_CAP];
        for dino in &mut dinos {
            *dino = Some(Dino::from_species(rand::random()));
        }
        Team { dinos }
    }

    fn damage(self, damage: [i8; TEAM_CAP]) -> Team {
        let mut new_dinos = [None; TEAM_CAP];
        let mut target_idx: usize = 0;
        let mut damage_idx: usize = 0;
        for i in 0..8 {
            if let Some(dino) = self.dinos[i] {
                let new_dino = dino.damage(damage[damage_idx]);
                if !new_dino.is_dead() {
                    new_dinos[target_idx] = Some(new_dino);
                    target_idx += 1;
                }
                damage_idx += 1;
            }
        }
        Team { dinos: new_dinos }
    }

    fn infight(self, damage: i8, species: Species) -> Team {
        let mut new_dinos = [None; TEAM_CAP];
        let mut current_idx: usize = 0;
        for i in 0..8 {
            if let Some(dino) = self.dinos[i] {
                if dino.species == species {
                    let new_dino = dino.damage(damage);
                    if !new_dino.is_dead() {
                        new_dinos[current_idx] = Some(new_dino);
                        current_idx += 1;
                    }
                } else {
                    new_dinos[current_idx] = Some(dino);
                    current_idx += 1;
                }
            }
        }
        Team { dinos: new_dinos }
    }

    fn heal(self, healing: [i8; TEAM_CAP]) -> Team {
        let mut new_dinos = [None; TEAM_CAP];
        let mut next_idx: usize = 0;
        for i in 0..8 {
            if let Some(dino) = self.dinos[i] {
                new_dinos[next_idx] = Some(dino.heal(healing[next_idx]));
                next_idx += 1;
            }
        }
        Team { dinos: new_dinos }
    }

    fn is_dead(self) -> bool {
        self.dinos
            .iter()
            .map(|dino| match dino {
                Some(dino) => dino.is_dead(),
                None => true,
            })
            .all(|x| x)
    }

    fn first_dino(self) -> Option<Dino> {
        self.dinos.iter().find_map(|dino| *dino)
    }
}

#[derive(Debug)]
pub enum BattleResult {
    LeftWins,
    RightWins,
    Stalemate,
}

impl Battle {
    pub fn new(left: Team, right: Team) -> Battle {
        Battle { left, right }
    }

    fn battle_one(self) -> Battle {
        println!("Left team: {}", self.left);
        println!("VS");
        println!("Right team: {}", self.right);
        match (self.left.first_dino(), self.right.first_dino()) {
            (Some(left_dino), Some(right_dino)) => Battle {
                left: self
                    .left
                    .heal(left_dino.to_healing())
                    .damage(right_dino.to_damage())
                    .infight(left_dino.infight, left_dino.species),
                right: self
                    .right
                    .heal(right_dino.to_healing())
                    .damage(left_dino.to_damage())
                    .infight(right_dino.infight, right_dino.species),
            },
            (_, _) => self,
        }
    }

    pub fn battle(self) -> BattleResult {
        let mut battle: Battle = self;
        loop {
            match (battle.left.is_dead(), battle.right.is_dead()) {
                (true, true) => return BattleResult::Stalemate,
                (true, false) => return BattleResult::RightWins,
                (false, true) => return BattleResult::LeftWins,
                (_, _) => (),
            }
            battle = battle.battle_one();
        }
    }
}
