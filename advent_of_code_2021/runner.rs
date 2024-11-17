use std::env;
use std::process::Command;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!(
            "Usage ./runner day.part
            \nexample command `./runner 4.2` will run day 4 part 2"
        );
        std::process::exit(1);
    }

    let source_folder = format!("day_{}", &args[1]);
    let main_file = format!("./{}/main.rs", &source_folder);
    let temp_executable = format!("./{}/temp_main", &source_folder);

    if !std::path::Path::new(&main_file).exists() {
        eprintln!("File {} does not exists", main_file);
        std::process::exit(1);
    }

    let output = Command::new("rustc")
        .arg(&main_file)
        .arg("-o")
        .arg(&temp_executable)
        .output();

    if let Err(e) = output {
        eprintln!("Failed to compile {}", e);
        std::process::exit(1);
    }

    let start = Instant::now();
    let run_time_output = Command::new(&temp_executable).output();
    let duration = start.elapsed();
    match run_time_output {
        Ok(output) => {
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }

            if !output.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }

        Err(e) => {
            eprintln!("Error during run time: {}", e);
            std::process::exit(1);
        }
    }

    println!("Running time: {:.2?}", duration);
    let _ = std::fs::remove_file(temp_executable);
}
