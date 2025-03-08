use crate::workspace::album::Album;
use crate::workspace::workspace::Workspace;
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
            viewport: &'a Option<Viewport>) -> MainWindow<'a> {
        let album_images = &album.images;
        let view_mode = workspace.get_view_mode();
        let parameters = workspace.current_parameters();
        let angle_degrees = workspace.current_angle_degrees();

        let image_selection_pane: ImageSelectionPane<'a> = ImageSelectionPane::new(album_images);
        let render_pane: RenderPane<'a> = RenderPane::new(&viewport, view_mode);
        let toolbox_pane: ToolboxPane<'a> = ToolboxPane::new(parameters, angle_degrees);

        Self { image_selection_pane, render_pane, toolbox_pane }
    }

    pub fn view(&self) -> iced::Element<'a, Message> {
        iced::widget::row![
                self.view_main_area(),
                self.toolbox_pane.view().map(Message::ToolboxMessage)
            ]
            .width(iced::Fill)
            .height(iced::Fill)
            .into()
    }

    fn view_main_area(&self) -> iced::Element<'a, Message> {
        iced::widget::column![
                self.render_pane.view().map(Message::RenderMessage),
                self.image_selection_pane.view().map(Message::ImageSelectionMessage)
            ]
            .into()
    }
}