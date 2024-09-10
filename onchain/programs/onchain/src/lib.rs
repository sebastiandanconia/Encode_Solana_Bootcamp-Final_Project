use anchor_lang::prelude::*;

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
