# ETAAcademy-Adudit: 12. Certora

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>12 Certora</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>Certora & Formal Verification</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# Formal Verification for Smart Contracts with Certora: A Mathematical Approach to Security

Formal verification, a mathematically rigorous approach to smart contract analysis, leverages propositional logic, predicate functions, and quantifiers (existential ∃ and universal ∀) to enable developers to precisely specify and systematically verify contract properties.

Certora has built a comprehensive verification ecosystem around this methodology, centered on the Certora Prover, which follows a four-step workflow: specification authoring, condition generation, SMT solving, and counterexample analysis. Its domain-specific language, CVL, provides a full suite of tools—from basic syntax to advanced features like ghost variables, hook systems, and invariant mechanisms—while integrating seamlessly with Solidity. Additionally, Certora has extended its framework with specialized tools for other blockchain platforms, such as Sunbeam for WebAssembly, a customized Prover for Solana, and a general-purpose equivalence checker.

To address the inherent complexity of formal verification, Certora incorporates advanced strategies including over-approximation to avoid missed bugs, vacuity checks to ensure rule validity, performance optimizations to overcome bottlenecks, loop unrolling to bridge code-spec mismatches, and spec soundness validation to maintain reliability.

---

Formal verification is a powerful method for analyzing smart contracts. At its core, it involves defining **properties**—similar to writing tests—and submitting them, along with compiled Solidity contracts, to a **remote prover**. The foundation of this process lies in **mathematical logic**, particularly **propositional logic** and **quantifiers**, which are essential when working with tools like the **Certora Prover**.

In propositional logic, we reason with Boolean variables and logical operations. Among these, the **implication operator** $(P \rightarrow Q)$ plays a critical role: it is only false when the premise $P$ is true and the conclusion $Q$ is false. **Predicates**, which are functions that return Boolean values, allow us to express conditions or properties over inputs. These can be extended using **quantifiers**: the **existential quantifier** $(\exists)$ asserts that at least one element satisfies a condition, while the **universal quantifier** $(\forall)$ requires all elements to satisfy it.

**Certora Prover** is a formal verification tool designed to prove that a smart contract adheres to a given set of rules or properties using rigorous mathematics. It follows a four-step workflow:

- **Writing specifications** that define contract rules and expected behaviors.
- **Compiling** these into verification conditions.
- **Using SMT solvers** (Satisfiability Modulo Theories) to automatically prove or refute those conditions.
- **Providing counterexamples** when verification fails, helping developers fix bugs early.

Certora also provides specialized tools for different blockchain platforms to address unique smart contract security challenges:

- **Certora Sunbeam** targets **WebAssembly-based smart contracts** written in **Rust**, especially on the **Stellar** platform. It requires installing the core Certora toolset along with the Rust/Stellar toolchain and the **WABT toolkit**, which converts WASM bytecode into an intermediate form suitable for verification.

- **Solana Certora Prover** is tailored for **Solana’s BPF (Berkeley Packet Filter)** format. It supports specific Rust versions and requires the `certora-sbf` subcommand.

- **The Certora Equivalence Checker** is a general-purpose component that **verifies the semantic equivalence** between two smart contract functions (currently supporting only pure functions). It checks whether two functions return the same result and revert condition on the same inputs. This is particularly useful for **contract optimization, refactoring**, or **upgrade assurance**.

**Certora Verification Language (CVL)** is a domain-specific language built to express and verify smart contract behavior precisely. It offers:

- A structured syntax covering **file organization, identifiers, comments, types, and expressions**
- Core constructs like **method blocks, rules, invariants, and ghost variables**
- Advanced features including **uninterpreted types, hooks**, and **transient storage**

One of CVL’s key strengths is its **invariant and rule-based system**, allowing developers to formally state the properties that must hold under various conditions. **Ghost variables** and **hooks** provide powerful mechanisms for tracking and intercepting state changes, making them especially suited for verifying complex dynamic behaviors. The design of method blocks and the type system ensures seamless integration with Solidity, allowing developers to express domain-specific smart contract concepts naturally and intuitively.

Certora’s framework incorporates several strategies for managing verification complexity:

- **Overapproximation** ensures no vulnerabilities are missed by considering all theoretically possible states—even unreachable ones. However, this may introduce false negatives and should be refined with precise preconditions.
- **Vacuum checks**, where rules pass simply because they never execute, are detected using options like `rule_sanity` and `assert false`.
- **Performance optimization** includes timeout analysis, rule modularization, and targeted solutions for issues like **path explosion** or **nonlinear arithmetic**.
- **Loop handling** and **code abstraction** gaps are addressed using **loop unrolling** and the `loop_iter` parameter, distinguishing between simple and complex loops.
- Finally, **tautology detection** and **hollow rule detection** ensure that specifications themselves are meaningful and non-trivial.

---

## 1. Formal Verification

**Formal verification** is a powerful method for analyzing smart contracts. In simple terms, it involves writing _properties_—similar to unit tests—and submitting them, along with the compiled contract (e.g., written in Solidity), to a **remote verifier**. Unlike fuzz testing, which explores only a limited set of specific input values, formal verification mathematically models the contract's behavior and exhaustively explores **all possible execution paths**, regardless of the contract language (e.g., Solidity or Rust).

**Mathematical Logic Behind Formal Verification**

Formal verification is fundamentally rooted in **mathematical logic**, including **propositional logic**, **predicates**, and **quantifiers**. These concepts are especially important when working with tools like the **Certora Prover**.

A **proposition** is essentially a function that takes Boolean inputs and returns a Boolean result, using only logical operators in its body. In propositional logic, the **implication operator**—commonly expressed as “if P, then Q” (written as $P \rightarrow Q$, and as `P => Q` in CVL)—is particularly significant.

