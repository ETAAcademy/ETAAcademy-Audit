# ETAAcademy-Audit: 28. Combinational & Sequential Logic Circuits

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>28 CLSL</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>CLSL</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## The Bedrock of Digital Design: Combinational vs Sequential Logic

Combinational and sequential logic are the bedrock of digital design across FPGA/ASIC, computer architecture, and hardware security (TPM, PUF, TEE). Starting from Boolean functions, the functional completeness of NAND and its efficient CMOS realization let us build from basic gates to composite structures, scale to multi‑bit datapaths and multi‑way selectors, and ultimately assemble a purely combinational ALU controlled by a small set of bits that carries out diverse arithmetic and logic operations and produces status flags.

Bringing in a clock and D flip‑flops makes these memoryless networks stateful, following the next‑state model: registers, the program counter, and RAM (formed from register arrays plus address decoding) sustain the loop of state → combinational compute → next state → clocked write, with familiar behaviors like PC priority (reset > load > inc > hold) and register load/hold semantics.

On the software side, a clean pipeline bridges high‑level code to hardware primitives: the compiler frontend emits a simple, portable VM instruction set, the VM translator lowers it to concrete assembly, the assembler converts symbolic code to 0/1 machine words, and the operating system provides service routines for screen/keyboard (via memory‑mapped I/O) along with common string, math, and memory‑management functions.

In the end, the data, address, and control buses drive the fetch–execute cycle—ROM supplies instructions, RAM holds data, and memory‑mapped I/O connects devices—so high‑level programs run predictably and reliably on hardware built from logic gates.

---

## 1. **From Boolean Functions to Hardware: Gates, Transistors, and Modular Digital Design**

All hardware circuits are, at their core, physical implementations of **Boolean functions**. Digital systems—from simple logic gates to full CPUs—can ultimately be described, analyzed, and constructed using Boolean algebra, truth tables, and combinations of fundamental logic components.

This article walks through the conceptual hierarchy:
**Boolean functions → gate logic → composite circuits → multi-bit and multi-way structures → full computer architecture.**

---

### 1.1 Boolean Functions and Truth Tables

Every digital circuit computes a Boolean function.
A Boolean function with _n_ inputs has **2ⁿ possible input combinations**, each producing a 0 or 1.

Example: A function f(x, y, z):

```
x y z | f
--------------
0 0 0 | 0
0 0 1 | 0
0 1 0 | 1
...
```

This **truth table** is the most direct representation of the function.

#### Boolean Expressions

Any Boolean function can also be expressed using logical operators:

- `·` = AND
- `+` = OR
- `~x` = NOT

Example:

$f(x, y, z) = (x + y) \cdot z$

Thus, **any hardware logic block can be expressed using AND, OR, NOT**, and every expression corresponds to a circuit.

#### Two-Input Boolean Functions (16 in total)

For two input variables (x, y), there are 4 combinations, and each output can be 0 or 1.
Hence:

$2^{2^2} = 16 \text{ possible Boolean functions}$

Examples include:

| Name        | Meaning      |
| ----------- | ------------ |
| AND         | x·y          |
| OR          | x+y          |
| XOR         | exclusive OR |
| NOR         | ~(x+y)       |
| NAND        | ~(x·y)       |
| Equivalence | x == y       |
| Implication | x → y        |

---

#### NAND as a Functionally Complete Primitive

The NAND gate is **functionally complete**, meaning **any** Boolean function and **any** digital circuit can be built using **only NAND gates**.

Examples:

- **NOT**: $\text{NOT } x = x \text{ NAND } x$

- **AND**: $x \cdot y = \text{NOT}(x \text{ NAND } y)$

- **OR** (via De Morgan): $x + y = ~(~x \cdot ~y)$

All expressible using NAND.

This theoretical property also aligns with **real chip design practices**: almost all CMOS logic can be implemented with a small set of standard cells, of which NAND is one of the simplest and most power-efficient.

#### Logic Gates as Hardware Devices

A gate is a physical device that implements a Boolean function. It has:

- input pins
- output pins
- an internal transistor network

Examples: AND, OR, NOT, NAND, NOR.

