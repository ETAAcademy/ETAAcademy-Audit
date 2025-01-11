# ETAAcademy-Adudit: 4. Cosmos Security

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>04. Cosmos Security</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>Cosmos Security</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# Cosmos: The Internet of Blockchains

Cosmos is a network of interoperable blockchains, often referred to as the "Internet of Blockchains." It comprises multiple blockchains that can communicate and transfer value seamlessly. To achieve this vision, the **Interchain ecosystem** relies on an **open-source toolkit** that includes the Inter-Blockchain Communication Protocol (IBC), its implementation within the Cosmos SDK, and the Tendermint base layer that ensures distributed state finality. Application-specific interoperable blockchains are built using the Cosmos SDK, which provides the prerequisites for enabling IBC and Tendermint consensus.

Interchain is a decentralized network of independent blockchains supported by Byzantine Fault Tolerant (BFT) consensus algorithms. It enables interoperability through the IBC protocol, facilitating token transfers and cross-chain communication. This ecosystem includes protocols, SDKs, tokens, wallets, applications, and more. The key components are as follows:

- **Cosmos SDK**: The Cosmos SDK is a modular and flexible framework for developers to build interoperable, application-specific blockchains. It prioritizes security through the **object-capability model**, which reduces the risk of attacks.

- **Tendermint and CometBFT**: **Tendermint** is a consensus and networking engine that abstracts away the complexities of consensus and network management, offering developers a high-performance consensus solution. Its evolution, **CometBFT**, integrates **Delegated Proof of Stake (DPoS)** and **Practical Byzantine Fault Tolerance (pBFT)** mechanisms to provide an efficient and reliable consensus layer. Up to 175 validators, ranked by token holdings, CometBFT secure the network and validate transactions, with block times averaging 7 seconds.

- **Inter-Blockchain Communication Protocol (IBC)**: The IBC protocol enables secure and efficient token and data transfers between blockchains. By leveraging Tendermint’s fast finality, blockchains with diverse architectures and applications can achieve interoperability. Use cases include value transfer, token swaps, and cross-chain data exchange.

- **Cosmos Hub**: Within the Interchain ecosystem, blockchains are categorized as **Hubs** and **Zones**. Zones connect to Hubs, enabling seamless data and value flow. **Cosmos Hub** is the first blockchain built using the Interchain technology stack. It operates as a **public Proof-of-Stake (PoS)** blockchain with ATOM as its native token, used for paying transaction fees and securing the network. As the “router” of the Interchain ecosystem, Cosmos Hub facilitates cross-chain transactions between Zones, acting as a central node for interoperability.

- **Ignite CLI**: The Ignite CLI is a command-line tool that simplifies the creation of application-specific blockchains. Built on Tendermint/CometBFT and the Cosmos SDK, it provides an intuitive interface for developers to build, test, and deploy blockchains efficiently.

- **CosmWasm**: CosmWasm is the cross-chain smart contract platform within the Interchain ecosystem. It leverages WebAssembly (Wasm) technology, enabling developers to build decentralized applications (dApps) that interact seamlessly with other blockchains in the ecosystem.

- **Alternative Frameworks and SDKs**: The Cosmos SDK’s modular design allows for easy integration of existing Go-based codebases. An example is **Ethermint**, which combines the Ethereum Virtual Machine (EVM) with the Cosmos SDK. Ethermint enables the creation of EVM-compatible blockchains, allowing developers to use Ethereum tools like Truffle and MetaMask while benefiting from Tendermint’s fast consensus and interoperability features.

---

## 1. Cosmos SDK: A Framework for Building Custom Blockchains

The Cosmos SDK, a Golang-based framework, provides developers with the tools to easily create **application-specific blockchains**. Unlike applications running on general-purpose blockchains, application-specific blockchains are tailored to serve a single application. This setup ensures that governance is entirely controlled by the application itself, free from the constraints of the underlying blockchain. These custom blockchains can interact seamlessly with others through the **Inter-Blockchain Communication (IBC)** protocol.

The Cosmos SDK includes a variety of built-in modules, such as staking, governance, and IBC, enabling rapid development of blockchain applications. Developers can also create custom modules to meet specific requirements. These modules interact with one another using interfaces and **keepers**, facilitating modular and efficient blockchain design.

Blockchains built using the Cosmos SDK generally consist of three layers:

- **Networking Layer**: Managed by Tendermint Core, it handles transaction propagation and ensures communication between nodes.

- **Consensus Layer**: Also powered by Tendermint Core, it guarantees agreement on the state of the blockchain across all nodes.

- **Application Layer**: Designed by developers, this layer processes transactions and updates the blockchain state according to the application’s logic.

### **ABCI: The Application-Blockchain Interface**

At the core of the Cosmos SDK is the **Application-Blockchain Interface (ABCI)**, which defines the interaction between the **application layer** and the **consensus layer**. By using the ABCI, developers can offload the complexities of consensus and networking to **CometBFT**, focusing solely on building their applications.

The ABCI employs a **socket protocol** for communication, allowing developers to implement the application layer in their preferred programming language. Key functionalities of the ABCI include:

- **CheckTx**: Validates transactions before they are broadcasted.
- **DeliverTx**: Processes transactions and updates the application state.
- **BeginBlock and EndBlock**: Manage block initialization and finalization, enabling tasks such as scheduled events.

### **State Machines: The Backbone of Blockchain**

A blockchain operates as a **state machine**, a computational model where the system transitions from one state to another based on predefined rules. Each blockchain starts with a **genesis state** and processes transactions to update to the current state. This structure ensures consistency and determinism in transaction execution.

In the Cosmos SDK, developers implement state transitions through concise **state transition functions**. These functions define how the blockchain’s state evolves with each transaction.

### **BaseApp: Simplifying Blockchain Development**

The Cosmos SDK provides **BaseApp**, a foundational framework that abstracts much of the complexity of blockchain development. Developers can focus on application-specific functionality while BaseApp handles core blockchain operations. BaseApp communicates with Tendermint using the ABCI interface and manages essential tasks such as state persistence and transaction processing.

Key components of BaseApp include:

- **Mempool**: Stores transactions validated by `CheckTx` for future processing.
- **AnteHandler**: Executes pre-transaction checks, including account validation and fee payment.
- **ProcessProposalHandler**: Proposes new blocks by extracting transactions from the mempool.
- **PostHandler**: Manages post-transaction operations.
- **State Caching**: Uses cached states (`checkState` and `deliverState`) during transaction processing to prevent premature state changes.

