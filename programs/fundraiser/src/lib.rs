use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("HXUDRhP4rdGeNeSXCCg8XeEhcQxyqbqK35aEzeM5wHxN");

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBKEY_LENGTH: usize        = 32;
const U64_LENGTH: usize           = 8;


const MAX_NAME_LENGTH: usize        = 100; 
const MAX_DESC_LENGTH: usize        = 300; 
const STRING_PREFIX_LENGTH: usize   = 4;   
const CAMPAIGN_ACCOUNT_SPACE: usize = 
    DISCRIMINATOR_LENGTH
  + PUBKEY_LENGTH                    
  + (STRING_PREFIX_LENGTH + MAX_NAME_LENGTH)
  + (STRING_PREFIX_LENGTH + MAX_DESC_LENGTH)
  + U64_LENGTH                       
  + 1;                               // bump

#[program]
pub mod fundraiser {
    use super::*;

    pub fn createcampaign(
        ctx: Context<Create>,
        name: String,
        description: String,
    ) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        campaign.admin = *ctx.accounts.user.key;
        campaign.name = name;
        campaign.description = description;
        campaign.amount_donated = 0;
        campaign.bump = ctx.bumps.campaign;
        Ok(())
    }
    pub fn withdrawMoneyfromCamtoUser(ctx:Context<Withdraw>,amount:u64)->ProgramResult{
        let campaign=&mut ctx.accounts.campaign;
        let user=&mut ctx.accounts.user;
        if campaign.admin != *user.key{
            return Err(ProgramError::IncorrectProgramId);
        }
        let rent_balance=Rent::get()?.minimum_balance(campaign.to_account_info().data_len());
        if  **campaign.to_account_info().lamports.borrow()-rent_balance <amount{
           return Err(ProgramError::InsufficientFunds) ;
        }
        **campaign.to_account_info().try_borrow_mut_lamports()?-=amount;
        **user.to_account_info().try_borrow_mut_lamports()?+=amount;
        Ok(())

    }

    pub fn donateTocamFromUSers(ctx:Context<Donate>,amount:u64)->ProgramResult{
        let tsc=anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),//from
            &ctx.accounts.campaign.key(),//to
            amount);
            anchor_lang::solana_program::program::invoke(
                &tsc,
                &[
                    ctx.accounts.user.to_account_info(),
                    ctx.accounts.campaign.to_account_info()
                ]
            );
            (&mut ctx.accounts.campaign).amount_donated+=amount;
            Ok(())


    }
    
}

#[derive(Accounts)]
#[instruction(name: String, description: String)]
pub struct Create<'info> {
    #[account(
        init,
        payer = user,
        space = CAMPAIGN_ACCOUNT_SPACE,
        seeds = [b"CAMPAIGN_DEMO", user.key().as_ref()],
        bump
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Campaign {
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
    pub amount_donated: u64,
    pub bump: u8,
}
