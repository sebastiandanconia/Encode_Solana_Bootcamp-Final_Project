// No imports needed: web3, anchor, pg and more are globally available

describe("Test", () => {

  it("buy", async () => {
    const [userAccount, userAccountBump] =
      web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user-holdings-v1"), pg.wallet.publicKey.toBuffer()],

        pg.program.programId
      );

    console.log(`Derived userAccount = ${userAccount}`);

    // Read account balance
    let userAccountData = await pg.program.account.userHoldings.fetch(
      userAccount
    );
    console.log(`Beginning account balance: {}`, userAccountData.a.toNumber());


    const txHash = await pg.program.methods
      .buy(new anchor.BN(20))
      .accounts({
        userHoldings: userAccount,
        signer: pg.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);

    // Reread account balance
    userAccountData = await pg.program.account.userHoldings.fetch(userAccount);
    console.log(`Final account balance: {}`, userAccountData.a.toNumber());
  });
});
