import * as anchor from '@project-serum/anchor';
import * as splToken from "@solana/spl-token";

import { Program, Wallet } from '@project-serum/anchor';
import { Transaction, PublicKey } from '@solana/web3.js';
import { MyDojo } from '../target/types/my_dojo';
import { expect } from 'chai';

describe('my_dojo', async() => {
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as Wallet;
  anchor.setProvider(provider);

  const program = anchor.workspace.MyDojo as Program<MyDojo>;

  const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  console.log("Mint Key: ", mintKey.publicKey.toString());

  const { SystemProgram, } = anchor.web3;
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  let metadataAddress;
  let beltMint;
  let nftTokenAccount;
  let mintAuthorityPda;
  let mintAuthorityPdaBump;
  let myDojoPda

  before(async() => {
    // Mint Address
    beltMint = await splToken.createMint(
      provider.connection,
      wallet.payer,
      wallet.publicKey,
      null,
      0,
      undefined,
      undefined,
      splToken.TOKEN_PROGRAM_ID
    );
    console.log("Belt Mint Account: ", beltMint.toString());

    [mintAuthorityPda, mintAuthorityPdaBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("mint_authority_"),
        beltMint.toBuffer(),
      ],
      program.programId,
    );

    let transferMintAuthorityTransaction = new Transaction()
      .add(splToken.createSetAuthorityInstruction(
        beltMint,
        wallet.publicKey,
        splToken.AuthorityType.MintTokens,
        mintAuthorityPda
    ));
    await anchor.web3.sendAndConfirmTransaction(
      provider.connection, 
      transferMintAuthorityTransaction, 
      [wallet.payer]
    );
    
    console.log("Mint Authority PDA: ", mintAuthorityPda.toString());
    console.log("mintAuthorityPdaBump: ", mintAuthorityPdaBump.toString());

    // Wallet ATA for belt nft
    nftTokenAccount = await splToken.createAccount(
      provider.connection,
      wallet.payer,
      beltMint,
      wallet.publicKey
    );
    console.log("Wallet ATA: ", nftTokenAccount.toString());

    // Derive metadata address
    metadataAddress = (await anchor.web3.PublicKey.findProgramAddress(
     [
       Buffer.from("metadata"),
       TOKEN_METADATA_PROGRAM_ID.toBuffer(),
       beltMint.toBuffer(),
     ],
     TOKEN_METADATA_PROGRAM_ID
    ))[0];
    console.log("Metadata Address: ", metadataAddress.toString());

    [myDojoPda] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("my-dojo"),
          provider.wallet.publicKey.toBuffer()
        ],
        program.programId
      );
    console.log("My dojo pda: ", myDojoPda)
  });
  
  it('Add a dojo', async () => {
    await program.methods
      .addDojo("Top Tier Gym", "USA", "Best gym in the states, owned by John Smith")
      .accounts({
        dojoOwner: provider.wallet.publicKey,
        myDojo: myDojoPda,
        // systemProgram: anchor.web3.SystemProgram.programId,
      }).rpc();

    expect((await program.account.myDojo.fetch(myDojoPda)).name).to.equal("Top Tier Gym");
  });

  it('Mint a black belt for dojo owner', async () => {
    const tx = await program.methods.mintBlackBelt(
      "https://arweave.net/y5e5DJsiwH0s_ayfMwYk-SnrZtVZzHLQDSTZ5dNRUHA",
      "John Smithington",
      mintAuthorityPdaBump)
      .accounts(
      {
        metadataAccount: metadataAddress,
        mintAccount: beltMint,
        mintAuthority: mintAuthorityPda,
        payer: wallet.publicKey,
        tokenAccount: nftTokenAccount,
        myDojo: myDojoPda,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: SystemProgram.programId,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      }).signers([wallet.payer]).rpc();

    console.log("Your transaction signature", tx)
  });
});