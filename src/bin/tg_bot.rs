use hello_axum::keyboard::{one_level_inline_keyboard, second_level_inline_keyboard, ManuState};
use lazy_static::lazy_static;
use std::{collections::HashMap, error::Error};
use teloxide::dispatching::UpdateFilterExt;
use teloxide::payloads::{EditMessageReplyMarkupSetters, SendMessage, SendMessageSetters};
use teloxide::prelude::Dispatcher;
use teloxide::requests::{JsonRequest, Requester};
use teloxide::types::{
    ButtonRequest, CallbackQuery, ChatId, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup, Me,
    Message, MessageKind, MessageWebAppData, ReplyMarkup, Update, WebAppInfo,
};

use teloxide::utils::command::BotCommands;
use teloxide::{dptree, Bot};
use tokio::sync::Mutex;
#[derive(BotCommands)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Start")]
    Start,
}

lazy_static! {
    // user_id -> menu state
    static ref USER_MENU_STATE: Mutex<HashMap<ChatId, ManuState>> = Mutex::new(HashMap::new());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting buttons bot...");
    let bot = Bot::from_env();
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(inline_keyboard_callback_handler));
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}

async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // get web app data from webapp
    if let teloxide::types::MessageKind::WebAppData(ref web_data) = msg.kind {
        let chat_id = msg.chat.id;
        let text = web_data.web_app_data.data.clone();
        let keyboard: InlineKeyboardMarkup = second_level_inline_keyboard();
        bot.send_message(chat_id, text)
            .reply_markup(keyboard)
            .await?;
    }
    // get command data from bot
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                // Just send the description of all commands.
                bot.send_message(msg.from().unwrap().id, Command::descriptions().to_string())
                    .await?;
            }
            Ok(Command::Start) => {
                let menu_state = ManuState::default();
                let text = "Eth Gas: 10 === Block: 1808696 === ETH: $1643 \n💵TokenTool WEBSITE  Tutorials \n".to_string();
                let keyboard = one_level_inline_keyboard(menu_state).await;
                bot.send_message(msg.from().unwrap().id, text)
                    .reply_markup(keyboard)
                    .await?;
            }
            Err(_) => {
                bot.send_message(msg.from().unwrap().id, "Command not found!")
                    .await?;
            }
        }
    }
    Ok(())
}

async fn inline_keyboard_callback_handler(
    bot: Bot,
    q: CallbackQuery,
) -> Result<(), Box<(dyn Error + Send + Sync)>> {
    let message = q.message.unwrap().clone();
    let chat_id = message.chat.id;
    let message_id = message.id.clone();
    // 获取当前用户的菜单状态
    let mut user_manu_state_lock = USER_MENU_STATE.lock().await;
    // 使用 entry 方法检查键是否存在 这段代码首先使用 entry 方法来检查键是否存在，如果存在则获取现有的可变引用，如果不存在则插入一个默认的 ManuState。
    let menu_state = user_manu_state_lock
        .entry(chat_id)
        .or_insert_with(|| ManuState::default());
    let data = q.data.clone();
    let mut text = "Eth Gas: 10 === Block: 1808696 === ETH: $1643 \n
    💵TokenTool WEBSITE  Tutorials \n"
        .to_string();
    if let Some(data) = data.clone() {
        // 根据data的值来渲染不同的菜单
        match data.as_str() {
            "bzdb" | "fhbb" | "fhlp" | "lpfh+tjjl" | "cbfl+tjjl" | "lpwk+tjjl" => {
                menu_state.focus_manu_item_symbol = data.as_str().to_owned();
                // Show details for option A with the "Confirm" and "Back" buttons.
                let keyboard: InlineKeyboardMarkup = second_level_inline_keyboard();
                bot.edit_message_text(chat_id, message_id, text).await?;
                bot.edit_message_reply_markup(chat_id, message_id)
                    .reply_markup(keyboard)
                    .await?;
            }
            "goerli" | "bsc" | "eth" | "op" => {
                menu_state.chain_symbol = data.as_str().to_owned();
                let latest_menu_state = menu_state.clone();
                let keyboard = one_level_inline_keyboard(latest_menu_state).await;
                // 使用 editMessageText 方法编辑消息
                bot.edit_message_reply_markup(chat_id, message_id)
                    .reply_markup(keyboard)
                    .await?;
            }
            "true" | "false" => {
                menu_state.account_symbol = data.as_str().to_owned();
                let latest_menu_state = menu_state.clone();
                let keyboard = one_level_inline_keyboard(latest_menu_state).await;
                // 使用 editMessageText 方法编辑消息
                bot.edit_message_reply_markup(chat_id, message_id)
                    .reply_markup(keyboard)
                    .await?;
            }
            "begin" => {
                // call web apps
                let begin =
                    KeyboardButton::new("Open Web App")
                        .request(ButtonRequest::WebApp(WebAppInfo {
                        url: "https://revenkroz.github.io/telegram-web-app-bot-example/index.html"
                            .parse()
                            .unwrap(),
                    }));

                let mut new_key_board = KeyboardMarkup::new(vec![vec![begin]]);
                new_key_board = new_key_board.one_time_keyboard(true);
                let _message = bot
                    .send_message(chat_id, "Open Web App")
                    .reply_markup(new_key_board)
                    .await?;
            }
            "cancel" => {
                // 查询当前focus的是哪一层
                let focus_manu_layer = menu_state.focus_manu_layer;
                // 将对应层的菜单设置为选中状态
                match focus_manu_layer {
                    0 => {
                        menu_state.erc20_symbol = "".to_owned();
                    }
                    1 => {
                        menu_state.chain_symbol = "".to_owned();
                    }
                    _ => {}
                }
                let latest_menu_state = menu_state.clone();
                let keyboard = one_level_inline_keyboard(latest_menu_state).await;
                let text = "请选择要部署的代币类型:\n".to_string();
                bot.edit_message_text(chat_id, message_id, text).await?;
                // 使用 editMessageText 方法编辑消息
                bot.edit_message_reply_markup(chat_id, message_id)
                    .reply_markup(keyboard)
                    .await?;
            }
            "confirm" => {
                // 查询当前focus的是哪一层
                let focus_manu_layer = menu_state.focus_manu_layer;
                // 将对应层的菜单设置为选中状态
                match focus_manu_layer {
                    0 => {
                        menu_state.erc20_symbol = menu_state.focus_manu_item_symbol.to_owned();
                    }
                    1 => {
                        menu_state.chain_symbol = menu_state.focus_manu_item_symbol.to_owned();
                    }
                    _ => {}
                }
                let latest_menu_state = menu_state.clone();
                let keyboard = one_level_inline_keyboard(latest_menu_state).await;
                // let text = "请选择要部署的代币类型:\n".to_string();
                bot.edit_message_text(chat_id, message_id, text).await?;
                // 使用 editMessageText 方法编辑消息
                bot.edit_message_reply_markup(chat_id, message_id)
                    .reply_markup(keyboard)
                    .await?;
                // replykeboard remove

                let res_kb = ReplyMarkup::kb_remove();
                bot.send_message(chat_id, "选择要部署的代币类型:\n".to_string())
                    .reply_markup(res_kb)
                    .await?;
            }
            _ => {
                // 打印默认的值
                println!("default");
            }
        }
    }
    // unlock user menu state
    drop(user_manu_state_lock);
    Ok(())
}
