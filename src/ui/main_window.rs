use crate::workspace::album::{self, Album};
use crate::workspace::workspace::Workspace;
use crate::Point;
use crate::viewport::Viewport;

use crate::ui::message::Message;
use crate::ui::panes::image_selection_pane::ImageSelectionPane;
use crate::ui::panes::render_pane::RenderPane;
use crate::ui::panes::toolbox_pane::ToolboxPane;

pub struct MainWindow<'a> {
    image_selection_pane: ImageSelectionPane<'a>,
    render_pane: RenderPane<'a>,
    toolbox_pane: ToolboxPane<'a>
}

impl<'a> MainWindow<'a> {
    pub fn new(
            album: &'a Album,
            workspace: &'a Workspace,
            viewport: &'a Viewport,
            mouse_position: &'a Point) -> MainWindow<'a> {
        let album_images = &album.images;
        let view_mode = workspace.get_view_mode();
        let parameters = workspace.current_parameters();
        let angle_degrees = workspace.current_crop().angle_degrees;

        let image_selection_pane: ImageSelectionPane<'a> = ImageSelectionPane::new(album_images);
        let render_pane: RenderPane<'a> = RenderPane::new(&viewport, mouse_position, view_mode);
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