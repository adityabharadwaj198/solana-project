const anchor = require('@project-serum/anchor');

const {SystemProgram} = anchor.web3;

const main = async() => {
  console.log("Starting test")
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Myepicproject;

  const baseAccount = anchor.web3.Keypair.generate();

  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  })

  console.log("Your transaction signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('GIF Count', account.totalGifs.toString())

  // Call add_gif!
  await program.rpc.addGif("https://c.tenor.com/_tMorpHF7hQAAAAd/fade-meme.gif",
  {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  await program.rpc.upvoteGif("https://c.tenor.com/_tMorpHF7hQAAAAd/fade-meme.gif", 
  {
    accounts: {
      baseAccount: baseAccount.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('GIF Count', account.totalGifs.toString())
  console.log('GIF list', account.gifList);
  console.log('UserAddress', account.gifList[0].userAddress)
}

const runMain = async() => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();