#[macro_export]
macro_rules! define_icons {
    ($($(#[$attr:meta])* $name:ident => $icon_name:expr),* $(,)?) => {
        $(
            $(#[$attr])*
            pub fn $name() -> iced::widget::Text<'static> {
                icon($icon_name)
            }
        )*
    };
}
