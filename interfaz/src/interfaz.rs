use eframe::egui;
use egui::{Color32, TextStyle, Ui, Visuals, WidgetText};
use egui_dock::{DockArea, DockState, NodeIndex, SurfaceIndex, TabViewer};
use egui_extras::{Size, StripBuilder};
use full_palette::GREY;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::serial;

pub struct TabViewerI {
    title: String,
    rgb: Option<Arc<RwLock<serial::RGB>>>,
    grafica: Option<ThreeD>,
}

pub struct App {
    rgb: Arc<RwLock<serial::RGB>>,
    dock_state: DockState<TabViewerI>, // Para manejar las pestañas
    grafica: ThreeD,
}

impl App {
    pub fn new(rgb: Arc<RwLock<serial::RGB>>) -> Self {
        // Configuración inicial de las pestañas
        let color = TabViewerI {
            title: String::from("RGB Monitor"),
            rgb: Some(Arc::clone(&rgb)),
            grafica: None,
        };
        let grafica = TabViewerI {
            title: String::from("Grafica"),
            rgb: None,
            grafica: Some(ThreeD::new()),
        };
        let mut dock_state = DockState::new(vec![color]);
        dock_state
            .main_surface_mut()
            .split_left(NodeIndex::root(), 0.5, vec![grafica]);

        let grafica = ThreeD::new();
        Self {
            rgb,
            dock_state,
            grafica,
        }
    }
}

// Implementación de TabViewer para manejar el contenido de cada pestaña
impl TabViewer for App {
    type Tab = TabViewerI;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title.clone().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab.title.as_str() {
            "RGB Monitor" => rgb_monitor_ui(Arc::clone(&self.rgb), ui),
            "Grafica" => grafica_ui(&mut self.grafica, ui),
            //"Logs" => logs_ui(tab, ui),
            //_ => ui.label(format!("Unknown tab: {}", tab)),
            _ => {}
        };
    }

    //fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
    //    false // No permitir cerrar pestañas en este caso
    //}
}

fn rgb_monitor_ui(rgb: Arc<RwLock<serial::RGB>>, ui: &mut Ui) {
    let rgb = rgb.read().unwrap();
    let str = format!("R: {} G: {} B: {}", rgb.r, rgb.g, rgb.b);

    StripBuilder::new(ui)
        .size(Size::exact(50.0))
        .vertical(|mut strip| {
            strip.cell(|ui| {
                ui.painter()
                    .rect_filled(ui.available_rect_before_wrap(), 0.0, Color32::BLUE);
                ui.label(str);
            });
        });
}

use egui_plotter::EguiBackend;
use plotters::prelude::*;
const MOVE_SCALE: f32 = 0.01;
const SCROLL_SCALE: f32 = 0.001;
#[derive(Clone)]
struct ThreeD {
    chart_pitch: f32,
    chart_yaw: f32,
    chart_scale: f32,
    chart_pitch_vel: f32,
    chart_yaw_vel: f32,
}

impl ThreeD {
    fn new() -> Self {
        Self {
            chart_pitch: 0.3,
            chart_yaw: 0.9,
            chart_scale: 0.9,
            chart_pitch_vel: 0.0,
            chart_yaw_vel: 0.0,
        }
    }
}
fn grafica_ui(sel: &mut ThreeD, ui: &mut Ui) {
    let mut chart_yaw_vel = 0.0;
    let (pitch_delta, yaw_delta, scale_delta) = ui.input(|input| {
        let pointer = &input.pointer;
        let delta = pointer.delta();

        let (pitch_delta, yaw_delta) = match pointer.primary_down() {
            true => (delta.y * MOVE_SCALE, -delta.x * MOVE_SCALE),
            false => (sel.chart_pitch_vel, chart_yaw_vel),
        };

        let scale_delta = input.raw_scroll_delta.y * SCROLL_SCALE;

        (pitch_delta, yaw_delta, scale_delta)
    });

    sel.chart_pitch_vel = pitch_delta;
    chart_yaw_vel = yaw_delta;

    sel.chart_pitch += sel.chart_pitch_vel;
    sel.chart_yaw += chart_yaw_vel;
    sel.chart_scale += scale_delta;

    // Next plot everything
    let root = EguiBackend::new(ui).into_drawing_area();

    root.fill(&GREY).unwrap();

    let x_axis = (-3.0..3.0).step(0.1);
    let z_axis = (-3.0..3.0).step(0.1);

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("3D Plot Test"), (FontFamily::SansSerif, 20))
        .build_cartesian_3d(x_axis, -3.0..3.0, z_axis)
        .unwrap();

    chart.with_projection(|mut pb| {
        pb.yaw = sel.chart_yaw as f64;
        pb.pitch = sel.chart_pitch as f64;
        pb.scale = sel.chart_scale as f64;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()
        .unwrap();

    let points: Vec<(f64, f64, f64)> = vec![
        (2.0, 2.0, 2.),
        (3.0, 3.0, 3.),
        (4.0, 4.0, 4.),
        (8.0, 8.0, 8.),
    ];
    chart
        .draw_series(PointSeries::<_, _, Circle<_, _>, _>::new(points, 4, &BLUE))
        .unwrap()
        .label("Surface");
    //.legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .draw()
        .unwrap();

    root.present().unwrap();

    // Limit framerate to 100fps
    std::thread::sleep(Duration::from_millis(10));
}
//
//fn logs_ui(sel: &mut App, ui: &mut Ui) {
//    ui.label("Logs tab content...");
//}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut dock_state = std::mem::replace(&mut self.dock_state, DockState::new(vec![]));
            DockArea::new(&mut dock_state)
                //.style(Style::from_egui(ui.style()))
                .show_inside(
                    ui, //&mut App::new_tab(String::from("AA"), Arc::clone(&self.rgb)),
                    self,
                );
            self.dock_state = dock_state;
        });
        ctx.request_repaint();
    }
}
