use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::Context;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "flatvr")]
#[command(about = "Flat-screen OpenXR/SteamVR control prototype")]
struct Cli {
    /// Optional JSON config file.
    #[arg(long)]
    config: Option<PathBuf>,

    /// Target simulation tick-rate.
    #[arg(long, default_value_t = 90)]
    tick_hz: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    mouse_sensitivity_yaw: f32,
    mouse_sensitivity_pitch: f32,
    keyboard_speed_mps: f32,
    keyboard_vertical_speed_mps: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mouse_sensitivity_yaw: 0.003,
            mouse_sensitivity_pitch: 0.002,
            keyboard_speed_mps: 2.5,
            keyboard_vertical_speed_mps: 1.5,
        }
    }
}

#[derive(Debug, Default)]
struct InputState {
    forward: bool,
    back: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct Pose {
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
    pitch: f32,
    roll: f32,
}

impl Default for Pose {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 1.65,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = load_config(cli.config)?;
    let tick_dt = Duration::from_secs_f32(1.0 / cli.tick_hz as f32);

    let mut pose = Pose::default();
    let mut input = InputState::default();
    let mut last_tick = Instant::now();

    println!("FlatVR prototype loop started. Press Esc to quit.");
    println!("WASD move, Space/Ctrl up/down, mouse controls yaw/pitch.");

    loop {
        while event::poll(Duration::from_millis(1))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Char('w') => input.forward = true,
                        KeyCode::Char('s') => input.back = true,
                        KeyCode::Char('a') => input.left = true,
                        KeyCode::Char('d') => input.right = true,
                        KeyCode::Char(' ') => input.up = true,
                        KeyCode::Char('c') => input.down = true,
                        KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
                Event::Key(key) if key.kind == KeyEventKind::Release => {
                    match key.code {
                        KeyCode::Char('w') => input.forward = false,
                        KeyCode::Char('s') => input.back = false,
                        KeyCode::Char('a') => input.left = false,
                        KeyCode::Char('d') => input.right = false,
                        KeyCode::Char(' ') => input.up = false,
                        KeyCode::Char('c') => input.down = false,
                        _ => {}
                    }
                }
                Event::Mouse(mouse) => {
                    if let MouseEventKind::Moved = mouse.kind {
                        pose.yaw += mouse.column as f32 * config.mouse_sensitivity_yaw;
                        pose.pitch = (pose.pitch - mouse.row as f32 * config.mouse_sensitivity_pitch)
                            .clamp(-1.4, 1.4);
                    }
                }
                _ => {}
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs_f32();
        if dt >= tick_dt.as_secs_f32() {
            simulate(&mut pose, &input, &config, dt);
            println!("{}", serde_json::to_string(&pose)?);
            last_tick = now;
        }
    }
}

fn simulate(pose: &mut Pose, input: &InputState, config: &Config, dt: f32) {
    let mut dz = 0.0;
    let mut dx = 0.0;
    let mut dy = 0.0;

    if input.forward {
        dz += 1.0;
    }
    if input.back {
        dz -= 1.0;
    }
    if input.left {
        dx -= 1.0;
    }
    if input.right {
        dx += 1.0;
    }
    if input.up {
        dy += 1.0;
    }
    if input.down {
        dy -= 1.0;
    }

    let speed = config.keyboard_speed_mps * dt;
    let vertical_speed = config.keyboard_vertical_speed_mps * dt;

    let yaw_sin = pose.yaw.sin();
    let yaw_cos = pose.yaw.cos();
    pose.x += (dx * yaw_cos - dz * yaw_sin) * speed;
    pose.z += (dx * yaw_sin + dz * yaw_cos) * speed;
    pose.y = (pose.y + dy * vertical_speed).clamp(0.2, 3.0);
}

fn load_config(path: Option<PathBuf>) -> anyhow::Result<Config> {
    match path {
        Some(path) => {
            let raw = fs::read_to_string(&path)
                .with_context(|| format!("failed to read config file {}", path.display()))?;
            let config = serde_json::from_str::<Config>(&raw)
                .with_context(|| format!("failed to parse config file {}", path.display()))?;
            Ok(config)
        }
        None => Ok(Config::default()),
    }
}
