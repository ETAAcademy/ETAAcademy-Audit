# ETAAcademy-Audit: 9. Echidna

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>09 Echidna</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>Echidna</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# Introduction to Echidna for Smart Contract Fuzzing

Fuzzing is a powerful technique for finding vulnerabilities or bugs in software by generating random inputs. Unlike traditional fuzzers that merely detect program crashes, **Echidna** performs **property-based fuzzing**. It systematically tests whether smart contracts violate specified properties or invariants using intelligently generated inputs.

In the context of smart contracts, **properties** are Solidity functions that define undesirable states. For example, Echidna can check if unauthorized access is possible, if funds can be manipulated in unintended ways, or if token balances can overflow.

## 1. Basic Methods

### Writing Property-Based Tests in Echidna

Echidna provides multiple test modes that can be chosen using the `testingMode` option in the configuration file (`config.yaml`) or with the `--testing-mode` CLI parameter. These modes include:

- **Boolean Properties**: This is the default testing mode where functions are prefixed with `echidna_`. These functions should not take any parameters and must return `true` if the test passes, or `false` (or revert) if it fails.

- **Assertions**: Functions in this mode can have any name and accept parameters. Tests are considered failed if the function uses `assert()` or emits an `AssertionFailed` event.

- **Dapptest Mode**: Inspired by Foundry's testing approach, this mode allows functions with parameters to fail using `revert()`. It also supports `FOUNDRY::ASSUME` to skip invalid states, making it ideal for stateless testing scenarios.

- **Additional Modes**

  - **Overflow Mode**: Detects integer overflow or underflow issues.
  - **Optimization Mode**: Attempts to maximize function outputs to find vulnerabilities like fund extraction.
  - **Exploration Mode**: Focuses on code coverage to identify untested parts.

- **State Testing Modes**: All testing modes in Echidna can run in two state modes.

  - **Stateful Mode**: The default mode, which retains state changes after each function call, allowing testing of sequential operations.
  - **Stateless Mode**: Enabled by setting `--seqLen 1`, resets the state to its initial condition after each test, focusing only on individual operations.

- **Function Filtering**: Use `filterBlacklist` to exclude certain functions from testing. Alternatively, create a whitelist using `filterWhitelist` to test specific functions only.

- **Testing with Ether**: Echidna can simulate transactions involving Ether using the following options.

  - `maxValue`: Sets the maximum Ether sent per transaction.
  - `balanceContract`: Specifies the initial Ether balance of the contract.

Beginners are advised to start with stateless testing: use boolean properties for simple logic, assertions for complex state changes, Dapptest for single-operation verification, and stateful testing for testing state sequences.

<details><summary>Code</summary>

<details><summary>Boolean Properties</summary>

```solidity

function echidna_property() public returns (bool) { // No arguments are required
  // The following statements can trigger a failure if they revert
  publicFunction(...);
  internalFunction(...);
  contract.function(...);

  // The following statement can trigger a failure depending on the returned value
  return ...;
} // side effects are *not* preserved

function echidna_revert_property() public returns (bool) { // No arguments are required
  // The following statements can *never* trigger a failure
  publicFunction(...);
  internalFunction(...);
  contract.function(...);

  // The following statement will *always* trigger a failure regardless of the value returned
  return ...;
} // side effects are *not* preserved

```

</details>

<details><summary>Assertions</summary>

```solidity

function checkInvariant(...) public { // Any number of arguments is supported
  // The following statements can trigger a failure using `assert`
  assert(...);
  publicFunction(...);
  internalFunction(...);

  // The following statement will always trigger a failure even if the execution ends with a revert
  emits AssertionFailed(...);

  // The following statement will *only* trigger a failure using `assert` if using solc 0.8.x or newer
  // To make sure it works in older versions, use the AssertionFailed(...) event
  anotherContract.function(...);

} // side effects are preserved

```

</details>

<details><summary>Dapptest</summary>

```solidity

function checkDappTest(..) public { // One or more arguments are required
  // The following statements can trigger a failure if they revert
  publicFunction(..);
  internalFunction(..);
  anotherContract.function(..);

  // The following statement will never trigger a failure
  require(.., “FOUNDRY::ASSUME”);
}

```

</details>

<details><summary>Overflow Optimization Exploration</summary>

```bash
// Overflow Mode
echidna ./Contract.sol --contract MyContract --test-mode overflow

// Optimization Mode
echidna ./Contract.sol --contract MyContract --test-mode optimization --test-limit 10000

// Exploration Mod
echidna ./Contract.sol --contract MyContract --test-mode exploration --corpus-dir ./corpus

```

</details>

<details><summary>Filtering Functions</summary>

```yaml
filterBlacklist: true
filterFunctions: ["C.reset1()", "C.reset2()"]
```

```yaml
filterBlacklist: false
filterFunctions: ["C.f(uint256)", "C.g(uint256)", "C.h(uint256)", "C.i()"]
```

```bash

echidna multi.sol --config blacklist.yaml
...
echidna_state4: failed!💥
  Call sequence:
    f(12)
    g(8)
    h(42)
    i()

```

</details>

<details><summary>ETH</summary>
    
```solidity

// **maxValue**
contract C {
function pay() public payable {
require(msg.value == 12000);
}

    function echidna_has_some_value() public returns (bool) {
        return (address(this).balance != 12000);
    }

}

````

```bash
$ echidna balanceSender.sol
...
echidna_has_some_value: failed!💥
    Call sequence:
    pay() Value: 0x2ee0

````

