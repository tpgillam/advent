# Part 1

mutable struct Position
    horizontal::Int64
    depth::Int64
end
Position() = Position(0, 0)

function move!(position::Position, direction::Symbol, amount::Integer)
    if direction == :forward
        position.horizontal += amount
    elseif direction == :down
        position.depth += amount
    elseif direction == :up
        position.depth -= amount
    end
end

function parse_line(line::AbstractString)
    direction, amount = split(line, " ")
    return (Symbol(direction), parse(Int64, amount))
end

function get_final_position(commands::AbstractString, position=Position())
    for line in split(commands, "\n")
        isempty(line) && continue
        direction, amount = parse_line(line)
        move!(position, direction, amount)
    end
    return position
end

"""Multiply two components to get final answer."""
get_ans(position::Position) = position.horizontal * position.depth
function get_ans(commands::AbstractString, initial=Position())
    return get_ans(get_final_position(commands, initial))
end

example = """
forward 5
down 5
forward 8
up 3
down 8
forward 2
"""

@assert get_ans(example) == 150

function _load_input(name::AbstractString)
    path = joinpath("inputs", name)
    return read(path, String)
end

commands = _load_input("day_02.txt")
get_ans(commands)

# Part 2

mutable struct PositionAim
    horizontal::Int64
    depth::Int64
    aim::Int64
end
PositionAim() = PositionAim(0, 0, 0)

get_ans(position::PositionAim) = position.horizontal * position.depth

function move!(state::PositionAim, direction::Symbol, amount::Integer)
    if direction == :forward
        state.horizontal += amount
        state.depth += state.aim * amount
    elseif direction == :down
        state.aim += amount
    elseif direction == :up
        state.aim -= amount
    end
end

@assert get_ans(example, PositionAim()) == 900

get_ans(commands, PositionAim())
