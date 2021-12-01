## Part 1
function _load_input(name::AbstractString)
    path = joinpath("inputs", name)
    return map(x -> parse(Int64, x), readlines(path))
end

function num_depth_increases(depths::AbstractVector{<:Real})
    length(depths) > 1 || throw(ArgumentError("Need at least two depths"))
    count = 0
    previous = @inbounds first(depths)
    for x in @view(depths[2:end])
        if x > previous
            count += 1
        end
        previous = x
    end
    return count
end

example = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
@assert num_depth_increases(example) == 7
depths = _load_input("day_01.txt")
num_depth_increases(depths)

## Part 2
using Dates
using TimeDag

function num_depth_increases(depths::AbstractVector{<:Real}, window::Int64)
    t_start = DateTime(2000)
    block = Block(
        TimeDag.unchecked, t_start:Day(1):(t_start + Day(1) * (length(depths) - 1)), depths
    )

    _eval(x) = evaluate(x, t_start, last(block.times) + Day(1))

    # Form a node whose last value will be the answer.
    depths_node = block_node(block)
    window_sum = sum(depths_node, window)
    previous = lag(window_sum, 1)
    # There's a bug in TimeDag where you can't do sums of Bools correctly right now:
    #   https://github.com/invenia/TimeDag.jl/issues/22
    result_node = sum(TimeDag.apply((x, y) -> x > y ? 1 : 0, window_sum, previous))
    return last(_eval(result_node).values)
end

@assert num_depth_increases(example, 3) == 5
@time num_depth_increases(depths, 3)
