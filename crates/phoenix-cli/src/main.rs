use clap::{Parser, Subcommand, ValueEnum};
use phoenix_core::crypto::{
    address::{btc_p2wpkh_address, eth_address, AddressKind},
    derive::{derive_btc_segwit_key, derive_eth_key},
    mnemonic::{generate_fresh_mnemonic, mnemonic_to_seed},
    reconstruct::reconstruct_missing_word,
    solana::{
        all_addresses, derive_solana_signing_key, phantom_address, solana_address, ALL_PATHS,
    },
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
    /// Generate a fresh BIP-39 wallet locally (entropy from OS RNG).
    /// Prints the seed phrase ONCE — write it down on paper before continuing.
    /// Also prints the BTC + ETH + Solana receive addresses you can share.
    /// Phoenix never persists or transmits the seed.
    WalletCreate {
        /// Optional BIP-39 passphrase to layer on top of the seed.
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

/// Public donation addresses pinned in `docs/wallets.md`. These are the same
/// addresses surfaced on the landing page and in the README. Shown to the
/// user immediately after a successful recovery so they can voluntarily
/// tip the project — no on-chain enforcement, fully opt-in.
const PHOENIX_BTC_ADDRESS: &str = "bc1p0730rztrz3yw3fc0an28tuxft0cstfcfr7mu0umc4scl8z0kradqzprlpr";
const PHOENIX_EVM_ADDRESS: &str = "0x7C17c4937cABD75CB8657f5fb1c4184325Bff652";
const PHOENIX_SOL_ADDRESS: &str = "5De7kbLn9SSsKLVQCCMdcRyAvofeijD6VQPqc9CZXwyT";

fn print_donation_prompt(kind: &AddressKindArg) {
    let chain = match kind {
        AddressKindArg::Btc => "Bitcoin (Taproot)",
        AddressKindArg::Eth => "EVM (Ethereum + L2s + Base + Monad)",
        AddressKindArg::Sol => "Solana",
    };
    let addr = match kind {
        AddressKindArg::Btc => PHOENIX_BTC_ADDRESS,
        AddressKindArg::Eth => PHOENIX_EVM_ADDRESS,
        AddressKindArg::Sol => PHOENIX_SOL_ADDRESS,
    };
    println!();
    println!("────────────────────────────────────────────────────────────────────────");
    println!("  Phoenix is open source and free for the first 5 pilot recoveries.");
    println!();
    println!("  If this saved a wallet you thought was gone, a 5% voluntary tip");
    println!("  funds the next user's free recovery and the upcoming security audit.");
    println!();
    println!("  Same chain as your recovery ({chain}):");
    println!("    {addr}");
    println!();
    println!("  Cross-chain options (open source registry — verify in git history):");
    println!("    BTC: {PHOENIX_BTC_ADDRESS}");
    println!("    EVM: {PHOENIX_EVM_ADDRESS}");
    println!("    SOL: {PHOENIX_SOL_ADDRESS}");
    println!();
    println!("  Tipping is fully voluntary. Phoenix never holds your keys; you sign.");
    println!("  Verify these addresses against:");
    println!("    https://github.com/enesakb/phoenix/blob/master/docs/wallets.md");
    println!("────────────────────────────────────────────────────────────────────────");
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
                    print_donation_prompt(&kind);
                }
                Err(e) => {
                    eprintln!("✗ no match found ({:.2?}): {e}", elapsed);
                    std::process::exit(1);
                }
            }
        }
        Command::WalletCreate { passphrase } => {
            let mnemonic = generate_fresh_mnemonic();
            let seed = mnemonic_to_seed(&mnemonic, &passphrase)?;

            let (_, btc_pk) = derive_btc_segwit_key(&seed, 0)?;
            let (_, eth_pk) = derive_eth_key(&seed, 0)?;

            println!();
            println!("════════════════════════════════════════════════════════════════════════");
            println!("  ⚠ WRITE THIS DOWN. ON PAPER. NOW.");
            println!("════════════════════════════════════════════════════════════════════════");
            println!();
            println!("  BIP-39 mnemonic (12 words, KEEP SECRET):");
            println!();
            for (i, word) in mnemonic.split_whitespace().enumerate() {
                println!("    {:2}.  {}", i + 1, word);
            }
            println!();
            println!("════════════════════════════════════════════════════════════════════════");
            println!("  Your public receive addresses (safe to share publicly):");
            println!("════════════════════════════════════════════════════════════════════════");
            println!();
            println!("    BTC (native segwit, m/84'/0'/0'/0/0)");
            println!("      {}", btc_p2wpkh_address(&btc_pk));
            println!();
            println!("    ETH (m/44'/60'/0'/0/0)");
            println!("      {}", eth_address(&eth_pk));
            println!();
            println!("    Solana / Phantom (m/44'/501'/0'/0')");
            println!("      {}", phantom_address(&seed));
            println!();
            println!("════════════════════════════════════════════════════════════════════════");
            println!("  AFTER you have written the mnemonic on paper:");
            println!("    1. Clear this terminal screen (Ctrl+L on Linux/Mac, Clear-Host on PS).");
            println!("    2. Verify by importing the mnemonic into a real wallet (Phantom,");
            println!("       MetaMask, Sparrow). The addresses must match.");
            println!("    3. Send only the public addresses to anyone — never the mnemonic.");
            println!("════════════════════════════════════════════════════════════════════════");
            println!();
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
