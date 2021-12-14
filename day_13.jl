include("common.jl")

example = """
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"""

abstract type Fold end

struct FoldX <: Fold
    location::Int64
end

struct FoldY <: Fold
    location::Int64
end

struct Problem
    points::Vector{Pair{Int64,Int64}}
    folds::Vector{Fold}
end

function Base.parse(::Type{Problem}, input::AbstractString)
    points = Pair{Int64,Int64}[]
    folds = Fold[]
    for line in split(input, '\n')
        isempty(line) && continue
        if startswith(line, "fold")
            line = replace(line, "fold along " => "")
            direction, location = split(line, '=')
            location = parse(Int64, location)
            push!(folds, direction == "x" ? FoldX(location) : FoldY(location))
        else
            push!(points, Pair(parse.(Int64, split(line, ','))...))
        end
    end
    return Problem(points, folds)
end

function apply_fold(points::Vector{Pair{Int64,Int64}}, fold::FoldX)
    points = map(points) do (x, y)
        @assert x != fold.location
        x = x < fold.location ? x : fold.location - (x - fold.location)
        return x => y
    end
    return unique(points)
end

function apply_fold(points::Vector{Pair{Int64,Int64}}, fold::FoldY)
    points = map(points) do (x, y)
        @assert y != fold.location
        y = y < fold.location ? y : fold.location - (y - fold.location)
        return x => y
    end
    return unique(points)
end


function apply_fold(problem::Problem)
    fold = first(problem.folds)
    points = apply_fold(problem.points, fold)
    return Problem(points, @view problem.folds[2:end])
end

num_dots(problem::Problem) = length(problem.points)

function get_ans(input::AbstractString)
    problem = parse(Problem, input)
    problem = apply_fold(problem)
    return num_dots(problem)
end

@assert get_ans(example) == 17
get_ans(load_input(13))

# Part 2

is_finished(problem::Problem) = isempty(problem.folds)

function apply_folds(problem::Problem)
    while !is_finished(problem)
        problem = apply_fold(problem)
    end
    return problem
end

function Base.convert(::Type{Matrix{Bool}}, problem::Problem)
    max_x = maximum(first, problem.points)
    max_y = maximum(last, problem.points)
    result = zeros(Bool, max_x + 1, max_y + 1)
    @inbounds for (x, y) in problem.points
        result[x + 1, y + 1] = 1
    end
    return result
end

function display_ans2(input::AbstractString)
    problem = apply_folds(parse(Problem, input))
    for row in eachcol(convert(Matrix{Bool}, problem))
        for x in row
            if x == 1
                print('1')
            else
                print(' ')
            end
        end
        print('\n')
    end
    return nothing
end

display_ans2(example)

display_ans2(load_input(13))
