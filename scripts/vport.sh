#!/bin/sh
tty0=$HOME/ttyUSB0
tty1=$HOME/ttyUSB1
gbg="\e[42m\e[38m"
gfg="\e[32m"
rbg="\e[41m\e[38m"
rfg="\e[31m"
end="\e[0m"
echo -e "$gbg LOG $end$gfg Starting Virtual Port $end"
socat -d -d -v PTY,raw,link=$tty0 PTY,raw,link=$tty1
echo -e "$rbg LOG $end$rfg Closing Virtual Port $end"
