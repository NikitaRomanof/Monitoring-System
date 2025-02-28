
/*
запилить панель с бесканечным циклом, где будут отображаться изменяемые значения, раз в 3 секунды

следующий шаг - вынести в отдельный файл всю логику с поведением панелей и добавить его в мод.

и в заключении когда логика будет готова - добавить картинки на кнопки и на фон

финальный этам добавить сборку во флэтпак

*/

use data::vew_data::ViewContainer;
use iced::Theme;

mod data;

pub fn main() -> iced::Result {
    iced::application("Monytoring System", ViewContainer::update, ViewContainer::view)
    .theme(|_| Theme::TokyoNightLight)
        .run()
}
