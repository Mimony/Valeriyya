use serenity::{
    model::id::GuildId,
    client::Context
};

pub async fn load_dev_guild_commands(ctx: Context) {
    const GUILD_ID: GuildId = GuildId(909850768947937290);

    let commands = GuildId::set_application_commands(&GUILD_ID, ctx, |cmds| {
        cmds
        .create_application_command(|cmd| {
            cmd.name("ping").description("A ping command")
        })
    }).await;

    println!("Loaded slash commands on guild({})\nSlash Commands: {:#?}", GUILD_ID, commands)
}