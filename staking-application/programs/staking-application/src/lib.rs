use anchor_lang::solana_program::sysvar::stake_history;
use anchor_lang::{
    prelude::*,
    solana_program::stake::{
        self,
        state::{Authorized, Lockup, StakeStateV2},
    },
};
use anchor_spl::stake::{ Stake};

declare_id!("7TEsf66Poea6PaLMCZiaA84vX6au4t5Bj6rLbLeWZyon");

#[program]
pub mod staking_application {
    use super::*;

    pub fn initialize_stake(ctx: Context<InitializeStake>) -> Result<()> {
        let ix: anchor_lang::solana_program::instruction::Instruction = stake::instruction::initialize(
            &ctx.accounts.stake_account.key(),
            &Authorized {
                staker: ctx.accounts.stake_authority.key(),
                withdrawer: ctx.accounts.stake_authority.key(),
            },
            &Lockup::default(),
        );
        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.stake_program.to_account_info(),
                ctx.accounts.stake_account.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
            &[],
        )?;

        Ok(())
    }

    pub fn delegate_stake(ctx: Context<DelegateStake>) -> Result<()> {

        let ix = stake::instruction::delegate_stake(
            &ctx.accounts.stake_account.key(),
            &ctx.accounts.stake_authority.key(),
            &ctx.accounts.validator_vote.key(),
        );
        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[
                ctx.accounts.stake_program.to_account_info(),
                ctx.accounts.stake_account.to_account_info(),
                ctx.accounts.stake_authority.to_account_info(),
                ctx.accounts.validator_vote.to_account_info(),
                ctx.accounts.clock.to_account_info(),
                ctx.accounts.stake_history.to_account_info(),
                ctx.accounts.stake_config.to_account_info(),
            ],
            &[&[b"stake_authority", &[ctx.bumps.stake_authority]]],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeStake<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: This account will be initialized as a Stake account by the Stake program
    #[account(
        init,
        payer = owner,
        space = std::mem::size_of::<StakeStateV2>(),
        owner = stake::program::ID,
    )]
    pub stake_account: AccountInfo<'info>,
    /// CHECK:: Account ownership not checked.
    #[account(
        seeds = [b"stake_authority"],
        bump
    )]
    pub stake_authority: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub stake_program: Program<'info, Stake>,
}

#[derive(Accounts)]
pub struct DelegateStake<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Unchecked Account
    #[account(mut)]
    pub stake_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub stake_program: Program<'info, Stake>,
    /// CHECK: Unchecked Account
    #[account(address = stake_history::ID)]
    pub stake_history: UncheckedAccount<'info>,
    /// CHECK: Unchecked Account
    #[account(address = stake::config::ID)]
    pub stake_config: UncheckedAccount<'info>,
    /// CHECK: Unchecked Account
    #[account(
        seeds = [b"stake_authority"],
        bump
    )]
    pub stake_authority: UncheckedAccount<'info>,
    /// CHECK: Unchecked Account
    #[account(mut)]
    pub validator_vote: UncheckedAccount<'info>,
    pub clock: Sysvar<'info, Clock>,
}
