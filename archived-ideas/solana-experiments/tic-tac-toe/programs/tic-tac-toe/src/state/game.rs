use crate::errors::TicTacToeError;
use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct Game {
    players: [Pubkey; 2], // (32 * 2) = 64
    turn: u8,             // 1
    board: [u8; 9],       // 9 [ 0 => null, 1 => player 1, 2 => player 2 ]
    state: GameState,     // 32 + 1 = 33
                          // 107 Bytes
}

impl Game {
    pub const MAXIMUM_SIZE: usize = (32 * 2) + 1 + (9 * (1 + 1)) + (32 + 1);

    pub fn start(&mut self, players: [Pubkey; 2]) -> Result<()> {
        require_eq!(self.turn, 0, TicTacToeError::GameAlreadyStarted);
        self.players = players;
        self.turn = 0;
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.state == GameState::Active
    }

    fn current_player_index(&self) -> u8 {
        let current_index = self.turn % 2; // 0 or 1

        // We need to add one because 0 is our "Nil"
        // Nil        -> 0
        // Player one -> 0 + 1 = 1
        // Player two -> 1 + 1 = 2
        current_index + 1
    }

    pub fn current_player(&self) -> Pubkey {
        let mut idx = usize::from(self.current_player_index());
        if idx > 0 {
            idx -= 1;
        }
        self.players[idx]
    }

    pub fn play(&mut self, tile: &Tile) -> Result<()> {
        require!(self.is_active(), TicTacToeError::GameAlreadyOver);

        match tile.pos {
            0..=8 => match self.board[tile.pos as usize] {
                0 => {
                    self.turn += 1;
                    self.board[tile.pos as usize] = self.current_player_index();
                }
                _ => {
                    return Err(TicTacToeError::TileAlreadySet.into());
                }
            },
            _ => return Err(TicTacToeError::TileOutOfBounds.into()),
        }

        self.update_state();

        Ok(())
    }

    fn is_winning_trio(&self, trio: [usize; 3]) -> bool {
        let [first, second, third] = trio;

        self.board[first] == self.current_player_index()
            && self.board[second] == self.current_player_index()
            && self.board[third] == self.current_player_index()
    }

    fn has_winner(&self) -> bool {
        // * Board mapping
        //  0 | 1 | 2
        // ---+---+---
        //  3 | 4 | 5
        // ---+---+---
        //  6 | 7 | 8
        // * Possible winnings (8 possible winnings)

        // * Horizontal
        // 0, 1, 2
        // 3, 4, 5
        // 6, 7, 8
        self.is_winning_trio([0, 1, 2])
        || self.is_winning_trio([3, 4, 5])
        || self.is_winning_trio([6, 7, 8])
        // * Vertical
        // 0, 3, 6
        // 1, 4, 7
        // 2, 5, 8
        || self.is_winning_trio([0, 3, 6])
        || self.is_winning_trio([1, 4, 7])
        || self.is_winning_trio([2, 5, 8])
        // * Diagonal
        // 0, 4, 8
        // 2, 4 ,6
        || self.is_winning_trio([0, 4, 8])
        || self.is_winning_trio([2, 4, 6])
    }

    fn update_state(&mut self) {
        if self.has_winner() {
            self.state = GameState::Won {
                winner: self.current_player(),
            };
        } else {
            // Search on board for empty spaces
            for pos in 0..=8 {
                if self.board[pos] == 0 {
                    return;
                }
            }
            // game has not been won
            // game has no more free tiles
            // -> game ends in a tie
            self.state = GameState::Tie;
        }
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, PartialEq, Eq)]
pub enum Sign {
    Nil = 0,
    X = 1,
    O = 2,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct Tile {
    pub pos: u8,
}
