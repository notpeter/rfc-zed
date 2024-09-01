use zed::{SlashCommand, SlashCommandOutput, Worktree};
use zed_extension_api as zed;

const RFC_BASE_URL: &str = "https://www.ietf.org/rfc/rfc";

struct SlashCommandRfcExtension;

impl zed::Extension for SlashCommandRfcExtension {
    fn new() -> Self {
        SlashCommandRfcExtension
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
        } else if args.is_empty() {
            return Err("need rfc number".to_string());
        }
        let Ok(rfc_number) = args[0].parse::<u32>() else {
            return Err("invalid rfc number".to_string());
        };

        let rfc_zero = format!("{:04}", rfc_number);
        let json_url = format!("{RFC_BASE_URL}/rfc{rfc_zero}.json");
        let text_url = format!("{RFC_BASE_URL}/rfc{rfc_zero}.txt");

        Ok(zed::SlashCommandOutput {
            text: format!("RFC {rfc_number} {text_url}"),
            sections: vec![],
        })
    }
}

zed::register_extension!(SlashCommandRfcExtension);
