#=
main:
- Julia version: 
- Author: hasan
- Date: 2019-09-06
Ctrl + Sfift + F10 for first run
=#

using Pkg
Pkg.add("Plots")
Pkg.add("Measures")
Pkg.add("PyPlot")
#Pkg.build("PyCall")
using Plots, Measures
pyplot()

sum2(x::Int, y::Float16)::Float16 = x + y
println(sum2(2,3))
data = [rand(100), rand(100)];

h = histogram(data, layout = 2,
              title = ["Dataset A" "Dataset B"], legend = false,
              ylabel = "ylabel", margin = 5mm)
display(h)
