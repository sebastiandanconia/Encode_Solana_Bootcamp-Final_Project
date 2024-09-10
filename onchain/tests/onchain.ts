import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
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

/*
    let buy1 = await program.methods.initUserAndBuy('A', 5).rpc();

    let read1 = await program.methods.balance().rpc();
    console.log("Account balance: {}\t", read1);
*/
  });
});
