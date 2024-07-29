use core::sync::atomic::{AtomicI32, Ordering};
use super::{MockLocation, MockPhotoSize, MockVideo};
use super::message::Message;
use super::{chat::MockPrivateChat, MockUser};
use chrono::{DateTime, Utc};
use mime::Mime;
use crate::proc_macros::Changeable;
use teloxide::types::*;

macro_rules! MessageCommon {  // Rust was supposed to be used withot inheritance, and yet here i am, reinventing it...
    (
        #[derive($($derive:meta),*)]
        $pub:vis struct $name:ident {
            $($fpub:vis $field:ident : $type:ty,)*
        }
    ) => {
        Message! {  // DRY is dangerous. This looks scary.
            #[derive($($derive),*)]
            $pub struct $name {
                pub from: Option<User>,
                pub sender_chat: Option<Chat>,
                pub author_signature: Option<String>,
                pub forward: Option<Forward>,
                pub reply_to_message: Option<Box<Message>>,
                pub edit_date: Option<DateTime<Utc>>,
                pub reply_markup: Option<InlineKeyboardMarkup>,
                pub is_topic_message: bool,
                pub is_automatic_forward: bool,
                pub has_protected_content: bool,
                $($fpub $field : $type,)*  // Just all of the other fields, nothig too scary here
            }
        }
        impl $name {  // Implements common functions
            pub const IS_TOPIC_MESSAGE: bool = false;  // Constant because why not
            pub const IS_AUTOMATIC_FORWARD: bool = false;
            pub const HAS_PROTECTED_CONTENT: bool = false;

            pub(crate) fn new_message_common($($field:$type,)*) -> Self {
                 $name::new_message(
                     Some(MockUser::new().build()),
                     Some(MockPrivateChat::new().build()), // I feel like a private chat is a sane default
                     None,
                     None,
                     None,
                     None,
                     None,
                     $name::IS_TOPIC_MESSAGE,
                     $name::IS_AUTOMATIC_FORWARD,
                     $name::HAS_PROTECTED_CONTENT,
                     $($field,)*  // All of the other fields from the child struct
                 )
            }

            pub(crate) fn build_message_common(self, media_kind: MediaKind) -> Message {
                self.clone().build_message(MessageKind::Common(MessageCommon {
                    from: self.from,
                    sender_chat: self.sender_chat,
                    author_signature: self.author_signature,
                    forward: self.forward,
                    reply_to_message: self.reply_to_message,
                    edit_date: self.edit_date,
                    reply_markup: self.reply_markup,
                    media_kind,
                    is_topic_message: self.is_topic_message,
                    is_automatic_forward: self.is_automatic_forward,
                    has_protected_content: self.has_protected_content,
                }))
            }
        }
    }
}

/*


        Structs below are just copies of MessageCommon with different MediaKinds, with some flattening of arguments where suitable.

        So boring they could've been autogenerated


*/

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageText {
        pub text: String,
        pub entities: Vec<MessageEntity>,
    }
}

impl MockMessageText {
    pub const TEXT: &'static str = "text";

