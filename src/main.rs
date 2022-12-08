use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

fn main() {
    match env::args().nth(1).unwrap_or_default().as_str() {
        "01" | "1" => day_01::run(),
        "02" | "2" => day_02::run(),
        "03" | "3" => day_03::run(),
        "04" | "4" => day_04::run(),
        "05" | "5" => day_05::run(),
        "06" | "6" => day_06::run(),
        "07" | "7" => day_07::run(),
        "08" | "8" => day_08::run(),
        _ => println!("Invalid day"),
    }
}
