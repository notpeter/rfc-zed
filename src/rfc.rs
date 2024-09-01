use regex::Regex;
use serde::Deserialize;
use zed_extension_api::{
    self as zed, http_client::HttpMethod, http_client::HttpRequest, http_client::RedirectPolicy,
    serde_json, SlashCommand, SlashCommandOutput, Worktree,
};

const RFC_BASE_URL: &str = "https://www.ietf.org/rfc";

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
        // Prepare the request
        let json_request = HttpRequest {
            method: HttpMethod::Get,
            url: json_url,
            headers: vec![],
            body: None,
            redirect_policy: RedirectPolicy::FollowAll,
        };
        let text_request = HttpRequest {
            method: HttpMethod::Get,
            url: text_url,
            headers: vec![],
            body: None,
            redirect_policy: RedirectPolicy::FollowAll,
        };
        let rfc: Rfc = match zed::http_client::fetch(&json_request) {
            Ok(response) => serde_json::from_slice(&response.body).unwrap(),
            Err(e) => {
                return Err(format!("Error: {}", e));
            }
        };

        let rfc_text = match zed::http_client::fetch(&text_request) {
            Ok(response) => String::from_utf8(response.body).unwrap(),
            Err(e) => {
                return Err(format!("Error: {}", e));
            }
        };
        let clean_regex = Regex::new(r"\n.+\[Page \d+\][\n\u{c}]+(RFC \d+.+\d{4}\n\n)?").unwrap();
        let clean_text = clean_regex.replace_all(&rfc_text, "");
        let clean_text = Regex::new("\n\n\n+")
            .unwrap()
            .replace_all(&clean_text, "\n\n");
        return Ok(zed::SlashCommandOutput {
            text: format!("{}\n{}", rfc, clean_text),
            sections: vec![],
        });
    }
}

#[derive(serde::Deserialize)]
struct Rfc {
    doc_id: String,
    #[serde(deserialize_with = "deserialize_trimmed_string")]
    title: String,
    pub_date: String,
    // doi: String,
}

fn deserialize_trimmed_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.trim().to_string())
}

impl std::fmt::Display for Rfc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} ({})", self.doc_id, self.title, self.pub_date)
    }
}

zed::register_extension!(SlashCommandRfcExtension);
