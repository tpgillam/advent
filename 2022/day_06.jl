include("common.jl")

using AssociativeWindowAggregation

function compute_ans(input::AbstractString, n::Int)
    input = strip(input)

    window = FixedWindowAssociativeOp{Set{Char},union}(n)
    for (i, char) in enumerate(input)
        update_state!(window, Set([char]))
        window_full(window) || continue
        length(window_value(window)) == n || continue
        # The window is full!
        return i
    end
    # The window never became full
    throw(ArgumentError("Never found a unique $n letter substring"))
end

get_ans(input::AbstractString) = compute_ans(input, 4)

example = "mjqjpqmgbljsphdztnvjfqwrcgsmlb"

@assert get_ans(example) == 7
get_ans(load_input(6))

# Part 2

get_ans2(input::AbstractString) = compute_ans(input, 14)


@assert get_ans2(example) == 19
get_ans2(load_input(6))
