use std::time::{Duration, Instant};
use std::thread::sleep;
use std::collections::HashMap;

mod ui;

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
    quests: Vec<Quest>,
}

#[derive(Debug)]
struct GameState {
    character: Character,
}

impl Character {
    fn new(name: &str) -> Self {
        Character {
            name: name.to_string(),
            level: 1,
            experience: 0.0,
            abilities: vec![Ability::new("Gather", AbilityType::Gather)],
            resources: HashMap::new(),
            quests: Vec::new(),
        }
    }
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

#[derive(Debug, Clone)]
struct Quest {
    name: String,
    description: String,
    goal: f64,
    completed: bool,
    reward: f64,
}

impl Quest {
    fn new(name: &str, description: &str, goal: f64, reward: f64) -> Self {
        Quest {
            name: name.to_string(),
            description: description.to_string(),
            goal,
            completed: false,
            reward,
        }
    }
}

fn main() -> iced::Result {
    let character = create_character();
    let mut game_state = initialize_game_state(character);
    // Sample quest
    add_quest(&mut game_state.character, Quest::new("Gather Wood", "Gather 100 units of wood", 100.0, 50.0));

    // Start the Iced UI
    ui::run(&game_state.character)
}

fn game_loop(game_state: &mut GameState) {
    loop {
        let start_time = Instant::now();

        // Gather resources
        for ability in &game_state.character.abilities {
            if let AbilityType::Gather = ability.ability_type {
            let elapsed_time = start_time.elapsed();
            gather_resources(&mut game_state.character.resources, std::slice::from_ref(ability), elapsed_time);
        }
    }      

        // Gain experience and level up
        gain_experience(&mut game_state.character, 10.0);

        // Check for quest completion
        check_quests(&mut game_state.character);

        // Print everything
        for resource in game_state.character.resources.values() {
            println!("{}: {}", resource.name, resource.amount);
        } 
        println!(); // Add an empty line for better readability

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
    }
}

fn create_character() -> Character {
    println!("Enter your character's name:");
    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read the character's name");
    name = name.trim().to_string();

    Character::new(&name)
}

fn gather_resources(resources: &mut HashMap<String, Resource>, abilities: &[Ability], elapsed_time: Duration) {
    // Apply the Gather ability bonus
    let gather_bonus = abilities
        .iter()
        .filter(|ability| matches!(ability.ability_type, AbilityType::Gather))
        .map(|ability| ability.level as f64)
        .sum::<f64>();

    // Update the resources
    for resource in resources.values_mut() {
        resource.amount += gather_bonus * elapsed_time.as_secs_f64();
    }
}

fn gain_experience(character: &mut Character, experience: f64) {
    character.experience += experience;
    let required_experience = (character.level as f64) * 100.0; // Simple leveling formula

    if character.experience >= required_experience {
        character.level += 1;
        character.experience -= required_experience;
        println!("Level up! {} is now level {}", character.name, character.level);
        for ability in &character.abilities {
            println!("{}: Level {}", ability.name, ability.level);
        }
        println!(); // Add an empty line for better readability
    }
}

fn add_quest(character: &mut Character, quest: Quest) {
    character.quests.push(quest);
}

fn check_quests(character: &mut Character) {
    for quest in &mut character.quests {
        if !quest.completed {
            let mut all_requirements_met = true;
            for requirement in &quest.goal {
                if let Some(resource) = character.resources.get(&requirement.resource_name) {
                    if resource.amount < requirement.amount {
                        all_requirements_met = false;
                        break;
                    }
                } else {
                    all_requirements_met = false;
                    break;
                }
            }
            if all_requirements_met {
                quest.completed = true;
                println!("Quest completed: {}", quest.name);
            }
        }
    }
}
