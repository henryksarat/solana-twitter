use anchor_lang::prelude::*;
// use anchor_lang::solana_program::system_program;

declare_id!("96cYJCVUEZreTJoyK3NyXHnAzHA2SzvKqcmXRVtxdTEF");

#[program]
pub mod solana_twitter {
    use super::*;
    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> Result<()> {
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        if topic.chars().count() > 50 {
            return Err(ErrorCode::TopicTooLong.into())
        }

        if content.chars().count() > 280 {
            return Err(ErrorCode::ContentTooLong.into())
        }

        tweet.author = *author.key;
        tweet.timestamp = clock.unix_timestamp;
        tweet.topic = topic;
        tweet.content = content;

        Ok(())
    }

    pub fn update_tweet(ctx: Context<UpdateTweet>, topic: String, content: String) -> Result<()> {
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;

        if topic.chars().count() > 50 {
            return Err(ErrorCode::TopicTooLong.into())
        }

        if content.chars().count() > 280 {
            return Err(ErrorCode::ContentTooLong.into())
        }

        tweet.topic = topic;
        tweet.content = content;

        Ok(())
    }

    pub fn delete_tweet(_ctx: Context<DeleteTweet>) -> Result<()> {
        Ok(())
    }

    pub fn message_me(ctx: Context<StoreToast>, message: String) -> Result<()> {
        let toast: &mut Account<Toast> = &mut ctx.accounts.toast;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();


        if message.chars().count() > 50 {
            return Err(ErrorCode::TopicTooLong.into())
        }

        toast.author = *author.key;
        toast.timestamp = clock.unix_timestamp;
        toast.message = message;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendTweet<'info> {
    #[account(init, payer = author, space = Tweet::LEN)]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub author: Signer<'info>,
    // #[account(address = system_program::ID)]
    // pub system_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdateTweet<'info> {
    #[account(mut, has_one = author)]
    pub tweet: Account<'info, Tweet>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteTweet<'info> {
    #[account(mut, has_one = author, close = author)]
    pub tweet: Account<'info, Tweet>,
    pub author: Signer<'info>,
}

#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}

#[derive(Accounts)]
pub struct StoreToast<'info> {
    #[account(init, payer = author, space = Toast::LEN)]
    pub toast: Account<'info, Toast>,
    #[account(mut)]
    pub author: Signer<'info>,
    // #[account(address = system_program::ID)]
    // pub system_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Toast {
    pub author: Pubkey,
    pub timestamp: i64,
    pub message: String,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_TOPIC_LENGTH: usize = 50 * 4; // 50 chars max.
const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max.

impl Tweet {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; // Content.
}

impl Toast {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH; // Topic.
}

// #[error]
#[error_code]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 280 characters long maximum.")]
    ContentTooLong,
}