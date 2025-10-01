use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Automatically generate directories and files from a YAML file  
    Create {
        /// Specify the directory structure definition file
        #[arg(short, long, default_value = "dirmap.yaml")]
        input: String,
        
        ///Specify the directory structure to extract to
        #[arg(short, long, default_value = ".")]
        output: String,
    },
    /// Export directory structures to a YAML file  
    Export {
        /// Specify the directory to export
        #[arg(short, long, default_value = ".")]
        target: String,
        
        /// Specify output file
        #[arg(short, long, default_value = "dirmap.yaml")]
        output: String,
    }
}

