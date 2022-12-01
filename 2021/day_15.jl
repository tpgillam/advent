include("common.jl")

example = """
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"""

struct Problem
    risk::Matrix{Int8}
end

function Base.parse(::Type{Problem}, input::AbstractString)
    rows = Vector{Int8}[]
    for line in split(input, '\n')
        isempty(line) && continue
        push!(rows, map(x -> parse(Int8, x), collect(line)))
    end
    return Problem(reduce(vcat, transpose.(rows)))
end

"""Determine the neighbours we can go to from this point."""
function neighbours(problem::Problem, i::Integer, j::Integer)
    i_max, j_max = size(problem.risk)

    result = Tuple{Int64,Int64}[]
    sizehint!(result, 4)

    if i > 1
        push!(result, (i - 1, j))
    end
    if j > 1
        push!(result, (i, j - 1))
    end
    if i < i_max
        push!(result, (i + 1, j))
    end
    if j < j_max
        push!(result, (i, j + 1))
    end

    return result
end

function neighbours(problem::Problem, point::Tuple{Integer,Integer})
    return neighbours(problem, point[1], point[2])
end

function get_total_risk(problem::Problem)
    total_risk = zeros(Int64, size(problem.risk))

    point_stack = [size(total_risk)]
    while !isempty(point_stack)
        point = pop!(point_stack)
        initial_risks = filter(
            !iszero, [total_risk[point_in...] for point_in in neighbours(problem, point)]
        )
        best_initial_risk = isempty(initial_risks) ? 0 : minimum(initial_risks)
        best_risk = best_initial_risk + problem.risk[point...]

        # If we have already visited this point, determine if this path is useful. It is, then
        # we should add the next points.
        is_better = iszero(total_risk[point...]) || total_risk[point...] > best_risk

        if is_better
            total_risk[point...] = best_risk
            # Add the next points onto the stack
            append!(point_stack, neighbours(problem, point))
        end
    end

    return total_risk
end

function get_ans(problem::Problem)
    total_risk = get_total_risk(problem)
    # The starting position is never entered, so its risk is not counted
    return total_risk[1, 1] - problem.risk[1, 1]
end

get_ans(input::AbstractString) = get_ans(parse(Problem, input))

@assert get_ans(example) == 40
get_ans(load_input(15))

# Part 2

function pluswrap(x::Integer, y::Integer)
    z = x + y
    return z > 9 ? z - 9 : z
end

@assert pluswrap(8, 1) == 9
@assert pluswrap(8, 2) == 1

"""Perform the appropriate tiling operation."""
function make_problem2(problem::Problem)
    size_i, size_j = size(problem.risk)
    new_risk = Matrix{Int8}(undef, 5 .* size(problem.risk))

    for i in 0:4
        for j in 0:4
            risk_increment = Int8(i + j)
            range_i = (1 + i * size_i):((i + 1) * size_i)
            range_j = (1 + j * size_j):((j + 1) * size_j)
            for (i_old, i_new) in enumerate(range_i)
                for (j_old, j_new) in enumerate(range_j)
                    new_risk[i_new, j_new] = pluswrap(
                        problem.risk[i_old, j_old], risk_increment
                    )
                end
            end
        end
    end
    return Problem(new_risk)
end

function get_ans2(input::AbstractString)
    problem = parse(Problem, input)
    problem2 = make_problem2(problem)
    return get_ans(problem2)
end

@assert get_ans2(example) == 315
get_ans2(load_input(15))
