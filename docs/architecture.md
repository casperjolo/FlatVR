# FlatVR architecture (Linux-first)

## Goal
Run OpenXR/SteamVR content with no physical HMD/controllers by feeding synthetic poses and actions from desktop inputs.

## Planned stack
1. **XR bridge layer**
   - Preferred: OpenXR API layer to intercept `xrLocateViews`, `xrLocateSpace`, and action state queries.
   - Compatibility fallback: OpenVR/driver shim for older SteamVR titles.
2. **Input mapper**
   - Mouse delta -> HMD yaw/pitch.
   - Keyboard WASD/space/ctrl -> locomotion translation.
3. **Runtime integration**
   - Bazzite/SteamOS-like setups: start from Steam session with OpenXR runtime exported.
   - Optionally integrate with Monado for pure OpenXR debugging.
4. **Output compositor modes**
   - Mono mirror (single flat camera).
   - Side-by-side (debug only).

## MVP milestones
- [x] CLI prototype loop for synthetic pose simulation (`src/main.rs`).
- [ ] Convert prototype to shared-memory pose publisher.
- [ ] Build OpenXR API layer library that consumes published pose.
- [ ] Add action emulation profiles (controller trigger/grip/menu).
- [ ] Add per-game profiles and auto-detection.
