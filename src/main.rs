use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::process::{Command, exit};
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Exercise {
    name: String,
    path: String,
    #[serde(rename = "type")]
    exercise_type: String,  // 解析 JSON 中的 `type` 字段
    score: i32, // 新增：每道题目的分值
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
    score: i32, // 新增：每道题目的分数
}

#[derive(Serialize, Deserialize, Debug)]
struct Statistics {
    total_exercises: usize,
    total_successes: usize,
    total_failures: usize,
    total_score: i32, // 新增：总分
    total_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Report {
    exercises: Vec<ExerciseResult>,
    statistics: Statistics,
}

// **主程序**
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
            total_score: 0, // 初始化总分
            total_time: 0,
        },
    };

    // **开始评测**
    evaluate_exercises_from_config(mode, config, &mut report);

    // **计算总时间**
    report.statistics.total_time = start_time.elapsed().as_secs();
    report.statistics.total_exercises = report.statistics.total_successes + report.statistics.total_failures;

    // **输出结果**
    println!("\nSummary:");
    println!("Total exercises: {}", report.statistics.total_exercises);
    println!("Total successes: {}", report.statistics.total_successes);
    println!("Total failures: {}", report.statistics.total_failures);
    println!("Total score: {}", report.statistics.total_score); // 输出总分

    // **保存报告**
    if let Err(e) = save_report_to_json("report.json", &report) {
        eprintln!("Error saving report: {}", e);
    }
}

// **加载 JSON 配置**
fn load_exercise_config(file_path: &str) -> Result<ExerciseConfig, io::Error> {
    let file = File::open(file_path)?;
    let config: ExerciseConfig = serde_json::from_reader(file)?;
    Ok(config)
}

// **根据 JSON 配置评测所有习题**
fn evaluate_exercises_from_config(mode: &str, config: ExerciseConfig, report: &mut Report) {
    let all_exercises = [config.easy, config.normal, config.hard].concat();
    
    for exercise in all_exercises {
        println!("\nEvaluating {}: {}", exercise.exercise_type, exercise.name);
        let result = evaluate_exercise(&exercise);

        // 根据习题评测结果，更新报告
        let score = if result { exercise.score } else { 0 };

        report.exercises.push(ExerciseResult {
            name: exercise.name.clone(),
            result,
            score, // 保存每道题目的分数
        });

        if result {
            report.statistics.total_successes += 1;
        } else {
            report.statistics.total_failures += 1;
        }

        // 累加分数
        report.statistics.total_score += score;

        if mode == "watch" && !ask_to_continue() {
            break;
        }
    }
}

// **评测单个习题**
fn evaluate_exercise(exercise: &Exercise) -> bool {
    let exercise_path = PathBuf::from(&exercise.path);
    match exercise.exercise_type.as_str() {
        "single_file" => evaluate_single_file(&exercise_path),
        "cargo_project" => evaluate_cargo_project(&exercise_path),
        _ => {
            eprintln!("Unknown exercise type: {}", exercise.exercise_type);
            false
        }
    }
}

// **评测单文件 Rust 习题**
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

// **评测 Cargo 项目**
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

// **运行 Cargo 命令**
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

// **用户确认是否继续**
fn ask_to_continue() -> bool {
    let mut input = String::new();
    println!("\nPress any key to continue, or 'q' to quit.");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase() != "q"
}

// **保存评测报告**
fn save_report_to_json(file_name: &str, report: &Report) -> io::Result<()> {
    let file = File::create(file_name)?;
    serde_json::to_writer_pretty(file, report)?;
    Ok(())
}
