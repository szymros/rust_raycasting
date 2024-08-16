use ::core::f32;

use glam::Vec2;
use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 400;
const SCREEN_HEIGHT: i32 = 300;

const GRID_ROWS: i32 = 8;
const GRID_COLS: i32 = 8;

#[rustfmt::skip]
const GRID: [[i32;8];8] = [
    [1,1,1,1,1,1,1,1],
    [1,0,0,0,0,0,0,1],
    [1,1,1,1,0,0,0,1],
    [1,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,1],
    [1,0,0,1,1,0,0,1],
    [1,0,0,0,0,0,0,1],
    [1,1,1,1,1,1,1,1],
];

const GRID_STRIDE_ROW: i32 = SCREEN_WIDTH / GRID_ROWS;
const GRID_STRIDE_COLS: i32 = SCREEN_HEIGHT / GRID_COLS;

fn get_rotation_matrix(rotation_deg: f32) -> glam::Mat2 {
    let rotation_radians = rotation_deg * f32::consts::PI / 180.0;
    return glam::Mat2::from_cols_array(&[
        f32::cos(rotation_radians),
        -f32::sin(rotation_radians),
        f32::sin(rotation_radians),
        f32::cos(rotation_radians),
    ])
    .into();
}

struct Player {
    pub pos: glam::Vec2,
    pub dir: glam::Vec2,
    pub plane: glam::Vec2,
    pub speed: f32,
    pub rotation_speed: f32,
}

impl Player {
    fn new(pos: glam::Vec2, dir: glam::Vec2, speed: f32, rotation_speed: f32) -> Self {
        let plane: glam::Vec2 = [0.0, 0.66].into();

        Player {
            pos,
            dir,
            plane,
            speed,
            rotation_speed,
        }
    }

    fn handle_input(&mut self, raylib_handle: &RaylibHandle, delta_time: f32) {
        let mut velocity = Vec2::ZERO;
        let mut angular_velocity = 0.0;
        let mouse_delta = raylib_handle.get_mouse_delta();
        let mouse_pos = raylib_handle.get_mouse_position();
        if mouse_delta.x > 0.0 || mouse_pos.x > SCREEN_WIDTH as f32 - 10.0 {
            angular_velocity -= self.rotation_speed * f32::consts::PI;
        }
        if mouse_delta.x < 0.0 || mouse_pos.x < 10.0 {
            angular_velocity += self.rotation_speed * f32::consts::PI;
        }
        if raylib_handle.is_key_down(raylib::consts::KeyboardKey::KEY_W) {
            velocity += self.dir * self.speed;
        }
        if raylib_handle.is_key_down(raylib::consts::KeyboardKey::KEY_S) {
            velocity -= self.dir * self.speed;
        }
        if raylib_handle.is_key_down(raylib::consts::KeyboardKey::KEY_D) {
            let rotation_matrix = get_rotation_matrix(-90.0);
            velocity += rotation_matrix * self.dir * self.speed;
        }
        if raylib_handle.is_key_down(raylib::consts::KeyboardKey::KEY_A) {
            let rotation_matrix = get_rotation_matrix(90.0);
            velocity += rotation_matrix * self.dir * self.speed;
        }
        self.pos += velocity * delta_time;
        self.dir = (get_rotation_matrix(angular_velocity * delta_time) * self.dir).normalize();
        self.plane = get_rotation_matrix(angular_velocity * delta_time) * self.plane;
    }
}
fn draw_grid(draw_handle: &mut RaylibDrawHandle, player: &Player) {
    for x in 0..GRID_COLS {
        for y in 0..GRID_ROWS {
            if GRID[y as usize][x as usize] == 1 {
                draw_handle.draw_rectangle(
                    x * GRID_STRIDE_ROW,
                    y * GRID_STRIDE_COLS,
                    GRID_STRIDE_ROW,
                    GRID_STRIDE_COLS,
                    Color::BLACK,
                );
            } else {
                draw_handle.draw_rectangle_lines(
                    x * GRID_STRIDE_ROW,
                    y * GRID_STRIDE_COLS,
                    GRID_STRIDE_ROW,
                    GRID_STRIDE_COLS,
                    Color::BLACK,
                )
            }
            draw_handle.draw_circle(
                (player.pos[0] * GRID_STRIDE_ROW as f32) as i32,
                (player.pos[1] * GRID_STRIDE_COLS as f32) as i32,
                8.0,
                Color::RED,
            );
            draw_handle.draw_line(
                (player.pos[0] * GRID_STRIDE_ROW as f32) as i32,
                (player.pos[1] * GRID_STRIDE_COLS as f32) as i32,
                (player.pos[0] * GRID_STRIDE_ROW as f32) as i32
                    + (player.dir[0] * GRID_STRIDE_ROW as f32) as i32,
                (player.pos[1] * GRID_STRIDE_COLS as f32) as i32
                    + (player.dir[1] * GRID_STRIDE_COLS as f32) as i32,
                Color::GREEN,
            );
            draw_handle.draw_line(
                (player.pos.x * GRID_STRIDE_ROW as f32) as i32
                    + ((player.dir.x - player.plane.x) * GRID_STRIDE_ROW as f32) as i32,
                (player.pos.y * GRID_STRIDE_COLS as f32) as i32
                    + ((player.dir.y - player.plane.y) * GRID_STRIDE_COLS as f32) as i32,
                (player.pos.x * GRID_STRIDE_ROW as f32) as i32
                    + ((player.dir.x + player.plane.x) * GRID_STRIDE_ROW as f32) as i32,
                (player.pos.y * GRID_STRIDE_COLS as f32) as i32
                    + ((player.dir.y + player.plane.y) * GRID_STRIDE_COLS as f32) as i32,
                Color::BLUE,
            );
        }
    }
}

