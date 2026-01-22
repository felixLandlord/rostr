mod theme;
mod top_bar;
mod action_bar;
mod attendance_table;

use attendance_table::{AttendanceTable, Message as AttendanceTableMessage};
use action_bar::{ActionBar, Message as ActionBarMessage};
use chrono::{Datelike, Local, NaiveDate};
use iced::task::Task;
use iced::widget::{column, container};
use iced::{Element, Length, Padding, Theme};
use lucide_icons::LUCIDE_FONT_BYTES;
use top_bar::{Message as TopBarMessage, TopBar};

pub fn main() -> iced::Result {
    iced::application(RostrApp::default, RostrApp::update, RostrApp::view)
        .title("rostr")
        .theme(RostrApp::theme)
        .font(LUCIDE_FONT_BYTES)
        .run()
}

struct RostrApp {
    is_dark: bool,
    search_query: String,
    current_date: NaiveDate,
    attendance_table: AttendanceTable,
}

impl Default for RostrApp {
    fn default() -> Self {
        let now = Local::now();
        Self {
            is_dark: false,
            search_query: String::new(),
            current_date: NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap(),
            attendance_table: AttendanceTable::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    TopBar(TopBarMessage),
    ActionBar(ActionBarMessage),
    AttendanceTable(AttendanceTableMessage),
}

impl RostrApp {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ActionBar(_) => {}
            Message::AttendanceTable(msg) => {
                self.attendance_table.update(msg);
            }
            Message::TopBar(top_bar_msg) => match top_bar_msg {
                TopBarMessage::SearchChanged(query) => self.search_query = query,
                TopBarMessage::ToggleTheme => self.is_dark = !self.is_dark,
                TopBarMessage::PreviousDate => {
                    self.current_date = if self.current_date.month() == 1 {
                        NaiveDate::from_ymd_opt(self.current_date.year() - 1, 12, 1).unwrap()
                    } else {
                        NaiveDate::from_ymd_opt(
                            self.current_date.year(),
                            self.current_date.month() - 1,
                            1,
                        )
                        .unwrap()
                    };
                }
                TopBarMessage::NextDate => {
                    self.current_date = if self.current_date.month() == 12 {
                        NaiveDate::from_ymd_opt(self.current_date.year() + 1, 1, 1).unwrap()
                    } else {
                        NaiveDate::from_ymd_opt(
                            self.current_date.year(),
                            self.current_date.month() + 1,
                            1,
                        )
                        .unwrap()
                    };
                }
                _ => {}
            },
        }
        Task::none()
    }

    fn theme(&self) -> Theme {
        if self.is_dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let date_str = self.current_date.format("%B %Y").to_string();
        let top_bar = TopBar::view(
            date_str,
            "Monthly Attendance Overview • 42 Active Employees".to_string(),
            &self.search_query,
            self.is_dark,
        )
        .map(Message::TopBar);

        let action_bar = ActionBar::view(self.is_dark).map(Message::ActionBar);
        let attendance_table = self.attendance_table.view(self.is_dark).map(Message::AttendanceTable);

        container(
            column![
                top_bar,
                container(action_bar).padding(Padding::from([24, 32])),
                container(attendance_table).padding(Padding {
                    top: 0.0,
                    right: 32.0,
                    bottom: 32.0,
                    left: 32.0,
                })
            ]
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |theme: &Theme| {
                let palette = theme.palette();
                container::Style {
                    background: Some(palette.background.into()),
                    ..Default::default()
                }
            })
            .into()
    }
}
