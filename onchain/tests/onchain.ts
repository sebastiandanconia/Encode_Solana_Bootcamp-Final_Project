import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Onchain } from "../target/types/onchain";

describe("onchain", () => {
  // Configure the client to use the local cluster.
  console.log(anchor.AnchorProvider.env());
  anchor.setProvider(anchor.AnchorProvider.env());

  console.log(Onchain);
  const program = anchor.workspace.Onchain as Program<Onchain>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Blob Test", async () => {
    const outcome = new u64(65);
    const value = new u64(5);
    
    let buy1 = await program.methods.initUserAndBuy(outcome, value).rpc();
/*
    let read1 = await program.methods.balance().rpc();
    console.log("Account balance: {}\t", read1);
*/
  });
});
