import pandas as pd
import pickle
import sys
from pathlib import Path

# Agregar el directorio superior al sys.path
parent_dir = Path(__file__).resolve().parent.parent
sys.path.append(str(parent_dir))

try:
    import cupy as np
except ImportError:
    print("Cupy no encontrado, usando Numpy.")
    import numpy as np

from utils import Modelo
import utils

try:
    import cupy as np
except ImportError:
    import numpy as np
import pickle

modelo = None


with open(
    "/home/plof/Documents/5to-semestre-fes/analisisDeAlgo/inteligencia/modelo.pkl", "rb"
) as f:
    modelo = pickle.load(f)


class A:
    def predecir(arr: list[int], dummy):
        res = modelo.predict(arr)
        res = list(res[0])
        res.append(0)
        print(res)
        return res
