import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { TicTacToe } from "../../target/types/tic_tac_toe";
import { playTurn } from "../helpers/play-turn";

export const p2WinsScenario678 = async (program: Program<TicTacToe>) => {
  const DEBUG_MODE = false;
  let gameTurn = 1;

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
  DEBUG_MODE ? console.log("Turn", gameTurn, "Starting new game") : null
  expect(gameInitialState.turn).to.equal(gameTurn);
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


  gameTurn += 1;
  DEBUG_MODE ? console.log("Turn", gameTurn) : null
  await playTurn(
    program,
    gameKeypair.publicKey,
    playerOne,
    { pos: 2 },
    gameTurn,
    "active",
    [
      0, 0, 1,
      0, 0, 0,
      0, 0, 0
    ]
  );

  gameTurn += 1;
  DEBUG_MODE ? console.log("Turn", gameTurn) : null
  await playTurn(
    program,
    gameKeypair.publicKey,
    playerTwo,
    { pos: 6 },
    gameTurn,
    "active",
    [
      0, 0, 1,
      0, 0, 0,
      2, 0, 0
    ]
  );

  gameTurn += 1;
  DEBUG_MODE ? console.log("Turn", gameTurn) : null
  await playTurn(
    program,
    gameKeypair.publicKey,
    playerOne,
    { pos: 3 },
    gameTurn,
    "active",
    [
      0, 0, 1,
      1, 0, 0,
      2, 0, 0
    ]
  );

  gameTurn += 1;
  DEBUG_MODE ? console.log("Turn", gameTurn) : null
  await playTurn(
    program,
    gameKeypair.publicKey,
    playerTwo,
    { pos: 7 },
    gameTurn,
    "active",
    [
      0, 0, 1,
      1, 0, 0,
      2, 2, 0
    ]
  );


  gameTurn += 1;
  DEBUG_MODE ? console.log("Turn", gameTurn) : null
  await playTurn(
    program,
    gameKeypair.publicKey,
    playerOne,
    { pos: 4 },
    gameTurn,
    "active",
    [
      0, 0, 1,
      1, 1, 0,
      2, 2, 0
    ]
  );

  gameTurn += 1;
  DEBUG_MODE ? console.log("Turn", gameTurn) : null
  await playTurn(
    program,
    gameKeypair.publicKey,
    playerTwo,
    { pos: 8 },
    gameTurn - 1,
    "won",
    [
      0, 0, 1,
      1, 1, 0,
      2, 2, 2
    ]
  );
  const finishedGame = await program.account.game.fetch(gameKeypair.publicKey);
  DEBUG_MODE ? console.log("Game: ", finishedGame) : null

  const winnerPk = finishedGame.state["won"]["winner"] as anchor.web3.PublicKey;
  DEBUG_MODE ? console.log("Winner: ", winnerPk) : null

  expect(winnerPk.equals(playerOne.publicKey))

}