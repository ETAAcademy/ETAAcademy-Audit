# ETAAcademy-Adudit: 11. Slither

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>11 Slither and Other Static Analysis Tools</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>Slither_and_Other_Static_Analysis_Tools</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# Slither Static Analysis Framework

Slither is a static analysis framework for smart contract, which combines structural analysis, control/data flow visualization, and an advanced intermediate representation (SlithIR) to enable precise vulnerability detection across Solidity and Vyper codebases.

Its modular architecture supports hundreds of built-in detectors covering critical issues like reentrancy, improper validation, and unauthorized access, while also providing extensibility through custom detectors, tools and APIs. Slither addresses limitations of other static analyzers by integrating interval analysis with SMT solvers like Z3 to handle complex types and constraints, enabling more accurate detection of unreachable code and logical vulnerabilities.

The article also introduces other static analysis tools, including SARIF Explorer for result visualization, Fickling for Python pickle security, Circomspect for zero-knowledge circuit analysis, Macroni for macro-aware C code analysis, Semgrep for detecting machine learning system vulnerabilities, Amarna for Cairo language security analysis, and CodeQL for deep static analysis capabilities. Together, these tools form a comprehensive security assessment ecosystem that has successfully prevented potential exploits worth millions of dollars in production environments.

---

**Slither** is a powerful static analysis framework designed specifically for **Ethereum smart contracts**. Unlike dynamic analysis tools that trace single execution paths, Slither evaluates **all possible execution paths**, offering comprehensive insights into contract behaviors and potential vulnerabilities. Its power stems from its multi-layered internal representations, primarily the **Abstract Syntax Tree (AST)** and **Control Flow Graph (CFG)**.

- **AST (Abstract Syntax Tree)**: Extracted from the Solidity compiler (`solc`), this tree-based structure represents the nested syntactic structure of the code. While essential for basic syntactic analysis, its deeply nested form can be challenging for more complex tasks. Slither offers an `ExpressionVisitor` class for recursive AST traversal.
- **CFG (Control Flow Graph)**: This graph representation illustrates all possible execution paths within a function, making it the backbone of most advanced analyses in Slither. CFGs are essential for control-flow-based vulnerability detection.

<div align=center>
    <img src="https://github.com/ETAAcademy/ETAAcademy-Images/blob/main/ETAAcademy-Audit/11_Slither_AST.gif?raw=true" alt="Image 1" width="30%" style="display: inline-block;">
    <img src="https://github.com/ETAAcademy/ETAAcademy-Images/blob/main/ETAAcademy-Audit/11_Slither_CFG.gif?raw=true" alt="Image 2" width="30%" style="display: inline-block;">
</div>

<details><summary>Code</summary>

```solidity
function safeAdd(uint256 a, uint256 b) internal pure returns (uint256) {
    if (a + b <= a) {
        revert();
    }
    return a + b;
}
```

</details>

---

## 1. Common Slither Functionalities

Slither provides a flexible command-line interface and API, supporting a wide range of use cases:

### 1.1 Basic Tools and Modes

#### Commands

- Analyze a project or file:

  ```bash
  slither .
  slither MyContract.sol
  slither <contract_address>
  ```

#### Tools and Modes

- Detectors: identify specific classes of vulnerabilities, such as `slither MyContract.sol --detect reentrancy` and `slither MyContract.sol --exclude suicidal`

  - `arbitrary-send`: detects unrestricted fund transfers
  - `reentrancy`: detects reentrancy vulnerabilities
  - `pragma`: checks for unsafe or outdated compiler versions
  - `uninitialized-state`: flags uninitialized state variables
  - `suicidal`: detects contracts vulnerable to `selfdestruct`
  - `naming-convention`: enforces Solidity naming standards

- Printers: Slither printers provide various structural, behavioral, and dataflow summaries, such as `slither MyContract.sol --print call-graph`

- Path Filtering: analyze specific files as `slither . --filter-paths MyContract.sol`

- Triage Mode and In-Code Annotations:

  - Triage mode filters analysis output by categories: `slither . --triage-mode`

  - Detectors inline: `// slither-disable-next-line reentrancy`

  - Tag functions for Slither using Solidity annotations: `/// @custom:security non-reentrant`

- Configuration: customize behavior via `slither.config.json`

### 1.2 Slither API

The Slither API is a powerful programming interface for smart contract analysis, providing six levels of abstraction: the Slither main object, SlitherCompilationUnit (compilation unit), Contract object, Function object, Node (control flow graph node), and SlithIR (intermediate representation). Together, they form a comprehensive analysis framework.

- Using the **Slither** object, developers can load both local and deployed smart contracts (including contracts accessed through Etherscan) for in-depth static analysis.
  - Load a local project: `slither = Slither('/path/to/project')`
  - Load a deployed contract: `slither = Slither('0x..')`
  - Load with an Etherscan API key: `slither = Slither('0x..', etherscan_api_key='..')`
  - Access compilation units: `sl.compilation_units`
- At the **compilation unit** level, developers can retrieve all contracts and understand their inheritance relationships.
  - Get all contracts: `compilation_unit.contracts`
  - Get contracts not inherited by others: `compilation_unit.contracts_derived`
  - Get a contract by name: `compilation_unit.get_contract_from_name(name)`
- At the **contract** level, you can access functions, modifiers, state variables, and inheritance hierarchies.
- At the **function** level, you can analyze the control flow graph, and inspect read/write operations on state variables as well as local variables.
- At the **node** level, you can traverse the control flow graph either sequentially or non-sequentially:
  - Non-sequential traversal: `for node in function.nodes`
  - Sequential traversal: use recursive visiting functions
  - Multi-iteration analysis: bind to a fixed number of iterations or create a fixed point
- At the **SlithIR** level, you can use `isinstance()` to check for specific operation types such as external calls, mathematical operations, and more.

<details><summary>Code</summary>

```python
from slither import Slither
sl = Slither("0xdac17f958d2ee523a2206206994597c13d831ec7")
compilation_unit = sl.compilation_units[0]
contract = compilation_unit.get_contract_from_name("TetherToken")[0]
totalSupply = contract.get_function_from_signature("totalSupply()")

# Print the external call made in the totalSupply function
for ir in totalSupply.slithir_operations:
    if isinstance(ir, HighLevelCall):
        print(f"External call found {ir} ({ir.node.source_mapping})")
```

</details>

### 1.3 SlithIR: Powerful Intermediate Representation

**SlithIR** is the core intermediate representation (IR) system of the Slither static analysis framework for smart contracts. It transforms Solidity code into a form that is easier to analyze, enabling highly accurate security analysis. As an IR specifically designed for smart contracts, SlithIR addresses many of Solidity's unique features and edge cases — for example, treating an array `push` as a distinct operation rather than a generic function call. It also establishes a hierarchical system of operators, making data flow tracking between variables intuitive and precise.

SlithIR supports various types of variables and a rich set of operators, building a powerful foundation for advanced analyses. With SlithIR, Slither can perform complex taint analysis, tracing how user inputs affect contract state, and even analyzing data flow across multiple transactions — which is critical for detecting deep security vulnerabilities.

You can view the IR representation of each function using the command:

