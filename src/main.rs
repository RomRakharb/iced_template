use iced::{
    Element, Length, Pixels, Subscription,
    keyboard::{self, key::Named},
    widget::{button, column, container, text},
};

fn main() -> iced::Result {
    iced::application("Iced Application", update, view)
        .subscription(subscription)
        .run()
}

#[derive(Default)]
struct State {
    value: i64,
}

#[derive(Clone, Debug)]
enum Message {
    Increment,
    Decrement,
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Increment => state.value += 1,
        Message::Decrement => state.value -= 1,
    }
}

fn view(state: &State) -> Element<Message> {
    container(
        column![
            button("+").on_press(Message::Increment),
            text(state.value),
            button("-").on_press(Message::Decrement),
        ]
        .spacing(Pixels(10.0)),
    )
    .center(Length::Fill)
    .into()
}

fn subscription(_state: &State) -> Subscription<Message> {
    keyboard::on_key_press(|key, _modifiers| match key {
        keyboard::Key::Named(Named::ArrowUp) => Some(Message::Increment),
        keyboard::Key::Named(Named::ArrowDown) => Some(Message::Decrement),
        _ => None,
    })
}
