import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";
import { TicTacToe } from "../target/types/tic_tac_toe";


export const setupGame = async (program: Program<TicTacToe>, playerTwo: anchor.web3.Keypair) => {
    const gameKeypair = anchor.web3.Keypair.generate();
    const playerOne = (program.provider as anchor.AnchorProvider).wallet;
    // const playerTwo = anchor.web3.Keypair.generate();
    console.log("playerOne: ", playerOne.publicKey.toBase58())
    console.log("playerOne: ", playerOne.publicKey)
    console.log("--")
    console.log("playerTwo: ", playerTwo.publicKey.toBase58())
    console.log("playerTwo: ", playerTwo.publicKey)
    console.log("--")
    console.log("gameKeypair: ", gameKeypair.publicKey.toBase58())
    console.log("gameKeypair: ", gameKeypair.publicKey)


    await program.methods
        .setupGame(playerTwo.publicKey)
        .accounts({
            game: gameKeypair.publicKey,
            playerOne: playerOne.publicKey,
        })
        .signers([gameKeypair])
        .rpc();

    let gameState = await program.account.game.fetch(gameKeypair.publicKey);
    console.log("Game: ", gameState)
    console.log("----------")
    expect(gameState.turn).to.equal(1);
    expect(gameState.players)
        .to
        .eql([playerOne.publicKey, playerTwo.publicKey]);
    expect(gameState.state).to.eql({ active: {} });
    expect(gameState.board)
        .to
        .eql([0, 0, 0, 0, 0, 0, 0, 0, 0]);
}