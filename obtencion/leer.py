import serial
import time

datos = []
def main():
    print("Preparese para leer..")
    time.sleep(2)
    ser = serial.Serial("/dev/ttyUSB0", 9600)
       
    while True:
        bs = ser.readline().decode('ascii')
        stri = bs[:-3]
        datos.append(stri)
        print(stri, f"###{len(datos)}###")

try:
    main()
except KeyboardInterrupt as _:
    file = input("Ingrese nombre del archivo: ")
    content = "\n".join(datos)
    content = "R,G,B\n"+content
    with open(file+".csv", "w") as f:
        f.write(content)