This implication is only false when the premise `P` is true but the conclusion `Q` is false. In cases where `P` can never be true, the implication `P → Q` is **always true**. This is known as **vacuous truth**, which may lead to **false positives** in verification: a rule might appear valid simply because the condition is never triggered.

Here’s a Solidity-like function representing a proposition:

```solidity
function P(bool Q, bool R) returns bool {
    return Q && R;
}
```

Or, more simply, a Boolean expression:

```solidity
bool P = Q && R;
```

In formal logic, the implication `P => Q` is logically equivalent to `¬P ∨ Q` (not P or Q). Since the inputs are Booleans, we can use a **truth table** to understand how the implication behaves:

| P     | Q     | P → Q (A) |
| ----- | ----- | --------- |
| false | false | true      |
| false | true  | true      |
| true  | false | false     |
| true  | true  | true      |

**Predicates and Quantifiers**

A **predicate** is a function that returns a Boolean value, i.e., `true` or `false`. For example, the function “whether `n` is divisible by 3” is a predicate. These predicates help describe conditions or rules, and **quantifiers** allow us to express these conditions over collections of values.

There are two primary quantifiers:

- **Existential Quantifier (∃)**: asserts that _at least one_ element in a set satisfies a predicate.
- **Universal Quantifier (∀)**: asserts that _all_ elements in a set satisfy a predicate.

These quantifiers enable us to express complex properties like “for all possible inputs, the contract never overflows,” or “there exists a transaction sequence that leads to an inconsistent contract state.”

This is used to say: "There exists an `x` such that `P(x)` is true", written as $\exists x \in S, P(x)$. It is equivalent to applying logical OR across all values in a set:
`P(1) ∨ P(2) ∨ P(3) ∨ ...`

In Certora Verification Language (CVL), this concept is implemented using the keyword `exists`:

```cvl
require (exists uint n. (n % 3) == 0);
```

This quantifier expresses that a predicate holds for **every** possible value in a set:
$\forall x \in S, P(x)$, equivalent to
`P(1) ∧ P(2) ∧ P(3) ∧ ...`

For example, the statement "for every natural number `n`, either `n` is even or `n + 1` is even" can be written in CVL using the keyword `forall`:

```cvl
require (forall uint n. (n % 2 == 0) || ((n + 1) % 2 == 0));
```

A key logical equivalence to note:
The **negation of a universal quantifier** is an existential quantifier:
¬(∀x, P(x)) ⟺ ∃x, ¬P(x)

**Logical Operators and Quantifiers in Formal Verification**

| Name                    | Math Symbol | CVL Syntax    | Description                            |
| ----------------------- | ----------- | ------------- | -------------------------------------- |
| and                     | ∧           | `&&`          | AND                                    |
| or                      | ∨           | \`            | OR                                     |
| implies                 | →           | `=>`          | if P then Q                            |
| not                     | ¬           | `!`           | negation                               |
| if and only if (equiv.) | ↔           | `<=>` or `==` | equivalence                            |
| forall                  | ∀           | `forall`      | Statement must hold for all values     |
| exist                   | ∃           | `exists`      | Statement holds for at least one value |

---

## 2. Certora Formal Verification Workflow

**Certora** applies formal verification techniques to convert smart contract bytecode and specifications into precise mathematical models. This enables comprehensive, pre-deployment detection of potential vulnerabilities and provides formal proofs for critical security properties. Compared to traditional manual audits, unit testing, and fuzzing, Certora offers significant advantages: it systematically analyzes infinite execution paths and the full state space, automatically re-verifies contracts upon code changes, mathematically guarantees the correctness of properties, and integrates security considerations directly into the development lifecycle.

Certora's tool suite has been used by leading DeFi protocols such as **Uniswap**, **Compound**, **AAVE**, and **Lido** to discover high-impact vulnerabilities—including issues with data consistency, delegation logic flaws, solvency risks, transfer manipulation attacks, and errors in staking reward calculations.

Certora ensures strong security guarantees through:

- **Automated, exhaustive analysis**
- **Counterexample generation** when properties fail
- **Rich specification language**
- **Interactive developer tooling**
- **Comprehensive validation coverage**

On the technical side, Certora relies on an advanced compilation architecture that includes bytecode decompilation, static analysis, verification condition generation, and **SMT solving** (Satisfiability Modulo Theories). Its design is grounded in two key principles:

- **"Verify what you execute"** – ensuring that the exact bytecode logic is verified
- **"Trust but verify"** – independently validating assumptions for maximal assurance

At the heart of Certora lies the **Certora Prover**, a formal verification engine based on rigorous mathematical foundations. It follows a structured **four-step workflow** to verify smart contracts:

- **Specification Writing**
  Developers begin by writing formal _specifications_, which define the expected properties and behaviors of a smart contract. These specifications are expressed as rules that the contract must obey.

- **Compilation into Verification Conditions**
  The specifications and the contract’s bytecode are compiled into **verification conditions**—complex logical formulas that encode the behavior and constraints of the contract.

- **Mathematical Proof via SMT Solving**
  An SMT (Satisfiability Modulo Theories) solver is then employed to either **prove** these verification conditions mathematically or find **counterexamples** that violate the rules.

- **Counterexample Generation and Debugging**
  If a rule fails, the solver not only reports the failure but also provides a **concrete counterexample**—a specific scenario in which the rule does not hold. This greatly aids debugging and refinement.

The Certora verification process also involves setting up a structured environment and following disciplined workflows to maximize analysis quality:

- **Project Configuration**

  - _Harness contracts_: Wrapper contracts to expose internal logic for verification
  - _Specification files_: Define the rules and invariants to be proven
  - _Configuration files_: Guide how verification is run
  - _Mutation tests_: Variations used to test rule coverage and resilience

- **Pre-Specification Preparation**

  - Isolate core functionality
  - Declare methods to be verified
  - Handle unresolved calls
  - Create _shadow variables_ to track changes across function calls

- **Property Design and Development**

  - Write **invariants** (conditions that must always hold)
  - Write **rules** (pre/post conditions, frame conditions, and assertions)

- **Quality Assurance and Iteration**

  - Conduct **manual mutation testing**
  - Analyze **rule coverage**
  - Leverage automated tooling to catch edge cases

---

### 2.1 Certora Prover and Configuration Files

Certora's formal verification process is built upon a well-structured project directory that standardizes how smart contract verification tasks are organized. The project is typically composed of four core directories:

```
certora\
    harness\
    specs\
    confs\
    mutations\
