import tensorflow as tf
import pandas as pd
import numpy as np
import sys
from pathlib import Path

# Agregar el directorio superior al sys.path
parent_dir = Path(__file__).resolve().parent.parent
sys.path.append(str(parent_dir))
import utils

df1 = pd.read_csv("./obtencion/cebolla.csv")
df2 = pd.read_csv("./obtencion/limon.csv")
df3 = pd.read_csv("./obtencion/manzana.csv")
df4 = pd.read_csv("./obtencion/zanahoria.csv")
df = pd.concat([df1, df2, df3, df4], axis=0)

df = df.sample(frac=1).reset_index(drop=True)
X = df[["R", "G", "B"]].to_numpy()
y = df[["cebolla", "manzana", "zanahoria", "limon"]].to_numpy()

model = utils.crear_modelo_tensorflow()

model.compile(
    optimizer=tf.keras.optimizers.SGD(learning_rate=0.001), loss="binary_crossentropy"
)
# loss='mean_squared_error')

history = model.fit(X, y, epochs=5_000, verbose=2, batch_size=len(X) // 10)

weights, biases = list(), list()
activaciones = list()

for e in model.layers:
    w, b = e.get_weights()
    weights.append(w)
    biases.append(b)
    activaciones.append(e.activation)


predicciones = model.predict(X)
for i in range(len(y)):
    prediccion_redondeada = np.round(predicciones[i], decimals=2)
    l = list()
    for pre in prediccion_redondeada:
        l.append(int(pre))
    print(f"Predicción: {prediccion_redondeada}, Real: {y[i]}")

model.save_weights("pesos_tensorflow_v2.weights.h5")
model.save("modelo_tensorflow.keras")
