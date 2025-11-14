import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Coinflip } from "../target/types/coinflip";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo } from "@solana/spl-token";

describe("coinflip", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Coinflip as Program<Coinflip>;
  const authority = provider.wallet;
  const player = Keypair.generate();
  let mint: PublicKey;
  let vault: PublicKey;
  let playerToken: PublicKey;

  it("Initialize", async () => {
    mint = await createMint(
      provider.connection,
      authority.payer,
      authority.publicKey,
      null,
      9
    );

    vault = await createAccount(
      provider.connection,
      authority.payer,
      mint,
      program.programId
    );

    const [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    );

    await program.methods
      .initialize(5, false) // 5% house edge, not paused
      .accounts({
        config: configPda,
        authority: authority.publicKey,
        vault,
        tokenMint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();
  });

  it("Play", async () => {
    await provider.connection.requestAirdrop(player.publicKey, 1_000_000_000);
    await new Promise((resolve) => setTimeout(resolve, 1000));

    playerToken = await createAccount(
      provider.connection,
      authority.payer,
      mint,
      player.publicKey
    );

    await mintTo(
      provider.connection,
      authority.payer,
      mint,
      playerToken,
      authority.payer,
      1_000_000
    );

    const gameResult = Keypair.generate();
    await program.methods
      .play(new anchor.BN(100_000), true) // Bet 100,000 tokens, choose Heads
      .accounts({
        config: PublicKey.findProgramAddressSync([Buffer.from("config")], program.programId)[0],
        player: player.publicKey,
        playerToken,
        vault,
        gameResult: gameResult.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      .signers([player])
      .rpc();

    const resultAccount = await program.account.gameResult.fetch(gameResult.publicKey);
    console.log("Final Game Result:", {
      player: resultAccount.player.toBase58(),
      betAmount: resultAccount.betAmount.toNumber(),
      choice: resultAccount.choice,
      result: resultAccount.result,
      won: resultAccount.won,
      timestamp: resultAccount.timestamp.toNumber(),
    });
  });
});
