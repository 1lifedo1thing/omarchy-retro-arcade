use eframe::egui;
use omarchy_invaders::{storage::Store, App};
fn main() -> eframe::Result<()> {
    if std::env::args().any(|a| a == "--version") {
        println!("omarchy-invaders 0.1.0");
        return Ok(());
    }
    let store = match Store::open() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    let icon = image::load_from_memory(include_bytes!("../packaging/omarchy-invaders.png"))
        .expect("embedded icon")
        .to_rgba8();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Omarchy Invaders")
            .with_icon(egui::IconData {
                width: icon.width(),
                height: icon.height(),
                rgba: icon.into_raw(),
            })
            .with_inner_size([860., 900.])
            .with_min_inner_size([600., 680.]),
        ..Default::default()
    };
    eframe::run_native(
        "Omarchy Invaders",
        options,
        Box::new(move |_| Ok(Box::new(App::new(store)))),
    )
}
