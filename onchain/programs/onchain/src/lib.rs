use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("J4S8hv9YmzqTsxpv4PWzn4DwMx6o5k3kCAhyQrmRKXcE");

#[program]
pub mod onchain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}







#[derive(Accounts)]
pub struct Initialize {}

/* Naive implementation
pub struct Initialize<'info> {

    #[account(init,
            payer = signer,
            space = size_of::<Val>() + 8,
            seeds = [&key.to_be_bytes().as_ref()],
            bump)]
    val: Account<'info, Val>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[account]
pub struct Val {
    value: u64,
}
*/

#[derive(Accounts)]
pub struct InitMarket {
    // We probably want to support more than one market off of the same contract,
    // which means using a PDA for the Market account.
    // What do we want to use for the Market seed? Is there a nonce associated with the
    // creating wallet?

    /* Example code from lecture notes. Where does the "lottery" variable live?:
    #[account(init, 
    seeds = [
        &lottery.count.to_be_bytes(), 
        lottery.key().as_ref()
    ], 
    bump, 
    payer = player, 
    space=80)
    ]    
     */

    /*
    #[account(mut)]
    pub authorized: Signer<'info>,
    pub oracle: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    */
}

#[derive(Accounts)]
pub struct ResolveMarket {
    // TODO
}


// TODO: Does init need to be a separate Solana Instruction from Buy?
#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(init,
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
pub struct ContractScoreboard {
    // Think of this like `static' writeable in C --- One copy for the contract
    counter: u64,
}

#[account]
pub struct Market {
    id: u64,
    a_shares_outstanding: u128,
    b_shares_outstanding: u128,

    bump: u8,
}

#[account]
pub struct UserHoldings {
    // Holdings
    a: u128,
    b: u128,

    bump: u8, // Figure out why the example does this
}