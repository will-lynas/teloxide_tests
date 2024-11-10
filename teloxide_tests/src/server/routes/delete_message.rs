use std::sync::Mutex;

use actix_web::error::ErrorBadRequest;
use actix_web::{web, Responder};
use serde::Deserialize;

use crate::server::routes::make_telegram_result;
use crate::server::DeletedMessage;
use crate::state::State;

use super::{check_if_message_exists, BodyChatId};

#[derive(Debug, Deserialize, Clone)]
pub struct DeleteMessageBody {
    pub chat_id: BodyChatId,
    pub message_id: i32,
}

pub async fn delete_message(
    state: web::Data<Mutex<State>>,
    body: web::Json<DeleteMessageBody>,
) -> impl Responder {
    let mut lock = state.lock().unwrap();
    check_if_message_exists!(lock, body.message_id);
    let deleted_message = lock.messages.delete_message(body.message_id).unwrap();
    lock.responses.deleted_messages.push(DeletedMessage {
        message: deleted_message.clone(),
        bot_request: body.into_inner(),
    });

    make_telegram_result(true)
}
