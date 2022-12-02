include("common.jl")

abstract type Play end

struct Rock <: Play end
struct Paper <: Play end
struct Scissors <: Play end

score(::Rock) = 1
score(::Paper) = 2
score(::Scissors) = 3

all_plays() = (Rock(), Paper(), Scissors())

Base.:<(::Scissors, ::Rock) = true
Base.:<(::Rock, ::Paper) = true
Base.:<(::Paper, ::Scissors) = true
Base.:<(::Play, ::Play) = false

struct Round
    opponent::Play
    me::Play
end

function score(round::Round)
    return score(round.me) + (
        6 * (round.opponent < round.me)
        + 3 * (round.opponent == round.me)
    )
end

score(rounds::AbstractVector{Round}) = sum(score, rounds)

function Base.parse(::Type{Play}, str::AbstractString)
    return if str == "A"
        Rock()
    elseif str == "B"
        Paper()
    elseif str == "C"
        Scissors()
    else
        throw(ArgumentError("Can't parse $str as a play"))
    end
end

function parse_my_play(str::AbstractString, ::Play, ::Part1)
    return if str == "X"
        Rock()
    elseif str == "Y"
        Paper()
    elseif str == "Z"
        Scissors()
    else
        throw(ArgumentError("Can't parse $str as a play"))
    end
end

function parse_my_play(str::AbstractString, opponent::Play, ::Part2)
    return if str == "X"
        # We must lose
        only(filter(<(opponent), all_plays()))
    elseif str == "Y"
        # We must draw
        opponent
    elseif str == "Z"
        # We must win
        only(filter(>(opponent), all_plays()))
    else
        throw(ArgumentError("Can't parse $str as a play"))
    end
end

function Base.parse(::Type{Round}, line::AbstractString, part::Part)
    str_op, str_me = split(line, ' ')
    opponent = parse(Play, str_op)
    me = parse_my_play(str_me, opponent, part)
    return Round(opponent, me)
end

function Base.parse(::Type{Vector{Round}}, input::AbstractString, part::Part)
    rounds = Round[]
    for line in split(input, '\n')
        isempty(line) && continue
        push!(rounds, parse(Round, line, part))
    end
    return rounds
end

function compute_ans(input::AbstractString, part::Part)
    rounds = parse(Vector{Round}, input, part)
    return score(rounds)
end

get_ans(input::AbstractString) = compute_ans(input, Part1())

example = """
A Y
B X
C Z
"""

@assert get_ans(example) == 15
get_ans(load_input(2))

get_ans2(input::AbstractString) = compute_ans(input, Part2())

@assert parse_my_play("X", Rock(), Part2()) == Scissors()
@assert parse_my_play("Y", Rock(), Part2()) == Rock()
@assert parse_my_play("Z", Rock(), Part2()) == Paper()

@assert get_ans2(example) == 12
get_ans2(load_input(2))
