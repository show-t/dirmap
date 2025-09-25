use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Create {
        #[arg(short, long, default_value = "dirmap.yaml")]
        input: String,
        
        #[arg(short, long, default_value = ".")]
        output: String,
    },
    Export {
        #[arg(short, long, default_value = ".")]
        target: String,
        
        #[arg(short, long, default_value = "dirmap.yaml")]
        output: String,
    }
}

