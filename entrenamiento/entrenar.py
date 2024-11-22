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


class Red:
    pass


# Cargar y combinar los datos
df1 = pd.read_csv("../obtencion/cebolla.csv")
df2 = pd.read_csv("../obtencion/limon.csv")
df3 = pd.read_csv("../obtencion/manzana.csv")
df4 = pd.read_csv("../obtencion/zanahoria.csv")
df = pd.concat([df1, df2, df3, df4], axis=0)
df = df.dropna()
df = df.sample(frac=1).reset_index(drop=True)

# Normalizar los datos
X = np.array(df[["R", "G", "B"]], dtype=np.float32)
X = X / 255.0  # Escalar entre 0 y 1
y = np.argmax(df[["cebolla", "manzana", "zanahoria"]].to_numpy(), axis=1)
print(y, "AAAAAAAAAAa")

# Inicializar parámetros
w = np.random.randn(3, 3) * np.sqrt(1 / 3)
b = np.random.rand(1, 3)
e = np.zeros((1, 3))
ep_val = 0.01  # Reducir la tasa de aprendizaje
ep = np.array([[ep_val]])

print(w)
print(b)
print(e)
print(ep)
print("------------------------------------")

# Entrenamiento
for epocas in range(5_000):
    if epocas % 300 == 0:
        errors = []

    for q in range(len(X)):
        mul = np.dot(w, X[q].T)
        valor = utils.activacion_sigmoide(mul + b)
        e = X[q] - valor  # Error entre la predicción y el dato real
        eP = np.mean(e)  # Cálculo del error promedio por muestra
        if epocas % 300 == 0:
            errors.append(eP)

        # Actualización de los pesos y el sesgo con gradiente
        w += ep * np.outer(e, X[q])  # Ajustar pesos con el error
        b += ep_val * e  # Ajustar sesgo con el error

    if epocas % 300 == 0:
        error_prom = np.array(errors)
        print(
            f"En la iteración {epocas} se tuvo un error promedio de {np.mean(error_prom)}"
        )

print("------------------------------------")
print(w)
print(b)
print(e)

print("-- -- -- -- -- -- -- -- -- -- -- --")
print("Guardado como modelo.pkl")


for i in range(10):
    salida = utils.activacion_sigmoide(np.dot(w, X[i].T) + b)
    print(salida, y[i])

# Guardar el modelo en un archivo pickle
modelo = Modelo(w, b)
with open("modelo.pkl", "wb") as f:
    pickle.dump(modelo, f)
