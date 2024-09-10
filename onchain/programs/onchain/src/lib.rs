use anchor_lang::prelude::*;

declare_id!("HKLaiMcEwpE13t8C3gNVaN2mMMvd1a4m9qr9GpoXHpo8");

#[program]
pub mod onchain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