### **Transaction Lifecycle**

The transaction process within the Cosmos SDK is divided into several key stages, ensuring the validity, execution, and persistence of transactions. These stages are as follows:

- **CheckTx**: When a transaction is received (provided by Tendermint as raw bytes), it undergoes preliminary validation to ensure its validity and prevent invalid or spam transactions from entering the mempool. During this stage, the transaction is decoded into `sdk.Msgs`. Each `sdk.Msg` is then subject to basic validation (e.g., `validateBasic`). Before invoking the `anteHandler`, the `checkState` is cached and branched to ensure that any failed validation in `anteHandler` does not result in unintended state changes. If the validation checks are successful, the transaction is added to both Tendermint’s mempool and, if applicable, the application-side mempool.

- **BeginBlock**: When Tendermint receives a new block proposal, it triggers the `BeginBlock` method on the application layer. At this stage, developers can execute code unrelated to transaction processing, such as initializing block-specific state or resetting gas counters.

- **DeliverTx**: This stage handles the core logic for processing transactions. Similar to `CheckTx`, validation checks are performed to ensure the transaction's integrity. The transaction is then routed through the application layer’s message routing mechanism, dispatching it to the appropriate module for execution. Upon completion, any state changes are written to the `CacheMultiStore` within `DeliverState`.

- **EndBlock**: After all transactions in a block are processed, the `EndBlock` method is invoked. This stage allows developers to perform any post-transaction tasks, such as updating the validator set or executing end-of-block operations.

- **Commit**: Once consensus is reached (i.e., Tendermint receives pre-commit votes from more than two-thirds of the validator nodes), the `Commit` message is sent to the application layer. At this point, the Cosmos SDK writes the branched multi-store from `DeliverState` to `app.cms` and persists it to storage. The `DeliverState` captures all state transitions from the `BeginBlock`, `DeliverTx`, and `EndBlock` stages. The application then returns the hash of the committed state to Tendermint, finalizing the block.

### **Security and Access Control**

To mitigate risks from malicious modules, the Cosmos SDK employs an **object-capability model** for access control. This ensures that modules interact only through explicitly exposed interfaces, preventing unauthorized access.

Each module typically has a **keeper**, which acts as a mediator between the module’s logic and its data storage. The keeper ensures that state changes are secure and provides methods for interacting with the storage layer.

#### Example: Staking Module Keeper

The staking module’s keeper integrates with the bank and authentication modules to manage inter-module interactions safely. Below is an example of the staking module keeper:

<details><summary>Code</summary>

```go
// Keeper of the x/staking store
type Keeper struct {
    storeKey   storetypes.StoreKey
    cdc        codec.BinaryCodec
    authKeeper types.AccountKeeper
    bankKeeper types.BankKeeper
    hooks      types.StakingHooks
    authority  string
}
```

The module exposes an interface defining key functionalities:

```go
type Keeper interface {
    MintCoins(ctx sdk.Context, moduleName string, amt sdk.Coins) error
    BurnCoins(ctx sdk.Context, moduleName string, amt sdk.Coins) error
    GetBalance(ctx sdk.Context, addr sdk.AccAddress, denom string) sdk.Coin
    ...
}
```

A subset of this interface, `BankKeeper`, is exposed to other modules like staking:

```go
type BankKeeper interface {
    GetBalance(ctx sdk.Context, addr sdk.AccAddress, denom string) sdk.Coin
    SendCoinsFromModuleToModule(ctx sdk.Context, senderPool, recipientPool string, amt sdk.Coins) error
    BurnCoins(ctx sdk.Context, name string, amt sdk.Coins) error
}
```

This selective exposure ensures that modules only access necessary methods, reinforcing security and modularity.

```go

// NewKeeper creates a new staking Keeper instance
func NewKeeper(
    cdc codec.BinaryCodec,
    key storetypes.StoreKey,
    ak types.AccountKeeper,
    bk types.BankKeeper,
    authority string,
) *Keeper {
    ...
    return &Keeper{
        storeKey:   key,
        cdc:        cdc,
        authKeeper: ak,
        bankKeeper: bk,
        hooks:      nil,
        authority:  authority,
    }
}

```

</details>

---

## 2. IBC: Interchain Security and Cross-Chain Communication

In addressing blockchain scalability, different ecosystems propose varying approaches. Solana advocates for a monolithic chain capable of handling all transactions, including data availability and execution. Ethereum prefers a modular architecture, separating layers such as execution, data availability, and consensus, exemplified by Layer-2 rollups. Cosmos addresses scalability by combining **horizontal scalability** through parallel app-specific blockchains with **vertical scalability** via optimized consensus and performance, using tools like Tendermint, Cosmos SDK, and IBC.

### IBC: Enabling Secure Cross-Chain Communication

IBC enables interoperability between blockchains, allowing secure and seamless data exchange. This system relies on lightweight clients, connection establishment, secure data relays, and modular communication.

- **Light Clients**: These nodes track and verify other chains' consensus states by storing and authenticating commitment proofs. Each client is uniquely identified by a client ID.
- **Connections and Handshakes**: Connections are established via a four-step handshake process, ensuring both chains verify each other's states using their light clients. This eliminates the need to trust intermediary relayers.
- **Data Relays**: Relayers facilitate cross-chain data transfers by submitting proofs from one chain to another. Malicious or incorrect data is rejected by light clients, maintaining security.
- **Channels and Module Communication**: Using ports and channels, modules securely exchange data, with the IBC/TAO layer (Transport, Authentication, and Ordering) managing packet delivery. Dynamic capabilities further restrict module operations, preventing misuse.

### Shared Security

Shared security refers to the ability of a blockchain or network to extend its security to multiple other blockchains or applications, ensuring these chains maintain high security during their consensus and transaction execution processes. Common shared security mechanisms include:

- **Rollups**: For example, Optimistic Rollups aggregate multiple off-chain transactions into a single on-chain transaction. Specialized sequencers process these transactions, while other participants can submit fraud proofs to verify their correctness.
- **Polkadot’s Relay Chain**: Polkadot provides security through a centralized Relay Chain. Multiple specialized blockchains (called parachains) share this security. Each parachain has its own block producers (collators), but all state transitions require validation from the Relay Chain.

In Cosmos, the **Interchain Security** model aims to enable a provider chain (e.g., Cosmos Hub) to allow its validators to participate in the consensus process of multiple consumer chains by staking the provider chain’s tokens (e.g., ATOM). This creates a cross-chain security guarantee.

