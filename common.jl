function load_input(day::Integer)
    day = @sprintf("%0.2d", day)
    name = "day_$(day).txt"
    path = joinpath("inputs", name)
    return read(path, String)
end
