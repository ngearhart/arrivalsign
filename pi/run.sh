#!/bin/bash
runscript(){
    # must CD for .env
    cd /root/arrivalsign/rustpi
    /root/arrivalsign/rustpi/target/release/metrosign
}

until runscript; do
    echo "'main.py' crashed with exit code $?. Restarting..." >&2
    sleep 1
done