### Interchain Security in Cosmos

One challenge in Cosmos is the differing security levels across independent application-specific chains. Cosmos addresses this through **Interchain Security**, leveraging a robust consensus mechanism for its application chains. Each Cosmos chain uses the same software stack and the Tendermint Core, a fault-tolerant Proof-of-Stake (PoS) consensus mechanism that remains operational even if up to one-third of nodes fail.

The economic security of Cosmos chains is provided by stakers. Validators can use their staked tokens on the provider chain (e.g., ATOM on Cosmos Hub) to produce blocks on consumer chains and earn rewards from both. Misbehaving validators risk slashing of their staked tokens on the provider chain. This is facilitated by the **Cross-Chain Validation (CCV)** mechanism, allowing consumer chains to submit evidence of validator misconduct to the provider chain.

A significant challenge is that each Cosmos chain requires an initial validator set, and as more chains are created, some may lack sufficient validators, leading to weaker security. To address this, Cosmos introduced **Interchain Security**, which allows application chains to “rent” security from Cosmos Hub. By doing so, application chains leverage Cosmos Hub’s validators and staked tokens to secure themselves, avoiding the validator scarcity problem during their early stages. The provider chain (Cosmos Hub) is compensated with gas fees and block rewards from the consumer chains.

The concept of **Practical Hub Minimalism** was introduced with Interchain Security, focusing on reducing Cosmos Hub’s complexity to improve its performance and manageability. By retaining only core functionalities, Cosmos Hub’s efficiency is enhanced.

However, Interchain Security also presents risks, such as:

- The cost of validating multiple chains could drive smaller validators out, increasing centralization.
- Some chains may not generate sufficient gas fees to compensate validators, leading to inflationary token policies to attract them.

#### IBC Event Handling Vulnerability

The **IBC data packet** is a fundamental building block of IBC. When a relayer submits a packet from the source chain to the target chain using a **`MsgRecvPacket`** message, the data is authenticated and processed through the `OnRecvPacket` callback, which delivers it to the appropriate application module. Unlike most other Cosmos functions, `OnRecvPacket` does not handle errors through a standard Go error return. As a result, even if a transaction fails, related events are still triggered.

<details><summary>Code</summary>

```go
func (k Keeper) RecvPacket(goCtx context.Context, msg *channeltypes.MsgRecvPacket) (*channeltypes.MsgRecvPacketResponse, error) {
    [...]
    // Perform application logic callback
    // Cache context so that we may discard state changes from callback if the acknowledgement is unsuccessful.
    cacheCtx, writeFn := ctx.CacheContext()
    ack := cbs.OnRecvPacket(cacheCtx, msg.Packet, relayer)
    if ack == nil || ack.Success() {
        // Write application state changes for asynchronous and successful acknowledgements
        writeFn()
    } else {
        // Emit events even for failed acknowledgments
        ctx.EventManager().EmitEvents(cacheCtx.EventManager().Events())
    }
}
```

</details>

If an IBC packet processing fails, it should not generate events. However, this function triggers events even in case of failure. An attacker could exploit this behavior to trigger false events during errors, deceiving off-chain applications (like centralized exchanges or cross-chain bridges) into believing certain operations are completed when they have not been confirmed on-chain.

This vulnerability affects centralized exchanges and cross-chain bridges relying on Cosmos events. For example, an attacker could generate fake deposit events, creating asset security risks. The solution is to ensure **events are not emitted for failed acknowledgments**.

#### IBC Address Mapping Vulnerability

IBC is a key strength of the Cosmos ecosystem, but its trust model must be well-understood during integration. For instance, in an **EVMOS airdrop**, due to differing address formats between EVMOS and Stride, the `convertAddressToStrideAddress` function failed to map addresses correctly. To resolve this, Stride allowed cross-chain IBC messages to update target addresses in the airdrop records, enabling users to claim their airdrops. This update mechanism, implemented through the `x/autopilot` module, intercepted incoming IBC ICS-20 transfers and attempted to extract Stride-specific instructions from their memo or receiver fields.

**Problem**: The update mechanism relied solely on the sender address within the IBC packet for protection, with no additional verification. An attacker could control a malicious IBC client and forge IBC transfer packets containing a victim’s address to update the airdrop address and steal the airdrop.

<details><summary>Code</summary>

```go
func (k Keeper) TryUpdateAirdropClaim(
	ctx sdk.Context,
	data transfertypes.FungibleTokenPacketData,
	packetMetadata types.ClaimPacketMetadata,
) error {
	[..]

    // grab relevant addresses
    senderStrideAddress := utils.ConvertAddressToStrideAddress(data.Sender)
    if senderStrideAddress == "" {
    	return errorsmod.Wrapf(sdkerrors.ErrInvalidAddress, fmt.Sprintf("invalid sender address (%s)", data.Sender))
    }
    newStrideAddress := packetMetadata.StrideAddress

    // update the airdrop
    airdropId := packetMetadata.AirdropId
    k.Logger(ctx).Info(fmt.Sprintf("updating airdrop address %s (orig %s) to %s for airdrop %s",
    	senderStrideAddress, data.Sender, newStrideAddress, airdropId))

    return k.claimKeeper.UpdateAirdropAddress(ctx, senderStrideAddress, newStrideAddress, airdropId)

}

```

</details>

---

## 3. CosmWasm: A Smart Contract Virtual Machine for Cosmos

If you prefer not to build a full blockchain, **CosmWasm** provides a smart contract virtual machine (VM) for the Cosmos SDK, enabling the development of decentralized applications (dApps) using smart contracts. CosmWasm supports multiple programming languages for writing contracts, which are then compiled into WebAssembly (WASM) bytecode and deployed on the Cosmos network.

#### Deploying Smart Contracts on CosmWasm

To deploy a contract, developers upload the WASM bytecode to the blockchain. Upon successful upload, the blockchain returns a unique code ID. To create an instance of the contract, the `instantiate` function is called. Each instantiation generates a unique contract address.

<details><summary>Code</summary>

```rust
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Implementation logic
}
```

</details>

#### Key Features and Security of CosmWasm

- **Actor Model**  
   CosmWasm employs an **Actor Model**, which ensures that each contract processes one message at a time. This design prevents common vulnerabilities such as reentrancy attacks. Additionally, the use of Rust's built-in integer overflow checks mitigates risks of arithmetic overflow vulnerabilities.

