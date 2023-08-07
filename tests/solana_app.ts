import * as anchor from "@coral-xyz/anchor";
import * as token from "@solana/spl-token";
import { Program } from "@coral-xyz/anchor";
import { LootboxApp } from "../target/types/lootbox_app";
import { StakingApp } from "../target/types/staking_app";
import { setupNft } from "./utils/setUpNft";
import { PROGRAM_ID as METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata"
import { expect } from "chai";


describe("solana_app", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.StakingApp as Program<StakingApp>;
  const program_loot = anchor.workspace.LootboxApp as Program<LootboxApp>;

  const wallet = anchor.workspace.StakingApp.provider.wallet

  let [stake_mint] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("mint")], program.programId)
  let user_ata = token.getAssociatedTokenAddressSync(stake_mint, wallet.payer.publicKey);
  let [loot_pda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("lootbox"), wallet.payer.publicKey.toBuffer()], program_loot.programId);

  before(async () => {
    ;({ nft, delegatedAuthPda, stakeStatePda, mint, mintAuth, tokenAddress } =
      await setupNft(program, wallet.payer))
  })

  let delegatedAuthPda: anchor.web3.PublicKey
  let stakeStatePda: anchor.web3.PublicKey
  let nft: any
  let mintAuth: anchor.web3.PublicKey
  let mint: anchor.web3.PublicKey
  let tokenAddress: anchor.web3.PublicKey

  it("Initializing Mint", async () => {
    console.log("Mint Address : ", stake_mint.toString())
    // Add your test here.
    const tx = await program.methods.initializeMint().rpc();
    console.log("Your transaction signature", tx);
  });

  it("StakingNFT", async() => {
    // let [pda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(wallet.PublicKey), nft.tokenAddress.toBuffer()], program.programId);
    // let [nftMasterEdition] = anchor.web3.PublicKey.findProgramAddressSync("")
    const tx = await program.methods.stakeNft().accounts({
      nftTokenAcc: nft.tokenAddress,
      nftMint: nft.mintAddress,
      nftEdition: nft.masterEditionAddress,
      metaProgram: METADATA_PROGRAM_ID,
    }).rpc();

    let stake_pda = await program.account.stakingPda.fetch(stakeStatePda);
    console.log(stake_pda.stakingStatus, `https://explorer.solana.com/tx/${tx}?cluster=devnet`);
    expect(stake_pda.stakingStatus === "Staked");
  })

  it("Redeeem Stake Token", async () => {

    let tx = await program.methods.redeemToken().accounts({
      nftMint: nft.mintAddress,
      nftTokenAcc: nft.tokenAddress,
      userAta: user_ata
    }).rpc()
    console.log(`reddem tx https://explorer.solana.com/tx/${tx}?cluster=devnet`)
    let amount = (await token.getAccount(program.provider.connection, user_ata)).amount
    console.log("AMount : ", amount);

    // expect(amount.toString() == "10");
  })

  it("Unstake", async() => {
    try{
      const tx = await program.methods.unstakeNft().accounts({
        nftMint: nft.mintAddress,
        nftEdition: nft.masterEditionAddress,
        nftTokenAcc: nft.tokenAddress,
        userAta: user_ata,
        metaProgram: METADATA_PROGRAM_ID
      }).rpc();
      console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
      const account = await program.account.stakingPda.fetch(stakeStatePda);
      expect(account.stakingStatus === "Unstaked")
    }
    catch(error){
      console.log(error)
    }
   

  });


  it("Opening lootbox", async () => {
    console.log("the PDA lootbox is ", loot_pda.toString());
    // Add your test here.
    const tx = await program_loot.methods
    .openLootbox()
    .accounts({
      mint: stake_mint,
      userAta: user_ata
    }).rpc();
    console.log("Your transaction signature", tx);
    const account = await program_loot.account.lootboxPointer.fetch(loot_pda);
    console.log("The mint will be : ", account.mint)
  });

  it("Redeeming lootbox", async() => {
    let account = await program_loot.account.lootboxPointer.fetch(loot_pda);
    const user_ata = await token.getAssociatedTokenAddressSync(account.mint, wallet.payer.publicKey);
    const tx = await program_loot.methods.redeemLootbox()
    .accounts({
      mint: account.mint,
      userMintAta: user_ata
    }).rpc()

    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`)

    account = await program_loot.account.lootboxPointer.fetch(loot_pda);
    console.log((await token.getAccount(provider.connection, user_ata)).amount)
    expect(account.claimed == "Claimed")

  })
    
});
