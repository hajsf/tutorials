#!/usr/bin/env julia

#=
main:
- Julia version: 
- Author: hasan
- Date: 2019-09-06
Ctrl + Sfift + F10 for first run
=#

  #  Pkg.add("Gadfly")
using Gadfly, LibPQ

conn = LibPQ.Connection("dbname=postgres user=postgres password=Dana0Yara")

p1 = plot([sin,cos], 0, 2pi)
p2 = plot((x,y)->sin(x)+cos(y), 0, 2pi, 0, 2pi)
p3 = spy(ones(33)*sin.(0:(pi/16):2pi)' + cos.(0:(pi/16):2pi)*ones(33)')
hstack(p1,p2,p3)
