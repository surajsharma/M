use std::env;
use std::path::Path;
use std::process::Command;
use std::process::ExitCode;

struct Flags {
    lex: bool,
    parse: bool,
    codegen: bool,
    help: bool,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let mut flags = Flags {
        lex: false,
        parse: false,
        codegen: false,
        help: false,
    };

    if args.len() < 2 {
        println!(
            "Usage: {} <file_name> [--lex] [--parse] [--codegen]",
            args[0]
        );

        return ExitCode::from(1);
    }

    // Process command line arguments
    for arg in &args[1..] {
        match arg.as_str() {
            "--lex" | "-L" => flags.lex = true,
            "--parse" | "-P" => flags.parse = true,
            "--codegen" | "-C" => flags.codegen = true,
            "--help" | "-h" => flags.help = true,
            _ => {}
        }
    }

    if flags.help {
        print_usage();
        return ExitCode::from(0);
    }

    let file_name = &args[1];
    let path = Path::new(file_name);

    if !path.is_file() {
        println!("{} is invalid file! see usage with --help or -h", file_name);
        return ExitCode::from(1);
    }

    let processed_file = add_processed_to_filename(file_name);

    // Execute the gcc command
    let output = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(file_name)
        .arg("-o")
        .arg(&processed_file)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("gcc preprocessor ran successfully.\n");
                if flags.lex {
                    println!("lexer will run\n");
                }

                if flags.parse {
                    println!("parser will run\n");
                }

                if flags.codegen {
                    println!("codegen will run\n");
                }
                ExitCode::from(0)
            } else {
                println!("gcc preprocessor failed to execute.\n");
                println!("Error: {}", String::from_utf8_lossy(&output.stderr));
                ExitCode::from(1)
            }
        }
        Err(e) => {
            println!("Failed to execute gcc preprocessor: {}\n", e);
            ExitCode::from(1)
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

fn add_processed_to_filename(file_name: &str) -> String {
    let path = Path::new(file_name);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    format!("{}_preprocessed.{}", stem, "i")
}

fn print_usage() {
    println!("Usage: program <input_file> [options]");
    println!("Options:");
    println!("  --lex, -L       Perform lexical analysis");
    println!("  --parse, -P     Perform parsing");
    println!("  --codegen, -C   Perform code generation");
    println!("  --help, -h      Display this help message");
}
