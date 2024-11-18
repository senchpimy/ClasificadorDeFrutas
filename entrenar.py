import pandas as pd
import pickle
import utils

try:
    import cupy as np
except:
    print("Cupy no encontrado")
    import numpy as np

class Red:
    pass

# df = pd.read_csv("./RGB.csv")
df1 = pd.read_csv("./cebolla.csv")
df2 = pd.read_csv("./limones.csv")
df3 = pd.read_csv("./manzana.csv")
df4 = pd.read_csv("./zanahoria.csv")
df = pd.concat([df1, df2, df3, df4], axis=0)
df["R"]=df["R"].map(utils.rgb_to_float)
df["G"]=df["G"].map(utils.rgb_to_float)
df["B"]=df["B"].map(utils.rgb_to_float)
print(df)
print("-----------------------------------------")
print("Combinando datos")
df = df.sample(frac=1).reset_index(drop=True)
print(df)
print("-----------------------------------------")
p = df[["R", "G", "B"]].to_numpy()
a = df[["a1", "a2", "a3"]].to_numpy()
w =  np.random.randn(3, 3) * np.sqrt(1 / 3)
b = np.random.rand(1, 3)
e = np.zeros((1, 3))
ep_val = 0.33
ep = np.array([[ep_val]])

print(w)
print(b)
print(e)
print(ep)
print("------------------------------------")

for epocas in range(100_000):
    #df = df.sample(frac=1).reset_index(drop=True)
    #a = df[["a1", "a2", "a3"]].to_numpy()
    if epocas %300 ==0 :
        errors = []
    for q in range(len(a)):
        mul = np.dot(w, p[q].T)
        valor = utils.activacion_sigmoide(mul + b)
        e = a[q].T - valor
        eP = np.mean(e)
        #print(a[q], valor, e, eP)
        if epocas %300 ==0 :
            errors.append(eP)
        #print(eP)
        #w += ep * eP * p[q] * np.outer(p[q], np.ones(3))
        w += ep * np.outer(e, p[q])
        #w += ep * eP * p[q]
        b += e * eP
    if epocas %300 ==0 :
        error_prom = np.array(errors)
        print(f"En la iteracion {epocas} se tuvo un error promedio de {np.mean(error_prom)}")
        #ep = np.array([[ep_val-0.02]])
print("------------------------------------")
print(w)
print(b)
print(e)

print("-- -- -- -- -- -- -- -- -- -- -- --")
print("Guardado como modelo.pkl")


class Modelo:
    def __init__(self, w, b) -> None:
        self.w = w
        self.b = b

for i in range(10):
    salida = utils.activacion_sigmoide(np.dot(w, p[i].T) + b)
    print(salida,a[i])

modelo = Modelo(w, b)
with open("modelo.pkl", "wb") as f:
    pickle.dump(modelo, f)
