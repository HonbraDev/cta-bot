use anyhow::Result;
use poise::serenity_prelude::{self as serenity, EmojiId, ReactionType};
use rand::seq::SliceRandom;
use serde::Deserialize;

use crate::framework::Context;

const NO: &str = "No.";
const REACTED: &str = "Reacted";
const DELETED: &str = "Deleted";
const DELETED_FAIL: &str = "Couldn't delete the message???";
const NOT_MY_MESSAGE: &str = "Not my message";

#[derive(Deserialize)]
struct CtaImageListing {
    pub base: String,
    pub images: Vec<String>,
}

#[poise::command(slash_command)]
/// Dispense a cta
pub async fn cta(ctx: Context<'_>) -> Result<()> {
    let response: CtaImageListing = reqwest::get("https://honbra.com/img/cta/listing.json")
        .await?
        .json()
        .await?;

    let maybe_image = response.images.choose(&mut rand::thread_rng());

    match maybe_image {
        Some(image) => {
            ctx.say(format!("{}{image}", response.base)).await?;
        }
        None => {
            ctx.send(|f| {
                f.content("No image found in Honbra's database :(")
                    .ephemeral(true)
            })
            .await?;
        }
    };

    Ok(())
}

#[poise::command(slash_command)]
/// Dispense a kitty (videos by ender_schesi)
pub async fn kitty(ctx: Context<'_>) -> Result<()> {
    let kitties: Vec<String> = reqwest::get("https://kitties.enderschesi.me/list")
        .await?
        .json()
        .await?;

    let maybe_video = kitties.choose(&mut rand::thread_rng());

    match maybe_video {
        Some(video) => {
            ctx.say(video.to_owned()).await?;
        }
        None => {
            ctx.send(|f| {
                f.content("No videos found in schesi's database :(")
                    .ephemeral(true)
            })
            .await?;
        }
    };

    Ok(())
}

#[poise::command(slash_command)]
/// Sneaky sneaky
pub async fn echo(ctx: Context<'_>, #[description = "What to meow"] message: String) -> Result<()> {
    if ctx.data().cool_people.contains(&ctx.author().id) {
        ctx.say(message).await?;
    } else {
        ctx.send(|f| f.content(NO).ephemeral(true)).await?;
    }

    Ok(())
}

#[poise::command(context_menu_command = "Delete own message")]
pub async fn delete(ctx: Context<'_>, message: serenity::Message) -> Result<()> {
    let response = if ctx.data().cool_people.contains(&ctx.author().id) {
        if message.author.id == ctx.framework().bot_id {
            let result = message.delete(ctx.serenity_context()).await;
            match result {
                Ok(_) => DELETED,
                Err(_) => DELETED_FAIL,
            }
        } else {
            NOT_MY_MESSAGE
        }
    } else {
        NO
    };

    ctx.send(|f| f.content(response).ephemeral(true)).await?;

    Ok(())
}

#[poise::command(context_menu_command = "React with 🐈")]
pub async fn react_cat(ctx: Context<'_>, message: serenity::Message) -> Result<()> {
    let response = if ctx.data().cool_people.contains(&ctx.author().id) {
        let reaction_type = ReactionType::Unicode(String::from("🐈"));
        message.react(ctx, reaction_type).await?;

        REACTED
    } else {
        NO
    };

    ctx.send(|f| f.content(response).ephemeral(true)).await?;

    Ok(())
}

#[poise::command(context_menu_command = "React with :cta:")]
pub async fn react_cta(ctx: Context<'_>, message: serenity::Message) -> Result<()> {
    const EMOJI: EmojiId = EmojiId(1085692180175278121);

    let response = if ctx.data().cool_people.contains(&ctx.author().id) {
        let reaction_type = ReactionType::Custom {
            animated: false,
            id: EMOJI,
            name: Some(String::from("cta")),
        };
        message.react(ctx, reaction_type).await?;

        REACTED
    } else {
        NO
    };

    ctx.send(|f| f.content(response).ephemeral(true)).await?;

    Ok(())
}
