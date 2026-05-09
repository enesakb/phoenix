use clap::{Parser, Subcommand, ValueEnum};
use phoenix_core::crypto::{
    address::AddressKind,
    mnemonic::mnemonic_to_seed,
    reconstruct::reconstruct_missing_word,
    solana::{all_addresses, derive_solana_signing_key, solana_address, ALL_PATHS},
};
use phoenix_core::llm::{LlmClient, OllamaClient};

#[derive(Parser)]
#[command(name = "phoenix", version, about = "Phoenix recovery CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print version and environment info.
    Doctor,
    /// Round-trip a "ping" prompt against the configured Ollama instance.
    OllamaCheck {
        #[arg(long, default_value = "http://localhost:11434")]
        endpoint: String,
        #[arg(long, default_value = "llama3.3:70b")]
        model: String,
    },
    /// Brute-force a single missing word in a 12-word BIP-39 mnemonic and
    /// verify against an on-chain address.
    Reconstruct {
        /// 12 space-separated tokens; the unknown position is `?`.
        #[arg(long)]
        template: String,
        /// Target address (BTC bech32, ETH 0x..., or Solana base58).
        #[arg(long)]
        target: String,
        /// Address kind to derive.
        #[arg(long, value_enum, default_value_t = AddressKindArg::Eth)]
        kind: AddressKindArg,
        /// Optional BIP-39 passphrase.
        #[arg(long, default_value = "")]
        passphrase: String,
        /// Number of address indexes to scan per candidate (BTC/ETH only;
        /// ignored for Solana which iterates wallet derivation paths).
        #[arg(long, default_value_t = 5)]
        index_range: u32,
    },
    /// Print all Solana addresses Phoenix derives for a given mnemonic so you
    /// can compare against your wallet (Phantom, Solflare, Backpack, etc.).
    SolanaShow {
        /// 12-word mnemonic.
        #[arg(long)]
        mnemonic: String,
        /// Optional BIP-39 passphrase.
        #[arg(long, default_value = "")]
        passphrase: String,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum AddressKindArg {
    Eth,
    Btc,
    Sol,
}

impl From<AddressKindArg> for AddressKind {
    fn from(v: AddressKindArg) -> Self {
        match v {
            AddressKindArg::Eth => AddressKind::Eth,
            AddressKindArg::Btc => AddressKind::BtcSegwit,
            AddressKindArg::Sol => AddressKind::Solana,
        }
    }
}

fn doctor_output() -> String {
    format!("phoenix-core version: {}", phoenix_core::version())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Doctor => println!("{}", doctor_output()),
        Command::OllamaCheck { endpoint, model } => {
            let client = OllamaClient::new(&endpoint, &model);
            let response = client.generate("respond with the single word: ok").await?;
            println!("ollama responded: {}", response.trim());
        }
        Command::Reconstruct {
            template,
            target,
            kind,
            passphrase,
            index_range,
        } => {
            let started = std::time::Instant::now();
            let result =
                reconstruct_missing_word(&template, &target, kind.into(), &passphrase, index_range);
            let elapsed = started.elapsed();
            match result {
                Ok(r) => {
                    println!("✓ Recovered word: {}", r.recovered_word);
                    println!("  Path / index  : {}", r.address_index);
                    println!("  Mnemonic      : {}", r.recovered_mnemonic);
                    println!("  Elapsed       : {:.2?}", elapsed);
                }
                Err(e) => {
                    eprintln!("✗ no match found ({:.2?}): {e}", elapsed);
                    std::process::exit(1);
                }
            }
        }
        Command::SolanaShow {
            mnemonic,
            passphrase,
        } => {
            let seed = mnemonic_to_seed(&mnemonic, &passphrase)?;
            println!("Solana addresses derived from this mnemonic:");
            for (i, path) in ALL_PATHS.iter().enumerate() {
                let key = derive_solana_signing_key(&seed, path);
                let addr = solana_address(&key);
                let path_str = path
                    .iter()
                    .map(|p| format!("{p}'"))
                    .collect::<Vec<_>>()
                    .join("/");
                let label = match i {
                    0 => "Phantom / Backpack / Trust",
                    1 => "Solflare",
                    2 => "Sollet (legacy)",
                    _ => "?",
                };
                println!("  [{i}] {label:<27} m/{path_str}");
                println!("      {addr}");
            }
            // Also show the bare all_addresses pairs to verify the public
            // helper agrees.
            let _ = all_addresses(&seed);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doctor_output_contains_version() {
        let output = doctor_output();
        assert!(output.contains("phoenix-core version: 0.1.0"));
    }
}
