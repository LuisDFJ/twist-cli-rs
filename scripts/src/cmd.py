from src.state import State

def cmd( state : State, cmd : bytes ) -> tuple[ bytes, State ]:
    c = b''
    if len(cmd) == 1:
        match cmd[0:1]:
            # Request Speed
            case b'a':
                c = state.get_speed_units().encode()
            # Set travel unit to revolutions
            case b'b':
                state.units = "REV"
            # Enter cycle mode
            case b'c':
                state.mode = "C"
            # Move crosshead CCW
            case b'd':
                state.direction = "CCW"
                state.run = True
            # Set travel unit to degrees
            case b'i':
                state.units = 'DEG'
            # Set test stand speed to max speed
            case b'j':
                state.prg_speed = 'M'
            # Set test stand speed to min speed
            case b'k':
                state.prg_speed = 'm'
            # Enter travel limit mode
            case b'l':
                state.mode = 'T'
            # Enter manual mode
            case b'm':
                state.mode = 'M'
            # Transmit travel and force reading
            case b'n':
                c = state.travel_force_read()
            # Set test stand speed to programmed speed
            case b'o':
                state.prg_speed = 'S'
            # Request stand status
            case b'p':
                c = state.status()
            # Request number of cycles completed
            case b'q':
                c = str(state.cycles_count).encode()
            # Request number of cycles set
            case b'r':
                c = str(state.cycles).encode()
            # Stop test stand motion
            case b's':
                state.run = False
            # Reset cycle counter to zero
            case b't':
                state.cycles_count = 0
            # Move test stand CW
            case b'u':
                state.direction = 'CW'
                state.run = True
            # Request CW travel limit
            case b'v':
                c = state.get_cw_limit_units().encode()
            # Request CCW travel limit
            case b'w':
                c = state.get_ccw_limit_units().encode()
            # Request rotational travel value
            case b'x':
                c = state.travel_read()
            # Reset rotational travel position to zero
            case b'z':
                state.x = 0.0
            case _ : pass
    else:
        match cmd[0:1]:
            case b'e':
                if len(cmd) == 6:
                    try: state.set_speed( float( cmd[1:] ) )
                    except: pass
            # Set cycles
            case b'f':
                if len(cmd) == 5:
                    try: state.cycles = int( cmd[1:] )
                    except: pass
            # Set CCW travel limit
            case b'g':
                if len(cmd) == 8:
                    try: state.set_ccw_limit( float( cmd[1:] ) )
                    except: pass
            # Set CW travel limit
            case b'h':
                if len(cmd) == 8:
                    try: state.set_cw_limit( float( cmd[1:] ) )
                    except: pass
            case _ : pass
    return c, state
