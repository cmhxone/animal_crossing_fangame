extern crate sdl2; 

mod physics;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::rect::Rect;
use std::collections::HashSet;
use physics::velocity::Velocity;

// FPS 값
const FRAME_PER_SECOND: u32 = 60;
// 화면 크기
const SCREEN_SIZE: (u32, u32) = (1024, 768);
// 스프라이트의 크기 상수
const SPRITE_TILE_SIZE: (u32, u32) = (64, 64);
// 플레이어 걷기 애니메이션의 최대 스프라이트 갯수
const PLAYER_WALKING_SPRITES: u32 = 4;
// 플레이어 걷기 속도
const PLAYER_SPEED: u32 = 4;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let _sdl_image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("똥물의 숲", SCREEN_SIZE.0, SCREEN_SIZE.1)
        .position_centered()
        .build()
        .unwrap();
 
    // 프레임 고정을 위한 타이머 선언
    let mut timer = sdl_context.timer().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // 카메라 논리 객체
    let camera = Rect::new(0, 0, SCREEN_SIZE.0, SCREEN_SIZE.1);
    
    // 플레이어 스프라이트
    let player_sprite = include_bytes!("../asset/resource/sprite/player.png");
    let player_texture = texture_creator.load_texture_bytes(player_sprite).unwrap();
    let mut player_src_rect = Rect::new(0, 0, SPRITE_TILE_SIZE.0, SPRITE_TILE_SIZE.1);
    let mut player_dst_rect = Rect::new(0, 0, SPRITE_TILE_SIZE.0 * 2, SPRITE_TILE_SIZE.1 * 2);
    let mut player_velocity = Velocity::new(0, 0, 0);

    let background_sprite = include_bytes!("../asset/resource/sprite/background.bmp");
    let background_texture = texture_creator.load_texture_bytes(background_sprite).unwrap();
    let mut background_dst_rect = Rect::new(0, 0, SCREEN_SIZE.0 * 2, SCREEN_SIZE.1 * 2);
 
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();
    // 키보드 입력을 처리하기 위한 해쉬셋
    let mut prev_keys = HashSet::new();

    // 플레이어 위치 지정
    player_dst_rect.center_on(sdl2::rect::Point::new(SCREEN_SIZE.0 as i32 / 2, SCREEN_SIZE.1 as i32 / 2));

    // 프레임 계산을 위한 변수
    let mut frame: u32 = 0;
    'running: loop {
        // 프레임 증가
        frame += 1;
        canvas.clear();

        // 게임루프 시작 틱(ms)
        let start_tick = timer.ticks();

        for event in event_pump.poll_iter() {
            match event {
                // Event::Quit {..} |
                // Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        /* 키보드 입력 */
        // 키보드 입력을 해쉬셋에 저장한다
        let keys: HashSet<_> = event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        // 새로 입력된 키 (KeyDown)
        let new_keys = &keys - &prev_keys;
        // 입력 해제 된 키 (KeyUp)
        let old_keys = &prev_keys - &keys;

        // 키보드 핸들링
        if !new_keys.is_empty() || !old_keys.is_empty() {
            // 입력 받은 키 (KeyDowns)
            for key in new_keys {
                if key == Keycode::Down {
                    player_velocity.set_y(PLAYER_SPEED as i32);
                } else if key == Keycode::Left {
                    player_velocity.set_x(-(PLAYER_SPEED as i32));
                } else if key == Keycode::Right {
                    player_velocity.set_x(PLAYER_SPEED as i32);
                } else if key == Keycode::Up {
                    player_velocity.set_y(-(PLAYER_SPEED as i32));
                }
            }

            // 입력 해제 된 키 (KeyUps)
            for key in old_keys {
                if key == Keycode::Down || key == Keycode::Up {
                    player_velocity.set_y(0);
                } else if key == Keycode::Left || key == Keycode::Right {
                    player_velocity.set_x(0);
                }
            }
        }
        // 키를 갱신한다
        prev_keys = keys;

        /* 업데이트 */
        // 플레이어의 벨로시티 값으로 플레이어 스프라이트를 변환한다
        if player_velocity.x() > 0 {
            player_src_rect.set_y(2 * SPRITE_TILE_SIZE.1 as i32);
            player_src_rect.set_x( SPRITE_TILE_SIZE.0 as i32 * (frame as i32 / 8 % PLAYER_WALKING_SPRITES as i32) );
        } else if player_velocity.x() < 0 {
            player_src_rect.set_y(1 * SPRITE_TILE_SIZE.1 as i32);
            player_src_rect.set_x( SPRITE_TILE_SIZE.0 as i32 * (frame as i32 / 8 % PLAYER_WALKING_SPRITES as i32) );
        } else if player_velocity.y() > 0 {
            player_src_rect.set_y(0 * SPRITE_TILE_SIZE.1 as i32);
            player_src_rect.set_x( SPRITE_TILE_SIZE.0 as i32 * (frame as i32 / 8 % PLAYER_WALKING_SPRITES as i32) );
        } else if player_velocity.y() < 0 {
            player_src_rect.set_y(3 * SPRITE_TILE_SIZE.1 as i32);
            player_src_rect.set_x( SPRITE_TILE_SIZE.0 as i32 * (frame as i32 / 8 % PLAYER_WALKING_SPRITES as i32) );
        } else if player_velocity.x() == 0 && player_velocity.y() == 0 {
            player_src_rect.set_x(0 * SPRITE_TILE_SIZE.0 as i32);
        }

        // 플레이어의 벨로시티 값으로 플레이어를 이동한다
        if player_velocity.x() != 0 && player_velocity.y() != 0 {
            if background_dst_rect.x() <= 0 && background_dst_rect.y() <= 0 {
                background_dst_rect.set_x(background_dst_rect.x() - (player_velocity.x() as f64 / 1.414213).round() as i32);
                background_dst_rect.set_y(background_dst_rect.y() - (player_velocity.y() as f64 / 1.414213).round() as i32);
                player_dst_rect.set_x(player_dst_rect.x() + (player_velocity.x() as f64 / 1.414213).round() as i32);
                player_dst_rect.set_y(player_dst_rect.y() + (player_velocity.y() as f64 / 1.414213).round() as i32);
            }
        } else {
            if background_dst_rect.x() <= 0 && background_dst_rect.y() <= 0 {
                background_dst_rect.set_x(background_dst_rect.x() - player_velocity.x());
                background_dst_rect.set_y(background_dst_rect.y() - player_velocity.y());
                player_dst_rect.set_x(player_dst_rect.x() + player_velocity.x());
                player_dst_rect.set_y(player_dst_rect.y() + player_velocity.y());
            }
        }

        println!("{:?}", background_dst_rect);

        /* 그리기 */
        // 플레이어 스프라이트 그리기
        canvas.copy_ex(&background_texture, None, background_dst_rect, 0.0, None, false, false).unwrap();
        canvas.copy_ex(&player_texture, player_src_rect, player_dst_rect, 0.0, None, false, false).unwrap();

        // 현재 캔버스를 윈도우에 그린다
        canvas.present();

        // 게임루프 현재 틱(ms) - 시작 틱(ms)
        let delay_tick = timer.ticks() - start_tick;

        // 걸린 시간이 1000(ms)/FPS보다 적은 경우, 남은 시간만큼 대기
        if delay_tick <= 1_000 / FRAME_PER_SECOND {
            timer.delay( (1_000 / FRAME_PER_SECOND) - delay_tick);
        }

        // 타이머가 1,000,000,000 이 넘어가는 경우 리셋한다
        if timer.ticks() >= 1_000_000_000 {
            timer = sdl_context.timer().unwrap();
        }

        // 1억 프레임을 넘어가는 경우 프레임 초기화
        if frame >= 1_000_000_000 {
            frame = 0;
        }
    }
}