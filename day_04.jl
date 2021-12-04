include("common.jl")

example = """
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"""

struct Bingo
    order::Vector{Int64}
    size::Tuple{Int64,Int64}
    boards::Vector{Matrix{Int64}}
end

function _gobble!(x::AbstractVector{<:AbstractString})
    while true
        isempty(x) && break
        isempty(@inbounds(first(x))) || break
        popfirst!(x)
    end
    return x
end

function Base.parse(::Type{Bingo}, input::AbstractString)
    lines = split(input, "\n")
    line = popfirst!(lines)
    order = map(x -> parse(Int, x), split(line, ","))

    _gobble!(lines)
    boards = Matrix{Int64}[]
    while !isempty(lines)
        rows = Vector{Int64}[]

        while true
            isempty(lines) && break
            line = popfirst!(lines)
            isempty(line) && break  # End of this board
            push!(rows, parse.(Int, split(line)))
        end

        # Assemble board
        push!(boards, vcat(transpose.(rows)...))

        _gobble!(lines)
    end

    size_ = only(unique(map(size, boards)))
    return Bingo(order, size_, boards)
end

function get_winning_score(bingo::Bingo)
    markers = [zeros(bingo.size) for _ in 1:length(bingo.boards)]
    row_markers = [zeros(bingo.size[1]) for _ in 1:length(bingo.boards)]
    col_markers = [zeros(bingo.size[2]) for _ in 1:length(bingo.boards)]

    for number in bingo.order
        @inbounds for i_board in 1:length(bingo.boards)
            board = bingo.boards[i_board]
            marker = markers[i_board]
            row_marker = row_markers[i_board]
            col_marker = col_markers[i_board]

            for index in findall(==(number), board)
                marker[index] = 1
                i_row, i_col = Tuple(index)
                row_marker[i_row] += 1
                col_marker[i_col] += 1
            end

            # Figure out if board is complete. If so, compute the score.
            complete = (
                any(==(bingo.size[1]), row_marker) ||
                any(==(bingo.size[2]), col_marker)
            )
            complete || continue

            # We have finished! compute the score and return it.
            unmarked_sum = sum(board .* (marker .< 1))
            return unmarked_sum * number
        end
    end
end


bingo = parse(Bingo, example)
@assert get_winning_score(bingo) == 4512

get_winning_score(parse(Bingo, load_input(4)))
