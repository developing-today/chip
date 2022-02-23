chip (chip):
runs byte code, emulates screen, timers, accepts input
  todo: parallelize
  clear separation for
    chip ops
    superchip
    custom just leaving the chip concept behind ops
    is there a way to allow chipLox->chip->rust exports
      or just chipLox->rust
      removing the emulation and using bevy directly.
    for beginners upgrading to their own 'real' projects
      they are sentimental and won't want to lose their 'serious progress'
  bytecode just contains memory data and cpu assuming 0x4 start point
    find format representation for saving memory stack registers counter pointer i delay sound
    use sentinel to indicate end of memory? put non-memory after?
      would these files be incompatible with other emulatorS?
      ensure emulator compatibility
      maybe save state files
      could allow for mutated memory output
      final memory could have a clean compile to output cross-compat memory
  accepts
    binary with chip embedded .clox.exe
    binary repr bytecode .clox.bin (1010110101 (which means 0A A2 F4 F5 B2 1C))
    bytecode (0A A2 F4 F5 B2 1C) .clox.hex
    lox source file (consider ext .clox, not final)
    op/asm source file (consider ext .clox, not final)
    some sort of op code repl?
    some sort of asm repl?
     chip 64, 6502, core wars all versions
     get lang built compile on all of these plus native, then add/replace IR inkject llvm somewhere
  flags:
    cycles (optional N cycles to run then pause)
    verbose (println eery op)
chiplox (chipl | chipc)
generates executables (byte code, binaries, wasm)
  use real words
  repl
  assume no previous knowledge
  domain-specific language
  domain is making pixel games using chip
  standard language provides tools for easier ops
  base language provides minimal functionality
  ideas like
    lox (duh)
    dada
    rust
    lisp
    visual basic
  interesting languages
    logo
    haskell
    smalltalk
    objC/swift
    powershell
    i want to be able to read apl so that i can never look at it again
    what are smt solvers? are those like sudoku?
  for absolute beginners, but could also be more.

chiploxide (chipi | chipo | chipe)
chip game developing environment
  surrounding the empty chipl window,
  click based tool to update keybindings
  click based pixel sprite generation
  live cpu cycle logs
  debugger with pause, run, step in, step over, forward ~N cycles
  base development is completely able to make a chiplox compatible game
  additional libraries for superchip compatible additional ops
  future support for full bevy access, chiplox only ops
  steam
  export exe, linux binary, chip/superChip/loxChip bytecode, wasm w/simple index.html
  import your own lox or rust code, bevy packages
  let beginners play in the basics until they organically discover & outgrow the constraints
blah names whatever
CHIP LOxIDE
CHIPloxIDE
chiploxide


"There's even a cunning trick you can do by placing the address of the previous branch instruction in the most recently emitted branch instruction, creating a linked list of instructions that need to be updated when the label is finally encountered; this allows the label to be referenced in multiple places without requiring any additional storage. But it does require all the emitted code to be in memory at once. You can't write it to disk on-the-fly, for example. Maybe useful for JITs?"

"This question comes up pretty often. I've written a couple of type checkers from toy languages, and I've hacked on a big static analyzer a little. I could be wrong but I believe that part of the reason type checking doesn't get a lot of attention in the literature is that... it actually is pretty easy.

It's obviously more elaborate if you're doing Hindley-Milner style full type inference with unification. But if you're just doing basic static type checking and local type inference, I think the process is essentially:

Walk all of the type declarations so you can find the set of types and their relations.

Walk all of the functions top level variables and constants and calculate their types.

Now walk all of the code bottom up (do a post-order traversal of the AST). When walking a block, keep track of the local variables and their types. When visiting expressions, leaf nodes are either literal expressions with obvious types, or references to variables. When visiting a call node, look up the function being called and type check the call's arguments against the function's parameter signature.

Generics and inference add some—OK a fair amount of—complexity. But a type checker for a first-order type system is about as complex as a pretty printer."


‘To allow both unary and binary minus in a language we may assume that the lexical analyzer translates
them to different operators. '

4. Postjix operator. analogous with a prefix operator.


https://dl.acm.org/doi/10.1145/3446804.3446846

https://www.engr.mun.ca/~theo/Misc/pratt_parsing.htm
And every other pratt parser website on the internet along with dysktra and lr ll blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah blah



things to and
  bools
  conds
  imports
  draw
  constraints
  types (type parse down, check reverse up)
  make -<numberdata> a neg number not 'unary minus' number
    what is x = --0
  make numbers only cast to string under duress
  if blah then blah else if blah then blah else blah; blah = blah+1++


leftComm[ "=" ] := new Chaining( 10 )
Now let's give "=" some friends

leftComm[ "<" ] := new Chaining( 10 )
leftComm[ "≤" ] := new Chaining( 10 )
leftComm[ ">" ] := new Chaining( 10 )
leftComm[ "≥" ] := new Chaining( 10 )
this is neat, but instead:
new operator(atomicToken)
new operator(atomicToken::Equals, op::LeftRight(10, 9)).chaining(&self)
and then feature flag with operator?

munificent was right, parsers are overrated

http://cs603.cs.ua.edu/lectures/chapter1-intro.pdf
https://github.com/jeanqasaur/learn-programming-languages/blob/master/README.md
https://www.youtube.com/watch?v=Nlqv6NtBXcA&t=1171s
constraint engines???
http://ftp.deas.harvard.edu/techreports/tr-11-05.pdf#toolbar=0

[Function]
1+ number
1- number

(1+ x) is the same as (+ x 1).

(1- x) is the same as (- x 1). Note that the short name may be confusing: (1- x) does not mean 1-x; rather, it means x-1.

Rationale: These are included primarily for compatibility with MacLisp and Lisp Machine Lisp. Some programmers prefer always to write (+ x 1) and (- x 1) instead of (1+ x) and (1- x).
Implementation note: Compiler writers are very strongly encouraged to ensure that (1+ x) and (+ x 1) compile into identical code, and similarly for (1- x) and (- x 1), to avoid pressure on a Lisp programmer to write possibly less clear code for the sake of efficiency. This can easily be done as a source-language transformation.

https://www.cs.cmu.edu/Groups/AI/html/cltl/clm/node125.html


start with dcrawford numbers operator
