use clap::{Parser, Subcommand};
use clap_port_flag::Port;
use enigo::Key;

mod client;
mod server;

#[derive(Debug, Parser)]
#[command(name = "clicktogether")]
#[command(about = "About text", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Start a clicktogether server
    //#[command(arg_required_else_help = true)]
    Host {
        #[clap(flatten)]
        port: Port,

        /// the keyboard key that will be clicked
        /// (use " " for space, \n for return)
        key: char,
    },
    /// Connect to a clicktogether server
    #[command(arg_required_else_help = true)]
    Join {
        /// the address of the server to connect to
        address: String,

        /// the username to join the server with
        username: String,
    },
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Host { port, key } => {
            let listener = port.bind_or(5012)?;

            let key = match key {
                '\n' => Key::Return,
                key => Key::Layout(key),
            };

            server::start_server(listener, key).expect("could not start server");
        }
        Commands::Join { address, username } => {
            client::start_client(address, username).expect("could not start client");
        }
    }

    Ok(())
}
