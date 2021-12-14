using DataStructures
using StatsBase

include("common.jl")

example = """
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"""

mutable struct Problem
    template::String
    rules::Trie{UInt8}
    previous_step::Int64

    Problem(template, rules) = new(template, rules, 0)
end

function Base.parse(::Type{Problem}, input::AbstractString)
    template = nothing
    rules = Trie{UInt8}()

    for line in split(input, '\n')
        isempty(line) && continue
        if isnothing(template)
            template = line
        else
            from, to = split(line, " -> ")
            rules[from] = only(to)
        end
    end

    return Problem(template, rules)
end

function step!(problem::Problem)
    template = problem.template
    new_template = UInt8[]

    push!(new_template, UInt8(first(template)))
    @inbounds for i in 1:(length(template) - 1)
        substring = template[i:i + 1]
        extra = get(problem.rules, substring, nothing)
        !isnothing(extra) && push!(new_template, extra)
        push!(new_template, UInt8(last(substring)))
    end

    problem.template = String(new_template)
    problem.previous_step += 1
    return problem
end

function get_discrepancy!(problem::Problem, num_steps::Integer)
    for _ in 1:num_steps
        step!(problem)
    end

    char_to_count = countmap(problem.template)
    min_count, max_count = extrema(pair -> last(pair), char_to_count)
    return max_count - min_count
end

get_ans(input::AbstractString) = get_discrepancy!(parse(Problem, input), 10)

@assert get_ans(example) == 1588
get_ans(load_input(14))

# Part 2

# Ah ok, so we don't actually care about the ordering of stuff. Try again!

struct PairCount
    pair_counts::Dict{Tuple{UInt8,UInt8},Int64}
end

PairCount() = PairCount(Dict{Tuple{UInt8,UInt8},Int64}())

function add_pair!(pc::PairCount, pair::Tuple{UInt8,UInt8}, count)
    add_pair!(pc.pair_counts, pair, count)
    return pc
end

function add_pair!(pc::Dict{Tuple{UInt8,UInt8}}, pair::Tuple{UInt8,UInt8}, count)
    setindex!(pc, get!(pc, pair, 0) + count, pair)
    return pc
end

remove_pair!(pc::PairCount, pair) = remove_pair!(pc.pair_counts, pair)
function remove_pair!(pc::Dict{Tuple{UInt8,UInt8}}, pair::Tuple{UInt8,UInt8})
    count = get!(pc, pair, 0)
    pc[pair] = 0
    return count
end

struct Rule
    pair::Tuple{UInt8,UInt8}
    additional::UInt8
end

function apply_rules!(pc::PairCount, rules::AbstractVector{Rule})
    to_add = PairCount()

    for rule in rules
        !haskey(pc.pair_counts, rule.pair) && continue
        count = remove_pair!(pc, rule.pair)

        add_pair!(to_add, (first(rule.pair), rule.additional), count)
        add_pair!(to_add, (rule.additional, last(rule.pair)), count)
    end

    for (pair, count) in to_add.pair_counts
        add_pair!(pc, pair, count)
    end

    return pc
end

struct ProblemState
    pair_count::PairCount
    rules::Vector{Rule}
    exterior_letters::Tuple{UInt8,UInt8}
end

function Base.parse(::Type{ProblemState}, input::AbstractString)
    pc = nothing
    rules = Rule[]
    exterior_letters = nothing

    for line in split(input, '\n')
        isempty(line) && continue
        if isnothing(pc)
            pc = PairCount()
            @inbounds for i in 1:(length(line) - 1)
                add_pair!(pc, UInt8.((line[i], line[i + 1])), 1)
            end
            exterior_letters = @inbounds (first(line), last(line))
        else
            from, additional = split(line, " -> ")
            pair = Tuple(UInt8.(collect(from)))
            push!(rules, Rule(pair, only(additional)))
        end
    end

    return ProblemState(pc, rules, exterior_letters)
end

step!(ps::ProblemState) = apply_rules!(ps.pair_count, ps.rules)

function string_length(ps::ProblemState)
    # The number of letters in the string is one more than the number of pairs.
    num_pairs = sum(values(ps.pair_count.pair_counts))
    return num_pairs + 1
end

function get_string_length_after(input::AbstractString, n::Integer)
    ps = parse(ProblemState, input)
    for _ in 1:n
        step!(ps)
    end
    return string_length(ps)
end

function letter_count_diff(ps::ProblemState)
    letter_count = Dict{UInt8,Int64}()
    for (pair, count) in ps.pair_count.pair_counts
        for letter in pair
            letter_count[letter] = get!(letter_count, letter, 0) + count
        end
    end

    # Once we are here, we will have double-counted every *interior* letter.
    # Since the exterior letters never change, we can special-case these.
    for letter in ps.exterior_letters
        letter_count[letter] = get!(letter_count, letter, 0) + 1
    end

    # Now we have double-counted every letter, so remember to divide by two.
    twice_min, twice_max = extrema(values(letter_count))
    return Int64((twice_max - twice_min) / 2)
end

@assert get_string_length_after(example, 5) == 97
@assert get_string_length_after(example, 10) == 3073

function get_discrepancy!(ps::ProblemState, num_steps::Integer)
    for _ in 1:num_steps
        step!(ps)
    end

    return letter_count_diff(ps)
end

# Verify part 1 with the new implementation.
get_discrepancy!(parse(ProblemState, example), 10) == 1588

get_ans2(input::AbstractString) = get_discrepancy!(parse(ProblemState, input), 40)

@assert get_ans2(example) == 2188189693529
get_ans2(load_input(14))
