import os

os.environ["TF_CPP_MIN_LOG_LEVEL"] = "3"
import tensorflow as tf

try:
    import cupy as np
except:
    import numpy as np
import pandas as pd

modelo_keras = tf.keras.models.load_model(
    "/home/plof/Documents/5to-semestre-fes/analisisDeAlgo/inteligencia/modelo_tensorflowv2.keras"
)


class A:
    def predecir(arr: list[int], dummy):
        arr = np.array(arr).reshape(1, -1)
        salida = modelo_keras.predict(arr, verbose=0)
        prediccion_redondeada = np.round(salida[0], decimals=0)
        # print(prediccion, a[i])
        print(salida)
        return salida[0]


# prediccion = predecir([23, 54, 55])
# print(prediccion)
