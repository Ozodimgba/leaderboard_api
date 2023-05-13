use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount};

#[program]
pub mod contest_program {
    use super::*;

    pub fn create_contest(ctx: Context<CreateContest>, name: String, time_range: u64, value: u64, num_participants: u64) -> ProgramResult {
        // Ensure the caller has enough SOL balance
        let sol_account = &ctx.accounts.sol_account;
        let sol_balance = sol_account.try_borrow_mut_spl_token()?;
        let sol_mint = ctx.accounts.sol_mint.load()?;
        if sol_balance.amount < value {
            return Err(ErrorCode::InsufficientBalance.into());
        }

        // Generate a unique contest ID
        let contest_id = ctx.accounts.system_program.create_address_with_seed(&name, &ctx.program_id)?;

        // Create the contest account
        let contest_account = &mut ctx.accounts.contest_account;
        contest_account.name = name;
        contest_account.time_range = time_range;
        contest_account.value = value;
        contest_account.num_participants = num_participants;
        contest_account.leaderboard = contest_id;

        // Mint SPL tokens to the contest account
        token::transfer(
            ctx.accounts.into(),
            value,
            TokenAccount::unpack_from_slice(&sol_mint.data.borrow())?.owner,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateContest<'info> {
    #[account(init)]
    pub contest_account: ProgramAccount<'info, ContestAccount>,
    #[account(mut)]
    pub sol_account: CpiAccount<'info, TokenAccount>,
    #[account(has_one = sol_mint)]
    pub sol_mint: Loader<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct ContestAccount {
    pub name: String,
    pub time_range: u64,
    pub value: u64,
    pub num_participants: u64,
    pub leaderboard: Pubkey,
}

#[error]
pub enum ErrorCode {
    #[msg("Insufficient SOL balance")]
    InsufficientBalance,
}

