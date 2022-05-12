use anchor_lang::prelude::*;

declare_id!("H649eEsXhsBggRq7jULojR1qf5igYMUHMjZYqafDeRMH");
// declare_id!("FwAQwZfCdjMmJjftbvcELYTbH6Xg1sadDTPe9d7MyABq");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
