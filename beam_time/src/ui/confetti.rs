use std::f32::consts::PI;

use engine::{
    drawable::sprite::Sprite,
    exports::nalgebra::{Rotation2, Vector2},
    graphics_context::{Anchor, GraphicsContext},
};
use rand::{thread_rng, Rng};

use crate::{assets::CONFETTI_PARTICLES, consts::layer};

pub struct Confetti {
    particles: Vec<Particle>,
}

struct Particle {
    sprite: Sprite,
    timer: f32,
    rotation: f32,
    position: Vector2<f32>,
    velocity: Vector2<f32>,
}

impl Confetti {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn emit(&mut self, position: Vector2<f32>, particles: u32, timer: f32) {
        let mut rng = thread_rng();

        for _ in 0..particles {
            let color = rng.gen_range(0..3);
            let style = rng.gen_range(0..2);

            let sprite = Sprite::new(CONFETTI_PARTICLES)
                .uv_offset(3 * Vector2::new(color, style))
                .scale(Vector2::repeat(4.0), Anchor::Center)
                .z_index(layer::OVERLAY);

            let angle = rng.gen_range(0.0..=2.0 * PI);
            let strength = rng.gen::<f32>() * 400.0;
            let velocity = Rotation2::new(angle) * Vector2::x() * strength;
            let rotation = (rng.gen::<f32>() * 2.0 - 1.0) * 3.0;

            self.particles.push(Particle {
                sprite,
                timer,
                rotation,
                position,
                velocity,
            });
        }
    }

    pub fn render<App>(&mut self, ctx: &mut GraphicsContext<App>) {
        let viewport = ctx.size();
        let half_size = 1.5 * 4.0 * ctx.scale_factor;

        self.particles.retain(|x| {
            x.position.x > -half_size
                && x.position.x < viewport.x + half_size
                && x.position.y > -half_size
        });

        for particle in self.particles.iter_mut() {
            if particle.timer > 0.0 {
                particle.timer -= ctx.delta_time;
                continue;
            }

            if particle.position.y < viewport.y + half_size {
                let sprite = particle.sprite.clone();
                ctx.draw(
                    sprite
                        .position(particle.position, Anchor::Center)
                        .rotate(ctx.frame as f32 / 100.0 * particle.rotation, Anchor::Center),
                );
            }

            const GRAVITY: f32 = 400.0;
            let half_dt = ctx.delta_time / 2.0;

            particle.velocity.y -= GRAVITY * half_dt;
            particle.position += particle.velocity * ctx.delta_time;
            particle.velocity.y -= GRAVITY * half_dt;
        }
    }
}