```

Each directory serves a specific role in ensuring the effectiveness, clarity, and maintainability of the verification process.

**Harness Contracts: Bridging Solidity and Formal Reasoning**

The `harness/` directory contains specially crafted **harness contracts** that extend the original smart contracts to facilitate verification. These contracts are a critical component in Certora’s architecture because the **Certora Prover** primarily interacts with public interfaces, limiting its ability to inspect private or internal contract logic directly.

Harness contracts solve this problem by:

- **Exposing internal and private state variables and functions** of the original contracts
- **Adding auxiliary helper functions** to overcome limitations of the Certora Verification Language (CVL), enabling more expressive and concise specifications
- **Mocking external dependencies**, allowing isolated verification of specific modules or behaviors

While not strictly mandatory, a well-designed harness layer can significantly reduce time and effort in the later stages of verification by increasing code coverage, enabling access to complex internal states, and breaking down large systems into manageable verification units.

**Specification Files (CVL): Defining the Rules of Correctness**

The `specs/` directory contains contract-specific `.spec` files written in the **Certora Verification Language (CVL)**. These specifications define the expected behaviors and properties that the target contract must satisfy. To ensure systematic and comprehensive coverage, each `.spec` file typically follows a consistent five-part structure:

- **methods**: Declares the external methods to be verified, including attributes like environmental dependencies.
- **definitions**: Functions as a macro layer, providing aliases for constants and complex expressions to improve readability and reduce duplication.
- **functions**: Includes helper logic or stubs for contract functions, facilitating code reuse and controllable behavior during verification.
- **ghosts & hooks**: Defines "ghost variables"—shadow copies of storage variables that help track internal state, including states not explicitly present in the original contract.
- **properties**: The core of the specification, this section defines **rules** and **invariants** that must always hold for the contract to be considered correct.

**Configuration Files (.conf): Controlling the Verification Pipeline**

Stored in the `confs/` directory, Certora configuration files use the **JSON5 format** (with `.conf` extension) to describe how verification should be executed. This structured configuration helps eliminate repetitive CLI commands, supports comments for documentation, and allows for version control within collaborative teams.

Key fields include:

- `"files"`: The Solidity files to compile and verify
- `"verify"`: The fully qualified target for verification (e.g., `ContractName:specFile`)
- `"msg"`: A human-readable message describing the run
- `"wait_for_results"`: Determines if the process waits for a cloud verification job to complete
- `"solc"`: Path to the Solidity compiler, enabling custom compiler usage
- `"rule_sanity"`: A critical feature that checks for **vacuous rules**—rules that always pass, even on faulty contracts—preventing false confidence in verification outcomes

<details><summary>Code</summary>

```json
{
  "files": ["contracts/ERC20.sol"],
  "verify": "ERC20:certora/spec/ERC20.spec",
  "msg": "ERC20Rules",
  "mutations": {
    "gambit": [
      {
        "filename": "contracts/ERC20.sol",
        "num_mutants": 5
      }
    ],
    "msg": "basic mutation configuration"
  }
}
```

</details>

**Mutation Testing: Validating the Validators**

The `mutations/` directory is dedicated to **mutation testing**, a technique used to test the effectiveness of the verification rules themselves. It works on the principle of **pass-fail contrast**: a well-written rule should pass when run against the original, correct contract, and fail when the same contract is intentionally broken (mutated).

Certora supports two mutation strategies:

- **Manual mutations**: Crafted by developers to simulate specific edge cases or known weaknesses
- **Automated mutations**: Generated by the **Gambit engine**, which produces a wide variety of contract modifications for robust coverage

Mutation testing workflow:

- Write and validate a specification rule on the original contract
- Create mutated versions of the contract (manually or automatically)
- Run verification against these versions
- Evaluate whether the rules correctly fail when logic is violated

This approach offers several benefits: validating rule correctness, reducing false positives, improving rule quality, providing coverage metrics, and increasing the overall reliability of verification outcomes. It is recommended to create and test corresponding mutations immediately after implementing each rule. This proactive strategy provides early feedback, helping developers quickly identify and fix specification flaws, ensuring that formal verification is not only theoretically sound but also practically effective in detecting vulnerabilities in smart contracts.

<details><summary>Code</summary>

```json
"gambit": [
  {
    "filename": "contracts/ERC20.sol",
    "num_mutants": 2,
    "mutations": [
      "require-mutation"
    ]
  },
  {
    "filename": "contracts/ERC20.sol",
    "num_mutants": 1,
    "mutations": [
      "assignment-mutation"
    ]
  }
],
```

</details>

#### Formal Verification for WebAssembly and Solana Smart Contracts with Certora

Certora is extending its formal verification capabilities beyond Ethereum to support Rust-based WebAssembly smart contracts and Solana programs. This is made possible through tools like **Certora Sunbeam** for WASM contracts and **Solana Certora Prover** for Solana-specific workflows. Both tools provide powerful frameworks for verifying the correctness and security of smart contracts through formal logic, backed by comprehensive toolchains and developer-friendly environments.

**Verifying Rust-WASM Smart Contracts with Certora Sunbeam**

**Certora Sunbeam** enables formal verification of smart contracts written in Rust and compiled to WebAssembly (WASM), such as those deployed on the Stellar blockchain. The verification setup is divided into two major phases:

- **Core Certora Setup**

  - **Create a Certora account** and retrieve your access key.
  - Install **Python** and **Java runtime** (required for the prover).
  - Use `pip` to install `certora-cli-beta`.
  - Configure your Certora credentials via environment variables or a configuration file.

- **Rust & Stellar Toolchain Configuration**

  - Install the **Rust programming language** and the WASM compilation target.
  - Set up the **Stellar CLI**.
  - Install **WABT** (WebAssembly Binary Toolkit), especially `wasm2wat` for readable WASM output.
  - Use `just` as a command runner and `rustfilt` to demangle Rust symbols for cleaner output.

Once the environment is ready, you can compile, inspect, and verify WebAssembly contracts. Here's an example of a CVL rule written in Rust to verify token transfers:

<details><summary>Code</summary>

```rust
#[rule]
fn transfer_is_correct(e: Env, to: Address, from: Address, amount: i64) {
    cvlr_assume!(
        e.storage().persistent().has(&from) &&
        e.storage().persistent().has(&to) &&
        to != from
    );
    let balance_from_before = Token::balance(&e, from.clone());
    let balance_to_before = Token::balance(&e, to.clone());
    Token::transfer(&e, from.clone(), to.clone(), amount);
    let balance_from_after = Token::balance(&e, from.clone());
    let balance_to_after = Token::balance(&e, to.clone());
    cvlr_assert!(
        (balance_to_after == balance_to_before + amount) &&
        (balance_from_after == balance_from_before - amount)
    );
}
```

</details>

**Formal Verification on Solana with Solana Certora Prover**

For smart contracts written in Rust and compiled for the **Solana blockchain**, Certora offers the **Solana Certora Prover**. This tool builds on Certora’s core prover and tailors the experience for Solana’s BPF execution model.

- **Basic Certora Setup**

  - Create your Certora account and retrieve the access key.
  - Install Python and Java.
  - Install the CLI tool and set the necessary environment variables.

- **Solana-Specific Environment**

  - Install **Rust** and multiple toolchains (to match Solana’s requirements).
  - Install the `certora-sbf` Cargo subcommand via Rust, and verifying the installation by running `cargo certora-sbf --no-build`, which also downloads necessary platform tools.
  - The Solana Certora Prover is optimized for Rust-based smart contracts and supports Solana-specific features, including `BPF-format` programs, enabling powerful formal verification capabilities.
  - Use **VSCode** with **rust-analyzer** to improve development workflow.

Here’s an example set of rules used to verify the behavior of a Solana function that computes transaction fees:

<details><summary>Code</summary>

```rust
pub fn compute_fee(amount: u64, fee_bps: u16) -> Option<u64> {
    if amount == 0 { return None; }
    let fee = amount.checked_mul(fee_bps).checked_div(10_000);
    fee
}

