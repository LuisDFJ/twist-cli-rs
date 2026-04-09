import time

from src.controller import Controller

if __name__ == "__main__":
    with Controller( "/home/luisdfj/ttyUSB0", 115200 ):
        while True:
            try: time.sleep(0.5)
            except: break

