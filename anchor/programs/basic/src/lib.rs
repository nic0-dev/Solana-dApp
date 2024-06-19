use anchor_lang::prelude::*;

declare_id!("Cwad3e1aFEBvXNgjQnuTgPe9TGomSTmfQAD8UCCMVrJy");

#[program]
pub mod basic {
    use super::*;

    pub fn greet(_ctx: Context<Initialize>) -> Result<()> {
        msg!("GM!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