- **Secure Token Transfers**  
   Token transfers are safely handled through well-defined APIs. For example:

<details><summary>Code</summary>

```rust
fn send_tokens(to_address: Addr, amount: Vec<Coin>, action: &str) -> Response {
    Response::new()
        .add_message(BankMsg::Send {
            to_address: to_address.clone().into(),
            amount,
        })
        .add_attribute("action", action)
        .add_attribute("to", to_address)
}
```

</details>

- **Clear Entry Points**  
   All contract transactions are processed through the `execute` function. This function routes messages to appropriate handlers based on their type, ensuring that unnecessary entry points remain inaccessible. This reduces the risk of accidental exposure to unauthorized actions.

<details><summary>Code</summary>

```rust
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePot { target_addr, threshold } =>
            execute_create_pot(deps, info, target_addr, threshold),
        ExecuteMsg::Receive(msg) =>
            execute_receive(deps, info, msg),
    }
}
```

</details>

- **Submessage Mechanism**  
   To obtain results from external contract calls, CosmWasm uses [submessages](https://docs.cosmwasm.com/docs/smart-contracts/message/submessage/). Submessages allow contracts to interact with other contracts or modules, and the calling contract can receive responses via callback functions.

- **Minimized Attack Surface**  
   CosmWasm eliminates several common vulnerabilities by design:
  - No uninitialized storage pointers.
  - No delegate calls, reducing the likelihood of complex security flaws.

### Vulnerability Example: Recursive Stack Overflow in CosmWasm

One significant vulnerability in CosmWasm affected multiple versions of **wasmvm** and **cosmwasm-vm**, potentially leading to virtual machine stack overflow crashes. This issue arose due to recursive loops created between **WASM imports and exports**, allowing malicious contracts to exploit the VM.

In this attack:

1. The contract invokes an imported function (e.g., `addr_validate`).
2. The host calls the `allocate` export function.
3. The contract recursively calls another imported function, and the host again invokes `allocate`.

This recursive loop can continue until the virtual machine stack overflows, typically after about 140 recursive calls. Malicious contracts could exploit this by defining an `allocate` function that deviates from standard behavior, creating an infinite recursion between the contract and the host. The issue caused VM crashes and presented a denial-of-service risk.

---

## 4. Cosmos Security

One of the key features of the Cosmos Stack is its "flexibility." The layers of the stack are porous, meaning developers can write application logic that influences various layers of the entire tech stack. This high degree of customization makes Cosmos very developer-friendly, but it also introduces several security risks.

### 4.1 Common Vulnerabilities in Cosmos

#### 4.1.1 **Non-Determinism**

The consensus mechanism of blockchains requires that, given the same input, all nodes must produce consistent output. When the consensus layer is opened to application developers, any non-deterministic parts in the code can lead to chain forks or unfair penalties for validators. Generally, anything that touches the state machine in the Cosmos SDK must be deterministic.

**Solutions:**

- **Static Analysis**: Use static code analysis tools (like custom CodeQL rules) to detect potential non-deterministic issues.
- **Cross-Architecture Testing**: Test applications across nodes with different architectures or require nodes to run on specific architectures to ensure consistency.
- **Prepare for Blockchain Fork Recovery**: Preemptively prepare and test programs and processes to recover from blockchain splits.

**Vulnerabilities:**

- **Randomness**: In Cosmos SDK, using Go's `rand` package to generate random numbers, especially within the state machine logic, can lead to non-determinism. It's recommended to use Chainlink’s VRF.
- **Go Map Internals**: Go’s maps are implemented using hash buckets, and if the state machine directly iterates through a Go map, it could introduce non-determinism. It is recommended to convert the map to a slice and sort it to avoid issues with unordered iteration.

<details><summary>Code</summary>

```go
  m := map[string]int{
      "a": 0,
      "b": 1,
      "c": 2,
      "d": 3,
  }
  // BAD: Non-deterministic
  for k, v := range m {
      fmt.Println(k, v)
  }

  // GOOD: Deterministic
  ks := make([]string, 0, len(m))
  for k := range m {
      ks = append(ks, k)
  }
  sort.Strings(ks)
  for _, k := range ks {
      fmt.Println(k, m[k])
  }
```

</details>

- **Invalid Time Handling**: Avoid using `time.Now()` (as nodes might not process messages at the same time). Instead, use `ctx.BlockTime()` to ensure time consistency.

<details><summary>Code</summary>

```go
  func (g Grant) ValidateBasic() error {
      if g.Expiration.Unix() < time.Now().Unix() {
          return sdkerrors.Wrap(ErrInvalidExpirationTime, "Time can't be in the past")
      }
  }
```

</details>

- **API Calls**: Network requests are typically non-deterministic and should be avoided within the state machine.
- **Concurrency and Multithreading (goroutines and select statements)**: The scheduling of goroutines in Go is unpredictable, which could lead to inconsistent behavior. It's recommended to use independent **CacheKV stores** in threads to avoid write conflicts.
- **Floating Point Issues**: Floating-point operations may behave inconsistently across platforms. Avoid using floats in the state machine; instead, use **fixed-point arithmetic** to ensure cross-platform consistency.

<details><summary>Code</summary>

```go
  var f, f2, f3 float64 = 0.1, 0.2, 0.3
  // FALSE
  fmt.Println((f+f2)+f3 == f+(f2+f3))
```

</details>

- **Gas Calculation Inconsistency**: Different nodes may calculate Gas inconsistently due to state reading differences. Ensure no unnecessary non-deterministic reads occur to maintain consistency in Gas calculation.
- **`signer` Validation**: In Cosmos SDK, transaction signatures are validated through the `GetSigners` method. In complex scenarios where multiple signatures or proxy mechanisms are needed, improper validation can allow attackers to spoof user identities.

<details><summary>Code</summary>

```go
service Msg {
      rpc CreatePost(MsgCreatePost) returns (MsgCreatePostResponse);
}

message MsgCreatePost {
  string signer = 1;
  string author = 2;
  string title = 3;
  string body = 4;
}

message MsgCreatePostResponse {
  uint64 id = 1;
}

message Post {
  string author = 1;
  uint64 id = 2;
  string title = 3;
  string body = 4;
}

// The signer field is used for signature verification
func (msg *MsgCreatePost) GetSigners() []sdk.AccAddress {
    signer, err := sdk.AccAddressFromBech32(msg.Signer)
    if err != nil {
        panic(err)
    }
    return []sdk.AccAddress{Signer}
}

func (msg *MsgCreatePost) GetSignBytes() []byte {
    bz := ModuleCdc.MustMarshalJSON(msg)
    return sdk.MustSortJSON(bz)
}

func (msg *MsgCreatePost) ValidateBasic() error {
    _, err := sdk.AccAddressFromBech32(msg.Signer)
    if err != nil {
        return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "invalid creator address (%s)", err)
    }
    return nil
}

// The author field is saved along with the post's content
func (k msgServer) CreatePost(goCtx context.Context, msg *types.MsgCreatePost) (*types.MsgCreatePostResponse, error) {
    ctx := sdk.UnwrapSDKContext(goCtx)

    var post = types.Post{
        Author: msg.Author,
        Title:  msg.Title,
        Body:   msg.Body,
    }

    id := k.AppendPost(ctx, post)

    return &types.MsgCreatePostResponse{Id: id}, nil
}

```

</details>

- **Platform-Dependent Types**: Platform-dependent features, such as the size of the `int` type (which differs between 32-bit and 64-bit systems) or file paths (`filepath.Ext`), should be handled with caution.
- **Memory Addresses**: Memory addresses may vary across different nodes, potentially affecting the calculation results.
- **Unsafe Packages, Reflection, and Runtime**: These packages could cause unpredictable behavior.

#### 4.1.2 **In-Protocol Panics**

In Cosmos SDK, certain code can be executed at the protocol layer (not just through specific transactions). In Go, panic represents a runtime error mechanism that usually indicates an irrecoverable error, and recover is used to capture and recover from a panic to allow the program to continue running. Cosmos SDK captures and recovers from panics during most transaction processing stages (e.g., proposal preparation, proposal processing, and transaction execution in PrepareProposal, ProcessProposal, and DeliverTx). However, if a panic occurs during the BeginBlock and EndBlock stages, the system does not have an automatic recovery mechanism.

- **Math Overflow**: During `BeginBlock` and `EndBlock`, math operations can panic due to overflow in `Coins`, `DecCoins`, `Dec`, `Int`, and `UInt` types. Handling overflows within `Begin/EndBlock` is essential to prevent an attacker from halting the blockchain using overflow attacks.

<details><summary>Code</summary>

```go
var x uint8 = 255
// INT OVERFLOW: prints 0
fmt.Println(x + 1)
```

</details>

- **Invalid Parameters (`new Dec`)**: Invalid parameters or unexpected conditions during the creation of `Dec` types may lead to panic. For example, when calling the SetParamSet method of the `x/params` module, passing invalid parameters may lead to a panic and cause the program to terminate.
- **Batch Transfers**: For batch transfer operations, especially the `SendCoins` function, care should be taken to prevent a panic from one transfer causing the entire chain to halt. The `SendCoins` function is a black-box function that allows transferring multiple types of tokens at once, without individually validating each token transfer. Best practice is to validate transfers individually to prevent a chain crash if an issue arises with a specific transfer (e.g., using `SendCoin` for individual validation of each transfer).
- **Error Handling**: In Go, errors are handled by comparing returned error values with `nil`. Failing to handle errors properly, especially in Cosmos SDK's `bankKeeper` module, may allow transactions to be executed even when the sender has insufficient funds.

<details><summary>Code</summary>

```go
// The SendCoins function is called without checking the error return value.
// If the sender doesn't have enough balance, the transaction should fail, but due to the lack of error handling, the transaction proceeds.
func (k msgServer) Transfer(goCtx context.Context, msg *types.MsgTransfer) (*types.MsgTransferResponse, error) {
    ...
    // Error not handled
    k.bankKeeper.SendCoins(ctx, sender, receiver, amount)
    ...
    return &types.MsgTransferResponse{}, nil
}
```

</details>

#### 4.1.3 Gas and Fee Market Issues

**1) Unmetered Computation & Infinite Loops in Hooks and CosmWasm Contracts**

