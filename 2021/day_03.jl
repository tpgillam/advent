include("common.jl")

function get_inputs(input::AbstractString)
    result = Int[]
    for line in split(input, "\n")
        isempty(line) && continue
        push!(result, parse(Int, line; base=2))
    end
    return result
end

# From: https://discourse.julialang.org/t/parse-an-array-of-bits-bitarray-to-an-integer/42361/24
function bitarr_to_int(arr, val=0)
    v = 2^(length(arr) - 1)
    @inbounds for i in eachindex(arr)
        val += v * arr[i]
        v >>= 1
    end
    return val
end

num_bits(n::Integer) = 8 * sizeof(n) - leading_zeros(n)

function get_gamma_epsilon(input::AbstractString)
    inputs = get_inputs(input)
    max_n = maximum(num_bits, inputs)
    masks = ones(Int64, max_n)
    @inbounds for i in 1:max_n
        masks[i] <<= (max_n - i)
    end

    count = 0
    masked_counts = zeros(Int, size(masks))
    for input in inputs
        count += 1
        @inbounds for i in 1:length(masks)
            mask = masks[i]
            masked_counts[i] += (mask & input) > 0
        end
    end

    any(masked_counts .== (count .- masked_counts)) && error("AMBIGUOUS!")
    gamma = bitarr_to_int(masked_counts .> (count .- masked_counts))
    epsilon = bitarr_to_int(masked_counts .< (count .- masked_counts))
    return gamma, epsilon
end

function get_ans(input::AbstractString)
    gamma, epsilon = get_gamma_epsilon(input)
    return gamma * epsilon
end

example = """
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"""

@assert get_ans(example) == 198
get_ans(load_input(3))

# Part 2

function filter_input(input::AbstractString, most_common::Bool)
    inputs = get_inputs(input)
    max_n = maximum(num_bits, inputs)
    for bit in max_n:-1:1
        println(bit, " ", inputs)
        inputs = filter_inputs(inputs, bit, most_common)
        length(inputs) == 1 && break
    end
    length(inputs) == 1 || error("Failed to find input")
    return only(inputs)
end

function filter_inputs(inputs::AbstractVector{<:Integer}, bit::Int, most_common::Bool)
    bit > 0 || throw(ArgumentError("Bit should be positive, got $bit"))
    mask = 1 << (bit - 1)
    count = length(inputs)
    masked_count = sum((mask .& inputs) .> 0)
    # masked_count == (count - masked_count) && error("AMBIGUITY!")

    bit = if most_common
        masked_count >= (count - masked_count)
    else
        masked_count < (count - masked_count)
    end

    println("BIT:  ", bit)
    println("MASK:  ", mask)
    println(mask .& inputs)
    println()

    return filter(inputs) do input
        (input & mask > 0) == bit
    end
end

function get_ans2(input)
    o2_rating = filter_input(input, true)
    co2_rating = filter_input(input, false)
    return o2_rating * co2_rating
end

@assert get_ans2(example) == 230
get_ans2(load_input(3))
