use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
use dataset::{chat::MockPrivateChat, message_common::MockMessageText};
use serde::Deserialize;
use serde_json::json;
use teloxide::types::{MessageEntity, ParseMode, ReplyMarkup};

use crate::{SentMessageText, MESSAGES, RESPONSES};


#[derive(Debug, Deserialize, Clone)]
pub struct SendMessageTextBody {
    pub chat_id: i64,
    pub text: String,
    pub parse_mode: Option<ParseMode>,
    pub entities: Option<Vec<MessageEntity>>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub allow_sending_without_reply: Option<bool>,
    pub reply_markup: Option<ReplyMarkup>,
}

pub async fn send_message(
    body: web::Json<SendMessageTextBody>,
) -> impl Responder {
    let mut message = // Creates the message, which will be mutated to fit the needed shape
        MockMessageText::new(&body.text).chat(MockPrivateChat::new().id(body.chat_id).build());

    message.entities = body.entities.clone().unwrap_or(vec![]);
    match body.reply_to_message_id {
        Some(id) => {
            message.reply_to_message = Some(Box::new(
                MESSAGES
                    .get_message(id)
                    .expect("Message to reply to was not found"),
            ))
        }
        None => {}
    }
    match body.reply_markup.clone() {
        // Only the inline keyboard can be inside of a message
        Some(ReplyMarkup::InlineKeyboard(markup)) => message.reply_markup = Some(markup),
        _ => {}
    }

    let last_id = MESSAGES.max_message_id();
    let message = message.id(last_id + 1).build();
    MESSAGES.add_message(message.clone());
    RESPONSES.lock().unwrap().sent_messages.push(message.clone());
    RESPONSES.lock().unwrap().sent_messages_text.push(SentMessageText {
        message: message.clone(),
        bot_request: body.into_inner(),
    });

    HttpResponse::Ok().body(
        json!({ // This is how telegram returns the message
            "ok": true,
            "result": message,
        })
        .to_string(),
    )
}