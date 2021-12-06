include("common.jl")

example = "3,4,3,1,2"

struct FishyBusiness
    counts::Vector{Int64}
    FishyBusiness() = new(zeros(Int64, 9))
end

function Base.parse(::Type{FishyBusiness}, input::AbstractString)
    result = FishyBusiness()
    for value in split(input, ",")
        value = parse(Int64, value)
        i = value + 1
        result.counts[i] += 1
    end
    return result
end

function evolve!(fb::FishyBusiness)
    @inbounds begin
        num_new_lanternfish = first(fb.counts)

        # Bump up the zeros to 7.
        fb.counts[7 + 1] += fb.counts[0 + 1]
        fb.counts[0 + 1] = 0

        # Shift all other numbers down.
        for day in 1:8
            i = day + 1
            fb.counts[i - 1] = fb.counts[i]
        end

        # Now set the count at 8 days to be the number of new lanternfish.
        fb.counts[8 + 1] = num_new_lanternfish
    end
    return fb
end

num_fish(fb::FishyBusiness) = sum(fb.counts)

function num_fish(input::AbstractString, num_days::Integer)
    fb = parse(FishyBusiness, input)
    for _ in 1:num_days
        evolve!(fb)
    end
    return num_fish(fb)
end

get_ans(input::AbstractString) = num_fish(input, 80)

@assert get_ans(example) == 5934
@time get_ans(load_input(6))


# Part 2

get_ans2(input::AbstractString) = num_fish(input, 256)

@assert get_ans2(example) == 26984457539
get_ans2(load_input(6))
