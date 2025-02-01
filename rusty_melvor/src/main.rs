fn main() {
    let skill = match std::env::args().nth(1) {
        Some(skill) => skill,
        None => {
            println!("Please provide a skill to train");
            return;
        }
    };

    println!("Hello, world!");
    println!("Training skill: {}", skill);
}
