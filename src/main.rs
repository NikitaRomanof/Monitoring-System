use data::vew_data::ViewContainer;
use iced::{window, Theme};

mod data;

#[cfg(target_os = "windows")]
pub fn main() -> iced::Result {
    let mut w = window::Settings::default();
    let mut ps = window::Settings::default().platform_specific;
    ps.drag_and_drop = false;
    w.platform_specific = ps;

    iced::application("Monytoring System", ViewContainer::update, ViewContainer::view)
    .theme(|_| Theme::TokyoNightLight)
    .window(w)
        .run()
}

#[cfg(target_os = "linux")]
pub fn main() -> iced::Result {
    

    iced::application("Monytoring System", ViewContainer::update, ViewContainer::view)
    .theme(|_| Theme::TokyoNightLight)
        .run()
}
