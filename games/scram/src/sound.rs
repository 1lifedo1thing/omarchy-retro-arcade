//! Short original synthesized cues. Playback never blocks the simulation.
use crate::game::Event;
use arcade_platform::audio::Playback;
#[derive(Default)]
pub struct Sound {
    playback: Playback,
}
pub fn wave(event: Event) -> Vec<u8> {
    let count = match event {
        Event::Death => 6400,
        Event::Clear => 8000,
        _ => 3200,
    };
    let rate = 22050u32;
    let mut out = Vec::with_capacity(44 + count * 2);
    out.extend(b"RIFF");
    out.extend((36 + count as u32 * 2).to_le_bytes());
    out.extend(b"WAVEfmt ");
    out.extend(16u32.to_le_bytes());
    out.extend(1u16.to_le_bytes());
    out.extend(1u16.to_le_bytes());
    out.extend(rate.to_le_bytes());
    out.extend((rate * 2).to_le_bytes());
    out.extend(2u16.to_le_bytes());
    out.extend(16u16.to_le_bytes());
    out.extend(b"data");
    out.extend((count as u32 * 2).to_le_bytes());
    let mut phase = 0f32;
    for i in 0..count {
        let t = i as f32 / count as f32;
        let freq = match event {
            Event::Death => 480. - 360. * t,
            Event::Clear => [440., 554., 660., 880.][(t * 4.) as usize],
            Event::Power => 330. + t * 440.,
            Event::Capture => 880. - t * 220.,
            _ => 660.,
        };
        phase += freq / rate as f32;
        let amplitude = if phase.fract() < 0.5 { 1. } else { -1. };
        let env = (t * 30.).min(1.) * (1. - t);
        out.extend(((amplitude * env * 1900.) as i16).to_le_bytes());
    }
    out
}
impl Sound {
    pub fn available() -> bool {
        Playback::available()
    }
    pub fn play(&self, event: Event) {
        if event == Event::Dot {
            return;
        }
        self.playback.play(move || wave(event));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_cues_have_valid_pcm_lengths() {
        for e in [Event::Power, Event::Capture, Event::Death, Event::Clear] {
            let w = wave(e);
            assert_eq!(&w[..4], b"RIFF");
            assert_eq!(
                u32::from_le_bytes(w[40..44].try_into().unwrap()) as usize,
                w.len() - 44
            );
            assert!(w.len() < 22050);
        }
    }
}
