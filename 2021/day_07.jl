using Statistics

include("common.jl")

example = "16,1,2,0,4,2,7,1,2,14"

get_locations(input::AbstractString) = map(x -> parse(Int64, x), split(input, ","))

fuel_cost(locations::AbstractVector{Int64}, x::Real) = sum(abs.(locations .- x))

function get_best_cost(locations::AbstractVector{Int64})
    optimal_loc = Int64(median(locations))
    return fuel_cost(locations, optimal_loc)
end

get_ans(input::AbstractString) = get_best_cost(get_locations(input))

@assert get_ans(example) == 37
get_ans(load_input(7))

# Part 2

function fuel_cost2(loc::Int64, x::Real)
    d = abs(loc - x)
    return Int(d * (d + 1) / 2)
end
fuel_cost2(locations::AbstractVector{Int64}, x::Real) = sum(fuel_cost2.(locations, x))

function get_best_cost2(locations::AbstractVector{Int64})
    # We're likely to be *close* to the mean, so start there and walk (since the objective
    # is convex).
    loc = Int64(round(mean(locations)))

    cost(x) = fuel_cost2(locations, x)

    while cost(loc + 1) < cost(loc)
        loc += 1
    end

    while cost(loc - 1) < cost(loc)
        loc -= 1
    end

    return cost(loc)
end

get_ans2(input::AbstractString) = get_best_cost2(get_locations(input))

@assert get_ans2(example) == 168
get_ans2(load_input(7))
