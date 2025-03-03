use super::{message::Message, panes::welcome_pane::WelcomePane};

pub struct WelcomeWindow {
    welcome_pane: WelcomePane
}

impl<'a> WelcomeWindow {
    pub fn new() -> WelcomeWindow {
        let welcome_pane: WelcomePane = WelcomePane::new();
        Self { welcome_pane }
    }

    pub fn view(&self) -> iced::Element<'a, Message> {
        self.welcome_pane.view()
            .map(Message::WelcomeMessage)
            .into()
    }
}