#[rule]
pub fn rule_fee_sanity() {
    compute_fee(nondet(), nondet()).unwrap();
    cvlr_satisfy!(true);
}

#[rule]
pub fn rule_fee_assessed() {
    let amt: u64 = nondet();
    let fee_bps: u16 = nondet();
    cvlr_assume!(fee_bps <= 10_000);
    let fee = compute_fee(amt, fee_bps).unwrap();
    clog!(amt, fee_bps, fee);
    cvlr_assert!(fee <= amt);
    if fee_bps > 0 { cvlr_assert!(fee > 0); }
}

#[rule]
pub fn rule_fee_liveness() {
    let amt: u64 = nondet();
    let fee_bps: u16 = nondet();
    cvlr_assume!(fee_bps <= 10_000);
    let fee = compute_fee(amt, fee_bps);
    clog!(amt, fee_bps, fee);
    if fee.is_none() {
        cvlr_assert!(amt == 0);
    }
}
```

</details>

**Certora Equivalence Checker**

Another important tool in the Certora ecosystem is the **Certora Equivalence Checker**, which ensures that two contract functions are semantically identical. This is especially useful during contract upgrades, optimizations, or refactorings.

- Compares only **pure functions** (i.e., functions without side effects).
- Automatically generates the required **CVL specs** and **config files**.
- Verifies whether two functions:

  - Produce the same output for the same inputs.
  - Revert under the same conditions.

- Usage Modes:

  - **CLI mode**: Directly specify contract paths, function names, and Solidity versions.
  - **Config mode**: Use prewritten configuration files for repeated equivalence checks.

The generated CVL will include two essential rules:

- `equivalence_of_revert_conditions`: ensures both functions revert under the same input conditions.
- `equivalence_of_return_value`: checks that return values are identical.

For functions involving bitwise operations, the `precise_bitwise_ops` flag can be enabled to ensure precise and accurate results.

Here’s a simplified example illustrating how the tool detects a difference between an addition and multiplication function:

<details><summary>Code</summary>

```solidity
using BasicMathBad as B;

// sets everything but the callee the same in two environments
function e_equivalence(env e1, env e2) {
    require e1.msg.sender == e2.msg.sender;
    require e1.block.timestamp == e2.block.timestamp;
    require e1.msg.value == e2.msg.value;
    // require e1.msg.data == e2.msg.data;
}