fn draw(
    draw_handle: &mut RaylibDrawHandle,
    player: &Player,
    show_map: bool,
    wall_texture: &mut Image,
    floor_texture: &mut Image,
) {
    let mut pixel_buffer =
        vec![vec![Color::new(0, 0, 0, 0); SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize];
    draw_handle.clear_background(Color::WHITE);
    if show_map {
        draw_grid(draw_handle, player);
    } else {
        cast_floors(player, floor_texture, &mut pixel_buffer);
        cast_rays(player, wall_texture, &mut pixel_buffer);
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                draw_handle.draw_pixel(x, y, pixel_buffer[x as usize][y as usize])
            }
        }
    }
}

fn cast_rays(player: &Player, texture: &mut Image, pixel_buffer: &mut Vec<Vec<Color>>) {
    for x in 0..SCREEN_WIDTH {
        let camera_space_x = 2.00 * x as f32 / SCREEN_WIDTH as f32 - 1.0;
        let ray_dir = player.dir + player.plane * camera_space_x;
        let delta_x = (1.0 / ray_dir.x).abs();
        let delta_y = (1.0 / ray_dir.y).abs();
        let mut map_pos = player.pos.floor();
        let mut side_lenght_y: f32;
        let mut side_lenght_x: f32;
        let mut side: i32 = 0;
        let step_x: f32;
        let step_y: f32;
        if ray_dir.x < 0.0 {
            step_x = -1.0;
            side_lenght_x = (player.pos.x - map_pos.x) * delta_x;
        } else {
            step_x = 1.0;
            side_lenght_x = (map_pos.x + 1.0 - player.pos.x) * delta_x;
        }
        if ray_dir.y < 0.0 {
            step_y = -1.0;
            side_lenght_y = (player.pos.y - map_pos.y) * delta_y;
        } else {
            step_y = 1.0;
            side_lenght_y = (map_pos.y + 1.0 - player.pos.y) * delta_y;
        }
        let mut hit = false;
        while !hit {
            if side_lenght_x < side_lenght_y {
                side_lenght_x += delta_x;
                map_pos.x += step_x;
                side = 0;
            } else {
                side_lenght_y += delta_y;
                map_pos.y += step_y;
                side = 1;
            }
            if map_pos.x < GRID_COLS as f32
                && map_pos.y < GRID_ROWS as f32
                && GRID[map_pos.y as usize][map_pos.x as usize] == 1
            {
                hit = true;
            }
        }
        let ray_lenght: f32;
        let mut tex_cord_x: f32;
        if side == 0 {
            ray_lenght = side_lenght_x - delta_x;
            tex_cord_x = player.pos.y + ray_lenght * ray_dir.y;
        } else {
            ray_lenght = side_lenght_y - delta_y;
            tex_cord_x = player.pos.x + ray_lenght * ray_dir.x;
        }
        let line_height = SCREEN_HEIGHT as f32 / ray_lenght;
        tex_cord_x = tex_cord_x.fract() * texture.width as f32;
        let mut line_start: i32 = -line_height as i32 / 2 + SCREEN_HEIGHT / 2;
        let mut line_end: i32 = line_height as i32 / 2 + SCREEN_HEIGHT / 2;
        if line_start < 0 {
            line_start = 0;
        };
        if line_end > SCREEN_HEIGHT {
            line_end = SCREEN_HEIGHT
        };
        let tex_step = texture.height as f32 / line_height;
        let mut tex_cord_y =
            (line_start - SCREEN_HEIGHT / 2 + line_height as i32 / 2) as f32 * tex_step;
        for y in line_start..line_end {
            if (tex_cord_y as i32) > texture.height {
                tex_cord_y = texture.height as f32 - 1.0;
            };
            pixel_buffer[x as usize][y as usize] =
                texture.get_color(tex_cord_x as i32, tex_cord_y as i32);
            tex_cord_y += tex_step;
        }
    }
}

