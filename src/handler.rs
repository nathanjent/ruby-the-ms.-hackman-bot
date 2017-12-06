use error::*;
use bot::BotState;
use field::*;
use player::*;
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
            let ref field = bot.field.borrow();
            let player_map = bot.players.borrow();
            let settings = bot.settings.borrow();
            let ref player_name = settings.name;
            let ref player_id = settings.id;
            
            let mut action = String::new();

            if let Some(player) = player_map.get(player_name) {
                // TODO this is where decisions need to be made
                action = make_move(field, player_id).to_string();

                if let Some(detonation_time) = player.bomb_drop {
                    // TODO maybe don't drop the bomb as soon as you get it
                    action = format!("{};drop_bomb {}", action, detonation_time);
                } 
            }
            Some(action)
        }
    };
    Ok(reply)
}

/// Decide the next move
fn make_move(field: &Field, player_id: &i32) -> Move {
    let mut next_move = Move::new();
    next_move
}