rule equivalence_of_revert_conditions()
{
    bool add_revert;
    bool add_mult_revert;
    // using this as opposed to generating input parameters is experimental
    env e_add; calldataarg args;
    env e_add_mult;
    e_equivalence(e_add, e_add_mult);

    add@withrevert(e_add, args);
    add_revert = lastReverted;

    B.add_mult@withrevert(e_add_mult, args);
    add_mult_revert = lastReverted;

    assert(add_revert == add_mult_revert);
}

rule equivalence_of_return_value()
{
    uint256 add_uint256_out0;
    uint256 add_mult_uint256_out0;

    env e_add; calldataarg args;
    env e_add_mult;

    e_equivalence(e_add, e_add_mult);

    add_uint256_out0 = add(e_add, args);
    add_mult_uint256_out0 = B.add_mult(e_add_mult, args);

    assert(add_uint256_out0 == add_mult_uint256_out0);
}
```

```json
{
  "disable_local_typechecking": true,
  "files": ["Test/EqCheck/BasicMathGood.sol", "Test/EqCheck/BasicMathBad.sol"],
  "msg": "EquivalenceCheck of add and add_mult",
  "optimistic_loop": true,
  "loop_iter": "4",
  "process": "emv",
  "send_only": true,
  "short_output": true,
  "rule_sanity": "basic",
  "server": "staging",
  "prover_version": "master",
  "solc_optimize": "200",
  "verify": "BasicMathGood:Test/EqCheck/add_to_add_mult_equivalence.spec",
  "solc": "solc8.0"
}
```

</details>

### 2.2 Rules and Common Pitfalls

Certora Prover offers a powerful framework for formal verification of smart contracts, and mastering its rule system is key to ensuring correctness, security, and robustness. This article explores parameterized rules, over-approximation, and vacuity—three fundamental concepts that shape the accuracy and effectiveness of Certora-based verification workflows.

**Parameterized Rules: Reusable Logic for All Functions**

Parameterized rules define general properties or invariants that apply across all functions in a contract. They utilize the `method f` parameter along with the `calldataarg` type to simulate the invocation of _any_ contract method, regardless of argument type or arity. This makes parameterized rules especially well-suited for expressing cross-cutting behaviors, such as “only the owner can change an allowance.”

The core advantage of this technique lies in its **reusability**, **implementation independence**, and **comprehensive coverage**. With one parameterized rule, developers can enforce constraints that would otherwise require writing multiple function-specific rules. Additionally, features like `selector` and `sig` enhance rule granularity, allowing precise targeting or exclusion of specific functions.

In practice, these rules are ideal for CI/CD integration, where every commit or pull request can be checked for regressions in core invariants. For example, one such rule may catch an integer overflow bug in `decreaseAllowance` that would otherwise allow an attacker to increase the approved amount.

<details><summary>Code</summary>

```solidity
/**
 * # ERC20 Parametric Example
 *
 * Another example specification for an ERC20 contract. This one using a parametric rule,
 * which is a rule that encompasses all the methods in the current contract. It is called
 * parametric since one of the rule's parameters is the current contract method.
 * To run enter:
 *
 * certoraRun ERC20.sol --verify ERC20:Parametric.spec --solc solc8.0 --msg "Parametric rule"
 *
 * The `onlyHolderCanChangeAllowance` fails for one of the methods. Look at the Prover
 * results and understand the counter example - which discovers a weakness in the
 * current contract.
 */

// The methods block below gives various declarations regarding solidity methods.
methods
{
    // When a function is not using the environment (e.g., `msg.sender`), it can be
    // declared as `envfree`
    function balanceOf(address) external returns (uint) envfree;
    function allowance(address,address) external returns(uint) envfree;
    function totalSupply() external returns (uint) envfree;
}


/// @title If `approve` changes a holder's allowance, then it was called by the holder
rule onlyHolderCanChangeAllowance(address holder, address spender, method f) {

    // The allowance before the method was called
    mathint allowance_before = allowance(holder, spender);

    env e;
    calldataarg args;  // Arguments for the method f
    f(e, args);

    // The allowance after the method was called
    mathint allowance_after = allowance(holder, spender);

    assert allowance_after > allowance_before => e.msg.sender == holder,
        "only the sender can change its own allowance";

    // Assert that if the allowance changed then `approve` or `increaseAllowance` was called.
    assert (
        allowance_after > allowance_before =>
        (
            f.selector == sig:approve(address, uint).selector ||
            f.selector == sig:increaseAllowance(address, uint).selector
        )
    ),
    "only approve and increaseAllowance can increase allowances";
}
```

</details>

**Over-Approximation: Powerful but Subtle**

Over-approximation is a deliberate design choice in Certora Prover. To avoid missing bugs, the tool considers **all** possible program states, including those that might never occur during actual execution. While this increases the soundness of the analysis, it can also produce _false positives_—that is, reports of rule violations in unreachable states.

A classic example involves checking whether a user’s balance is always less than or equal to the total supply. Without any assumptions, the verifier might flag a violation, even though such a state would never occur in a real deployment. The fix is to add _preconditions_ that constrain the initial state space to only reachable states.This ensures the Prover only examines feasible states, improving result precision.

<details><summary>Code</summary>

```solidity
/// @title Total supply after mint is at least the recipient's balance (with precondition)
rule totalSupplyAfterMintWithPrecondition(address account, uint256 amount) {
    env e;

    uint256 userBalanceBefore = balanceOf(account);
    uint256 totalBefore = totalSupply();

    // Precondition: total supply must be at least user balance before minting
    require totalBefore >= userBalanceBefore;

    mint(e, account, amount);

    uint256 userBalanceAfter = balanceOf(account);
    uint256 totalAfter = totalSupply();

    assert totalAfter >= userBalanceAfter, "total supply is less than a user's balance";
}
```

</details>

**Vacuity: The Illusion of Safety**

Vacuity is one of the most dangerous and misunderstood aspects of formal verification. A rule is said to be _vacuously true_ when its assumptions are never satisfied—meaning the rule doesn’t actually test anything meaningful, yet still passes.

This often stems from contradictory assumptions (e.g., `x > y && y > x`) or unhandled execution paths. In Solidity 0.8.0 and later, arithmetic overflows cause implicit reverts, which the Prover **ignores by default**. If a rule depends on paths that revert, it might quietly pass without ever being evaluated.

To detect vacuous rules, Certora provides the `--rule_sanity basic` option (or `"rule_sanity": "basic"` in config files), which checks whether any valid execution paths satisfy the rule's conditions. You can also explicitly track reverts using `@withrevert` and the `lastReverted` variable, adding robust assertions about expected failures.

In practice, enabling rule sanity checks is a **best practice**. It ensures that every rule contributes real value, identifies broken or overly restrictive logic, and improves confidence in your verification outcomes.

<details><summary>Code</summary>

```solidity
pragma solidity ^0.8.0;

