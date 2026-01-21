mod theme;
mod top_bar;

use iced::widget::{column, container};
use iced::{Element, Length, Theme};
use lucide_icons::LUCIDE_FONT_BYTES;
use top_bar::{Message as TopBarMessage, TopBar};

pub fn main() -> iced::Result {
    iced::application("Rostr", RostrApp::update, RostrApp::view)
        .theme(RostrApp::theme)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

#[derive(Default)]
struct RostrApp {
    is_dark: bool,
    search_query: String,
}

#[derive(Debug, Clone)]
enum Message {
    TopBar(TopBarMessage),
}

impl RostrApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::TopBar(top_bar_msg) => match top_bar_msg {
                TopBarMessage::SearchChanged(query) => self.search_query = query,
                _ => {}
            },
        }
    }

    fn theme(&self) -> Theme {
        if self.is_dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let top_bar = TopBar::view(
            "October 2023",
            "Monthly Attendance Overview • 42 Active Employees",
            &self.search_query,
            self.is_dark,
        )
        .map(Message::TopBar);

        container(column![top_bar])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|theme: &Theme| {
                let palette = theme.palette();
                container::Style {
                    background: Some(palette.background.into()),
                    ..Default::default()
                }
            })
            .into()
    }
}
