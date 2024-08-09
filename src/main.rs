use ::core::f32;

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
    pub fov: f32,
    pub plane: glam::Vec2,
}

impl Player {
    fn new(pos: glam::Vec2, dir: glam::Vec2, fov: f32) -> Self {
        let plane: glam::Vec2 = [0.0, 0.66].into();

        Player {
            pos,
            dir,
            fov,
            plane,
        }
    }

    fn handle_input(&mut self, raylib_handle: &RaylibHandle) {
        let speed = 0.5;
        if raylib_handle.is_key_down(raylib::consts::KeyboardKey::KEY_E) {
            let rotation_matrix = get_rotation_matrix(1.0);
            self.dir = (rotation_matrix * self.dir).normalize();
            self.plane = rotation_matrix * self.plane;
        }
        if raylib_handle.is_key_down(raylib::consts::KeyboardKey::KEY_Q) {
            let rotation_matrix = get_rotation_matrix(-1.0);
            self.dir = (rotation_matrix * self.dir).normalize();
            self.plane = rotation_matrix * self.plane;
        }
        if raylib_handle.is_key_down(raylib::consts::KeyboardKey::KEY_W) {
            self.pos = (self.pos + self.dir * 0.01 * speed);
        }
        if raylib_handle.is_key_down(raylib::consts::KeyboardKey::KEY_S) {
            let rotation_matrix = get_rotation_matrix(180.0);
            self.pos = (self.pos + rotation_matrix * self.dir * 0.01 * speed);
        }
        if raylib_handle.is_key_down(raylib::consts::KeyboardKey::KEY_D) {
            let rotation_matrix = get_rotation_matrix(90.0);
            self.pos = (self.pos + rotation_matrix * self.dir * 0.01 * speed);
        }
        if raylib_handle.is_key_down(raylib::consts::KeyboardKey::KEY_A) {
            let rotation_matrix = get_rotation_matrix(-90.0);
            self.pos = (self.pos + rotation_matrix * self.dir * 0.01 * speed);
        }
    }
}

fn draw(draw_handle: &mut RaylibDrawHandle, player: &Player, show_map: bool) {
    draw_handle.clear_background(Color::WHITE);
    if show_map {
        for x in 0..GRID_COLS {
            for y in 0..GRID_ROWS {
                if GRID[y as usize][x as usize] == 1 {
                    draw_handle.draw_rectangle(
                        x as i32 * GRID_STRIDE_ROW,
                        y as i32 * GRID_STRIDE_COLS,
                        GRID_STRIDE_ROW,
                        GRID_STRIDE_COLS,
                        Color::BLACK,
                    );
                } else {
                    draw_handle.draw_rectangle_lines(
                        x as i32 * GRID_STRIDE_ROW,
                        y as i32 * GRID_STRIDE_COLS,
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
    } else {
        let line_heights = cast_ray(&player);
        for (idx, line_height) in line_heights.iter().enumerate() {
            let line_start: i32 = -line_height / 2 + SCREEN_HEIGHT / 2 as i32;
            let line_end: i32 = line_height / 2 + SCREEN_HEIGHT / 2 as i32;
            draw_handle.draw_line(
                idx as i32,
                if line_start > 0 { line_start } else { 0 },
                idx as i32,
                if line_end < SCREEN_HEIGHT {
                    line_end
                } else {
                    SCREEN_HEIGHT
                },
                Color::BLUE,
            );
        }
    }
}

fn cast_ray(player: &Player) -> Vec<i32> {
    let mut wall_heights: [i32; SCREEN_WIDTH as usize] = [0; SCREEN_WIDTH as usize];
    let mut lines: Vec<i32> = Vec::new();
    for x in 0..SCREEN_WIDTH {
        let camera_x = 2.00 * x as f32 / SCREEN_WIDTH as f32 - 1.0;
        let ray_dir_x = player.dir.x + player.plane.x * camera_x;
        let ray_dir_y = player.dir.y + player.plane.y * camera_x;
        let delta_x = f32::abs(1.0 / ray_dir_x);
        let delta_y = f32::abs(1.0 / ray_dir_y);
        let mut map_pos_x = player.pos.x.floor();
        let mut map_pos_y = player.pos.y.floor();
        let mut side_dir_y: f32;
        let mut side_dir_x: f32;
        let mut side: i32 = 0;
        let step_x: f32;
        let step_y: f32;
        if ray_dir_x < 0.0 {
            step_x = -1.0;
            side_dir_x = (player.pos.x - map_pos_x) * delta_x;
        } else {
            step_x = 1.0;
            side_dir_x = (map_pos_x + 1.0 - player.pos.x) * delta_x;
        }
        if ray_dir_y < 0.0 {
            step_y = -1.0;
            side_dir_y = (player.pos.y - map_pos_y) * delta_y;
        } else {
            step_y = 1.0;
            side_dir_y = (map_pos_y + 1.0 - player.pos.y) * delta_y;
        }
        let mut hit = false;
        while !hit {
            if side_dir_x < side_dir_y {
                side_dir_x += delta_x;
                map_pos_x += step_x;
                side = 0;
            } else {
                side_dir_y += delta_y;
                map_pos_y += step_y;
                side = 1;
            }
            if GRID[map_pos_y as usize][map_pos_x as usize] == 1 {
                hit = true;
            }
        }
        let ray_lenght: f32;
        if side == 0 {
            ray_lenght = side_dir_x - delta_x;
        } else {
            ray_lenght = side_dir_y - delta_y;
        }
        let line_height = SCREEN_HEIGHT as f32 / ray_lenght;
        lines.push(line_height as i32);
    }
    return lines;
}

fn main() {
    let mut player = Player::new([4.0, 4.0].into(), [1.0, 0.0].into(), 66.0);
    let mut show_map = true;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        player.handle_input(&rl);
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_TAB) {
            show_map = !show_map;
        }
        let mut draw_handle = rl.begin_drawing(&thread);
        draw(&mut draw_handle, &player, show_map);
    }
}
