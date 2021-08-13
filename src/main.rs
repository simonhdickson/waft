use macroquad::prelude::*;

struct Player {
    pos: Vec2,
}

struct Wave {
    pos: Vec2,
    radius: f32,
    thickness: f32,
}

impl Wave {
    fn update(&mut self, delta: f32) -> bool {
        self.radius += delta * 50.;
        self.thickness -= delta * 4.;
        self.thickness <= f32::EPSILON
    }

    fn render(&self) {
        draw_circle_lines(self.pos.x, self.pos.y, self.radius, self.thickness, DARKBLUE);
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut waves = Vec::new();

    loop {
        clear_background(BLUE);

        let delta = get_frame_time();
        
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();

            waves.push(Wave {
                pos: Vec2::new(x, y),
                radius: 0.,
                thickness: 5.,
            })
        }

        {
            let mut i = 0;
            while i < waves.len() {
                if waves[i].update(delta) {
                    waves.remove(i);
                } else {
                    i += 1;
                }
            }
        }
        
        for wave in waves.iter() {
            wave.render();
        }

        next_frame().await
    }
}
