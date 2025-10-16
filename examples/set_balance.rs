// Rust CLI tool to set exact balance using Surfpool cheatcodes
// Usage: cargo run --bin set_balance -- <address> <lamports>

use serde_json::json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{account::Account, pubkey::Pubkey};
use std::env;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <address> <lamports>", args[0]);
        eprintln!("\nExamples:");
        eprintln!("  {} BgKUXdS29YcH... 1000000000    # Set to 1 SOL", args[0]);
        eprintln!(
            "  {} BgKUXdS29YcH... 1              # Set to 1 lamport",
            args[0]
        );
        eprintln!(
            "  {} BgKUXdS29YcH... 5000000000000 # Set to 5000 SOL",
            args[0]
        );
        std::process::exit(1);
    }

    let address = Pubkey::from_str(&args[1])?;
    let lamports: u64 = args[2].parse()?;

    set_balance(address, lamports)?;

    Ok(())
}

fn set_balance(address: Pubkey, target_lamports: u64) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Setting EXACT balance for {}", address);
    let sol_amount = target_lamports as f64 / 1_000_000_000.0;
    println!(
        "   Target: {} lamports ({:.9} SOL)\n",
        target_lamports, sol_amount
    );

    // Connect to local Surfpool
    let client = RpcClient::new("http://127.0.0.1:8899".to_string());

    // Get current balance
    let current = client.get_balance(&address)?;
    let current_sol = current as f64 / 1_000_000_000.0;
    println!(
        "Current balance: {} lamports ({:.9} SOL)",
        current, current_sol
    );

    // Get account info
    println!("Fetching account info...");
    let account = client.get_account(&address)?;

    println!("Account owner: {}", account.owner);
    println!("Account executable: {}", account.executable);
    println!("Data length: {} bytes\n", account.data.len());

    // Call surfnet_setAccount cheatcode
    println!("Calling surfnet_setAccount cheatcode...");

    // Convert data to hex string (Surfpool expects hex format)
    let data_hex = hex::encode(&account.data);

    let params = json!([
        address.to_string(),
        {
            "lamports": target_lamports,
            "data": data_hex,  // Hex-encoded string
            "owner": account.owner.to_string(),
            "executable": account.executable,
            "rentEpoch": account.rent_epoch,
        }
    ]);

    let result: serde_json::Value = client.send(
        solana_client::rpc_request::RpcRequest::Custom {
            method: "surfnet_setAccount",
        },
        params,
    )?;

    println!("✅ Balance set successfully!");

    // Verify the change
    std::thread::sleep(std::time::Duration::from_millis(500));
    let new_balance = client.get_balance(&address)?;
    let new_sol = new_balance as f64 / 1_000_000_000.0;
    let diff = new_balance as i64 - current as i64;
    let diff_sol = diff as f64 / 1_000_000_000.0;

    println!(
        "\n✓ Old balance: {} lamports ({:.9} SOL)",
        current, current_sol
    );
    println!(
        "✓ New balance: {} lamports ({:.9} SOL)",
        new_balance, new_sol
    );
    println!("✓ Changed by: {} lamports ({:.9} SOL)", diff, diff_sol);

    Ok(())
}

// For integration into existing code:
#[allow(dead_code)]
pub fn set_account_balance(
    address: Pubkey,
    lamports: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClient::new("http://127.0.0.1:8899".to_string());

    // Get current account to preserve data
    let account = client.get_account(&address)?;

    let params = json!([
        address.to_string(),
        {
            "lamports": lamports,
            "data": account.data,
            "owner": account.owner.to_string(),
            "executable": account.executable,
            "rentEpoch": account.rent_epoch,
        }
    ]);

    client.send(
        solana_client::rpc_request::RpcRequest::Custom {
            method: "surfnet_setAccount",
        },
        params,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Run with: cargo test -- --ignored
    fn test_set_balance() {
        let address = Pubkey::from_str("BgKUXdS29YcHCFrPm5M8oLHiTzZaMDjsebggjoaQ6KFL").unwrap();

        // Set to 1000 SOL
        set_account_balance(address, 1_000_000_000_000).unwrap();

        // Verify
        let client = RpcClient::new("http://127.0.0.1:8899".to_string());
        let balance = client.get_balance(&address).unwrap();
        assert_eq!(balance, 1_000_000_000_000);
    }
}