We treat each gate as a **black box** implementing:

$f(a, b, ...)$

#### Transistor-Level Implementation (CMOS)

In CMOS technology:

- A **NAND gate uses 4 transistors**
- Two **parallel PMOS** pull the output high
- Two **series NMOS** pull the output low

Simplified structure:

```case
     Vdd (1)
      |
    ┌────┐  PMOS (A)
    └────┘
      |-------> OUT
    ┌────┐  PMOS (B)
    └────┘
      |
     -----
      |
    ┌────┐  NMOS (A)
    └────┘
      |
    ┌────┐  NMOS (B)
    └────┘
      |
     GND (0)
```

This efficient topology is why NAND is so dominant in integrated circuits.

#### Basic (1-bit) Gates

These include:

| Gate | Function                            |
| ---- | ----------------------------------- |
| NOT  | invert bit                          |
| AND  | a & b                               |
| OR   | a + b                               |
| XOR  | a ≠ b                               |
| Mux  | select a or b based on `sel`        |
| DMux | route `in` to a or b based on `sel` |

All of them can be constructed purely from NANDs.

#### Primitive vs. Composite Gates

- **Primitive gates**: AND, OR, NOT, NAND, NOR
- **Composite gates**: built from other gates (e.g., a 3-input AND)

Example:
3-input AND:

```
out = AND(AND(a, b), c)
```

The outside world sees it as one logical unit, but inside it’s just a combination of smaller gates.

This compositionality is the core philosophy of digital design:

> Build small gates → use them to build modules → combine modules into a CPU.

#### Multi-bit Gates

Real computers operate on multi-bit values (8, 16, 32, 64 bits).
A multi-bit gate is simply **many 1-bit gates in parallel**.

Example: **And16**

```
for i = 0..15:
    out[i] = And(a[i], b[i])
```

No new logic—just replication.

#### Multi-way Gates

To scale beyond two inputs, we build multi-way selectors.

**Multi-way Mux**

A standard Mux selects between two values.
But CPUs often need 4-way or 8-way multiplexers.

Example: **4-way Mux with 2-bit selector (00,01,10,11)**

Implemented with a tree of 2-way Muxes:

```
Stage 1:
    m1 = Mux(a, b, sel[0])
    m2 = Mux(c, d, sel[0])

Stage 2:
    out = Mux(m1, m2, sel[1])
```

**Multi-way Demux**

DMux distributes one input to one of many outputs.
Example: writing to one of 8 registers → use DMux8Way.

Again implemented through hierarchical combination of 2-way DMuxes.

This hierarchical flow:

**Boolean functions → gates → composite logic → multi-bit circuits → multi-way circuits → full computer systems**

is the foundation of hardware design.

---

### 1.2 HDL: Hardware Source Code for Modern Digital Systems

In modern digital hardware design, engineers no longer assemble circuits by manually wiring AND/OR/NOT gates as in the 1970s. Instead, they write **HDL (Hardware Description Language)**—a high-level textual description of hardware behavior and structure. These HDL files are the _source code_ of the hardware world.

Once written, HDL designs are:

- **Simulated** to verify correctness (just like running a software program).
- **Synthesized** into physical layouts.
- Finally **fabricated** into silicon chips by semiconductor foundries.

HDL virtualizes the entire hardware development process.

#### HDL as Structural Hardware Code

HDL describes hardware composition in terms of **existing components**.
Engineers build larger circuits by combining smaller gates, without ever touching transistor-level details.

For example, a classic 2-input **XOR gate** can be constructed using two NOT gates, two AND gates, and one OR gate:

```hdl
CHIP Xor {
    IN a, b;
    OUT out;

    PARTS:
    Not(in=a, out=nota);
    Not(in=b, out=notb);
    And(a=a, b=notb, out=w1);
    And(a=nota, b=b, out=w2);
    Or(a=w1, b=w2, out=out);
}
```

This HDL implementation exactly matches the logical composition diagram seen in textbooks.

#### Understanding the HDL Structure

HDL files typically contain two parts:

**Header (Interface Declaration)**

This defines the chip’s external interface—its inputs and outputs:

```hdl
CHIP Xor {
    IN a, b;
    OUT out;
```

It is analogous to a function signature in software:
inputs → processed internally → produce outputs.

**PARTS Section (Component Composition)**

This section is where the designer wires together existing modules to form more complex logic.

Example:

- `Not(in=a, out=nota);`
  Instantiates a **Not gate**.
  The signal `a` feeds its input pin, and the result goes into an internal wire `nota`.

- `And(a=nota, b=b, out=w2);`
  Feeds internal wires into other components.

HDL naturally reflects the physical wiring that would exist inside an actual chip.

**Internal Wires and Component Instantiation**

Signals such as `nota`, `notb`, `w1`, and `w2` are **internal wires**.
They do not appear on the chip’s exterior—they only connect internal components.

This mirrors the real world:

- A **Part** in HDL → a **physical gate**
- An internal wire (e.g., `nota`) → an actual piece of circuitry (a metal trace)
- Defining connections in `PARTS:` → specifying how gates are wired together

Thus, HDL is not a program that runs in time; it is a **static wiring diagram** described textually.

**Correspondence Between HDL and Physical Hardware**

A useful mapping summarizing how HDL elements relate to real silicon:

| HDL Concept            | Real Hardware Equivalent              |
| ---------------------- | ------------------------------------- |
| A `PART` instance      | A physical logic gate (AND/NOT/OR)    |
| Internal pins (`nota`) | Internal electrical wires             |
| Input pins (`a`, `b`)  | Chip input pins (external connectors) |
| Output pins (`out`)    | Chip output pins                      |
| PARTS composition      | Actual wiring and logic interconnect  |

In short:

> HDL is the blueprint; the chip is the building.

---

### 1.3 Binary Arithmetic, Adders, Two’s Complement, and the ALU

Modern CPUs perform arithmetic and logic operations on **fixed-width binary numbers**. In a 16-bit system, every number is represented using 16 bits:

```
b15 b14 … b1 b0
```

Each bit has a weight of (2^i), so the value of the binary number is:

$value = \sum_{i=0}^{15} (b_i \times 2^i)$

A **k-bit** unsigned integer can represent values in the range:

$0 \text{to} 2^k - 1$

For example, with 16 bits the maximum is:

$2^{16} - 1 = 65535$

#### Adders: Hardware for Performing Addition

Addition in hardware is carried out using **adders**, which operate on single-bit operands and propagate carries.

**Half Adder**

A half adder takes two input bits (a) and (b), and produces:

- **sum = a XOR b**
- **carry = a AND b**

It can add two bits but cannot accept a carry-in.

**Full Adder**

A full adder extends the half adder by also accepting a **carry-in**:

Inputs:

- (a), (b), (cin)

Outputs:

- sum
- carry-out

Formulas:

$sum = a \oplus b \oplus cin$
$cout = ab + a \cdot cin + b \cdot cin$

**Ripple-Carry 16-bit Adder**

By chaining one half-adder and fifteen full-adders, we obtain a 16-bit adder:

```
LSB                MSB
HA → FA → FA → … → FA
```

The carry “ripples” from bit 0 upward to bit 15.
Arithmetic is performed modulo ($2^{16}$): any final carry past bit 15 is discarded.

This modulo behavior is essential for representing **negative numbers** and implementing **two’s complement arithmetic**.

---

#### Two’s Complement: Representing Negative Numbers

Two’s complement allows hardware to perform both addition and subtraction using the **same adder**.

The mathematical definition:

$-x \equiv 2^n - x \pmod{2^n}$

In a 16-bit system:

$-x = 65536 - x$

Example:

$-3 = 65536 - 3 = 65533 = 0xFFFD$

**Efficient Hardware Rule:**

$-x = \sim x + 1$

This works because:

$2^n - x = (2^n - 1 - x) + 1 = \sim x + 1$

Thus negation is implemented by:

- **bitwise NOT**,
- **plus 1** using the adder.

Subtraction follows directly:

$y - x = y + (-x) = y + (\sim x + 1)$

Hardware never needs a dedicated subtractor.

---

#### The ALU: A Combinational Circuit for Arithmetic and Logic

