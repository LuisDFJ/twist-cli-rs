import math

def endline_bytes( f ):
    def wrapper( *args, **kargs ):
        return f(*args, **kargs) + b"\r\n"
    return wrapper

def endline( f ):
    def wrapper( *args, **kargs ):
        return f(*args, **kargs) + "\r\n"
    return wrapper

def clip( x : float, a : float, b : float ) -> float:
    return max( min(x,b),a )

class State:
    def __init__(self) -> None:
        self.max_speed = 5.0
        self.min_speed = 0.2
        self.speed = 1.0
        self.prg_speed = 'S'
        self.direction = 'CW'
        self.units = "DEG"
        self.mode = "M"
        self.cycles = 0
        self.cycles_count = 0
        self.ccw_limit = 180.0
        self.cw_limit = 180.0
        self.run = False
        self.y = 0.0
        self.x = 0.0
        self.e = 0.0
    def step(self, t : float) -> None:
        if self.run:
            sign = -1 if self.direction == 'CW' else 1
            dx = sign * self.get_speed() * t
            dy = math.sqrt(abs(self.x)) * dx
            de = dx * ( self.y + dy )
            if de > 0:
                self.e += de
                self.x += dx
                self.y += dy
            else:
                a = 0.1
                self.e += de + a*dx*dy
                self.x += dx
                self.y += (1+a)*dy

    def set_speed( self, speed : float ) -> None:
        if self.units == 'DEG': speed = speed
        else: speed = speed * 6.0
        self.speed = clip( speed, self.min_speed, self.max_speed )
    def set_ccw_limit( self, limit : float ) -> None:
        if self.units == 'DEG': self.ccw_limit = limit
        else: self.ccw_limit = limit * 360.0
    def set_cw_limit( self, limit : float ) -> None:
        if self.units == 'DEG': self.cw_limit = limit
        else: self.cw_limit = limit * 360.0
    def get_speed(self) -> float:
        match self.prg_speed:
            case 'M': return self.max_speed
            case 'm': return self.min_speed
            case  _ : return self.speed

    @endline
    def get_ccw_limit_units(self) -> str:
        limit = self.ccw_limit
        if self.units == "DEG":
            return f"{limit:06.2f} DEG"
        else:
            return f"{limit/360.0:06.2f} REV"
    @endline
    def get_cw_limit_units(self) -> str:
        limit = self.cw_limit
        if self.units == "DEG":
            return f"{limit:06.2f} DEG"
        else:
            return f"{limit/360.0:06.2f} REV"
    @endline
    def get_speed_units(self) -> str:
        speed = self.get_speed()
        if self.units == "DEG":
            return f"{speed:06.2f} DEG/S"
        else:
            return f"{speed/6.0:06.2f} RPM"
    @endline_bytes
    def travel_force_read(self) -> bytes:
        return "{:.2f} {}\r\n {:.2f} {}".format(
            self.x if self.units == "DEG" else self.x / 360.0,
            self.units,
            self.y,
            "ncm"
        ).encode()
    @endline_bytes
    def travel_read(self) -> bytes:
        return "{:.2f} {}".format(
            self.x if self.units == "DEG" else self.x / 360.0,
            self.units,
        ).encode()
    @endline_bytes
    def status(self) -> bytes:
        status = 'S' if not self.run else self.direction
        return "Status: {} Mode: {} Speed: {} Cycles: {} Cycles C: {}: TF: {}".format(
            status,
            self.mode,
            self.get_speed_units(),
            self.cycles,
            self.cycles_count,
            self.travel_force_read()
        ).encode()
