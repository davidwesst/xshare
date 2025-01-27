use clap::{Parser, Subcommand};
use std::file;

mod commands;

#[derive(Parser)]
#[command(name = "xshare")]
#[command(author = "David Wesst <david@cocobkostudios.com>")]
#[command(version = "0.1.0")]
#[command(about = "A CLI tool to help share and crosspost blog posts.", long_about = None)]
pub struct Cli {
    #[arg()]
    /// Path to the folder containing the post and related post contents.
    post_path: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Say hello
    Hello {
        // name to say hello to
        #[arg(short, long)]
        name: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let output = run_command(cli);
    println!("{}", output);
}

pub fn run_command(cli: Cli) -> String {
    match cli.command {
        Some(Commands::Hello { name }) => commands::hello::say_hello(name),
        None => "No command provided. Use --help to see available commands.".to_string(),
    }
}

pub fn validate_post_path(path: &str) -> bool {
    return false;
}

#[cfg(test)]
mod tests {

    use super::*;
    //use tempfile::tempdir;

    #[test]
    fn test_validate_post_path_returns_false_if_path_does_not_exist() {
        // arrange
        let does_not_exist_path = "/does/not/exist/";

        // act
        let result = validate_post_path(&does_not_exist_path);

        // assert
        assert_eq!(result, false);
    }
}
