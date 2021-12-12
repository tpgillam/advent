using Graphs

include("common.jl")

example_1 = """
start-A
start-b
A-c
A-b
b-d
A-end
b-end
"""

example_2 = """
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"""

example_3 = """
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"""

struct Caves
    cave_names::Vector{String}
    is_big::Vector{Bool}
    graph::SimpleGraph{Int64}
end

start_index(caves::Caves) = findfirst(==("start"), caves.cave_names)
end_index(caves::Caves) = findfirst(==("end"), caves.cave_names)
Base.@propagate_inbounds is_big(caves::Caves, index::Int64) = caves.is_big[index]

function Base.parse(::Type{Caves}, input::AbstractString)
    graph = SimpleGraph{Int64}()
    cave_names = Vector{String}()
    is_big = Vector{Bool}()
    cave_to_index = Dict{String,Int64}()

    function get_cave_index(cave::AbstractString)
        next_index = nv(graph) + 1
        index = get!(cave_to_index, cave, next_index)
        if index == next_index
            push!(cave_names, cave)
            push!(is_big, isuppercase(first(cave)))
            @assert add_vertex!(graph)
        end
        return index
    end

    for line in split(input, '\n')
        isempty(line) && continue
        cave_1, cave_2 = split(line, '-')

        i1 = get_cave_index(cave_1)
        i2 = get_cave_index(cave_2)

        @assert add_edge!(graph, i1, i2)
    end
    return Caves(cave_names, is_big, graph)
end

"""
    Path

Represent a path through the caves, starting in the correct location.
"""
mutable struct Path
    caves::Caves
    cave_order::Vector{Int64}
    visit_count::Vector{Int64}
    n_additional_small::Int64

    function Path(caves::Caves, n_additional_small::Int64)
        i_start = start_index(caves)
        visit_count = zeros(Int64, nv(caves.graph))
        @inbounds visit_count[i_start] = 1
        return new(caves, [i_start], visit_count, n_additional_small)
    end

    function Path(path::Path)
        return new(
            path.caves,
            copy(path.cave_order),
            copy(path.visit_count),
            path.n_additional_small,
        )
    end
end

function Base.show(io::IO, path::Path)
    repr = join((path.caves.cave_names[i] for i in path.cave_order), '-')
    print(io, repr)
    return nothing
end

start_index(path::Path) = start_index(path.caves)
end_index(path::Path) = end_index(path.caves)

is_finished(path::Path) = @inbounds last(path.cave_order) == end_index(path)

"""
Return true iff the given node is visitable, given the visit counts.
"""
Base.@propagate_inbounds function is_visitable(path::Path, index::Int64)
    return if is_big(path.caves, index)
        true
    elseif index == start_index(path) || index == end_index(path)
        path.visit_count[index] < 1
    else
        path.visit_count[index] < (1 + path.n_additional_small)
    end
end

"""
Get a vector of the next cave indices that are visitable.
Returns an empty vector if the path is finished, or we cannot take further steps.
"""
function next_caves(path::Path)
    is_finished(path) && return Int64[]
    index = @inbounds last(path.cave_order)
    return @inbounds filter(i -> is_visitable(path, i), neighbors(path.caves.graph, index))
end

"""
Add a cave to the path.
Does NOT do any checking to ascertain whether this is a valid move.
"""
Base.@propagate_inbounds function add_cave!(path::Path, index::Int64)
    push!(path.cave_order, index)
    path.visit_count[index] += 1
    @inbounds if !is_big(path.caves, index)
        # Decrement the number of additional small cave visits we can make if we have used.
        if path.visit_count[index] > 1
            path.n_additional_small -= 1
        end
    end
    return path
end

function get_paths(caves::Caves, n_additional_small::Int64)
    finished_paths = Path[]
    paths = Path[Path(caves, n_additional_small)]
    while !isempty(paths)
        path = pop!(paths)

        if is_finished(path)
            # The path is finished, bung it on the finished pile
            push!(finished_paths, path)
            continue
        end

        # Create new paths with every option.
        for index in next_caves(path)
            new_path = Path(path)
            add_cave!(new_path, index)
            push!(paths, new_path)
        end
    end
    return finished_paths
end

get_ans(input::AbstractString) = length(get_paths(parse(Caves, input), 0))

@assert get_ans(example_1) == 10
@assert get_ans(example_2) == 19
@assert get_ans(example_3) == 226
get_ans(load_input(12))

# Part 2

get_ans2(input::AbstractString) = length(get_paths(parse(Caves, input), 1))

@assert get_ans2(example_1) == 36
@assert get_ans2(example_2) == 103
@assert get_ans2(example_3) == 3509
get_ans2(load_input(12))