In the Cosmos SDK, only state write operations incur gas costs. This means that if some computations are not limited by gas, attackers can exploit them by consuming infinite computational resources, potentially causing the chain to stall. **A solution to this issue is to wrap risky function calls in a separate context with a gas limit.** This ensures that these calls have a gas budget allocated, preventing them from running indefinitely and halting the chain.

- **Infinite Loops in Hooks and CosmWasm Contracts**: For instance, if an attacker uploads a CosmWasm contract with an infinite loop, the execution of that contract could prevent the chain from continuing.

- **Improper Loop/Recursion Exit Conditions**: When loops or recursion lack proper exit conditions, they can lead to infinite loops or excessive recursion, consuming too many resources and causing the blockchain to stall. It is crucial to ensure that all loops and recursive functions have well-defined and effective exit conditions.

- **Slow Convergence in Mathematical Operations**: In mathematical computations, especially complex functions like **PowApprox**, slow convergence can be exploited by attackers, which may lead to the blockchain halting. A solution is to implement constant bounds on the number of iterations in such functions and use more efficient algorithms, such as **rational approximations** instead of **Taylor expansions**, to improve performance and accuracy.

- **Lack of Gas Limits in ABCI Methods**: In blockchain applications, methods in the Application Blockchain Interface (ABCI), like `EndBlocker`, do not have gas limits, meaning their execution time is unlimited. This can lead to unforeseen performance issues if the methods are not carefully managed.

**2) Fee Pricing and Market Mechanisms**

Developers need to pay close attention to transaction fees, particularly during high-load periods, to ensure that the fee mechanisms can effectively prevent Denial-of-Service (DoS) attacks.

- **State Write Fee Pricing**: Ensure that each state-write operation has a sufficiently high fee to resist spam attacks. For operations that may trigger externalities, fees should scale incrementally to prevent DoS attacks.

- **Contract Call Fees**: Ideally, contract calls should charge fees as additional gas, which would accurately transfer the costs to the user. If this is not feasible, contract design should handle fee distribution precisely to avoid disproportionate fees or losses.

- **Fee Market Mechanism**: During periods of high network usage, it’s essential to have an effective fee market mechanism in place to maintain the smooth operation of the blockchain. EIP-1559 provides an effective model, and in the long term, integrating it into consensus could be an optimal solution. A more robust long-term solution involves integrating the fee market directly into consensus through `ABCI 2.0`, with the Skip team leading the way in implementing this approach.