- `slither file.sol --print slithir`
  To see the SSA (Static Single Assignment) form of SlithIR, use:
- `slither file.sol --print slithir-ssa`
  In SSA form, each variable is assigned exactly once, and each modification creates a new version of the variable, making it especially useful for data dependency analysis.

Key features:

- **SlithIR supports various variable types**:
  - StateVariable, LocalVariable, Constant, SolidityVariable, TupleVariable, TemporaryVariable, and ReferenceVariable.
- **SlithIR defines a wide range of operators**, including:
  - Assignment, Binary Operation, Unary Operation, Indexing, Member Access, New Operators, Push Operator, Delete Operator, Conversion, Unpack, Array Initialization, various Call Operators, Return, and Conditionals.

### 1.4 Data Dependency Analysis

Slither’s **analysis module** also provides **data dependency analysis**, which identifies relationships between variables in a smart contract — specifically, determining whether the value of one variable is influenced by another. A key feature of this analysis is **context sensitivity**, meaning the dependency results can differ based on the chosen scope (e.g., within a single function or across the entire contract). For example, a state variable `b` may not depend on variable `a` within the context of the `setA` function, but across the entire contract, `b` could be dependent on `a` (and transitively on `input_a`).

Slither provides the function `is_dependent(variable, variable_source, context)` to perform this analysis, allowing you to check whether a variable depends on a source variable within a given context.

<details><summary>Code</summary>

```solidity
contract MyContract{
    uint a = 0;
    uint b = 0;

    function setA(uint input_a) public{
        a = input_a;
    }

    function setB() public{
        b = a;
    }

}
```

```python
from slither import Slither
from slither.analyses import is_dependent

slither = Slither('data_dependency_simple_example.sol')

myContract = slither.get_contract_from_name('MyContract')
funcA = myContract.get_function_from_signature('setA(uint256)')
input_a = funcA.parameters[0]

a = myContract.get_state_variable_from_name('a')
b = myContract.get_state_variable_from_name('b')

print(f'{b.name} is dependant from {input_a.name}?: {is_dependent(b, a, funcA)}')
print(f'{b.name} is dependant from {input_a.name}?: {is_dependent(b, a, myContract)}')

```

</details>

### 1.5 Printers: Visual and Structural Insight

Slither’s **Printers** module is a core feature of the static analysis framework, offering dozens of output tools designed to showcase the structure, behavior, and properties of smart contracts from multiple dimensions — not just to detect vulnerabilities. These printers cover every aspect of contract analysis: **structural analysis** (e.g., `inheritance`, `variable-order`) helps in understanding the architecture; **functional analysis** (e.g., `function-summary`, `function-id`, `modifiers`) reveals function behavior and interfaces; **control flow analysis** (e.g., `cfg`, `dominator`) and **data flow analysis** (e.g., `data-dependency`, `vars-and-auth`) dive deep into execution logic; **intermediate representation** printers (`slithir`, `slithir-ssa`) expose the low-level operations; **visualization tools** (`call-graph`, `inheritance-graph`) provide intuitive graphical views of relationships; and **complexity measurement tools** (`ck`, `halstead`, `martin`) assess code quality.

The usage is simple and consistent: just run `slither file.sol --print [printer-name]`. Some printers generate `.dot` files, which can be viewed with `xdot` or converted into other formats. These printers are extremely useful in practice — for example, `contract-summary` quickly summarizes contracts, `function-summary` provides detailed function insights, `data-dependency` analyzes variable relationships, `vars-and-auth` audits state variable permissions, `inheritance-graph` visualizes inheritance, and `require` helps locate critical security checks.

### 1.6 Slither Tools Overview

Beyond its powerful core static analysis engine, **Slither** offers a rich ecosystem of specialized tools to tackle various needs in smart contract security, code management, documentation, and quality assurance. These tools greatly enhance a developer’s or auditor’s ability to understand, test, and secure Solidity codebases efficiently. Here’s an overview:

#### 1.6.1 Slither-Simil

**Slither-Simil** combines static analysis with advanced machine learning to detect similar (and potentially vulnerable) Solidity functions. It uses Facebook’s **FastText** for embedding code into numerical vectors, and offers a pretrained model trained on 60,000 contracts and 850,000+ functions.

It provides four operation modes:

- **test**: Quickly finds functions similar to a target function across a large corpus (e.g., identify similar code among 800,000 functions in ~20 seconds).
- **train**: Train a new model based on custom datasets(`slither-simil train model.bin --input contracts`).
- **plot**: Visualize function similarity clusters using dimensionality reduction.
- **info**: Inspect model internals and function representations.

#### 1.6.2 Slither-Flat

**Slither-Flat** is a Solidity code flattener that consolidates code scattered across multiple files into a simpler structure, greatly easing the management of complex projects.  
It supports three flattening strategies:

- **MostDerived** (default): Outputs derived contracts individually.
- **OneFile**: Merges all contracts into a single file.
- **LocalImport**: Preserves file separation with local imports.

Additionally, it offers code patching options (e.g., changing `external` to `public` for compatibility with fuzzers like Echidna, removing `assert()` calls) and supports flexible output formats (directory output, JSON, ZIP archives).  
It elegantly handles circular dependencies and works seamlessly with major frameworks like Truffle, Embark, Buidler, and Etherlime.

#### 1.6.3 Slither-Documentation

**Slither-Documentation** is an experimental tool that automatically generates **NatSpec** comments for Solidity contracts. It leverages OpenAI’s **Codex** model, trained specifically for code understanding and generation, to automatically document functions, parameters, return values, and state variables — solving the common issue of missing or inadequate documentation in smart contract development.

#### 1.6.4 Slither-Doctor

**Slither-Doctor** is a dedicated troubleshooting tool that helps diagnose and solve technical issues encountered when running Slither on smart contract projects. It aims to make using Slither more reliable and user-friendly.

#### 1.6.5 Slither-Check-ERC

**Slither-Check-ERC** is a compliance checker for Ethereum standards. It verifies whether contracts properly implement various ERC standards, including ERC20, ERC721, ERC1155, and even more specialized ones like ERC4626. This tool ensures that your contract adheres strictly to the standard specifications.

#### 1.6.6 Slither-Interface

**Slither-Interface** automatically generates Solidity interface code from existing contracts (both local files and deployed contracts).  
It analyzes public function signatures, events, custom errors, enums, and structs to produce standardized interfaces, with options to:

- Unroll structs (`--unroll-structs`)
- Exclude events (`--exclude-events`)
- Exclude errors (`--exclude-errors`)
- Exclude enums and structs

#### 1.6.7 Slither-Mutate

**Slither-Mutate** is an experimental mutation testing tool designed specifically for Solidity contracts. It systematically introduces small mutations (e.g., changing operators, adjusting logic, deleting statements) to evaluate the effectiveness of your test suites. If a mutation causes a test failure, it suggests that the code is well-covered; if not, it may highlight blind spots. It integrates smoothly with frameworks like Foundry and offers custom options for selecting mutators, timeouts, and specific contracts. Example usage: `slither-mutate src --test-cmd='forge test'`.

#### 1.6.8 Slither-Find-Paths

