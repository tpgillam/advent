include("common.jl")

struct Trees
    data::Matrix{Int}
end

function Base.parse(::Type{Trees}, str::AbstractString)
    lines = map(strip, filter(!isempty, split(str, '\n')))
    nrow = length(lines)
    ncol = length(first(lines))
    data = Matrix{Int}(undef, nrow, ncol)
    for (i, line) in enumerate(lines)
        for (j, char) in enumerate(line)
            data[i, j] = parse(Int, char)
        end
    end
    return Trees(data)
end

function num_visible(trees::Trees)
    # A visibility map.
    is_visible = zeros(Bool, size(trees.data))

    for (i, row) in enumerate(eachrow(trees.data))
        tallest = -1
        for (j, x) in enumerate(row)
            x <= tallest && continue
            is_visible[i, j] = true
            tallest = x
        end

        tallest = -1
        for (j, x) in Iterators.reverse(enumerate(row))
            x <= tallest && continue
            is_visible[i, j] = true
            tallest = x
        end
    end

    for (j, col) in enumerate(eachcol(trees.data))
        tallest = -1
        for (i, x) in enumerate(col)
            x <= tallest && continue
            is_visible[i, j] = true
            tallest = x
        end

        tallest = -1
        for (i, x) in Iterators.reverse(enumerate(col))
            x <= tallest && continue
            is_visible[i, j] = true
            tallest = x
        end
    end

    return sum(is_visible)
end

function get_ans(input::AbstractString)
    trees = parse(Trees, input)
    return num_visible(trees)
end

example = """
30373
25512
65332
33549
35390
"""

@assert get_ans(example) == 21
get_ans(load_input(8))

# Part 2

function compute_scenic_scores(trees::Trees)
    scores = Matrix{Int}(undef, size(trees.data))

    for ind in CartesianIndices(trees.data)
        x = trees.data[ind]
        i, j = ind.I

        row = trees.data[i, :]
        col = trees.data[:, j]

        # Look left
        nl = 0
        for y in row[(j - 1):-1:1]
            nl += 1
            y >= x && break
        end

        # Look right
        nr = 0
        for y in row[(j + 1):end]
            nr += 1
            y >= x && break
        end

        # Look up
        nu = 0
        for y in col[(i - 1):-1:1]
            nu += 1
            y >= x && break
        end

        # Look down
        nd = 0
        for y in col[(i + 1):end]
            nd += 1
            y >= x && break
        end

        score = nl * nr * nu * nd
        scores[ind] = score
    end

    return scores
end

function get_ans2(input::AbstractString)
    trees = parse(Trees, input)
    return maximum(compute_scenic_scores(trees))
end

@assert get_ans2(example) == 8
get_ans2(load_input(8))
