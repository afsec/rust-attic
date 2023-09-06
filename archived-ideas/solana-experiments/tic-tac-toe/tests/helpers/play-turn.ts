// * To run tests: `$ BROWSER="" anchor test`
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { TicTacToe } from "../../target/types/tic_tac_toe";

export async function playTurn(program: Program<TicTacToe>, game, player,
  tile, expectedTurn, expectedGameState, expectedBoard) {
  // TODO: Add more expects to track program result
  await program.methods
    .play(tile)
    .accounts({
      player: player.publicKey,
      game
    })
    .signers(player instanceof (anchor.Wallet as any) ? [] : [player])
    .rpc()

  const gameState = await program.account.game.fetch(game)
  
  expect(expectedGameState in gameState.state).to.be.true
  
  expect(gameState.turn).to.equal(expectedTurn)
  
  expect(gameState.board).to.eql(expectedBoard)
}