**Slither-Find-Paths** identifies all possible call paths leading to a specific target function inside a smart contract, with a simple command `slither-find-paths file.sol contract.function`

it systematically maps out direct and indirect call chains, making it easier to understand complex function interactions.

#### 1.6.9 Slither-Prop

**Slither-Prop** is an automated property generation tool.
It analyzes smart contracts to automatically generate testable properties and invariants, wrapping them into executable test code.
The workflow involves:

- Generating test files
- Customizing the constructor if needed
- Running Truffle unit tests to validate the basic properties
- Performing advanced fuzz testing with Echidna

It particularly supports ERC20 contract properties like **Transferable**, **Pausable**, **NotMintable**, ensuring vital security checks like "no transfer to zero address," "user balance never exceeds total supply," etc.

#### 1.6.10 Slither-Read-Storage

**Slither-Read-Storage** is designed for smart contract storage analysis.
It retrieves and displays the contract’s storage slots layout and actual stored values, supporting:

- Local or deployed contracts
- Mapping keys, struct members
- Nested complex data structures (arrays, mappings of structs, etc.)
- Different block heights (with archive node access)

It offers JSON output and table views, supports variable-specific queries, and can even analyze proxy contracts by tracking storage layouts through proxies and implementations.

#### 1.6.11 Slither-Format

**Slither-Format** is an automated code formatting and patching tool.
It identifies common issues and generates **Git-compatible** patch files by `slither-format target`.
It can automatically fix:

- Unused state variables
- Outdated solc versions
- Inconsistent `pragma` directives
- Naming convention violations
- Functions missing `external` visibility
- State variables that can be made `constant`
- Functions that can be made `pure` or `view`

Patches are output to the `crytic-export/patches` directory.

#### 1.6.12 Slither-Check-Upgradeability

**Slither-Check-Upgradeability** specializes in auditing upgradeable contracts using the **delegatecall proxy pattern** (`slither-check-upgradeability project ContractName`).

It checks for many types of issues, categorized as:

- **High-risk** (e.g., variable constant changes, function ID collisions, shadowed functions, initialization issues)
- **Medium-risk** (e.g., extra variables, missing variables)
- **Informational** (e.g., new variables in v2, initialization suggestions)

It verifies storage layout consistency, checks compatibility between proxy and implementation contracts, and integrates well with upgrade frameworks like **OpenZeppelin SDK (ZOS)**.

---

## 2. Customizing Slither for Advanced Smart Contract Analysis

Slither is not just a set of predefined checks—it’s a modular platform for smart contract analysis. With support for custom detectors, tool development, and standardized output formats, it empowers developers and auditors to tailor static analysis to their specific needs, automate workflows, and build deeper insights into Solidity codebases.

Whether you're auditing for security or building next-gen development tools, Slither’s extensibility and powerful engine make it an essential component of the smart contract development ecosystem.

### 2.1 Custom Detectors

Slither, the powerful static analysis framework for Solidity smart contracts, offers highly extensible capabilities that range from basic syntax analysis (such as variable shadowing detection and interface consistency verification) to advanced semantic analysis (such as data dependency tracking and fixpoint-based reentrancy detection). Using specialized **detectors**, Slither identifies code patterns at various levels, targeting contracts, functions, variables, and more.

One of Slither’s strengths lies in its **data dependency analysis**, which traces the relationships between variables to detect subtle vulnerabilities. Its **fixpoint computation** allows it to efficiently handle loops and recursive structures, not by limiting the number of analysis passes, but by determining when no new information can be derived. Additionally, Slither translates Solidity code into its own intermediate representation, **SlithIR**, enabling sophisticated semantic analyses that are otherwise difficult to perform directly on Solidity source code.

For developers, building a simple detector only requires basic knowledge of syntax and semantics, while writing advanced detectors demands a deeper understanding of SlithIR and Slither’s internal analysis engine. Along with a rich set of built-in detectors for common vulnerabilities, Slither empowers developers to **create custom detectors**, making it an indispensable tool for both security auditing and maintaining code quality.

#### Adding Custom Detectors

To extend Slither’s detection capabilities, developers can create **custom detectors** by subclassing `AbstractDetector` and implementing the required structure:

- **ARGUMENT**: the command-line argument name (e.g., `'mydetector'`).
- **HELP**: a description of what the detector does.
- **IMPACT**: the severity of the issue (OPTIMIZATION, INFORMATIONAL, LOW, MEDIUM, HIGH), displayed in green or red.
- **CONFIDENCE**: the confidence level of the findings (LOW, MEDIUM, HIGH).
- **\_detect()**: the core method, which returns a list of findings. Each finding is generated with `self.generate_result(info)`, where `info` contains either a text description or references to contract objects (contracts, functions, nodes, etc.).

Within a detector, developers can access the entire Slither project and its analysis capabilities through the `slither` attribute.

New detectors can be integrated in two ways:

- **Direct Integration**: Add the detector to `slither/detectors/all_detectors.py`.
- **Plugin Package**: Create a standalone detector package.

For example, a basic `backdoor.py` detector could scan for any function names containing the word "backdoor," illustrating the process of developing a simple custom detector.

<details><summary>Code</summary>

```python
from slither.detectors.abstract_detector import AbstractDetector, DetectorClassification

class Skeleton(AbstractDetector):
    """
    Documentation
    """

    ARGUMENT = 'mydetector' # slither will launch the detector with slither.py --detect mydetector
    HELP = 'Help printed by slither'
    IMPACT = DetectorClassification.HIGH
    CONFIDENCE = DetectorClassification.HIGH

    WIKI = ''

    WIKI_TITLE = ''
    WIKI_DESCRIPTION = ''
    WIKI_EXPLOIT_SCENARIO = ''
    WIKI_RECOMMENDATION = ''

    def _detect(self):
        info = ['This is an example']
        res = self.generate_result(info)

        return [res]
```

```python
from slither.detectors.abstract_detector import AbstractDetector, DetectorClassification

class Backdoor(AbstractDetector):
    """
    Detect function named backdoor
    """

    ARGUMENT = "backdoor"  # slither will launch the detector with slither.py --mydetector
    HELP = "Function named backdoor (detector example)"
    IMPACT = DetectorClassification.HIGH
    CONFIDENCE = DetectorClassification.HIGH

    WIKI = "https://github.com/trailofbits/slither/wiki/Adding-a-new-detector"
    WIKI_TITLE = "Backdoor example"
    WIKI_DESCRIPTION = "Plugin example"
    WIKI_EXPLOIT_SCENARIO = ".."
    WIKI_RECOMMENDATION = ".."

    def _detect(self):
        results = []

        for contract in self.slither.contracts_derived:
            # Check if a function has 'backdoor' in its name
            for f in contract.functions:
                if "backdoor" in f.name:
                    # Info to be printed
                    info = ["Backdoor function found in ", f, "\n"]

                    # Add the result in result
                    res = self.generate_result(info)

                    results.append(res)

        return results
```

</details>

### 2.2 Creating Custom Tools

Slither also allows the development of **custom tools** using its analysis capabilities as a library. The `tools/demo` directory provides a scaffold for new tool development.

To integrate a new tool into the command-line interface:

