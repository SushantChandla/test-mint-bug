import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MintBug } from "../target/types/mint_bug";

describe("mint-bug", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MintBug as Program<MintBug>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
