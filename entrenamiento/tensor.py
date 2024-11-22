import tensorflow as tf
import pandas as pd
import numpy as np
import sys
from pathlib import Path

# Agregar el directorio superior al sys.path
parent_dir = Path(__file__).resolve().parent.parent
sys.path.append(str(parent_dir))
import utils

df1 = pd.read_csv("../obtencion/cebolla.csv")
df2 = pd.read_csv("../obtencion/limon.csv")
df3 = pd.read_csv("../obtencion/manzana.csv")
df4 = pd.read_csv("../obtencion/zanahoria.csv")
df = pd.concat([df1, df2, df3, df4], axis=0)
df = df.dropna()
df = df.sample(frac=1).reset_index(drop=True)

X = np.array(df[["R", "G", "B"]], dtype=np.float32)
y = np.argmax(df[["cebolla", "manzana", "zanahoria", "limon"]].to_numpy(), axis=1)

y_one_hot = tf.keras.utils.to_categorical(y, num_classes=4)

model = utils.crear_modelo_tensorflow()

model.compile(optimizer="adam", loss="categorical_crossentropy", metrics=["accuracy"])
## loss='mean_squared_error')
#
history = model.fit(
    X, y_one_hot, epochs=1_000, verbose=2, batch_size=32, validation_split=0.2
)

weights, biases = list(), list()
activaciones = list()

predicciones = model.predict(X)
for i in range(len(y)):
    prediccion_redondeada = np.round(predicciones[i], decimals=2)
    l = list()
    for pre in prediccion_redondeada:
        l.append(int(pre))
    print(f"Predicción: {prediccion_redondeada}, Real: {y[i]}")

model.save("modelo_tensorflowv2.keras")
