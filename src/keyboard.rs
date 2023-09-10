use lazy_static::lazy_static;
use std::{collections::HashMap, error::Error};
use teloxide::dispatching::dialogue::ErasedStorage;
use teloxide::dispatching::UpdateFilterExt;
use teloxide::payloads::{EditMessageReplyMarkupSetters, SendMessageSetters};
use teloxide::prelude::Dispatcher;
use teloxide::requests::Requester;
use teloxide::types::{
    ButtonRequest, CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, InlineQuery,
    InlineQueryResultArticle, InputMessageContent, InputMessageContentText, KeyboardButton,
    KeyboardMarkup, Me, Message, ReplyMarkup, Update, WebAppInfo,
};

use teloxide::utils::command::BotCommands;
use teloxide::{dptree, Bot};
use tokio::sync::Mutex;

#[derive(Clone, Default)]
pub struct ManuState {
    pub erc20_symbol: String,
    pub chain_symbol: String,
    pub account_symbol: String,
    pub focus_manu_layer: u8,
    pub focus_manu_item_symbol: String,
}

struct TokenTemplateInfo {
    name: String,
    symbol: String,
    description: String,
}
// chain info
struct ChainInfo {
    name: String,
    symbol: String,
    description: String,
    rpc_url: String,
    block_scan_key: String,
}

// create wallet
struct WalletInfo {
    name: String,
    is_selected: String,
}

