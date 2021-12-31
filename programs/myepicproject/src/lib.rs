use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("69fefy3HdEzo6dpGHMpryoNSiB3VUWanFGScRCaAPAbj");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGifs>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

    	// Build the struct.
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: user.to_account_info().key(),
            upvotes: 0,
            comments: HashMap::new(),
        };

        // Add it to the gif_list vector which keeps track of all the gifs in the program.
        base_account.gif_list.push(item.clone());
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn upvote_gif(ctx: Context<UpvoteGifs>, gif_link: String) -> ProgramResult {
        let gif_list = &mut ctx.accounts.base_account.gif_list;
        for gif_item in gif_list {
            if gif_item.gif_link == gif_link {
                gif_item.upvotes+=1;
            }
        }
        Ok(())
    }

    pub fn downvote_gif(ctx: Context<DownvoteGifs>, gif_link: String) -> ProgramResult {
        let gif_list = &mut ctx.accounts.base_account.gif_list;
        for gif_item in gif_list {
            if gif_item.gif_link == gif_link {
                gif_item.upvotes-=1;
            }
        }
        Ok(())
    }

    pub fn comment_on_gif(ctx: Context<CommentGifs>, gif_link: String, comment_string: String) -> ProgramResult {
        let gif_list = &mut ctx.accounts.base_account.gif_list;
        let user = &mut ctx.accounts.user;
        for gif_item in gif_list {
            if gif_item.gif_link == gif_link {
                gif_item.comments.insert(user.to_account_info().key(),comment_string.clone());
            }
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddGifs<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct UpvoteGifs<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct DownvoteGifs<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct CommentGifs<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub upvotes: u64,
    pub comments: HashMap<Pubkey, String>,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}
