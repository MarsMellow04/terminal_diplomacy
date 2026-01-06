use clap::{Parser, Subcommand};

use cli::auth::session::FileSessionKeeper;
use cli::commands::util::{CommandError, TcpClient};
use cli::commands::{
    login::LoginCommand,
    join::JoinCommand,
    order::OrderCommand,
    register::RegisterCommand,
    create::CreateCommand,
};
use cli::commands::util::Command;

#[derive(Parser)]
#[command(name = "terminal_diplomacy")]
#[command(about = "A CLI for Terminal Diplomacy", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Subcommand)]
enum Commands {
    // Connect {
    //     host: String,
    //     port: u16,
    // },
    Login {
        username: String,
        password: String,
    },
    Join {
        game: String,
    },
    Order {
        #[arg(short, long)]
        orders: Option<String>
    },
    Register {
        username: String,
        password: String,
    },
    Create {},
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.debug > 0 {
        println!("Debug level: {}", cli.debug);
    }

    let Some(cmd) = cli.command else {
        println!("No command provided. Use --help.");
        return Ok(());
    };

    let session = FileSessionKeeper;
    let client: TcpClient = TcpClient::connect("127.0.0.1:8080").await.unwrap();

    let result: Result<(), CommandError> = match cmd {
        // Commands::Connect { host, port } => {
        //     let addr = format!("{host}:{port}");
        //     let tcp = TcpClient::connect(&addr).await?;
        //     client = Some(tcp);
        //     println!("Connected to {addr}");
        //     Ok(())
        // }

        Commands::Login { username, password } => {
            let mut cmd = LoginCommand::new(client, &session, username, password);
            cmd.execute().await
        }

        Commands::Join { game } => {
            let mut cmd = JoinCommand::new(client, &session, game);
            cmd.execute().await
        }

        Commands::Order { orders } => {
            let mut cmd = OrderCommand::new(client, &session, orders);
            cmd.execute().await
        }

        Commands::Register { username, password } => {
            let mut cmd = RegisterCommand::new(client, &session, username, password);
            cmd.execute().await
        }

        Commands::Create {} => {
            let mut cmd = CreateCommand::new(client, &session);
            cmd.execute().await
        }
    };

    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
    }
    
    Ok(())
}
