use sdl2::rect::Rect;

pub fn is_collide(src: Rect, dst: Rect) -> Option<Rect> {
    Rect::intersection(&src, dst)
}