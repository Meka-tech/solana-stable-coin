use anchor_lang::prelude::*;
use anchor_spl::{token_2022::{mint_to, MintTo, Token2022}, token_interface::{Mint, TokenAccount}};

use crate::constants::SEED_CONFIG_ACCOUNT;


pub fn mint_tokens<'info>(
     mint_account: &InterfaceAccount<'info, Mint>,
    token_program: &Program<'info, Token2022>, 
    token_account: &InterfaceAccount<'info, TokenAccount>,
    bump :u8 , 
    amount : u64,
   
) -> Result<()> {


    let signer_seeds: &[&[&[u8]]] = &[&[SEED_CONFIG_ACCOUNT, &[bump]]];


    let accounts = MintTo{
        mint: mint_account.to_account_info(),
        to: token_account.to_account_info(),
        authority: mint_account.to_account_info(),
    };




    let cpi_ctx = CpiContext::new_with_signer(token_program.to_account_info(), accounts, signer_seeds);

    mint_to(cpi_ctx, amount)?;

    Ok(())
}