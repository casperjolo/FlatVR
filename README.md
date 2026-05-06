# FlatVR

FlatVR is a Linux-focused project to let users play VR games from a flat screen with no physical headset or motion controllers.

## Project status
Early prototype.

Current prototype supports:
- Mouse/keyboard-driven virtual HMD + locomotion simulation.
- JSON pose output in stdout.
- Pose publishing to a local mmap-backed file (`/tmp/flatvr_pose.bin`) for bridge consumers.
- Mouse-driven virtual HMD yaw/pitch.
- Keyboard locomotion (WASD + Space/C).
- JSON pose output at configurable tick-rate.

This is the foundation for an OpenXR/SteamVR bridge layer that will make games think a headset is present.

## Why this approach
For your Bazzite environment, we are targeting:
- Steam + SteamVR compatibility.
- OpenXR-native bridge path first.
- Linux-first tooling and launch scripts.

## Quick start
```bash
cargo run --release -- --tick-hz 90
```

Custom pose output file:
```bash
cargo run -- --pose-file /tmp/flatvr_pose.bin
```

Optional config file:
```bash
cargo run -- --config flatvr.json
```

## Next implementation steps
See `docs/architecture.md` for the roadmap.


Inspect mmap output:
```bash
cargo run --bin read_pose -- /tmp/flatvr_pose.bin
```

The mmap packet is a fixed binary protocol with magic (`FLATVR01`), version, sequence, timestamp, then pose floats.
Example `flatvr.json`:
```json
{
  "mouse_sensitivity_yaw": 0.003,
  "mouse_sensitivity_pitch": 0.002,
  "keyboard_speed_mps": 2.5,
  "keyboard_vertical_speed_mps": 1.5
}
```

## Next implementation steps
See `docs/architecture.md` for the roadmap.
