use clap::Parser;
use saffron_cli::cli::{Cli, Commands};
use saffron_cli::handlers::{handle_collection, handle_env, handle_history, handle_send};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Send {
            url,
            method,
            header,
            body,
            json,
            data,
            timeout,
            follow_redirects,
            env,
            verbose,
            from_collection,
        } => {
            handle_send(
                url,
                method,
                header,
                body,
                json,
                data,
                timeout,
                follow_redirects,
                env,
                verbose,
                from_collection,
            );
        }
        Commands::Collection { action } => {
            handle_collection(action);
        }
        Commands::Env { action } => {
            handle_env(action);
        }
        Commands::History { action } => {
            handle_history(action);
        }
    }
}
