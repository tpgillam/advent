include("common.jl")

example = """
2199943210
3987894921
9856789892
8767896789
9899965678
"""

struct Floor
    data::Matrix{Int64}
end

function Base.parse(::Type{Floor}, input::AbstractString)
    rows = []
    for line in split(input, '\n')
        isempty(line) && continue
        row = [parse(Int64, x) for x in line]
        push!(rows, row)
    end
    data = transpose(reduce(hcat, rows))
    return Floor(data)
end

function shift(floor::Floor, si::Integer, sj::Integer)
    si == 0 && sj == 0 && return floor.data
    result = similar(floor.data)
    ni = size(result, 1)
    nj = size(result, 2)
    for i_out in 1:ni
        for j_out in 1:nj
            i_in = i_out + si
            j_in = j_out + sj
            value = if (i_in in 1:ni) && (j_in in 1:nj)
                floor.data[i_in, j_in]
            else
                # We have gone off the edge of the data, use a placeholder that is higher
                # than any value.
                10
            end
            result[i_out, j_out] = value
        end
    end
    return result
end

neighbour_offsets() = ((1, 0), (0, 1), (-1, 0), (0, -1))

function get_low_point_mask(floor::Floor)
    x = floor.data
    total = zeros(Int64, size(x))
    for (si, sj) in neighbour_offsets()
        total += x .< shift(floor, si, sj)
    end
    return total .== 4
end

function get_ans(input::AbstractString)
    floor = parse(Floor, input)
    mask = get_low_point_mask(floor)
    low_points = floor.data[mask]
    return sum(1 .+ low_points)
end

@assert get_ans(example) == 15
get_ans(load_input(9))

# Part 2

# Find the basin starting from the given low point.
function basin_size(floor::Floor, i::Integer, j::Integer)
    ni = size(floor.data, 1)
    nj = size(floor.data, 2)

    # Populate a mask with the basin. Start with the
    mask = zeros(Int64, size(floor.data))

    # Maintain a stack of points that we need to process, and a set of points that we have
    # already visited so that we don't duplicate work.
    mask[i, j] = 1
    stack = [(i, j)]

    visited = Set{Tuple{Int64,Int64}}()
    while !isempty(stack)
        i, j = pop!(stack)
        for (si, sj) in neighbour_offsets()
            i_new = i + si
            j_new = j + sj

            # If the neighbour isn't in the grid, it doesn't count.
            in(i_new, 1:ni) || continue
            in(j_new, 1:nj) || continue

            # If the neighbour has already been visited, we shouldn't consider it again.
            in((i_new, j_new), visited) && continue

            # Now we know that we haven't visited this neighbour before, so determine if
            # it should be added to the basin.
            height = floor.data[i_new, j_new]

            # Heights of 9 are never in a basin. All basins are also bordered by a 9.
            height == 9 && continue

            # We're in the basin!
            mask[i_new, j_new] = 1
            push!(stack, (i_new, j_new))
        end

        push!(visited, (i, j))
    end

    return sum(mask)
end

function get_ans2(input::AbstractString)
    floor = parse(Floor, input)
    mask = get_low_point_mask(floor)
    sizes = [
        basin_size(floor, Tuple(index)...)
        for index in findall(==(1), mask)
    ]
    @assert length(sizes) >= 3
    partialsort!(sizes, 3; rev=true)
    return sizes[1] * sizes[2] * sizes[3]
end


@assert get_ans2(example) == 1134
get_ans2(load_input(9))

# Note to self — actually one doesn't have to find the low points first.
# Really this is just a segmentation problem, with the areas to be found being bounded by
# '9' always. So actually it is doable in a single pass over the matrix.
