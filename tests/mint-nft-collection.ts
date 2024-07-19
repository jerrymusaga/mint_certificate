import { Program } from "@coral-xyz/anchor";
import { MintNftCollection } from "../target/types/mint_nft_collection";
import * as anchor from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { PublicKey, SystemProgram } from "@solana/web3.js";

describe("mint-nft-collection", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.local());

  const program = anchor.workspace
    .MintNftCollection as Program<MintNftCollection>;

  it("Is initialized!", async () => {
    // Add your test here.
    const provider = anchor.AnchorProvider.local();
    anchor.setProvider(provider);

    const program = anchor.workspace.MintNftCollection;
    const mintKeypair = anchor.web3.Keypair.generate();

    const tokenAccount = await anchor.web3.PublicKey.findProgramAddressSync(
      [mintKeypair.publicKey.toBuffer(), TOKEN_PROGRAM_ID.toBuffer()],
      program.programId
    );

    await program.rpc.mintNft("metadata_uri", {
      accounts: {
        mint: mintKeypair.publicKey,
        tokenAccount: tokenAccount[0],
        authority: provider.wallet.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [mintKeypair],
    });
  });
});
