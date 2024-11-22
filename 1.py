import pandas as pd
import numpy as np


def activacion_sigmoide(x):
    return 1 / (1 + np.exp(-x))


def escalon(n):
    for x in range(n.shape[0]):
        for y in range(n.shape[1]):
            n[x][y] = activacion_sigmoide(n[x][y])
    return n


df = pd.read_csv("./RGB.csv")
p = df[["R", "G", "B"]].to_numpy()
a = df[["a1", "a2"]].to_numpy()
w = np.random.rand(2, 3) - 1
b = np.random.rand(1, 2) - 1
e = np.zeros((1, 2))
ep = np.array([[0.33]])
for epocas in range(2440):
    for q in range(15):
        valor = escalon((np.dot(w, p[q].T) + b))
        if (valor != a[q]).all:
            e = a[q].T - (escalon(np.dot(w, p[q].T) + b))
            eP = (e[:, 0] + e[:, 1]) / 2
            w = w + (eP * p[q] * ep)
            b = b + eP
print(w)
print(b)
print(e)
for q in range(len(p)):
    salida = escalon(np.dot(w, p[q].T) + b)
    print(salida)
