use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options.get(0).unwrap().resolved.as_ref().unwrap();
    if let CommandDataOptionValue::String(palavra) = option {
        let mut hasher = DefaultHasher::new();
        palavra.hash(&mut hasher);
        let picles: f64 = hasher.finish() as f64 / u64::MAX as f64 * 100 as f64;
        format!("funny level da palavra: {&palavra} = {&picles}%")
    }
    else {
        "nao deu certo".to_string()
    }

}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("funny").description("calcular o funny level da palavra")
        .create_option(|option| {
            option
                .name("palavra")
                .description("palavra pra medir o funny")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
