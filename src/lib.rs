#[cfg(test)]
mod tests {
    use bs58;
    use solana_sdk::{self, signature::Keypair, signer::Signer};
    use std::io::{self, BufRead};

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana Wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and past the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {}

    #[test]
    fn transfer_sol() {}

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let base58 = stdin
            .lock()
            .lines()
            .next()
            .expect("Failed to read input")
            .expect("Failed to parse input");

        println!("Your wallet file is:");
        let wallet = bs58::decode(base58)
            .into_vec()
            .expect("Failed to decode base58");

        println!("{:?}", wallet);
    }

    #[test]
    fn test_wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = std::io::stdin();
        let wallet: Vec<u8> = stdin
            .lock()
            .lines()
            .next()
            .expect("Failed to read input")
            .expect("Failed to parse input")
            .trim_matches(['[', ']'])
            .split(',')
            .map(|s| s.trim().parse().expect("Failed to parse byte"))
            .collect();
        println!("your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }
}
