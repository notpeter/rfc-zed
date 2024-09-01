use zed_extension_api::{self as zed, SlashCommand, SlashCommandOutput, Worktree};

struct SlashCommandsExampleExtension;

impl zed::Extension for SlashCommandsExampleExtension {
    fn new() -> Self {
        SlashCommandsExampleExtension
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<zed_extension_api::SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "rfc" => Ok(vec![]),
            command => Err(format!("unknown slash command: \"{command}\"")),
        }
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        if command.name != "rfc" {
            return Err("Invalid command. Expected 'rfc'.".into());
        }
        if args.is_empty() {
            return Err("need rfc number".to_string());
        }
        if let Ok(rfc_number) = args[0].parse::<u32>() {
            let rfc_zero = format!("{:04}", rfc_number);
            Ok(zed::SlashCommandOutput {
                text: format!("RFC {rfc_number} https://www.ietf.org/rfc/rfc{rfc_zero}.json"),
                sections: vec![],
            })
        } else {
            Err("invalid rfc number".to_string())
        }
    }
}

zed::register_extension!(SlashCommandsExampleExtension);
