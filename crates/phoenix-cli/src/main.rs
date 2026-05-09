use clap::{Parser, Subcommand, ValueEnum};
use phoenix_core::crypto::{address::AddressKind, reconstruct::reconstruct_missing_word};
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
        /// Target address (BTC bech32 or ETH 0x...).
        #[arg(long)]
        target: String,
        /// Address kind to derive.
        #[arg(long, value_enum, default_value_t = AddressKindArg::Eth)]
        kind: AddressKindArg,
        /// Optional BIP-39 passphrase.
        #[arg(long, default_value = "")]
        passphrase: String,
        /// Number of address indexes to scan per candidate.
        #[arg(long, default_value_t = 5)]
        index_range: u32,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum AddressKindArg {
    Eth,
    Btc,
}

impl From<AddressKindArg> for AddressKind {
    fn from(v: AddressKindArg) -> Self {
        match v {
            AddressKindArg::Eth => AddressKind::Eth,
            AddressKindArg::Btc => AddressKind::BtcSegwit,
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
                    println!("  Address index : {}", r.address_index);
                    println!("  Mnemonic      : {}", r.recovered_mnemonic);
                    println!("  Elapsed       : {:.2?}", elapsed);
                }
                Err(e) => {
                    eprintln!("✗ no match found ({:.2?}): {e}", elapsed);
                    std::process::exit(1);
                }
            }
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
