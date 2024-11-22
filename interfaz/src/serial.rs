use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub r_raw: f64,
    pub g_raw: f64,
    pub b_raw: f64,
    pub error: Option<String>,
    pub alive: bool,
}

pub const MIN_ROJO: u16 = 30;
pub const MAX_ROJO: u16 = 200;

pub const MIN_GREEN: u16 = 30;
pub const MAX_GREEN: u16 = 200;

pub const MIN_BLUE: u16 = 30;
pub const MAX_BLUE: u16 = 200;

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
                    if r.len() >= 2 {
                        //dbg!(&full_str);
                        let r_u16: u16 = r.parse().unwrap();
                        let g_u16: u16 = split.next().unwrap().parse().unwrap();
                        let b_u16: u16 = split.next().unwrap().parse().unwrap();
                        let r = map(r_u16, MIN_ROJO, MAX_ROJO);
                        let g = map(g_u16, MIN_GREEN, MAX_GREEN);
                        let b = map(b_u16, MIN_BLUE, MAX_BLUE);
                        println!("r:{} g:{} b:{}", r, g, b);
                        let rgb = RGB {
                            r,
                            g,
                            b,
                            r_raw: r_u16 as f64,
                            g_raw: g_u16 as f64,
                            b_raw: b_u16 as f64,
                            error: None,
                            alive,
                        };
                        match rgb_rw.write() {
                            Ok(mut val) => *val = rgb,
                            Err(_) => {
                                println!("Error cambiando el valor RGB de manera global")
                            }
                        }
                    } else {
                        println!("Error: insuficientes ',', el output no es el correcto");
                    }
                }
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
    if value < from_min {
        //println!("El valor obtenido fue menor que el minimo predeterminado");
        return 0;
    }
    if value > from_max {
        //println!("El valor obtenido fue mayor que el maximo predeterminado");
        return 255;
    }

    const TO_MIN: u8 = 0;
    const TO_MAX: u8 = 255;

    let result = ((value as u32 - from_min as u32) * (TO_MAX as u32 - TO_MIN as u32))
        / (from_max as u32 - from_min as u32)
        + TO_MIN as u32;

    result as u8
}
