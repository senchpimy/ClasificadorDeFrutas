import tensorflow as tf

try:
    import cupy as np
except:
    print("Cupy no encontrado")
    import numpy as np


def activacion_sigmoide(x):
    return 1 / (1 + np.exp(-x))


def activacion_relu(x):
    l = lambda x: max(0, x)
    fun = np.vectorize(l)
    return fun(x)


class Capa:
    def __init__(self, activacion, pesos, sesgo) -> None:
        # self.activacion = None
        self.pesos = pesos
        self.sesgo = sesgo
        match activacion:
            case "relu":
                self.activacion = activacion_relu
            case "sigmoid":
                self.activacion = activacion_sigmoide
            case _:
                self.activacion = activacion

    def calc(self, w):
        z = np.dot(w, self.pesos)
        return self.activacion(z) + self.sesgo


class Modelo:
    def __init__(self, w, b, activaciones, loss):
        assert len(w) == len(activaciones)
        self.w = w
        self.b = b
        self.activaciones = activaciones
        self.capas = list()
        self.loss = loss
        for i, e in enumerate(w):
            self.capas.append(Capa(activaciones[i], e, b[i]))

    def create_capas(self):
        self.capas = list()
        for i, e in enumerate(self.w):
            self.capas.append(Capa(self.activaciones[i], e, self.b[i]))

    def verify(self):
        try:
            t = self.capas[0].activacion is None
        except:
            self.create_capas()

    def predict(self, vec):
        assert len(vec) == 3
        self.verify()
        calcular = vec
        for capa in self.capas:
            calcular = capa.calc(calcular)
        return calcular.numpy()


def crear_modelo_tensorflow():
    model = tf.keras.Sequential(
        [
            tf.keras.layers.Dense(256, input_shape=(3,), activation="relu"),
            tf.keras.layers.Dense(128, activation="relu"),
            tf.keras.layers.Dense(64, activation="relu"),  # Capa oculta con 16 neuronas
            tf.keras.layers.Dense(32, activation="relu"),  # Capa oculta con 16 neuronas
            tf.keras.layers.Dense(32, activation="relu"),  # Capa oculta con 16 neuronas
            tf.keras.layers.Dense(16, activation="relu"),  # Capa oculta con 16 neuronas
            tf.keras.layers.Dense(8, activation="relu"),
            tf.keras.layers.Dense(4, activation="sigmoid"),
        ]
    )
    return model
