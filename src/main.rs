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
    level: u32,
    resources: HashMap<String, Resource>,
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

        // Print the current wood amount
        println!(
            "Wood: {}",
            game_state.character.resources.get("wood").unwrap().amount
        );

        sleep(Duration::from_millis(100)); // Adjust the duration to control the game loop's speed
    }
}

fn gather_resource(character: &mut Character, resource_name: &str, amount: f64) {
    if let Some(resource) = character.resources.get_mut(resource_name) {
        resource.amount += amount;
    } else {
        println!("Resource not found: {}", resource_name);
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

    GameState {
        character: Character {
            level: 1,
            resources: initial_resources,
        },
    }
}