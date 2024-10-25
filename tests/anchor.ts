import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import type { TodoList } from "../target/types/todo_list";

describe("Test", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TodoList as anchor.Program<TodoList>;
  
  it("initialize", async () => {
    // Generate keypair for the new account
    const newAccountKp = new web3.Keypair();

    const [userAccountPDA, bump] =
      await anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user_account"), program.provider.publicKey.toBuffer()],
        program.programId
      );

    // Send transaction

    const txHahInizialize = await program.methods
      .inizializeUserList()
      .accounts({
        userAccount: userAccountPDA,
        user: program.provider.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([program.provider.wallet.payer])
      .rpc();
    
    console.log(`Transaction success -> ${txHahInizialize}`);
  });
  it("Add element user List", async () => {
    const [userAccountPDA, bump] =
      await anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user_account"), program.provider.publicKey.toBuffer()],
        program.programId
      );

    const txHashUpdateList = await program.methods
      .addElementUserList(
        "Studiare Rust",
        "devo impar/are a fare i test  e se possibile usare il slp library oggi",
        "18/10/24"
      )
      .accounts({
        userAccount: userAccountPDA,
        user: program.provider.publicKey,
      })
      .signers([program.provider.wallet.payer])
      .rpc();
  });

  it("Read element in Account", async () => {
    const [userAccountPDA, bump] =
      await anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user_account"), program.provider.publicKey.toBuffer()],
        program.programId
      );

    const userAccountData = await program.account.userList.fetch(
      userAccountPDA
    );

    console.log(`
        Titolo: ${userAccountData.titles[0]}
    
        Descrizione: ${userAccountData.descriptions[0]}
    
        Dates: ${userAccountData.dates[0]}
    
        Status: ${userAccountData.status[0]}
    
    `);
  });
  it("Turn Off element", async () => {
    const [userAccountPDA, bump] =
      await anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user_account"), program.provider.publicKey.toBuffer()],
        program.programId
      );

    const txHashTurnOffelement = await program.methods
      .turnOffElement("Studiare Rust")
      .accounts({
        userAccount: userAccountPDA,
        user: program.provider.publicKey,
      })
      .signers([program.provider.wallet.payer])
      .rpc();
    
    console.log(txHashTurnOffelement);
    const userAccountData = await program.account.userList.fetch(
      userAccountPDA
    );

    console.log(userAccountData.status[0]);
  });

  it("delete one element", async () => {
    const [userAccountPDA, bump] =
      await anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user_account"), program.provider.publicKey.toBuffer()],
        program.programId
      );

    const txHashDeletAccount = await program.methods
      .deleteElement("Studiare Rust")
      .accounts({
        userAccount: userAccountPDA,
        user: program.provider.publicKey,
      })
      .signers([program.provider.wallet.payer])
      .rpc();

    console.log(txHashDeletAccount);
  });
});
