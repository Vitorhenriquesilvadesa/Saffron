use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "saffron")]
#[command(about = "A lightweight HTTP client for the command line", long_about = None)]
#[command(version = "0.1.5")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Send an HTTP request")]
    Send {
        #[arg(help = "The URL to send the request to (optional if using --from-collection)")]
        url: Option<String>,

        #[arg(short, long, default_value = "GET", help = "HTTP method")]
        method: String,

        #[arg(short = 'H', long, help = "Headers in key:value format", value_parser = parse_header)]
        header: Vec<(String, String)>,

        #[arg(short, long, help = "Request body (text or JSON)")]
        body: Option<String>,

        #[arg(short, long, help = "JSON request body")]
        json: Option<String>,

        #[arg(short = 'd', long, help = "Form data in key=value format", value_parser = parse_form)]
        data: Vec<(String, String)>,

        #[arg(short, long, help = "Timeout in seconds")]
        timeout: Option<u64>,

        #[arg(short = 'L', long, help = "Follow redirects")]
        follow_redirects: bool,

        #[arg(short = 'e', long, help = "Environment name to use")]
        env: Option<String>,

        #[arg(short = 'v', long, help = "Verbose output (show headers)")]
        verbose: bool,

        #[arg(
            short = 'f',
            long = "from-collection",
            help = "Load request from collection (format: collection_name/request_name)"
        )]
        from_collection: Option<String>,
    },

    #[command(about = "Manage collections")]
    Collection {
        #[command(subcommand)]
        action: CollectionAction,
    },

    #[command(about = "Manage environments")]
    Env {
        #[command(subcommand)]
        action: EnvAction,
    },

    #[command(about = "View request history")]
    History {
        #[command(subcommand)]
        action: HistoryAction,
    },
}

#[derive(Subcommand)]
pub enum HistoryAction {
    #[command(about = "List request history")]
    List {
        #[arg(short, long, default_value = "10", help = "Number of entries to show")]
        limit: usize,
    },

    #[command(about = "Show details of a history entry")]
    Show {
        #[arg(help = "Entry ID or index (1-based)")]
        id: String,
    },

    #[command(about = "Rerun a request from history")]
    Rerun {
        #[arg(help = "Entry ID or index (1-based)")]
        id: String,

        #[arg(short = 'v', long, help = "Verbose output")]
        verbose: bool,
    },

    #[command(about = "Clear all history")]
    Clear,
}

#[derive(Subcommand)]
pub enum CollectionAction {
    #[command(about = "Create a new collection")]
    New {
        #[arg(help = "Collection name")]
        name: String,

        #[arg(short, long, help = "Collection description")]
        description: Option<String>,
    },

    #[command(about = "List all collections")]
    List,

    #[command(about = "Show collection details")]
    Show {
        #[arg(help = "Collection name")]
        name: String,
    },

    #[command(about = "Add a request to a collection")]
    Add {
        #[arg(help = "Collection name")]
        collection: String,

        #[arg(help = "Request name")]
        name: String,

        #[arg(help = "URL")]
        url: String,

        #[arg(short, long, default_value = "GET")]
        method: String,

        #[arg(short = 'H', long, value_parser = parse_header)]
        header: Vec<(String, String)>,

        #[arg(short, long)]
        body: Option<String>,

        #[arg(short, long)]
        description: Option<String>,
    },

    #[command(about = "Delete a collection")]
    Delete {
        #[arg(help = "Collection name")]
        name: String,
    },

    #[command(about = "Export collection to file")]
    Export {
        #[arg(help = "Collection name")]
        name: String,

        #[arg(help = "Output file path")]
        output: String,
    },

    #[command(about = "Import collection from file")]
    Import {
        #[arg(help = "Input file path")]
        input: String,
    },
}

#[derive(Subcommand)]
pub enum EnvAction {
    #[command(about = "List all environments")]
    List,

    #[command(about = "Create or update an environment")]
    Set {
        #[arg(help = "Environment name")]
        name: String,

        #[arg(help = "Variables in key=value format", value_parser = parse_env_var)]
        variables: Vec<(String, String)>,
    },

    #[command(about = "Show environment variables")]
    Show {
        #[arg(help = "Environment name")]
        name: String,
    },

    #[command(about = "Delete an environment")]
    Delete {
        #[arg(help = "Environment name")]
        name: String,
    },

    #[command(about = "Set active environment")]
    Use {
        #[arg(help = "Environment name")]
        name: String,
    },
}

fn parse_header(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find(':')
        .ok_or_else(|| format!("Invalid header format: '{}'. Expected 'Key:Value'", s))?;
    Ok((s[..pos].trim().to_string(), s[pos + 1..].trim().to_string()))
}

fn parse_form(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("Invalid form data format: '{}'. Expected 'key=value'", s))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

fn parse_env_var(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("Invalid variable format: '{}'. Expected 'key=value'", s))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}
