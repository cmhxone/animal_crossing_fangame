use sdl2::rect::Rect;

pub fn aligned_rect(camera: Rect, object: Rect) -> Rect {
    Rect::new(object.x() - camera.x(), object.y() - camera.y(), object.width(), object.height())
}