#![warn(clippy::all, clippy::pedantic)]

use iced::keyboard::key::Named;
use iced::keyboard::{on_key_press, Key, Modifiers};
use iced::widget::{column, text_input};
use iced::{Center, Element, Subscription, Theme};

fn main() -> iced::Result {
    iced::application(
        "FRO Calcualtor",
        ApplicationCalculator::update,
        ApplicationCalculator::view,
    )
    .subscription(ApplicationCalculator::user_press_enter)
    .theme(ApplicationCalculator::theme)
    .run()
}

#[derive(Default)]
struct ApplicationCalculator {
    user_input: String,
}

#[derive(Debug, Clone)]
enum AppEvent {
    UserPushSymbol(String),
    UserPressEnter,
}

impl ApplicationCalculator {
    fn update(&mut self, app_event: AppEvent) {
        match app_event {
            AppEvent::UserPushSymbol(user_input) => {
                self.user_input = user_input.clone();
                println!("{}", user_input.clone());
            }
            AppEvent::UserPressEnter => {
                self.user_input = "".to_string();
            }
        }
    }

    fn view(&self) -> Element<'_, AppEvent> {
        column![
            text_input("", &self.user_input).on_submit(AppEvent::UserPushSymbol("f".to_string())),
        ]
        .padding(20)
        .align_x(Center)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }

    fn user_press_enter(&self) -> Subscription<AppEvent> {
        on_key_press(|key: Key, _modifiers: Modifiers| match key {
            Key::Named(Named::Enter) => Some(AppEvent::UserPressEnter),
            _ => None,
        })
    }
}