- **Consistency Between Transaction Simulation and Execution**: Due to challenges in gas estimation in Cosmos SDK, the gas estimate used during simulation often differs from the actual execution cost. Ensuring that gas estimates are consistent between simulation and real execution is critical, particularly by correctly handling simulation parameters and logic in the AnteHandler. This helps avoid instability and transaction failures caused by inconsistent gas estimates, ensuring a smoother user experience.

<details><summary>Code</summary>

```go
func (mfd MyMemPoolDecorator) AnteHandle(ctx sdk.Context, tx sdk.Tx, simulate bool, next sdk.AnteHandler) (newCtx sdk.Context, err error) {
    if !simulate {
    // Do some gas intensive logic such as many store reads/writes
    // This will lead to inconsistencies between transaction simulation and execution
    }
}

```

</details>

#### 4.1.4 Key Malleability & Prefix Iteration

In Cosmos SDK, operations related to key-value storage can introduce potential security vulnerabilities. This is especially true during prefix iteration. If the components of the key are mutable (such as IDs), attackers can manipulate these IDs to access data they should not have access to. The Cosmos SDK team has released a collection package designed to mitigate the risks associated with the key-value storage issues discussed below.

**1) Prefix Iteration**

When developers use mutable IDs (e.g., pool IDs) as part of the key prefix, attackers could exploit this by creating additional pools with incremented IDs, causing the iteration process to include objects that should not be part of the result. There are two primary ways to prevent this issue:

- **Adding a separator suffix to the prefix**: By appending a delimiter to the prefix, developers can ensure that the key structure remains intact and does not inadvertently include extra entries during iteration.
- **Converting extendable numbers to big-endian format**: This approach ensures that numbers are represented in a consistent order, making it harder for attackers to manipulate the key structure during iteration.

**2) Key Uniqueness**

In many cases, developers attempt to generate a unique identifier for a data structure by combining multiple fields. For example, a unique key for a pool might be created using the two tokens in the pool and an expansion factor. While the code may appear straightforward, it can result in key collisions when multiple pools have the same token combination and expansion factor, leading to overwriting of existing entries.

To prevent key collisions, developers should add a unique identifier component (e.g., an ID) or another equivalent unique marker to ensure that each key remains distinct.

<details><summary>Code</summary>

```go
// KeyPool returns the key for the given pool
func KeyPool(pool Pool) []byte {
    return []byte(fmt.Sprintf("%s%s%s%s", PoolPrefix, pool.GetToken0(), pool.GetToken1(), pool.GetSpreadFactor()))
}
```

</details>

**Iterator Boundaries**

When using iterators, particularly with prefix iteration, developers must ensure they handle the boundaries correctly. The starting byte is included, but the ending byte is excluded. For reverse iteration, an additional offset is required for the end byte to prevent errors that could arise from "off-by-one" mistakes, which may cause certain data to be missed during iteration. Careful attention to these boundaries is crucial for ensuring the integrity of the iteration process and avoiding unintended results.

#### 4.1.5. Message Processing

**1) Messages Without Priority**

In the Cosmos mempool, transactions are, by default, sorted in a "first come, first served" (FIFO) manner without any fee-based sorting mechanism. However, certain types of messages may be more important than others and should be processed with higher priority. This means that the more critical message types should be included in blocks as early as possible to ensure their precedence. If messages are not properly prioritized, attackers could exploit this by filling the mempool with low-fee transactions, gaining unfair advantages in message processing. Additionally, during network congestion, some messages may remain unprocessed for an extended period, leading to system failures.

**Example: Price Update (Commit and Reveal)**

Price updates should occur after every voting period. However, attackers can send a large number of low-fee transactions that fill up block space, preventing critical price updates from being included in a block. This would allow the attacker to profit from outdated prices within the system.

<details><summary>Code</summary>

```go
service Msg {
  rpc Lend(MsgLend) returns (MsgLendResponse);
  rpc Borrow(MsgBorrow) returns (MsgBorrowResponse);
  rpc Liquidate(MsgLiquidate) returns (MsgLiquidateResponse);
  rpc OracleCommitPrice(MsgOracleCommitPrice) returns (MsgOracleCommitPriceResponse);
  rpc OracleRevealPrice(MsgOracleRevealPrice) returns (MsgOracleRevealPriceResponse);
}

```

</details>

**Example: Pool Operations (Swap vs Pause)**

When vulnerabilities are discovered in a pool implementation, attackers and pool operators may compete to determine which message (Swap or Pause) gets executed first. Prioritizing the Pause message allows pool operators to prevent vulnerabilities from being exploited. However, if the attacker's message is processed first, the priority mechanism won't fully prevent the attacker's transaction from executing.

<details><summary>Code</summary>

```go
service Msg {
  rpc CreatePool(MsgCreatePool) returns (MsgCreatePoolResponse);
  rpc Deposit(MsgDeposit) returns (MsgDepositResponse);
  rpc Withdraw(MsgWithdraw) returns (MsgWithdrawResponse);
  rpc Swap(MsgSwap) returns (MsgSwapResponse);
  rpc Pause(MsgPause) returns (MsgPauseResponse);
  rpc Resume(MsgResume) returns (MsgResumeResponse);
}
```

</details>

In this case, the `Pause` message allows privileged users to pause a liquidity pool. However, attackers may collaborate with malicious validators to manipulate the transaction order, bypassing priority restrictions.

**Solutions:**

- **Use the CheckTx Priority Return Value for Sorting Messages**: This feature sorts transactions based on the transaction itself, not individual messages. A transaction may contain multiple messages, and the overall priority of the transaction should be considered for sorting.
- **Verify Authorization Early**: Perform authorization checks during the `CheckTx` phase to prevent attackers from filling the mempool with invalid but prioritized transactions, thus blocking legitimate transactions from entering the mempool.
- **Charge Higher Fees for High-Priority Transactions**: By requiring higher fees for high-priority transactions, attackers are deterred from flooding the system with low-fee messages, increasing the cost of launching attacks.

**2) Unregistered Message Handlers**

In Cosmos SDK, message handlers are responsible for processing different types of messages. When handling messages, the application must ensure that all message handlers are registered. If a message handler is not registered, users will be unable to send that message. In previous versions of the Cosmos SDK, message handlers needed to be manually registered in the module's `NewHandler` method. However, in the latest version, this manual registration is no longer necessary.

**Example: Missing Handler for `CancelCall`**

Consider an application where multiple message types are defined. If the handler for a specific message type (e.g., `CancelCall`) is not registered correctly, the system will fail to process requests related to that message type.

