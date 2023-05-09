use colored::*;
//need ' colored = "2.0" ' under dependencies in the Cargo.toml

fn main() {
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}",
        "H".red(),
        "e".yellow().bold(),
        "l".yellow(),
        "l".green(),
        "o,".cyan(),
        " W".blue(),
        "o".magenta(),
        "r".purple(),
        "l".red(),
        "d".yellow().bold(),
        "!".green()
    );
}
