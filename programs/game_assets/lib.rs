use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};
declare_id!("GameAssEt11111111111111111111111111111111111");

#[program]
pub mod game_assets {
    use super::*;

    pub fn mint_nft_item(ctx: Context<MintNft>, name: String, uri: String) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
        asset.kind = AssetKind::Item;
        asset.owner = ctx.accounts.authority.key();
        asset.mint = ctx.accounts.mint.key();
        // Simplified: mint 1 token to recipient ATA
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient_ata.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;
        asset.metadata_uri = uri;
        asset.name = name;
        Ok(())
    }

    pub fn mint_land(ctx: Context<MintNft>, world: String, coords: String, uri: String) -> Result<()> {
        let name = format!("Land {} @ {}", world, coords);
        mint_nft_item(ctx, name, uri)
    }

    pub fn mint_currency(ctx: Context<MintCurrency>, amount: u64) -> Result<()> {
        // Mint fungible tokens to dest
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.dest_ata.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn mint_achievement(ctx: Context<MintNft>, title: String, uri: String) -> Result<()> {
        // Mint and mark as soulbound (flag in account)
        let asset = &mut ctx.accounts.asset;
        asset.kind = AssetKind::Achievement;
        asset.owner = ctx.accounts.authority.key();
        asset.mint = ctx.accounts.mint.key();
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient_ata.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;
        asset.metadata_uri = uri;
        asset.name = title;
        asset.soulbound = true;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AssetKind { Currency, Item, Land, Achievement }

#[account]
pub struct Asset {
    pub kind: AssetKind,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub name: String,
    pub metadata_uri: String,
    pub soulbound: bool,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, space = 8 + 1 + 32 + 32 + 4 + 64 + 4 + 256 + 1 + 1, seeds = [b"asset", mint.key().as_ref()], bump)]
    pub asset: Account<'info, Asset>,

    #[account(mut)]
    pub mint: Signer<'info>,
    pub mint_authority: Signer<'info>,

    #[account(mut)]
    pub recipient_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintCurrency<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub mint_authority: Signer<'info>,
    #[account(mut)]
    pub dest_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
