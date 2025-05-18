import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakingApplication } from "../target/types/staking_application";

describe("staking-application", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.stakingApplication as Program<StakingApplication>;
  let stakeAccount,owner;
  before(()=>{
    // Create a new stake account  and owner keypair
    stakeAccount = anchor.web3.Keypair.generate();
    owner = anchor.web3.Keypair.generate();
  })

  it("TEST - 1: Can initialize stake account", async () => {
    
    console.log("Stake Account:", stakeAccount.publicKey.toString());
    console.log("Owner:", owner.publicKey.toString());

    // Airdrop SOL to the account
    await anchor.AnchorProvider.env().connection.requestAirdrop(
      stakeAccount.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );

    // Wait for confirmation
    await new Promise(resolve => setTimeout(resolve, 1000));

    // Derive the stake authority PDA
    const [stakeAuthority, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("stake_authority")],
      program.programId
    );
    console.log("Stake Authority PDA:", stakeAuthority.toString());

    const tx = await program.methods.initializeStake()
      .accounts({
        owner: owner.publicKey,
        stakeAccount: stakeAccount.publicKey,
        stakeAuthority: stakeAuthority,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        stakeProgram: anchor.web3.StakeProgram.programId,  
      })
      .signers([owner, stakeAccount])
      .rpc();

      console.log("Initialize stake account transaction signature:", tx);
  });
});
