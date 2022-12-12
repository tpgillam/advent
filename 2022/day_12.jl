include("common.jl")

using Graphs

struct Hill
    heights::Matrix{Char}
    i_start::Tuple{Int,Int}
    i_end::Tuple{Int,Int}
end

_to_index(i::Int, num_cols::Int) = divrem(i, num_cols) .+ (1, 0)

function Base.parse(::Type{Hill}, str::AbstractString)
    i_start = -1
    i_end = -1
    heights = Char[]

    # We will use this to remember where we are in the array.
    i = 1
    num_cols = -1
    for line in split(str, '\n')
        isempty(line) && continue
        line = strip(line)
        num_cols = length(line)
        for char in line
            char_height = if char == 'S'
                i_start = i
                'a'
            elseif char == 'E'
                i_end = i
                'z'
            else
                char
            end
            'a' <= char_height <= 'z' || throw(error("Unexpected char: $char"))
            push!(heights, char_height)
            i += 1
        end
    end

    @assert i_start >= 1
    @assert i_end >= 1

    return Hill(
        permutedims(reshape(heights, (num_cols, :))),
        _to_index(i_start, num_cols),
        _to_index(i_end, num_cols),
    )
end

function out_locs(hill::Hill, loc::Tuple{Int,Int})
    height = hill.heights[loc...]
    nx, ny = size(hill.heights)

    return filter(
        map([(0, -1), (0, 1), (-1, 0), (1, 0)]) do delta
            loc .+ delta
        end,
    ) do (x, y)
        1 <= x <= nx || return false
        1 <= y <= ny || return false
        test_height = hill.heights[x, y]
        return test_height <= height + 1
    end
end

out_locs(hill::Hill, loc::CartesianIndex{2}) = out_locs(hill, loc.I)

struct HillGraph
    i_start::Int
    i_end::Int
    g::SimpleDiGraph{Int}
    hill::Hill
end

function make_hill_graph(hill::Hill)
    # Each square on the hill is a node in the graph. The node index will be the linear
    # index.
    g = SimpleDiGraph(length(hill.heights))
    cartesian_locs = CartesianIndices(hill.heights)
    linear_locs = LinearIndices(hill.heights)
    for i::Int in eachindex(hill.heights)
        for out_loc in out_locs(hill, cartesian_locs[i])
            i_out = linear_locs[out_loc...]
            add_edge!(g, i, i_out)
        end
    end

    i_start = linear_locs[hill.i_start...]
    i_end = linear_locs[hill.i_end...]

    return HillGraph(i_start, i_end, g, hill)
end

function get_ans(input::AbstractString)
    hill = parse(Hill, input)
    hill_graph = make_hill_graph(hill)

    state = dijkstra_shortest_paths(hill_graph.g, hill_graph.i_start)
    return state.dists[hill_graph.i_end]
end

example = """
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"""

@assert get_ans(example) == 31
get_ans(load_input(12))

# Part 2

function get_ans2(input::AbstractString)
    hill = parse(Hill, input)
    hill_graph = make_hill_graph(hill)

    possible_starting_nodes = findall(==('a'), vec(hill.heights))
    state = dijkstra_shortest_paths(hill_graph.g, possible_starting_nodes)
    return state.dists[hill_graph.i_end]
end

@assert get_ans2(example) == 29
get_ans2(load_input(12))
