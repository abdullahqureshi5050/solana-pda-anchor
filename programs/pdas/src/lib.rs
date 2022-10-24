use anchor_lang::prelude::*;

declare_id!("5FDj86XKQNuuxi4StFD8KUFep71eajy4kMD9fTvKZt8a");

#[program]
pub mod pdas {
    use super::*;

    pub fn create_ledger(
        ctx: Context<CreateLedger>,
        //counter: u32,
    ) -> Result<()> {
        let ledger_account = &mut ctx.accounts.ledger_account;
        //ledger_account.counter = counter;
        
        ledger_account.count = 0;
        ledger_account.timebound_stage_count = 0;
        ledger_account.free_stage_count = 0;
        ledger_account.limited_stage_count = 0;
        ledger_account.public_stage_count = 0;

        msg!("count = {} limited: {} public: {} ", ledger_account.count, ledger_account.limited_stage_count, ledger_account.public_stage_count);
        Ok(())
    }

    // pub fn mint_nft(
    //     //ctx: Context<>,
    // ) -> Result<> {

    //      Ok(());
    // }
    
    pub fn modify_ledger(
        ctx: Context<ModifyLedger>,
    ) -> Result<()> {

        let ledger_account = &mut ctx.accounts.ledger_account;
        
        if ledger_account.free_stage_count < 1 {
            ledger_account.free_stage_count +=1;
            ledger_account.count += 1;
         }
 
         else if ledger_account.limited_stage_count < 2 {
             ledger_account.limited_stage_count += 1;
             ledger_account.count += 1;
         }
 
         else if ledger_account.public_stage_count < 6 {
             ledger_account.public_stage_count += 1;
             ledger_account.count += 1;
         }
         else {
         msg!("Maximim NFTs minted");
         }
         msg!("count = {} limited: {} public: {} ", ledger_account.count, ledger_account.limited_stage_count, ledger_account.public_stage_count);

        Ok(())
    }
}

#[derive(Accounts)]
// #[instruction(color: String)]
pub struct CreateLedger<'info> {
    #[account(
        init_if_needed,
        payer = wallet,
        space = 82,
        seeds = [
            //wallet.key().as_ref(),
            b"new_seed",
            //color.as_ref(),
        ],
        bump
    )]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyLedger<'info> {
    #[account(mut)]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
}

#[account]
pub struct Ledger {
    //max minted
    pub count: u32, 
    //max 100 
    pub free_stage_count: u32,
    //max 200 0.2 SOL
    pub limited_stage_count: u32,
    //max time 0.4 SOL
    pub timebound_stage_count: u32,
    //0.6 SOL each
    pub public_stage_count: u32,
}