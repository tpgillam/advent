include("common.jl")

using ResumableFunctions

struct Point
    x::Int
    y::Int
end

Base.parse(::Type{Point}, str::AbstractString) = Point(parse.(Ref(Int), split(str, ','))...)

struct Line
    points::Vector{Point}
end

Base.length(line::Line) = length(line.points)
Base.parse(::Type{Line}, str::AbstractString) = Line(parse.(Ref(Point), split(str, " -> ")))
function Base.parse(::Type{Vector{Line}}, str::AbstractString)
    return parse.(Ref(Line), filter(!isempty, split(str, '\n')))
end

abstract type Material end
struct Air <: Material end
struct Rock <: Material end
struct Sand <: Material end

struct Map
    data::Matrix{Material}
    offset_x::Int
    offset_y::Int
    function Map(nx::Integer, ny::Integer, offset_x::Integer, offset_y::Integer)
        data = Matrix{Material}(undef, (nx, ny))
        fill!(data, Air())
        return new(data, offset_x, offset_y)
    end
end

_ind(map::Map, point::Point) = (point.x - map.offset_x, point.y - map.offset_y)

Base.getindex(map::Map, point::Point) = map.data[_ind(map, point)...]
function Base.setindex!(map::Map, material::Material, point::Point)
    map.data[_ind(map, point)...] = material
    return map
end

@resumable function points(lines::AbstractVector{Line})
    for line in lines
        for point in line.points
            @yield point
        end
    end
end

# This is the location of the sand source
const SOURCE = Point(500, 0)

function add_line!(map::Map, line::Line)
    n = length(line)
    n >= 2 || throw(ArgumentError("Invalid line: $line"))

    for i in 1:(n - 1)
        p1 = line.points[i]
        p2 = line.points[i + 1]
        add_line!(map, p1, p2)
    end

    return map
end

function add_line!(map::Map, a::Point, b::Point)
    if a.x == b.x
        # Move in y
        y1, y2 = minmax(a.y, b.y)
        for y in y1:1:y2
            map[Point(a.x, y)] = Rock()
        end
    elseif a.y == b.y
        # Move in x
        x1, x2 = minmax(a.x, b.x)
        for x in x1:1:x2
            map[Point(x, a.y)] = Rock()
        end
    else
        throw(ArgumentError("Can only draw horizontal or vertical lines. Got: $a -> $b"))
    end
    return map
end

function Base.convert(::Type{Map}, lines::AbstractVector{Line})
    # Find the bounding box of all the lines we have.
    min_x, max_x = extrema(p -> p.x, points(lines))
    min_y, max_y = extrema(p -> p.y, points(lines))

    # Ensure that we include the source.
    min_x = min(min_x, SOURCE.x)
    max_x = max(max_x, SOURCE.x)
    min_y = min(min_y, SOURCE.y)
    max_y = max(max_y, SOURCE.y)

    # size of our array
    nx = 1 + max_x - min_x
    ny = 1 + max_y - min_y

    # Create a map. The offsets are chosen such that `min_x - offset_x = 1`.
    map = Map(nx, ny, min_x - 1, min_y - 1)

    foreach(lines) do line
        add_line!(map, line)
    end

    return map
end

function off_map(map::Map, point::Point)
    i, j = _ind(map, point)
    1 <= i <= size(map.data, 1) || return true
    1 <= j <= size(map.data, 2) || return true
    return false
end

"""
    drop_sand!(map::Map)

Drop a particle of sand, and mutate map with the sand.

Returns `true` iff the sand comes to rest, else `false`.
"""
function drop_sand!(map::Map)
    p = SOURCE
    while true
        for (dx, dy) in [(0, 1), (-1, 1), (1, 1)]
            proposal = Point(p.x + dx, p.y + dy)
            # If we're falling off the map with this proposal, we're done. No sand comes
            # to rest.
            off_map(map, proposal) && return false

            material = map[proposal]

            if isa(material, Air)
                # Accept the proposal.
                p = proposal
                @goto next_move
            end
        end
        # We have proposed all possible points, but not accepted any.
        # The sand comes to rest.
        map[p] = Sand()
        return true

        @label next_move
    end
    return map
end

function get_ans(input::AbstractString)
    lines = parse(Vector{Line}, input)
    moo = convert(Map, lines)
    num_sand_particles = 0
    while drop_sand!(moo)
        num_sand_particles += 1
    end
    return num_sand_particles
end

example = """
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"""

@assert get_ans(example) == 24
get_ans(load_input(14))

# Part 2

function get_ans2(input::AbstractString)
    lines = parse(Vector{Line}, input)

    max_y = maximum(p -> p.y, points(lines))
    y = max_y + 2
    x_left = SOURCE.x - y
    x_right = SOURCE.x + y
    bottom = Line([Point(x_left, y), Point(x_right, y)])

    # Add a line for the bottom rock
    push!(lines, bottom)

    moo = convert(Map, lines)

    num_sand_particles = 0
    while isa(moo[SOURCE], Air)
        @assert drop_sand!(moo)
        num_sand_particles += 1
    end
    return num_sand_particles
end

@assert get_ans2(example) == 93
get_ans2(load_input(14))
