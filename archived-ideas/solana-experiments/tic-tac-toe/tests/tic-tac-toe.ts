// * To run tests: `$ BROWSER="" anchor test`
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { TicTacToe } from "../target/types/tic_tac_toe";
import { setupGame } from "./setup-game";
// Player 1
import { p1WinsScenario012 } from "./player-one-wins/scenario-012";
import { p1WinsScenario345 } from "./player-one-wins/scenario-345";
import { p1WinsScenario678 } from "./player-one-wins/scenario-678";
import { p1WinsScenario036 } from "./player-one-wins/scenario-036";
import { p1WinsScenario147 } from "./player-one-wins/scenario-147";
import { p1WinsScenario258 } from "./player-one-wins/scenario-258";
import { p1WinsScenario048 } from "./player-one-wins/scenario-048";
import { p1WinsScenario246 } from "./player-one-wins/scenario-246";
// Player 2
import { p2WinsScenario012 } from "./player-two-wins/scenario-012";
import { p2WinsScenario345 } from "./player-two-wins/scenario-345";
import { p2WinsScenario678 } from "./player-two-wins/scenario-678";
import { p2WinsScenario036 } from "./player-two-wins/scenario-036";
import { p2WinsScenario147 } from "./player-two-wins/scenario-147";
import { p2WinsScenario258 } from "./player-two-wins/scenario-258";
import { p2WinsScenario048 } from "./player-two-wins/scenario-048";
import { p2WinsScenario246 } from "./player-two-wins/scenario-246";
import { getShuffledArray } from "./helpers/random-int";
import { playRandomGame } from "./random-game";
import { FixedArray } from "./helpers/fixed-array";
import { playCustomGame } from "./custom-game";

describe("tic-tac-toe", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TicTacToe as Program<TicTacToe>;
  // const playerOne = (program.provider as anchor.AnchorProvider).wallet;
  const playerTwo = anchor.web3.Keypair.generate();


  // it('setup game!', async () => { await setupGame(program, playerTwo) });
  // // * Player 1 wins:
  // // * Horizontal
  // it('player one wins 012', async () => { await p1WinsScenario012(program) });
  // it('player one wins 345', async () => { await p1WinsScenario345(program) });
  // it('player one wins 678', async () => { await p1WinsScenario678(program) });
  // // * Vertical
  // it('player one wins 036', async () => { await p1WinsScenario036(program) });
  // it('player one wins 147', async () => { await p1WinsScenario147(program) });
  // it('player one wins 258', async () => { await p1WinsScenario258(program) });
  // // * Diagonal
  // it('player one wins 048', async () => { await p1WinsScenario048(program) });
  // it('player one wins 246', async () => { await p1WinsScenario246(program) });

  // // * Player 2 wins:
  // // * Horizontal
  // it('player two wins 012', async () => { await p2WinsScenario012(program) });
  // it('player two wins 345', async () => { await p2WinsScenario345(program) });
  // it('player two wins 678', async () => { await p2WinsScenario678(program) });
  // // * Vertical
  // it('player two wins 036', async () => { await p2WinsScenario036(program) });
  // it('player two wins 147', async () => { await p2WinsScenario147(program) });
  // it('player two wins 258', async () => { await p2WinsScenario258(program) });
  // // * Diagonal
  // it('player two wins 048', async () => { await p2WinsScenario048(program) });
  // it('player two wins 246', async () => { await p2WinsScenario246(program) });
  // it('random game', async () => { await playRandomGame(program) });
  // it('random game', async () => { await playRandomGame(program) });
  // it('random game', async () => { await playRandomGame(program) });
  it('custom game', async () => {
    /* Tiles - valid moves
     0 | 1 | 2
    ---+---+---
     3 | 4 | 5
    ---+---+---
     6 | 7 | 8
    */
    let moves: FixedArray<number, 9> = [2, 6, 0, 1, 5, 8, 7, 4, 3];
    /*
     1 | 2 | 1
    ---+---+---
     1 | 2 | 1
    ---+---+---
     2 | 1 | 2
    */
    await playCustomGame(program, moves)
  });

  // it('custom game: Player 1 wins!', async () => {
  //   let moves: FixedArray<number, 9> = [7, 1, 8, 3, 5, 4, 0, 2, 6];
  //   await playCustomGame(program, moves)
  // });
  // it('custom game: Player 2 wins!', async () => {
  //   // Must fail
  //   let moves: FixedArray<number, 9> = [2, 11, 0, 1, 5, 8, 7, 4, 3];
  //   // Invalid move -----------------------^
  //   await playCustomGame(program, moves)
  // });
  // it('custom game: Tie', async () => {
  //   let moves: FixedArray<number, 9> = [2, 6, 0, 1, 5, 8, 7, 4, 3];
  //   await playCustomGame(program, moves)
  // });

  // it('custom game: Tie', async () => {
  //   // Must fail
  //   let moves: FixedArray<number, 9> = [2, 11, 0, 1, 5, 8, 7, 4, 3];
  //   // Invalid move -----------------------^
  //   await playCustomGame(program, moves)
  // });
});