```solidity
// **balanceContrac**
contract A {
    C internal c;

    constructor() public payable {
        require(msg.value == 12000);
        c = new C();
    }

    function payToContract(uint256 toPay) public {
        toPay = toPay % (address(this).balance + 1);
        c.pay{ value: toPay }();
    }

    function echidna_C_has_some_value() public returns (bool) {
        return (address(c).balance != 12000);
    }
}

contract C {
    function pay() public payable {
        require(msg.value == 12000);
    }
}

```

```bash
$ echidna balanceContract.sol
...
echidna: Deploying the contract 0x00a329c0648769A73afAc7F9381E08FB43dBEA72 failed (revert, out-of-gas, sending ether to an non-payable constructor, etc.):

```

</details>

</details>

---

### Common Testing Approaches for Smart Contracts

Testing smart contracts involves simulating various interactions across multiple accounts. Echidna supports three main types of testing approaches:

- **Internal Testing**

  - Properties are directly defined within the contract.
  - Provides full access to internal state variables and functions.
  - Suitable for validating complex internal logic.

- **External Testing**: Properties are tested through external calls from different contracts, accessing only public/external variables or functions. Echidna can only call test properties or functions named `echidna` and cannot invoke other contract properties or functions. To enable Echidna to call all functions, there are two solutions:

  - **Contract Wrapper**: Add wrapper functions in the `ExternalTest` contract that call methods from the `System` contract. Transactions will be sent from the `ExternalTest` contract instead of Echidna’s simulated sender (e.g., `0x10000`, ...). This method requires manual implementation.
  - **Using `allContracts` Mode**: Allows Echidna to directly call all contract functions. However, this may lead to unintended behavior, such as invoking public functions meant for initialization only. **The solution** is to use a function blacklist to filter out functions that should not be called.

- **Partial Testing**: When simulating the full system is not feasible (e.g., systems relying on off-chain components), partial testing methods can be used:
  - **Isolated Testing**: Tests components that are sufficiently abstracted from the rest of the system, suitable for stateless properties like math libraries.
  - **Function Override**: Uses Solidity's function override feature to disable or modify certain functions, making testing easier.
  - **Model Testing**: When direct testing of the original system is not possible, a simplified model preserving core logic is created to test key aspects of the system.

<details><summary>Code</summary>

<details><summary>Internal Testing</summary>

```solidity

contract InternalTest is System {
    function echidna_state_greater_than_X() public returns (bool) {
        return stateVar > X;
    }
}

```

</details>

<details><summary>External Testing</summary>

```solidity
contract ExternalTest {
    constructor() public {
       // addr = ...;
    }

    function method(...) public returns (...) {
        return System(addr).method();
    }

    function echidna_state_greater_than_X() public returns (bool) {
        return System(addr).stateVar() > X;
    }
}

```

```yaml
filterBlacklist: true
filterFunctions: ["MockERC20.mint(uint256, address)"]
```

</details>

<details><summary>Partial Testing</summary>

```solidity

// 1. Function override
contract InternalTestOverridingSignatures is System {
    function verifySignature(...) public override returns (bool) {
        return true; // signatures are always valid
    }

    function echidna_state_greater_than_X() public returns (bool) {
        executeSomethingWithSignature(...);
        return stateVar > X;
    }
}

// 2. Model testing
// original code
contract System {
    ...

    function calculateSomething() public returns (uint256) {
        if (booleanState) {
            stateSomething = (uint256State1 * uint256State2) / 2 ** 128;
            return stateSomething / uint128State;
        }

        ...
    }
}

// model
contract SystemModel {
    function calculateSomething(bool boolState, uint256 uint256State1, ...) public returns (uint256) {
        if (boolState) {
            stateSomething = (uint256State1 * uint256State2) / 2 ** 128;
            return stateSomething / uint128State;
        }
        ...
    }
}

```

</details>

</details>

---

## 2. Advanced Features of Echidna

### **Corpus Management**

The **Corpus** in Echidna acts as the memory and guide for the fuzzing tool. It records the history of tests, providing insights into where the fuzzer encounters difficulties, which code paths have been covered, and which remain untested. By analyzing coverage reports (`covered.*.txt`) and manually creating or modifying corpus entries, developers can guide the testing process more effectively. Providing a seed corpus can help Echidna identify challenging code paths faster.

**Example: Using Corpus to Test Specific Functions**

Suppose Echidna struggles to test the `magic` function because it cannot satisfy all `require` conditions. You can create a corpus directory and configure Echidna to use it:

<details><summary>Code</summary>

```solidity

// magic.sol
contract C {
    bool value_found = false;

    function magic(uint256 magic_1, uint256 magic_2, uint256 magic_3, uint256 magic_4) public {
        require(magic_1 == 42);
        require(magic_2 == 129);
        require(magic_3 == magic_4 + 333);
        value_found = true;
        return;
    }

    function echidna_magic_values() public view returns (bool) {
        return !value_found;
    }
}

```

```
    1 | *   | contract C {
    2 |     |     bool value_found = false;
    3 |     |
    4 | *   |     function magic(uint256 magic_1, uint256 magic_2, uint256 magic_3, uint256 magic_4) public {
    5 | *r  |         require(magic_1 == 42);
    6 | *r  |         require(magic_2 == 129);
    7 | *r  |         require(magic_3 == magic_4 + 333);
    8 |     |         value_found = true;
    9 |     |         return;
    10 |     |     }
    11 |     |
    12 |     |     function echidna_magic_values() public returns (bool) {
    13 |     |         return !value_found;
    14 |     |     }
    15 |     | }

```

