# Hello Axum

## 项目介绍

这是一个 axum web 模版工程，方便用户快速在 axum 上快速进行 web 开发，为了帮助 java、go 开发者找到 web 开发的感觉。因此整个项目的划分结构和传统的项目结构保持一一致。

## 项目目录介绍

controller 层主要放置处理器 handler。路由部分暂时统一定义在 main.rs 中。

middleware 层主要定义中间件，比如 auth 认证、session、链路跟踪等，对应 spring 中的 aop 面向切面编程。

service 层，这层主要处理服务，当然项目较小，可以直接 controller 到底。

database 层对应的是 dao、mapper，这个目录下主要放对 db 的增删改查操作。

state 文件主要定义项目所需上下文，例如 redis 链接、db 链接等。

## 项目技术栈

-   axum
-   sqlx
-   swagger-ui
-   postgres
-   redis
-   待完善

use std::{collections::HashMap, error::Error};
use teloxide::{
payloads::SendMessageSetters,
prelude::\*,
types::{
InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
InputMessageContentText, Me,
},
utils::command::BotCommands,
};
use tracing_subscriber::fmt::format;

#[derive(BotCommands)] #[command(
rename_rule = "lowercase",
description = "These commands are supported:"
)]
enum Command { #[command(description = "Display this text")]
Help, #[command(description = "Start")]
Start,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
dotenv::dotenv().ok();
pretty_env_logger::init();
log::info!("Starting buttons bot...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())

}

/// Creates a keyboard made by buttons in a big column.
fn make_keyboard() -> InlineKeyboardMarkup {
let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
// 选择 erc20 token 类型
let erc20_lists = [
"标准代币",
"分红本币",
"LP 分红",
"LP 分红+推荐奖励",
"持币复利+推荐奖励",
"LP 挖矿+推荐奖励",
];

    // 创建一个hashmap  key是erc20的工具  value是token的描述
    let mut erc20_description: HashMap<u8, String> = HashMap::new();
    erc20_description.insert(0, "干净合约、方便上手、无税无功能、Ave检测全绿".to_owned());
    erc20_description.insert(
        1,
        "简单干净合约,无黑白名单,无权限,加池自动开盘,持币即可获益!".to_owned(),
    );
    erc20_description.insert(2, "加池参与分红、池子越来越厚,币价螺旋上涨!".to_owned());
    erc20_description.insert(3, "下级交易、上级奖励、持续裂变、壮大规模!".to_owned());
    erc20_description.insert(
        4,
        "持币自动生息、代币资产累积、打造去中心化银行!".to_owned(),
    );
    erc20_description.insert(
        5,
        "加池挖矿、恒定产出、无前端无后端、完全去中心化运行".to_owned(),
    );

    for versions in erc20_lists.chunks(2) {
        let row = versions
            .iter()
            .map(|&version| InlineKeyboardButton::callback(version.to_owned(), version.to_owned()))
            .collect();

        keyboard.push(row);
    }
    // // 选择链
    // let chan_lists = ["Goerli", "BSC", "ETH", "Op", "Arb", "Base"];

    // for versions in chan_lists.chunks(3) {
    //     let row = versions
    //         .iter()
    //         .map(|&version| InlineKeyboardButton::callback(version.to_owned(), version.to_owned()))
    //         .collect();

    //     keyboard.push(row);
    // }
    // // 是否创建钱包
    // let need_new_wallet = ["create new wallet🟢", "no new wallet🛑"];
    // for versions in need_new_wallet.chunks(2) {
    //     let row = versions
    //         .iter()
    //         .map(|&version| InlineKeyboardButton::callback(version.to_owned(), version.to_owned()))
    //         .collect();

    //     keyboard.push(row);
    // }
    InlineKeyboardMarkup::new(keyboard)

}

/// Parse the text wrote on Telegram and check if that text is a valid command
/// or not, then match the command. If the command is `/start` it writes a
/// markup with the `InlineKeyboardMarkup`.
async fn message_handler(
bot: Bot,
msg: Message,
me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
if let Some(text) = msg.text() {
match BotCommands::parse(text, me.username()) {
Ok(Command::Help) => {
// Just send the description of all commands.
bot.send_message(msg.chat.id, Command::descriptions().to_string())
.await?;
}
Ok(Command::Start) => {
// Create a list of buttons and send them.
let keyboard = make_keyboard();
bot.send_message(msg.chat.id, "发行代币:")
.reply_markup(keyboard)
.await?;
}

            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found!").await?;
            }
        }
    }

    Ok(())

}

async fn inline_query_handler(
bot: Bot,
q: InlineQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
let choose_debian_version = InlineQueryResultArticle::new(
"0",
"Chose debian version",
InputMessageContent::Text(InputMessageContentText::new("Debian versions:")),
)
.reply_markup(make_keyboard());

    bot.answer_inline_query(q.id, vec![choose_debian_version.into()])
        .await?;

    Ok(())

}

/// When it receives a callback from a button it edits the message with all
/// those buttons writing a text with the selected Debian version.
///
/// **IMPORTANT**: do not send privacy-sensitive data this way!!!
/// Anyone can read data stored in the callback button.
async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
if let Some(version) = q.data {
let text = format!("You chose: {version}");
// token descrition
let desc = format!("简单干净合约,无黑白名单,无权限,加池自动开盘,持币即可获益!");

        // show back buttion


        // Tell telegram that we've seen this query, to remove 🕑 icons from the
        // clients. You could also use `answer_callback_query`'s optional
        // parameters to tweak what happens on the client side.
        bot.answer_callback_query(q.id).await?;

        // Edit text of the message to which the buttons were attached
        if let Some(Message { id, chat, .. }) = q.message {
            bot.edit_message_text(chat.id, id, text).await?;
        } else if let Some(id) = q.inline_message_id {
            bot.edit_message_text_inline(id, text).await?;
        }

        log::info!("You chose: {}", version);
    }

    Ok(())

}
