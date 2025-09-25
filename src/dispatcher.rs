use std::fs::{self, File};
use std::path::Path;

use anyhow::Result;
use clap::Parser;
use serde_yaml::Value;

use crate::macros::*;

use crate::cli::{Args, Commands};
use crate::structure_service::{ create_from_value, dir_to_yaml };

pub struct Dispatcher {}

impl Dispatcher {
    pub fn new() -> Self {
        Self {}
    }
}

impl Dispatcher {
    pub async fn dispatch(&self) -> Result<()> {
        let args: Args = try_with_log!(Args::try_parse().map_err(|e| e));

        match args.command {
            Commands::Create {
                input,
                output
            } => {
                let yaml_str = try_with_log!(fs::read_to_string(&input));
                let data: Value = try_with_log!(serde_yaml::from_str(&yaml_str));
                let base = Path::new(&output);
                
                try_with_log!(fs::create_dir_all(base));
                try_with_log!(create_from_value(base, &data));

                tracing::info!("Directory structure created successfully at `{}`.", base.display());
                
            }
            Commands::Export { 
                target, 
                output 
            } => {
                let yaml_value = try_with_log!(dir_to_yaml(Path::new(&target)));
        
                let file = try_with_log!(File::create(Path::new(&output)));
                try_with_log!(serde_yaml::to_writer(file, &yaml_value));

                tracing::info!("YAML written to {}", output);
            }
            
        }

        Ok(())

    }    
}