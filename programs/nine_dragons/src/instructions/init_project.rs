use anchor_lang::prelude::*;

use crate::states::{CodeList, Project};

pub fn init_project(
    ctx: Context<InitProject>,
    price: u64,
    seller_fee_basis_points: u16,
    is_mutable: bool,
) -> Result<()> {

    let project = &mut ctx.accounts.project;

    project.nonce = 0;
    project.price = price;
    project.seller_fee_basis_points = seller_fee_basis_points;
    project.recipient = ctx.accounts.recipient.key();
    project.owner = ctx.accounts.owner.key();
    project.original_owner = ctx.accounts.owner.key();
    project.update_authority = ctx.accounts.update_authority.key();
    project.pending_owner = Some(project.owner);
    project.is_mutable = is_mutable;
    project.operator = ctx.accounts.operator.key();
    project.codes = ctx.accounts.codes.key();

    msg!("Initialize project successful");

    Ok(())
}

#[derive(Accounts)]
pub struct InitProject<'info> {
    #[account(mut)]
    owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [Project::PROJECT_SEED_PREFIX],
        bump,
        space = 8 + Project::INIT_SPACE
    )]
    project: Account<'info, Project>,

    recipient: SystemAccount<'info>,

    operator: SystemAccount<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [Project::CODES_SEED_PREFIX],
        bump,
        space = 8 + CodeList::INIT_SPACE
    )]
    codes: Account<'info, CodeList>,

    /// CHECK: PDA account
    #[account(
        init,
        payer = owner,
        seeds = [Project::AUTHORITY_SEED_PREFIX],
        bump,
        space = 100
    )]
    update_authority: UncheckedAccount<'info>,

    system_program: Program<'info, System>,
}
