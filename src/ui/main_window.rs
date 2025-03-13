use crate::workspace::album::Album;
use crate::workspace::image_manager::ImageManager;
use crate::workspace::workspace::Workspace;
use crate::viewport::Viewport;

use super::message::Message;
use super::panes::bottom_pane::BottomPane;
use super::panes::image_selection_pane::ImageSelectionPane;
use super::panes::render_pane::RenderPane;
use super::panes::toolbox_pane::ToolboxPane;
use super::panes::top_pane::TopPane;

pub struct MainWindow<'a> {
    bottom_pane: BottomPane,
    image_selection_pane: ImageSelectionPane<'a>,
    render_pane: RenderPane<'a>,
    toolbox_pane: ToolboxPane,
    top_pane: TopPane
}

impl<'a> MainWindow<'a> {
    pub fn new(
            image_manager: &'a ImageManager,
            album: &'a Album,
            workspace: &'a Workspace,
            viewport: &'a Option<Viewport>) -> MainWindow<'a> {
        let photo_id = album.get_photo_id();
        let album_images = &album.get_images();
        let image_index = album.get_image_index();
        let parameters = workspace.current_parameters();
        let angle_degrees = workspace.current_angle_degrees();
        let mask_index = workspace.get_mask_index();
        let toolbox_enabled = viewport.is_some();
        let parameters_visible = workspace.get_parameters_visible();
        let is_favorite = parameters.is_favorite;
        let is_filter_active = image_manager.get_is_filter_active();

        let bottom_pane: BottomPane = BottomPane::new(photo_id, parameters_visible, is_favorite);
        let image_selection_pane: ImageSelectionPane<'a> = ImageSelectionPane::new(album_images, image_index);
        let render_pane: RenderPane<'a> = RenderPane::new(&viewport);
        let toolbox_pane: ToolboxPane = ToolboxPane::new(parameters, angle_degrees, mask_index, toolbox_enabled);
        let top_pane: TopPane = TopPane::new(is_filter_active);

        Self {
            bottom_pane,
            image_selection_pane,
            render_pane,
            toolbox_pane,
            top_pane
        }
    }

    pub fn view(&self) -> iced::Element<'a, Message> {
        let toolbox_pane = iced::widget::container(self.toolbox_pane.view().map(Message::ToolboxMessage))
            .width(300);
        iced::widget::row![
                self.view_main_area(),
                toolbox_pane
            ]
            .width(iced::Fill)
            .height(iced::Fill)
            .into()
    }

    fn view_main_area(&self) -> iced::Element<'a, Message> {
        iced::widget::column![
                self.top_pane.view().map(Message::TopPaneMessage),
                self.render_pane.view().map(Message::RenderMessage),
                self.bottom_pane.view().map(Message::BottomPaneMessage),
                self.image_selection_pane.view().map(Message::ImageSelectionMessage),
            ]
            .into()
    }
}