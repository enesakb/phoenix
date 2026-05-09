use bech32::Hrp;
use ripemd::{Digest as RipemdDigest, Ripemd160};
use secp256k1::PublicKey;
use sha2::Sha256;
use tiny_keccak::{Hasher, Keccak};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressKind {
    BtcSegwit,
    Eth,
}

/// SHA256 then RIPEMD160 — the classic hash160.
fn hash160(input: &[u8]) -> [u8; 20] {
    let sha = Sha256::digest(input);
    let mut ripe = Ripemd160::new();
    RipemdDigest::update(&mut ripe, sha);
    let out = ripe.finalize();
    let mut bytes = [0u8; 20];
    bytes.copy_from_slice(&out);
    bytes
}

/// BTC native segwit (P2WPKH, witness v0, mainnet bc1...) address.
pub fn btc_p2wpkh_address(pk: &PublicKey) -> String {
    let compressed = pk.serialize();
    let h160 = hash160(&compressed);
    let hrp = Hrp::parse("bc").expect("valid hrp");
    bech32::segwit::encode_v0(hrp, &h160).unwrap_or_default()
}

/// Ethereum 0x-prefixed lowercase address.
pub fn eth_address(pk: &PublicKey) -> String {
    let uncompressed = pk.serialize_uncompressed();
    // Skip the 0x04 prefix; use the 64-byte X||Y.
    let key_bytes = &uncompressed[1..];
    let mut hasher = Keccak::v256();
    hasher.update(key_bytes);
    let mut out = [0u8; 32];
    hasher.finalize(&mut out);
    let addr_bytes = &out[12..];
    format!("0x{}", hex::encode(addr_bytes))
}
