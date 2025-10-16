use std::str::FromStr;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use solana_client::rpc_client::RpcClient;
use solana_pubkey::Pubkey;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// RPC URL to connect to
    #[arg(short, long)]
    rpc_url: String,
}

fn main() {
    let args = Args::parse();
    let mainnet_client = RpcClient::new(args.rpc_url);
    let local_client = RpcClient::new("http://127.0.0.1:8899");

    let programs = [
        Pubkey::from_str("HistoryJTGbKQD2mRgLZ3XhqHnN811Qpez8X9kCcGHoa").unwrap(),
        Pubkey::from_str("Stewardf95sJbmtcZsyagb2dg4Mo8eVQho8gpECvLx8").unwrap(),
    ];

    println!("=== Starting Account Fetch ===\n");

    for (prog_idx, program) in programs.iter().enumerate() {
        println!("Program {}/{}: {}", prog_idx + 1, programs.len(), program);
        println!("Fetching accounts from mainnet...");

        // Get program accoutnts from mainnet
        let program_accs = match mainnet_client.get_program_accounts(&program) {
            Ok(accs) => accs,
            Err(e) => {
                eprintln!("❌ Failed to fetch accounts: {}", e);
                continue;
            }
        };

        let total = program_accs.len();
        println!("✓ Found {} accounts\n", total);

        // Dump accounts
        let pb = ProgressBar::new(total as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) | Dumped: {msg}")
                .unwrap()
                .progress_chars("█▓░")
        );

        let mut dumped = 0;
        let mut skipped = 0;

        for program_acc in program_accs.iter() {
            if let Err(_e) = local_client.get_account(&program_acc.0) {
                skipped += 1;
            } else {
                dumped += 1;
            }

            pb.set_message(format!("{} | Skipped: {}", dumped, skipped));
            pb.inc(1);
        }

        pb.finish_with_message(format!("{} dumped, {} skipped", dumped, skipped));
        println!();
    }
}
