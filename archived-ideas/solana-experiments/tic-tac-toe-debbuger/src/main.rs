use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use borsh::BorshDeserialize;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(long)]
    address: Pubkey,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    let url = "http://localhost:8899";
    let client = RpcClient::new(url);

    let account_info = client.get_account(&opts.address)?;

    dbg!(&opts.address, &account_info);

    let mut buf = &account_info.data.as_slice()[8..];
    
    println!("buf: {:?}", &buf);
    let game = Game::deserialize(&mut buf)?;

    dbg!(&game);

    Ok(())
}

#[derive(Debug, BorshDeserialize)]
pub struct Game {
    players: [Pubkey; 2], // (32 * 2) = 64
    turn: u8,             // 1
    board: [u8; 9],       // 9 [ 0 => null, 1 => player 1, 2 => player 2 ]
    state: GameState,     // 32 + 1 = 33
                          // 107 Bytes
}

#[derive(Debug, BorshDeserialize)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}
