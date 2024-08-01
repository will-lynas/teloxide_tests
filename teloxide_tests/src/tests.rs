use super::*;
use crate::dataset::*;
use serde::{Deserialize, Serialize};
use teloxide::dispatching::{HandlerExt, UpdateHandler};
use teloxide::dptree::case;
use teloxide::net::Download;
use teloxide::payloads::{BanChatMemberSetters, CopyMessageSetters};
use teloxide::requests::Requester;
use teloxide::types::{
    ChatPermissions, InlineKeyboardButton, InlineKeyboardMarkup, InputFile, InputMedia,
    InputMediaAudio, InputMediaDocument, InputMediaPhoto, InputMediaVideo, Message, MessageEntity,
    Update,
};
use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateFilterExt,
    },
    dptree::deps,
    macros::BotCommands,
    prelude::*,
};

//
//
//

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
enum State {
    #[default]
    Start,
    NotStart,
}

async fn handler_with_state(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    bot.send_message(msg.chat.id, msg.text().unwrap()).await?;

    dialogue.update(State::NotStart).await?;
    Ok(())
}

async fn handler_with_not_start_state(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    bot.send_message(msg.chat.id, "Not start!").await?;

    dialogue.update(State::Start).await?;
    Ok(())
}

fn get_dialogue_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dialogue::enter::<Update, InMemStorage<State>, State, _>().branch(
        Update::filter_message()
            .branch(case![State::NotStart].endpoint(handler_with_not_start_state))
            .endpoint(handler_with_state),
    )
}

#[tokio::test]
async fn test_echo_with_start_state() {
    let bot = MockBot::new(MockMessageText::new().text("test"), get_dialogue_schema());
    let storage = InMemStorage::<State>::new();
    bot.dependencies(deps![storage]);
    bot.set_state(State::Start).await;

    bot.dispatch().await;

    let last_response = bot.get_responses().sent_messages.pop().unwrap();
    let state: State = bot.get_state().await;
    assert_eq!(state, State::NotStart);

    assert_eq!(last_response.text(), Some("test"));
}

#[tokio::test]
async fn test_echo_with_not_start_test() {
    let bot = MockBot::new(MockMessageText::new().text("test"), get_dialogue_schema());
    let storage = InMemStorage::<State>::new();
    bot.dependencies(deps![storage]);
    bot.set_state(State::NotStart).await;

    bot.dispatch().await;

    let last_response = bot.get_responses().sent_messages.pop().unwrap();
    let state: State = bot.get_state().await;
    assert_eq!(state, State::Start);

    assert_eq!(last_response.text(), Some("Not start!"));
}

//
//
//

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum AllCommands {
    #[command()]
    Echo,
    #[command()]
    Edit,
    #[command()]
    Delete,
    #[command()]
    EditReplyMarkup,
    #[command()]
    Photo,
    #[command()]
    Video,
    #[command()]
    Audio,
    #[command()]
    Voice,
    #[command()]
    VideoNote,
    #[command()]
    Document,
    #[command()]
    Animation,
    #[command()]
    MediaGroup,
    #[command()]
    EditCaption,
    #[command()]
    PinMessage,
    #[command()]
    ForwardMessage,
    #[command()]
    CopyMessage,
    #[command()]
    Ban,
    #[command()]
    Restrict,
}

type MyDialogue = Dialogue<State, InMemStorage<State>>;

