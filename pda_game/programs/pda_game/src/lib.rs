use anchor_lang::prelude::*;

declare_id!("34BxKsRar8Pb5qvuEfV6UqYTHVVdwwSBKbcugmexYcLf");
#[program]
pub mod pda_game {
    use super::*;
    // handler function create user data holder
    pub fn create_user_stats(ctx: Context<CreateUserStats>, name: String) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.level = 0;
        if name.as_bytes().len() > 200 {
            // proper error handling omitted for brevity
            panic!();
        }
        user_stats.name = name;
        user_stats.bump = *ctx.bumps.get("user_stats").unwrap();
        Ok(())
    }

    // change user's name created with handler above
    pub fn change_user_name(ctx: Context<ChangeUserName>, new_name: String) -> Result<()> {
        if new_name.as_bytes().len() > 200 {
            // proper error handling omitted for brevity
            panic!();
        }
        ctx.accounts.user_stats.name = new_name;
        Ok(())
    }
}

#[account]
pub struct UserStats {
    level: u16,
    name: String,
    bump: u8, // why it's needed here??
}
// We need bump in the account because later we need to find the account that holds  this data.
// Have a look in ChangeUserName Contexts initialization code down below. To get account we use
// known constant seeds &str and user.address.

// This Context create account off curve for later to be find with bumps BTreeMap,constant string seed
// and address of user.
#[derive(Accounts)]
pub struct CreateUserStats<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // space: 8 discriminator + 2 level + 4 name length + 200 name + 1 bump
    #[account(
        init,
        payer = user,
        space = 8 + 2 + 4 + 200 + 1,
        seeds = [b"user-stats", user.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,
    pub system_program: Program<'info, System>,
}

// validation struct
#[derive(Accounts)]
pub struct ChangeUserName<'info> {
    pub user: Signer<'info>,
    // reference to user holding info.
    #[account(mut, seeds = [b"user-stats", user.key().as_ref()], bump = user_stats.bump)]
    pub user_stats: Account<'info, UserStats>,
}
