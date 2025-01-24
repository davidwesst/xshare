use clap::{Parser, Subcommand};

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

