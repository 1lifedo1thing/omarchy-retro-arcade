//! Application identity, desktop window configuration and singleton lock.
use eframe::egui;
use fs2::FileExt;
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

pub(crate) fn acquire_session_lock() -> Result<File, Box<dyn std::error::Error>> {
    let state = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/state")))
        .ok_or("No state directory")?
        .join("omarchy-retro-arcade");
    std::fs::create_dir_all(&state)?;
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(state.join("session.lock"))?;
    lock.try_lock_exclusive()
        .map_err(|_| "Omarchy Arcade is already running")?;
    Ok(lock)
}

pub(crate) fn native_options(
    size: [f32; 2],
) -> Result<eframe::NativeOptions, Box<dyn std::error::Error>> {
    let image = egui_extras::image::load_svg_bytes_with_size(
        include_bytes!("../../packaging/omarchy-retro-arcade.svg"),
        Some(egui::load::SizeHint::Size(128, 128)),
    )?;
    let icon = egui::IconData {
        rgba: image.pixels.iter().flat_map(|p| p.to_array()).collect(),
        width: 128,
        height: 128,
    };
    Ok(eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(size)
            .with_min_inner_size([900., 760.])
            .with_app_id("io.github.tcballard.omarchy-retro-arcade")
            .with_icon(icon),
        ..Default::default()
    })
}
