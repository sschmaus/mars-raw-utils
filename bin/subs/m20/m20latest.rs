use mars_raw_utils::prelude::*;

use crate::subs::runnable::RunnableSubcommand;

use std::process;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Report sols with new images", long_about = None)]
pub struct M20Latest {
    #[arg(long, short, help = "List sols with new images only")]
    list: bool,
}

#[async_trait::async_trait]
impl RunnableSubcommand for M20Latest {
    async fn run(&self) {
        let latest: m20::latest::LatestData = match m20::remote::fetch_latest().await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error fetching latest data from M20 remote server: {}", e);
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
