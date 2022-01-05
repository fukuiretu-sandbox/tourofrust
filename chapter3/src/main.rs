#![allow(dead_code)]

// struct SeaCreature {
//     animal_type: String,
//     name: String,
//     arms: i32,
//     legs: i32,
//     weapon: String,
// }

struct Location(i32, i32);

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam,
}

enum PositionType {
    Acidic,
    Painful,
    Lethal,
}

enum Size {
    Big,
    Small,
}

enum Weapon {
    Claw(i32, Size),
    None,
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

fn main() {
    // let ferris = SeaCreature {
    //     animal_type: String::from("crab"),
    //     name: String::from("Ferris"),
    //     arms: 2,
    //     legs: 4,
    //     weapon: String::from("claw"),
    // };
    // let sarah = SeaCreature {
    //     animal_type: String::from("octopus"),
    //     name: String::from("Sarah"),
    //     arms: 8,
    //     legs: 0,
    //     weapon: String::from("none"),
    // };
    // println!(
    //     "{} is a {}. They have {} arms, {} legs, and a {} weapon",
    //     ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon
    // );
    // println!(
    //     "{} is a {}. They have {} arms, and {} legs. They have no weapon..",
    //     sarah.name, sarah.animal_type, sarah.arms, sarah.legs
    // );

    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);

    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::None,
    };
    match ferris.species {
        Species::Crab => println!("{} is a crab", ferris.name),
        Species::Octopus => println!("{} is a octpus", ferris.name),
        _ => println!("ohter!!"),
    }
}
