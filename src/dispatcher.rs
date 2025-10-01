use std::fs::{self, File};
use std::path::Path;

use anyhow::Result;
use clap::{Parser, error::ErrorKind};
use serde_yaml::Value;

use logkit_macros::try_with_log;

use crate::cli::{Args, Commands};
use crate::structure_service::{ create_from_value, dir_to_yaml };

pub struct Dispatcher {
    args: Args,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            args: Self::parse(),
        }
    }

    fn parse() -> Args {
        match  Args::try_parse() {
            Ok(a) => a,
            Err(err) if err.kind() == ErrorKind::DisplayHelp
                    || err.kind() == ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
                => {
                    _ = err.print();
                    std::process::exit(0);
                }
            Err(err) => {
                tracing::error!("Something went wrong: {:?}", err);
                std::process::exit(0);
            }
        }
    }
}

impl Dispatcher {
    #[try_with_log]
    pub async fn dispatch(&self) -> Result<()> {
        match &self.args.command {
            Commands::Create {
                input,
                output
            } => {
                let yaml_str = fs::read_to_string(&input)?;
                let data: Value = serde_yaml::from_str(&yaml_str)?;
                let base = Path::new(&output);
                
                fs::create_dir_all(base)?;
                create_from_value(base, &data)?;

                tracing::info!("Directory structure created successfully at `{}`.", base.display());
                
            }
            Commands::Export { 
                target, 
                output 
            } => {
                let yaml_value = dir_to_yaml(Path::new(&target))?;
        
                let file = File::create(Path::new(&output))?;
                serde_yaml::to_writer(file, &yaml_value)?;

                tracing::info!("YAML written to {}", output);
            }
            
        }

        Ok(())

    }    
}