<details><summary>Code</summary>

```go
func NewHandler(k keeper.Keeper) sdk.Handler {
    msgServer := keeper.NewMsgServerImpl(k)

    return func(ctx sdk.Context, msg sdk.Msg) (*sdk.Result, error) {
        ctx = ctx.WithEventManager(sdk.NewEventManager())
        switch msg := msg.(type) {
        // Handle various messages...
        case *types.MsgCancelCall:  // Missing handler for MsgCancelCall
            return nil, sdkerrors.Wrap(sdkerrors.ErrUnknownRequest, "Unrecognized Gravity Msg type: CancelCall")
        // Handle other message types...
        }
    }

}

```

</details>

**Solutions:**

- **Use the New Msg Service Mechanism**: The latest versions of Cosmos SDK no longer require manual registration of message types. By using the new Msg service mechanism, this issue can be avoided.
- **Deploy Static Analysis in CI Pipelines**: Static analysis tools can be employed in the CI pipeline to check for missing message handlers and ensure that all message types are correctly registered, preventing potential runtime issues caused by unregistered handlers.

#### 4.1.6 Managing Token Transfers with the `x/bank` Module in Cosmos SDK

In the Cosmos SDK, the `x/bank` module is the standard token management module responsible for handling token minting, burning, and transferring. However, if an application implements custom financial management (such as through the `x/hodl` module), special care must be taken when utilizing the functions provided by the `x/bank` module. Failure to do so can result in inconsistent accounting and create security vulnerabilities that attackers could exploit.

**Solutions:**

- **Blacklist Mechanism**: One effective approach is to implement a blacklist (or blocklist) to prevent tokens from being transferred to specific addresses. For instance, certain addresses, like those associated with the `x/hodl` module, can be blacklisted to ensure that tokens are not mistakenly or maliciously sent to these addresses. This is particularly important in cases where tokens need to be locked or restricted from being spent immediately.

- **`SendEnabled` Parameter**: The `SendEnabled` parameter within the `x/bank` module can be used to restrict token transfers on a per-token basis. By setting this parameter, you can ensure that only authorized transfers are executed. This provides an additional layer of control over how and when tokens can be moved, reducing the risk of unauthorized or malicious transfers.

- **Blacklist Checks**: When implementing new token transfer functionality, it is crucial to explicitly check against the blacklist to prevent unauthorized transfers. This check should be performed every time a transfer is initiated, ensuring that the system consistently enforces the rules and prevents malicious actors from bypassing restrictions.

<details><summary>Code</summary>

```go
// A malicious user can bypass the accounting management of the x/hodl module by directly transferring tokens to it,
// causing the accounting check to fail, which may lead to a Denial-of-Service (DoS) attack that halts the chain.
func BalanceInvariant(k Keeper) sdk.Invariant {
    return func(ctx sdk.Context) (string, bool) {
        weAreFine := true
        msg := "hodling hard"

        weHold := k.bankKeeper.SpendableCoins(authtypes.NewModuleAddress(types.ModuleName)).AmountOf("BTC")
        usersDeposited := k.GetTotalDeposited("BTC")

        if weHold != usersDeposited {
            msg = fmt.Sprintf("%dBTC missing! Halting chain.\n", usersDeposited - weHold)
            weAreFine = false
        }

        return sdk.FormatInvariant(types.ModuleName, "hodl-balance",), weAreFine
    }
}

// A malicious user can manipulate the exchange rate between tokens and xToken
// by transferring tokens directly to the module or by using IBC to move tokens to another chain,
// potentially gaining unfair profits.
func (k Keeper) GetExchangeRate(tokenDenom string) sdk.Coin {
    uTokenDenom := createUDenom(tokenDenom)

    tokensHeld := k.bankKeeper.SpendableCoins(authtypes.NewModuleAddress(types.ModuleName)).AmountOf(tokenDenom).ToDec()
    tokensBorrowed := k.GetTotalBorrowed(tokenDenom)
    uTokensInCirculation := k.bankKeeper.GetSupply(uTokenDenom).Amount

    return (tokensHeld + tokensBorrowed) / uTokensInCirculation
}
```

</details>

#### 4.1.7 Rounding, Overflow, Underflow, and Type Conversion Issues

When developing applications using the Cosmos SDK, handling numerical rounding issues is critical, especially when dealing with token quantities. The Cosmos SDK provides two primary numerical types: `sdk.Int` (for integers) and `sdk.Dec` (for decimals). While `sdk.Dec` supports decimal arithmetic, it may encounter precision and rounding issues, potentially leading to rounding errors. Additionally, improper rounding directions can cause system inconsistencies or open the door to malicious exploits.

Overflow and underflow problems arise during integer operations. Overflow occurs when a value exceeds the maximum representable range, while underflow happens when the value goes below the minimum range. For example, when an 8-bit unsigned integer overflows, it wraps around to 0, yielding unexpected results. Similarly, type conversions (such as converting signed integers to unsigned integers) can lead to unintended behavior.

**Solutions:**

- **Rounding Rules**: Ensure rounding is handled in a way that favors system integrity rather than user advantage. It is essential to define the correct rounding direction.
- **"Multiply Before Divide" Approach**: To reduce rounding errors, it is better to perform multiplication first and then division.

<details><summary>Code</summary>

```go
func TestDec() {
    a := sdk.MustNewDecFromStr("10")
    b := sdk.MustNewDecFromStr("1000000010")
    x := a.Quo(b).Mul(b)
    fmt.Println(x)  // 9.999999999999999000

    q := float32(10)
    w := float32(1000000010)
    y := (q / w) * w
    fmt.Println(y)  // 10

}

```

</details>

#### 4.1.8 Using Vulnerable Components

Over time, older versions of libraries and tools may contain known security vulnerabilities. This issue may also be present in components such as the Cosmos SDK and Go language libraries. If these components are not updated promptly, they could pose significant security risks to blockchain projects.

**Recommendation**: Always maintain dependencies at their latest versions and regularly check for updates to the Cosmos SDK and other related components. In particular, address any vulnerabilities by promptly applying patches and updates.

### 4.2 Fuzz Testing in Cosmos: Enhancing Code Reliability

The Cosmos SDK divides testing into four levels, each with an increasing scope, to ensure the functionality and stability of both individual modules and the entire application. Here's an overview of each level:

