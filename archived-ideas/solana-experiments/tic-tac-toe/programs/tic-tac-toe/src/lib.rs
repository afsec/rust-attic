use anchor_lang::prelude::*;
use instructions::*;
use state::Tile;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("A5HhU7UteoLRNgfzNYNT3MfHUMjrppvcJP5dJx7ZvcPH");

#[program]
pub mod tic_tac_toe {
    use super::*;

    // * Instruction
    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
        instructions::setup_game::handler(ctx, player_two)
    }

    // * Instruction
    pub fn play(ctx: Context<Play>, tile: Tile) -> Result<()> {
        instructions::play::handler(ctx, tile)
    }
}
