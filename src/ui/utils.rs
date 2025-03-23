pub fn icon_button<'a, T>(icon: iced_fonts::Nerd) -> iced::widget::Button<'a, T> {
    let icon_string = iced_fonts::nerd::icon_to_char(icon);
    let content = iced::widget::text(icon_string)
        .font(iced_fonts::NERD_FONT);
    iced::widget::button(content)
        .style(iced::widget::button::text)
}

pub fn slider_scaled<'a, Message, Theme>(
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    scale: f32,
    on_change: impl Fn(f32) -> Message + 'a,
) -> iced::widget::Slider<'a, f32, Message, Theme>
where
    Message: Clone,
    Theme: iced::widget::slider::Catalog + 'a,
{
    iced::widget::slider(range, value * scale, move |value| on_change(value / scale))
}