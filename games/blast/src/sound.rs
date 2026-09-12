//! Original short PCM cues. The owned child is stopped and reaped on pause/drop.
use crate::rules::Event;
use std::{
    io::Write,
    process::{Child, Command, Stdio},
};
#[derive(Default)]
pub struct Sound {
    child: Option<Child>,
    file: Option<tempfile::NamedTempFile>,
}
impl Sound {
    pub fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        self.file = None;
    }
    pub fn play(&mut self, event: Event) {
        if self
            .child
            .as_mut()
            .is_some_and(|c| matches!(c.try_wait(), Ok(None)))
            && event == Event::Place
        {
            return;
        }
        self.stop();
        if let Ok(mut file) = tempfile::NamedTempFile::new() {
            if file.write_all(&wave(event)).is_ok() {
                if let Ok(child) = Command::new("paplay")
                    .arg(file.path())
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                {
                    self.child = Some(child);
                    self.file = Some(file);
                }
            }
        }
    }
}
impl Drop for Sound {
    fn drop(&mut self) {
        self.stop();
    }
}
pub fn wave(event: Event) -> Vec<u8> {
    let count = if event == Event::End { 8000 } else { 4000 };
    let mut out = Vec::new();
    out.extend(b"RIFF");
    out.extend((36 + count * 2u32).to_le_bytes());
    out.extend(b"WAVEfmt ");
    out.extend(16u32.to_le_bytes());
    out.extend(1u16.to_le_bytes());
    out.extend(1u16.to_le_bytes());
    out.extend(22050u32.to_le_bytes());
    out.extend(44100u32.to_le_bytes());
    out.extend(2u16.to_le_bytes());
    out.extend(16u16.to_le_bytes());
    out.extend(b"data");
    out.extend((count * 2).to_le_bytes());
    let mut phase = 0f32;
    for i in 0..count {
        let t = i as f32 / count as f32;
        let freq = match event {
            Event::Place => 180. + 80. * t,
            Event::Explode => 100. - 65. * t,
            Event::Pickup => 600. + 800. * t,
            Event::End => [392., 494., 587., 784.][(t * 4.) as usize],
        };
        phase += freq / 22050.;
        let sample = (phase * std::f32::consts::TAU).sin() * (1. - t).powi(2) * (t * 35.).min(1.);
        out.extend(((sample * 5000.) as i16).to_le_bytes());
    }
    out
}