The **Arithmetic Logic Unit (ALU)** is central to the CPU. It performs both arithmetic operations (add, subtract, increment, decrement) and logical operations (AND, OR, NOT).

In the Hack computer, the ALU takes:

**Inputs**

- (x): 16-bit
- (y): 16-bit
- Six control bits: `zx nx zy ny f no`

**Outputs**

- `out`: 16-bit result
- `zr`: 1 if `out == 0`
- `ng`: 1 if `out` is negative (most significant bit = 1)

These status flags drive conditional jump instructions in the CPU.

#### ALU Control Bits and Their Meaning

The six control bits manipulate the inputs and control the operation:

```
zx = 1 → x := 0
nx = 1 → x := ~x

zy = 1 → y := 0
ny = 1 → y := ~y

f  = 1 → out = x + y
f  = 0 → out = x & y

no = 1 → out := ~out
```

Through combinations of these transformations, the ALU can implement **18 fundamental operations**, such as:

```
0, 1, -1
x, y
!x, !y
-x, -y
x+1, y+1
x-1, y-1
x+y
x-y, y-x
x&y
x|y
```

#### Internal Execution Pipeline (Pure Combinational Logic)

The ALU operates without clocks or registers. All operations occur through a fixed cascade of combinational gates:

```
x0 = (zx ? 0 : x)
x1 = (nx ? ~x0 : x0)

y0 = (zy ? 0 : y)
y1 = (ny ? ~y0 : y0)

o0 = (f ? (x1 + y1) : (x1 & y1))

out = (no ? ~o0 : o0)
```

This structure leverages:

- zeroing (set to 0)
- bitwise negation
- adder output
- bitwise AND
- final negation

Together, this generates all required arithmetic and logical functions.

#### Examples of Clever ALU Tricks

**Producing -1 From Control Bits**

```
zx = 1 → x = 0
nx = 1 → x = ~0 = 1111...1111 = -1
```

**Computing OR Without an OR Gate**

Using De Morgan’s law:

x | y = ~(~x & ~y)

Control bits:

```
nx = 1
ny = 1
f  = 0
no = 1
```

**Computing y - x**

Use two’s complement:

$y - x = y + (\sim x + 1)$

Control bits:

```
nx = 1      → makes ~x
ny = 0      → keeps y
f  = 1      → addition
no = 1      → final +1 via negation rule
```

The ALU is a powerful demonstration of how seemingly simple Boolean primitives—zero, NOT, AND, addition, final NOT—can be composed into a rich set of arithmetic and logic operations. Through fixed wiring and carefully chosen control bits, the ALU implements:

- addition
- subtraction
- logical operations
- increments/decrements
- zero/one/negative constants
- flag generation for conditional control flow

All of this is achieved with **pure combinational logic**, without any sequential elements or clocks.

---

## 2. Combinational Logic and Sequential Logic — The Foundation of Digital Hardware

**Combinational logic** and **sequential logic** form the core of digital circuit design. They are fundamental to modern FPGA/ASIC systems, computer architecture, and even security hardware such as TPM, PUF, and TEE.
In short:

- **Combinational logic = pure computation (no memory).**
- **Sequential logic = computation + memory (state).**

Understanding the distinction between the two is the key to understanding how processors, RAM, and all digital systems work.

#### Combinational Logic — Stateless Hardware (“Pure Functions”)

Combinational logic circuits have **no memory**.
The output depends _only_ on the **current inputs**, not on any previous values.

Mathematically:

$\text{output} = f(\text{input})$

This is exactly like a pure function in programming.

**Examples of Combinational Circuits**

- AND, OR, NOT gates
- Adders (half adder, full adder)
- Comparators
- Encoders / decoders
- Multiplexers (MUX)
- Arithmetic circuits
- AES S-box (can be implemented as a combinational logic network)

**Example: Full Adder**

Inputs: `A`, `B`, `Cin`
Outputs: `Sum`, `Cout`
The outputs depend _only_ on those inputs → purely combinational.

#### Sequential Logic — Hardware with Memory (State Machines)

