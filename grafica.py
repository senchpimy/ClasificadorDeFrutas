import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import pandas as pd

np.random.seed(19680801)


def randrage(n, vmin, vmax):
    return (vmax - vmin) * np.random.rand(n) + vmin


# Crear una malla de puntos
x = np.linspace(-600, 700, 100)
y = np.linspace(-600, 700, 100)
x, y = np.meshgrid(x, y)

# leyendo los puntos del archivo
df = pd.read_csv("./RGB.csv")
# el archivo tienen 150 registros por captura de rgb
x_1 = df["R"][0:150]
y_1 = df["G"][0:150]
z_1 = df["B"][0:150]
# puntos dos
x_2 = df["R"][151:301]
y_2 = df["G"][151:301]
z_2 = df["B"][151:301]
# puntos tres
x_3 = df["R"][302:451]
y_3 = df["G"][302:451]
z_3 = df["B"][302:451]

# Definir la funcion el plano, por ejemplo: z= ax + by + cz
z = 13 * x - 13 * y + 60  # Cambiar los coeficientres segun sea necesario
z2 = (2 * (x - 6000) + 22 * (y)) - 500

# Crear la figura y el eje 3D
fig = plt.figure()
ax = fig.add_subplot(111, projection="3d")

# Dibujar el plano
ax.plot_surface(x, y, z, alpha=0.5, rstride=100, cstride=100, color="cyan")
ax.plot_surface(x, y, z2, alpha=0.5, rstride=100, cstride=100, color="pink")
ax.scatter(x_1, y_1, z_1, marker="o", color="red")
ax.scatter(x_2, y_2, z_2, marker="o", color="red")
ax.scatter(x_3, y_3, z_3, marker="o", color="red")

# Configurar etiqueta
ax.set_xlabel("Eje X")
ax.set_ylabel("Eje y")
ax.set_zlabel("Eje z")

# mosrar el grafico
plt.show()
