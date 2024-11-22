import tensorflow as tf

try:
    import cupy as np
except ImportError:
    print("Cupy no encontrado, usando Numpy.")
    import numpy as np


def activacion_sigmoide(x):
    return 1 / (1 + np.exp(-x))


class Modelo:
    def __init__(self, w, b):
        self.w = w  # Pesos
        self.b = b  # Sesgos

    def predict(self, vec):
        vec = np.array(vec, dtype=np.float32)

        salida = activacion_sigmoide(np.dot(self.w, vec.T) + self.b)
        return salida


def crear_modelo_tensorflow():
    model = tf.keras.Sequential(
        [
            tf.keras.layers.Dense(32, input_shape=(3,), activation="relu"),
            # tf.keras.layers.Dense(128, activation="relu"),
            # tf.keras.layers.Dense(64, activation="relu"),  # Capa oculta con 16 neuronas
            # tf.keras.layers.Dense(32, activation="relu"),  # Capa oculta con 16 neuronas
            # tf.keras.layers.Dense(32, activation="relu"),  # Capa oculta con 16 neuronas
            # tf.keras.layers.Dense(16, activation="relu"),  # Capa oculta con 16 neuronas
            tf.keras.layers.Dense(16, activation="relu"),  # Capa oculta con 16 neuronas
            tf.keras.layers.Dense(4, activation="softmax"),
        ]
    )
    return model