Sequential logic circuits **remember past events**.
Their output depends not only on the current inputs, but also on **internal state** stored in memory elements.

Mathematically:

$\text{output} = f(\text{input}, \text{state})$

$\text{state}_{t+1} = g(\text{input}, \text{state}_t)$

**Key Building Blocks of Sequential Logic**

- **Flip-flops (FF)**
- **Registers** (multiple flip-flops)
- **Finite State Machines (FSMs)**
- **Program counters (PC)**
- **CPU pipeline registers**
- **TPM PCR registers**
- **Caches’ tag/state storage**

Sequential logic = **Store + Control**

It is responsible for all “time-based” behavior in computing.

#### Why Sequential Logic Is Necessary

Real hardware must deal with three constraints:

**(1) Hardware must be reusable over time**

A single adder must compute different operations across different clock cycles.

**(2) Some computations require remembering past values**

Example: computing
`sum = sum + next_value`
requires storing the old `sum`.

**(3) Electrical signals require time to settle**

Voltage does **not** change instantly.
Gate outputs take time to propagate.
Cutting continuous physical time into clean “steps” requires a **clock**.

Hence:

> **Combinational logic computes.
> Sequential logic stores.
> The clock synchronizes everything.**

---

#### The Clock — Turning Continuous Physics Into Discrete Time

Physical time is continuous — but digital logic cannot operate reliably in continuous time.

So hardware introduces a **clock**:

- A stable square wave signal
- Each rising edge marks a **new logical time step**
- Combinational logic stabilizes between edges
- Flip-flops latch values only on clock edges

Thus the system becomes predictable:

```case
Inputs change
↓
Combinational logic computes
↓
Clock edge arrives
↓
Flip-flops update state
```

This is the universal model of synchronous digital design.

#### Flip-Flops: The Smallest Unit of Memory

The **D Flip-Flop (DFF)** is the most fundamental sequential component.

- Input: `D`
- Output: `Q`
- Behavior: on each rising clock edge,

$Q[t] = D[t-1]$

It stores 1 bit of information and updates it one cycle later.

**Example Timeline**

| Cycle | D   | Q (output)  |
| ----- | --- | ----------- |
| t1    | 1   | ? (unknown) |
| t2    | 0   | 1           |
| t3    | 0   | 0           |
| t4    | 1   | 0           |
| t5    | 0   | 1           |

This delay is **not a bug** — it is what makes stateful computation possible.

#### From Flip-Flops to Complete Systems

**Register = multiple DFFs + logic**

A 16-bit register contains **16 DFFs**.

HDL logic:

```hdl
if load:
    next = in
else:
    next = out // hold
```

The new value is latched on the next clock edge.

**PC = register + state-update logic**

The Program Counter has controls:

- `reset`
- `load`
- `inc`
- `hold`

Priority:

```
reset > load > inc > hold
```

**RAM = array of registers + address decoding**

RAM =

- Many registers (memory cells)
- MUX for reading
- DMUX for writing

A classic multi-level hierarchy:

```
DFF    → Register
Register → Register Array
Register Array → RAM
```

#### Universal Structure of All Sequential Circuits

Every synchronous digital system — from a CPU to RAM to a TPM — can be drawn using the same architecture:

```
                +------------------------------+
                |   Combinational Logic        |
state --------> | computes next_state          | ----> next_state
input --------> | computes output (optional)   | ----> output
                +------------------------------+
                              |
                            clock
                              ↓
                     +----------------+
                     | State Register |
                     +----------------+
                              ↑
                            state
```

This “**state register + next-state logic**” model is the foundation of modern digital design.

#### The Whole Computer = Combinational Logic + Sequential Logic + Clock

A CPU is nothing more than:

- **State**
  (register file, PC, pipeline registers, flags)

- **Combinational logic**
  (ALU, decoders, control logic, forwarding logic)

- **Clock**
  (synchronizes the entire system)

Every cycle:

- Read state
- Run combinational logic
- Compute outputs + next state
- Clock edge → commit new state

This loop continues forever.

---

## 3. Bridging the Semantic Gap: From High-Level Languages to Hardware

