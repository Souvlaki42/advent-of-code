use std::{env, fs};

pub struct Solution {
    pub year: u32,
    pub day: u32,
    pub part: u32,
    pub run: fn(String),
}

inventory::collect!(Solution);

// To add a year, copy a year's folder, rename it add a mod statement for that year below.
mod year2024;
mod year2025;

// Just run `cargo run -- <year> <day> <part>`.

// To add a day, copy a day's folder, rename it, modify the year's `mod.rs` file and change the `inventory` block inside `part1.rs` and `part2.rs` files to register them.

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: cargo run -- <year> <day> <part>");
        return;
    }

    let year: u32 = args[1].parse().expect("Invalid year");
    let day: u32 = args[2].parse().expect("Invalid day");
    let part: u32 = args[3].parse().expect("Invalid part");

    if let Some(solution) = inventory::iter::<Solution>
        .into_iter()
        .find(|s| s.year == year && s.day == day && s.part == part)
    {
        println!("🎄 Running Year {} Day {} Part {}...", year, day, part);
        let Ok(input_file) = fs::read_to_string(format!("year{year}/day{day:02}/input.txt")) else {
            eprintln!("Missing input file for {year}'s day {day:02} part {part}");
            return;
        };

        let start_time = std::time::Instant::now();
        (solution.run)(input_file);
        let elapsed_time = start_time.elapsed();
        println!(
            "🎁 Finished in {}.{:03}s",
            elapsed_time.as_secs(),
            elapsed_time.subsec_millis()
        );
    } else {
        eprintln!("❌ Solution not found. Did you register it?");
    }
}
