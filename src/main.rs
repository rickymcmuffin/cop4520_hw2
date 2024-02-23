pub mod cupcake;
pub mod labrynth;
pub mod vase;

fn main() {
    let mut line = String::new();

    while line.trim() != "3" {
        println!(
            "Welcome to the Minasour Party! Enter a number to get started
             \n(1) Labrynth Game
             \n(2) Vase Game
             \n(3) Exit"
        );

        let _b1 = std::io::stdin().read_line(&mut line).unwrap();
        while line.trim() != "1" && line.trim() != "2" && line.trim() != "3" {
            println!("{line}");
            println!("Value must be 1, 2, or 3. Try again: ");
            line = "".to_string();
            let _b1 = std::io::stdin().read_line(&mut line).unwrap();
        }

        if line.trim() == "1" {
            labrynth::labrynth_game();
        }
        if line.trim() == "2" {
            vase::vase_sim();
        }
    }
}
