use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

use crate::checks::*;

#[command]
#[checks(not_blacklisted)]
#[description = "Stops the currently playing track, and clears the queue"]
#[aliases("leave", "die")]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let manager = songbird::get(ctx).await.unwrap().clone();

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await?;

        msg.channel_id.say(ctx, "Cleared queue").await?;
    } else {
        msg.reply_ping(ctx, "Not in a voice channel").await?;
    }

    Ok(())
}
