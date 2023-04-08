use anyhow::Error;
use poise::serenity_prelude as serenity;
use std::collections::HashSet;

use crate::commands;
use crate::config::Config;

pub struct Data {
    pub cool_people: HashSet<serenity::UserId>,
}

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub fn new(config: Config) -> poise::FrameworkBuilder<Data, Error> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::cta(),
                commands::kitty(),
                commands::echo(),
                commands::delete(),
                commands::react_cat(),
                commands::react_cta(),
            ],
            ..Default::default()
        })
        .token(config.discor_token)
        .intents(serenity::GatewayIntents::GUILDS)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    cool_people: config.cool_people,
                })
            })
        })
}