async fn handler(
    bot: Bot,
    msg: Message,
    cmd: AllCommands,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let sent_message = bot.send_message(msg.chat.id, msg.text().unwrap()).await?;
    assert!(msg.text().unwrap() == sent_message.text().unwrap()); // The message actually made it through!
    match cmd {
        AllCommands::Echo => {}
        AllCommands::Edit => {
            bot.edit_message_text(msg.chat.id, sent_message.id, "edited")
                .await?;
        }
        AllCommands::Delete => {
            bot.delete_message(msg.chat.id, sent_message.id).await?;
        }
        AllCommands::EditReplyMarkup => {
            bot.edit_message_reply_markup(msg.chat.id, sent_message.id)
                .reply_markup(InlineKeyboardMarkup::new(vec![vec![
                    InlineKeyboardButton::callback("test", "test"),
                ]]))
                .await?;
        }
        AllCommands::Photo => {
            let photo = InputFile::memory("somedata".to_string()).file_name("test.jpg");
            bot.send_photo(msg.chat.id, photo)
                .caption("test")
                .caption_entities(vec![MessageEntity::bold(0, 3)])
                .reply_to_message_id(msg.id)
                .await?;
        }
        AllCommands::Video => {
            let video = InputFile::memory("somedata".to_string()).file_name("test.mp4");
            bot.send_video(msg.chat.id, video)
                .caption("test")
                .caption_entities(vec![MessageEntity::bold(0, 3)])
                .reply_to_message_id(msg.id)
                .await?;
        }
        AllCommands::Audio => {
            let audio = InputFile::memory("somedata".to_string()).file_name("test.mp3");
            bot.send_audio(msg.chat.id, audio)
                .caption("test")
                .caption_entities(vec![MessageEntity::bold(0, 3)])
                .reply_to_message_id(msg.id)
                .await?;
        }
        AllCommands::Voice => {
            let voice = InputFile::memory("somedata".to_string()).file_name("test.mp3");
            bot.send_voice(msg.chat.id, voice)
                .caption("test")
                .caption_entities(vec![MessageEntity::bold(0, 3)])
                .reply_to_message_id(msg.id)
                .await?;
        }
        AllCommands::VideoNote => {
            let video_note = InputFile::memory("somedata".to_string()).file_name("test.mp4");
            bot.send_video_note(msg.chat.id, video_note)
                .reply_to_message_id(msg.id)
                .await?;
        }
        AllCommands::EditCaption => {
            let photo = InputFile::file_id("fileid".to_string());
            let photo_message = bot.send_photo(msg.chat.id, photo).await?;
            bot.edit_message_caption(msg.chat.id, photo_message.id)
                .caption("edited")
                .await?;
        }
        AllCommands::Document => {
            let document =
                InputFile::file("/home/laster/http_requests.txt".to_string()).file_name("test.txt");
            let document_message = bot
                .send_document(msg.chat.id, document)
                .caption("test")
                .caption_entities(vec![MessageEntity::bold(0, 3)])
                .reply_to_message_id(msg.id)
                .await?;
            let gotten_document = bot
                .get_file(document_message.document().unwrap().file.id.clone())
                .await?;
            assert!(
                gotten_document.meta.unique_id
                    == document_message.document().unwrap().file.unique_id
            );
            let mut dest = tokio::fs::File::create("test.txt").await?;

            bot.download_file(&gotten_document.path, &mut dest).await?;
            assert!(tokio::fs::read_to_string("test.txt").await.is_ok());
            tokio::fs::remove_file("test.txt").await?;
        }
        AllCommands::Animation => {
            let animation = InputFile::memory("somedata".to_string()).file_name("animation.mp4");
            bot.send_animation(msg.chat.id, animation).await?;
        }
        AllCommands::MediaGroup => {
            let audio1 = InputFile::memory("somedata".to_string()).file_name("audio1.mp3");
            let audio2 = InputFile::memory("somedata2".to_string()).file_name("audio2.mp3");
            let media_group = vec![
                InputMedia::Audio(InputMediaAudio::new(audio1.clone()).caption("test")),
                InputMedia::Audio(InputMediaAudio::new(audio2.clone())),
            ];
            bot.send_media_group(msg.chat.id, media_group)
                .reply_to_message_id(msg.id)
                .await?;

            let document1 = InputFile::memory("somedata".to_string()).file_name("document1.txt");
            let document2 = InputFile::memory("somedata2".to_string()).file_name("document2.txt");
            let media_group = vec![
                InputMedia::Document(InputMediaDocument::new(document1.clone()).caption("test")),
                InputMedia::Document(InputMediaDocument::new(document2.clone())),
            ];
            bot.send_media_group(msg.chat.id, media_group)
                .reply_to_message_id(msg.id)
                .await?;

            let photo1 = InputFile::memory("somedata".to_string());
            let photo2 = InputFile::memory("somedata2".to_string());
            let media_group = vec![
                InputMedia::Photo(InputMediaPhoto::new(photo1.clone()).caption("test")),
                InputMedia::Photo(InputMediaPhoto::new(photo2.clone())),
            ];
            bot.send_media_group(msg.chat.id, media_group)
                .reply_to_message_id(msg.id)
                .await?;

            let video1 = InputFile::memory("somedata".to_string()).file_name("video1.mp4");
            let video2 = InputFile::memory("somedata2".to_string()).file_name("video2.mp4");
            let media_group = vec![
                InputMedia::Video(InputMediaVideo::new(video1.clone()).caption("test")),
                InputMedia::Video(InputMediaVideo::new(video2.clone())),
            ];
            bot.send_media_group(msg.chat.id, media_group)
                .reply_to_message_id(msg.id)
                .await?;
        }
        AllCommands::PinMessage => {
            bot.pin_chat_message(msg.chat.id, sent_message.id).await?;
            bot.unpin_chat_message(msg.chat.id).await?;
            bot.unpin_all_chat_messages(msg.chat.id).await?;
        }
        AllCommands::ForwardMessage => {
            bot.forward_message(msg.chat.id, msg.chat.id, sent_message.id)
                .await?;
        }
        AllCommands::CopyMessage => {
            let document =
                InputFile::file("/home/laster/http_requests.txt".to_string()).file_name("test.txt");
            let document_message = bot.send_document(msg.chat.id, document).await?;
            bot.copy_message(msg.chat.id, msg.chat.id, document_message.id)
                .caption("test")
                .reply_markup(InlineKeyboardMarkup::new(vec![vec![
                    InlineKeyboardButton::callback("test", "test"),
                ]]))
                .await?;
        }
        AllCommands::Ban => {
            bot.ban_chat_member(msg.chat.id, msg.from().unwrap().id)
                .revoke_messages(true)
                .await?;
            // Test revoking messages
            let result = bot.delete_message(msg.chat.id, msg.id).await;
            assert!(result.is_err());
            bot.unban_chat_member(msg.chat.id, msg.from().unwrap().id)
                .await?;
        }
        AllCommands::Restrict => {
            bot.restrict_chat_member(
                msg.chat.id,
                msg.from().unwrap().id,
                ChatPermissions::empty(),
            )
            .await?;
        }
    }
    Ok(())
}

