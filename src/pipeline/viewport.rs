use crate::pipeline::primitive;
use crate::types::RawImage;

use iced::mouse;
use iced::widget::shader;

pub struct Viewport {
    // TODO: Probably should put nice things here
    // - Parameters
    // TODO: These shouldn't be pub
    pub image: RawImage,
    pub image_index: usize,
}

impl<Message> shader::Program<Message> for Viewport {
    type State = ();
    type Primitive = primitive::Primitive;

    fn draw(&self, _state: &Self::State, _cursor: mouse::Cursor, _bounds: iced::Rectangle) -> Self::Primitive {
        primitive::Primitive::new(self.image.clone(), self.image_index)
    }
}