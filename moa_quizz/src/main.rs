use angle_calculation::{AngleDropDistance, AngleType};
use clap::Parser;
use moa_quizz::Target;
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

#[derive(clap::ValueEnum, Clone, Debug, Copy)]
enum Mode {
    Angle,
    Drop,
    Random,
    Target,
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

fn get_random_drop(signed: bool) -> f64 {
    if signed {
        let drops: [f64; 9] = [-4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0];
        get_random_element(&drops)
    } else {
        let drops: [f64; 10] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        get_random_element(&drops)
    }
}

fn check_answer(prompt: &str, correct_value: f64, tolerance: f64) -> bool {
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
    let drop: f64 =
        match unit {
            Unit::Moa => AngleDropDistance::from_angle_distance(AngleType::MOA(moa), distance)
                .get_drop_in_cm(),
            Unit::Mrad => AngleDropDistance::from_angle_distance(AngleType::MIL(moa), distance)
                .get_drop_in_cm(),
        };
    println!("Distance: {} meters", distance);
    println!("MOA: {}", moa);
    check_answer("Find in cm drop: ", drop, tolerance)
}

fn distance_cm(tolerance: f64, unit: Unit) -> bool {
    let drop: f64 = get_random_drop(false);
    let distance: f64 = get_random_distance();
    let angle_raw = AngleDropDistance::from_drop_distance(drop / 100.0, distance).get_angle();
    let angle: f64 = match unit {
        Unit::Moa => angle_raw.get_moa(),
        Unit::Mrad => angle_raw.get_mrad(),
    };
    println!("Distance: {} meters", distance);
    println!("Drop: {} cm", drop);
    check_answer("Find MOA: ", angle, tolerance)
}

fn distance_target(tolerance: f64, unit: Unit) -> bool {
    // 1) Generate random distance and drops (in cm).
    let distance = get_random_distance();
    let (x, y) = (get_random_drop(true), get_random_drop(true));

    println!("Distance: {} meters", distance);
    println!(
        "Find the impact point of a shot with a drop of x {} cm and y {} cm",
        x, y
    );

    // 2) Create and display the target with a 9x9 grid offset of 4.0 units.
    let target = Target::new(x + 4.0, y + 4.0);
    println!("{}", target);

    // 3) Convert drop (cm) to an AngleType for both x and y.
    //    We divide cm by 100 to get meters.
    let angle_x = AngleDropDistance::from_drop_distance(x / 100.0, distance).get_angle();
    let angle_y = AngleDropDistance::from_drop_distance(y / 100.0, distance).get_angle();

    // 4) Depending on the desired unit (MOA or MRAD), retrieve the angles.
    let (expected_x, expected_y) = match unit {
        Unit::Moa => (angle_x.get_moa(), angle_y.get_moa()),
        Unit::Mrad => (angle_x.get_mrad(), angle_y.get_mrad()),
    };

    // 5) Check the user’s answers (negative because we’re adjusting POA).
    let ok_x = check_answer("Find x: ", -expected_x, tolerance);
    let ok_y = check_answer("Find y: ", -expected_y, tolerance);

    ok_x && ok_y
}

struct QuizzOptions {
    mode: Mode,
    unit: Unit,
}

fn quizz(quizzopt: QuizzOptions, tolerance: f64, number_of_questions: u32) {
    let mut score: u16 = 0;
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
            Mode::Target => {
                if distance_target(tolerance, quizzopt.unit) {
                    score += 1;
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