/// @title Examples of vacuous rules
contract Vacuous {

  mapping (address => uint256) public amounts;
  uint256 public immutable maximalAmount = 1000;

  function add(address user, uint256 amount) public returns (uint256) {
    require(amounts[user] + amount <= maximalAmount);
    amounts[user] += amount;
    return amounts[user];
  }

  function sub(address user, uint256 amount) public returns (uint256) {
    amounts[user] -= amount;
    return amounts[user];
  }
}
```

```solidity
rule subtleVacuousRule(address user, uint256 amount) {
    uint256 userAmount = amounts(user);
    require amount > userAmount;
    sub(user, amount);
    assert false;  // Should always fail
}

rule revertingRule(address user, uint256 amount) {
    uint256 userAmount = amounts(user);
    require amount > userAmount;
    sub@withrevert(user, amount);
    assert lastReverted;
}
```

</details>

**Identifying and Diagnosing Performance Issues**

Performance issues in formal verification often manifest as timeouts or memory exhaustion. Certora Prover provides both error messages and visual indicators to help diagnose these issues. For instance, a "very low available memory" warning or an orange clock icon signals a potential performance bottleneck.

A systematic approach to diagnosing timeouts begins with classification: distinguishing between different timeout sources (e.g., path explosion, SMT solver complexity, or inefficient memory use). Certora’s technical analysis and classification (TAC) reports and difficulty statistics provide further insight into the root causes.

Complex smart contracts frequently involve challenging features such as:

- **Path explosion**: Occurs when numerous conditional branches lead to an exponential number of execution paths.
- **Non-linear arithmetic**: Operations such as `x * y` are notoriously difficult for SMT solvers to reason about.
- **Memory/storage-heavy structures**: Deeply nested structs or dynamic arrays increase the symbolic execution burden.
- **Inline assembly**: Introduces opaque operations that complicate reasoning.

To mitigate these challenges, Certora offers several strategies:

- **Modular rule design**: Break rules into smaller components to isolate and identify performance bottlenecks.
- **Use sanity checks**: Define basic rules (e.g., asserting known invariants) to ensure your environment and assumptions are correctly modeled.
- **Run rules individually**: Helps isolate the rule causing a timeout.
- **Function summarization**: Replace complex internal calls with summaries when full inlining is unnecessary or too expensive.

Additionally, command-line optimizations such as enabling destructive optimizations (`--optimize`) can help reduce verification time. This kind of dynamic data structure creates symbolic variables for each internal array or string, increasing memory complexity and posing a challenge for the SMT solver. In such cases, consider simplifying inputs, limiting iterations, or replacing complex types with abstract representations.

<details><summary>Code</summary>

```solidity
rule myRule() {
    MyStruct x;
    foo(x);
}

struct MyStruct {
    bytes b;
    string s;
    uint[] u1;
    uint8[] u2;
}

function foo(MyStruct x) public {
    ...
}
```

</details>

**Loop Handling: Bridging the High-Level and Low-Level Gap**

A unique challenge in Certora Prover stems from the gap between high-level Solidity source code and the low-level EVM bytecode that the Prover analyzes. Loops are a prime example. Certora handles loops using _loop unrolling_, controlled via the `--loop_iter` parameter, and supports both assertive and optimistic strategies with `--optimistic_loop`.

For simple `for` loops like `for (uint i = 0; i < 3; i++)`, Certora can often infer the iteration count even with a smaller `--loop_iter`. However, for complex or infinite loops (`while(true)`), Certora uses a conservative strategy: it performs one extra iteration to detect early exits.

This may lead to unintuitive outcomes—for example, setting `--loop_iter 3` might unroll the loop four times at the bytecode level due to structural differences. Understanding these nuances is essential to avoid misinterpreting verification results and to fine-tune loop-related configurations.

<details><summary>Code</summary>

```solidity
uint x; // global state variable
uint i = 0;
// iteration #1
x++;
if (i >= 3) {
    goto after_original_while_loop_end;
}
i += 1;

// iteration #2
x++;
if (i >= 3) {
    goto after_original_while_loop_end;
}
i += 1;

// iteration #3
x++;
if (i >= 3) {
    goto after_original_while_loop_end;
}
i += 1;

// iteration #4
x++;
if (i >= 3) {
    goto after_original_while_loop_end;
}
i += 1;

assert(false); // require(false) if `--optimistic_loop` is set

