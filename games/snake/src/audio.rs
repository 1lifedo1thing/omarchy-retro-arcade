//! Original PCM cues; the owned player is stopped on pause, exit and drop.
use std::{
    io::Write,
    process::{Child, Command, Stdio},
};
#[derive(Default)]
pub struct Audio {
    child: Option<Child>,
    file: Option<tempfile::NamedTempFile>,
}
impl Audio {
    pub fn stop(&mut self) {
        if let Some(mut c) = self.child.take() {
            let _ = c.kill();
            let _ = c.wait();
        }
        self.file = None;
    }
    pub fn play(&mut self, lost: bool) {
        self.stop();
        let rate = 22050u32;
        let count = if lost { 4400u32 } else { 1800 };
        let mut bytes = Vec::new();
        bytes.extend(b"RIFF");
        bytes.extend((36 + count * 2).to_le_bytes());
        bytes.extend(b"WAVEfmt ");
        bytes.extend(16u32.to_le_bytes());
        bytes.extend(1u16.to_le_bytes());
        bytes.extend(1u16.to_le_bytes());
        bytes.extend(rate.to_le_bytes());
        bytes.extend((rate * 2).to_le_bytes());
        bytes.extend(2u16.to_le_bytes());
        bytes.extend(16u16.to_le_bytes());
        bytes.extend(b"data");
        bytes.extend((count * 2).to_le_bytes());
        for i in 0..count {
            let t = i as f64 / rate as f64;
            let f = if lost {
                180. - 240. * t
            } else {
                740. + 1800. * t
            };
            let envelope = (1. - i as f64 / count as f64).powi(2) * (i as f64 / 80.).min(1.);
            let sample = ((t * f * std::f64::consts::TAU).sin() * envelope * 2800.) as i16;
            bytes.extend(sample.to_le_bytes());
        }
        let Ok(mut file) = tempfile::NamedTempFile::new() else {
            return;
        };
        if file.write_all(&bytes).is_err() {
            return;
        }
        self.child = Command::new("paplay")
            .arg(file.path())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .ok();
        self.file = Some(file);
    }
}
impl Drop for Audio {
    fn drop(&mut self) {
        self.stop();
    }
}
