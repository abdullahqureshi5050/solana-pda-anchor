use anchor_lang::prelude::*;

declare_id!("4zf7ZLaP15VP4rTkFWaHxQwjdn8CnKEZJPYxN7SQD6jQ");

#[program]
pub mod pdas {
    use super::*;

    pub fn create_ledger(
        ctx: Context<CreateLedger>,
        //counter: u32,
    ) -> Result<()> {
        let ledger_account = &mut ctx.accounts.ledger_account;
        //ledger_account.counter = counter;
        ledger_account.balance = 0;
        ledger_account.count += 2;
        msg!("balance = {}", ledger_account.balance);
        Ok(())
    }

    // pub fn modify_ledger(
    //     ctx: Context<ModifyLedger>,
    //     new_balance: u32,
    // ) -> Result<()> {

    //     let ledger_account = &mut ctx.accounts.ledger_account;
    //     ledger_account.balance = new_balance;
        
    //     Ok(())
    // }
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

// #[derive(Accounts)]
// pub struct ModifyLedger<'info> {
//     #[account(mut)]
//     pub ledger_account: Account<'info, Ledger>,
//     #[account(mut)]
//     pub wallet: Signer<'info>,
// }

#[account]
pub struct Ledger {
    pub count: u32,
    pub balance: u32,
}