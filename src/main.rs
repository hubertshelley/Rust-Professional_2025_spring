use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self};
use std::path::PathBuf;
use std::process::{exit, Command};
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Exercise {
    name: String,
    path: String,
    #[serde(rename = "type")]
    exercise_type: String,
    score: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExerciseConfig {
    easy: Vec<Exercise>,
    normal: Vec<Exercise>,
    hard: Vec<Exercise>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExerciseResult {
    name: String,
    result: bool,
    score: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Statistics {
    total_exercises: usize,
    total_successes: usize,
    total_failures: usize,
    total_score: i32,
    total_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Report {
    exercises: Vec<ExerciseResult>,
    statistics: Statistics,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a command: 'watch' or 'all'");
        exit(1);
    }

    let mode = &args[1];
    let start_time = Instant::now();

    // 加载 JSON 配置
    let config = match load_exercise_config("exercise_config.json") {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load config file: {}", e);
            exit(1);
        }
    };

    let mut report = Report {
        exercises: Vec::new(),
        statistics: Statistics {
            total_exercises: 0,
            total_successes: 0,
            total_failures: 0,
            total_score: 0,
            total_time: 0,
        },
    };

    evaluate_exercises_from_config(mode, config, &mut report);

    report.statistics.total_time = start_time.elapsed().as_secs();
    report.statistics.total_exercises =
        report.statistics.total_successes + report.statistics.total_failures;

    println!("\nSummary:");
    println!("Total exercises: {}", report.statistics.total_exercises);
    println!("Total successes: {}", report.statistics.total_successes);
    println!("Total failures: {}", report.statistics.total_failures);
    println!("Total score: {}", report.statistics.total_score);

    if let Err(e) = save_report_to_json("report.json", &report) {
        eprintln!("Error saving report: {}", e);
    }
}

fn load_exercise_config(file_path: &str) -> Result<ExerciseConfig, io::Error> {
    let file = File::open(file_path)?;
    let config: ExerciseConfig = serde_json::from_reader(file)?;
    Ok(config)
}

fn evaluate_exercises_from_config(mode: &str, config: ExerciseConfig, report: &mut Report) {
    let all_exercises = [config.easy, config.normal, config.hard].concat();

    for exercise in all_exercises {
        println!("\nEvaluating {}: {}", exercise.exercise_type, exercise.name);
        let result = evaluate_exercise(&exercise);

        let score = if result { exercise.score } else { 0 };

        report.exercises.push(ExerciseResult {
            name: exercise.name.clone(),
            result,
            score,
        });

        if result {
            report.statistics.total_successes += 1;
        } else {
            report.statistics.total_failures += 1;
        }

        report.statistics.total_score += score;

        if mode == "watch" && !ask_to_continue() {
            break;
        }
    }
}

fn evaluate_exercise(exercise: &Exercise) -> bool {
    let exercise_path = PathBuf::from(&format!("./exercises/{}", exercise.path));
    match exercise.exercise_type.as_str() {
        "single_file" => evaluate_single_file(&exercise_path),
        "cargo_project" => evaluate_cargo_project(&exercise_path),
        _ => {
            eprintln!("Unknown exercise type: {}", exercise.exercise_type);
            false
        }
    }
}

fn evaluate_single_file(file_path: &PathBuf) -> bool {
    let output = Command::new("rustc").arg(file_path).output();

    match output {
        Ok(out) => {
            if out.status.success() {
                println!("\x1b[32m{}: PASSED\x1b[0m", file_path.display());
                true
            } else {
                println!("\x1b[31m{}: FAILED\x1b[0m", file_path.display());
                false
            }
        }
        Err(_) => {
            eprintln!("Error executing rustc for {}", file_path.display());
            false
        }
    }
}

fn evaluate_cargo_project(proj_path: &PathBuf) -> bool {
    let build_success = run_cargo_command(proj_path, "build");
    let test_success = run_cargo_command(proj_path, "test");
    let clippy_success = run_cargo_command(proj_path, "clippy");

    let passed = build_success && test_success && clippy_success;

    if passed {
        println!("\x1b[32m{}: PASSED\x1b[0m", proj_path.display());
    } else {
        println!("\x1b[31m{}: FAILED\x1b[0m", proj_path.display());
    }

    passed
}

fn run_cargo_command(proj_path: &PathBuf, command: &str) -> bool {
    let output = Command::new("cargo")
        .arg(command)
        .current_dir(proj_path)
        .output();

    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

fn ask_to_continue() -> bool {
    let mut input = String::new();
    println!("\nPress any key to continue, or 'q' to quit.");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase() != "q"
}

fn save_report_to_json(file_name: &str, report: &Report) -> io::Result<()> {
    let file = File::create(file_name)?;
    serde_json::to_writer_pretty(file, report)?;
    Ok(())
}
