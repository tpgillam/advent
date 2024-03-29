using Printf

function load_input(day::Integer)
    day = @sprintf("%0.2d", day)
    name = "day_$(day).txt"
    path = joinpath("2022", "inputs", name)
    return read(path, String)
end

abstract type Part end
struct Part1 <: Part end
struct Part2 <: Part end
