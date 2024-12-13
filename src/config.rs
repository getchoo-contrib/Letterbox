use poise::serenity_prelude as serenity;
use std::{collections::HashSet, fs, path::Path};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
	pub server_id: serenity::GuildId,
	pub forum_channel_id: serenity::ChannelId,
	#[serde(default = "staff_roles_default" /* [] */)]
	pub staff_roles: HashSet<serenity::RoleId>,
	#[serde(default = "prefix_default" /* = */)]
	pub prefix: String,
	#[serde(default = "status_default" /* Message me to contact mods! */)]
	pub status: String,
	#[serde(default = "anonymous_display_name") /* Staff Member */]
	pub anonymous_display_name: String,
	pub mention: Option<MentionMode>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum MentionMode {
	Everyone,
	Here,
	RoleId(u64),
}

impl Config {
	pub fn new_from_file(path: &Path) -> eyre::Result<Config> {
		let data = fs::read_to_string(path)?;
		Ok(toml::from_str(&data)?)
	}

	pub fn is_staff(&self, roles: &[serenity::RoleId]) -> bool {
		roles.iter().any(|role| self.staff_roles.contains(role))
	}
}

// https://github.com/serde-rs/serde/issues/368

fn staff_roles_default() -> HashSet<serenity::RoleId> {
	HashSet::new()
}

fn prefix_default() -> String {
	"=".to_string()
}

fn status_default() -> String {
	"Message me to contact mods!".to_string()
}

fn anonymous_display_name() -> String {
	"Staff Member".to_string()
}
