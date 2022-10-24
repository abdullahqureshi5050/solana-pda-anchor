use anchor_lang::prelude::*;
use anchor_lang::prelude::Clock;

declare_id!("5FDj86XKQNuuxi4StFD8KUFep71eajy4kMD9fTvKZt8a");

#[program]
pub mod pdas {
    use super::*;

    pub fn create_ledger(
        ctx: Context<CreateLedger>,
        current_sale: u8
    ) -> Result<()> {
        let ledger_account = &mut ctx.accounts.ledger_account;
        //ledger_account.counter = counter;
        let clock = Clock::get()?;

        ledger_account.count = 0;
        ledger_account.timebound = clock.unix_timestamp;
        ledger_account.timebound_stage_count = 0;
        ledger_account.free_stage_count = 0;
        ledger_account.limited_stage_count = 0;
        ledger_account.public_stage_count = 0;
        ledger_account.current_sale = current_sale;

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
        
        if ledger_account.current_sale == 0 && ledger_account.free_stage_count < 100 {
            ledger_account.free_stage_count +=1;
            ledger_account.count += 1;

            if ledger_account.free_stage_count > 100 - 1 {
                ledger_account.current_sale +=1;
            }
         }
 
         else if ledger_account.current_sale == 1 && ledger_account.limited_stage_count < 200 {
             ledger_account.limited_stage_count += 1;
             ledger_account.count += 1;

             if ledger_account.limited_stage_count > 200 - 1 {
                
                ledger_account.current_sale +=1;

                let clock = Clock::get()?;
                let current_timestamp = clock.unix_timestamp;
        
                //allow minting for 10 minutes 
                ledger_account.timebound = current_timestamp + 3 * 60;

            }
         }

          else if ledger_account.current_sale == 2 && Clock::get().unwrap().unix_timestamp < ledger_account.timebound {
             // _price = _price * 3;
             ledger_account.timebound_stage_count +=1;
             ledger_account.count += 1;
          }

         else if ledger_account.count < 6000 {
             ledger_account.public_stage_count += 1;
             ledger_account.count += 1;
         }
         else {
         msg!("Maximim NFTs minted");
         }
         msg!("count = {} limited: {} timebound: {} public: {} ", ledger_account.count, ledger_account.limited_stage_count, ledger_account.timebound, ledger_account.public_stage_count);

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
    pub timebound: i64,
    pub current_sale: u8,
}