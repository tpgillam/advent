using Combinatorics
using StaticArrays

include("common.jl")

struct Digit
    # Record whether each segment is active.
    segments::SVector{7,Bool}
end

function Base.parse(::Type{Digit}, code::AbstractString)
    result = MVector{7,Bool}(undef)
    fill!(result, false)
    for char in code
        char_index = Int64(char) - Int64('a') + 1
        result[char_index] = true
    end
    return Digit(SVector(result))
end

struct Display
    digits::Vector{Digit}
end

function Base.parse(::Type{Display}, input::AbstractString)
    digits = Digit[]
    for digit in split(strip(input), ('|', ' '))
        isempty(digit) && continue
        push!(digits, parse(Digit, digit))
    end
    return Display(digits)
end


real_digits = [
    parse(Digit, "abcefg"),  # 0
    parse(Digit, "cf"),
    parse(Digit, "acdeg"),
    parse(Digit, "acdfg"),
    parse(Digit, "bcdf"),
    parse(Digit, "abdfg"),
    parse(Digit, "abdefg"),
    parse(Digit, "acf"),
    parse(Digit, "abcdefg"),
    parse(Digit, "abcdfg"),  # 9
]


# value(digit::Digit) = findfirst(


permute(digit::Digit, perm) = Digit(digit.segments[perm])
permute(display::Display, perm) = Display(map(x -> permute(x, perm), display.digits))

"""Get all the digits that can be parsed. Terminates as soon as invalid digit is found."""
function get_all_numbers(display::Display)
    result = Int64[]
    for digit in display.digits
        index = findfirst(==(digit), real_digits)
        isnothing(index) && return result  # invalid digit!
        push!(result, index - 1)
    end
    return result
end

example = """
acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
"""

function unscramble_display(display::Display)
    for perm in permutations(1:7)  # Only 5040 cases... not too bad to bash.
        this_display = permute(display, perm)
        numbers = get_all_numbers(this_display)
        length(numbers) == 14 && return this_display
    end
    error("Couldn't find correct permutation :(")
end

function get_test_numbers(display::Display)
    all_numbers = get_all_numbers(unscramble_display(display))
    return all_numbers[11:14]
end

example_2 = """
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"""

function get_ans(input::AbstractString)
    num_interesting_digits = 0
    for line in split(input, "\n")
        isempty(line) && continue
        display = parse(Display, line)
        for n in get_test_numbers(display)
            if n in (1, 4, 7, 8)
                num_interesting_digits += 1
            end
        end
    end
    return num_interesting_digits
end

@assert get_ans(example_2) == 26
get_ans(load_input(8))

# Part 2

function make_num(digits::Vector{Int64})
    result = 0
    multiplier = 1
    for digit in reverse(digits)
        result += multiplier * digit
        multiplier *= 10
    end
    return result
end

function get_ans2(input::AbstractString)
    total = 0
    for line in split(input, "\n")
        isempty(line) && continue
        display = parse(Display, line)
        number = make_num(get_test_numbers(display))
        total += number
    end
    return total
end

@assert get_ans2(example_2) == 61229
get_ans2(load_input(8))
