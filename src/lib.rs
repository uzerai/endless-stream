pub mod game {
	pub mod player_control;
	pub mod movement;
	pub mod enemy;
	pub mod entity_health;
	pub mod game_plugin;
}

pub mod menu {
	pub mod menu_plugin;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Paused,
}