use minifb::{Key, Window, WindowOptions};


const WIDTH: usize = 1600;
const HEIGHT: usize = 900;

const PADDLE_WIDTH: usize = 20;
const PADDLE_HEIGHT: usize = 100;
const BALL_SIZE: usize = 20;

const PADDLE_SPEED: f32 = 400.0;
const BALL_SPEED: f32 = 300.0;

struct Paddle {
    x: usize,
    y: f32,
}

struct Ball {
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
}

fn draw_rect(buffer: &mut [u32], x: usize, y: usize, w: usize, h: usize, color: u32) {
    for iy in 0..h {
        for ix in 0..w {
            let px = x + ix;
            let py = y + iy;
            if px < WIDTH && py < HEIGHT {
                buffer[py * WIDTH + px] = color;
            }
        }
    }
}

fn main() {
    let mut window = match Window::new(
        "Pong",
        WIDTH,
        HEIGHT,
        WindowOptions::default()
    ){
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut left_paddle = Paddle { x: 20, y: (HEIGHT / 2 - PADDLE_HEIGHT / 2) as f32 };
    let mut right_paddle = Paddle { x: WIDTH - 40, y: (HEIGHT / 2 - PADDLE_HEIGHT / 2) as f32 };
    
    
    let mut ball = Ball {
        x: (WIDTH / 2) as f32,
        y: (HEIGHT / 2) as f32,
        vel_x: BALL_SPEED,
        vel_y: BALL_SPEED,
    };

    let mut last_time = std::time::Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = std::time::Instant::now();
        let dt = (now - last_time).as_secs_f32();
        last_time = now;

        if window.is_key_down(Key::W) {
            left_paddle.y -= PADDLE_SPEED * dt;
        }
        if window.is_key_down(Key::S) {
            left_paddle.y += PADDLE_SPEED * dt;
        }
        if window.is_key_down(Key::Up) {
            right_paddle.y -= PADDLE_SPEED * dt;
        }
        if window.is_key_down(Key::Down) {
            right_paddle.y += PADDLE_SPEED * dt;
        }

        left_paddle.y = left_paddle.y.clamp(0.0, (HEIGHT - PADDLE_HEIGHT) as f32);
        right_paddle.y = right_paddle.y.clamp(0.0, (HEIGHT - PADDLE_HEIGHT) as f32);

        ball.x += ball.vel_x * dt;
        ball.y += ball.vel_y * dt;

        if ball.y <= 0.0 || ball.y >= (HEIGHT - BALL_SIZE) as f32 {
            ball.vel_y = -ball.vel_y;
        }

        if ball.x <= 0.0 as f32 {
            //score1 += 1;
            left_paddle.y = (HEIGHT / 2 - PADDLE_HEIGHT / 2) as f32;
            right_paddle.y = (HEIGHT / 2 - PADDLE_HEIGHT / 2) as f32;

            ball.x = (WIDTH / 2) as f32;
            ball.y = (HEIGHT / 2) as f32;
            ball.vel_x = BALL_SPEED;
            ball.vel_y = BALL_SPEED;
        }
        if ball.x >= WIDTH as f32 {
            //score0 += 1;
            left_paddle.y = (HEIGHT / 2 - PADDLE_HEIGHT / 2) as f32;
            right_paddle.y = (HEIGHT / 2 - PADDLE_HEIGHT / 2) as f32;

            ball.x = (WIDTH / 2) as f32;
            ball.y = (HEIGHT / 2) as f32;
            ball.vel_x = BALL_SPEED;
            ball.vel_y = BALL_SPEED;
        }

        if ball.x <= (left_paddle.x + PADDLE_WIDTH) as f32
            && ball.y + BALL_SIZE as f32 >= left_paddle.y
            && ball.y <= left_paddle.y + PADDLE_HEIGHT as f32
        {
            ball.vel_x = BALL_SPEED;
        }

        if ball.x + BALL_SIZE as f32 >= right_paddle.x as f32
            && ball.y + BALL_SIZE as f32 >= right_paddle.y
            && ball.y <= right_paddle.y + PADDLE_HEIGHT as f32
        {
            ball.vel_x = -BALL_SPEED;
        }

        buffer.fill(0);

        draw_rect(&mut buffer, left_paddle.x, left_paddle.y as usize, PADDLE_WIDTH, PADDLE_HEIGHT, 0xFFFFFF);
        draw_rect(&mut buffer, right_paddle.x, right_paddle.y as usize, PADDLE_WIDTH, PADDLE_HEIGHT, 0xFFFFFF);
        draw_rect(&mut buffer, ball.x as usize, ball.y as usize, BALL_SIZE, BALL_SIZE, 0xFFFFFF);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}