include("common.jl")

example = """
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"""

function to_matrix(input::AbstractString)
    rows = Vector{Int64}[]
    for line in split(input, '\n')
        isempty(line) && continue
        push!(rows, [parse(Int64, x) for x in line])
    end
    return transpose(reduce(hcat, rows)) |> collect
end

function apply_flashes!(x::AbstractMatrix{Int64}, flashed::AbstractMatrix{Bool})
    @assert size(x) == size(flashed)
    ni = size(flashed, 1)
    nj = size(flashed, 2)
    @inbounds for i in 1:ni
        for j in 1:nj
            flashed[i, j] || continue

            # Update neighbours.
            for oi in max(1, (i - 1)):min((i + 1), ni)
                for oj in max(1, (j - 1)):min((j + 1), nj)
                    # Skip the thing that has flashed.
                    oi == i && oj == j && continue
                    x[oi, oj] += 1
                end
            end
        end
    end
    return x
end

function step!(x::AbstractMatrix{Int64})
    x .+= 1  # Energy level increases.

    flashed = zeros(Bool, size(x))
    while true
        # An octopus shouldn't flash more than once.
        this_flashed = @. ((x > 9) & (!flashed))
        sum(this_flashed) == 0 && break  # Done flashing
        apply_flashes!(x, this_flashed)
        flashed .|= this_flashed  # Update the record of all flashes.
    end

    x[flashed] .= 0
    return sum(flashed)
end

function evolve!(x::AbstractMatrix, num_steps)
    total = 0
    for _ in 1:num_steps
        total += step!(x)
    end
    return total
end

function get_ans(input::AbstractString, n=100)
    x = to_matrix(input)
    return evolve!(x, n)
end

@assert get_ans(example, 10) == 204
@assert get_ans(example) == 1656
get_ans(load_input(11))


# Part 2

function get_ans2(input::AbstractString)
    x = to_matrix(input)
    step = 1
    while true
        num_flashes = step!(x)
        num_flashes == length(x) && return step
        step += 1
    end
end

@assert get_ans2(example) == 195
get_ans2(load_input(11))
