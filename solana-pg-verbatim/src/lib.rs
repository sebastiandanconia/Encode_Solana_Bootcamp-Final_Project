use anchor_lang::prelude::*;

use std::mem::size_of;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("2Fr2VCW2fgw7nkfCCBpq67s9J3YCYtmv4TfskJeREAsi");

#[program]
mod hello_anchor {
    use super::*;

    pub fn buy(ctx: Context<Buy>, num: u64) -> Result<()> {
        // Deduct payment

        // Credit shares
        ctx.accounts.user_holdings.a += num;
        Ok(())
    }

}


#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(init_if_needed,
    payer = authority,
    space = size_of::<UserHoldings>() + 8,
    seeds = [
            b"user-holdings-v1",
            authority.key().as_ref()],
    bump
    )]
    pub user_holdings: Account<'info, UserHoldings>,

    #[account(mut)]
    authority: Signer<'info>,

    system_program: Program<'info, System>,
}


#[account]
pub struct UserHoldings {
    // Holdings
    a: u64,
    b: u64,
}