- Update the `entry_points` section of the `setup.py` file to register the tool.
- Follow best practices during development:
  - Prefer the **Python logging** module over simple `print` statements for output control.
  - Use **exceptions** instead of `sys.exit` to make the tools more modular.
  - Write **unit tests** to validate your tool’s behavior and reliability.

<details><summary>Code</summary>

```python
import argparse
import logging
from crytic_compile import cryticparser
from slither import Slither

logging.basicConfig()
logging.getLogger("Slither").setLevel(logging.INFO)

logger = logging.getLogger("Slither-demo")

def parse_args() -> argparse.Namespace:
    """
    Parse the underlying arguments for the program.
    :return: Returns the arguments for the program.
    """
    parser = argparse.ArgumentParser(description="Demo", usage="slither-demo filename")

    parser.add_argument(
        "filename", help="The filename of the contract or truffle directory to analyze."
    )

    # Add default arguments from crytic-compile
    cryticparser.init(parser)

    return parser.parse_args()

def main() -> None:
    args = parse_args()

    # Perform slither analysis on the given filename
    _slither = Slither(args.filename, **vars(args))

    logger.info("Analysis done!")

if __name__ == "__main__":
    main()
```

</details>

### 2.3 JSON Output for Integration

Slither’s `--json` option generates standardized JSON output, delivering machine-readable static analysis results that facilitate integration with CI/CD pipelines, IDE plugins, and security toolchains. This structured output allows developers to automate security checks and streamline workflows.

The JSON structure includes:

- **Top-level fields**:

  - `success`: whether the analysis completed successfully.
  - `error`: any encountered errors.
  - `results`: the main analysis findings.

- **Inside `results`**:
  - `detectors`: an array containing results from vulnerability detectors.
  - `upgradeability-check`: an object containing upgradeability analysis results, if applicable.

Each entry in the `detectors` array contains:

- `check`: the detector’s identifier.
- `impact`: the severity level.
- `confidence`: the confidence in the finding.
- `description`: a textual description of the issue.
- `elements`: an array of associated code elements.

Each `element` provides precise source code information:

- Type, name, and `source_mapping`, which includes:
  - Start position
  - Length
  - File path (in multiple formats)
  - Line and column numbers

Some detectors provide additional context-specific fields. For example:

- The `constant-function` detector might include a `contain_assembly` flag.
- The `naming-convention` detector may include `convention` and `target` fields.

For upgradeability analysis (`slither-check-upgradeability`), the JSON output includes specialized fields covering initializer checks, function ID comparisons, variable ordering, and more.

---

## 3. Examples of Slither in Action

Slither has proven its flexibility and power by enabling the detection of complex, real-world smart contract vulnerabilities. Below are several notable examples showcasing how custom Slither detectors and tools have been used to tackle critical issues.

### 3.1 Detecting Out-of-Order Execution in Arbitrum Retryable Tickets

**Arbitrum Nitro** enables communication between L1 and L2 through a mechanism called **retryable tickets**, created by calling the `createRetryableTicket` function in the L1 Inbox contract. However, this system introduces a significant security risk: **retryable transactions are not guaranteed to execute in the order they are created**.

When retryable transactions fail (for example, due to insufficient gas), they are stored in a memory pool for up to a week, and **anyone** can manually redeem them in any order. This can lead to several dangerous scenarios:

- **Multiple transactions fail** and are redeemed out of order.
- **Partial transaction success** causes inconsistencies between expected and actual states.
- **New rounds of transactions** are created before failed earlier transactions are executed, leading to unexpected behaviors.

Protocols that rely on transaction ordering—such as "claim rewards first, then unstake"—are particularly vulnerable. If an unstake transaction executes before the rewards are claimed, users may permanently lose their rewards.

To address this risk, a new Slither detector called **`out-of-order-retryable`** was developed. This detector helps developers identify contracts that incorrectly assume retryable tickets will execute sequentially.

Mitigations include:

- **Combining critical operations** into a single transaction.
- **Designing logic** that can gracefully handle out-of-order execution.

<details><summary>Code</summary>

```solidity
/**
    * @notice Put a message in the L2 inbox that can be reexecuted for some fixed amount of time if it reverts
    * @dev all msg.value will deposited to callValueRefundAddress on L2
    * @dev Gas limit and maxFeePerGas should not be set to 1 as that is used to trigger the RetryableData error
    * @param to destination L2 contract address
    * @param l2CallValue call value for retryable L2 message
    * @param maxSubmissionCost Max gas deducted from user's L2 balance to cover base submission fee
    * @param excessFeeRefundAddress gasLimit x maxFeePerGas - execution cost gets credited here on L2 balance
    * @param callValueRefundAddress l2Callvalue gets credited here on L2 if retryable txn times out or gets cancelled
    * @param gasLimit Max gas deducted from user's L2 balance to cover L2 execution. Should not be set to 1 (magic value used to trigger the RetryableData error)
    * @param maxFeePerGas price bid for L2 execution. Should not be set to 1 (magic value used to trigger the RetryableData error)
    * @param data ABI encoded data of L2 message
    * @return unique message number of the retryable transaction
    */
function createRetryableTicket(
    address to,
    uint256 l2CallValue,
    uint256 maxSubmissionCost,
    address excessFeeRefundAddress,
    address callValueRefundAddress,
    uint256 gasLimit,
    uint256 maxFeePerGas,
    bytes calldata data
) external payable returns (uint256);
```

```solidity
function claim_rewards(address user) public onlyFromL1 {
    // rewards is computed based on balance and staking period
    uint unclaimed_rewards = _compute_and_update_rewards(user);
    token.safeTransfer(user, unclaimed_rewards);
}

// Call claim_rewards before unstaking, otherwise you lose your rewards
function unstake(address user) public onlyFromL1 {
    _free_rewards(user); // clean up rewards related variables
    balance = balance[user];
    balance[user] = 0;
    staked_token.safeTransfer(user, balance);
}
```

```solidity
// Retryable A
IInbox(inbox).createRetryableTicket({
    to: l2contract,
    l2CallValue: 0,
    maxSubmissionCost: maxSubmissionCost,
    excessFeeRefundAddress: msg.sender,
    callValueRefundAddress: msg.sender,
    gasLimit: gasLimit,
    maxFeePerGas: maxFeePerGas,
    data: abi.encodeCall(l2contract.claim_rewards, (msg.sender))
});
// Retryable B
IInbox(inbox).createRetryableTicket({
    to: l2contract,
    l2CallValue: 0,
    maxSubmissionCost: maxSubmissionCost,
    excessFeeRefundAddress: msg.sender,
    callValueRefundAddress: msg.sender,
    gasLimit: gasLimit,
    maxFeePerGas: maxFeePerGas,
    data: abi.encodeCall(l2contract.unstake, (msg.sender))
});
```

</details>

### 3.2 Identifying `msg.value` Misuse in Loops

The **`msg-value-loop`** and **`delegatecall-loop`** detectors were specifically created to uncover vulnerabilities similar to those found in the historical SushiSwap MISO and Opyn contracts—issues that once threatened over **$350 million** worth of funds.

While the two vulnerabilities had different appearances, they shared the same underlying flaw: **reusing the same `msg.value` across multiple iterations inside a loop**.

