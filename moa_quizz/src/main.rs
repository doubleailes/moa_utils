use angle_calculation::{MOADD, MRADDD};
use clap::Parser;
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

#[derive(clap::ValueEnum, Clone, Debug, Copy)]
enum Mode {
    Angle,
    Drop,
    Random,
}

#[derive(clap::ValueEnum, Clone, Debug, Copy)]
enum Unit {
    Moa,
    Mrad,
}

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long, default_value = "DistanceMOA")]
    mode: Mode,
    #[clap(short, long, default_value = "0.05")]
    tolerance: f64,
    #[clap(short, long, default_value = "10")]
    number_of_questions: u32,
    #[clap(short, long, default_value = "Moa")]
    unit: Unit,
}

fn get_random_element<T: Copy>(elements: &[T]) -> T {
    let mut rng: ThreadRng = thread_rng();
    *elements.choose(&mut rng).unwrap_or(&elements[0])
}

fn get_random_distance() -> f64 {
    let distances: [f64; 7] = [25.0, 50.0, 100.0, 200.0, 400.0, 800.0, 1000.0];
    get_random_element(&distances)
}

fn get_random_moa() -> f64 {
    let moas: [f64; 4] = [0.2, 0.5, 1.0, 2.0];
    get_random_element(&moas)
}

fn get_random_drop() -> f64 {
    let drops: [f64; 4] = [0.5, 1.0, 2.0, 5.0];
    get_random_element(&drops)
}

fn check_answer<T: std::str::FromStr + std::fmt::Display>(
    prompt: &str,
    correct_value: f64,
    tolerance: f64,
) -> bool {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_value: f64 = input.trim().parse().unwrap();
    if correct_value * (1.0 - tolerance) < input_value
        && input_value < correct_value * (1.0 + tolerance)
    {
        println!("Correct! {}", correct_value);
        true
    } else {
        println!("Incorrect! Correct answer: {}", correct_value);
        false
    }
}

fn distance_moa(tolerance: f64, unit: Unit) -> bool {
    let moa: f64 = get_random_moa();
    let distance: f64 = get_random_distance();
    match unit {
        Unit::Moa => {
            let drop: f64 = MOADD::new_from_moa_distance(moa, distance).get_drop_in_cm();
            println!("Distance: {} meters", distance);
            println!("MOA: {}", moa);
            check_answer::<f64>("Find in cm drop: ", drop, tolerance)
        }
        Unit::Mrad => {
            let drop: f64 = MRADDD::new_from_mrad_distance(moa, distance).get_drop_in_cm();
            println!("Distance: {} meters", distance);
            println!("MRAD: {}", moa);
            check_answer::<f64>("Find in cm drop: ", drop, tolerance)
        }
    }
}

fn distance_cm(tolerance: f64, unit: Unit) -> bool {
    let drop: f64 = get_random_drop();
    let distance: f64 = get_random_distance();
    match unit {
        Unit::Moa => {
            let moa: f64 = MOADD::new_from_drop_distance(drop / 100.0, distance).get_moa();
            println!("Distance: {} meters", distance);
            println!("Drop: {} cm", drop);
            check_answer::<f64>("Find MOA: ", moa, tolerance)
        }
        Unit::Mrad => {
            let moa: f64 = MRADDD::new_from_drop_distance(drop / 100.0, distance).get_mrad();
            println!("Distance: {} meters", distance);
            println!("Drop: {} cm", drop);
            check_answer::<f64>("Find MRAD: ", moa, tolerance)
        }
    }
}

struct QuizzOptions {
    mode: Mode,
    unit: Unit,
}

fn quizz(quizzopt: QuizzOptions, tolerance: f64, number_of_questions: u32) {
    let mut score = 0;
    for _ in 0..number_of_questions {
        println!("== Question {}/{} ==", score + 1, number_of_questions);
        match quizzopt.mode {
            Mode::Angle => {
                if distance_moa(tolerance, quizzopt.unit) {
                    score += 1;
                }
            }
            Mode::Drop => {
                if distance_cm(tolerance, quizzopt.unit) {
                    score += 1;
                }
            }
            Mode::Random => {
                let modes: [Mode; 2] = [Mode::Angle, Mode::Drop];
                let mode: Mode = get_random_element(&modes);
                match mode {
                    Mode::Angle => {
                        if distance_moa(tolerance, quizzopt.unit) {
                            score += 1;
                        }
                    }
                    Mode::Drop => {
                        if distance_cm(tolerance, quizzopt.unit) {
                            score += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    println!("Score: {}/{}", score, number_of_questions);
}

fn main() {
    let args: Args = Args::parse();
    let quizzopt = QuizzOptions {
        mode: args.mode,
        unit: args.unit,
    };
    quizz(quizzopt, args.tolerance, args.number_of_questions);
}
