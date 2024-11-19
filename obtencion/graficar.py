import os
import csv
import random
import matplotlib.pyplot as plt

def leer_valores_csv(directorio):
    datos_por_archivo = {}
    
    for archivo in os.listdir(directorio):
        if archivo.endswith('.csv'):
            ruta = os.path.join(directorio, archivo)
            r, g, b = [], [], []
            with open(ruta, newline='', encoding='utf-8') as csvfile:
                lector = csv.DictReader(csvfile)
                for fila in lector:
                    r.append(float(fila['R']))
                    g.append(float(fila['G']))
                    b.append(float(fila['B']))
            datos_por_archivo[archivo] = (r, g, b)
    
    return datos_por_archivo

def generar_color_aleatorio():
    return "#" + ''.join(random.choices('0123456789ABCDEF', k=6))

def graficar_valores_3d(datos_por_archivo):
    fig = plt.figure()
    ax = fig.add_subplot(111, projection='3d')
    
    for nombre_archivo, (r_values, g_values, b_values) in datos_por_archivo.items():
        color = generar_color_aleatorio()
        ax.scatter(r_values, g_values, b_values, color=color, label=nombre_archivo, s=20)
    
    ax.set_title("Gráfico 3D de Valores RGB")
    ax.set_xlabel("Rojo")
    ax.set_ylabel("Verde")
    ax.set_zlabel("Azul")
    ax.legend()
    plt.show()

def main():
    directorio = "."
    
    datos_por_archivo = leer_valores_csv(directorio)
    
    graficar_valores_3d(datos_por_archivo)

if __name__ == "__main__":
    main()