- In the Opyn case, the vulnerability occurred because the payable function reused `msg.value` multiple times within a loop.
- In the MISO case, the vulnerability was more subtle, involving a `delegatecall` inside a loop, which implicitly reused the original `msg.value`.

Leveraging Slither’s **control flow graph** and **SlithIR intermediate representation**, these detectors operate by:

- **`msg-value-loop`**: Scanning loops for repeated accesses to `msg.value`.
- **`delegatecall-loop`**: Identifying loops within payable functions that contain `delegatecall` operations.

Both detectors have been validated against real-world exploits and provide developers with effective tools to prevent similar catastrophic vulnerabilities in future smart contracts.

<details><summary>Code</summary>

```solidity
contract C {
    mapping (address => uint256) balances;
    function addBalances(address[] memory receivers) public payable {
            for (uint256 i = 0; i < receivers.length; i++) {
                                balances[receivers[i]] += msg.value;
}
    }
}
```

```solidity
contract C {
    mapping (address => uint256) balances;

    function addBalance(address a) public payable {
                balances[a] += msg.value;
    }

    function addBalances(address[] memory receivers) public payable {
                for (uint256 i = 0; i < receivers.length; i++) {
                                address(this).delegatecall(abi.encodewithsignature(“addBalance(address)”, receivers [i]));
}
    }
}
```

</details>

### 3.3 Exploring Contract Storage with `slither-read-storage`

**`slither-read-storage`** is a specialized tool designed for parsing and retrieving smart contract storage slots, providing developers and researchers with deep insight into the Solidity storage layout.

This tool uses Slither's type analysis to accurately compute the storage layout for complex structures, including mappings and dynamic arrays. It can also **query actual on-chain values** via an Ethereum RPC endpoint. Compared to manual calculations or browsing Etherscan, `slither-read-storage` offers substantial time savings, reduces human error, and enables reliable access to internal contract states without requiring publicly exposed getter functions.

Key use cases include:

- **On-chain security research**:  
  Directly query critical state variables without relying on contract interfaces.  
  Example: Fetching the `frax_pools_array` from the FRAX token contract:

  ```bash
  slither-read-storage 0x853d955aCEf822Db058eb8505911ED77F175b99e --variable-name frax_pools_array --rpc-url $RPC_URL --value
  ```

- **Arbitrage bot optimization**:  
  Retrieve key price data (e.g., Uniswap V3 `sqrtPriceX96`) with minimal RPC overhead.  
  Example:

  ```bash
  slither-read-storage 0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8 --layout
  ```

- **Asset management systems**:  
  Monitor ERC20 balance changes without relying on slow or unreliable external data.  
  Example:

  ```bash
  slither-read-storage 0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984 --variable-name balances --key 0xab5801a7d398351b8be11c439e05c5b3259aec9b --rpc-url $RPC_URL --value
  ```

- **Handling upgradeable proxy contracts**:  
  Correctly retrieve storage data by specifying both the proxy and logic contract addresses.  
  Example:
  ```bash
  slither-read-storage 0xa2327a938Febf5FEC13baCFb16Ae10EcBc4cbDCF --variable-name balances --key 0xab5801a7d398351b8be11c439e05c5b3259aec9b --storage-address 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 --rpc-url $RPC_URL --value
  ```

### 3.4 Dataflow Analysis and its Application to Solidity Smart Contract Verification

**Dataflow analysis** is a mature paradigm in static program analysis, originating from the field of compiler optimization. It simulates runtime behavior without executing the program, analyzing the possible values and states of variables and expressions at each point in a program's control flow graph (CFG). While dataflow analysis depends heavily on control flow analysis, the two serve different purposes. Dataflow analysis can be categorized based on flow direction (forward/backward), analysis scope (intra-procedural/inter-procedural), flow sensitivity, and path sensitivity (may/must analysis). Typical applications include live variable analysis, available expression analysis, and constant propagation.

Formally, dataflow analysis is defined on the control flow graph $G(P) = (V, E, cmd)$ of a program $P$, where:

- $V$ is the set of vertices (program points),
- $E \subseteq V \times V$ is the set of edges (control flow),
- $cmd$ represents the commands or statements in the program.

The dataflow system $S = (Lab, Ext, Flw, (D, ⊑), ε, ϕ)$ consists of:

- $Lab = V$, the set of program labels,
- $Ext$, a subset of labels marking the program's start and end points,
- $Flw$, the flow relations: $Flw = E$ for forward analysis and $Flw = E^{-1}$ for backward analysis,
- $(D, ⊑)$, a complete lattice of dataflow values,
- $ε$, the initial state values,
- $ϕ$, the transfer functions $ϕ_\ell: D → D$ that describe how statements affect program states.

The solution to the dataflow analysis is obtained by computing the least fixed point of the associated equation system, typically using iterative algorithms such as worklist methods. To prevent non-termination in the presence of loops, _widening_ and _narrowing_ operators are employed.

#### Interval Analysis in Solidity

**Interval analysis** is a form of static analysis that approximates the possible values of program variables using numerical intervals. It is a practical instance of abstract interpretation. We extend traditional interval analysis by formalizing it using the dataflow system $S$ and introducing a _constraint set_ $Con$ to capture the restrictions imposed by conditional statements such as `if`, loops, and Solidity-specific constructs like `require` and `assert`.

Furthermore, we recursively expand the domain of variable values (Val) to support complex types like integers, booleans, structs, arrays, and mappings. We adapt set operations like subset ($⊆$), union ($∪$), and intersection ($∩$) to handle these types, addressing challenges in constraint processing and ensuring termination in loops. To verify the satisfiability of states under constraints, SMT solvers like Z3 are employed.

A practical example is given with the `magicNumber` function, demonstrating how to track variable states and accumulated constraints after each program statement. Variables such as `x`, `index`, and `value` have their possible interval values updated, and constraints are accumulated. The symbol $∅$ indicates uninitialized variables, and $[0, ∞]$represents the interval from zero to positive infinity.

<details><summary>Code</summary>

```solidity
function magicNumber(uint x) pure external returns(uint) {
    uint index = 0;         // statement 1
    uint value = x;         // statement 2
    require(x < 15);        // statement 3
    while(index < x)        // statement 4
    {
        if(index % 2 == 0)  // statement 5
        {
            value = value * 2; // statement 6
        }
        else
        {
            value = value * 3; // statement 7
        }
        x = x + 1;          // statement 8
    }
    return value;           // statement 9
}

```

| Statements | x     | index | value | Constraints                     |
| ---------- | ----- | ----- | ----- | ------------------------------- |
| 1          | [0,∞] | ∅     | ∅     | {}                              |
| 2          | [0,∞] | [0,0] | ∅     | {}                              |
| 3          | [0,∞] | [0,0] | [0,∞] | {}                              |
| 4          | [0,∞] | [0,∞] | [0,∞] | {x<15}                          |
| 5          | [0,∞] | [0,∞] | [0,∞] | {x<15 && index<x}               |
| 6          | [0,∞] | [0,∞] | [0,∞] | {x<15 && index<x && index%2==0} |
| 7          | [0,∞] | [0,∞] | [0,∞] | {x<15 && index<x && index%2!=0} |
| 8          | [0,∞] | [0,∞] | [0,∞] | {x<15}                          |
| 9          | [0,∞] | [0,∞] | [0,∞] | {x<15}                          |
| end        | [0,∞] | [0,∞] | [0,∞] | {x<15}                          |
|            |       |       |       |                                 |

