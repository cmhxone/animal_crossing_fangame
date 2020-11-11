use sdl2::rect::Rect;
use super::velocity::Velocity;

pub fn is_collide(src: Rect, dst: Rect) -> Option<Rect> {
    Rect::intersection(&src, dst)
}

pub fn replace_collide(collision_rect: Rect, mut player_dst_rect: Rect, player_velocity: Velocity) -> Rect {
    let replace_speed = if player_velocity.x() != 0 && player_velocity.y() != 0 {
        (
            (player_velocity.x() as f64 / 1.414213).round() as i32,
            (player_velocity.y() as f64 / 1.414213).round() as i32
        )
    } else {
        (player_velocity.x(), player_velocity.y())
    };

    if collision_rect.width() <= replace_speed.0.abs() as u32 {
        // 좌우 충돌
        if collision_rect.x() == player_dst_rect.x() {
            player_dst_rect.set_x(player_dst_rect.left() - replace_speed.0);
        } else {
            player_dst_rect.set_right(player_dst_rect.right() - replace_speed.0);
        }

        // if collision_rect.height() <= replace_speed.1.abs() as u32 {
        //     if collision_rect.y() == player_dst_rect.y() {
        //         player_dst_rect.set_y(player_dst_rect.top() - replace_speed.1);
        //     } else {
        //         player_dst_rect.set_bottom(player_dst_rect.bottom() - replace_speed.1);
        //     }
        // }
    } else if collision_rect.height() <= replace_speed.1.abs() as u32 {
        // 상하 충돌
        if collision_rect.y() == player_dst_rect.y() {
            player_dst_rect.set_y(player_dst_rect.top() - replace_speed.1);
        } else {
            player_dst_rect.set_bottom(player_dst_rect.bottom() - replace_speed.1);
        }
    }

    player_dst_rect.clone()
}