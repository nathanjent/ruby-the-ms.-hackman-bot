use error::*;
use bot::BotState;
use player::Player;
use message::*;

pub fn handle_message(line: String, bot: &BotState) -> Result<Option<String>> {
    let msg = line.parse::<Message>()?;
    let reply = match msg {
        Message::Settings(Setting::TimeBank(n)) => {
            let mut settings = bot.settings.borrow_mut();
            settings.time_bank = n;
            Some(format!("set time_bank {}", n))
        }
        Message::Settings(Setting::TimePerMove(n)) => {
            bot.settings.borrow_mut().time_per_move = n;
            None
        }
        Message::Settings(Setting::PlayerNames(names)) => {
            for name in names {
                let player_name = name.clone();
                bot.players
                    .borrow_mut()
                    .insert(name, Player::new(player_name));
            }
            None
        }
        Message::Settings(Setting::YourBot(bot_name)) => {
            let mut settings = bot.settings.borrow_mut();
            settings.name = bot_name;
            None
        }
        Message::Settings(Setting::YourBotId(id)) => {
            let mut settings = bot.settings.borrow_mut();
            settings.id = id;
            None
        }
        Message::Settings(Setting::FieldWidth(w)) => {
            let mut field = bot.field.borrow_mut();
            field.set_width(w as usize);
            None
        }
        Message::Settings(Setting::FieldHeight(h)) => {
            let mut field = bot.field.borrow_mut();
            field.set_height(h as usize);
            None
        }
        Message::Settings(Setting::MaxRounds(max)) => {
            let mut settings = bot.settings.borrow_mut();
            settings.max_rounds = max;
            None
        }
        Message::Update(Update::GameRound(n)) => {
            let mut settings = bot.settings.borrow_mut();
            settings.round = n;
            None
        }
        Message::Update(Update::GameField(field_update)) => {
            let mut field = bot.field.borrow_mut();
            field.update_field(field_update);
            None
        }
        Message::Update(Update::PlayerSnippets(player, n)) => {
            let mut player_map = bot.players.borrow_mut();
            if let Some(player) = player_map.get_mut(&player) {
                player.snippets = n;
            }
            None
        }
        Message::Update(Update::PlayerBombs(player, n)) => {
            let mut player_map = bot.players.borrow_mut();
            if let Some(player) = player_map.get_mut(&player) {
                player.bombs = n;
            }
            None
        }
        Message::Action(Action::Character { time_to_respond: n }) => {
            // TODO allow character choice configuration
            Some("bixie".into())
        }
        Message::Action(Action::Move { time_to_respond: n }) => {
            let player_map = bot.players.borrow();
            let ref player_name = bot.settings.borrow().name;
            let mut action = Some("up".into());
            if let Some(player) = player_map.get(player_name) {
                if let Some(detonation_time) = player.bomb_drop {
                    action = Some(format!("up;drop_bomb {}", detonation_time));
                } 
            }
            action
        }
    };
    Ok(reply)
}
