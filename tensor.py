import tensorflow as tf
import pandas as pd
import numpy as np
import pickle
import utils  # Asegúrate de que `utils` esté definido y disponible.

# Cargar y procesar datos
df1 = pd.read_csv("./cebolla.csv")
df2 = pd.read_csv("./limones.csv")
df3 = pd.read_csv("./manzana.csv")
df4 = pd.read_csv("./zanahoria.csv")
df = pd.concat([df1, df2, df3, df4], axis=0)

# Mezclar y preparar datos
df = df.sample(frac=1).reset_index(drop=True)
X = df[["R", "G", "B"]].to_numpy()
y = df[["a1", "a2", "a3"]].to_numpy()

# Crear el modelo de TensorFlow
model = tf.keras.Sequential([
  tf.keras.layers.Dense(128, input_shape=(3,), activation='relu'),  # Capa oculta con 64 neuronas
    tf.keras.layers.Dense(64, activation='relu'),                    # Capa oculta con 32 neuronas
    tf.keras.layers.Dense(32, activation='relu'),                    # Capa oculta con 16 neuronas
    tf.keras.layers.Dense(16, activation='relu'),                    # Capa oculta con 16 neuronas
    tf.keras.layers.Dense(16, activation='relu'),                    # Capa oculta con 16 neuronas
    tf.keras.layers.Dense(3, activation='sigmoid') 
])

# Compilar el modelo
model.compile(optimizer=tf.keras.optimizers.SGD(learning_rate=0.001),
              loss='mean_squared_error')

# Entrenar el modelo
history = model.fit(X, y, epochs=4000, verbose=2, batch_size=len(X) // 10)

weights, biases = list(), list()
activaciones = list()

for e in model.layers:
    w, b = e.get_weights()
    weights.append(w)
    biases.append(b)
    activaciones.append(e.activation)

# Guardar el modelo como un archivo .pkl

modelo = utils.Modelo(weights, biases, activaciones)
with open("modelo_tensor.pkl", "wb") as f:
    pickle.dump(modelo, f)

# Probar el modelo entrenado
predicciones = model.predict(X)
for i in range(len(y)):
    prediccion_redondeada = np.round(predicciones[i], decimals=2)
    l = list()
    for pre in prediccion_redondeada:
        l.append(int(pre))
    print(f"Predicción: {prediccion_redondeada}, Real: {y[i]}")