```json
[
  {
    "_gas'": "0xffffffff",
    "_delay": ["0x13647", "0xccf6"],
    "_src": "00a329c0648769a73afac7f9381e08fb43dbea70",
    "_dst": "00a329c0648769a73afac7f9381e08fb43dbea72",
    "_value": "0x0",
    "_call": {
      "tag": "SolCall",
      "contents": [
        "magic",
        [
          {
            "contents": [
              256,
              "93723985220345906694500679277863898678726808528711107336895287282192244575836"
            ],
            "tag": "AbiUInt"
          },
          {
            "contents": [256, "334"],
            "tag": "AbiUInt"
          },
          {
            "contents": [
              256,
              "68093943901352437066264791224433559271778087297543421781073458233697135179558"
            ],
            "tag": "AbiUInt"
          },
          {
            "tag": "AbiUInt",
            "contents": [256, "332"]
          }
        ]
      ]
    },
    "_gasprice'": "0xa904461f1"
  }
]
```

</details>

---

### **Identifying High Gas Consumption**

Echidna can also identify functions with high gas consumption using the `estimateGas` configuration. By adding the following option to your configuration file:

```yaml
estimateGas: true
```

After completing a fuzzing session, Echidna will report the maximum gas consumption for each function. This feature is particularly useful in optimizing functions with unexpectedly high gas usage. It helps developers set appropriate gas limits and ensure efficient smart contract operations.

<details><summary>Code</summary>

```bash

echidna gas.sol --config config.yaml
...
echidna_test: passed! 🎉

f used a maximum of 1333608 gas
  Call sequence:
    f(42,123,249) Gas price: 0x10d5733f0a Time delay: 0x495e5 Block delay: 0x88b2

Unique instructions: 157
Unique codehashes: 1
Seed: -325611019680165325

```

</details>

---

### **Large-Scale Fuzz Testing**

Large-scale fuzz testing involves running multiple **Echidna** instances in parallel on dedicated high-performance servers for extended periods. This approach enables deeper exploration of code paths, uncovering hidden vulnerabilities and achieving higher code coverage compared to short-term testing.

The process begins with setting up a **dedicated environment** (32GB+ RAM, multi-core CPU), followed by short-term tests to verify configurations. Then, **echidna-parade** is used to launch continuous testing, coordinating multiple instances indefinitely. Developers can monitor coverage, detect property failures, refine test properties, and optimize code for better results. The key advantage is the ability to autonomously explore complex code paths, detect rare bugs, and ensure thorough security testing over days or weeks.

**Key Components:**

- **Hardware Requirements**:
  - **High-performance server** with **32GB+ RAM** and **16-64 CPU cores** for handling extensive test data
  - Create a dedicated **non-root** user for fuzz testing
- **Software Stack:**
  - **Echidna** – Core fuzzing engine
  - **echidna-parade** – Manages multiple Echidna instances
  - **Slither** – Static analysis for detecting vulnerabilities
  - **Monitoring scripts** – Track progress and failures
  - **Symbolic execution** – Tools like **Manticore** for unreachable code paths
  - **Formal verification** – Tools like **Certora** for validating critical properties

**Example Configuration:**

- **Empty initial corpus**
- **Configuration file**: `exploration.yaml`
- **Initial run**: 3600 seconds (1 hour)
- **Each generation**: 1800 seconds (30 minutes)
- **Runs indefinitely** (`timeout = -1`)
- **8 Echidna instances** per generation (adjust based on server capacity)
- **Target contract**: `C`
- **Contract file**: `test.sol`
- Logs **stdout & stderr** to `parade.log` and `parade.err`
- Runs as a background process for uninterrupted execution
- To stop the testing when coverage goals are met, use `killall echidna-parade echidna`

<details><summary>Code</summary>

```bash
# adduser echidna
# usermod -aG sudo echidna

```

```bash
echidna-parade test.sol --config exploration.yaml --initial_time 3600 --gen_time 1800 --timeout -1 --ncores 8 --contract C > parade.log 2> parade.err &

```

</details>

---

### **Bytecode Contract Testing**

Echidna supports robust fuzz testing of smart contracts that exist only as bytecode, without the corresponding source code. This capability is particularly useful for security researchers analyzing closed-source contracts, verifying third-party library behavior, conducting cross-language compatibility tests (e.g., comparing Solidity and Vyper implementations), and performing reverse engineering on deployed contracts.

The key to this approach is using a **proxy contract**. The proxy contract employs inline assembly within its constructor to deploy the target contract's bytecode. It then uses a known ABI interface to interact with the target contract while defining testable security properties. By systematically exploring different states and inputs, Echidna can uncover vulnerabilities, such as integer overflows or access control flaws.

Real-world applications of this technique include validating contract behavior under various input conditions, ensuring different implementations adhere to the same standards, and confirming that new contract versions maintain functional consistency with previous deployments. This proactive testing approach enhances smart contract security and ensures the resilience of blockchain applications.

<details><summary>Code</summary>

```bash
echidna bytecode_only.sol --contract TestBytecodeOnly
echidna_test_balance: failed!💥
    Call sequence:
    transfer(0x0,1002)
```

Differential Fuzzing test that they always return the same values：

```python
@view
@external
def my_func(a: uint256, b: uint256, c: uint256) -> uint256:
    return a * b / c

```

```solidity
contract SolidityVersion {
    function my_func(uint256 a, uint256 b, uint256 c) public view {
        return (a * b) / c;
    }
}

```

