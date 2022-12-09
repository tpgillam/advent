include("common.jl")

struct Position
    x::Int
    y::Int
end

abstract type Direction end
struct Up <: Direction end
struct Down <: Direction end
struct Left <: Direction end
struct Right <: Direction end

function Base.parse(::Type{Direction}, str::AbstractString)
    return if str == "U"
        Up()
    elseif str == "D"
        Down()
    elseif str == "L"
        Left()
    elseif str == "R"
        Right()
    else
        throw(ArgumentError("Cannot parse $str as a Direction."))
    end
end

struct Motion
    count::Int
    direction::Direction
end

function Base.parse(::Type{Motion}, str::AbstractString)
    dir_str, amount_str = split(str, ' ')
    return Motion(parse(Int, amount_str), parse(Direction, dir_str))
end

mutable struct State
    head::Position
    tail::Position
    tail_positions::Set{Position}
    State() = new(Position(0, 0), Position(0, 0), Set([Position(0, 0)]))
end

function apply!(state::State, motion::Motion)
    for _ in 1:(motion.count)
        apply!(state, motion.direction)
    end
end

function apply!(state::State, direction::Direction)
    _move_head!(state, direction)
    _update_tail!(state)
    _save_tail_position!(state)
    return nothing
end

_move(position::Position, ::Up) = Position(position.x, position.y + 1)
_move(position::Position, ::Down) = Position(position.x, position.y - 1)
_move(position::Position, ::Left) = Position(position.x - 1, position.y)
_move(position::Position, ::Right) = Position(position.x + 1, position.y)

_move_head!(state::State, direction::Direction) = state.head = _move(state.head, direction)

function _get_dir_x(delta_x::Int, threshold::Int)
    return if delta_x < -threshold
        Left()
    elseif delta_x > threshold
        Right()
    else
        nothing
    end
end

function _get_dir_y(delta_y::Int, threshold::Int)
    return if delta_y < -threshold
        Down()
    elseif delta_y > threshold
        Up()
    else
        nothing
    end
end

function _push_direction!(collection, dir)
    isnothing(dir) && return nothing
    return push!(collection, dir)
end

function _get_new_tail_position(head::Position, tail::Position)
    delta_x = head.x - tail.x
    delta_y = head.y - tail.y

    directions = Set{Direction}()
    _push_direction!(directions, _get_dir_x(delta_x, 1))
    _push_direction!(directions, _get_dir_y(delta_y, 1))
    abs(delta_x) > 1 && _push_direction!(directions, _get_dir_y(delta_y, 0))
    abs(delta_y) > 1 && _push_direction!(directions, _get_dir_x(delta_x, 0))

    return foldl(directions; init=tail) do position, direction
        _move(position, direction)
    end
end

function _update_tail!(state::State)
    state.tail = _get_new_tail_position(state.head, state.tail)
    return state
end

function _save_tail_position!(state::State)
    push!(state.tail_positions, state.tail)
    return state
end

function get_ans(input::AbstractString)
    state = State()
    for line in split(input, '\n')
        isempty(line) && continue
        motion = parse(Motion, line)
        apply!(state, motion)
    end
    return length(state.tail_positions)
end

example = """
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"""

@assert get_ans(example) == 13
get_ans(load_input(9))

# Part 2

mutable struct State2
    rope::Vector{Position}
    tail_positions::Set{Position}
    State2(num_knots::Int) = new(repeat([Position(0, 0)], num_knots), Set([Position(0, 0)]))
end

function apply!(state::State2, motion::Motion)
    for _ in 1:(motion.count)
        apply!(state, motion.direction)
    end
end

function apply!(state::State2, direction::Direction)
    _move_head!(state, direction)
    _update_tail!(state)
    _save_tail_position!(state)
    return state
end

function _move_head!(state::State2, direction::Direction)
    head = first(state.rope)
    return state.rope[1] = _move(head, direction)
end

function _update_tail!(state::State2)
    for i in 2:length(state.rope)
        head = state.rope[i - 1]
        tail = state.rope[i]
        state.rope[i] = _get_new_tail_position(head, tail)
    end
    return state
end

function _save_tail_position!(state::State2)
    push!(state.tail_positions, last(state.rope))
    return state
end

function compute_ans(input::AbstractString, tail_length::Int)
    state = State2(tail_length)
    for line in split(input, '\n')
        isempty(line) && continue
        motion = parse(Motion, line)
        apply!(state, motion)
    end
    return length(state.tail_positions)
end

get_ans2(input::AbstractString) = compute_ans(input, 10)

@assert compute_ans(example, 2) == 13
@assert get_ans2(example) == 1

example2 = """
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"""

@assert get_ans2(example2) == 36
get_ans2(load_input(9))
