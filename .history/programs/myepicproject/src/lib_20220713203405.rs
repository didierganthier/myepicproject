use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        let base_account= &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff {}
