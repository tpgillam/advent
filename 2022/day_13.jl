include("common.jl")

struct Packet
    value::Vector
end

notfalse(::Missing) = true
notfalse(b::Bool) = b

"""
    ordered(left, right)

Return `true` iff `left` and `right` are in the correct order.
"""
ordered(left::Packet, right::Packet) = notfalse(ordered(left.value, right.value))
function ordered(left::Int, right::Int)
    left == right && return missing
    return left < right
end
function ordered(left::AbstractVector, right::AbstractVector)
    for (l, r) in zip(left, right)
        result = ordered(l, r)
        ismissing(result) && continue
        return result
    end

    # Equal length inputs => undetermined.
    length(left) == length(right) && return missing

    # A shorter left list => good input, shorter right list => bad input.
    return length(left) < length(right)
end
ordered(left::AbstractVector, right::Int) = ordered(left, [right])
ordered(left::Int, right::AbstractVector) = ordered([left], right)

Base.parse(::Type{Packet}, str::AbstractString) = Packet(eval(Meta.parse(str)))

struct PacketPair
    first::Packet
    second::Packet
end

ordered(pair::PacketPair) = ordered(pair.first, pair.second)

function Base.parse(::Type{PacketPair}, str::AbstractString)
    a, b = filter(!isempty, split(str, '\n'))
    return PacketPair(parse(Packet, a), parse(Packet, b))
end

function Base.parse(::Type{Vector{PacketPair}}, str::AbstractString)
    return parse.(Ref(PacketPair), split(str, "\n\n"))
end

function get_ans(input::AbstractString)
    moo = parse(Vector{PacketPair}, input)
    return sum(findall(ordered, moo))
end

example = """
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"""

@assert get_ans(example) == 13
get_ans(load_input(13))

# Part 2

function get_ans2(input::AbstractString)
    packets = parse.(Ref(Packet), filter(!isempty, split(input, '\n')))
    dividers = [Packet([[2]]), Packet([[6]])]
    append!(packets, dividers)
    sort!(packets; lt=ordered)
    i_divider_1 = only(findall(==(dividers[1]), packets))
    i_divider_2 = only(findall(==(dividers[2]), packets))
    return i_divider_1 * i_divider_2
end

@assert get_ans2(example) == 140
get_ans2(load_input(13))