Programs written in high-level languages—such as **Solidity, Rust, Python, JavaScript, or C**—bear almost no resemblance to what the hardware actually executes.
When writing such programs, you never:

- manipulate CPU registers directly
- compute jump addresses by hand
- write pixels into memory-mapped screen buffers
- scan keyboard registers manually

Yet the machine itself understands **only raw machine code (binary 0/1 instructions).**

This difference between human-friendly abstractions and hardware-level operations forms the _semantic gap_. To bridge it, modern computer systems rely on a multi-layer translation pipeline:

```
High-Level Language
      ↓
Compiler Front-End
      ↓
Virtual Machine (VM)
      ↓
VM Translator
      ↓
Assembly
      ↓
Machine Code (0/1)
```

And beyond translation, a program must interact with **device I/O, strings, memory, math utilities**, etc., which are not part of the language itself. These capabilities are provided by the **operating system (OS)** through service routines.
Together, the **VM layer** and the **OS layer** form the critical infrastructure that allows sophisticated programs to run on simple hardware.

---

#### The Virtual Machine (VM): A Crucial Middle Layer

The Virtual Machine acts as a **semi-abstract execution model** between high-level languages and the physical CPU.

**Why a VM is Needed**

High-level languages have rich semantics:

- functions
- variables & scopes
- objects
- loops & recursion
- stack frames
- arrays and strings

Machine languages, in contrast, are extremely primitive:

- operate on registers
- load/store from memory
- jump to fixed addresses
- arithmetic on raw integers

Directly compiling high-level constructs into raw machine instructions is extremely complex.
The VM provides a **uniform, simple, portable instruction set** that _every_ high-level structure can be mapped onto.

**The VM Execution Path**

```
High-level language semantics
         ↓ (compiler front-end)
VM instructions (push, pop, add, eq, call, return...)
         ↓ (VM translator)
Hardware-specific assembly
```

- **Unified instruction set** independent of CPU details
- **Push/pop-based stack model** fits all high-level control flows
- **High portability**
- **Clear separation of concerns**

  - the compiler front-end understands language semantics
  - the VM translator understands hardware encoding
  - neither needs to understand both

The VM effectively behaves like a **virtual CPU**, allowing high-level languages to target it easily.

#### Operating System (OS) Services: The High-Level Interface to Hardware

Even after translation, user programs must perform common tasks such as:

- screen drawing
- printing characters
- handling keyboard input
- manipulating strings and arrays
- performing arithmetic beyond addition
- allocating/deallocating memory

The OS provides these capabilities.

**Examples of OS Services**

- **I/O Services**

  - Drawing pixels or shapes
  - Writing text
  - Reading keyboard input

- **String/Array Utilities**

  - Printing strings
  - Comparison
  - Traversal

- **Math Routines**

  - multiplication, division
  - random numbers
  - square roots

- **Memory Management**

  - allocate arrays/objects
  - free memory
  - manage heap and stack regions

Although the physical operations are very low-level—for example:

- Screen memory is mapped to RAM addresses `16384 …`
- Keyboard state is read from RAM address `24576`

the OS hides these implementation details, offering clean, safe APIs.

Thus:

> **VM handles “language → low-level logic”.** > **OS handles “program → hardware functionality”.**

Together they make high-level programming possible on simple hardware.

#### Putting It All Together: The Complete Execution Pipeline

Below is the full abstraction stack that takes a human-written program and turns it into physical machine behavior:

```
┌───────────────────────────────────────┐
│           High-Level Program          │
└───────────────────┬───────────────────┘
                    ↓
        Compiler Front-End (produces VM code)
                    ↓
      ┌───────────────────────────────┐
      │         VM Instruction Set    │  ← Unified high-level expression
      └───────────────────────────────┘
                    ↓
         VM → Assembly Translator
                    ↓
                 Assembler
                    ↓
┌───────────────────────────────────────┐
│             Machine Code              │
└───────────────────────────────────────┘
                    ↓
                 Hardware
                    ↓
         Program Calls OS Services
```

---

### **Building a Computer: Connecting the CPU, Memory, and Instructions**

