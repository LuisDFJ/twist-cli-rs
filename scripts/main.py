import time
import subprocess
from src.controller import Controller

from os import path
VPORT = path.join( path.dirname(__file__), 'vport.sh' )
if __name__ == "__main__":
    with subprocess.Popen( ['sh', VPORT], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True ) as process:
            time.sleep(0.5)
            try:
                with Controller( "/home/luisdfj/ttyUSB0", 115200 ):
                    for line in process.stdout:
                        print( line )
            except:
                time.sleep(0.1)
                process.kill()
                for line in process.stdout:
                    print( line )
                print("")
#            while True:
#                try: time.sleep(0.5)
#                except: break

