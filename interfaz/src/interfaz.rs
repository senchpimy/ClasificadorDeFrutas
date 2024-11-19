use eframe::egui;
use egui::{Color32, TextStyle, Ui, Visuals, WidgetText};
use egui_dock::{DockArea, DockState, NodeIndex, SurfaceIndex, TabViewer};
use egui_extras::{Size, StripBuilder};
use full_palette::GREY;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::leer;
use crate::serial;

pub struct TabViewerI {
    title: String,
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
        };
        let grafica = TabViewerI {
            title: String::from("Grafica"),
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

impl TabViewer for App {
    type Tab = TabViewerI;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title.clone().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab.title.as_str() {
            "RGB Monitor" => rgb_monitor_ui(Arc::clone(&self.rgb), ui),
            "Grafica" => grafica_ui(&mut self.grafica, ui),
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
    if let Some(val) = &rgb.error {
        ui.colored_label(Color32::RED, val);
    } else {
        dbg!(&rgb);
    }

    ui.label(str);
    StripBuilder::new(ui)
        .size(Size::exact(50.0))
        .vertical(|mut strip| {
            strip.cell(|ui| {
                let color = Color32::from_rgb(rgb.r, rgb.g, rgb.b);
                ui.painter()
                    .rect_filled(ui.available_rect_before_wrap(), 0.0, color);
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
    data: Vec<leer::CsvData>,
}

impl ThreeD {
    fn new() -> Self {
        let dir = "/home/plof/Documents/5to-semestre-fes/analisisDeAlgo/inteligencia/obtencion/"; // Cambia a tu directorio deseado
        let data = leer::read_csv_files_from_directory(dir);
        Self {
            chart_pitch: 0.3,
            chart_yaw: 0.9,
            chart_scale: 0.9,
            chart_pitch_vel: 0.0,
            data,
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

    let x_axis = (0.0..150.0).step(10.);
    let y_axis = (0.0..80.0).step(10.);
    let z_axis = (0.0..150.0).step(10.);

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("3D Plot Test"), (FontFamily::SansSerif, 20))
        .build_cartesian_3d(x_axis, y_axis, z_axis)
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

    let colores = [BLUE, GREEN, RED, YELLOW];
    let mut index = 0;
    for i in &sel.data {
        let points: Vec<(f64, f64, f64)> = i.rows.iter().map(|a| (a.R, a.G, a.B)).collect();
        let color = colores[index].clone();
        chart
            .draw_series(PointSeries::<_, _, Circle<_, _>, _>::new(
                points,
                4,
                &colores[index],
            ))
            .unwrap()
            .label(&i.filename)
            .legend(move |(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], color));
        index += 1;
    }

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .draw()
        .unwrap();

    root.present().unwrap();
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut dock_state = std::mem::replace(&mut self.dock_state, DockState::new(vec![]));
            DockArea::new(&mut dock_state).show_inside(
                ui, //&mut App::new_tab(String::from("AA"), Arc::clone(&self.rgb)),
                self,
            );
            self.dock_state = dock_state;
        });
        ctx.request_repaint();
    }
}
