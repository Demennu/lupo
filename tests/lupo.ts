import * as anchor from "@coral-xyz/anchor";
import { Program  } from "@coral-xyz/anchor";
import { Lupo, IDL } from "../target/types/lupo";
import { PublicKey, Commitment, Keypair, SystemProgram } from "@solana/web3.js"
import { ASSOCIATED_TOKEN_PROGRAM_ID as associatedTokenProgram, TOKEN_PROGRAM_ID as tokenProgram, createMint, createAccount, mintTo, getAssociatedTokenAddress } from "@solana/spl-token"
import { randomBytes } from "crypto"
import { assert } from "chai"

const commitment: Commitment = "confirmed"

describe("lupo", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Deposit as Program<Lupo>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
