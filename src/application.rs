use serenity::{
    client::Context, 
    model::{
        id::GuildId, 
        interactions::application_command::ApplicationCommandOptionType
    }
};

pub async fn load_dev_guild_commands(ctx: Context) {
    const GUILD_ID: GuildId = GuildId(909850768947937290);

    let commands = GuildId::set_application_commands(&GUILD_ID, ctx, |cmds| {
        cmds
        .create_application_command(|cmd| {
            cmd.name("ping").description("A ping command")
        })
        .create_application_command(|cmd| {
            cmd.name("user").description("Displays the information of a specified user.")
            .create_option(|option| {
                option
                .name("user")
                .description("The user to get the information from.")
                .kind(ApplicationCommandOptionType::User)
                .required(false)
            })
        })
    }).await;

    println!("Loaded slash commands on guild({})\nSlash Commands: {:#?}", GUILD_ID, commands)
}