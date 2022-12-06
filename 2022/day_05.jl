include("common.jl")

struct Crate
    letter::Char
end

struct Stack
    crates::Vector{Crate}
end
Stack() = Stack(Crate[])

struct Stacks
    data::Vector{Stack}
end

struct Move
    count::Int
    source::Int
    destination::Int
end

function move_top!(from::Stack, to::Stack)
    crate = pop!(from.crates)
    push!(to.crates, crate)
    return nothing
end

function apply!(stacks::Stacks, move::Move, ::Part1)
    source = stacks.data[move.source]
    destination = stacks.data[move.destination]
    for _ in 1:(move.count)
        move_top!(source, destination)
    end
    return stacks
end

function Base.parse(::Type{Crate}, str::AbstractString)
    regex = r"\[([A-Z])\]"
    m = match(regex, str)
    return Crate(only(only(m.captures)))
end

"""Parse stacks. Note that this expects reversed input."""
function Base.parse(::Type{Stacks}, lines::AbstractVector{<:AbstractString})
    stack_indices = map(split(first(lines))) do x
        parse(Int, x)
    end
    n = maximum(stack_indices)
    stacks = [Stack() for _ in 1:n]
    for line in lines[2:end]
        for i in 1:n
            i_start = 1 + (i - 1) * 4
            i_end = i_start + 2
            if i_end > length(line)
                # We are beyond the end of the line - this means that there are no more
                # crates on this row.
                break
            end
            crate_str = line[i_start:i_end]
            crate_str == "   " && continue
            push!(stacks[i].crates, parse(Crate, crate_str))
        end
    end
    return Stacks(stacks)
end

function Base.parse(::Type{Move}, str::AbstractString)
    regex = r"move (\d+) from (\d+) to (\d+)"
    m = match(regex, str)
    return Move(map(capture -> parse(Int, capture), m.captures)...)
end

function compute_ans(input::AbstractString, part::Part)
    moves = Move[]

    lines = split(input, '\n')

    # Trim starting and ending empty lines.
    while isempty(first(lines))
        popfirst!(lines)
    end
    while isempty(last(lines))
        pop!(lines)
    end

    i_empty = only(findall(isempty, lines))
    # NB we parse stacks in reverse order.
    stacks_lines = lines[(i_empty - 1):-1:1]
    moves_lines = lines[(i_empty + 1):end]

    stacks = parse(Stacks, stacks_lines)
    moves = map(moves_lines) do line
        parse(Move, line)
    end

    for move in moves
        apply!(stacks, move, part)
    end

    return join(
        map(stacks.data) do stack
            last(stack.crates).letter
        end,
        "",
    )
end

get_ans(input::AbstractString) = comptue_ans(input, Part1())

@assert parse(Move, "move 3 from 1 to 12") == Move(3, 1, 12)

example = """
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"""

@assert get_ans(example) == "CMZ"
get_ans(load_input(5))

# Part 2

function apply!(stacks::Stacks, move::Move, ::Part2)
    from = stacks.data[move.source]
    to = stacks.data[move.destination]

    i = length(from.crates) - move.count + 1
    append!(to.crates, from.crates[i:end])
    deleteat!(from.crates, i:length(from.crates))
    return stacks
end

get_ans2(input::AbstractString) = compute_ans(input, Part2())

@assert get_ans2(example) == "MCD"
get_ans2(load_input(5))
