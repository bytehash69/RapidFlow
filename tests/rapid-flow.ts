import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RapidFlow } from "../target/types/rapid_flow";

import { createMint, getAssociatedTokenAddress } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { before } from "mocha";

describe("rapid-flow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  const program = anchor.workspace.rapidFlow as Program<RapidFlow>;

  let baseMint: anchor.web3.PublicKey;
  let quoteMint: anchor.web3.PublicKey;
  let marketPda: anchor.web3.PublicKey;
  let bidsPda: anchor.web3.PublicKey;
  let asksPda: anchor.web3.PublicKey;
  let baseVault: anchor.web3.PublicKey;
  let quoteVault: anchor.web3.PublicKey;

  before(async() => {
    baseMint = await createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      null,
      9
    )

    quoteMint = await createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      null,
      6
    )

    console.log("Base Mint:", baseMint.toBase58());
    console.log("Quote Mint:", quoteMint.toBase58());

    [marketPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("market"), baseMint.toBuffer(), quoteMint.toBuffer()],
      program.programId
    );

    [bidsPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("bids"), marketPda.toBuffer()],
      program.programId
    );

    [asksPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("asks"), marketPda.toBuffer()],
      program.programId
    );

    baseVault = await getAssociatedTokenAddress(
      baseMint,
      marketPda,
      true 
    );

    quoteVault = await getAssociatedTokenAddress(
      quoteMint,
      marketPda,
      true
    );

    console.log("Market PDA:", marketPda.toBase58());
    console.log("Bids PDA:", bidsPda.toBase58());
    console.log("Asks PDA:", asksPda.toBase58());
    console.log("Base Vault:", baseVault.toBase58());
    console.log("Quote Vault:", quoteVault.toBase58());
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
        signer: wallet.publicKey,
        baseMint,
        quoteMint,
        //@ts-ignore
        market: marketPda,
        bids: bidsPda,
        asks: asksPda,
        baseVault,
        quoteVault,
    }).rpc();
    console.log("Your transaction signature", tx);
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet}`);
  });
});
