use crate::serial;
use plotters::data;
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

pub struct TensorFlowPredictor {
    module: PyObject,
}
impl TensorFlowPredictor {
    pub fn new() -> PyResult<Self> {
        let path = Path::new(
            "/home/plof/Documents/5to-semestre-fes/analisisDeAlgo/inteligencia/prediccion/",
        );
        let py_app =
            CString::new(fs::read_to_string(path.join("conneccion_tensorflow.py")).unwrap())
                .unwrap();
        Python::with_gil(|py| {
            let module = PyModule::from_code(
                py,
                py_app.as_c_str(),
                c_str!("conneccion_tensorflow.py"),
                c_str!("conneccion_tensorflow"),
            )
            .unwrap();
            let preddict = module.getattr("A").unwrap();

            let elements: Vec<i32> = vec![0, 0, 0];
            //dbg!(&elements);
            let list = PyList::new(py, elements).unwrap();
            let res = preddict.call_method1("predecir", (list, 0));
            Ok(TensorFlowPredictor {
                module: module.into(),
            })
        })
    }
    pub fn predecir(&self, data: Arc<RwLock<serial::RGB>>) -> Option<Prediccion> {
        Python::with_gil(|py| {
            let data = data.read().unwrap();
            if data.alive {
                let module = self.module.clone_ref(py);
                let predictor = module.getattr(py, "A").unwrap();
                let elements: Vec<f64> = vec![data.r_raw, data.g_raw, data.b_raw];
                let list = PyList::new(py, elements).unwrap();
                let res = predictor.call_method1(py, "predecir", (list, 0));
                match res {
                    Ok(val) => {
                        let num = val.extract::<Vec<bool>>(py);
                        match num {
                            Ok(val) => {
                                return Some(Prediccion {
                                    cebolla: val[0],
                                    manzana: val[1],
                                    zanahoria: val[2],
                                    limon: val[3],
                                });
                            }
                            Err(e) => {
                                eprintln!("No se pudo predecir {}", e);
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("No exite el metodo");
                    }
                }
            }
            None
        })
    }
}
