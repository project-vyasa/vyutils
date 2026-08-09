use clap::{CommandFactory, Parser, Subcommand};
use mpt::{
    add_part, find_part, merge, parse, remove_part, render_command_tree, to_string, Document,
    Format, HeaderBlock, InsertPosition, Part,
};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "mpt", about = "Multi-part-text container tools (RFC-0001)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse and validate an mpt document.
    Validate {
        /// Path to the mpt file.
        path: PathBuf,
    },
    /// Parse and print canonical serialized form to stdout.
    Canonicalize {
        /// Path to the mpt file.
        path: PathBuf,
    },
    /// Merge mpt documents (parts concatenated in order).
    Merge {
        /// Input mpt files (at least one).
        #[arg(required = true)]
        inputs: Vec<PathBuf>,
        /// Write merged document to this path (default: stdout).
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Print a concise tree of all commands and options.
    Tree,
    /// Part operations on an mpt document.
    Part {
        #[command(subcommand)]
        command: PartCommands,
    },
}

#[derive(Subcommand)]
enum PartCommands {
    /// Extract a part body (or header / full part document).
    Extract {
        /// Path to the mpt file.
        path: PathBuf,
        /// Part id to extract.
        id: String,
        /// Write extracted content to this path (default: stdout).
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Extract the part header payload instead of the body.
        #[arg(long, conflicts_with = "full")]
        header: bool,
        /// Extract as a standalone single-part mpt document.
        #[arg(long, conflicts_with = "header")]
        full: bool,
    },
    /// Add a part to a document.
    Add {
        /// Path to the mpt file.
        path: PathBuf,
        /// Part id for the new part.
        id: String,
        /// Body format (default: text).
        #[arg(long, default_value = "text")]
        format: String,
        /// Read body from this file (default: stdin).
        #[arg(long)]
        body_file: Option<PathBuf>,
        /// Part-level header payload file (toml by default).
        #[arg(long)]
        header_file: Option<PathBuf>,
        /// Header payload format when --header-file is set (default: toml).
        #[arg(long, default_value = "toml")]
        header_format: String,
        /// Insert before this part id.
        #[arg(long, conflicts_with = "after")]
        before: Option<String>,
        /// Insert after this part id.
        #[arg(long, conflicts_with = "before")]
        after: Option<String>,
        /// Write updated document to this path (default: stdout).
        #[arg(short, long, conflicts_with = "in_place")]
        output: Option<PathBuf>,
        /// Rewrite the input file in place.
        #[arg(long, conflicts_with = "output")]
        in_place: bool,
    },
    /// Remove a part from a document.
    Remove {
        /// Path to the mpt file.
        path: PathBuf,
        /// Part id to remove.
        id: String,
        /// Write updated document to this path (default: stdout).
        #[arg(short, long, conflicts_with = "in_place")]
        output: Option<PathBuf>,
        /// Rewrite the input file in place.
        #[arg(long, conflicts_with = "output")]
        in_place: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    if matches!(cli.command, Commands::Tree) {
        print!("{}", render_command_tree(&Cli::command()));
        return ExitCode::SUCCESS;
    }
    let result = match cli.command {
        Commands::Tree => unreachable!("handled above"),
        Commands::Validate { path } => run_validate(&path).map(|_| None),
        Commands::Canonicalize { path } => run_canonicalize(&path).map(Some),
        Commands::Merge { inputs, output } => run_merge(&inputs, output.as_ref()).map(Some),
        Commands::Part { command } => match command {
            PartCommands::Extract {
                path,
                id,
                output,
                header,
                full,
            } => run_part_extract(&path, &id, output.as_ref(), header, full).map(Some),
            PartCommands::Add {
                path,
                id,
                format,
                body_file,
                header_file,
                header_format,
                before,
                after,
                output,
                in_place,
            } => run_part_add(PartAddOptions {
                path: &path,
                id: &id,
                format_name: &format,
                body_file: body_file.as_ref(),
                header_file: header_file.as_ref(),
                header_format_name: &header_format,
                before: before.as_deref(),
                after: after.as_deref(),
                output: output.as_ref(),
                in_place,
            })
            .map(Some),
            PartCommands::Remove {
                path,
                id,
                output,
                in_place,
            } => run_part_remove(&path, &id, output.as_ref(), in_place).map(Some),
        },
    };

    match result {
        Ok(Some(content)) => {
            print!("{content}");
            ExitCode::SUCCESS
        }
        Ok(None) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("{msg}");
            ExitCode::FAILURE
        }
    }
}

fn read_file(path: impl AsRef<Path>) -> Result<String, String> {
    let path = path.as_ref();
    fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn read_body(body_file: Option<&PathBuf>) -> Result<String, String> {
    match body_file {
        Some(path) => read_file(path),
        None => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .map_err(|e| format!("stdin: {e}"))?;
            Ok(buf)
        }
    }
}

