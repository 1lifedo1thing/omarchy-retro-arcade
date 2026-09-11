use omarchy_chess::{
    storage::{self, SessionLock},
    ui::ChessApp,
};
fn main() -> eframe::Result<()> {
    if std::env::args().any(|a| a == "--version") {
        println!("Omarchy Chess {} (Rust)", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let dir = storage::state_dir();
    let _lock = match SessionLock::acquire(&dir) {
        Ok(lock) => lock,
        Err(error) => {
            eprintln!("{error}");
            rfd::MessageDialog::new()
                .set_title("Omarchy Chess")
                .set_description(&error)
                .show();
            return Ok(());
        }
    };
    let mut size = [1060., 780.];
    if cfg!(feature = "screenshot")
        && std::env::var("OMARCHY_CHESS_PREVIEW_SIZE").as_deref() == Ok("compact")
    {
        size = [740., 560.];
    }
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size(size)
            .with_min_inner_size([740., 560.])
            .with_app_id("omarchy-chess"),
        ..Default::default()
    };
    eframe::run_native(
        "Omarchy Chess",
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(ChessApp::new(dir)))
        }),
    )
}
