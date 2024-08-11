use std::env;
use std::path::Path;
use std::process::Command;
use std::process::ExitCode;
use std::process::Output;

use colored::*;

struct Flags {
    lex: bool,
    parse: bool,
    codegen: bool,
    help: bool,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match process_args(&args) {
        Err(err) => {
            println!("\n{} {}", "Error: ".red().bold(), err);
            return ExitCode::FAILURE;
        }

        Ok(flags) => {
            let file = &args[1];
            let processed_file = get_processed_filename(file);

            if flags.help {
                print_usage();
                return ExitCode::SUCCESS;
            }

            // Execute the gcc preprocessor command
            match run_preprocessor(&file, &processed_file) {
                Ok(output) => {
                    if output.status.success() {
                        println!(
                            "\n{} gcc preprocessor ran successfully.\n",
                            "Success:".green().bold()
                        );

                        if flags.lex {
                            run_lexer();
                        }

                        if flags.parse {
                            run_parser();
                        }

                        if flags.codegen {
                            run_codegen();
                        }

                        return ExitCode::SUCCESS;
                    } else {
                        println!(
                            "\n{} {}",
                            "Error:".red().bold(),
                            String::from_utf8_lossy(&output.stderr)
                        );
                        return ExitCode::FAILURE;
                    }
                }
                Err(e) => {
                    println!(
                        "\n{} Failed to execute gcc preprocessor: {}\n{}\n",
                        "Error: ".red().bold(),
                        e,
                        "Is gcc installed and in $PATH?".yellow().bold()
                    );
                    return ExitCode::FAILURE;
                }
            }
        }
    }
}

/* STEP 2 - Compile the preprocessed source file and output an assembly file with a .s extension.
    You’ll have to stub out this step, since you haven’t written your compiler yet.
    Delete the preprocessed file when you’re done with it.
*/

/* STEP 3 - Assemble and link the assembly file to produce an executable, using
    gcc ASSEMBLY_FILE -o OUTPUT_FILE
    Delete the assembly file when you’re done with it.
*/

fn run_lexer() {
    println!("✅ lexer will run");
}

fn run_parser() {
    println!("✅ parser will run");
}

fn run_codegen() {
    println!("✅ codegen will run");
}

fn run_preprocessor(file: &str, processed_file: &str) -> Result<Output, std::io::Error> {
    return Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(file)
        .arg("-o")
        .arg(processed_file)
        .output();
}

fn process_args(args: &[String]) -> Result<Flags, String> {
    let mut flags = Flags {
        lex: false,
        parse: false,
        codegen: false,
        help: false,
    };

    if args.len() < 2 {
        return Err(format!(
            "{} {} <file_name> [--lex] [--parse] [--codegen]",
            "Usage: ".green().bold(),
            args[0]
        ));
    }

    let mut i = 1; // Skip the program name and file name

    while i < args.len() {
        match args[i].as_str() {
            "--lex" | "-L" => flags.lex = true,
            "--parse" | "-P" => flags.parse = true,
            "--codegen" | "-C" => flags.codegen = true,
            "--help" | "-h" => flags.help = true,
            _ => {
                if i == 1 {
                    let file = &args[i];
                    let path = Path::new(file);

                    if !path.is_file() && !flags.help {
                        return Err(format!(
                            "Invalid file: '{}'. Try {} or {} for more information.",
                            file,
                            "--help".cyan().bold(),
                            "-h".cyan().bold()
                        ));
                    }
                } else {
                    return Err(format!(
                        "Invalid argument: '{}'. Try {} or {} for more information.",
                        args[i],
                        "--help".cyan().bold(),
                        "-h".cyan().bold()
                    ));
                }
            }
        }
        i += 1;
    }

    Ok(flags)
}

fn get_processed_filename(file_name: &str) -> String {
    let path = Path::new(file_name);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    format!("{}_preprocessed.{}", stem, "i")
}

fn print_usage() -> ExitCode {
    println!(
        "\n{} program <input_file> [options]",
        "Usage: ".green().bold()
    );
    println!("{}", "Options:".cyan().bold());
    println!(
        "{},\t\t{}\tPerform lexical analysis",
        "--lex".cyan().bold(),
        "-L".cyan()
    );
    println!(
        "{},\t{}\tPerform parsing",
        "--parse".cyan().bold(),
        "-P".cyan()
    );
    println!(
        "{},\t{}\tPerform code generation",
        "--codegen".cyan().bold(),
        "-C".cyan()
    );
    println!(
        "{},\t\t{}\tDisplay this help message\n",
        "--help".cyan().bold(),
        "-h".cyan()
    );
    return ExitCode::SUCCESS;
}
