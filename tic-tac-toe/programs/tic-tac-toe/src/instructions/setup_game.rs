use crate::state::game::*;
use anchor_lang::prelude::*;

pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
    ctx.accounts
        .game
        .start([ctx.accounts.player_one.key(), player_two])
}

#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(init, payer = player_one, space = Game::MAXIMUM_SIZE + 8)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_one: Signer<'info>,
    pub system_program: Program<'info, System>,
}
// look in test setup game test to understand this code!!
// first because I going to pay for all the goodness I use my wallet as signer and it's player_one.
// than in test gameKeypair is created. This is keypair for accout shaped as Game account and it must
// have an address to be accessed. In test agan first instruction is not clear. It looks like first we
// ask for account to run some code, with accounts call we pass required accounts. We use data accout
// to sign resulting transaction and we send it with rpc.