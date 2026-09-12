use clap::Parser;
use std::io::{self, Read};
use vyasa_lipi::Script;
use vyasa_patha::model::PathaMode;
use vyasa_patha::{
    format_pada_patha, generate_krama_in_script, generate_krama_patha, parse_pada_patha,
};

#[derive(Parser, Debug)]
#[command(
    name = "vyasa-patha",
    about = "Vedic & Classical Sanskrit recitation generator (Prakṛti and Vikṛti pāṭhas)",
    version
)]
struct Cli {
    /// Input Pada-pāṭha text. If omitted, reads from standard input (stdin).
    #[arg(value_name = "TEXT")]
    text: Option<String>,

    /// Recitation mode: 'krama' (default) or 'pada'.
    #[arg(short, long, default_value = "krama")]
    mode: String,

    /// Target script format (e.g. 'devanagari', 'telugu', 'kannada', 'iast', 'iso15919').
    #[arg(short, long, default_value = "devanagari")]
    to: String,

    /// Emit numbered steps line-by-line instead of continuous recitation format.
    #[arg(short, long)]
    steps: bool,
}

fn main() {
    let cli = Cli::parse();

    let input = match cli.text {
        Some(t) => t,
        None => {
            let mut buffer = String::new();
            if let Err(e) = io::stdin().read_to_string(&mut buffer) {
                eprintln!("Error reading from stdin: {}", e);
                std::process::exit(1);
            }
            buffer
        }
    };

    let target_script = match Script::from_name(&cli.to) {
        Some(s) => s,
        None => {
            eprintln!("Error: Unknown target script '{}'", cli.to);
            std::process::exit(1);
        }
    };

    let mode = match cli.mode.to_lowercase().as_str() {
        "krama" => PathaMode::Krama,
        "pada" => PathaMode::Pada,
        _ => {
            eprintln!(
                "Error: Unsupported mode '{}'. Available: 'krama', 'pada'",
                cli.mode
            );
            std::process::exit(1);
        }
    };

    let padas = parse_pada_patha(&input);

    match mode {
        PathaMode::Krama => {
            if cli.steps {
                let krama_steps = generate_krama_patha(&padas);
                for step in krama_steps {
                    let text = if target_script == Script::Devanagari {
                        step.text
                    } else {
                        vyasa_lipi::transliterate(&step.text, Script::Devanagari, target_script)
                    };

                    if let Some(sec) = step.second_index {
                        println!(
                            "{}. ({}-{}) {}",
                            step.step_number, step.first_index, sec, text
                        );
                    } else {
                        println!("{}. ({}) {}", step.step_number, step.first_index, text);
                    }
                }
            } else {
                let result = generate_krama_in_script(&input, target_script);
                println!("{}", result);
            }
        }
        PathaMode::Pada => {
            let result = format_pada_patha(&padas);
            let text = if target_script == Script::Devanagari {
                result
            } else {
                vyasa_lipi::transliterate(&result, Script::Devanagari, target_script)
            };
            println!("{}", text);
        }
        _ => {}
    }
}
