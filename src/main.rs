use std::time::{Duration, Instant};
use std::thread::sleep;
use std::collections::HashMap;

#[derive(Debug)]
struct Resource {
    name: String,
    amount: f64,
}

#[derive(Debug)]
struct Character {
    name: String,
    resources: HashMap<String, Resource>,
    level: u32,
    experience: f64,
    abilities: Vec<Ability>,
}

#[derive(Debug)]
struct GameState {
    character: Character,
}

#[derive(Debug, Clone)]
enum AbilityType {
    Gather,
    // Add more ability types here as needed
}

#[derive(Debug, Clone)]
struct Ability {
    name: String,
    ability_type: AbilityType,
    level: u32,
}

impl Ability {
    fn new(name: &str, ability_type: AbilityType) -> Self {
        Ability {
            name: name.to_string(),
            ability_type,
            level: 1,
        }
    }
}

fn main() {
    let character = Character {
        name: "Thomoze Thounis".to_string(),
        level: 1,
        experience: 0.0,
        abilities: vec![
            Ability::new("Gather", AbilityType::Gather),
        ],
        resources: HashMap::new(),
    };
    
    let mut game_state = initialize_game_state(character);

    game_loop(&mut game_state);
}

fn game_loop(game_state: &mut GameState) {
    loop {
        let start_time = Instant::now();

        // Gather resources
        gather_resources(&mut game_state.character.resources, &game_state.character.abilities);

        // Gain experience and level up
        gain_experience(&mut game_state.character, 10.0);

        println!("{:?}", game_state.character.resources);

        let elapsed_time = start_time.elapsed();
        let sleep_duration = Duration::from_secs(1).checked_sub(elapsed_time).unwrap_or_default();
        sleep(sleep_duration);
    }
}

fn initialize_game_state(character: Character) -> GameState {
    let mut initial_resources = HashMap::new();

    initial_resources.insert(
        String::from("food"),
        Resource {
            name: String::from("food"),
            amount: 0.0,
        },
    );
    initial_resources.insert(
        String::from("ore"),
        Resource {
            name: String::from("ore"),
            amount: 0.0,
        },
    );
    initial_resources.insert(
        String::from("wood"),
        Resource {
            name: String::from("wood"),
            amount: 0.0,
        },
    );

    GameState {
        character: character,
    };
}

fn gather_resources(resources: &mut HashMap<String, Resource>, abilities: &[Ability]) {
    // Apply the Gather ability bonus
    let gather_bonus = abilities
        .iter()
        .filter(|ability| matches!(ability.ability_type, AbilityType::Gather))
        .map(|ability| ability.level as f64)
        .sum::<f64>();

     for ability in gather_ability {
        if let Some(resource) = resources.get_mut(&ability.target_resource) {        
            let amount = 1.0 + gather_bonus;
            resource.amount += amount;
            println!("Gathered {:.1} {}.", amount, resource.name);
        }
    }
}

fn gain_experience(character: &mut Character, experience: f64) {
    character.experience += experience;
    let required_experience = (character.level as f64) * 100.0; // Simple leveling formula

    if character.experience >= required_experience {
        character.level += 1;
        character.experience -= required_experience;
        println!("Level up! {} is now level {}", character.name, character.level);
    }
}