Once all the individual components—registers, ALU, PC, RAM, muxes, and others—are constructed, they can be assembled into a full computer capable of running programs.
In essence, **a computer = CPU + Memory + Instruction Stream**, wired together in a continuous cycle.

Inside the machine, three distinct kinds of information flow simultaneously; these are the computer’s three lifelines.

#### The Three Buses: Data, Address, and Control

**(1) Data Bus — the flow of actual values**

The data bus carries numbers between CPU components:

- registers → ALU
- ALU → registers
- RAM → CPU

For example, when executing `D = M + 1`, the CPU must:

- fetch `M` from memory
- send `M` and constant `1` to the ALU
- send the result back into register `D`

This movement happens entirely on the **data bus**.

**(2) Address Bus — selecting memory locations**

The address bus carries the question:
**“Which memory address do we want to access?”**

For example:
Setting the A register to 300 means the CPU is now addressing `RAM[300]`.

**(3) Control Bus — signals generated from instruction control bits**

The control bus carries **actions**, not values.
These control signals come from the instruction currently being executed, specifying:

- Which ALU operation to perform
- Which register to write
- Whether to write to RAM
- Whether to jump

In short, **instructions drive the control bus**, determining what the hardware should do.

#### The Fetch–Execute Cycle: The Heartbeat of the CPU

The CPU continuously repeats the fundamental cycle:

**Fetch**

- The Program Counter (PC) outputs an address.
- ROM accesses this address.
- The instruction stored at that address is sent to the CPU.

**Execute**

The CPU decodes the instruction and:

- selects ALU inputs via muxes
- configures the ALU (via zx, nx, zy, ny, f, no)
- writes results to A/D/M if needed
- checks `zr`/`ng` flags
- updates the PC (possibly jumping)

**Why separate ROM and RAM?**

In the Hack platform, program memory (ROM) and data memory (RAM) are separated.
This avoids the “single memory” conflict of the Von Neumann model and simplifies hardware.
It behaves like a simplified Harvard architecture.

#### CPU Architecture (Hack CPU)

The CPU has three main registers:

**A register**

- may hold a data value
- or act as a memory address
- provides `addressM` for RAM access

**D register**

- purely a data register
- one of the ALU’s inputs

**PC register**

- holds the address of the next instruction

The rest of the CPU is built using muxes, ALU logic, and wiring that connects to memory.

#### CPU Input and Output Signals

**Inputs**

- **inM**: the value read from RAM at the address specified by A
- **instruction**: the 16-bit instruction fetched from ROM
- **reset**: whether to clear the PC

**Outputs**

- **outM**: the value to write to RAM
- **addressM**: the RAM address to write (comes from A)
- **writeM**: whether to write to RAM
- **PC**: the next instruction address

#### Instruction Types and Data Path

Hack defines two types of instructions:

**(1) A-instruction: opcode = 0**

```
0 v14 v13 ... v0
```

This simply loads a 15-bit constant into the A register.
A then becomes:

- a value for computation, or
- an address for memory access

Example:
`@300` loads 300 into A → CPU will access `RAM[300]`.

**(2) C-instruction: opcode = 1**

A C-instruction has three fields:

```
1 a c1..c6 dest jump
```

**ALU control bits**

- `zx`, `nx` → pre-process x (x = A or M)
- `zy`, `ny` → pre-process y (y = D)
- `f` → add vs. bitwise AND
- `no` → negate output

**Destination bits: where to write?**

- A
- D
- M (RAM at address A)

**Jump bits: should the PC jump?**

Depending on zero/negative flags.

> In summary, **a C-instruction = ALU operation + destination selection + jump logic.**

#### Memory and I/O Mapping

Data memory (RAM) is divided into regions:

- **0–16K**: general-purpose RAM
- **16K–24K** (8K): screen memory-mapped display

  - writing to these addresses updates the screen

- **24576** (address 24,576): keyboard memory-mapped register

  - holds current key scan code

**Screen chip**

Internally just RAM, but with side effects: automatically drives the display device.

**Keyboard chip**

A 16-bit register containing the current keyboard state.

**Program memory**

- **32K ROM** stores the instructions
- PC selects the instruction by providing the address

#### Full-System Data Flow

