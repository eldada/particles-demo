use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum Preset {
    #[default]
    Fountain,
    Classic,
    Fire,
    Snow,
    Galaxy,
}

impl Preset {
    pub fn name(self) -> &'static str {
        match self {
            Self::Fountain => "Fountain",
            Self::Classic => "Classic",
            Self::Fire => "Fire",
            Self::Snow => "Snow",
            Self::Galaxy => "Galaxy",
        }
    }

    pub fn from_digit(d: u8) -> Option<Self> {
        match d {
            1 => Some(Self::Fountain),
            2 => Some(Self::Classic),
            3 => Some(Self::Fire),
            4 => Some(Self::Snow),
            5 => Some(Self::Galaxy),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Parser)]
#[command(
    name = "particles-demo",
    about = "Interactive particle fountain demo",
    version
)]
pub struct Cli {
    /// Particles per system (Classic uses 4 systems)
    #[arg(long, default_value_t = 1600)]
    pub particles: u32,

    /// Starting preset
    #[arg(long, value_enum, default_value_t = Preset::Fountain)]
    pub preset: Preset,

    /// Enable / disable vsync
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub vsync: bool,

    /// RNG seed for reproducible demos
    #[arg(long, default_value_t = 42)]
    pub seed: u64,

    /// Initial window width
    #[arg(long, default_value_t = 1280)]
    pub width: u32,

    /// Initial window height
    #[arg(long, default_value_t = 800)]
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub particles_per_system: u32,
    pub preset: Preset,
    pub vsync: bool,
    pub seed: u64,
    pub width: u32,
    pub height: u32,
}

impl From<Cli> for AppConfig {
    fn from(cli: Cli) -> Self {
        Self {
            particles_per_system: cli.particles.clamp(100, 200_000),
            preset: cli.preset,
            vsync: cli.vsync,
            seed: cli.seed,
            width: cli.width.max(320),
            height: cli.height.max(240),
        }
    }
}

pub const GRAVITY_STEP: f32 = 0.2;
pub const DEFAULT_PARTICLE_SIZE: f32 = 0.015;
pub const MIN_PARTICLES: u32 = 100;
pub const MAX_PARTICLES: u32 = 200_000;
pub const REF_DT: f32 = 1.0 / 60.0;
