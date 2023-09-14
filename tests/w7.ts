import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { W7 } from "../target/types/w7";

describe("w7", () => {

  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.W7 as Program<W7>;

  async function generateKeypair() {
    let keypair = anchor.web3.Keypair.generate();
    await provider.connection.requestAirdrop(
      keypair.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await new Promise( resolve => setTimeout(resolve, 3 * 1000) ); // Sleep 3s
    return keypair;
  }

  it("Is initialized!", async () => {
    // Add your test here.
    const mintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();



    let [metadata_account, _] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        new anchor.web3.PublicKey(
          "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        ).toBuffer(),
        mintKeypair.publicKey.toBuffer(),
      ],
      program.programId
    );

    let wallet = await generateKeypair()
    const tx = await program.methods.createToken(
        "my token",
        "my symbol",
        "my icon",
    ).accounts({
      extMintAccount: metadata_account,
      authAccount: wallet.publicKey,
      mintAccount: mintKeypair.publicKey
    })
    .signers([wallet, mintKeypair])
    .rpc();
    console.log("Your transaction signature", tx);
    let metadata = await program.account.extMint.fetch(metadata_account);
    console.log(metadata)
  });
});
