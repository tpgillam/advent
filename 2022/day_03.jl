include("common.jl")

struct Rucksack
    compartment_1::String
    compartment_2::String
end

function Base.parse(::Type{Rucksack}, str::AbstractString)
    n = length(str)
    isodd(n) && throw(ArgumentError("Should have an even number of items, got $str"))
    i_half::Int = n / 2
    return Rucksack(str[1:i_half], str[(i_half + 1):end])
end

function Base.parse(::Type{Vector{Rucksack}}, str::AbstractString)
    result = Rucksack[]
    for line in split(str, '\n')
        isempty(line) && continue
        push!(result, parse(Rucksack, line))
    end
    return result
end

function misplaced_item(rucksack::Rucksack)
    return only(intersect(rucksack.compartment_1, rucksack.compartment_2))
end

function priority(x::Char)
    xi = Int(x)
    lower_start = Int('a')
    upper_start = Int('A')
    n = 26
    lower_start <= xi < lower_start + n && return xi - (lower_start - 1)
    upper_start <= xi < upper_start + n && return n + xi - (upper_start - 1)
    throw(ArgumentError("Unsupport char $x"))
end

function get_ans(input::AbstractString)
    rucksacks = parse(Vector{Rucksack}, input)
    return sum(rucksacks) do rucksack
        priority(misplaced_item(rucksack))
    end
end

example = """
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"""

@assert get_ans(example) == 157
get_ans(load_input(3))

# Part 2
struct Group
    r1::Rucksack
    r2::Rucksack
    r3::Rucksack
end

function Base.parse(::Type{Vector{Group}}, str::AbstractString)
    rucksacks = parse(Vector{Rucksack}, str)
    n = length(rucksacks)
    n % 3 == 0 || throw(ArgumentError("Got n = $n, but should be divisible by three"))
    m = reshape(rucksacks, (3, :))  # (3, n_groups)
    return map(x -> Group(x...), eachcol(m))
end

items(rucksack::Rucksack) = union(rucksack.compartment_1, rucksack.compartment_2)
badge(group::Group) = only(intersect(items(group.r1), items(group.r2), items(group.r3)))

function get_ans2(input::AbstractString)
    groups = parse(Vector{Group}, input)
    return sum(groups) do group
        priority(badge(group))
    end
end

@assert get_ans2(example) == 70
get_ans2(load_input(3))
