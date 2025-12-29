use clap::{Parser, Subcommand};
use cli::commands::util::Command;

// Need it in main

// Command design structure:
// This setup allows commands to be easily separated and reused.
// Even if we later create a non-CLI version, the commands remain modular.

pub use cli::commands::connect::ConnectCommand;
pub use cli::commands::login::LoginCommand;
pub use cli::commands::join::JoinCommand;
pub use cli::commands::order::OrderCommand;
pub use cli::commands::map::MapCommand;
pub use cli::commands::register::RegisterCommand;
pub use cli::commands::create::CreateCommand;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}


#[derive(Subcommand)]
enum Commands {
    /// Connect a user to the server
    Connect {
        #[arg(long, required = true)]
        host: String,

        #[arg(short, long, required = true)]
        port: String,
    },
    // /// Login to terminal diplomacy
    // Login {
    //     #[arg(short, long, required = true)]
    //     username: String,

    //     #[arg(short, long, required = true)]
    //     password: String,
    // },
    // Join a game of diplomacy
    // Join {
    //     #[arg(short, long, required = true)]
    //     game: String,
    // },
    // Submit order to game
    // Order {
    //     #[arg(short, long, required = false)]
    //     name: String,

    //     #[arg(short, long, required = true)]
    //     game: String,
    // },
    // Showcase the map of the game
    // Map {
    //     save_image: bool
    // },
    // //// Register user 
    // Register {
    //     #[arg(short, long, required = true)]
    //     username: String,

    //     #[arg(short, long, required = true)]
    //     password: String,
    // },
    // /// Create a new Gaem
    // Create {}
}

impl Commands {
    fn into_command(self) -> Box<dyn Command> {
        match self {
            // We have to create a heap pointer, because of the different types
            Commands::Connect { host, port } => Box::new(ConnectCommand::new(host, port)),
            // Commands::Login { username, password } => Box::new(LoginCommand::new(username, password)),
            // Commands::Join { game } => Box::new(JoinCommand::new(game)),
            // Commands::Order { name , game} => Box::new(OrderCommand::new(Some(name), game)),
            // Commands::Map {save_image} => Box::new(MapCommand::new(save_image)),
            // Commands::Register { username, password } => Box::new(RegisterCommand::new(username, password)),
            // Commands::Create {  } => Box::new(CreateCommand::new())
        }
    }
}

fn main() {
    let cli = Cli::parse();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    if let Some(cmd) = cli.command {
        let command_obj = cmd.into_command();
        command_obj.execute();
    } else {
        println!("No command provided. Use --help for usage.");
    }

}