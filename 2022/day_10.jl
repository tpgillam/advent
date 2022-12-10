include("common.jl")

abstract type Instruction end
struct Noop <: Instruction end
struct AddX <: Instruction
    V::Int
end

function Base.parse(::Type{Instruction}, str::AbstractString)
    return if str == "noop"
        Noop()
    elseif startswith(str, "addx ")
        AddX(parse(Int, split(str, " ")[2]))
    else
        throw(ArgumentError("Unknown instruction $str"))
    end
end

struct CPU
    register_values::Vector{Int}
    CPU() = new([1])
end

register(cpu::CPU) = last(cpu.register_values)
clock(cpu::CPU) = length(cpu.register_values)

signal_strengths(cpu::CPU) = (1:length(cpu.register_values)) .* cpu.register_values

function _tick!(cpu::CPU, value::Int)
    push!(cpu.register_values, value)
    return cpu
end

apply!(cpu::CPU, ::Noop) = _tick!(cpu, register(cpu))
function apply!(cpu::CPU, instruction::AddX)
    _tick!(cpu, register(cpu))
    return _tick!(cpu, register(cpu) + instruction.V)
end

function Base.parse(::Type{CPU}, str::AbstractString)
    cpu = CPU()
    for line in split(str, '\n')
        isempty(line) && continue
        instruction = parse(Instruction, line)
        apply!(cpu, instruction)
    end
    return cpu
end

function get_ans(input::AbstractString)
    cpu = parse(CPU, input)
    interesting_cycles = 20:40:220
    return sum(signal_strengths(cpu)[interesting_cycles])
end

example_small = """
noop
addx 3
addx -5
"""
@assert parse(CPU, example_small).register_values == [1, 1, 1, 4, 4, -1]

example = """
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"""

@assert get_ans(example) == 13140
get_ans(load_input(10))

# Part 2

function render(cpu::CPU)
    for clock in 1:240
        x = cpu.register_values[clock]
        position = ((clock - 1) % 40)
        position == 0 && print('\n')
        pixel = abs(x - position) <= 1 ? '#' : ' '
        print(pixel)
    end
    return print('\n')
end

function display_ans2(input::AbstractString)
    cpu = parse(CPU, input)
    render(cpu)
    return nothing
end

display_ans2(example)

display_ans2(load_input(10))
