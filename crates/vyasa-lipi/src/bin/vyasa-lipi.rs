use std::io::{self, Read};
use vyasa_lipi::{
    detect_script, transliterate_with_options, AccentMode, Script, TransliterateOptions,
};

fn print_usage() {
    eprintln!(
        "Usage: vyasa-lipi --to <script> [--from <script>] [--accent <scholarly|unicode>] [text]\n\
         Supported scripts: devanagari, telugu, kannada, iso15919, iast"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let mut from_script: Option<Script> = None;
    let mut to_script: Option<Script> = None;
    let mut accent_mode = AccentMode::ScholarlyRoman;
    let mut text_parts = Vec::new();

    let mut idx = 1;
    while idx < args.len() {
        match args[idx].as_str() {
            "--to" => {
                if idx + 1 < args.len() {
                    to_script = Script::from_name(&args[idx + 1]);
                    idx += 2;
                } else {
                    eprintln!("Error: --to requires a script name");
                    std::process::exit(1);
                }
            }
            "--from" => {
                if idx + 1 < args.len() {
                    from_script = Script::from_name(&args[idx + 1]);
                    idx += 2;
                } else {
                    eprintln!("Error: --from requires a script name");
                    std::process::exit(1);
                }
            }
            "--accent" => {
                if idx + 1 < args.len() {
                    match args[idx + 1].to_lowercase().as_str() {
                        "unicode" => accent_mode = AccentMode::PreserveUnicode,
                        "scholarly" => accent_mode = AccentMode::ScholarlyRoman,
                        _ => {
                            eprintln!("Unknown accent mode, valid: scholarly, unicode");
                            std::process::exit(1);
                        }
                    }
                    idx += 2;
                } else {
                    eprintln!("Error: --accent requires scholarly or unicode");
                    std::process::exit(1);
                }
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            arg => {
                text_parts.push(arg.to_string());
                idx += 1;
            }
        }
    }

    let to = match to_script {
        Some(s) => s,
        None => {
            eprintln!("Error: --to <script> is required");
            print_usage();
            std::process::exit(1);
        }
    };

    let input_text = if !text_parts.is_empty() {
        text_parts.join(" ")
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    };

    let from = match from_script {
        Some(s) => s,
        None => match detect_script(&input_text) {
            Some(detected) => detected,
            None => {
                eprintln!(
                    "Could not auto-detect source script. Please specify with --from <script>"
                );
                std::process::exit(1);
            }
        },
    };

    let options = TransliterateOptions { accent_mode };
    let result = transliterate_with_options(&input_text, from, to, &options);
    println!("{}", result);
}