after_original_while_loop_end: ...
```

</details>

**Ensuring Specification Correctness**

An often overlooked but critically important aspect of formal verification is ensuring the correctness and meaningfulness of the specifications themselves. Even the most rigorous verification process can fail if the specification is flawed. Two common types of specification issues—and their detection methods—are particularly important:

- **Vacuous Specifications**

A vacuous specification is one that technically passes verification but doesn’t actually test any meaningful property. This typically happens when the rule’s preconditions cannot be satisfied by any possible execution path. For example, a rule that includes the condition `balanceOf(0x0, token) == 0` may always revert due to a zero-address check in the contract logic, meaning the assertion is never reached.

To detect vacuous specifications, Certora allows developers to append `assert false;` at the end of the rule. If this assertion passes, it indicates that no execution path reaches the assertion point—revealing the rule is vacuous. Certora can also run vacuity checks automatically using rule sanity checks.

- **Tautology Specifications**

Tautological specifications are a special case of vacuous specifications where the assertion is trivially true, regardless of the contract state. For example, a rule that asserts `x == x` will always pass, offering no real insight or verification value.

Certora detects tautologies by stripping away the rule’s preconditions and actions, evaluating whether the assertion still holds in all cases. If it does, the rule is deemed tautological.

By identifying and addressing vacuous and tautological rules, developers can ensure their specifications are meaningful, test relevant behaviors, and contribute to the overall soundness of the verification process.

<details><summary>Code</summary>

```solidity
rule held_token_should_exist{
    address user;
    uint256 token;
    require balanceOf(0, token) == 0;

    require balanceOf(user, token) <= totalSupplyOf(token);
    assert balanceOf(user, token) > 0 => token_exists(token);
}

rule something_is_always_transferred{
    address receiver;
    uint256 balance_before_transfer = balanceOf(receiver);
    require balanceOf(receiver) == 0;

    uint256 amount;
    require amount > 0;

    transfer(receiver, amount);
    uint256 balance_after_transfer = balanceOf(receiver);
    assert balanceOf(receiver) <= balance_after_transfer;
}
```

/details>

---

### 2.3 Invariants and Ghost Variables in Formal Verification

In the realm of formal verification, _invariants_ and _ghost variables_ are two powerful tools that significantly enhance the expressiveness and robustness of smart contract analysis. These mechanisms not only improve the ability to reason about complex contract behaviors, but also enable more precise and maintainable specification writing—especially in large-scale or security-critical systems.

**Invariants: Enforcing Lifelong Properties**

Invariants are conditions that must hold true throughout the entire lifecycle of a contract. They provide a strong guarantee that certain properties—such as relationships between variables or system-wide constraints—are always maintained, regardless of the contract’s operational path.

Certora Prover supports invariants through a construct that automatically checks their correctness in two stages:

- **Base Case:** After the constructor has executed, the invariant must hold.
- **Inductive Step:** Assuming the invariant holds before a function call, the Prover verifies that it still holds afterward.

This inductive reasoning aligns with the mathematical foundations of invariant logic and ensures that any state transition respects the declared properties.

A simple **voting contract** illustrates the use of invariants. The invariant ensures that the sum of votes in favor and against equals the total number of votes. This can be expressed directly as an invariant, or alternatively translated into a rule that checks the property before and after function execution.

A correctly translated rule requires the invariant to hold before the function call and asserts it afterward. A common mistake is to omit the precondition. Such a rule may pass verification vacuously but fails to demonstrate the invariant's preservation through function calls. One key advantage of using the `invariant` keyword over manual rule writing is automation: Certora automatically checks the invariant for all relevant transitions, including constructor execution—something manual rules often neglect.

<details><summary>Code</summary>

```solidity

/**
 * # Simple voting invariant example
 *
 * A simple invariant example. Additionally there are two rules, one is a correct
 * translation of the invariant to a rule, and the other is a wrong translation.
 */

methods
{
    function votesInFavor() external returns (uint256) envfree;
    function votesAgainst() external returns (uint256) envfree;
    function totalVotes() external returns (uint256) envfree;
}


/// @title Sum of voter in favor and against equals total number of voted
invariant sumResultsEqualsTotalVotes()
    votesInFavor() + votesAgainst() == to_mathint(totalVotes());


/// @title This rule is a correct translation of the invariant
rule sumResultsEqualsTotalVotesAsRule(method f) {
    // Precondition
    require votesInFavor() + votesAgainst() == to_mathint(totalVotes());

    env e;
    calldataarg args;
    f(e, args);

    assert (
        votesInFavor() + votesAgainst() == to_mathint(totalVotes()),
        "Sum of votes should equal total votes"
    );
}


/// @title This rule is a wrong translation of the invariant
rule sumResultsEqualsTotalVotesWrong() {
    assert (
        votesInFavor() + votesAgainst() == to_mathint(totalVotes()),
        "Sum of votes should equal total votes"
    );
}
```

</details>

**Preserved Blocks: Contextual Support for Invariants**

To verify invariants under more complex interactions, _preserved blocks_ provide necessary context and assumptions. They define additional requirements that must be met in specific functions in order for the invariant to hold.

Their importance is clearly illustrated in the debt token example: when verifying the seemingly simple invariant that "collateral value must always be greater than or equal to the debt balance," the `transferDebt` function causes a failure. Further analysis reveals the root cause—this function allows a debt to be transferred from an address that violates the invariant (`another`) to one that satisfies it (`account`), ultimately causing the receiving address to violate the invariant post-transfer.

The solution involves using a `preserved` block along with a `requireInvariant` statement to enforce that the sender (`e.msg.sender`) must also satisfy the same invariant before the transfer. This reflects the principle of inductive reasoning in formal verification. Preserved blocks not only offer a clear syntax (including default and function-specific preserved blocks, as well as environment access) but also maintain reliability in verification—unlike regular `require` statements that may lead to unsound proofs, `requireInvariant` guarantees correctness under already-proven invariants. This mechanism is particularly valuable in complex systems like fund managers, where critical properties—such as "no two funds share the same manager"—must be rigorously enforced.

<details><summary>Code</summary>

```solidity

