//! Render a concise command/option tree from a clap [`Command`](clap::Command).

use clap::Command;
use std::fmt::Write as _;

/// Build a `tree(1)`-style snapshot of `root` and all subcommands.
pub fn render(root: &Command) -> String {
    let mut out = String::new();
    writeln!(out, "{}", root.get_name()).expect("writing to string");
    render_children(root, "", &mut out);
    out
}

fn render_children(cmd: &Command, prefix: &str, out: &mut String) {
    let subs: Vec<_> = cmd
        .get_subcommands()
        .filter(|sub| sub.get_name() != "help")
        .collect();
    let last = subs.len().saturating_sub(1);
    for (index, sub) in subs.iter().enumerate() {
        let connector = if index == last { "└── " } else { "├── " };
        let branch = if index == last { "    " } else { "│   " };
        writeln!(out, "{prefix}{connector}{}", command_label(sub)).expect("writing to string");
        render_flags(sub, &format!("{prefix}{branch}"), out);
        render_children(sub, &format!("{prefix}{branch}"), out);
    }
}

fn render_flags(cmd: &Command, prefix: &str, out: &mut String) {
    let flags = option_lines(cmd);
    let last = flags.len().saturating_sub(1);
    for (index, flag) in flags.iter().enumerate() {
        let connector = if index == last { "└── " } else { "├── " };
        writeln!(out, "{prefix}{connector}{flag}").expect("writing to string");
    }
}

fn command_label(cmd: &Command) -> String {
    let mut label = cmd.get_name().to_string();
    let positionals: Vec<_> = cmd
        .get_arguments()
        .filter(|arg| arg.is_positional())
        .collect();
    for arg in positionals {
        label.push(' ');
        label.push_str(&positional_label(arg));
    }
    label
}

fn positional_label(arg: &clap::Arg) -> String {
    let id = arg.get_id().to_string();
    let repeatable = arg
        .get_num_args()
        .is_some_and(|range| range.max_values() > 1);
    if repeatable {
        format!("<{}>…", id.trim_end_matches('s'))
    } else {
        format!("<{id}>")
    }
}

fn option_lines(cmd: &Command) -> Vec<String> {
    let mut flags: Vec<String> = cmd
        .get_arguments()
        .filter(|arg| !arg.is_positional())
        .filter(|arg| arg.get_long().is_some() || arg.get_short().is_some())
        .filter(|arg| !arg.is_hide_set())
        .map(format_option)
        .collect();
    flags.sort();
    flags
}

fn format_option(arg: &clap::Arg) -> String {
    let mut parts = Vec::new();
    if let Some(short) = arg.get_short() {
        parts.push(format!("-{short}"));
    }
    if let Some(long) = arg.get_long() {
        parts.push(format!("--{long}"));
    }
    let mut line = parts.join(", ");
    if option_takes_value(arg) {
        if let Some(values) = arg.get_value_names() {
            if let Some(name) = values.first() {
                line.push(' ');
                line.push_str(&format!("<{}>", name.to_lowercase()));
            }
        }
    }
    if let Some(default) = arg.get_default_values().first() {
        if let Some(text) = default.to_str() {
            line.push_str(&format!(" [default: {text}]"));
        }
    }
    line
}

fn option_takes_value(arg: &clap::Arg) -> bool {
    !matches!(
        arg.get_action(),
        clap::ArgAction::SetTrue | clap::ArgAction::SetFalse
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Arg, Command};

    #[test]
    fn renders_nested_subcommands_and_flags() {
        let cmd = Command::new("demo")
            .subcommand(
                Command::new("merge")
                    .arg(Arg::new("inputs").num_args(1..))
                    .arg(Arg::new("output").long("output").short('o')),
            )
            .subcommand(
                Command::new("part").subcommand(
                    Command::new("extract")
                        .arg(Arg::new("path"))
                        .arg(Arg::new("id"))
                        .arg(Arg::new("header").long("header")),
                ),
            );

        let tree = render(&cmd);
        assert!(tree.contains("demo"));
        assert!(tree.contains("merge"));
        assert!(tree.contains("<input>…"));
        assert!(tree.contains("--output"));
        assert!(tree.contains("part"));
        assert!(tree.contains("extract <path> <id>"));
        assert!(tree.contains("--header"));
    }
}
