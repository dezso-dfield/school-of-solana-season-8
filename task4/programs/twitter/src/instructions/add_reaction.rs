//-------------------------------------------------------------------------------
///
/// TASK: Implement the add reaction functionality for the Twitter program
/// 
/// Requirements:
/// - Initialize a new reaction account with proper PDA seeds
/// - Increment the appropriate counter (likes or dislikes) on the tweet
/// - Set reaction fields: type, author, parent tweet, and bump
/// - Handle both Like and Dislike reaction types
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_reaction(ctx: Context<AddReactionContext>, reaction_type: ReactionType) -> Result<()> {
    // TODO: Implement add reaction functionality
    let tweet = &mut ctx.accounts.tweet;
    let reaction = &mut ctx.accounts.tweet_reaction;

    match reaction_type {
        ReactionType::Like => {
            tweet.likes = tweet
                .likes
                .checked_add(1)
                .ok_or(TwitterError::MaxLikesReached)?;
        }
        ReactionType::Dislike => {
            tweet.dislikes = tweet
                .dislikes
                .checked_add(1)
                .ok_or(TwitterError::MaxDislikesReached)?;
        }
    }

    reaction.reaction_author = ctx.accounts.reaction_author.key();
    reaction.reaction = reaction_type;
    reaction.parent_tweet = tweet.key();
    reaction.bump = ctx.bumps.tweet_reaction;
    Ok(())
}

#[derive(Accounts)]
pub struct AddReactionContext<'info> {
    // TODO: Add required account constraints
    #[account(mut)]
    pub reaction_author: Signer<'info>,
    #[account(
        init,
        payer = reaction_author,
        seeds = [TWEET_REACTION_SEED.as_bytes(), reaction_author.key().as_ref(), tweet.key().as_ref()],
        space = 8 + Reaction::INIT_SPACE,
        bump
    )]
    pub tweet_reaction: Account<'info, Reaction>,
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
    
    pub system_program: Program<'info, System>,
}