- **Unit Tests**: Unit tests focus on validating the functionality of a single module. Dependencies of the module are replaced with **mocks** to simulate their behavior. These tests ensure that the module's internal functions work as intended, verifying that they correctly interact with the mocked dependencies and handle responses as expected. Comprehensive unit testing involves covering all functions within the module.

- **Integration Tests**: Integration tests expand the scope by testing the module along with all its necessary dependencies. Unlike unit tests, dependencies are no longer mocked. Instead, the tests use a minimum viable application that includes the module and its required dependencies, ensuring they function together correctly. This approach avoids unnecessary complexity by limiting the application to the essential components.

- **Simulation Tests**: Building on integration tests, simulation tests introduce random parameters to evaluate how the module behaves under various conditions. This level of testing simulates more complex and unpredictable scenarios, providing insights into how the module performs in a real-world environment.

- **End-to-End (E2E) Tests**: At the top of the testing hierarchy, end-to-end tests evaluate the entire application workflow, simulating user interactions. These tests verify that all system components and workflows operate seamlessly, ensuring a smooth and reliable user experience. Unlike other levels, E2E tests focus on the application's overall functionality rather than individual modules.

Cosmos is a platform that supports the creation of blockchains using Go (or other languages), with its reference implementation, Cosmos SDK, leveraging fuzz testing extensively during the development process to identify potential issues in the code. The Cosmos SDK employs two main types of fuzz testing techniques:

- **Smart Fuzzing**: Used for low-level code, this method efficiently explores the input space of a program by instrumenting the source code.
- **Dumb Fuzzing**: Used for high-level simulation, relying on randomly generated transactions for testing.

#### Low-Level Testing: Smart Fuzzing

Smart fuzzing utilizes tools like **AFL**, **go-fuzz**, or Go’s built-in fuzz testing framework to test low-level functions in the Cosmos SDK, typically those that are stateless. This method is highly effective at quickly identifying issues in low-level code. However, its limitation is that it can only discover simple, low-level problems and often misses more complex bugs.

For example, consider testing the parsing function for normalized coin types:

<details><summary>Code</summary>

```go
func FuzzTypesParseCoin(f *testing.F) {
    f.Fuzz(func(t *testing.T, data []byte) {
        _, _ = types.ParseCoinNormalized(string(data))
    })
}
```

</details>

In this case, the fuzz testing engine generates random input data, which is passed to the `ParseCoinNormalized` function. The goal is to find any errors or unexpected behaviors that occur when processing random byte arrays.

While smart fuzzing is efficient at detecting low-level issues, it often cannot uncover deeper, more complex bugs that may arise in the context of the entire blockchain application.

#### High-Level Testing: Dumb Fuzzing

To identify more intricate bugs, Cosmos SDK provides the **Cosmos Blockchain Simulator**, which takes a top-down approach rather than the bottom-up approach used in smart fuzzing. This tool executes transactions with random operations, starting from a random or predefined genesis state, and simulates a full transaction sequence. The random nature of the transactions allows it to uncover potential problems in Cosmos applications. This method is called "Dumb Fuzzing" because it relies purely on randomness to drive the tests.

In the simulator, developers implement necessary functions to generate random genesis states and transactions. For example, testing the `MsgSend` operation in the `x/nft` module might look like this:

<details><summary>Code</summary>

```go
func SimulateMsgSend(
    cdc *codec.ProtoCodec,
    ak nft.AccountKeeper,
    bk nft.BankKeeper,
    k keeper.Keeper,
) simtypes.Operation {
    return func(
        r *rand.Rand, app *baseapp.BaseApp, ctx sdk.Context, accs []simtypes.Account, chainID string,
    ) (simtypes.OperationMsg, []simtypes.FutureOperation, error) {
        sender, _ := simtypes.RandomAcc(r, accs)
        receiver, _ := simtypes.RandomAcc(r, accs)
        // ...
    }
}
```

</details>

Here, the simulator generates random accounts and simulates a transaction between them, providing valuable insight into how the Cosmos blockchain behaves under various conditions.

#### Combining Smart and Dumb Fuzzing

To maximize the advantages of both fuzz testing techniques, Cosmos SDK combines smart fuzzing and dumb fuzzing by using Go’s native fuzz testing engine to control the randomness of the genesis state, transaction generation, and block creation. This hybrid approach allows the fuzz testing engine to effectively control the seed for the simulator's random values, making the testing process more controlled and efficient.

For example, smart fuzzing can control the seed used in the simulator, ensuring that the randomness generated during the test is consistently reproducible:

<details><summary>Code</summary>

```go
func FuzzFullAppSimulation(f *testing.F) {
    f.Fuzz(func(t *testing.T, input []byte) {
       config.Seed = IntFromBytes(input)  // Control the seed with smart fuzzing
       // Execute the full simulator test
    })
}
```

</details>

This integration ensures that the fuzz testing engine is able to dictate the randomness involved in the simulator, allowing for better control over the inputs and making it easier to track down bugs.

To enhance control over random inputs, a custom random number generator can be defined using Go’s standard library `math/rand`. For instance, a new random source `arraySource` can be created, where input data (such as an array of `int64` values) acts as a deterministic random number source:

<details><summary>Code</summary>

```go
type arraySource struct {
    pos int
    arr []int64
    src *rand.Rand
}

func (rng *arraySource) Uint64() uint64 {
    if rng.pos >= len(rng.arr) {
        return rng.src.Uint64()  // Switch to standard random values once the array is exhausted
    }
    val := rng.arr[rng.pos]
    rng.pos++
    if val < 0 {
        return uint64(-val)
    }
    return uint64(val)
}


```

</details>

In this approach, the testing engine continues generating random numbers using the custom-defined array until it is exhausted, at which point it falls back to standard random values. This ensures that the fuzz testing process remains continuous, even if the custom random values have been fully utilized.

---

### Conclusion

Cosmos, known as the "Internet of Blockchains," enables interoperability through components like the Cosmos SDK, IBC protocol, Tendermint for finality, Cosmos Hub, Ignite CLI, and CosmWasm for smart contracts. Its flexibility allows developers to create customizable blockchains but introduces security risks, including non-determinism, in-protocol panics, and gas issues. Fuzz testing, both smart and dumb, strengthens the Cosmos SDK by identifying vulnerabilities and edge cases, ensuring secure and reliable blockchain operations.

---

[interchain-security](https://github.com/ETAAcademy/ETAAcademy-Audit/tree/main/Articles/Appendix/interchain-security)

<div  align="center">
<img src="img/04_cosmos_security.gif" width="50%" />
</div>
