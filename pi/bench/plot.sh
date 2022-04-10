#!/bin/bash
FILE=dat/termgen-n-ms.dat
if [ ! -f "$FILE" ]; then
    cargo run --bin bench > $FILE &
    sleep 1
fi
gnuplot -c termgen-n-ms.gp
