use std::sync::{Arc, RwLock};
use std::thread;

#[derive(Debug, Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub error: Option<String>,
    pub alive: bool,
}

const MIN_ROJO: u16 = 100;
const MAX_ROJO: u16 = 230;

const MIN_GREEN: u16 = 75;
const MAX_GREEN: u16 = 150;

const MIN_BLUE: u16 = 110;
const MAX_BLUE: u16 = 250;

pub fn leer(rgb_rw: Arc<RwLock<RGB>>) {
    //let mut error = None;
    let mut alive = true;
    let port = serialport::new("/dev/ttyUSB0", 9_600)
        .open()
        .inspect_err(|f| {
            let err_str = format!("{}: Reinicie la aplicacion.", &f.description);
            println!("{}", &err_str);
            //error = Some(err_str.clone());
            match rgb_rw.write() {
                Ok(mut val) => {
                    if let Some(inner_val) = &mut val.error {
                        *inner_val = err_str;
                    }
                }
                Err(_) => {}
            }
            alive = false;
        });
    let mut serial_buf = String::new();
    let mut full_str = String::new();
    let mut listo = false;
    let mut port = port.unwrap();
    loop {
        let _res = port.read_to_string(&mut serial_buf);
        if serial_buf.len() > 0 {
            if serial_buf == "\r\n" {
                let mut split = full_str.split(",");
                if let Some(r) = split.next() {
                    if r.len() >= 3 {
                        dbg!(&full_str);
                        let r_u16: u16 = r.parse().unwrap();
                        let g_u16: u16 = split.next().unwrap().parse().unwrap();
                        let b_u16: u16 = split.next().unwrap().parse().unwrap();
                        let r = map(r_u16, MIN_ROJO, MAX_ROJO);
                        let g = map(g_u16, MIN_GREEN, MAX_GREEN);
                        let b = map(b_u16, MIN_BLUE, MAX_BLUE);
                        let rgb = RGB {
                            r,
                            g,
                            b,
                            error: None,
                            alive,
                        };
                        match rgb_rw.write() {
                            Ok(mut val) => *val = rgb,
                            Err(_) => {}
                        }
                    }
                }
                //dbg!(&full_str);
                full_str = String::new();
                listo = true;
            }
            if listo {
                full_str = format!("{serial_buf}{full_str}");
            }
        }
        serial_buf = String::new();
    }
}

fn map(value: u16, from_min: u16, from_max: u16) -> u8 {
    // Asegurarnos de que 'value' está dentro del rango [from_min, from_max]
    if value < from_min {
        return 0; // Si value es menor que from_min, devolvemos el valor mínimo
    }
    if value > from_max {
        return 255; // Si value es mayor que from_max, devolvemos el valor máximo
    }

    const TO_MIN: u8 = 0;
    const TO_MAX: u8 = 255;

    // Calcular el mapeo utilizando la fórmula
    let result = ((value as u32 - from_min as u32) * (TO_MAX as u32 - TO_MIN as u32))
        / (from_max as u32 - from_min as u32)
        + TO_MIN as u32;

    // Aseguramos que el resultado esté dentro del rango [0, 255]
    result as u8
}
