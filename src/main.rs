use data::vew_data::ViewContainer;
use iced::Theme;

mod data;

pub fn main() -> iced::Result {
    iced::application("Monytoring System", ViewContainer::update, ViewContainer::view)
    .theme(|_| Theme::TokyoNightLight)
        .run()
}
