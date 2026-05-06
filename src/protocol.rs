use std::time::{SystemTime, UNIX_EPOCH};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PosePacket {
    pub magic: [u8; 8],
    pub version: u32,
    pub sequence: u64,
    pub timestamp_ns: u128,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl PosePacket {
    pub const MAGIC: [u8; 8] = *b"FLATVR01";

    pub fn new(sequence: u64, pose: super::Pose) -> Self {
        Self {
            magic: Self::MAGIC,
            version: 1,
            sequence,
            timestamp_ns: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
            x: pose.x,
            y: pose.y,
            z: pose.z,
            yaw: pose.yaw,
            pitch: pose.pitch,
            roll: pose.roll,
        }
    }

    pub fn encode(self) -> Vec<u8> {
        let mut out = Vec::with_capacity(56);
        out.extend_from_slice(&self.magic);
        out.extend_from_slice(&self.version.to_le_bytes());
        out.extend_from_slice(&self.sequence.to_le_bytes());
        out.extend_from_slice(&self.timestamp_ns.to_le_bytes());
        out.extend_from_slice(&self.x.to_le_bytes());
        out.extend_from_slice(&self.y.to_le_bytes());
        out.extend_from_slice(&self.z.to_le_bytes());
        out.extend_from_slice(&self.yaw.to_le_bytes());
        out.extend_from_slice(&self.pitch.to_le_bytes());
        out.extend_from_slice(&self.roll.to_le_bytes());
        out
    }
}
