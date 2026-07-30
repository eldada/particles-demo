use glam::{Mat3, Vec3};
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use std::f32::consts::{FRAC_PI_6, TAU};

use crate::config::{
    DEFAULT_PARTICLE_SIZE, GRAVITY_STEP, MAX_PARTICLES, MIN_PARTICLES, REF_DT, Preset,
};

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub pos: Vec3,
    pub vel: Vec3,
    pub gravity: Vec3,
    pub life: f32,
    pub acc: f32,
    /// Polar angle around the galaxy center (galaxy motion only).
    pub orbit_angle: f32,
    /// Distance from the galaxy center in the disk plane (galaxy motion only).
    pub orbit_radius: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum MotionKind {
    /// Classic fountain-style ballistic motion.
    Ballistic,
    /// Particles spawn near center, spin, and spiral outward on a tilted disk.
    Galaxy {
        /// Disk tilt around X, in radians.
        tilt: f32,
        /// Base angular speed (rad per ref-frame).
        spin: f32,
        /// Outward radial growth per ref-frame.
        expand: f32,
        /// Half-thickness of the disk.
        thickness: f32,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct SystemStyle {
    pub color: [f32; 4],
    pub offset: Vec3,
    pub size: f32,
    pub vel_min: Vec3,
    pub vel_max: Vec3,
    pub life_min: f32,
    pub life_max: f32,
    pub acc_min: f32,
    pub acc_max: f32,
    pub base_gravity: Vec3,
    pub motion: MotionKind,
}

impl Default for SystemStyle {
    fn default() -> Self {
        Self {
            color: [1.0, 1.0, 1.0, 1.0],
            offset: Vec3::ZERO,
            size: DEFAULT_PARTICLE_SIZE,
            // 30% slower than the original ±0.5 default speed
            vel_min: Vec3::splat(-0.35),
            vel_max: Vec3::splat(0.35),
            // +50% life vs original 0..4
            life_min: 0.0,
            life_max: 6.0,
            acc_min: 0.0,
            acc_max: 0.006,
            base_gravity: Vec3::ZERO,
            motion: MotionKind::Ballistic,
        }
    }
}

#[derive(Debug)]
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub style: SystemStyle,
    pub shared_gravity: Vec3,
    rng: StdRng,
}

impl ParticleSystem {
    pub fn new(count: usize, style: SystemStyle, seed: u64) -> Self {
        let mut sys = Self {
            particles: Vec::with_capacity(count),
            style,
            shared_gravity: style.base_gravity,
            rng: StdRng::seed_from_u64(seed),
        };
        sys.particles
            .resize(count, Particle::dead());
        sys.reinit_all();
        sys
    }

    pub fn resize(&mut self, count: usize) {
        let count = count.clamp(MIN_PARTICLES as usize, MAX_PARTICLES as usize);
        let old = self.particles.len();
        if count == old {
            return;
        }
        if count > old {
            // Keep existing particles; only spawn the newly added slots.
            self.particles.resize(count, Particle::dead());
            for i in old..count {
                self.respawn(i);
            }
        } else {
            self.particles.truncate(count);
        }
    }

    pub fn reinit_all(&mut self) {
        for i in 0..self.particles.len() {
            self.respawn(i);
            // Match original InitParticles: start with zero accumulated gravity
            // but keep shared_gravity for ongoing force (cleared separately).
            self.particles[i].gravity = self.shared_gravity;
        }
    }

    pub fn clear_gravity(&mut self) {
        self.shared_gravity = self.style.base_gravity;
        for p in &mut self.particles {
            p.gravity = self.shared_gravity;
        }
    }

    pub fn add_gravity(&mut self, delta: Vec3) {
        self.shared_gravity += delta;
        for p in &mut self.particles {
            p.gravity += delta;
        }
    }

    pub fn apply_attractor(&mut self, world_point: Vec3, strength: f32, dt: f32) {
        let local = world_point - self.style.offset;
        for p in &mut self.particles {
            let dir = local - p.pos;
            let dist_sq = dir.length_squared().max(0.0004);
            let force = dir.normalize_or_zero() * (strength / dist_sq) * dt;
            p.gravity += force;
        }
    }

    fn respawn(&mut self, i: usize) {
        let s = &self.style;
        let life = self.rng.random_range(s.life_min..=s.life_max.max(s.life_min + 0.01));
        let acc = self.rng.random_range(s.acc_min..=s.acc_max.max(s.acc_min + 1e-6));

        match s.motion {
            MotionKind::Ballistic => {
                let vel = Vec3::new(
                    self.rng.random_range(s.vel_min.x..=s.vel_max.x),
                    self.rng.random_range(s.vel_min.y..=s.vel_max.y),
                    self.rng.random_range(s.vel_min.z..=s.vel_max.z),
                );
                self.particles[i] = Particle {
                    pos: Vec3::ZERO,
                    vel,
                    gravity: self.shared_gravity,
                    life,
                    acc,
                    orbit_angle: 0.0,
                    orbit_radius: 0.0,
                };
            }
            MotionKind::Galaxy {
                thickness, ..
            } => {
                // Spawn near the center with a random phase; vel carries spin/expand jitter.
                let spin_jitter = self.rng.random_range(0.85..1.15);
                let expand_jitter = self.rng.random_range(0.7..1.3);
                let height = self.rng.random_range(-thickness..=thickness);
                self.particles[i] = Particle {
                    pos: Vec3::ZERO,
                    vel: Vec3::new(spin_jitter, expand_jitter, height),
                    gravity: self.shared_gravity,
                    life,
                    acc,
                    orbit_angle: self.rng.random_range(0.0..TAU),
                    orbit_radius: self.rng.random_range(0.01..0.06),
                };
            }
        }
    }

    /// Frame-rate independent update. Scales original per-frame feel to ~60 FPS.
    pub fn update(&mut self, dt: f32) {
        let steps = (dt / REF_DT).clamp(0.0, 4.0);
        if steps <= 0.0 {
            return;
        }

        match self.style.motion {
            MotionKind::Ballistic => self.update_ballistic(steps),
            MotionKind::Galaxy {
                tilt,
                spin,
                expand,
                ..
            } => self.update_galaxy(steps, tilt, spin, expand),
        }
    }

    fn update_ballistic(&mut self, steps: f32) {
        for i in 0..self.particles.len() {
            if self.particles[i].life > 0.0 {
                let p = &mut self.particles[i];
                let life_drain: f32 = self.rng.random_range(0.0..0.1) * steps;
                p.life -= life_drain;
                p.pos += (p.vel + p.gravity) * p.acc * steps;
                let decay: f32 = 1.0 - self.rng.random_range(0.0..0.01) * steps.min(1.0);
                p.acc *= decay.max(0.0);
            } else {
                self.respawn(i);
            }
        }
    }

    fn update_galaxy(&mut self, steps: f32, tilt: f32, spin: f32, expand: f32) {
        let tilt_mat = Mat3::from_rotation_x(tilt);
        for i in 0..self.particles.len() {
            if self.particles[i].life > 0.0 {
                let p = &mut self.particles[i];
                let life_drain: f32 = self.rng.random_range(0.0..0.05) * steps;
                p.life -= life_drain;

                // Slowly spiral outward while rotating; farther-out particles orbit slower.
                p.orbit_radius += expand * p.vel.y * steps;
                let omega = spin * p.vel.x / (0.2 + p.orbit_radius).sqrt();
                p.orbit_angle = (p.orbit_angle + omega * steps) % TAU;

                let (s, c) = p.orbit_angle.sin_cos();
                let disk = Vec3::new(
                    p.orbit_radius * c,
                    p.vel.z * (1.0 / (1.0 + p.orbit_radius * 0.8)),
                    p.orbit_radius * s,
                );
                // Arrow-key / attractor gravity gently shifts the whole orbit.
                p.pos = tilt_mat * disk + p.gravity * 0.05;
            } else {
                self.respawn(i);
            }
        }
    }

    pub fn count(&self) -> usize {
        self.particles.len()
    }
}

impl Particle {
    fn dead() -> Self {
        Self {
            pos: Vec3::ZERO,
            vel: Vec3::ZERO,
            gravity: Vec3::ZERO,
            life: 0.0,
            acc: 0.0,
            orbit_angle: 0.0,
            orbit_radius: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct Scene {
    pub systems: Vec<ParticleSystem>,
    pub preset: Preset,
    pub seed: u64,
    pub particles_per_system: u32,
}

impl Scene {
    pub fn new(preset: Preset, particles_per_system: u32, seed: u64) -> Self {
        let mut scene = Self {
            systems: Vec::new(),
            preset,
            seed,
            particles_per_system,
        };
        scene.apply_preset(preset);
        scene
    }

    pub fn total_particles(&self) -> usize {
        self.systems.iter().map(|s| s.count()).sum()
    }

    pub fn shared_gravity(&self) -> Vec3 {
        self.systems
            .first()
            .map(|s| s.shared_gravity)
            .unwrap_or(Vec3::ZERO)
    }

    pub fn apply_preset(&mut self, preset: Preset) {
        self.preset = preset;
        let n = self.particles_per_system as usize;
        let styles = styles_for_preset(preset);
        self.systems = styles
            .into_iter()
            .enumerate()
            .map(|(i, style)| ParticleSystem::new(n, style, self.seed.wrapping_add(i as u64 * 9973)))
            .collect();
    }

    pub fn set_particle_count(&mut self, count: u32) {
        self.particles_per_system = count.clamp(MIN_PARTICLES, MAX_PARTICLES);
        for sys in &mut self.systems {
            sys.resize(self.particles_per_system as usize);
        }
    }

    pub fn adjust_particle_count(&mut self, factor: f32) {
        let next = ((self.particles_per_system as f32) * factor).round() as u32;
        self.set_particle_count(next.max(MIN_PARTICLES));
    }

    pub fn reinit_all(&mut self) {
        for sys in &mut self.systems {
            sys.reinit_all();
        }
    }

    pub fn clear_gravity(&mut self) {
        for sys in &mut self.systems {
            sys.clear_gravity();
        }
    }

    pub fn add_gravity(&mut self, delta: Vec3) {
        for sys in &mut self.systems {
            sys.add_gravity(delta);
        }
    }

    pub fn apply_attractor(&mut self, world_point: Vec3, strength: f32, dt: f32) {
        for sys in &mut self.systems {
            sys.apply_attractor(world_point, strength, dt);
        }
    }

    pub fn update(&mut self, dt: f32) {
        for sys in &mut self.systems {
            sys.update(dt);
        }
    }
}

fn styles_for_preset(preset: Preset) -> Vec<SystemStyle> {
    match preset {
        Preset::Classic => vec![
            SystemStyle {
                color: [0.8, 0.8, 0.1, 1.0],
                offset: Vec3::new(-0.25, 0.0, -2.0),
                ..SystemStyle::default()
            },
            SystemStyle {
                color: [0.8, 0.1, 0.1, 1.0],
                offset: Vec3::new(0.25, 0.0, -2.0),
                ..SystemStyle::default()
            },
            SystemStyle {
                color: [0.1, 0.8, 0.1, 1.0],
                offset: Vec3::new(0.25, -0.2, -1.7),
                ..SystemStyle::default()
            },
            SystemStyle {
                color: [0.1, 0.1, 0.8, 1.0],
                offset: Vec3::new(-0.25, -0.2, -1.7),
                ..SystemStyle::default()
            },
        ],
        Preset::Fountain => vec![SystemStyle {
            color: [0.45, 0.75, 1.0, 1.0],
            offset: Vec3::new(0.0, -0.6, -2.2),
            size: 0.0175,
            vel_min: Vec3::new(-0.175, 0.42, -0.175),
            vel_max: Vec3::new(0.175, 0.84, 0.175),
            life_min: 1.5,
            life_max: 5.25,
            acc_min: 0.002,
            acc_max: 0.008,
            base_gravity: Vec3::new(0.0, -0.35, 0.0),
            motion: MotionKind::Ballistic,
        }],
        Preset::Fire => vec![
            SystemStyle {
                color: [1.0, 0.45, 0.08, 1.0],
                offset: Vec3::new(0.0, -0.5, -2.0),
                size: 0.02,
                vel_min: Vec3::new(-0.14, 0.21, -0.14),
                vel_max: Vec3::new(0.14, 0.63, 0.14),
                life_min: 0.45,
                life_max: 2.7,
                acc_min: 0.003,
                acc_max: 0.01,
                base_gravity: Vec3::new(0.0, 0.15, 0.0),
                motion: MotionKind::Ballistic,
            },
            SystemStyle {
                color: [1.0, 0.15, 0.05, 1.0],
                offset: Vec3::new(0.0, -0.5, -2.0),
                size: 0.014,
                vel_min: Vec3::new(-0.105, 0.28, -0.105),
                vel_max: Vec3::new(0.105, 0.77, 0.105),
                life_min: 0.3,
                life_max: 1.8,
                acc_min: 0.004,
                acc_max: 0.012,
                base_gravity: Vec3::new(0.0, 0.25, 0.0),
                motion: MotionKind::Ballistic,
            },
        ],
        Preset::Snow => vec![SystemStyle {
            color: [0.85, 0.9, 1.0, 1.0],
            offset: Vec3::new(0.0, 0.9, -2.4),
            size: 0.0125,
            vel_min: Vec3::new(-0.105, -0.245, -0.07),
            vel_max: Vec3::new(0.105, -0.035, 0.07),
            life_min: 3.0,
            life_max: 9.0,
            acc_min: 0.001,
            acc_max: 0.004,
            base_gravity: Vec3::new(0.0, -0.05, 0.0),
            motion: MotionKind::Ballistic,
        }],
        Preset::Galaxy => {
            // 30° tilted disk; particles spawn at center and spiral outward while orbiting.
            let tilt = FRAC_PI_6;
            let center = Vec3::new(0.0, 0.0, -2.4);
            vec![
                SystemStyle {
                    color: [0.75, 0.45, 1.0, 1.0],
                    offset: center,
                    size: 0.01,
                    life_min: 8.0,
                    life_max: 16.0,
                    motion: MotionKind::Galaxy {
                        tilt,
                        spin: 0.045,
                        expand: 0.0045,
                        thickness: 0.035,
                    },
                    ..SystemStyle::default()
                },
                SystemStyle {
                    color: [0.25, 0.75, 1.0, 1.0],
                    offset: center,
                    size: 0.008,
                    life_min: 7.0,
                    life_max: 14.0,
                    motion: MotionKind::Galaxy {
                        tilt,
                        spin: 0.055,
                        expand: 0.0055,
                        thickness: 0.025,
                    },
                    ..SystemStyle::default()
                },
                SystemStyle {
                    color: [1.0, 0.85, 0.4, 1.0],
                    offset: center,
                    size: 0.012,
                    life_min: 6.0,
                    life_max: 12.0,
                    motion: MotionKind::Galaxy {
                        tilt,
                        spin: 0.035,
                        expand: 0.0035,
                        thickness: 0.02,
                    },
                    ..SystemStyle::default()
                },
            ]
        },
    }
}

pub fn gravity_from_arrows(up: bool, down: bool, left: bool, right: bool) -> Vec3 {
    let mut g = Vec3::ZERO;
    if up {
        g.y += GRAVITY_STEP;
    }
    if down {
        g.y -= GRAVITY_STEP;
    }
    if left {
        g.x -= GRAVITY_STEP;
    }
    if right {
        g.x += GRAVITY_STEP;
    }
    g
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_has_four_systems() {
        let scene = Scene::new(Preset::Classic, 100, 1);
        assert_eq!(scene.systems.len(), 4);
        assert_eq!(scene.total_particles(), 400);
    }

    #[test]
    fn update_keeps_pool_size() {
        let mut scene = Scene::new(Preset::Classic, 200, 7);
        for _ in 0..120 {
            scene.update(REF_DT);
        }
        assert_eq!(scene.total_particles(), 800);
    }

    #[test]
    fn gravity_accumulates() {
        let mut scene = Scene::new(Preset::Classic, 50, 3);
        scene.add_gravity(Vec3::new(0.2, 0.0, 0.0));
        scene.add_gravity(Vec3::new(0.2, 0.0, 0.0));
        assert!((scene.shared_gravity().x - 0.4).abs() < 1e-5);
        scene.clear_gravity();
        assert!(scene.shared_gravity().length() < 1e-5);
    }

    #[test]
    fn resize_preserves_existing_particles() {
        let mut scene = Scene::new(Preset::Fountain, 200, 11);
        scene.update(REF_DT);
        let before = scene.systems[0].particles[0].pos;
        scene.adjust_particle_count(2.0);
        assert_eq!(scene.systems[0].count(), 400);
        assert_eq!(scene.systems[0].particles[0].pos, before);
        scene.adjust_particle_count(0.5);
        assert_eq!(scene.systems[0].count(), 200);
        assert_eq!(scene.systems[0].particles[0].pos, before);
    }

    #[test]
    fn galaxy_spirals_outward() {
        let mut scene = Scene::new(Preset::Galaxy, 100, 5);
        assert_eq!(scene.systems.len(), 3);
        let r0 = scene.systems[0].particles[0].orbit_radius;
        for _ in 0..90 {
            scene.update(REF_DT);
        }
        let p = &scene.systems[0].particles[0];
        assert!(p.orbit_radius > r0, "radius should grow over time");
        assert!(p.pos.length() > 0.0);
    }
}
