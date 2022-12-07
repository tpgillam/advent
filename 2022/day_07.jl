include("common.jl")

using AbstractTrees

abstract type Item end

struct File <: Item
    name::String
    size::Int
end

struct Directory <: Item
    name::String
    items::Vector{Item}
    Directory(name::AbstractString) = new(name, Item[])
end

struct FileSystem
    root::Directory
    directory_to_parent::Dict{Directory,Directory}
    FileSystem() = new(Directory("/"), Dict{Directory,Directory}())
end

AbstractTrees.children(dir::Directory) = dir.items
AbstractTrees.nodevalue(item::Item) = name(item)

name(item::Item) = item.name
size(file::File) = file.size
size(directory::Directory) = sum(size, directory.items)
size(fs::FileSystem) = size(fs.root)

function cd(fs::FileSystem, current_dir::Directory, target::AbstractString)
    return if target == ".."
        fs.directory_to_parent[current_dir]
    elseif target == "/"
        fs.root
    else
        only(
            filter(current_dir.items) do item
                isa(item, File) && return false
                name(item) == target || return false
                return true
            end,
        )
    end
end

function add_child!(::FileSystem, parent::Directory, file::File)
    push!(parent.items, file)
    return parent
end

function add_child!(fs::FileSystem, parent::Directory, directory::Directory)
    push!(parent.items, directory)
    fs.directory_to_parent[directory] = parent
    return parent
end

function Base.parse(::Type{Item}, str::AbstractString)
    return if startswith(str, "dir")
        Directory(replace(str, "dir " => ""))
    else
        size_str, name = split(str, ' ')
        File(name, parse(Int, size_str))
    end
end

function Base.parse(::Type{FileSystem}, str::AbstractString)
    fs = FileSystem()
    current_dir = fs.root
    for line in split(str, '\n')
        isempty(line) && continue
        if startswith(line, raw"$")
            command = replace(line, raw"$ " => "")
            command == "ls" && continue
            startswith(command, "cd") || error("Unexpected command $command")
            target = replace(command, "cd " => "")
            current_dir = cd(fs, current_dir, target)
        else
            item = parse(Item, line)
            add_child!(fs, current_dir, item)
        end
    end
    return fs
end

function get_ans(input::AbstractString)
    fs = parse(FileSystem, input)
    return sum(PreOrderDFS(fs.root)) do item
        isa(item, Directory) || return 0
        dirsize = size(item)
        return dirsize <= 100000 ? dirsize : 0
    end
end

example = raw"""
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"""

@assert get_ans(example) == 95437
get_ans(load_input(7))


# Part 2

function get_ans2(input::AbstractString)
    fs = parse(FileSystem, input)
    free_space = 70000000 - size(fs.root)
    minimum_deletion_size = 30000000 - free_space
    best_size = 0
    for item in PreOrderDFS(fs.root)
        isa(item, Directory) || continue
        dirsize = size(item)
        dirsize < minimum_deletion_size && continue
        best_size = if iszero(best_size)
            dirsize
        else
            min(best_size, dirsize)
        end
    end
    return best_size
end

@assert get_ans2(example) == 24933642
get_ans2(load_input(7))
