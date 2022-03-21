use serenity::model::interactions::application_command::{
    ApplicationCommandInteraction, 
    ApplicationCommandInteractionDataOptionValue
};

pub fn execute(int: &ApplicationCommandInteraction) -> String {
    let user_option = int.data
    .options
    .get(0)
    .expect("Expected user option")
    .resolved
    .as_ref()
    .expect("Expected user object");

    if let ApplicationCommandInteractionDataOptionValue::User(user, _member) = user_option {
        format!("{}", user.tag())
    } else {
        "Error shit".to_string()
    }
}