</details>

#### A Proposed Slither Solution

Although **Slither** is a powerful static analysis tool for Solidity smart contracts, it has two significant limitations:

- **Lack of Fine-grained State Querying**: Slither does not support querying the contract state at every execution point within a function, and it lacks approximations like interval analysis.
- **Constraint Ignorance**: Slither fails to consider constraints imposed by previous statements, which leads to its inability to detect unreachable branches. For instance, in a contract containing `require(msg.value > 10)`, any subsequent condition like `newBid > 10` is always true due to Solidity’s unsigned integers, making the `else` branch logically unreachable—yet Slither cannot detect this inconsistency.

To address these issues, we propose a system that combines Slither as a **contract parser** and **Z3** as a **constraint solver**. The system workflow consists of:

- Contract parsing,
- Information extraction,
- Fixed-point computation using minimal fixed-point algorithms,
- Constraint solving, and
- Result reporting.

The system adopts design patterns such as _adapters_, _composite structures_, and _template methods_ to ensure modularity and extensibility, facilitating the integration of other third-party tools.

Key technical highlights include:

- Custom variable representations supporting additional metadata,
- Dictionary-based representations of complex types like arrays, mappings, and structs,
- Program state representations that separate numerical and boolean variables,
- Implementation of widening operators to guarantee loop termination,
- Carefully designed constraint flow mechanisms: constraints propagate downward in the CFG but do not cross loop-back edges.

During the implementation, significant challenges were overcome, including representation conversion and constraint identification, particularly managing constraints across loop structures.

<details><summary>Code</summary>

```solidity
pragma solidity 0.8.23;

contract BidContract {
    mapping(uint => uint) public bidders;
    function bid(uint bidderNumber) public payable {
        require(msg.value > 10);
        uint newBid = bidders[bidderNumber] + msg.value;
        if(newBid > 10)
        {
            // Since msg.value >10 implies that newBid >10 ,
            // this brach will always execute
            bidders[bidderNumber] = newBid;
        }
        else
        {
            // Since the " then " branch is based on a tautology ,
            // this branch will never execute
            revert("Inssuficient bid");
        }
    }
}
```

```python
def get_least_upper_bound_numeric_interval ( first_element :
    NumericIntervalApproxValue , second_element : NumericIntervalApproxValue ) :
    if len( first_element ) == 0:
        return second_element
    if len( second_element ) == 0:
        return first_element
    lower_bound = first_element [0] if first_element [0] <= second_element [0] else
        float (
        "-inf")
    upper_bound = first_element [1] if first_element [1] >= second_element [1] else
        float (
        "inf")
    return NumericIntervalApproxValue (( lower_bound , upper_bound ) )
```

</details>

---

## 4. Other Static Analysis Tools

### 4.1 SARIF Explorer

SARIF Explorer is a VSCode extension designed to streamline the triage of static analysis results. It offers a powerful set of features, including support for opening and classifying multiple SARIF files at once, an intuitive results navigation pane that allows users to jump directly to code locations and view data flow steps, and an efficient triage system where findings can be labeled as "bug," "false positive," or "todo," with the ability to add custom comments and use keyboard shortcuts.  
Advanced filtering capabilities allow users to combine criteria like keywords, inclusion/exclusion paths, severity levels, and classification states. SARIF Explorer also integrates GitHub features for copying permalinks or creating formatted issue reports and seamlessly connects with tools like weAudit for one-click submission of marked bugs.  
Developed to address real-world challenges faced during security audits using Semgrep, CodeQL, and other static analysis tools, SARIF Explorer revolutionizes the workflow—moving from manually sifting through results in terminal windows to a complete professional pipeline: from analysis tool output to team collaboration and final reporting. It significantly improves efficiency and usability for security engineers, removing key barriers to broader adoption of static analysis in security work.

### 4.2 Fickling

Fickling is a decompiler, static analyzer, and bytecode rewriter for Python’s `pickle` module, helping detect, analyze, and even craft malicious pickle files.  
It addresses persistent security issues in the machine learning (ML) ecosystem through three key capabilities: a modular analysis API that outputs detailed JSON reports, breaking down findings into specific categories of malicious behavior; an extended PyTorch module that supports static analysis and code injection into PyTorch files, enabling deeper verification; and a polymorphic file module that identifies, differentiates, and creates polymorphic files across PyTorch versions (from v0.1.1 to v1.3, TorchScript v1.0 to v1.4, and beyond).  
Fickling can safely analyze potentially dangerous files by symbolically executing a custom-built Pickle Machine instead of running arbitrary code. It also enables the creation of polymorphic test files that can trigger vulnerabilities across different formats. Despite its power, the recommendation remains to abandon `pickle` entirely in favor of safer serialization methods like `safetensors`, while promoting better security practices through tools like Semgrep and continued vulnerability reporting.

<details><summary>Code</summary>

```python
import torch
import torchvision.models as models

from fickling.pytorch import PyTorchModelWrapper

# Load example PyTorch model

model = models.mobilenet_v2()
torch.save(model, "mobilenet.pth")

# Wrap model file into fickling

result = PyTorchModelWrapper("mobilenet.pth")

# Inject payload, overwriting the existing file instead of creating a new one

temp_filename = "temp_filename.pt"
result.inject_payload(
    "print('!!!!!!Never trust a pickle!!!!!!')",
    temp_filename,
    injection="insertion",
    overwrite=True,
)

# Load file with injected payload

# This outputs “!!!!!!Never trust a pickle!!!!!!”.

torch.load("mobilenet.pth")

```

</details>

### 4.3 Circomspect and Sindri

**Circomspect** is a static analysis tool specifically for the Circom framework, designed to help developers write safer zero-knowledge proof circuits. Circom development is especially challenging: circuits are complex, and even basic tests can take minutes or longer to execute, drastically slowing iteration cycles.
**Sindri** accelerates circuit execution via specialized hardware and offers a simple API and CLI tools, removing the burden of managing complicated infrastructure.
The Sindri CLI aims to become a cross-framework, general-purpose tool for static analysis, code checking, compilation, and proving. Circomspect, regarded as one of the best tools for secure Circom development, is one of its most critical integrations.

### 4.4 Macroni

**Macroni** addresses long-standing shortcomings in Clang’s static analysis of macros. It smartly lowers C code and macros into MLIR (Multi-Level Intermediate Representation), allowing developers to build fully macro-aware static analysis tools.
Macroni balances enhancing the C language and maintaining backward compatibility, as shown through several key use cases:

- Implementing _STRONG_TYPEDEF_ for true strong typedefs without breaking API or ABI compatibility.
- Improving the _Sparse_ tool for Linux kernel security checking by hooking into macros like `__user`.
- Enabling _Rust-like_ unsafe blocks within C/C++/Objective-C codebases.
- Building safer signal handling systems with _SIG_HANDLER_ and _SIG_SAFE_ macros to enforce signal-safety constraints.
  By combining macro awareness and MLIR, Macroni opens the door to defining custom analysis, transformations, and even new language features — without sacrificing compatibility with existing codebases.

