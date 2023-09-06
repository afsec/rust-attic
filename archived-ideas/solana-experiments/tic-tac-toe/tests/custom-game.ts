import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { TicTacToe } from "../target/types/tic_tac_toe";
import { FixedArray } from "./helpers/fixed-array";
import { getShuffledArray } from "./helpers/random-int";

function delay(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

export async function playMove(program: Program<TicTacToe>, game, player,
  tile) {
  // TODO: Add more expects to track program result
  await program.methods
    .play(tile)
    .accounts({
      player: player.publicKey,
      game
    })
    .signers(player instanceof (anchor.Wallet as any) ? [] : [player])
    .rpc()
}

function hasValidmoves(moves: FixedArray<number, 9>): boolean {
  const maxMovesIdx = moves.length - 1

  for (let idx = 0; idx <= maxMovesIdx; idx++) {
    if ((moves[idx] < 0) || (moves[idx] > maxMovesIdx)) return false
  }
  return true
}

export const playCustomGame = async (program: Program<TicTacToe>, moves: FixedArray<number, 9>) => {
  // let moves: number[] = getShuffledArray();
  expect(hasValidmoves(moves)).to.be.true
  console.log("");
  console.log("------");
  console.log(" Game");
  console.log("------");
  console.log("Moves:", moves);

  const DEBUG_MODE = true;

  const gameKeypair = anchor.web3.Keypair.generate();
  const playerOne = (program.provider as anchor.AnchorProvider).wallet;
  const playerTwo = anchor.web3.Keypair.generate();
  await program.methods
    .setupGame(playerTwo.publicKey)
    .accounts({
      game: gameKeypair.publicKey,
      playerOne: playerOne.publicKey,
    })
    .signers([gameKeypair])
    .rpc();


  const gameInitialState = await program.account.game.fetch(gameKeypair.publicKey);
  expect(gameInitialState.players)
    .to
    .eql([playerOne.publicKey, playerTwo.publicKey]);
  expect(gameInitialState.state).to.eql({ active: {} });
  expect(gameInitialState.board)
    .to
    .eql(
      [
        0, 0, 0,
        0, 0, 0,
        0, 0, 0
      ]
    );

  let currentPlayerSign = 1;

  while ("active" in ((await program.account.game.fetch(gameKeypair.publicKey)).state)) {
    const player = currentPlayerSign === 1 ? playerOne : playerTwo
    await playMove(
      program,
      gameKeypair.publicKey,
      player,
      { pos: moves.shift() }
    )
    currentPlayerSign = currentPlayerSign === 1 ? 2 : 1
    ////
    const runningGame = await program.account.game.fetch(gameKeypair.publicKey)
    DEBUG_MODE ? console.log("runningGame", runningGame) : null
    const board = runningGame.board;

    DEBUG_MODE ? console.log("Board: [") : null
    DEBUG_MODE ? console.log("Board: ", `${board[0]}, ${board[1]}, ${board[2]}`) : null
    DEBUG_MODE ? console.log("Board: ", `${board[3]}, ${board[4]}, ${board[5]}`) : null
    DEBUG_MODE ? console.log("Board: ", `${board[6]}, ${board[7]}, ${board[8]}`) : null
    DEBUG_MODE ? console.log("Board: ]") : null
    DEBUG_MODE ? console.log("---") : null
    await delay(10000);
  }



  const finishedGame = await program.account.game.fetch(gameKeypair.publicKey)
  DEBUG_MODE ? console.log("finishedGame", finishedGame) : null
  const board = finishedGame.board;

  DEBUG_MODE ? console.log("Board: [") : null
  DEBUG_MODE ? console.log("Board: ", `${board[0]}, ${board[1]}, ${board[2]}`) : null
  DEBUG_MODE ? console.log("Board: ", `${board[3]}, ${board[4]}, ${board[5]}`) : null
  DEBUG_MODE ? console.log("Board: ", `${board[6]}, ${board[7]}, ${board[8]}`) : null
  DEBUG_MODE ? console.log("Board: ]") : null
  DEBUG_MODE ? console.log("RESULT: ") : null

  if ("won" in finishedGame.state) {
    const winnerPk = (finishedGame.state["won"]["winner"] as anchor.web3.PublicKey).toBase58()
    const p1PubKey = playerOne.publicKey.toBase58();
    const p2PubKey = playerTwo.publicKey.toBase58();
    DEBUG_MODE ? console.log("P1", p1PubKey) : null
    DEBUG_MODE ? console.log("P2", p2PubKey) : null
    DEBUG_MODE ? console.log("winnerPk", winnerPk) : null
    const winner = winnerPk === p1PubKey ? "Player 1" : "Player 2"
    console.log(winner, "Won!")
  } else if ("tie" in finishedGame.state) {
    console.log("Tie!")
  } else {
    DEBUG_MODE ? console.log("ERROR: Impossible state!") : null
  }
  expect(("won" in finishedGame.state) || ("tie" in finishedGame.state))
}