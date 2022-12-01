include("common.jl")

struct Item
    calories::Int
end

struct Elf
    items::Vector{Item}
end

calories(item::Item) = item.calories
calories(elf::Elf) = sum(calories, elf.items)
calories(elves::Vector{Elf}) = sum(calories, elves)

Base.parse(::Type{Item}, str::AbstractString) = Item(parse(Int, str))


"""Flush `seen_items` into `elves`."""
function _flush!(elves, seen_items)
    isempty(seen_items) && return seen_items
    push!(elves, Elf(seen_items))
    return Item[]
end

function Base.parse(::Type{Vector{Elf}}, input::AbstractString)
    elves = Elf[]
    seen_items = Item[]

    for line in split(input, '\n')
        if isempty(line)
            seen_items = _flush!(elves, seen_items)
            continue
        end
        item = parse(Item, line)
        push!(seen_items, item)

    end
    seen_items = _flush!(elves, seen_items)
    return elves
end


function get_ans(input::AbstractString)
    elves = parse(Vector{Elf}, input)
    return maximum(calories, elves)
end

example = """
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"""

@assert get_ans(example) == 24000
get_ans(load_input(1))

# Part 2

function get_ans2(input::AbstractString)
    elves = parse(Vector{Elf}, input)
    all_calories = map(calories, elves)
    partialsort!(all_calories, 3; rev=true)
    return sum(all_calories[1:3])
end

@assert get_ans2(example) == 45000
get_ans2(load_input(1))