fn write_output(content: &str, path: &Path, in_place: bool, output: Option<&PathBuf>) -> Result<(), String> {
    let target = if in_place {
        path
    } else if let Some(out) = output {
        out
    } else {
        print!("{content}");
        return Ok(());
    };
    fs::write(target, content).map_err(|e| format!("{}: {e}", target.display()))
}

fn parse_format(name: &str, path: &Path) -> Result<Format, String> {
    Format::parse_name(name).map_err(|e| format!("{}: {e}", path.display()))
}

struct PartAddOptions<'a> {
    path: &'a Path,
    id: &'a str,
    format_name: &'a str,
    body_file: Option<&'a PathBuf>,
    header_file: Option<&'a PathBuf>,
    header_format_name: &'a str,
    before: Option<&'a str>,
    after: Option<&'a str>,
    output: Option<&'a PathBuf>,
    in_place: bool,
}

fn run_validate(path: &PathBuf) -> Result<(), String> {
    let input = read_file(path)?;
    parse(&input).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(())
}

fn run_canonicalize(path: &PathBuf) -> Result<String, String> {
    let input = read_file(path)?;
    let doc = parse(&input).map_err(|e| format!("{}: {e}", path.display()))?;
    to_string(&doc).map_err(|e| format!("{}: {e}", path.display()))
}

fn run_merge(inputs: &[PathBuf], output: Option<&PathBuf>) -> Result<String, String> {
    if inputs.is_empty() {
        return Err("merge: at least one input file required".to_string());
    }
    let mut documents = Vec::with_capacity(inputs.len());
    for path in inputs {
        let input = read_file(path)?;
        let doc = parse(&input).map_err(|e| format!("{}: {e}", path.display()))?;
        documents.push(doc);
    }
    let merged = merge(documents).map_err(|e| format!("merge: {e}"))?;
    let content = to_string(&merged).map_err(|e| format!("merge: {e}"))?;
    if let Some(out) = output {
        fs::write(out, &content).map_err(|e| format!("{}: {e}", out.display()))?;
        return Ok(String::new());
    }
    Ok(content)
}

fn run_part_extract(
    path: &PathBuf,
    id: &str,
    output: Option<&PathBuf>,
    header: bool,
    full: bool,
) -> Result<String, String> {
    let input = read_file(path)?;
    let doc = parse(&input).map_err(|e| format!("{}: {e}", path.display()))?;
    let part = find_part(&doc, id).map_err(|e| format!("{}: {e}", path.display()))?;

    let content = if full {
        let single = Document {
            file_header: None,
            parts: vec![part.clone()],
        };
        to_string(&single).map_err(|e| format!("{}: {e}", path.display()))?
    } else if header {
        let header_block = part
            .header
            .as_ref()
            .ok_or_else(|| format!("{}: part `{id}` has no header block", path.display()))?;
        header_block.payload.clone()
    } else {
        part.body.clone()
    };

    if let Some(out) = output {
        fs::write(out, &content).map_err(|e| format!("{}: {e}", out.display()))?;
        return Ok(String::new());
    }
    Ok(content)
}

fn run_part_add(opts: PartAddOptions<'_>) -> Result<String, String> {
    let PartAddOptions {
        path,
        id,
        format_name,
        body_file,
        header_file,
        header_format_name,
        before,
        after,
        output,
        in_place,
    } = opts;
    let input = read_file(path)?;
    let mut doc = parse(&input).map_err(|e| format!("{}: {e}", path.display()))?;
    let body_format = parse_format(format_name, path)?;
    let body = read_body(body_file)?;

    let header = if let Some(header_path) = header_file {
        let payload = read_file(header_path)?;
        let format = parse_format(header_format_name, path)?;
        Some(HeaderBlock { format, payload })
    } else {
        None
    };

    let position = match (before, after) {
        (Some(b), None) => InsertPosition::Before(b.to_string()),
        (None, Some(a)) => InsertPosition::After(a.to_string()),
        (None, None) => InsertPosition::End,
        _ => unreachable!("clap prevents both before and after"),
    };

    let part = Part {
        id: id.to_string(),
        body_format,
        header,
        body,
    };
    add_part(&mut doc, part, position).map_err(|e| format!("{}: {e}", path.display()))?;

    let content = to_string(&doc).map_err(|e| format!("{}: {e}", path.display()))?;
    write_output(&content, path, in_place, output)?;
    Ok(String::new())
}

fn run_part_remove(
    path: &PathBuf,
    id: &str,
    output: Option<&PathBuf>,
    in_place: bool,
) -> Result<String, String> {
    let input = read_file(path)?;
    let mut doc = parse(&input).map_err(|e| format!("{}: {e}", path.display()))?;
    remove_part(&mut doc, id).map_err(|e| format!("{}: {e}", path.display()))?;
    let content = to_string(&doc).map_err(|e| format!("{}: {e}", path.display()))?;
    write_output(&content, path, in_place, output)?;
    Ok(String::new())
}
