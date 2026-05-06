use std::fs::OpenOptions;
use std::io::Read;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).unwrap_or("/tmp/flatvr_pose.bin".into());
    let mut buf = vec![0u8; 256];
    let mut file = OpenOptions::new()
        .read(true)
        .open(&path)
        .with_context(|| format!("failed to open {path}"))?;
    file.read_exact(&mut buf)?;

    let magic = &buf[0..8];
    if magic != b"FLATVR01" {
        anyhow::bail!("invalid magic in pose file");
    }

    let version = u32::from_le_bytes(buf[8..12].try_into()?);
    let sequence = u64::from_le_bytes(buf[12..20].try_into()?);
    let ts = u128::from_le_bytes(buf[20..36].try_into()?);
    let x = f32::from_le_bytes(buf[36..40].try_into()?);
    let y = f32::from_le_bytes(buf[40..44].try_into()?);
    let z = f32::from_le_bytes(buf[44..48].try_into()?);
    let yaw = f32::from_le_bytes(buf[48..52].try_into()?);
    let pitch = f32::from_le_bytes(buf[52..56].try_into()?);
    let roll = f32::from_le_bytes(buf[56..60].try_into()?);

    println!(
        "version={version} seq={sequence} ts_ns={ts} pos=({x:.3},{y:.3},{z:.3}) rot=({yaw:.3},{pitch:.3},{roll:.3})"
    );
    Ok(())
}
