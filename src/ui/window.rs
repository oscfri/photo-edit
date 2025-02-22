use crate::{album, view_mode};
use crate::workspace::Workspace;
use crate::Point;
use crate::viewport::Viewport;
use crate::view_mode::ViewMode;
use crate::album::Parameters;

use crate::ui::message::Message;
use crate::ui::panes::image_selection_pane::ImageSelectionPane;
use crate::ui::panes::render_pane::RenderPane;
use crate::ui::panes::toolbox_pane::ToolboxPane;

pub struct Window<'a> {
    image_selection_pane: ImageSelectionPane<'a>,
    render_pane: RenderPane<'a>,
    toolbox_pane: ToolboxPane<'a>,
}

impl<'a> Window<'a> {
    pub fn new(
            workspace: &'a Workspace,
            viewport: &'a Viewport,
            mouse_position: &'a Point) -> Window<'a> {
        let album_images = workspace.album_images();
        let view_mode = workspace.get_view_mode();
        let parameters = workspace.current_parameters();
        let angle_degrees = workspace.current_crop().angle_degrees;
        let image_selection_pane: ImageSelectionPane<'a> = ImageSelectionPane::new(album_images);
        let render_pane: RenderPane<'a> = RenderPane::new(viewport, mouse_position, view_mode);
        let toolbox_pane: ToolboxPane<'a> = ToolboxPane::new(parameters, angle_degrees);

        Self { image_selection_pane, render_pane, toolbox_pane }
    }

    pub fn view(&self) -> iced::Element<'a, Message> {
        iced::widget::row![
                self.view_main_area(),
                self.toolbox_pane.view()
            ]
            .width(iced::Fill)
            .height(iced::Fill)
            .into()
    }

    fn view_main_area(&self) -> iced::Element<'a, Message> {
        iced::widget::column![
                self.render_pane.view(),
                self.image_selection_pane.view()
            ]
            .into()
    }
}