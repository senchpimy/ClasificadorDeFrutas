try:
    import cupy as np
except:
    print("Cupy no encontrado")
    import numpy as np

def rgb_to_float(num:int)->float:
    return num / 1024

def float_to_rgb(num:float)->int:
    return int(num*1024)

def activacion_sigmoide(x):
    return 1 / (1 + np.exp(-x))

def activacion_relu(x):
    l = lambda x: max(0,x)
    fun = np.vectorize(l)
    return fun(x)

class Capa:
    def __init__(self, activacion, pesos, sesgo) -> None:
        #self.activacion = None
        self.pesos = pesos
        self.sesgo = sesgo
        match activacion:
            case "relu":
                self.activacion = activacion_relu
            case "sigmoid":
                self.activacion = activacion_sigmoide
            case _:
                pass
                #self.activacion = activacion

    def calc(self, w):
        z = np.dot(w, self.pesos)
        return self.activacion(z)+self.sesgo

class Modelo:
    def __init__(self, w, b, activaciones):
        assert len(w) == len(activaciones)
        self.w = w
        self.b = b
        self.activaciones = activaciones
        self.capas = list()
        for i,e in enumerate(w):
            self.capas.append(Capa(activaciones[i],e, b[i]))

    def create_capas(self):
        self.capas = list()
        for i,e in enumerate(self.w):
            self.capas.append(Capa(self.activaciones[i],e, self.b[i]))
        
    def verify(self):
        if self.capas[0].activacion is None:
            self.create_capas()
    
    def predict(self, vec):
        assert len(vec)==3
        self.verify()
        calcular = vec
        for capa in self.capas:
            calcular = capa.calc(calcular)
        return calcular.numpy()
