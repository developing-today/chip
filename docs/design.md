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
