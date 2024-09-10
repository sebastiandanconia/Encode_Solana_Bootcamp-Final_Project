use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("J4S8hv9YmzqTsxpv4PWzn4DwMx6o5k3kCAhyQrmRKXcE");


//type Cookie = u32;

/*
#[derive(BorshSerialize, BorshDeserialize)]
enum Outcome {
    OutcomeA,
    OutcomeB
}
*/

// Anchor hates type aliases. I don't know why.
//type Outcome = u32;


#[program]
pub mod onchain {
    use super::*;

    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    
/*
    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        ctx.user_holdings.a = 0;
        ctx.user_holdings.b = 0;
    }

    pub fn buy(ctx: Context<Buy>, outcome: Outcome, num: uint128) -> Result<()> {
        match outcome {
            A => ctx.user_holdings.a += num,
            B => ctx.user_holdings.b += num,
        }
        Ok(())
    }
    */
    pub fn init_user_and_buy(ctx: Context<InitUserAndBuy>,
        outcome: u32, num: u128) -> Result<()> {
            ctx.accounts.user_holdings.a = 0;
            ctx.accounts.user_holdings.b = 0;
            match char::from_u32(outcome).unwrap() {
                'A' => ctx.accounts.user_holdings.a = num,
                'B' => ctx.accounts.user_holdings.b = num,
                _ => ()
                }
        Ok(())
    }
    

    pub fn balance(ctx: Context<CBalance>) -> Result<(u128, u128)> {
        Ok((ctx.accounts.user_holdings.a, ctx.accounts.user_holdings.b))
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


#[derive(Accounts)]
pub struct InitUserAndBuy<'info> {
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

#[derive(Accounts)]
pub struct CBalance<'info> {
    //#[account]
    pub user_holdings: Account<'info, UserHoldings>,

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

/*
#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(
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
}
*/


#[account]
pub struct ContractScoreboard {
    // Think of this like `static' writeable in C --- One copy for the contract,
    // regardless of the number of markets.
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