use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ferreiro")]
#[command(about = "A Django-inspired web framework for Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Ferreiro project
    Startproject { name: String },

    /// Create a new app within the project
    Startapp { name: String },

    /// Run the development server
    Runserver {
        #[arg(short = 'H', long, default_value = "127.0.0.1")]
        host: String,

        #[arg(short, long, default_value = "8000")]
        port: u16,

        #[arg(long)]
        hot_reload: bool,
    },

    /// Run database migrations
    Migrate {
        #[arg(short, long)]
        app: Option<String>,
    },

    /// Create new migration files
    Makemigrations {
        #[arg(short, long)]
        app: Option<String>,
    },

    /// Create a superuser
    Createsuperuser,

    /// Launch an interactive shell
    Shell,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Startproject { name } => {
            println!("Creating project: {}", name);
            println!("Not yet implemented. This will be added in future iterations.");
        }
        Commands::Startapp { name } => {
            println!("Creating app: {}", name);
            println!("Not yet implemented. This will be added in future iterations.");
        }
        Commands::Runserver {
            host,
            port,
            hot_reload,
        } => {
            println!("Starting server at {}:{}", host, port);
            if hot_reload {
                println!("Hot reload enabled");
            }
            println!("Not yet implemented. This will be added in future iterations.");
        }
        Commands::Migrate { app } => {
            if let Some(app_name) = app {
                println!("Running migrations for app: {}", app_name);
            } else {
                println!("Running all migrations");
            }
            println!("Not yet implemented. This will be added in future iterations.");
        }
        Commands::Makemigrations { app } => {
            if let Some(app_name) = app {
                println!("Creating migrations for app: {}", app_name);
            } else {
                println!("Creating migrations for all apps");
            }
            println!("Not yet implemented. This will be added in future iterations.");
        }
        Commands::Createsuperuser => {
            println!("Creating superuser");
            println!("Not yet implemented. This will be added in future iterations.");
        }
        Commands::Shell => {
            println!("Launching interactive shell");
            println!("Not yet implemented. This will be added in future iterations.");
        }
    }
}
