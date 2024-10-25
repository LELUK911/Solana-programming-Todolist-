#To-Do List Program on Solana
Overview

This project is a sample implementation of a To-Do List on Solana, using Anchor as a framework to streamline the program development. The program enables users to:

    Initialize a personal To-Do List account
    Add items to the list
    Mark items as completed
    Delete items from the list

Features

    User Management: Each user has a separate To-Do List stored on a Program Derived Address (PDA).
    Simple Interface: The program functions allow adding, updating, and removing items from the list.
    On-Chain Persistence: All data is stored on-chain, ensuring availability and immutability.

Prerequisites

Make sure to have the following tools installed:

    Solana CLI - For managing accounts and deploying the program.
    Anchor - For managing the Solana program.
    Node.js and npm - For running test scripts.

Setup

    Clone the Repository

    bash

git clone https://github.com/LELUK911/Solana-programming-Todolist-.git
cd Solana-programming-Todolist-

Install Dependencies Install the necessary dependencies for the project:

bash

anchor build
npm install

Set the Network Set the network for deployment (e.g., devnet):

bash

solana config set --url https://api.devnet.solana.com

Airdrop Funds Make sure your account has funds by requesting an airdrop on devnet:

bash

    solana airdrop 2

Deploy the Program

    Build and Deploy the Program to Solana

    bash

    anchor deploy

    Get the Program ID After deploying, copy the Program ID and verify it's correctly set in the code.

Using the Program Functions

Once deployed, you can run tests to interact with the program.
Example Commands for Testing

    Initialize the User's To-Do List

    javascript

const txHash = await pg.program.methods
   .inizializeUserList()
   .accounts({
      userAccount: userAccountPDA,
      user: pg.wallet.publicKey,
      systemProgram: web3.SystemProgram.programId,
   })
   .signers([pg.wallet.keypair])
   .rpc();
console.log(`Initialization successful, transaction hash: ${txHash}`);

Add an Item to the To-Do List

javascript

const txHash = await pg.program.methods
   .addElementUserList("Study Rust", "Learn to write tests", "18/10/24")
   .accounts({
      userAccount: userAccountPDA,
      user: pg.wallet.publicKey,
   })
   .signers([pg.wallet.keypair])
   .rpc();

Mark an Item as Completed

javascript

const txHash = await pg.program.methods
   .turnOffElement("Study Rust")
   .accounts({
      userAccount: userAccountPDA,
      user: pg.wallet.publicKey,
   })
   .signers([pg.wallet.keypair])
   .rpc();

Delete an Item

javascript

    const txHash = await pg.program.methods
       .deleteElement("Study Rust")
       .accounts({
          userAccount: userAccountPDA,
          user: pg.wallet.publicKey,
       })
       .signers([pg.wallet.keypair])
       .rpc();

Testing

To test the program functions, use the provided test scripts. Run the tests with:

bash

anchor test

This command will execute a series of tests to verify that the To-Do List program functions correctly.
Project Structure

    programs/todo-list: Contains the Rust code for the Solana program, including the lib.rs file defining the list management logic.
    tests: Contains test scripts written in JavaScript/TypeScript.
    client: Code for the client interface to interact with the program.

Conclusion

This project is a simple exercise demonstrating the development of a To-Do List program on Solana using Anchor. It highlights skills in blockchain programming on Solana. Feel free to reach out for questions or contribute to the project.
