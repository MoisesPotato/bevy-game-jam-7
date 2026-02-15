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

pub const MESSAGES: &[Message] = &[
    Message::new("Oh no", 2.),
    Message::new("We were counting sheep, and now we are sheep", 3.),
    Message::new_clear("Which sheep?", 2.),
    Message::new("Yes, that is the which sheep we are", 2.),
    Message::new("This is clear to us, as sheep", 3.),
    Message::new_clear("Press Space to express your individuality", 3.),
    Message::Pause,
    Message::new("Well done! Very individual", 3.),
    Message::new_clear("Press WASD to move around", 3.),
    Message::new("Do not express your individuality by pressing WASD", 2.),
    Message::new("We also express our individuality", 3.),
    Message::new_clear("We also like cabbage, it is our food", 3.),
    Message::Pause,
    Message::new("We are not food", 2.),
    Message::new("We wish happy sheep to we", 2.),
];
