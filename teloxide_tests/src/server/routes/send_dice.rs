use crate::server::{SentMessageDice, MESSAGES, RESPONSES};
use crate::MockMessageDice;
use actix_web::{web, Responder};
use serde::Deserialize;
use teloxide::types::{DiceEmoji, ReplyMarkup};

use super::{make_telegram_result, BodyChatId};

#[derive(Debug, Deserialize, Clone)]
pub struct SendMessageDiceBody {
    pub chat_id: BodyChatId,
    pub message_thread_id: Option<i64>,
    pub emoji: Option<DiceEmoji>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub message_effect_id: Option<String>,
    pub reply_markup: Option<ReplyMarkup>,
    pub reply_to_message_id: Option<i32>,
}

pub async fn send_dice(body: web::Json<SendMessageDiceBody>) -> impl Responder {
    let chat = body.chat_id.chat();
    let mut message = // Creates the message, which will be mutated to fit the needed shape
        MockMessageDice::new().chat(chat);
    message.emoji = body.emoji.clone().unwrap_or(MockMessageDice::EMOJI);
    // Random from 1 to 5 because it fits all the emoji
    message.value = (1 + rand::random::<u8>() % 5) as i32;

    let last_id = MESSAGES.max_message_id();
    let message = MESSAGES.add_message(message.id(last_id + 1).build());

    let mut responses_lock = RESPONSES.lock().unwrap();
    responses_lock.sent_messages.push(message.clone());
    responses_lock.sent_messages_dice.push(SentMessageDice {
        message: message.clone(),
        bot_request: body.into_inner(),
    });

    make_telegram_result(message)
}
