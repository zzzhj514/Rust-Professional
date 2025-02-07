use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, exit};
use std::time::Instant;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Exercise {
    name: String,
    path: String,
    #[serde(rename = "type")]
    exercise_type: String,  
    score: i32,  // Added: Each exercise score
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
    score: i32,  // Store score for each exercise
}

#[derive(Serialize, Deserialize, Debug)]
struct Statistics {
    total_exercises: usize,
    total_successes: usize,
    total_failures: usize,
    total_score: i32,  // Total score for the assessment
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

    // Load the exercise config
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
            total_score: 0, // Initialize total score to 0
            total_time: 0,
        },
    };

    // Evaluate exercises from config
    evaluate_exercises_from_config(mode, config, &mut report);

    // Calculate total time
    report.statistics.total_time = start_time.elapsed().as_secs();
    report.statistics.total_exercises = report.statistics.total_successes + report.statistics.total_failures;

    // Output summary
    println!("\nSummary:");
    println!("Total exercises: {}", report.statistics.total_exercises);
    println!("Total successes: {}", report.statistics.total_successes);
    println!("Total failures: {}", report.statistics.total_failures);
    println!("Total score: {}", report.statistics.total_score);  // Output the total score

    // Save the report to a JSON file
    if let Err(e) = save_report_to_json("report.json", &report) {
        eprintln!("Error saving report: {}", e);
    }
}

// Load exercise configuration from JSON
fn load_exercise_config(file_path: &str) -> Result<ExerciseConfig, io::Error> {
    let file = File::open(file_path)?;
    let config: ExerciseConfig = serde_json::from_reader(file)?;
    Ok(config)
}

// Evaluate all exercises from the configuration
fn evaluate_exercises_from_config(mode: &str, config: ExerciseConfig, report: &mut Report) {
    let all_exercises = [config.easy, config.normal, config.hard].concat();
    
    for exercise in all_exercises {
        println!("\nEvaluating {}: {}", exercise.exercise_type, exercise.name);
        let result = evaluate_exercise(&exercise);

        // Calculate score based on result
        let score = if result { exercise.score } else { 0 };

        // Add result to the report
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

        // Add score to total score
        report.statistics.total_score += score;

        if mode == "watch" && !ask_to_continue() {
            break;
        }
    }
}

// Evaluate a single exercise
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

// Evaluate a single file Rust exercise
fn evaluate_single_file(file_path: &PathBuf) -> bool {
    let output = Command::new("rustc")
        .arg(file_path)
        .output();

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

// Evaluate a cargo project
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

    // Clean up the target directory after evaluation
    clean_target_directory(proj_path);

    passed
}

// Run a cargo command (build, test, clippy)
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

// Clean up the target directory after evaluating a cargo project
fn clean_target_directory(proj_path: &PathBuf) {
    let target_dir = proj_path.join("target");

    if target_dir.exists() {
        if let Err(e) = fs::remove_dir_all(&target_dir) {
            eprintln!("Failed to clean up target directory: {}", e);
        } else {
            println!("Successfully cleaned up target directory in: {}", proj_path.display());
        }
    }
}

// Ask the user whether to continue after each evaluation
fn ask_to_continue() -> bool {
    let mut input = String::new();
    println!("\nPress any key to continue, or 'q' to quit.");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase() != "q"
}

// Save the report to a JSON file
fn save_report_to_json(file_name: &str, report: &Report) -> io::Result<()> {
    let file = File::create(file_name)?;
    serde_json::to_writer_pretty(file, report)?;
    Ok(())
}
