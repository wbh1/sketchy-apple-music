use apple_music::{AppleMusic, PlayerState};
use std::{thread, time::Duration};

fn main() {
    loop {
        update_music_status();
        thread::sleep(Duration::from_millis(250));
    }
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn update_music_status() {
    let state = AppleMusic::get_application_data().unwrap();

    let icon = match state.player_state {
        Some(PlayerState::Playing) => "".into(),
        Some(PlayerState::Paused) => "".into(),
        _ => String::from(""),
    };

    let label = if let Some(PlayerState::Stopped) = state.player_state {
        String::new()
    } else {
        let current = AppleMusic::get_current_track().unwrap();
        let elapsed = state.player_position.unwrap_or_default();
        let elapsed_formatted =
            format!("{:02}:{:02}", elapsed as u64 / 60, elapsed as u64 % 60);
        format!(
            "{} • {} • {} [{}/{}]",
            current.name,
            current.artist,
            current.album,
            elapsed_formatted,
            current.time,
        )
    };

    let message = format!("--set mpd icon=\"{icon}\" label=\"{label}\"");
    sketchybar_rs::message(&message, Some("bottombar")).unwrap();
}
