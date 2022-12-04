include("common.jl")

struct ElfPair
    range_1::UnitRange{Int}
    range_2::UnitRange{Int}
end

function _parse_range(str::AbstractString)
    (a, b) = split(str, '-')
    return parse(Int, a):parse(Int, b)
end

function Base.parse(::Type{ElfPair}, str::AbstractString)
    a, b = split(str, ',')
    return ElfPair(_parse_range(a), _parse_range(b))
end

function Base.parse(::Type{Vector{ElfPair}}, str::AbstractString)
    return map(filter(!isempty, split(str, '\n'))) do line
        parse(ElfPair, line)
    end
end

is_contained(x::ElfPair) = issubset(x.range_1, x.range_2) ||issubset(x.range_2, x.range_1)

function get_ans(input::AbstractString)
    pairs = parse(Vector{ElfPair}, input)
    return sum(is_contained, pairs)
end

# Tests
@assert _parse_range("1-3") == 1:3
@assert _parse_range("2-5") == 2:5
@assert parse(ElfPair, "2-4,6-8") == ElfPair(2:4, 6:8)

example = """
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"""

@assert get_ans(example) == 2
get_ans(load_input(4))

# Part 2

overlaps(x::ElfPair) = !isempty(intersect(x.range_1, x.range_2))

function get_ans2(input::AbstractString)
    pairs = parse(Vector{ElfPair}, input)
    return sum(overlaps, pairs)
end

@assert get_ans2(example) == 4
get_ans2(load_input(4))
