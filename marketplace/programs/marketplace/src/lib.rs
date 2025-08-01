#![allow(unexpected_cfgs, deprecated)]

use anchor_lang::prelude::*;

mod state;
use state::*;

mod context;
use context::*;


declare_id!("GUYL5GA2ederBh1MhEDSjz11SX5DhMMc3FxZ1aaQRhWN");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;
        Ok(())
    }

    pub fn listing(ctx: Context<List>, price: u64) ->Result<()>{
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit()?;
        Ok(())
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.delist()?;
        ctx.accounts.close_mint_vault()?;
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_sol()?;
        ctx.accounts.receive_nft()?;
        ctx.accounts.receive_rewards()?;
        ctx.accounts.close_mint_vault()?;
        Ok(())
    }
}
