use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

use crate::checks::*;

#[command]
#[only_in(guilds)]
#[aliases("vol")]
#[checks(Player)]
async fn volume(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let new_volume = match args.single_quoted::<i32>() {
        Ok(vol) => vol,
        Err(_) => {
            let manager = songbird::get(ctx).await.unwrap().clone();

            if let Some(handler_lock) = manager.get(guild_id) {
                let handler = handler_lock.lock().await;
                let queue = handler.queue();

                if let Some(handle) = queue.current() {
                    let mut current_volume = handle.get_info()?.await?.volume * 100f32;
                    current_volume = current_volume.round();

                    msg.channel_id
                        .say(ctx, format!("The current volume is {}", current_volume))
                        .await?;
                } else {
                    msg.reply_ping(ctx, "Nothing playing").await?;
                }
            } else {
                msg.reply_ping(ctx, "Not in a voice channel").await?;
            }

            return Ok(());
        }
    };

    if new_volume < 0 || new_volume > 100 {
        msg.reply_ping(ctx, "Please select a value from 0 to 100")
            .await?;
        return Ok(());
    }

    let new_volume: f32 = new_volume as f32 / 100f32;

    let manager = songbird::get(ctx).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        if let Some(handle) = queue.current() {
            handle.set_volume(new_volume)?;
        } else {
            msg.reply_ping(ctx, "Nothing playing").await?;
        }
    } else {
        msg.reply_ping(ctx, "Not in a voice channel").await?;
    }

    Ok(())
}
