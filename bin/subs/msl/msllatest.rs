use crate::subs::runnable::RunnableSubcommand;
use async_trait::async_trait;
use mars_raw_utils::prelude::*;

use std::process;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Report sols with new images", long_about = None)]
pub struct MslLatest {
    #[arg(long, short, help = "List sols with new images only")]
    list: bool,
}

#[async_trait]
impl RunnableSubcommand for MslLatest {
    async fn run(&self) {
        let latest: msl::latest::LatestData = match msl::remote::fetch_latest().await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error fetching latest data from MSL remote server: {}", e);
                process::exit(1);
            }
        };

        if self.list {
            latest.latest_sols.iter().for_each(|s| {
                println!("{}", s);
            });
        } else {
            println!("Latest data: {}", latest.latest);
            println!("Latest sol: {}", latest.latest_sol);
            println!("Latest sols: {:?}", latest.latest_sols);
            println!("New Count: {}", latest.new_count);
            println!("Sol Count: {}", latest.sol_count);
            println!("Total: {}", latest.total);
        }
    }
}
