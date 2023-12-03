let
    lib = (import <nixpkgs> {}).lib;
    input = builtins.readFile ./input.txt;
    input_list = lib.strings.splitString "\n" input;
    sum_list = lib.lists.forEach input_list (l:
        let 
            chars = lib.stringToCharacters l;
            digits = builtins.filter (c: (builtins.tryEval (lib.toInt c)).success) chars;
            ints = builtins.map (d: lib.toInt d) digits;
            first = lib.lists.head ints;
            last = lib.lists.last ints;
        in 
            first + last
    );
    sum = builtins.foldl' (x: y: x+ y) 0 sum_list;
in
 sum