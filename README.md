# FlatVR

FlatVR is a Linux-focused project to let users play VR games from a flat screen with no physical headset or motion controllers.

## Project status
Early prototype.

Current prototype supports:
- Mouse/keyboard-driven virtual HMD + locomotion simulation.
- JSON pose output in stdout.
- Pose publishing to a local mmap-backed file (`/tmp/flatvr_pose.bin`) for bridge consumers.

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