The entire computer operates through the following loops:

**Instruction path**

```
PC → ROM → instruction → CPU
```

**Data path**

```
A register → RAM address line
RAM[address] → inM → CPU
CPU → outM/addressM/writeM → RAM
```

---

### How an Assembler Works: Turning Code into Binary Machine Instructions

An **assembler** is one of the simplest forms of a compiler. Its job is straightforward:
**convert human-readable assembly code into machine-readable binary instructions.**

The CPU cannot understand symbolic expressions like:

D = M + 1

It can only execute **raw 0/1 bit patterns** such as:

1111110111010000

The assembler is the tool that performs this translation.

#### The Two-Pass Assembly Process

To handle labels and variables, the assembler typically uses the classic **two-pass algorithm**.

**Pass 1 — Scan for Labels (No Code Generated)**

The assembler walks through the program line by line.

- If it encounters a label such as `(LOOP)`, it adds:

  ```
  LOOP → current instruction address
  ```

- If the line is an actual instruction, it increments the instruction counter (the future PC).

This pass does **not** generate any binary output.
It only determines where jump labels will eventually point.

**Pass 2 — Replace Symbols and Generate Machine Code**

On the second pass:

- All labels already have known addresses from Pass 1.
- Variables are allocated starting at address 16.
- Each line is translated into the corresponding 16-bit Hack machine instruction.

Example logic:

```
@i      →  if i is not in the symbol table:
              assign it the next free RAM address starting at 16
D=M+1   →  translated into the appropriate C-instruction bit pattern
```

This pass **produces the actual binary machine instruction file**.

#### Full Assembler Architecture

A complete assembler implementation can be broken into four main modules.

**(1) Input Module — Clean the Source File**

The assembler reads the file line by line and removes:

- whitespace
- comments (`// ...`)
- empty lines

What remains is a clean sequence of meaningful instructions.

**(2) First Pass — Collect Labels**

<details><summary>Pseudocode:</summary>

Pseudocode:

```python
pc = 0
for line in clean_lines:
    if is_label(line):
        label = extract_label(line)
        symbol_table[label] = pc
    else:
        pc += 1
```

</details>

Result: A symbol table containing all labels and their corresponding addresses.

**(3) Second Pass — Allocate Variables and Generate Machine Code**

<details><summary>Pseudocode:</summary>

```python
next_var_addr = 16
for line in clean_lines:
    if is_A_instruction(line):
        symbol = get_value(line)
        if is_number(symbol):
            value = int(symbol)
        else:
            if symbol not in symbol_table:
                symbol_table[symbol] = next_var_addr
                next_var_addr += 1
            value = symbol_table[symbol]
        machine_code = encode_A(value)
    else:
        dest, comp, jump = parse_C(line)
        machine_code = encode_C(dest, comp, jump)

    output.append(machine_code)
```

</details>

**What happens here?**

- Numeric constants (`@21`) → encoded directly
- Labels (`@LOOP`) → replaced using the symbol table
- Variables (`@i`, `@sum`) → assigned RAM addresses starting at 16
- C-instructions (`D=M+1;JGT`) → translated using predefined bit tables

This step produces the final 16-bit machine instructions.

**(4) Output Module — Write the binary File**

The assembler writes each 16-bit instruction to a file:

0000000000010101
1110110000010000
...

Together, this implements the classic fetch–execute cycle that allows programs to run.

Building all the primitive chips—ALU, registers, PC, memory, muxes—lays the foundation.
By wiring them together with clear data, address, and control paths, we obtain a complete computer capable of executing machine instructions.

Programs run because:

- The **PC fetches** instructions from ROM.
- The **CPU decodes** them and drives the control bus.
- The **ALU computes** results.
- The **A, D, and RAM** are updated accordingly.
- The **screen and keyboard** respond via memory-mapped I/O.
- The **PC moves to the next instruction** or jumps.

This continuous loop is what makes the hardware a real, functioning computer.

---

[68_ZK_ASIC_FPGA_GPU](https://github.com/ETAAcademy/ETAAcademy-ZK-Meme/blob/main/68_ZK_ASIC_FPGA_GPU.md)