// 1 level menu
pub async fn one_level_inline_keyboard(menu_state: ManuState) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    // 分割线 请选择要部署的链
    keyboard.push(vec![InlineKeyboardButton::callback(
        ":::::: 选择代币类型 :::::: ",
        "token",
    )]);
    // 选择 erc20 token 类型
    let mut token_template_info_list: Vec<TokenTemplateInfo> = Vec::new();
    token_template_info_list.push(TokenTemplateInfo {
        name: "标准代币".to_owned(),
        symbol: "bzdb".to_owned(),
        description: "干净合约、方便上手、无税无功能、Ave检测全绿".to_owned(),
    });
    token_template_info_list.push(TokenTemplateInfo {
        name: "分红本币".to_owned(),
        symbol: "fhbb".to_owned(),
        description: "简单干净合约,无黑白名单,无权限,加池自动开盘,持币即可获益!".to_owned(),
    });
    token_template_info_list.push(TokenTemplateInfo {
        name: "LP 分红".to_owned(),
        symbol: "lpfh".to_owned(),
        description: "加池参与分红、池子越来越厚,币价螺旋上涨!".to_owned(),
    });
    token_template_info_list.push(TokenTemplateInfo {
        name: "LP 分红+推荐奖励".to_owned(),
        symbol: "lpfh+tjjl".to_owned(),
        description: "下级交易、上级奖励、持续裂变、壮大规模!".to_owned(),
    });
    token_template_info_list.push(TokenTemplateInfo {
        name: "持币复利+推荐奖励".to_owned(),
        symbol: "cbfl+tjjl".to_owned(),
        description: "持币自动生息、代币资产累积、打造去中心化银行!".to_owned(),
    });
    token_template_info_list.push(TokenTemplateInfo {
        name: "LP 挖矿+推荐奖励".to_owned(),
        symbol: "lpwk+tjjl".to_owned(),
        description: "加池挖矿、恒定产出、无前端无后端、完全去中心化运行".to_owned(),
    });
    // 一排两个
    for versions in token_template_info_list.chunks(2) {
        let row = versions
            .iter()
            .map(|version| {
                // 判断当前erc20 token模版是否被选中
                if menu_state.erc20_symbol == version.symbol {
                    InlineKeyboardButton::callback(
                        version.name.to_owned() + "🟢",
                        version.symbol.to_owned(),
                    )
                } else {
                    InlineKeyboardButton::callback(
                        version.name.to_owned(),
                        version.symbol.to_owned(),
                    )
                }
            })
            .collect();

        keyboard.push(row);
    }
    // 分割线 请选择要部署的链
    keyboard.push(vec![InlineKeyboardButton::callback(
        ":::::: 请选择要部署的链 :::::: ",
        "chain",
    )]);
    // 链
    let mut chain_info_list: Vec<ChainInfo> = Vec::new();
    chain_info_list.push(ChainInfo {
        name: "Goerli".to_owned(),
        symbol: "goerli".to_owned(),
        description: "goerli testnet".to_owned(),
        rpc_url: "https://goerli.infura.io/v3/".to_owned(),
        block_scan_key: "goerli".to_owned(),
    });
    chain_info_list.push(ChainInfo {
        name: "BSC".to_owned(),
        symbol: "bsc".to_owned(),
        description: "Binance Smart Chain".to_owned(),
        rpc_url: "https://bsc-dataseed.binance.org/".to_owned(),
        block_scan_key: "bsc".to_owned(),
    });
    chain_info_list.push(ChainInfo {
        name: "ETH".to_owned(),
        symbol: "eth".to_owned(),
        description: "Ethereum".to_owned(),
        rpc_url: "https://mainnet.infura.io/v3/".to_owned(),
        block_scan_key: "eth".to_owned(),
    });
    chain_info_list.push(ChainInfo {
        name: "Op".to_owned(),
        symbol: "op".to_owned(),
        description: "Optimism".to_owned(),
        rpc_url: "https://mainnet.optimism.io/".to_owned(),
        block_scan_key: "op".to_owned(),
    });
    // 一列3个
    for versions in chain_info_list.chunks(3) {
        let row = versions
            .iter()
            .map(|version| {
                if menu_state.chain_symbol == version.symbol {
                    InlineKeyboardButton::callback(
                        version.name.to_owned() + "🟢",
                        version.symbol.to_owned(),
                    )
                } else {
                    InlineKeyboardButton::callback(
                        version.name.to_owned(),
                        version.symbol.to_owned(),
                    )
                }
            })
            .collect();

        keyboard.push(row);
    }
    // 分割线 请选择要部署的链
    keyboard.push(vec![InlineKeyboardButton::callback(
        ":::::: 是否需要新钱包 :::::: ",
        "chain",
    )]);
    // 创建新钱包 是 或 否
    let mut wallet_button_list: Vec<WalletInfo> = Vec::new();
    wallet_button_list.push(WalletInfo {
        name: "是".to_owned(),
        is_selected: "true".to_owned(),
    });
    wallet_button_list.push(WalletInfo {
        name: "否".to_owned(),
        is_selected: "false".to_owned(),
    });
    // 一行两列
    for wallets in wallet_button_list.chunks(2) {
        let row = wallets
            .iter()
            .map(|wallet| {
                if menu_state.account_symbol == wallet.is_selected {
                    InlineKeyboardButton::callback(
                        wallet.name.to_owned() + "🟢",
                        wallet.is_selected.to_owned(),
                    )
                } else {
                    InlineKeyboardButton::callback(
                        wallet.name.to_owned(),
                        wallet.is_selected.to_owned(),
                    )
                }
            })
            .collect();

        keyboard.push(row);
    }

    // 是否开始部署
    keyboard.push(vec![InlineKeyboardButton::callback("✍️开始部署 ", "begin")]);
    // 展示
    InlineKeyboardMarkup::new(keyboard)
}

pub fn second_level_inline_keyboard() -> InlineKeyboardMarkup {
    let buttons: Vec<InlineKeyboardButton> = vec![
        InlineKeyboardButton::callback("Confirm", "confirm"),
        InlineKeyboardButton::callback("Cancel", "cancel"),
    ];
    let keyboard = vec![buttons];
    InlineKeyboardMarkup::new(keyboard)
}

pub fn deploy_contract_keyboard() -> InlineKeyboardMarkup {
    let buttons: Vec<InlineKeyboardButton> = vec![
        InlineKeyboardButton::callback("Deploy Now🔥", "deploy"),
        InlineKeyboardButton::callback("Cancel", "cancel"),
    ];
    let keyboard = vec![buttons];
    InlineKeyboardMarkup::new(keyboard)
}
