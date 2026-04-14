import serial
import threading
import queue
import time

from src.state import State
from src.cmd import cmd as cmd_parser

class Controller:
    def __init__(self, port : str, baudrate : int = 115200) -> None:
        self.state = State()
        self.port = port
        self.baudrate = baudrate
        self.run = False
        self.ser = None
        self.t_read = None
        self.t_write = None
        self.queue = queue.Queue()
    def __enter__( self ):
        self.run = True
        self.ser = serial.Serial( self.port, self.baudrate, timeout=0.001 )
        self.t_control = threading.Thread(
            target=self._control_task,
            daemon=True
        ).start()
        self.t_read = threading.Thread(
            target=self._read_task,
            daemon=True
        ).start()
        self.t_write = threading.Thread(
            target=self._write_task,
            daemon=True
        ).start()
    def __exit__( self, *_ ):
        self.run = False
        if isinstance( self.ser, serial.Serial ):
            self.ser.close()
        if isinstance( self.t_write, threading.Thread):
            self.t_write.join()
        if isinstance( self.t_read, threading.Thread):
            self.t_read.join()
        if isinstance( self.t_control, threading.Thread):
            self.t_control.join()
    @classmethod
    def _parse( cls, buf : bytes ) -> tuple[ bytes, bytes ]:
        i = buf.find(b'\r\n')
        cmd = b''
        if not i < 0:
            cmd = buf[:i]
            buf = buf[i+2:]
        return cmd, buf
    def _control_task( self ):
        h = 0.001
        while self.run:
            self.state.step( h )
            time.sleep( h )
    def _read_task( self ):
        if isinstance( self.ser, serial.Serial ):
            buf = b""
            while self.run:
                if self.ser.in_waiting:
                    buf = buf + self.ser.readall()
                    cmd, buf = Controller._parse(buf)
                    while cmd:
                        res, self.state = cmd_parser( self.state, cmd )
                        if res: self.queue.put( res )
                        cmd, buf = Controller._parse(buf)
                time.sleep( 0.005 )
    def _write_task( self ):
        if isinstance( self.ser, serial.Serial ):
            while self.run:
                res = self.queue.get()
                self.ser.write( res )