async fn callback_handler(
    bot: Bot,
    call: CallbackQuery,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    bot.answer_callback_query(call.id)
        .text(call.data.unwrap())
        .await?;
    Ok(())
}

fn get_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<AllCommands>()
                .endpoint(handler),
        )
        .branch(Update::filter_message().endpoint(handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
}

#[tokio::test]
async fn test_echo() {
    let bot = MockBot::new(MockMessageText::new().text("/echo echo"), get_schema());

    bot.dispatch().await;

    let last_response = bot.get_responses().sent_messages.pop().unwrap();

    assert_eq!(last_response.text(), Some("/echo echo"));
}

#[tokio::test]
#[should_panic]
async fn test_panic() {
    // Nothing else should fail because it panics
    let bot = MockBot::new(MockMessageText::new().text("/echo echo"), get_schema());

    bot.dispatch().await;

    let last_response = bot.get_responses().sent_messages.pop().unwrap();
    if last_response.text() == Some("/echo echo") {
        panic!("panic!");
    }

    drop(bot);
}

#[tokio::test]
async fn test_send_photo() {
    let bot = MockBot::new(MockMessageText::new().text("/photo"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_sent_photo = bot.get_responses().sent_messages_photo.pop().unwrap();
    assert_eq!(last_sent_message.caption(), Some("test"));
    assert_eq!(
        last_sent_message.reply_to_message().unwrap().text(),
        Some("/photo")
    );
    assert_eq!(last_sent_message.caption_entities().unwrap().len(), 1);
    assert_eq!(last_sent_photo.bot_request.file_name, "test.jpg");
    assert_eq!(last_sent_photo.bot_request.file_data, "somedata");
}

#[tokio::test]
async fn test_send_video() {
    let bot = MockBot::new(MockMessageText::new().text("/video"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_sent_video = bot.get_responses().sent_messages_video.pop().unwrap();
    assert_eq!(last_sent_message.caption(), Some("test"));
    assert_eq!(
        last_sent_message.reply_to_message().unwrap().text(),
        Some("/video")
    );
    assert_eq!(last_sent_message.caption_entities().unwrap().len(), 1);
    assert_eq!(last_sent_video.bot_request.file_name, "test.mp4");
    assert_eq!(last_sent_video.bot_request.file_data, "somedata");
}

#[tokio::test]
async fn test_send_audio() {
    let bot = MockBot::new(MockMessageText::new().text("/audio"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_sent_audio = bot.get_responses().sent_messages_audio.pop().unwrap();
    assert_eq!(last_sent_message.caption(), Some("test"));
    assert_eq!(
        last_sent_message.reply_to_message().unwrap().text(),
        Some("/audio")
    );
    assert_eq!(last_sent_message.caption_entities().unwrap().len(), 1);
    assert_eq!(last_sent_audio.bot_request.file_name, "test.mp3");
    assert_eq!(last_sent_audio.bot_request.file_data, "somedata");
}

#[tokio::test]
async fn test_send_voice() {
    let bot = MockBot::new(MockMessageText::new().text("/voice"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_sent_voice = bot.get_responses().sent_messages_voice.pop().unwrap();
    assert_eq!(last_sent_message.caption(), Some("test"));
    assert_eq!(
        last_sent_message.reply_to_message().unwrap().text(),
        Some("/voice")
    );
    assert_eq!(last_sent_message.caption_entities().unwrap().len(), 1);
    assert_eq!(last_sent_voice.bot_request.file_name, "test.mp3");
    assert_eq!(last_sent_voice.bot_request.file_data, "somedata");
}

#[tokio::test]
async fn test_send_video_note() {
    let bot = MockBot::new(MockMessageText::new().text("/videonote"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_sent_video_note = bot.get_responses().sent_messages_video_note.pop().unwrap();
    assert_eq!(
        last_sent_message.reply_to_message().unwrap().text(),
        Some("/videonote")
    );
    assert_eq!(last_sent_video_note.bot_request.file_name, "test.mp4");
    assert_eq!(last_sent_video_note.bot_request.file_data, "somedata");
}

#[tokio::test]
async fn test_send_document() {
    let bot = MockBot::new(MockMessageText::new().text("/document"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_sent_photo = bot.get_responses().sent_messages_document.pop().unwrap();
    assert_eq!(last_sent_message.caption(), Some("test"));
    assert_eq!(
        last_sent_message.reply_to_message().unwrap().text(),
        Some("/document")
    );
    assert_eq!(last_sent_message.caption_entities().unwrap().len(), 1);
    assert_eq!(last_sent_photo.bot_request.file_name, "test.txt");
}

#[tokio::test]
async fn test_send_animation() {
    let bot = MockBot::new(MockMessageText::new().text("/animation"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_sent_animation = bot.get_responses().sent_messages_animation.pop().unwrap();
    assert_eq!(
        last_sent_message.animation().unwrap().file_name,
        Some("animation.mp4".to_string())
    );
    assert_eq!(last_sent_animation.bot_request.file_name, "animation.mp4");
}

#[tokio::test]
async fn test_send_media_group() {
    let bot = MockBot::new(MockMessageText::new().text("/mediagroup"), get_schema());

    bot.dispatch().await;

    let responses = bot.get_responses();

    let audio_group = responses.sent_media_group[0].clone();
    assert_eq!(audio_group.messages.first().unwrap().caption(), Some("test"));
    assert_eq!(audio_group.messages.first().unwrap().audio().unwrap().file_name, Some("audio1.mp3".to_string()));
    assert_eq!(audio_group.messages.first().unwrap().reply_to_message().unwrap().text(), Some("/mediagroup"));
    assert_eq!(audio_group.bot_request.media.len(), 2);

    let document_group = responses.sent_media_group[1].clone();
    assert_eq!(document_group.messages.first().unwrap().caption(), Some("test"));
    assert_eq!(document_group.messages.first().unwrap().document().unwrap().file_name, Some("document1.txt".to_string()));
    assert_eq!(document_group.messages.first().unwrap().reply_to_message().unwrap().text(), Some("/mediagroup"));
    assert_eq!(document_group.bot_request.media.len(), 2);

    let photo_group = responses.sent_media_group[2].clone();
    assert_eq!(photo_group.messages.first().unwrap().caption(), Some("test"));
    assert!(!photo_group.messages.first().unwrap().photo().unwrap().is_empty());
    assert_eq!(photo_group.messages.first().unwrap().reply_to_message().unwrap().text(), Some("/mediagroup"));
    assert_eq!(photo_group.bot_request.media.len(), 2);

    let video_group = responses.sent_media_group[3].clone();
    assert_eq!(video_group.messages.first().unwrap().caption(), Some("test"));
    assert_eq!(video_group.messages.first().unwrap().video().unwrap().file_name, Some("video1.mp4".to_string()));
    assert_eq!(video_group.messages.first().unwrap().reply_to_message().unwrap().text(), Some("/mediagroup"));
    assert_eq!(video_group.bot_request.media.len(), 2);
}

#[tokio::test]
async fn test_edit_message() {
    let bot = MockBot::new(MockMessageText::new().text("/edit"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_edited_response = bot.get_responses().edited_messages_text.pop().unwrap();

    assert_eq!(last_sent_message.text(), Some("/edit"));
    assert_eq!(last_edited_response.message.text(), Some("edited"));
}

#[tokio::test]
async fn test_edit_caption() {
    let bot = MockBot::new(MockMessageText::new().text("/editcaption"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_edited_response = bot.get_responses().edited_messages_caption.pop().unwrap();

    assert_eq!(last_sent_message.caption(), None);
    assert_eq!(last_edited_response.message.caption(), Some("edited"));
}

#[tokio::test]
async fn test_edit_reply_markup() {
    let bot = MockBot::new(
        MockMessageText::new().text("/editreplymarkup"),
        get_schema(),
    );

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_edited_response = bot
        .get_responses()
        .edited_messages_reply_markup
        .pop()
        .unwrap();

    assert_eq!(last_sent_message.reply_markup(), None);
    assert_eq!(
        last_edited_response
            .message
            .reply_markup()
            .unwrap()
            .inline_keyboard[0][0]
            .text,
        "test"
    );
}

#[tokio::test]
async fn test_delete_message() {
    let bot = MockBot::new(MockMessageText::new().text("/delete"), get_schema());

    bot.dispatch().await;

    let last_sent_message = bot.get_responses().sent_messages.pop().unwrap();
    let last_deleted_response = bot.get_responses().deleted_messages.pop().unwrap();

    assert_eq!(last_sent_message.text(), Some("/delete"));
    assert_eq!(last_deleted_response.message.id, last_sent_message.id);
}

#[tokio::test]
async fn test_answer_callback_query() {
    let bot = MockBot::new(MockCallbackQuery::new().data("test"), get_schema());

    bot.dispatch().await;

    let answered_callback = bot.get_responses().answered_callback_queries.pop().unwrap();

    assert_eq!(answered_callback.text, Some("test".to_string()));
}

#[tokio::test]
async fn test_pin_message() {
    let bot = MockBot::new(MockMessageText::new().text("/pinmessage"), get_schema());

    bot.dispatch().await;

    let pinned_message = bot.get_responses().pinned_chat_messages.pop();
    let unpinned_message = bot.get_responses().unpinned_chat_messages.pop();
    let unpinned_all_chat_messages = bot.get_responses().unpinned_all_chat_messages.pop();

    assert!(pinned_message.is_some());
    assert!(unpinned_message.is_some());
    assert!(unpinned_all_chat_messages.is_some());
}

#[tokio::test]
async fn test_forward_message() {
    let bot = MockBot::new(MockMessageText::new().text("/forwardmessage"), get_schema());

    bot.dispatch().await;

    let responses = bot.get_responses();
    let first_sent_message = responses.sent_messages.first().unwrap();
    let last_sent_message = responses.sent_messages.last().unwrap();

    assert_eq!(last_sent_message.text(), Some("/forwardmessage"));
    assert_eq!(
        last_sent_message.forward_date(),
        Some(first_sent_message.date)
    );
}

#[tokio::test]
async fn test_copy_message() {
    let bot = MockBot::new(MockMessageText::new().text("/copymessage"), get_schema());

    bot.dispatch().await;

    let responses = bot.get_responses();
    let second_sent_message = responses.sent_messages.get(1).unwrap();
    let last_sent_message = responses.sent_messages.last().unwrap();

    assert!(second_sent_message.caption().is_none());
    assert!(last_sent_message.document().is_some());
    assert_eq!(last_sent_message.caption(), Some("test"));
    assert_eq!(
        last_sent_message.reply_markup().unwrap().inline_keyboard[0][0].text,
        "test"
    );
}

#[tokio::test]
async fn test_ban_and_unban() {
    let bot = MockBot::new(MockMessageText::new().text("/ban"), get_schema());

    bot.dispatch().await;

    let responses = bot.get_responses();
    let banned_user = responses.banned_chat_members.last().unwrap();
    let unbanned_user = responses.unbanned_chat_members.last().unwrap();

    assert_eq!(banned_user.user_id, MockUser::ID);
    assert_eq!(unbanned_user.user_id, MockUser::ID);
}

#[tokio::test]
async fn test_restrict() {
    let bot = MockBot::new(MockMessageText::new().text("/restrict"), get_schema());

    bot.dispatch().await;

    let responses = bot.get_responses();
    let restricted_user = responses.restricted_chat_members.last().unwrap();

    assert_eq!(restricted_user.user_id, MockUser::ID);
    assert_eq!(restricted_user.permissions, ChatPermissions::empty());
}
