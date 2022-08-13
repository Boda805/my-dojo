// import * as anchor from "@project-serum/anchor";
// import { Program } from "@project-serum/anchor";
// import { MyDojo } from "../target/types/my_dojo";

// describe("my-dojo", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.MyDojo as Program<MyDojo>;

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   });
// });

import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { MyDojo } from '../target/types/my_dojo';
import { expect } from 'chai';

describe('my_dojo', async() => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MyDojo as Program<MyDojo>;

  it('Add a dojo', async () => {
    const [myDojoPDA, bump] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("my-dojo"),
          provider.wallet.publicKey.toBuffer()
        ],
        program.programId
      );

    await program.methods
      .addDojo("Next Level Combat", "Woodbury, MN", "No-gi 10th planet subsidiary")
      .accounts({
        dojoOwner: provider.wallet.publicKey,
        myDojo: myDojoPDA,
        // systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    expect((await program.account.myDojo.fetch(myDojoPDA)).name).to.equal("Next Level Combat");
    expect((await program.account.myDojo.fetch(myDojoPDA)).bump).to.equal(bump);
  });
});