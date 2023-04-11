use std::time::Duration;
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
}

#[derive(Debug)]
struct GameState {
    character: Character,
}

fn main() {
    let mut game_state = initialize_game_state();

    loop {
        // Game logic goes here
        // Gather wood
        gather_resource(&mut game_state.character, "wood", 1.0);

        // Gain experience and level up
        gain_experience(&mut game_state.character, 10.0);

        // Print the current wood amount
        println!(
            "Wood: {}",
            game_state.character.resources.get("wood").unwrap().amount
        );

        sleep(Duration::from_millis(1000)); // Adjust the duration to control the game loop's speed
    }
}

fn initialize_game_state() -> GameState {
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

    let character = Character {
        name: "Thomoze Thounis".to_string(),
        resources: resources,
        level: 1,
        experience: 0.0,
    };

    GameState {
        character: Character {
            level: 1,
            resources: initial_resources,
        },
    }
}

fn gather_resource(character: &mut Character, resource_name: &str, amount: f64) {
    if let Some(resource) = character.resources.get_mut(resource_name) {
        resource.amount += amount;
    } else {
        println!("Resource not found: {}", resource_name);
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