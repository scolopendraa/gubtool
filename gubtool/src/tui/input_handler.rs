use anyhow::{Result, anyhow, ensure};
use crossterm::event::{KeyCode, KeyEvent};
use crate::{
    config::{Config, Preferences, UiState},
    core::{self, chr_ins::ChrInsExt},
    tui::{
        ResultExt,
        app::{App, CurrentScreen},
        tabs::player_tab::{STATS_IDX, Stats},
    }
};

pub enum InputField {
    PlayerHp,
    GiveRunes,
    PlayerSpeed,
    PlayerStat,
    TargetHpVal,
    TargetHpPercentage,
    TargetForceAct,
    TargetForceActSequence,
    Fps,
    GameSpeed,
    Quantity,
    Upgrade,
    NewGameCycle,
    ConfigFps,
    Event,
}

pub fn input_set_val(app: &mut App) {
    let tx = app.sender.clone();
    let text = &app.input.text;
    match app.input_field {
        InputField::PlayerHp => {
            if let Ok(v) = text.parse() {
                app.player.hp = v;
                UiState::update(|c| { c.player_set_health = v; }).ok();
            }
        }
        InputField::GiveRunes => {
            if let Ok(v) = text.parse() {
                app.player.runes = v;
                UiState::update(|c| { c.give_runes = v; }).ok();
            }
        }
        InputField::PlayerSpeed => {
            if let Ok(v) = text.parse() {
                core::player::player_ins()
                    .set_animation_speed(v)
                    .send_error(tx);
            }
        }
        InputField::PlayerStat => {
            if let Ok(v) = text.parse() {
                let idx = app.player.tab.lists[STATS_IDX].selected().unwrap_or_default();
                let stat = &Stats::array(app)[idx];
                stat.set_stat(v).send_error(tx);
            }
        }
        InputField::Fps => {
            if let Ok(v) = text.parse() {
                core::utility::set_fps_cap(v).send_error(tx);
            }
        }
        InputField::GameSpeed => {
            if let Ok(v) = text.parse() {
                core::utility::set_game_speed(v).send_error(tx);
            }
        }
        InputField::TargetHpVal => {
            if let Ok(v) = text.parse() {
                app.target.hp_val = v;
                UiState::update(|c| { c.target_set_health = v; }).ok();
            }
        }
        InputField::TargetHpPercentage => {
            if let Ok(v) = text.parse() {
                app.target.hp_percentage = v;
                UiState::update(|c| { c.target_set_health_pct = v; }).ok();
            }
        }
        InputField::TargetForceAct => {
            if let Ok(v) = text.parse() {
                app.target.act = v;
                UiState::update(|c| { c.target_act = v; }).ok();
            }
        }
        InputField::TargetForceActSequence => {
            let result = parse_act_sequence(text);
            if let Ok(v) = result {
                app.target.act_array = v.clone();
                UiState::update(|c| { c.target_act_array = v; }).ok();
            } else {
                result.send_error(tx)
            }
        }
        InputField::Quantity => {
            if let Ok(v) = text.parse() {
                app.item.quantity = v;
                app.item.handle_item_switch(app.game_state.dlc);
            }
        }
        InputField::Upgrade => {
            if let Ok(v) = text.parse() {
                app.item.upgrade = v;
                app.item.handle_item_switch(app.game_state.dlc);
            }
        }
        InputField::NewGameCycle => {
            if let Ok(v) = text.parse() {
                core::utility::set_ng_cycle(v).send_error(tx);
            }
        }
        InputField::ConfigFps => {
            if let Ok(v) = text.parse() {
                Preferences::update(|c| {
                    c.fps = Some(v);
                })
                .send_error(tx);
            } else if text.is_empty() {
                Preferences::update(|c| {
                    c.fps = None;
                })
                .send_error(tx);
            }
        }
        InputField::Event => {
            if let Ok(v) = text.parse() {
                app.event.event = Some(v);
                UiState::update(|c| { c.event = Some(v); }).ok();
            }
        }
    }
}

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => {
            input_set_val(app);
            app.input.set_text("");
            app.current_screen = CurrentScreen::Tab;
        }
        KeyCode::Esc => {
            app.input.set_text("");
            app.current_screen = CurrentScreen::Tab;
        }
        _ => {
            let _ = app.input.update(key);
        }
    }
}

fn parse_act_sequence(input: &str) -> Result<Vec<i32>> {
    input
        .split_whitespace()
        .map(|s| {
            let val = s
                .parse::<i32>()
                .map_err(|_| anyhow!("Expects integers seperated by spaces"))?;
            ensure!(val <= 50, "Highest act number is 50");
            Ok(val)
        })
        .collect()
}
