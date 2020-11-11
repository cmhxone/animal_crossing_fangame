extern crate sdl2; 

mod physics;
mod camera;
mod entity;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::rect::Rect;
use std::collections::HashSet;
use physics::velocity::Velocity;
use camera::camera::{ aligned_rect };
use physics::collision::{ is_collide, replace_collide };
use rand::Rng;

// FPS 값
const FRAME_PER_SECOND: u32 = 60;
// 화면 크기
const SCREEN_SIZE: (u32, u32) = (1024, 768);
// 스프라이트의 크기 상수
const SPRITE_TILE_SIZE: (u32, u32) = (34, 52);
const OBJ_SPRITE_TILE_SIZE: (u32, u32) = (16, 16);
// 플레이어 걷기 애니메이션의 최대 스프라이트 갯수
const PLAYER_WALKING_SPRITES: u32 = 4;
// 플레이어 걷기 속도
const PLAYER_SPEED: u32 = 4;
// 맵 타일 크기
const TILE_SIZE: (u32, u32) = (64, 64);
// 맵 타일 가로 최대 인덱스
const TILE_HINDEX_MAX: u32 = 10;
const OBJ_TILE_HINDEX_MAX: u32 = 4;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let _sdl_image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
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
    let mut main_cam = Rect::new(0, 0, SCREEN_SIZE.0, SCREEN_SIZE.1);
    
    // 맵 파일 읽어오기
    let map = String::from_utf8_lossy(include_bytes!("../asset/resource/map/town.map"));
    let map_line: Vec<&str> = map.lines().collect();
    let mut map_size: (u32, u32) = (0, 0);
    let mut map_tiles: Vec<(Rect, &str)> = Vec::new();

    // 맵 타일 토큰화
    for line in map_line {
        // 토큰을 다시 타일로 토큰화
        let field: Vec<&str> = line.split_whitespace().collect();
        map_size.1 = 0;
        for tile in field {
            // println!("Tile{}: {}", map_size.1, tile);
            map_tiles.push((Rect::new(map_size.1 as i32, map_size.0 as i32, TILE_SIZE.0, TILE_SIZE.1), tile));
            map_size.1 += TILE_SIZE.1;
        }
        map_size.0 += TILE_SIZE.0;
    }

    // 타일 스프라이트
    let tile_sprite = include_bytes!("../asset/resource/sprite/tiles_edit.png");
    let tile_texture = texture_creator.load_texture_bytes(tile_sprite).unwrap();

    // 플레이어 스프라이트
    let player_sprite = include_bytes!("../asset/resource/sprite/player.png");
    let player_texture = texture_creator.load_texture_bytes(player_sprite).unwrap();
    let mut player_src_rect = Rect::new(0, 0, SPRITE_TILE_SIZE.0, SPRITE_TILE_SIZE.1);
    let mut player_dst_rect = Rect::new(0, 0, TILE_SIZE.0, TILE_SIZE.1);
    let mut player_velocity = Velocity::new(0, 0, 0);

    // let mut player = Player::new(
    //     Rect::new(0, 0, SPRITE_TILE_SIZE.0, SPRITE_TILE_SIZE.1),
    //     Rect::new(0, 0, SPRITE_TILE_SIZE.0, SPRITE_TILE_SIZE.1),
    //     Velocity::new(0, 0, 0),
    // );

    // 배경화면 스프라이트
    let background_sprite = include_bytes!("../asset/resource/sprite/gray_background.png");
    let background_texture = texture_creator.load_texture_bytes(background_sprite).unwrap();
    let background_src_rect = Rect::new(0, 0, background_texture.query().width, background_texture.query().height);
    let background_dst_rect = Rect::new(0, 0, map_size.1, map_size.0);

    // 둥근모꼴 폰트
    let dung_geun_mo = include_bytes!("../asset/resource/font/DungGeunMo.ttf");
    let dgm_font = ttf_context.load_font_from_rwops(sdl2::rwops::RWops::from_bytes(dung_geun_mo).unwrap(), 128).unwrap();
    let font_surface = dgm_font.render("플레이어").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
    let font_texture = texture_creator.create_texture_from_surface(&font_surface).unwrap();
 
    // 오브젝트 생성
    let object_sprite = include_bytes!("../asset/resource/sprite/objects.png");
    let object_texture = texture_creator.load_texture_bytes(object_sprite).unwrap();
    let object_src_rect = Rect::new(0, 0, OBJ_SPRITE_TILE_SIZE.0, OBJ_SPRITE_TILE_SIZE.1);
    let object_dst_rect = Rect::new(640, 640, TILE_SIZE.0, TILE_SIZE.1);

    // 오브젝트 벡터 생성
    let mut objects: Vec<(Rect, u32)> = Vec::new();
    for number in 0 .. 20 {
        let mut rng = rand::thread_rng();
        objects.push(
            (
                Rect::new(
                    // number * TILE_SIZE.0 as i32,
                    // 704,
                    rng.gen_range(0, map_size.0 as i32 - TILE_SIZE.0 as i32),
                    rng.gen_range(0, map_size.1 as i32 - TILE_SIZE.1 as i32),
                    TILE_SIZE.0,
                    TILE_SIZE.1,
                ),
                number as u32
            )
        );
    }

    // 화면 초기화
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
                    // player.set_velocity_y(PLAYER_SPEED as i32);
                } else if key == Keycode::Left {
                    player_velocity.set_x(-(PLAYER_SPEED as i32));
                    // player.set_velocity_x(-(PLAYER_SPEED as i32));
                } else if key == Keycode::Right {
                    player_velocity.set_x(PLAYER_SPEED as i32);
                    // player.set_velocity_x(PLAYER_SPEED as i32);
                } else if key == Keycode::Up {
                    player_velocity.set_y(-(PLAYER_SPEED as i32));
                    // player.set_velocity_y(-(PLAYER_SPEED as i32));
                }
            }

            // 입력 해제 된 키 (KeyUps)
            for key in old_keys {
                if key == Keycode::Down || key == Keycode::Up {
                    player_velocity.set_y(0);
                    // player.set_velocity_y(0);
                } else if key == Keycode::Left || key == Keycode::Right {
                    player_velocity.set_x(0);
                    // player.set_velocity_x(0);
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
            // player_dst_rect.set_x(player_dst_rect.x() + (player_velocity.x() as f64 / 1.414213).round() as i32);
            // player_dst_rect.set_y(player_dst_rect.y() + (player_velocity.y() as f64 / 1.414213).round() as i32);
        } else {
            player_dst_rect.set_x(player_dst_rect.x() + player_velocity.x());
            player_dst_rect.set_y(player_dst_rect.y() + player_velocity.y());
        }

        // 플레이어가 배경 밖으로 나가려고 할 때, 플레이어를 재위치 시킨다
        if player_dst_rect.x() < 0 {
            player_dst_rect.set_x(0);
        }
        if player_dst_rect.y() < 0 {
            player_dst_rect.set_y(0);
        }
        if player_dst_rect.right() > background_dst_rect.right() {
            player_dst_rect.set_right(background_dst_rect.right());
        }
        if player_dst_rect.bottom() > background_dst_rect.bottom() {
            player_dst_rect.set_bottom(background_dst_rect.bottom());
        }

        // 카메라의 중앙을 플레이어의 중앙에 맞춘다
        main_cam.center_on(player_dst_rect.center());

        // 카메라의 위치를 배경 내로 조정한다
        if main_cam.x() < 0 {
            main_cam.set_x(background_dst_rect.left());
        }
        if main_cam.y() < 0 {
            main_cam.set_y(background_dst_rect.top());
        }
        if main_cam.right() > background_dst_rect.right() {
            main_cam.set_right(background_dst_rect.right());
        }
        if main_cam.bottom() > background_dst_rect.bottom() {
            main_cam.set_bottom(background_dst_rect.bottom());
        }

        // 물체와 충돌 시 위치 재조정
        match is_collide(player_dst_rect, object_dst_rect) {
            Some(collision_rect) => {
                player_dst_rect = replace_collide(collision_rect, player_dst_rect, player_velocity);
            },
            None => {},
        }

        /* 그리기 */
        // 배경화면 스프라이트 그리기
        canvas.copy_ex(&background_texture, background_src_rect, aligned_rect(main_cam, background_dst_rect), 0.0, None, false, false).unwrap();
        
        // 타일 그리기
        for tile in &map_tiles {
            let tile_index = tile.1.parse::<i32>().unwrap();

            // 타일에 맞춰 드로잉 한다
            canvas.copy_ex(
                &tile_texture,
                Rect::new(
                    tile_index % TILE_HINDEX_MAX as i32 *TILE_SIZE.0 as i32,
                    tile_index / TILE_HINDEX_MAX as i32 *TILE_SIZE.1 as i32,
                    TILE_SIZE.0, TILE_SIZE.1),
                aligned_rect(main_cam, tile.0),
                0.0, None, false, false
            ).unwrap();
            
            match tile_index {
                8|9|18|19|28|29|38|39|48|49 => {
                    match is_collide(player_dst_rect, tile.0) {
                        Some(collision_rect) => {
                            player_dst_rect = replace_collide(collision_rect, player_dst_rect, player_velocity);
                        },
                        None => {},
                    }
                },
                _ => {},
            }
        }

        // 오브젝트 벡터 그리기, 충돌 판정
        for object in &objects {
            canvas.copy_ex(
                &object_texture,
                Rect::new(
                    object.1 as i32 % OBJ_TILE_HINDEX_MAX as i32 * OBJ_SPRITE_TILE_SIZE.0 as i32,
                    object.1 as i32 / OBJ_TILE_HINDEX_MAX as i32 * OBJ_SPRITE_TILE_SIZE.1 as i32,
                    OBJ_SPRITE_TILE_SIZE.0, OBJ_SPRITE_TILE_SIZE.1
                ),
                aligned_rect(main_cam, object.0),
                0.0, None, false, false
            ).unwrap();
            
            match is_collide(player_dst_rect, object.0) {
                Some(collision_rect) => {
                    player_dst_rect = replace_collide(collision_rect, player_dst_rect, player_velocity);
                },
                None => {},
            }
        }

        // 오브젝트 스프라이트 그리기
        canvas.copy_ex(&object_texture, object_src_rect, aligned_rect(main_cam, object_dst_rect), 0.0, None, false, false).unwrap();

        // 플레이어 스프라이트 그리기
        canvas.copy_ex(&player_texture, player_src_rect, aligned_rect(main_cam, player_dst_rect), 0.0, None, false, false).unwrap();

        // 글자 그리기
        canvas.copy_ex(&font_texture, None, aligned_rect(main_cam, Rect::new(player_dst_rect.x(), player_dst_rect.y() - 16, player_dst_rect.width(), player_dst_rect.height() / 4)), 0.0, None, false, false). unwrap();

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