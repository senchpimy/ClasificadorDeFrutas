import serial
import time

datos = []


def main():
    print("Preparese para leer..")
    time.sleep(2)
    ser = serial.Serial("/dev/ttyUSB0", 9600)

    while True:
        bs = ser.readline().decode("ascii")
        stri = bs[:-3]
        datos.append(stri)
        print(stri, f"###{len(datos)}###")


try:
    main()
except KeyboardInterrupt as _:
    file = input("Ingrese nombre del archivo: ")
    index = int(
        input("Ingrese indice de la seleccion (cebolla,manzana,limon,zanahoria): ")
    )
    stri_ind = ",".join(["1" if x == index - 1 else "0" for x in range(4)])
    print(stri_ind)
    content = ""
    for i in datos:
        content += i + "," + stri_ind + "\n"
    # content = "\n".join(datos)
    content = "R,G,B,cebolla,manzana,limon,zanahoria\n" + content
    with open(file + ".csv", "w") as f:
        f.write(content)
