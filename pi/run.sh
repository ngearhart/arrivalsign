#!/bin/bash
runscript(){
    cd /root/arrivalsign/pi
    python3 main.py
}

until runscript; do
    echo "'main.py' crashed with exit code $?. Restarting..." >&2
    sleep 1
done
