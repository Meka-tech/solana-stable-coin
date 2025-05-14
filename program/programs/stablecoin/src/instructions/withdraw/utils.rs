use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{token_2022::{burn, Burn}, token_interface::{ Mint, Token2022, TokenAccount}};

use crate::constants::SEED_SOL_ACCOUNT;


pub fn withdraw_sol<'info>(
    depositor_key : &Pubkey,
    amount : u64,
    bump : u8,
    system_program : &Program<'info, System>,
    from: &SystemAccount<'info>,
    to: &AccountInfo<'info>,
) -> Result<()> {

    let signer_seeds: &[&[&[u8]]] = &[&[SEED_SOL_ACCOUNT, depositor_key.as_ref(), &[bump]]];

    let cpi_ctx = CpiContext::new_with_signer(system_program.to_account_info(), Transfer{from: from.to_account_info(), to: to.to_account_info()}, signer_seeds);

    transfer(cpi_ctx, amount)?;

 
    Ok(())
}



pub fn burn_tokens<'info>(
    amount : u64,
    token_program : &Program<'info, Token2022>,
    token_account : &InterfaceAccount<'info, TokenAccount>,
    mint_account : &InterfaceAccount<'info, Mint>,
    authority : &Signer<'info>,
) -> Result<()> {

    let cpi_ctx = CpiContext::new(token_program.to_account_info(), Burn{mint: mint_account.to_account_info(), from: token_account.to_account_info(), authority: authority.to_account_info()});


    burn(cpi_ctx, amount)?;

    Ok(())
}

