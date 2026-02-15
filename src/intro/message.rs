pub enum Message {
    Text {
        text: &'static str,
        clears_screen: bool,
        delay: f32,
    },
    Pause,
}

impl Message {
    const fn new(text: &'static str, delay: f32) -> Self {
        Self::Text {
            text,
            clears_screen: false,
            delay,
        }
    }

    const fn new_clear(text: &'static str, delay: f32) -> Self {
        Self::Text {
            text,
            clears_screen: true,
            delay,
        }
    }
}

#[cfg(not(feature = "dev"))]
const SHORT_MSG: f32 = 3.;
#[cfg(feature = "dev")]
const SHORT_MSG: f32 = 1.;

#[cfg(not(feature = "dev"))]
const LONG_MSG: f32 = 5.;
#[cfg(feature = "dev")]
const LONG_MSG: f32 = 2.;

pub const MESSAGES: &[Message] = &[
    Message::new("Oh no", SHORT_MSG),
    Message::new("We were counting sheep, and now we are sheep", LONG_MSG),
    Message::new_clear("Which sheep?", SHORT_MSG),
    Message::new("Yes, that is the which sheep we are", SHORT_MSG),
    Message::new("This is clear to us, as sheep", LONG_MSG),
    Message::new_clear("Press Space to express your individuality", 0.),
    Message::Pause,
    Message::new("Well done! Very individual", LONG_MSG),
    Message::new_clear("We also express our individuality", LONG_MSG),
    Message::new("It is confusing", LONG_MSG),
    Message::new("But what can you sheep", LONG_MSG),
    Message::new_clear("Press WASD to move around", LONG_MSG),
    Message::new(
        "Do not express your individuality by pressing WASD",
        LONG_MSG,
    ),
    Message::new_clear("We also like cabbage, it is our food", 0.),
    Message::Pause,
    Message::new("Well done!", SHORT_MSG),
    Message::new("We are not food", LONG_MSG),
    Message::new_clear("We wish happy sheep to we", SHORT_MSG),
];
