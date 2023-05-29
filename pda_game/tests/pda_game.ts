import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { PublicKey } from '@solana/web3.js'
import { PdaGame } from '../target/types/pda_game'
import { expect } from 'chai'


describe('pda_game', async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program: Program<PdaGame> = anchor.workspace.PdaGame;

  it('Sets and changes name!', async () => {
    const [userStatsPDA, _] = await PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('user-stats'),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    )


    await program.methods
      .createUserStats('brian')
      .accounts({
        user: provider.wallet.publicKey,
        userStats: userStatsPDA,
      })
      .rpc()


    expect((await program.account.userStats.fetch(userStatsPDA)).name).to.equal(
      'brian'
    )


    await program.methods
      .changeUserName('tom')
      .accounts({
        user: provider.wallet.publicKey,
        userStats: userStatsPDA,
      })
      .rpc()


    expect((await program.account.userStats.fetch(userStatsPDA)).name).to.equal(
      'tom'
    )
  })
})