using Statistics

include("common.jl")

example = """
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"""

abstract type Status end
struct Complete <: Status end
struct Incomplete <: Status
    stack::Vector{Char}
end
struct Corrupted <: Status
    char::Char
end

isopen(char::Char) = char in ('(', '[', '{', '<')

function toopen(char::Char)
    return if char == ')'
        '('
    elseif char == ']'
        '['
    elseif char == '}'
        '{'
    elseif char == '>'
        '<'
    else
        throw(ArgumentError("Unsupported char $char"))
    end
end

function get_status(line::AbstractString)
    stack = Char[]
    for char in line
        if isopen(char)
            push!(stack, char)
            continue
        else
            # The close char should match what we pop off the stack.
            last = pop!(stack)
            last == toopen(char) || return Corrupted(char)
        end
    end

    return isempty(stack) ? Complete() : Incomplete(stack)
end

function score(s::Corrupted)
    return if s.char == ')'
        3
    elseif s.char == ']'
        57
    elseif s.char == '}'
        1197
    elseif s.char == '>'
        25137
    else
        throw(ArgumentError("Unsupported char $char"))
    end
end

function get_ans(input::AbstractString)
    total = 0
    for line in split(input, '\n')
        isempty(line) && continue
        status = get_status(line)
        isa(status, Corrupted) || continue
        total += score(status)
    end
    return total
end

@assert get_ans(example) == 26397
get_ans(load_input(10))

# Part 2

function score(s::Incomplete)
    total = 0
    for char in Iterators.reverse(s.stack)
        # Unwind the stack.
        total *= 5
        total += if char == '('
            1
        elseif char == '['
            2
        elseif char == '{'
            3
        elseif char == '<'
            4
        end
    end
    return total
end

function get_ans2(input::AbstractString)
    scores = Int64[]
    for line in split(input, '\n')
        isempty(line) && continue
        status = get_status(line)
        isa(status, Incomplete) || continue
        push!(scores, score(status))
    end
    @assert length(scores) % 2 == 1
    return Int64(median!(scores))
end

@assert get_ans2(example) == 288957
get_ans2(load_input(10))