fn cast_floors(player: &Player, texture: &mut Image, pixel_buffer: &mut Vec<Vec<Color>>) {
    let ray_dir_0 = player.dir - player.plane;
    let ray_dir_1 = player.dir + player.plane;
    for y in 0..SCREEN_HEIGHT {
        let y_h = y - SCREEN_HEIGHT / 2;
        let row_distance = 0.5 * SCREEN_HEIGHT as f32 / y_h as f32;
        let step = row_distance * (ray_dir_1 - ray_dir_0) / SCREEN_WIDTH as f32;
        let mut current_pos = player.pos + row_distance * ray_dir_0;
        for x in 0..SCREEN_WIDTH {
            let mut tex_cords = Vec2::from_array([texture.width as f32, texture.height as f32])
                * (current_pos - current_pos.floor());
            if tex_cords.x > texture.width as f32 {
                tex_cords.x = texture.width as f32 - 1.0;
            }
            if tex_cords.y > texture.height as f32 {
                tex_cords.y = texture.height as f32 - 1.0;
            }
            current_pos.x += step.x;
            current_pos.y += step.y;
            pixel_buffer[x as usize][y as usize] =
                texture.get_color(tex_cords.x as i32, tex_cords.y as i32);
            pixel_buffer[x as usize][(SCREEN_HEIGHT - y - 1) as usize] =
                texture.get_color(tex_cords.x as i32, tex_cords.y as i32)
        }
    }
}

fn main() {
    let mut player = Player::new([4.0, 4.0].into(), [1.0, 0.0].into(), 2.0, 16.0);
    let mut show_map = true;
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();
    let mut wall_texture =
        raylib::core::texture::Image::load_image("./textures/texture1.png").unwrap();
    let mut floor_texture =
        raylib::core::texture::Image::load_image("./textures/texture2.png").unwrap();
    rl.hide_cursor();
    let mut last_frame_time = rl.get_time();
    while !rl.window_should_close() {
        let delta_time = rl.get_time() - last_frame_time;
        last_frame_time = rl.get_time();
        player.handle_input(&rl, delta_time as f32);
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_TAB) {
            show_map = !show_map;
        }
        let mut draw_handle = rl.begin_drawing(&thread);
        draw(
            &mut draw_handle,
            &player,
            show_map,
            &mut wall_texture,
            &mut floor_texture,
        )
    }
}