<details><summary>Code</summary>

1. Stronger typedef

```c
typedef double fahrenheit;
typedef double celsius;
fahrenheit F;
celsius C;
F = C; // No compiler error or warning

```

```c
#define STRONG_TYPEDEF(name) name
typedef double STRONG_TYPEDEF(fahrenheit);
typedef double STRONG_TYPEDEF(celsius);

```

2. Linux Sparse

```c
# define __user __attribute__((noderef, address_space(__user)))

```

3. unsafe

```c
#define unsafe if (0); else

fahrenheit convert(celsius C) {
    fahrenheit F;
    unsafe {
        F = (C * 9.0 / 5.0) + 32.0;
    }
    return F;
}

```

4. Signal Handling

```c
#define SIG_HANDLER(name) name
#define SIG_SAFE(name) name

int SIG_SAFE(do_detach)(int, const char*);
static void SIG_HANDLER(sig_handler)(int signo) { ... }

```

</details>

### 4.5 Semgrep for Machine Learning Security

**Semgrep** has proven valuable in securing machine learning systems by identifying three prevalent risks:

- Arbitrary code execution through `pickle` serialization, especially in PyTorch distributed computing and NumPy usage (e.g., CVE-2019-6446).
- Poor randomization in datasets caused by improper use of NumPy random number generators, leading to ineffective data augmentation.
- Incompatibility between NumPy operations and PyTorch’s ONNX export or symbolic tracing features.  
   Machine learning security should be end-to-end, covering preprocessing pipelines, models, and underlying hardware, and Semgrep is well-suited for catching these hidden flaws.

<details><summary>Code</summary>

1. PyTorch Distributed Pickle

```yaml
rules:
- id: pickles-in-torch-distributed
    patterns:
    - pattern-either:
        - pattern: torch.distributed.broadcast_object_list(...)
        - pattern: torch.distributed.all_gather_object(...)
        - pattern: torch.distributed.gather_object(...)
        - pattern: torch.distributed.scatter_object_list(...)
    message: |
    Functions reliant on pickle can result in arbitrary code execution.
    For more information, see <https://blog.trailofbits.com/2021/03/15/never-a-dill-moment-exploiting-machine-learning-pickle-files/>
    languages: [python]
    severity: WARNING

```

2. NumPy Pickle

```yaml
rules:
    - id: pickles-in-numpy
    patterns:
        - pattern: numpy.load(..., allow_pickle=$VALUE)
        - metavariable-regex:
            metavariable: $VALUE
            regex: (True|^\\d*[1-9]\\d*$)
    message: |
        Functions reliant on pickle can result in arbitrary code execution.
        Consider using fickling or switching to a safer serialization method.
    languages:
        - python
    severity: ERROR

```

3. Randomness

```yaml
rules:
- id: numpy-in-torch-datasets
    patterns:
    - pattern-either:
        - pattern: |
            class $X(Dataset):
            ...
            def __getitem__(...):
                ...
                np.random.randint(...)
                ...
        # ...
    message: |
    Using the NumPy RNG inside of a Torch dataset can lead to a number of issues...
    languages: [python]
    severity: WARNING

```

</details>

### 4.6 Amarna for Cairo Language

**Amarna** is a static analysis and linting tool for the Cairo programming language, essential in the emerging “provable programs” space. Cairo development introduces unique security challenges, such as:

- _Hints_: Cairo allows embedding arbitrary Python code via "hints," a major attack surface.
- _Memory Misuse_: Poor data structure design and recursion can lead to control of uninitialized memory.
- _Non-Deterministic Jumps_: Control flow can be hijacked if jumps depend on hints' values.  
  Amarna uses a custom syntax tree analysis system built with the _lark_ toolkit, supporting three rule types (local, collection, and postprocessing rules) and currently detects 10+ categories of issues, from arithmetic overflow risks to inconsistent assertions.  
  Amarna outputs SARIF reports for easy integration into IDEs like VSCode and is set to expand with more complex analyses like dataflow tracking in the future.

<details><summary>Code</summary>

1. Basic Cairo Program Example (Computing Pedersen Hash Function):

```cairo
# validate_hash.cairo
%builtins output pedersen

from starkware.cairo.common.cairo_builtins import HashBuiltin
from starkware.cairo.common.hash import hash2
from starkware.cairo.common.serialize import serialize_word

func main{output_ptr:felt*, pedersen_ptr : HashBuiltin*}():
    alloc_locals
    local input
    %{ ids.input = 4242 %}  # Using a hint to set the input value

    # Computes the Pedersen hash of the tuple (input, 1)
    let (hash) = hash2{hash_ptr=pedersen_ptr}(input, 1)

    # Prints the computed hash
    serialize_word(hash)

    return ()
end

```

2. Hints Usage Example:

```cairo
%builtins output

from starkware.cairo.common.serialize import serialize_word

func main{output_ptr:felt*}():
    # Arbitrary Python code
    %{
        import os
        os.system('whoami')  # Execute system command
    %}

    # Prints 1
    serialize_word(1)

    return ()
end

```

3. Using Hints to Calculate Square Root:

```cairo
func sqrt(n) -> (res):
    alloc_locals
    local res

    # Set the value of res using a Python hint
    %{
        import math
        # Use the ids variable to access the value of a Cairo variable
        ids.res = int(math.sqrt(ids.n))
    %}

    # The following line guarantees that `res` is a square root of `n`
    assert n = res * res
    return (res)
end

```

4. Recursion and Underconstrained Structures Example (Squaring Array Elements):

```cairo
# Fills `new_array` with the squares of the first `length` elements in `array`
func _inner_sqr_array(array : felt*, new_array : felt*, length : felt):
    # Recursion base case
    if length == 0:
        return ()
    end

    # Recursive case: the first element of the new_array will
    # be the first element of the array squared
    assert [new_array] = [array] * [array]

    # Recursively call, advancing the arrays and decrementing the length
    _inner_sqr_array(array=array + 1, new_array=new_array + 1, length=length - 1)
    return ()
end

func sqr_array(array : felt*, length : felt) -> (new_array : felt*):
    alloc_locals
    # Allocates an arbitrary length array
    let (local res_array) = alloc()

    # Fills the newly allocated array with squares of elements from input array
    _inner_sqr_array(array, res_array, length)
    return (res_array)
end

```

5. Malicious Exploitation of Zero-Length Recursion:

```cairo
func sqr_array(array : felt*, length : felt) -> (new_array : felt*):
    alloc_locals
    let (local res_array) = alloc()

    %{  # Write to the result array if the length is 0
        if ids.length == 0:
            data = [1, 3, 3, 7]  # Malicious data
            for idx, d in enumerate(data):
                memory[ids.res_array + idx] = d
    %}

    _inner_sqr_array(array, res_array, length)
    return (res_array)
end

```

6. Vulnerable Nondeterministic Jump Example:

```cairo
func are_equal(x, y) -> (eq):
    # Sets the ap register to True or False depending on the equality of x and y
    %{ memory[ap] = ids.x == ids.y %}

    # Jump to the label equal if the elements were equal
    jmp equal if [ap] != 0; ap++

    # Case x != y
    not_equal:
    return (0)

    # Case x == y
    equal:
    return (1)
end

```

