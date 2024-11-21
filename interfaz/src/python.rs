use crate::serial;
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::ffi::CString;
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Prediccion {
    pub cebolla: bool,
    pub manzana: bool,
    pub zanahoria: bool,
    pub limon: bool,
}

impl Prediccion {
    pub fn new() -> Self {
        Prediccion {
            cebolla: false,
            manzana: false,
            zanahoria: false,
            limon: false,
        }
    }
}

pub fn thread(rgb: Arc<RwLock<serial::RGB>>, datos: Arc<RwLock<Prediccion>>) {
    let path =
        Path::new("/home/plof/Documents/5to-semestre-fes/analisisDeAlgo/inteligencia/prediccion/");
    let py_app =
        CString::new(fs::read_to_string(path.join("conneccion_tensorflow.py")).unwrap()).unwrap();
    Python::with_gil(|py| {
        let module = PyModule::from_code(
            py,
            py_app.as_c_str(),
            c_str!("conneccion_tensorflow.py"),
            c_str!("conneccion_tensorflow"),
        )
        .unwrap();
        let preddict = module.getattr("A").unwrap();
        let rgb_n = rgb.read();
        match rgb_n {
            Ok(rgb) => loop {
                if !rgb.alive {
                    let elements: Vec<f64> = vec![rgb.r_raw, rgb.g_raw, rgb.b_raw];
                    dbg!(&elements);
                    let list = PyList::new(py, elements).unwrap();
                    let res = preddict.call_method1("predecir", (list, 0));
                    match res {
                        Ok(val) => {
                            let num = val.extract::<Vec<bool>>();
                            //let mut pred_w = datos.write().unwrap();
                            //dbg!(pred_w);
                            match num {
                                Ok(val) => {
                                    //*pred_w = Prediccion {
                                    //    cebolla: val[0],
                                    //    manzana: val[1],
                                    //    zanahoria: val[2],
                                    //    limon: val[3],
                                    //}
                                }
                                Err(_) => {}
                            }
                        }
                        Err(_) => {}
                    }
                } else {
                    println!("### No esta viva la coneccion ###");
                    break;
                }
            },
            Err(_) => {}
        }
    })
}