/**
 * Debt token invariant
 *
 * An invariant claiming the collateral of
 * an account is never less than the balance.
 */
methods {
    function balanceOf(address) external returns (uint256) envfree;
    function collateralOf(address) external returns (uint256) envfree;
}


/// @title Collateral is never less than the balance
invariant collateralCoversBalance(address account)
    collateralOf(account) >= balanceOf(account)
    {
        preserved transferDebt(address recipient) with (env e)
        {
            requireInvariant collateralCoversBalance(e.msg.sender);
        }
    }
```

</details>

#### Ghost Variables, Storage Hooks, and Opcode Hooks in CVL: Building Deep Formal Verification Models

In Certora Verification Language (CVL), **ghost variables**, **storage hooks**, and **opcode hooks** form the backbone of advanced formal verification strategies. Together, they extend CVL’s expressiveness beyond direct contract state inspection, enabling fine-grained reasoning about cumulative behaviors, historical state transitions, and even low-level Ethereum Virtual Machine (EVM) execution details. These mechanisms allow verification engineers to build precise, powerful “shadow models” of contract behavior—models that mirror and monitor internal processes without altering the original contract code.

**Ghost Variables: Modeling Additional State**

Ghost variables are specification-defined variables that simulate contract storage and track custom state during execution. Crucially, they are **rollback-aware**, meaning they are automatically reverted when the underlying transaction reverts—mirroring the actual behavior of EVM storage. This fidelity makes them ideal for modeling auxiliary state without side effects.

Ghost variables are initialized using `init_state axiom`, which allows precise control over their initial values—critical for setting up consistent verification conditions. For example, in a simple **voting contract**, a ghost variable `numVoted` can be used to count the number of accounts that have voted, independent of the actual implementation logic:

**Storage Hooks: Monitoring State Transitions**

To dynamically update ghost variables in response to actual contract behavior, CVL provides **storage hooks**—special constructs that intercept read or write operations on specific storage variables.

Two types of storage hooks are available:

- `Sstore`: Triggers when a storage variable is written to.
- `Sload`: Triggers when a storage variable is read.

These hooks allow the verifier to respond to storage interactions and adjust ghost state accordingly. For instance, in the same voting contract, we can increment `numVoted` each time `_hasVoted[address]` is set from `false` to `true`. This lets us assert an invariant that the ghost-tracked vote count is always equal to the contract’s `totalVotes()`.

By combining ghost variables and hooks, we can model behaviors like cumulative updates, state histories, and hidden transitions—behaviors that are otherwise difficult to capture through standard contract state snapshots alone.

<details><summary>Code</summary>

```solidity
/**
 * # Simple voting ghost invariant example
 */

methods
{
    function votesInFavor() external returns (uint256) envfree;
    function votesAgainst() external returns (uint256) envfree;
    function totalVotes() external returns (uint256) envfree;
}

ghost mathint numVoted {
    // No votes at start
    init_state axiom numVoted == 0;
}

hook Sstore _hasVoted[KEY address voter]
    bool newVal (bool oldVal) {
    numVoted = numVoted + 1;
}

/// @title Total voted intergrity
invariant sumResultsEqualsTotalVotes()
     to_mathint(totalVotes()) == numVoted;
```

</details>

**Opcode Hooks: Reaching into the EVM**

While CVL typically abstracts away the low-level details of EVM execution, **opcode hooks** offer a way to selectively reach into the EVM layer. This is particularly useful when verifying properties that depend on EVM internals that are not exposed through high-level interfaces or the `env` object.

This mechanism addresses the intentional abstraction between CVL and the EVM by granting access to low-level EVM state information—such as `CHAINID` or `GASPRICE`—that standard environment objects (`env`) cannot capture. In practice, opcode hooks can be used for simple tasks like constraining the chain ID to simulate a specific network (e.g., Ethereum Mainnet), or more complex scenarios such as monitoring the execution of `CALL` instructions to verify that no external calls are made during emergency modes.

- Execution restrictions under emergency modes.
- Network constraints (e.g., ensuring CHAINID matches mainnet).
- Gas metering and dynamic execution analysis.
- Fine-grained control of state changes and external interactions.

The true value of opcode hooks lies in their elegant ability to observe specific instruction execution, access underlying state, express richer properties, and constrain the verification environment when necessary. This makes it possible for the Certora Prover to verify nuanced properties involving fund flows, state transition permissions, or execution path restrictions. When combined with ghost variables, opcode hooks further empower verifiers to build persistent state models and complex verification logic—without altering the original contract code—enabling thorough, precise, and low-level security guarantees for smart contracts. For instance, a contract might prohibit any external calls during emergency mode. Using an opcode hook on `CALL`, the verifier can check whether this critical security property is ever violated.

<details><summary>Code</summary>

```solidity
ghost uint chain_id;
hook CHAINID uint id {
    chain_id = id;
}

/// The contract's behavior changes appropriately when run on mainnet
rule chainid_behavior_correct {
    ...
    if (chain_id == 1) {
        ...
    } else {
        ...
    }
}
```

```solidity
ghost bool made_call;

hook CALL(uint g, address addr, uint value, uint argsOffset, uint argsLength, uint retOffset, uint retLength) uint rc {
    made_call = true;
}

/// While in `emergencyMode`, no function can make an external call.
rule no_call_during_emergency {
    require !made_call;
    require emergencyMode();

    method f; env e; calldataarg args;
    f(e, args);

    assert !made_call;
}
```

</details>