```solidity
interface Target {
    function my_func(uint256, uint256, uint256) external returns (uint256);
}

contract SolidityVersion {
    Target target;

    constructor() public {
        address targetAddress;

        // vyper bytecode
        bytes
            memory targetCreationBytecode = hex"61007756341561000a57600080fd5b60043610156100185761006d565b600035601c52630ff198a3600051141561006c57600435602435808202821582848304141761004657600080fd5b80905090509050604435808061005b57600080fd5b82049050905060005260206000f350005b5b60006000fd5b61000461007703610004600039610004610077036000f3";

        uint256 size = targetCreationBytecode.length;

        assembly {
            targetAddress := create(0, add(targetCreationBytecode, 0x20), size)
        }
        target = Target(targetAddress);
    }

    function test(uint256 a, uint256 b, uint256 c) public returns (bool) {
        assert(my_func(a, b, c) == target.my_func(a, b, c));
    }

    function my_func(uint256 a, uint256 b, uint256 c) internal view returns (uint256) {
        return (a * b) / c;
    }
}

```

```
echidna  vyper.sol --config config.yaml --contract SolidityVersion --test-mode assertion
assertion in test: passed! 🎉

```

</details>

---

### **Cheat Codes**

Cheat codes are powerful utilities in smart contract testing that allow developers to manipulate the Ethereum Virtual Machine (EVM) and bypass Solidity’s standard restrictions. Echidna fully supports the use of cheat codes from [HEVM](https://hevm.dev/std-test-tutorial.html#supported-cheat-codes), providing greater control over test scenarios.

Key cheat codes include:

- **`prank`**: Alters the `msg.sender` for the next external call, enabling tests for role-based access control.
- **`warp`**: Adjusts the block timestamp to simulate time-dependent behaviors.
- **`roll`**: Changes the block number, useful for testing mining and block-related logic.
- **`sign`**: Generates cryptographic signatures to validate signature verification processes.

These cheat codes significantly simplify complex test scenarios, especially in cases involving:

- Permissioned operations
- Time-based mechanisms (e.g., vesting schedules or time locks)
- Signature-based authentication
- Complex state transitions

However, cheat codes must be used responsibly. Excessive reliance can lead to unrealistic test conditions, introducing false positives by simulating impossible blockchain states. Additionally, they may reduce the readability and maintainability of the test suite. Developers are advised to limit cheat code usage to targeted scenarios, document their applications clearly, and validate behavior using alternative methods in production-like environments.

<details><summary>Code</summary>

```solidity

interface IHevm {
    function prank(address) external;
}

contract TestPrank {
  address constant HEVM_ADDRESS = 0x7109709ECfa91a80626fF3989D68f67F5b1DD12D;
  IHevm hevm = IHevm(HEVM_ADDRESS);
  Contract c = ...

  function prankContract() public payable {
    hevm.prank(address(0x42424242);
    c.f(); // `c` will be called with `msg.sender = 0x42424242`
  }
}

```

</details>

---

### **End-to-End Testing with Echidna**

For complex smart contracts, particularly those with intricate initializations, Echidna offers an efficient end-to-end testing strategy. This method leverages **Etheno**, a powerful tool that captures deployment and transaction sequences from a local blockchain environment such as Ganache. Echidna can then replay these captured transactions for comprehensive fuzz testing.

**How It Works:**

1. **Project Setup**: Ensure the Solidity project has complete deployment and test scripts.
2. **Start Etheno**: Launch an Etheno server configured with a Ganache instance. It’s recommended to set a high gas limit for complex contract deployments.
3. **Capture Transactions**: Run the project’s test or deployment script, allowing Etheno to record all transactions in a JSON file.
4. **Configure Echidna**: Develop Echidna test properties targeting the deployed contract and specify the captured blockchain state using the JSON file.
5. **Run Tests**: Echidna will replay the transactions while introducing random input variations to identify vulnerabilities.

**Advantages of End-to-End Testing:**

- **Realistic Testing Environment**: Validates contracts in a state closely resembling production.
- **Reduced Setup Effort**: Avoids manually recreating complex initial states.
- **Improved Interaction Testing**: Evaluates interactions across multiple contracts and external dependencies.
- **Enhanced Vulnerability Detection**: Identifies integration issues that are difficult to uncover in isolated unit tests.

**Considerations and Limitations:**

- **Account Mismatch**: Transactions using Ganache test accounts may fail during Echidna tests. Ensure addresses are correctly mapped.
- **Time-Dependent Behavior**: Tests relying on `block.timestamp` may produce inconsistent results due to time simulation differences between environments.

End-to-end testing is especially beneficial for DeFi protocols, governance systems, and complex multi-contract applications. By ensuring deployed contracts behave as expected under real-world conditions, developers can confidently reduce the risk of vulnerabilities and improve overall application reliability.

<details><summary>Code</summary>

```jsx
const SimpleStorage = artifacts.require("SimpleStorage");

contract("SimpleStorage", (accounts) => {
  it("...should store the value 89.", async () => {
    const simpleStorageInstance = await SimpleStorage.deployed();

    // Set value of 89
    await simpleStorageInstance.set(89, { from: accounts[0] });

    // Get stored value
    const storedData = await simpleStorageInstance.storedData.call();

    assert.equal(storedData, 89, "The value 89 was not stored.");
  });
});
```

```bash
etheno --ganache --ganache-args="--miner.blockGasLimit 10000000" -x init.json

```

```solidity
docker run -it -p 8545:8545 -v ~/etheno:/home/etheno/ trailofbits/etheno
(you will now be working within the Docker instance)
etheno --ganache --ganache-args="--miner.blockGasLimit 10000000" -x init.json
```

```bash
truffle test test/test.js

```

```bash
buidler test test/test.js --network localhost

```

```bash
truffle test test/simplestorage.js --network develop
import "../SimpleStorage.sol";

contract E2E {
    SimpleStorage st = SimpleStorage(0x871DD7C2B4b25E1Aa18728e9D5f2Af4C4e431f5c);

    function crytic_const_storage() public returns (bool) {
        return st.storedData() == 89;
    }
}

```

```yaml
prefix: crytic_
initialize: init.json
allContracts: true
cryticArgs: ["--truffle-build-directory", "app/src/contracts/"]
```

```
echidna . --contract E2E --config echidna.yaml
...
crytic_const_storage: failed!💥
  Call sequence:
    (0x871dd7c2b4b25e1aa18728e9d5f2af4c4e431f5c).set(0) from: 0x0000000000000000000000000000000000010000

```

</details>

---

### Testing External Libraries

Solidity supports two types of libraries: internal and external. Internal libraries contain only internal functions and are directly compiled into the contracts that use them. External libraries, on the other hand, include external or public functions, requiring separate deployment and linking. For contracts relying on external libraries, Echidna provides a two-step process to ensure proper testing:

- **Deploy Libraries:** Use the `deployContracts` option in the configuration file to deploy the library to a specified address. Example: `deployContracts: [["0x1f", "ConvertLib"]]`.
- **Link Libraries:** Configure the `cryticArgs` option to instruct the crytic-compile tool on how to link the contract bytecode with the deployed library address. Example: `cryticArgs: ["--compile-libraries=(ConvertLib,0x1f)"]`.

This method allows developers to comprehensively test complex contracts dependent on external libraries, ensuring that all code paths, including library interactions, are validated effectively.

<details><summary>Code</summary>

```yaml
deployContracts: [["0x1f", "ConvertLib"]]

cryticArgs: ["--compile-libraries=(ConvertLib,0x1f)"]
```

```bash
$ echidna . --test-mode exploration --corpus-dir corpus --contract MetaCoin --config echidna.yaml

```

</details>

---

### On-Chain Fuzzing with State Forking

Echidna introduces a powerful state forking capability for smart contract testing. This technique enables developers to run fuzz tests starting from the real blockchain state fetched from external RPC services like Infura, Alchemy, or a local node. By using the `ECHIDNA_RPC_URL` and `ECHIDNA_RPC_BLOCK` environment variables, Echidna can query contract code and storage from a specified block, facilitating tests in near-production environments.

**Advantages of State Forking**

- **Simplified Environment Setup:** No need to simulate complex test states manually.
- **Real-World Testing:** Identify vulnerabilities that may only manifest in specific blockchain states.
- **Seamless Protocol Integration:** Test new contracts interacting with established DeFi protocols like Compound, Uniswap, or Aave.
- **Faster Development:** Validate updates to existing systems without complete redeployment.

Echidna also supports state caching to store blockchain data locally, significantly improving test execution speeds, especially in continuous integration (CI) environments. Additionally, its Etherscan integration can automatically fetch verified source code and mappings for deployed contracts, generating detailed coverage reports.

**Real-World Example**
A notable case of leveraging Echidna’s state forking was the detection of the $2.3 million Stax Finance vulnerability. Echidna's advanced state exploration identified exploitable transaction sequences by maximizing attacker gains, exposing bugs that conventional testing methods often miss.

<details><summary>Code</summary>

```bash
$ ECHIDNA_RPC_URL=http://.. ECHIDNA_RPC_BLOCK=16771449 echidna compound.sol --test-mode assertion --contract TestCompoundEthMint
...
assertNoBalance(): failed!💥
    Call sequence, shrinking (885/5000):
    assertNoBalance() Value: 0xd0411a5

```

```solidity
interface IHevm {
    function warp(uint256 newTimestamp) external;

    function roll(uint256 newNumber) external;
}

interface Compound {
    function mint() external payable;

    function balanceOf(address) external view returns (uint256);
}

contract TestCompoundEthMint {
    address constant HEVM_ADDRESS = 0x7109709ECfa91a80626fF3989D68f67F5b1DD12D;
    IHevm hevm = IHevm(HEVM_ADDRESS);
    Compound comp = Compound(0x4Ddc2D193948926D02f9B1fE9e1daa0718270ED5);

    constructor() {
        hevm.roll(16771449);
        hevm.warp(1678131671);
    }

    function assertNoBalance() public payable {
        require(comp.balanceOf(address(this)) == 0);
        comp.mint{ value: msg.value }();
        assert(comp.balanceOf(address(this)) == 0);
    }
}

```

</details>

---

### Chain Interaction Using FFI

Echidna provides a Foreign Function Interface (FFI) cheat code that facilitates off-chain data interactions during smart contract testing. With FFI, developers can execute arbitrary commands on the host machine, read external data, and incorporate the results into their fuzz tests.

**Enabling FFI**
To use FFI, set the `allowFFI: true` option in the Echidna configuration file. This ensures developers are fully aware of the security implications since FFI can manipulate the EVM environment and potentially harm the host system.

**Use Cases for FFI**

- **Real-Time Data Access:** Retrieve market prices from off-chain oracles.
- **Enhanced Randomness:** Incorporate complex randomness from external algorithms.
- **Algorithm Integration:** Leverage external computation resources for complex cryptographic or mathematical operations.
- **Cross-Implementation Validation:** Compare results between Solidity contracts and external programs.

Developers can invoke a Python script using FFI to generate cryptographic signatures or simulate external system behavior. The external script can return ABI-encoded data for decoding within the smart contract. This approach is particularly useful for testing protocols relying on real-world data and algorithms not easily implementable in Solidity.

**Best Practices for FFI Usage**

- **Limit Scope:** Restrict FFI to essential use cases.
- **Validate Data:** Ensure proper validation of external inputs.
- **Error Handling:** Implement robust error-handling mechanisms.
- **Documentation:** Clearly document the purpose and behavior of FFI calls.
- **Isolation:** Run tests in sandboxed environments to minimize security risks.
- **Performance Monitoring:** Monitor external calls to prevent negative impacts on test performance.

By employing FFI responsibly, developers can expand the scope of their tests, simulate real-world scenarios, and ensure comprehensive contract validation, contributing to overall blockchain security and reliability.

<details><summary>Code</summary>
    
```yaml
testMode: "assertion"
allowFFI: true
```

```python
import sys
import secrets
from web3 import Web3
from eth_abi import encode

# Usage: python3 script.py number
number = int(sys.argv[1])

# Generate a 10-byte random number
random = int(secrets.token_hex(10), 16)

# Generate the keccak hash of the input value
hashed = Web3.solidity_keccak(['uint256'], [number])

# ABI-encode the output
abi_encoded = encode(['uint256', 'bytes32'], [random, hashed]).hex()

# Make sure that it doesn't print a newline character
print("0x" + abi_encoded, end="")

```

```solidity
pragma solidity ^0.8.0;

// HEVM helper
import "@crytic/properties/contracts/util/Hevm.sol";

// Helpers to convert uint256 to string
import "@crytic/properties/contracts/util/PropertiesHelper.sol";

contract TestFFI {
    function test_ffi(uint256 number) public {
        // Prepare the array of executable and parameters
        string[] memory inp = new string[](3);
        inp[0] = "python3";
        inp[1] = "script.py";
        inp[2] = PropertiesLibString.toString(number);

        // Call the program outside the EVM environment
        bytes memory res = hevm.ffi(inp);

        // Decode the return values
        (uint256 random, bytes32 hashed) = abi.decode(res, (uint256, bytes32));

        // Make sure the return value is the expected
        bytes32 hashed_solidity = keccak256(abi.encodePacked(number));
        assert(hashed_solidity == hashed);
    }
}

```

</details>

---

## 3. Other Methods

### Hybrid Echidna: Combining Fuzzing and Symbolic Execution

Hybrid Echidna enhances the detection of complex vulnerabilities by integrating random fuzzing (via Echidna) with symbolic execution (using Maat). Traditional Echidna fuzzing excels at identifying general contract issues, but it struggles with conditions that require satisfying specific arithmetic constraints, such as `x / 4 == -20` or `(x >> 30) / 7 == 2`. The probability of randomly generating a 256-bit integer that meets such constraints is extremely low (around $1/2^{256}$).

Hybrid Echidna solves this issue using an iterative process:

- **Initial Fuzzing**: Echidna collects a corpus of inputs from its initial run.
- **Symbolic Execution**: The inputs are analyzed through symbolic execution to identify uncovered code branches and their respective constraints.
- **Constraint Solving**: Maat solves these constraints, generating concrete inputs that trigger the targeted code paths.
- **Corpus Expansion**: The generated inputs are added to the fuzzing corpus.
- **Iteration**: Echidna continues fuzzing with the enriched corpus until no further code paths are identified.

This approach is particularly effective for discovering vulnerabilities that random fuzzing alone would miss. Real-world cases have demonstrated Hybrid Echidna’s success in identifying assertion failures and logical errors.

<details><summary>Code</summary>
    
```solidity
pragma solidity ^0.7.1;
    
contract VulnerableContract {
    
    function func_one(int128 x) public pure {
        if (x / 4 == -20) {
            assert(false); // BUG
        }
    }
    
    function func_two(int128 x) public pure {
        if ((x >> 30) / 7 == 2) {
            assert(false); // BUG
        }
    }
}
```

</details>

---

### Reproducer Trace and Fuzz-Utils

Echidna’s **reproducer trace** feature provides in-depth insights into contract execution, capturing comprehensive state changes at every step. During testing, it logs detailed information, including function call parameters, return values, emitted events, and storage updates. This allows developers to analyze long and complex transaction sequences (up to 70 transactions over 29 million blocks) and pinpoint the root cause of errors.

Additionally, the **fuzz-utils** tool automates the conversion of assertion failures into unit tests. Instead of spending hours manually reproducing and analyzing failed test cases, fuzz-utils streamlines the process, reducing it to around 30 minutes. It extracts the essential sequence of transactions that triggered the failure and generates unit test code with setup, execution, and assertions, significantly accelerating bug resolution.

<details><summary>Code</summary>
    
```solidity
Traces:
0x46662E22D131Ea49249E0920C286E1484FEEf76E::queryUserLocksLength(@0x00a329c0648769A7
3afAc7F9381E08FB43dBEA72) (curvance-contracts/tests/fuzzing/FuzzVECVE.sol:788)
└╴← (1)
call
0x46662E22D131Ea49249E0920C286E1484FEEf76E::0x9521e5bb000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000010000000000000000000000000000000000000000000000000000000000000001000000
00000000000000000062d69f6867a0a084c6d313943dc22023bc26369100000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000000000000000000000000000000000000000000000001200000000000000000000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0000000000000000000000 (curvance-contracts/tests/fuzzing/FuzzVECVE.sol:810)
├╴call
0x0A64DF94bc0E039474DB42bb52FEca0c1d540402::epochsToClaim(@0x00a329c0648769A73afAc7F
9381E08FB43dBEA72) <no source map>
│ └╴← (0)
├╴Transfer(1000000000000000000) <no source map>
└╴← 0x
call
0x46662E22D131Ea49249E0920C286E1484FEEf76E::queryUserLocksLength(@0x00a329c0648769A7
3afAc7F9381E08FB43dBEA72) (curvance-contracts/tests/fuzzing/FuzzVECVE.sol:1260)
└╴← (2)
AssertEqFail(«Invalid: 1!=2, reason: VE_CVE - when relocking, the number of locks
should be equivalent»)
(curvance-contracts/tests/fuzzing/helpers/PropertiesHelper.sol:45)
```

</details>

---

### Differential Fuzzing and Library Testing

Differential fuzzing is a powerful technique for testing smart contract libraries. Libraries are often used by multiple contracts, so any vulnerability can propagate across dependent systems. Effective library testing involves several strategies:

- **Multiple Implementation Comparison**: Test the same function using different implementations to identify discrepancies.
- **Revert Testing**: Ensure functions revert under expected invalid conditions.
- **Invariant Checks**: Validate that core properties (e.g., correctness of set operations) hold under all circumstances.
- **Cross-Validation**: Perform input-output verification using different methods.

For example, consider a `hasDuplicate` function that checks for duplicate elements in an array. When dealing with empty arrays, it may trigger an **underflow** in `uint256`, leading to infinite loops and potential **DoS (Denial of Service)** attacks. Using property-based fuzzing in Echidna, developers can write boolean properties to ensure the function behaves as expected across all edge cases. Functions like `indexOf` and `indexOfFromEnd` can cross-validate the existence of duplicates, further strengthening the testing process.

<details><summary>Code</summary>
    
    ```solidity
    // Set Protocol
    /**
    * Returns whether or not there's a duplicate. Runs in O(n^2).
    * @param A Array to search
    * @return Returns true if duplicate, false otherwise
    */
    function hasDuplicate(address[] memory A) returns (bool)
       {
         for (uint256 i = 0; i < A.length - 1; i++) {
           for (uint256 j = i + 1; j < A.length; j++) {
             if (A[i] == A[j]) {
                return true;
             }
           }
       }
       return false;
    }
    ```
    
    ```solidity
        
      // indexOf and indexOfFromEnd
        for (uint i = 0; i < addrs1.length; i++) {
          (i1, b) = AddressArrayUtils.indexOf(addrs1, addrs1[i]);
          (i2, b) = AddressArrayUtils.indexOfFromEnd(addrs1, addrs1[i]);
          if (i1 != (i2-1)) { // -1 because fromEnd return is off by one
    	hasDup = true;
          }
        }
        return hasDup == AddressArrayUtils.hasDuplicate(addrs1);
      }
    ```

</details>

---

### Diffusc for Upgradable Smart Contracts

**Diffusc** is designed for differential fuzzing of upgradable smart contracts (USCs). By combining static analysis with fuzzing, it detects unintended behavioral changes introduced by contract upgrades. The tool operates using the following process:

- **Static Analysis**: Slither analyzes the code to identify functions affected by the upgrade.
- **Differential Fuzzing**: Diffusc generates specialized fuzzing contracts and uses Echidna to compare the behavior of the pre-upgrade and post-upgrade versions.
- **Execution Modes**: Diffusc supports both **local testnet mode** (for preliminary testing) and **fork mode** (for using live blockchain data).
- **Taint Analysis**: It tracks the impact of code changes, identifying affected functions and variables.

Diffusc has been successfully applied in cases like the Compound protocol, where it detected a token distribution bug that could have caused a $40 million loss. Despite its automation capabilities, Diffusc still benefits from developer input in setting initial conditions, customizing initialization logic, and verifying test results.

<details><summary>Code</summary>

```solidity
// **Compound's Upgrade-Related Functions**
function _become(SimpleUnitroller unitroller) public {
    require(msg.sender == unitroller.admin(), "only unitroller admin can change brains");
    require(unitroller._acceptImplementation() == 0, "change not authorized");
    SimpleComptrollerV2(address(unitroller))._upgradeSplitCompRewards();
}

function _upgradeSplitCompRewards() public {
    require(msg.sender == comptrollerImplementation, "only brains can become itself");
    uint32 blockNumber = safe32(getBlockNumber(), "block number exceeds 32 bits");
    for (uint i = 0; i < allMarkets.length; i ++) {
        CompMarketState storage supplyState = compSupplyState[address(allMarkets[i])];
        if (supplyState.index == 0) {
            supplyState.index = compInitialIndex;
            supplyState.block = blockNumber;
        }
    }
}

```

```solidity

// **Reward Distribution**
function distributeSupplierComp(address cToken, address supplier) internal {
    CompMarketState storage supplyState = compSupplyState[cToken];
    uint supplyIndex = supplyState.index;
    uint supplierIndex = compSupplierIndex[cToken][supplier];
    compSupplierIndex[cToken][supplier] = supplyIndex;

    if (supplierIndex == 0 && supplyIndex > compInitialIndex) {
        supplierIndex = compInitialIndex;  // BUG: line not reached due to new initialization
    }

    Double memory deltaIndex = Double({mantissa: sub_(supplyIndex, supplierIndex)});
    uint supplierTokens = CToken(cToken).balanceOf(supplier);
    uint supplierDelta = mul_(supplierTokens, deltaIndex);
    uint supplierAccrued = add_(compAccrued[supplier], supplierDelta);
    compAccrued[supplier] = supplierAccrued;
}

```

```solidity
// **Diffusc Test Wrapper**
function TargetContract_balanceOf(address a) public virtual {
    hevm.prank(msg.sender);
    (bool successV1, bytes memory outputV1) = address(proxyV1).call(
        abi.encodeWithSelector(
            targetContractV1.balanceOf.selector, a
        )
    );
    hevm.prank(msg.sender);
    (bool successV2, bytes memory outputV2) = address(proxyV2).call(
        abi.encodeWithSelector(
            targetContractV2.balanceOf.selector, a
        )
    );
    assert(successV1 == successV2);
    assert((!successV1 && !successV2) || keccak256(outputV1) == keccak256(outputV2));
}

```

```solidity
// **Constructor in Standard Mode**
constructor() public {
    targetContractV1 = ITargetContractV1(address(new TargetContractV1()));
    targetContractV2 = ITargetContractV2(address(new TargetContractV2()));
    proxyV1 = IProxy(address(new Proxy()));
    proxyV2 = IProxy(address(new Proxy()));
    hevm.store(
        address(proxyV1),
        bytes32(uint(0)),
        bytes32(uint256(uint160(address(targetContractV1))))
    );
    hevm.store(
        address(proxyV2),
        bytes32(uint(0)),
        bytes32(uint256(uint160(address(targetContractV1))))
    );
}

```

```solidity
// **Constructor in Fork Mode**
constructor() public {
    hevm.roll(13322796);
    fork1 = hevm.createFork();
    fork2 = hevm.createFork();
    targetContractV1 = ITargetContractV1(0x75442Ac771a7243433e033F3F8EaB2631e22938f);
    targetContractV2 = ITargetContractV2(0x374ABb8cE19A73f2c4EFAd642bda76c797f19233);
    proxy = IProxy(0x3d9819210A31b4961b30EF54bE2aeD79B9c9Cd3B);
    hevm.selectFork(fork1);
    hevm.store(
        address(proxy),
        bytes32(uint(0)),
        bytes32(uint256(uint160(address(targetContractV1))))
    );
    hevm.selectFork(fork2);
    hevm.store(
        address(proxy),
        bytes32(uint(0)),
        bytes32(uint256(uint160(address(targetContractV2))))
    );
}

```

```solidity
// **Custom Upgrade Function**
function upgradeV2() external override {
    unitrollerV2._setPendingImplementation(address(comptrollerV2));
    comptrollerV2._become(address(unitrollerV2));
}

```

</details>

---

### Invariant-Driven Development (IDD)

**Invariant-Driven Development (IDD)** is a paradigm that embeds invariants into the development lifecycle to ensure logical consistency and correctness. Invariants are properties that must always remain true across all states and execution paths. Examples include:

- **Token Contracts**: User balances should never exceed the total token supply.
- **AMMs (Automated Market Makers)**: The product of token reserves (`x * y = k`) must hold true without fees.

Invariants fall into two categories:

- **Function-Level Invariants**: Validate the correctness of specific calculations (e.g., interest calculations must be monotonically increasing).
- **System-Level Invariants**: Ensure global properties of the entire system remain valid (e.g., total asset value exceeding liabilities).

Invariants can be used at various stages:

- **Design**: Teams should define and document critical invariants.
- **Development and Testing**: Tools like Echidna, Medusa, and formal verification tools like Certora and KEVM help ensure the code adheres to invariants.
- **Deployment and Monitoring**: On-chain checks and monitoring systems (e.g., Hexagate, Tenderly) continuously validate invariants.
- **Auditing and Evaluation**: Invariants assist security engineers in reviewing and understanding critical contract logic.

Embedding invariants into the contract code using assert statements or external monitoring reduces the risk of catastrophic failures. Notably, protocols like Uniswap leverage on-chain invariant checks for their constant product formula.

<details><summary>Code</summary>

```solidity

// User balance must not exceed the total supply
function test_ERC20_userBalanceNotHigherThanSupply() public {
    assertLte(
        balanceOf(msg.sender),
        totalSupply(),
        "User balance higher than total supply"
    );
}

```

</details>

---

### Conclusion

Echidna has established itself as a comprehensive property-based fuzzing platform for smart contract security. At its core, Echidna offers essential testing capabilities through multiple modes (**Boolean properties**, **Assertions**, **Dapptest**), flexible state management (**Stateful**, **Stateless**), and diverse testing strategies (**Internal**, **External**, **Partial**). These fundamentals are enhanced by advanced features including **corpus management** for intelligent test case generation, **gas analysis** for optimization, and support for **bytecode testing**, **cheat codes**, **end-to-end testing** with Etheno, **external library testing**, and **on-chain fuzzing** with state forking. The platform's evolution has led to sophisticated extensions such as **Hybrid Echidna**, **differential fuzzing with Diffusc**, **reproducer trace**, **fuzz-utils**, and **Invariant-Driven Development (IDD)**.

---

<div  align="center">
<img src="https://github.com/ETAAcademy/ETAAcademy-Images/blob/main/ETAAcademy-Audit/09_Echidna.gif?raw=true" width="50%" />
</div>
