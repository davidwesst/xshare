use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "xshare")]
#[command(author = "David Wesst <david@cocobkostudios.com>")]
#[command(version = "0.1.0")]
#[command(about = "A CLI tool to help share and crosspost blog posts.", long_about = None)]
pub struct Cli {
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
        Some(Commands::Hello { name }) => say_hello(name),
        None => "No command provided. Use --help to see available commands.".to_string(),
    }
}

pub fn say_hello(name: Option<String>) -> String {
    match name {
        Some(name_value) => format!("Hello {}! Thank you for using XShare.", name_value),
        None => {
            "Hello there! I don't know your name, but I know you are amazing because you are using XShare.".to_string()
        }
    }
}