    /// Creates a new easily changable message text builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessageText::new()
    ///     .text("/start")
    ///     .build();
    /// assert_eq!(message.text().unwrap(), "/start");
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message_common(Self::TEXT.to_string(), vec![])
    }

    /// Builds the message text
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessageText::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.text().unwrap(), teloxide_tests::MockMessageText::TEXT.to_string());  // TEXT is a default value
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Text(MediaText {
                text: self.text,
                entities: self.entities,
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageAnimation {
        pub caption: Option<String>,
        pub caption_entities: Vec<MessageEntity>,
        pub has_media_spoiler: bool,
        // Animation
        pub width: u32,
        pub height: u32,
        pub duration: u32,
        pub thumb: Option<PhotoSize>,
        pub file_name: Option<String>,
        pub mime_type: Option<Mime>,
        // FileMeta
        pub file_id: String,
        pub file_unique_id: String,
        pub file_size: u32,
    }
}

impl MockMessageAnimation {
    pub const HAS_MEDIA_SPOILER: bool = false;
    pub const WIDTH: u32 = 50;
    pub const HEIGHT: u32 = 50;
    pub const DURATION: u32 = 50;
    pub const FILE_ID: &'static str = "file_id";
    pub const UNIQUE_FILE_ID: &'static str = "file_unique_id";
    pub const FILE_SIZE: u32 = 50;

    /// Creates a new easily changable message animation builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessageAnimation::new()
    ///     .has_media_spoiler(true)
    ///     .build();
    /// assert_eq!(message.has_media_spoiler(), true);
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message_common(
            None,
            vec![],
            Self::HAS_MEDIA_SPOILER,
            Self::WIDTH,
            Self::HEIGHT,
            Self::DURATION,
            None,
            None,
            None,
            Self::FILE_ID.to_string(),
            Self::UNIQUE_FILE_ID.to_string(),
            Self::FILE_SIZE,
        )
    }

    /// Builds the message animation
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessageAnimation::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.has_media_spoiler(), teloxide_tests::MockMessageAnimation::HAS_MEDIA_SPOILER);  // HAS_MEDIA_SPOILER is a default value
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Animation(MediaAnimation {
                caption: self.caption,
                caption_entities: self.caption_entities,
                has_media_spoiler: self.has_media_spoiler,
                animation: Animation {
                    file: FileMeta {
                        id: self.file_id,
                        unique_id: self.file_unique_id,
                        size: self.file_size,
                    },
                    width: self.width,
                    height: self.height,
                    duration: self.duration,
                    thumb: self.thumb,
                    file_name: self.file_name,
                    mime_type: self.mime_type,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageAudio {
        pub caption: Option<String>,
        pub caption_entities: Vec<MessageEntity>,
        pub media_group_id: Option<String>,
        // Audio
        pub duration: u32,
        pub performer: Option<String>,
        pub title: Option<String>,
        pub thumb: Option<PhotoSize>,
        pub file_name: Option<String>,
        pub mime_type: Option<Mime>,
        // FileMeta
        pub file_id: String,
        pub file_unique_id: String,
        pub file_size: u32,
    }
}

impl MockMessageAudio {
    pub const DURATION: u32 = 236;
    pub const FILE_ID: &'static str = "CQADAgADbQEAAsnrIUpNoRRNsH7_hAI";
    pub const UNIQUE_FILE_ID: &'static str = "file_unique_id";
    pub const FILE_SIZE: u32 = 9507774;

    /// Creates a new easily changable message audio builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessageAudio::new()
    ///     .duration(236)
    ///     .build();
    /// assert_eq!(message.audio().unwrap().duration, 236);  // DURATION is a default value
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message_common(
            None,
            vec![],
            None,
            Self::DURATION,
            None,
            None,
            None,
            None,
            None,
            Self::FILE_ID.to_string(),
            Self::UNIQUE_FILE_ID.to_string(),
            Self::FILE_SIZE,
        )
    }

    /// Builds the message audio
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessageAudio::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.audio().unwrap().duration, teloxide_tests::MockMessageAudio::DURATION);  // DURATION is a default value
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Audio(MediaAudio {
                caption: self.caption,
                caption_entities: self.caption_entities,
                media_group_id: self.media_group_id,
                audio: Audio {
                    file: FileMeta {
                        id: self.file_id,
                        unique_id: self.file_unique_id,
                        size: self.file_size,
                    },
                    duration: self.duration,
                    performer: self.performer,
                    title: self.title,
                    thumb: self.thumb,
                    file_name: self.file_name,
                    mime_type: self.mime_type,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageContact {
        pub phone_number: String,
        pub first_name: String,
        pub last_name: Option<String>,
        pub user_id: Option<UserId>,
        pub vcard: Option<String>,
    }
}

impl MockMessageContact {
    pub const PHONE_NUMBER: &'static str = "+123456789";
    pub const FIRST_NAME: &'static str = "First";

    /// Creates a new easily changable message contact builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessageContact::new()
    ///     .phone_number("+123456789")
    ///     .build();
    /// assert_eq!(message.contact().unwrap().phone_number, "+123456789");  // PHONE_NUMBER is a default value
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message_common(
            Self::PHONE_NUMBER.to_string(),
            Self::FIRST_NAME.to_string(),
            None,
            None,
            None,
        )
    }

    /// Builds the message contact
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessageContact::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.contact().unwrap().phone_number, teloxide_tests::MockMessageContact::PHONE_NUMBER);  // PHONE_NUMBER is a default value
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Contact(MediaContact {
                contact: Contact {
                    phone_number: self.phone_number,
                    first_name: self.first_name,
                    last_name: self.last_name,
                    user_id: self.user_id,
                    vcard: self.vcard,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageDocument {
        pub caption: Option<String>,
        pub caption_entities: Vec<MessageEntity>,
        pub media_group_id: Option<String>,
        // Document
        pub thumb: Option<PhotoSize>,
        pub file_name: Option<String>,
        pub mime_type: Option<Mime>,
        // FileMeta
        pub file_id: String,
        pub file_unique_id: String,
        pub file_size: u32,
    }
}

impl MockMessageDocument {
    pub const FILE_ID: &'static str = "BQADAgADpgADy_JxS66XQTBRHFleAg";
    pub const UNIQUE_FILE_ID: &'static str = "file_unique_id";
    pub const FILE_SIZE: u32 = 21331;

    /// Creates a new easily changable message document builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessageDocument::new()
    ///     .file_id("12345")
    ///     .build();
    /// assert_eq!(message.document().unwrap().file.id, "12345");  // FILE_ID is a default value
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message_common(
            None,
            vec![],
            None,
            None,
            None,
            None,
            Self::FILE_ID.to_string(),
            Self::UNIQUE_FILE_ID.to_string(),
            Self::FILE_SIZE,
        )
    }

    /// Builds the message document
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessageDocument::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.document().unwrap().file.id, teloxide_tests::MockMessageDocument::FILE_ID);  // FILE_ID is a default value
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Document(MediaDocument {
                caption: self.caption,
                caption_entities: self.caption_entities,
                media_group_id: self.media_group_id,
                document: Document {
                    file: FileMeta {
                        id: self.file_id,
                        unique_id: self.file_unique_id,
                        size: self.file_size,
                    },
                    thumb: self.thumb,
                    file_name: self.file_name,
                    mime_type: self.mime_type,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageGame {
        pub title: String,
        pub description: String,
        pub photo: Vec<PhotoSize>,
        pub text: Option<String>,
        pub text_entities: Option<Vec<MessageEntity>>,
        pub animation: Option<Animation>,
    }
}

impl MockMessageGame {
    pub const TITLE: &'static str = "Title";
    pub const DESCRIPTION: &'static str = "Description";

    /// Creates a new easily changable message game builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessageGame::new()
    ///     .title("title")
    ///     .build();
    /// assert_eq!(message.game().unwrap().title, "title");  // TITLE is a default value
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message_common(
            Self::TITLE.to_string(),
            Self::DESCRIPTION.to_string(),
            vec![MockPhotoSize::new().build()],
            None,
            None,
            None,
        )
    }

    /// Builds the message game
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessageGame::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.game().unwrap().title, teloxide_tests::MockMessageGame::TITLE);  // TITLE is a default value
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Game(MediaGame {
                game: Game {
                    title: self.title,
                    description: self.description,
                    photo: self.photo,
                    text: self.text,
                    text_entities: self.text_entities,
                    animation: self.animation,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageVenue {
        pub location: Location,
        pub title: String,
        pub address: String,
        pub foursquare_id: Option<String>,
        pub foursquare_type: Option<String>,
        pub google_place_id: Option<String>,
        pub google_place_type: Option<String>,
    }
}

impl MockMessageVenue {
    pub const TITLE: &'static str = "Title";
    pub const ADDRESS: &'static str = "Address";

    /// Creates a new easily changable message venue builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessageVenue::new()
    ///     .title("title")
    ///     .build();
    /// assert_eq!(message.venue().unwrap().title, "title");  // TITLE is a default value
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message_common(
            MockLocation::new().build(),
            Self::TITLE.to_string(),
            Self::ADDRESS.to_string(),
            None,
            None,
            None,
            None,
        )
    }

    /// Builds the message venue
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessageVenue::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.venue().unwrap().title, teloxide_tests::MockMessageVenue::TITLE);  // TITLE is a default value
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Venue(MediaVenue {
                venue: Venue {
                    location: self.location,
                    title: self.title,
                    address: self.address,
                    foursquare_id: self.foursquare_id,
                    foursquare_type: self.foursquare_type,
                    google_place_id: self.google_place_id,
                    google_place_type: self.google_place_type,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageLocation {
        pub latitude: f64,
        pub longitude: f64,
        pub horizontal_accuracy: Option<f64>,
        pub live_period: Option<u32>,
        pub heading: Option<u16>,
        pub proximity_alert_radius: Option<u32>,
    }
}

impl MockMessageLocation {
    pub const LATITUDE: f64 = 50.0;
    pub const LONGITUDE: f64 = 30.0;

    /// Creates a new easily changable message location builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessageLocation::new()
    ///     .latitude(50.0)
    ///     .build();
    /// assert_eq!(message.location().unwrap().latitude, 50.0);  // LATITUDE is a default value
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message_common(Self::LATITUDE, Self::LONGITUDE, None, None, None, None)
    }

    /// Builds the message location
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessageLocation::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.location().unwrap().latitude, teloxide_tests::MockMessageLocation::LATITUDE);  // LATITUDE is a default value
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Location(MediaLocation {
                location: Location {
                    longitude: self.longitude,
                    latitude: self.latitude,
                    horizontal_accuracy: self.horizontal_accuracy,
                    live_period: self.live_period,
                    heading: self.heading,
                    proximity_alert_radius: self.proximity_alert_radius,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessagePhoto {
        pub caption: Option<String>,
        pub caption_entities: Vec<MessageEntity>,
        pub media_group_id: Option<String>,
        pub has_media_spoiler: bool,
        pub photo: Vec<PhotoSize>,
    }
}

impl MockMessagePhoto {
    pub const HAS_MEDIA_SPOILER: bool = false;

    /// Creates a new easily changable message photo builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessagePhoto::new()
    ///     .has_media_spoiler(true)
    ///     .photo(vec![teloxide_tests::MockPhotoSize::new().build(); 3])
    ///     .build();
    /// assert_eq!(message.has_media_spoiler(), true);
    /// assert_eq!(message.photo().unwrap().len(), 3);
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message_common(
            None,
            vec![],
            None,
            Self::HAS_MEDIA_SPOILER,
            vec![MockPhotoSize::new().build()],
        )
    }

    /// Builds the message photo
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessagePhoto::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.photo().unwrap().len(), 1);  // By default, there is only one photo, just MockPhotoSize::new().build()
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Photo(MediaPhoto {
                caption: self.caption,
                caption_entities: self.caption_entities,
                media_group_id: self.media_group_id,
                has_media_spoiler: self.has_media_spoiler,
                photo: self.photo,
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessagePoll {
        pub poll_id: String,
        pub question: String,
        pub options: Vec<PollOption>,
        pub is_closed: bool,
        pub total_voter_count: i32,
        pub is_anonymous: bool,
        pub poll_type: PollType,
        pub allows_multiple_answers: bool,
        pub correct_option_id: Option<u8>,
        pub explanation: Option<String>,
        pub explanation_entities: Option<Vec<MessageEntity>>,
        pub open_period: Option<u16>,
        pub close_date: Option<DateTime<Utc>>,
    }
}

impl MockMessagePoll {
    pub const POLL_ID: &'static str = "12345";
    pub const QUESTION: &'static str = "Question";
    pub const IS_CLOSED: bool = true;
    pub const IS_ANONYMOUS: bool = true;
    pub const TOTAL_VOTER_COUNT: i32 = 50;
    pub const POLL_TYPE: PollType = PollType::Regular;
    pub const ALLOW_MULTIPLE_ANSWERS: bool = true;

    /// Creates a new easily changable message poll builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessagePoll::new()
    ///     .poll_id("123456")
    ///     .build();
    ///
    /// assert_eq!(message.poll().unwrap().id, "123456");
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message_common(
            Self::POLL_ID.to_string(),
            Self::QUESTION.to_string(),
            vec![],
            Self::IS_CLOSED,
            Self::TOTAL_VOTER_COUNT,
            Self::IS_ANONYMOUS,
            Self::POLL_TYPE,
            Self::ALLOW_MULTIPLE_ANSWERS,
            None,
            None,
            None,
            None,
            None,
        )
    }

    /// Builds the message poll
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessagePoll::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.poll().unwrap().id, teloxide_tests::MockMessagePoll::POLL_ID);  // POLL_ID is a default value
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Poll(MediaPoll {
                poll: Poll {
                    id: self.poll_id,
                    question: self.question,
                    options: self.options,
                    is_closed: self.is_closed,
                    total_voter_count: self.total_voter_count,
                    is_anonymous: self.is_anonymous,
                    poll_type: self.poll_type,
                    allows_multiple_answers: self.allows_multiple_answers,
                    correct_option_id: self.correct_option_id,
                    explanation: self.explanation,
                    explanation_entities: self.explanation_entities,
                    open_period: self.open_period,
                    close_date: self.close_date,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageSticker {
        pub width: u16,
        pub height: u16,
        pub kind: StickerKind,
        pub format: StickerFormat,
        pub thumb: Option<PhotoSize>,
        pub emoji: Option<String>,
        pub set_name: Option<String>,
        // File meta
        pub file_id: String,
        pub file_unique_id: String,
        pub file_size: u32,
    }
}

impl MockMessageSticker {
    pub const WIDTH: u16 = 512;
    pub const HEIGHT: u16 = 512;
    pub const KIND: StickerKind = StickerKind::Regular {
        premium_animation: None,
    };
    pub const FORMAT: StickerFormat = StickerFormat::Raster;
    pub const FILE_ID: &'static str = "AAbbCCddEEffGGhh1234567890";
    pub const FILE_UNIQUE_ID: &'static str = "file_unique_id";
    pub const FILE_SIZE: u32 = 12345;

    /// Creates a new easily changable message sticker builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessageSticker::new().file_id("12345").build();
    ///
    /// assert_eq!(message.sticker().unwrap().file.id, "12345");
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message_common(
            Self::WIDTH,
            Self::HEIGHT,
            Self::KIND,
            Self::FORMAT,
            None,
            None,
            None,
            Self::FILE_ID.to_string(),
            Self::FILE_UNIQUE_ID.to_string(),
            Self::FILE_SIZE,
        )
    }

    /// Builds the message sticker
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessageSticker::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.sticker().unwrap().file.id, teloxide_tests::MockMessageSticker::FILE_ID);  // FILE_ID is a default value
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Sticker(MediaSticker {
                sticker: Sticker {
                    file: FileMeta {
                        id: self.file_id,
                        unique_id: self.file_unique_id,
                        size: self.file_size,
                    },
                    width: self.width,
                    height: self.height,
                    kind: self.kind,
                    format: self.format,
                    thumb: self.thumb,
                    emoji: self.emoji,
                    set_name: self.set_name,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageVideo {
        pub caption: Option<String>,
        pub caption_entities: Vec<MessageEntity>,
        pub media_group_id: Option<String>,
        pub has_media_spoiler: bool,
        pub video: Video,
    }
}

impl MockMessageVideo {
    pub const HAS_MEDIA_SPOILER: bool = false;

    /// Creates a new easily changable message video builder
    ///
    /// # Example
    /// ```
    /// let message =
    /// teloxide_tests::MockMessageVideo::new().video(teloxide_tests::MockVideo::new().file_id("12345").build()).build();
    /// assert_eq!(message.video().unwrap().file.id, "12345");
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message_common(
            None,
            vec![],
            None,
            Self::HAS_MEDIA_SPOILER,
            MockVideo::new().build(),
        )
    }

    /// Builds the message video
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessageVideo::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.video().unwrap().file.id, teloxide_tests::MockVideo::FILE_ID);  // FILE_ID is a default value
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Video(MediaVideo {
                caption: self.caption,
                caption_entities: self.caption_entities,
                media_group_id: self.media_group_id,
                has_media_spoiler: self.has_media_spoiler,
                video: self.video,
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageVideoNote {
        pub length: u32,
        pub duration: u32,
        pub thumb: Option<PhotoSize>,
        // File meta
        pub file_id: String,
        pub file_unique_id: String,
        pub file_size: u32,
    }
}

impl MockMessageVideoNote {
    pub const LENGTH: u32 = 50;
    pub const DURATION: u32 = 50;
    pub const FILE_ID: &'static str = "file_id";
    pub const FILE_UNIQUE_ID: &'static str = "file_unique_id";
    pub const FILE_SIZE: u32 = 50;

    /// Creates a new easily changable message video note builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessageVideoNote::new().length(50).build();
    /// assert_eq!(message.video_note().unwrap().length, 50);
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message_common(
            Self::LENGTH,
            Self::DURATION,
            None,
            Self::FILE_ID.to_string(),
            Self::FILE_UNIQUE_ID.to_string(),
            Self::FILE_SIZE,
        )
    }

    /// Builds the message video note
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessageVideoNote::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.video_note().unwrap().file.id, teloxide_tests::MockMessageVideoNote::FILE_ID);  // FILE_ID is a default value
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::VideoNote(MediaVideoNote {
                video_note: VideoNote {
                    file: FileMeta {
                        id: self.file_id,
                        unique_id: self.file_unique_id,
                        size: self.file_size,
                    },
                    length: self.length,
                    duration: self.duration,
                    thumb: self.thumb,
                },
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageVoice {
        pub duration: u32,
        pub mime_type: Option<Mime>,
        pub caption: Option<String>,
        pub caption_entities: Vec<MessageEntity>,
        // File meta
        pub file_id: String,
        pub file_unique_id: String,
        pub file_size: u32,
    }
}

impl MockMessageVoice {
    pub const DURATION: u32 = 1;
    pub const FILE_ID: &'static str = "AwADawAgADADy_JxS2gopIVIIxlhAg";
    pub const FILE_UNIQUE_ID: &'static str = "file_unique_id";
    pub const FILE_SIZE: u32 = 4321;

    /// Creates a new easily changable message voice builder
    ///
    /// # Example
    /// ```
    /// let message = teloxide_tests::MockMessageVoice::new().duration(1).build();
    /// assert_eq!(message.voice().unwrap().duration, 1);
    /// ```
    ///
    pub fn new() -> Self {
        Self::new_message_common(
            Self::DURATION,
            None,
            None,
            vec![],
            Self::FILE_ID.to_string(),
            Self::FILE_UNIQUE_ID.to_string(),
            Self::FILE_SIZE,
        )
    }

    /// Builds the message voice
    ///
    /// # Example
    /// ```
    /// let mock_message = teloxide_tests::MockMessageVoice::new();
    /// let message = mock_message.build();
    /// assert_eq!(message.voice().unwrap().file.id, teloxide_tests::MockMessageVoice::FILE_ID);  // FILE_ID is a default value
    /// ```
    ///
    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Voice(MediaVoice {
                voice: Voice {
                    file: FileMeta {
                        id: self.file_id,
                        unique_id: self.file_unique_id,
                        size: self.file_size,
                    },
                    duration: self.duration,
                    mime_type: self.mime_type,
                },
                caption: self.caption,
                caption_entities: self.caption_entities,
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageMigrationFromChat {
        pub migrate_from_chat_id: i64,
    }
}

impl MockMessageMigrationFromChat {
    pub const MIGRATE_FROM_CHAT_ID: i64 = 1;

    pub fn new() -> Self {
        Self::new_message_common(Self::MIGRATE_FROM_CHAT_ID)
    }

    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Migration(ChatMigration::From {
                chat_id: ChatId(self.migrate_from_chat_id),
            }))
    }
}

MessageCommon! {
    #[derive(Changeable, Clone)]
    pub struct MockMessageMigrationToChat {
        pub migrate_to_chat_id: i64,
    }
}

impl MockMessageMigrationToChat {
    pub const MIGRATE_TO_CHAT_ID: i64 = 1;

    pub fn new() -> Self {
        Self::new_message_common(Self::MIGRATE_TO_CHAT_ID)
    }

    pub fn build(self) -> Message {
        self.clone()
            .build_message_common(MediaKind::Migration(ChatMigration::To {
                chat_id: ChatId(self.migrate_to_chat_id),
            }))
    }
}