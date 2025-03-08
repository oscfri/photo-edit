pub fn icon_button<'a, T>(icon: iced_fonts::Bootstrap) -> iced::widget::Button<'a, T> {
    let icon_string = iced_fonts::bootstrap::icon_to_string(icon);
    let content = iced::widget::text(icon_string)
        .font(iced_fonts::BOOTSTRAP_FONT);
    iced::widget::button(content)
        .style(iced::widget::button::text)
}