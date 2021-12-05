include("common.jl")

example = """
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"""

struct Line
    start_x::Int64
    start_y::Int64
    end_x::Int64
    end_y::Int64
end

function Base.parse(::Type{Line}, line::AbstractString)
    l_start, l_end = split(line, " -> ")
    start_x, start_y = split(l_start, ",")
    end_x, end_y = split(l_end, ",")
    return Line(map(x -> parse(Int64, x), (start_x, start_y, end_x, end_y))...)
end

function Base.parse(::Type{Vector{Line}}, input::AbstractString)
    lines = split(input, "\n")
    result = Line[]
    for line in lines
        isempty(line) && continue
        push!(result, parse(Line, line))
    end
    return result
end

horizontal(line::Line) = line.start_y == line.end_y
vertical(line::Line) = line.start_x == line.end_x

rectilinear(line::Line) = horizontal(line) || vertical(line)

function range_index_x(line::Line)
    ix1 = line.start_x + 1
    ix2 = line.end_x + 1
    return ix1 < ix2 ? (ix1:ix2) : (ix1:-1:ix2)
end

function range_index_y(line::Line)
    iy1 = line.start_y + 1
    iy2 = line.end_y + 1
    return iy1 < iy2 ? (iy1:iy2) : (iy1:-1:iy2)
end

function make_grid(lines::AbstractVector{Line})
    max_x = 0
    max_y = 0
    for line in lines
        max_x = max(max_x, line.start_x)
        max_x = max(max_x, line.end_x)
        max_y = max(max_y, line.start_y)
        max_y = max(max_y, line.end_y)
    end
    # Note that we have a zero-based indexing in terms of Line, so add one.
    return zeros(Int64, (max_x + 1, max_y + 1))
end

function populate_rectilinear!(grid::Matrix{Int64}, lines::AbstractVector{Line})
    for line in lines
        if horizontal(line)
            i_y = line.start_y + 1
            @inbounds for i_x in range_index_x(line)
                grid[i_x, i_y] += 1
            end
        elseif vertical(line)
            i_x = line.start_x + 1
            @inbounds for i_y in range_index_y(line)
                grid[i_x, i_y] += 1
            end
        end
        # Line gets skipped if it isn't rectilinear.
    end
    return grid
end

function populate_rectilinear(lines::AbstractVector{Line})
    return populate_rectilinear!(make_grid(lines), lines)
end

function get_ans(input::AbstractString)
    lines = parse(Vector{Line}, input)
    grid = populate_rectilinear(lines)
    return sum(grid .> 1)
end


get_ans(example)
@assert get_ans(example) == 5
get_ans(load_input(5))


# Part 2

function populate_all!(grid::Matrix{Int64}, lines::AbstractVector{Line})
    for line in lines
        if horizontal(line)
            i_y = line.start_y + 1
            @inbounds for i_x in range_index_x(line)
                grid[i_x, i_y] += 1
            end
        elseif vertical(line)
            i_x = line.start_x + 1
            @inbounds for i_y in range_index_y(line)
                grid[i_x, i_y] += 1
            end
        else
            # It should be the case that the line as at 45 degrees...
            Ix = range_index_x(line)
            Iy = range_index_y(line)
            @assert length(Ix) == length(Iy)
            for (i_x, i_y) in zip(Ix, Iy)
                grid[i_x, i_y] += 1
            end
        end
    end
    return grid
end

populate_all(lines) = populate_all!(make_grid(lines), lines)

function get_ans2(input::AbstractString)
    lines = parse(Vector{Line}, input)
    grid = populate_all(lines)
    return sum(grid .> 1)
end

@assert get_ans2(example) == 12
get_ans2(load_input(5))
