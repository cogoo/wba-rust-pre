#[cfg(test)]
mod tests {
    use bs58;
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        self,
        signature::{read_keypair_file, Keypair, Signer as SSigner},
        signer::Signer,
        transaction::Transaction,
    };
    use std::io::{self, BufRead};
    use std::str::FromStr;

    const RPC_URL: &str = "https://api.devnet.solana.com";

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
    fn airdrop() {
        let keypair = read_keypair_file("./dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => {
                println!("Error requesting airdrop: {}", e);
            }
        }
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("./dev-wallet.json").expect("Couldn't find wallet file");

        let to_pubkey = Pubkey::from_str("7z7Q3UH4cMxSNDTATsQcC34rr4MVA9ydqpbeqnU4q7ba").unwrap();

        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("failed tto send transaction");

        println!(
            "success! check out your tx  here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        )
    }

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
