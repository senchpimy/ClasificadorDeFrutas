use std::sync::{Arc, RwLock};
use std::thread;
mod interfaz;
mod leer;
mod python;
mod serial;

fn main() -> eframe::Result {
    let mut RGB = serial::RGB {
        r: 0,
        g: 0,
        b: 0,
        r_raw: 0.,
        g_raw: 0.,
        b_raw: 0.,
        error: None,
        alive: false,
    };
    let data = Arc::new(RwLock::new(RGB));
    let data_clone = Arc::clone(&data);
    let reading = thread::spawn(move || {
        serial::leer(data_clone);
    });

    let options = eframe::NativeOptions {
        ..Default::default()
    };
    let gui = interfaz::App::new(Arc::clone(&data));
    eframe::run_native(
        "tablet_utils",
        options,
        Box::new(|cc| {
            let context = &cc.egui_ctx;

            context.tessellation_options_mut(|tess_options| {
                tess_options.feathering = false;
            });

            Ok(Box::<interfaz::App>::new(gui))
        }),
    )
}
