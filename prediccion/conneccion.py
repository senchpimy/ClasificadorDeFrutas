try:
    import cupy as np
except:
    import numpy as np
import pickle
import pandas as pd
import time
import utils
from utils import Modelo

modelo = None


df1 = pd.read_csv("./cebolla.csv")
df2 = pd.read_csv("./limones.csv")
df3 = pd.read_csv("./manzana.csv")
df4 = pd.read_csv("./zanahoria.csv")
df = pd.concat([df1, df2, df3, df4], axis=0)
df["R"]=df["R"].map(utils.rgb_to_float)
df["G"]=df["G"].map(utils.rgb_to_float)
df["B"]=df["B"].map(utils.rgb_to_float)
print(df)
p = df[["R", "G", "B"]].to_numpy()
a = df[["a1", "a2", "a3"]].to_numpy()
print(a)

with open("modelo_tensor.pkl", "rb") as f:
    modelo = pickle.load(f) 


print(len(a))
for i in range(len(a)):
    salida = modelo.predict(a[i])
    print(salida,a[i])
