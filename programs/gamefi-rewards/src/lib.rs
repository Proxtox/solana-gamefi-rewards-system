use anchor_lang::prelude::*;

declare_id!("GameFiRewardsProgram111111111111111111111111111");

#[program]
pub mod gamefi_rewards {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("GameFi Rewards program initialized");
        Ok(())
    }

    pub fn distribute_reward(ctx: Context<DistributeReward>, amount: u64) -> Result<()> {
        let pool = &mut ctx.accounts.reward_pool;
        pool.total_distributed = pool.total_distributed.checked_add(amount).unwrap();
        msg!("Distributed {} rewards to players", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub reward_pool: Account<'info, RewardPool>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DistributeReward<'info> {
    #[account(mut)]
    pub reward_pool: Account<'info, RewardPool>,
    pub user: Signer<'info>,
}

#[account]
pub struct RewardPool {
    pub total_distributed: u64,
}