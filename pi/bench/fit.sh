#!/bin/gnuplot
fit a * x**2 + b * x + c 'termgen-n-ms.dat' via a, b, c