7. Fixed Nondeterministic Jump Example:

```cairo
func are_equal(x, y) -> (eq):
    %{ memory[ap] = ids.x == ids.y %}
    jmp equal if [ap] != 0; ap++

    # Case x != y
    not_equal:
    # We are in the not_equal case
    # so we can't have equal x and y
    if x == y:
        # Add unsatisfiable assert
        assert x = x + 1
    end
    return (0)

    # Case x == y
    equal:
    # We are in the equal case
    # so x and y must equal
    assert x = y
    return (1)
end

```

8. Simpler Equality Check Implementation:

```cairo
func are_equal(x, y) -> (eq):
    if x == y:
        return (1)
    else:
        return (0)
    end
end

```

</details>

### 4.7 Static and Dynamic Analysis in Go

Common pitfalls in Go programming—scoping issues, goroutine leaks, poor error handling, and fragmented dependency management—make static analysis especially challenging.  
Popular tools like `go vet`, `staticcheck`, `errcheck`, and `ineffassign` are useful but limited. For deeper security testing:

- **Fuzzing** (e.g., `go-fuzz`, `gofuzz`) explores unexpected inputs.
- **Property testing** (`testing/quick`, `gopter`) verifies invariants across generated data.
- **Fault injection** (`krf`, `on-edge`) reveals hidden bugs by simulating system faults.  
  Go’s compiler also offers security testing advantages:
- _//go:linkname_ accesses internal functions.
- Coverage-guided testing highlights untested paths.
- 32-bit vs 64-bit platform testing uncovers type safety issues.  
  While Go’s ecosystem remains fragmented, with the adoption of `go.mod`, dependency auditing is becoming easier and more reliable.

<details><summary>Code</summary>

1. short variable declaration

```go
func A() (bool, error) { return false, fmt.Errorf("I get overridden!") }
func B() (bool, error) { return true, nil }

func main() {
    aSuccess, err := A()  // The error from A() is lost
    bSuccess, err := B()  // This reassigns err, hiding the previous error
    if err != nil {
        fmt.Println(err)
    }
    fmt.Println(aSuccess, ":", bSuccess)
}
```

2. Property Testing

```go
properties.Property("Divide should never fail.", prop.ForAll(
    func(a uint32, b uint32) bool {
    inpCompute := Compute{A: a, B: b}
    inpCompute.CoerceInt()
    inpCompute.Divide()
    return true
    },
    gen.UInt32Range(0, math.MaxUint32),
    gen.UInt32Range(0, math.MaxUint32),
))
```

3. Fault Injection

```go
// The error is logged but not returned to the caller
stdoutb, souterr := ioutil.ReadAll(stdoutp)
if souterr != nil {
    klog.Errorf("Failed to read from stdout for cmd %v - %v", cmd.Args, souterr)
}

// Later code tries to index an empty stdout
usageInKb, err := strconv.ParseUint(strings.Fields(stdout)[0], 10, 64)
```

</details>

### 4.8 CodeQL for Iterator Invalidations in C++

**CodeQL** can detect subtle iterator invalidation bugs in C++, which often lead to undefined behavior and serious vulnerabilities.
Iterators in C++ involve operations like dereferencing and incrementing, and mistakes—such as calling `erase` inside a loop—can easily invalidate them.
Using examples from projects like _Cataclysm: Dark Days Ahead_, CodeQL models iterators through an object-oriented schema:

- **Iterator** (the variable)
- **Iterated** (the container)
- **Invalidator** (the risky operation)
- **Invalidation** (the detected bug)
  The `InvalidationFlows` query connects these elements via dataflow analysis. CodeQL’s approach balances detection depth with manageable false positives, identifying even deeply hidden vulnerabilities, such as multi-layer function call issues in Google’s regex libraries and Apple’s LLVM forks.

#### CodeQL for OpenSSL Misuse Detection

Specialized CodeQL queries also target **OpenSSL’s libcrypto API** to catch two major misuse patterns:

- **Key Size Mismatches**: By modeling how encryption algorithms like `EVP_aes_256_cbc` encode key sizes in their names and tracing key flow through the code, CodeQL can detect incorrect key lengths.
- **Engine Initialization Errors**: By modeling how cryptographic engines should be created and initialized (`ENGINE_init`, `ENGINE_set_default`), CodeQL can spot missing calls that might cause memory leaks or operational failures.
  Even minor misuses of OpenSSL can lead to catastrophic security breaches, but these CodeQL queries offer a practical way to enforce correct usage at scale.

<details><summary>Code</summary>

1. Iterator Invalidation

```sql
struct Frame {
    Frame(Regexp** sub, int nsub)
        : sub(sub),
        nsub(nsub),
        round(0) {}

    Regexp** sub;
    int nsub;
    int round;
    std::vector<Splice> splices;
    int spliceidx;
};

int Regexp::FactorAlternation(Regexp** sub, int nsub, ParseFlags flags) {
    std::vector<Frame> stk;
    stk.emplace_back(sub, nsub);

    for (;;) {
    ...
    auto& splices = stk.back().splices;
    auto& spliceiter = stk.back().spliceiter;

    if (splices.empty()) {
        round++;
    } else if (spliceiter != splices.end()) {
        stk.emplace_back(spliceiter->sub, spliceiter->nsub);
        continue;
    } else { ... }

    switch (round) { ... }

    if (splices.empty() || round == 3) {
        spliceiter = splices.end();
    } else {
        spliceiter = splices.begin();
    }
    }
}
```

2. Key Size Mismatches

```sql
class Key extends Expr {
    Key() {
    exists(FunctionCall init |
        init.getTarget().getName() = "EVP_EncryptInit_ex" and
        this.flowsTo(init.getArgument(init.getTarget().getParameter(this.getKey()).getIndex())
    )
    }

    int getKey() { result = 3 }
}

```

```sql
int getKeySize() {
    result = this.getUnderlyingType().getSize()
}

```

```sql
class EVP_CIPHER extends FunctionCall {
    int keySize;

    EVP_CIPHER() {
    exists(string name |
        name = this.getTarget().getName() and
        (
        name.matches("EVP_aes_%_ecb") and keySize = name.regexpCapture("EVP_aes_(\\\\d+)_ecb", 1).toInt() / 8 or
        name.matches("EVP_aes_%_cbc") and keySize = name.regexpCapture("EVP_aes_(\\\\d+)_cbc", 1).toInt() / 8 or

        )
    )
    }
}

```

3. Engine Initialization Errors

```sql
class CreateEngine extends FunctionCall {
    CreateEngine() {
    this.getTarget().getName() = "ENGINE_by_id" or
    this.getTarget().getName() = "ENGINE_get_first" or
    this.getTarget().getName() = "ENGINE_get_next" or
    this.getTarget().getName() = "ENGINE_get_default_RSA"
    // ...可能还有其他方式
    }
}

```

```sql
class ENGINE_init extends FunctionCall {
    ENGINE_init() {
    this.getTarget().getName() = "ENGINE_init"
    }
}

```

</details>
