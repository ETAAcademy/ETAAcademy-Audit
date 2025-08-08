# ETAAcademy-Audit: 19. Trust Flow

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>19 Trust Flow</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>Trust Flow</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# AI-Driven Detection of Smart Contract Interaction Vulnerabilities: From Graph Neural Networks to Cross-Chain Bridge Security

Trustflow analysis of smart contract interactions is crucial for ensuring secure and verifiable call flows, particularly given that interaction vulnerabilities pose a major security threat to DApps. The field has shifted from traditional expert-based detection methods to AI-powered automated systems. Traditional approaches like symbolic execution and static analysis often produce false positives and miss real vulnerabilities when dealing with complex cross-contract interactions. This has led to the rise of deep learning and other AI-driven methods that can learn vulnerability patterns automatically, significantly boosting both accuracy and coverage.

Graph Convolutional Networks (GCNs) have proven particularly effective by using call graph structures and semantic relationships to spot vulnerabilities more accurately. Since most Ethereum contracts are only available as bytecode (not source code), researchers have developed sophisticated deep learning approaches that combine opcodes, function signature recovery, and attention mechanisms. These methods use multi-label classification to integrate semantic context with function signature recovery, employing bidirectional LSTM encoders, attention decoders, and focal loss functions for precise detection.

For cross-chain bridge security, researchers have developed a two-stage graph mining framework based on cross-chain transaction execution graphs. This approach captures both big-picture structural patterns through global mining and detailed anomalies through local mining of network motifs, allowing it to identify attacks without needing to match transaction pairs. Additionally, researchers are pursuing both formal verification approaches for mathematical security proofs and innovative cross-chain exchange mechanisms that enable secure bridgeless transactions using trade value equivalence principles.

Despite these advances, current detection tools still fall short in semantic coverage and cross-chain security analysis. The future likely lies in hybrid AI approaches that blend sequential models with graph-based models, incorporating multiple semantic inputs to create highly accurate, low-noise specialized detection tools capable of handling increasingly sophisticated security threats.

---

## 1. GCN-Based Detection of Interaction Vulnerabilities in Smart Contracts

**Interaction vulnerabilities** are a critical area of research in modern software security. These vulnerabilities arise when systems interact with external components—be it users, third-party protocols, or external data sources. While prevalent in various domains such as IoT systems and large distributed architectures, they are especially dangerous in **smart contracts**, where interaction bugs can lead to severe financial losses. As decentralized applications (DApps) continue to grow in complexity and interdependence, detecting these vulnerabilities in blockchain-based systems becomes increasingly challenging.

In the context of smart contracts, **interaction vulnerabilities** typically refer to bugs that occur during interactions with **external contracts, protocols, users, or other on-chain components**. These include well-known exploits such as:

- **Reentrancy attacks**
- **Unchecked external calls**
- **Authorization bypasses**
- **Race conditions**
- **Time-dependency vulnerabilities**
- **Flash loan attacks**

One of the most prominent and representative issues is the **unchecked external call** vulnerability. When a smart contract invokes an external call—such as `transfer()`, `send()`, `call()`, or `delegatecall()`—it must verify the return value to confirm whether the operation succeeded. Failing to do so can result in faulty logic that assumes success and proceeds incorrectly.

For instance, in a well-documented **Uniswap flash loan exploit**, attackers manipulated the price by leveraging the unchecked return value of the `getReserves()` function. This function retrieves token reserve data used for price calculations. If it fails but the return value is still trusted, attackers can inject false data to exploit the protocol. The code snippet below highlights this logic flaw:

<details><summary>Code</summary>

```solidity
contract UniswapV2Pair {
    function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external {
        require(amount0Out > 0 || amount1Out > 0, 'UniswapV2: INSUFFICIENT_OUTPUT_AMOUNT');

        // Unchecked external call
        (uint balance0, uint balance1) = getReserves();

        uint amountIn = amount0Out > 0 ? balance0 : balance1;
        require(amountIn > 0, 'UniswapV2: INSUFFICIENT_INPUT_AMOUNT');
    }
}
```

</details>

### Limitations of Traditional Detection Methods

Traditional vulnerability detection methods—such as **symbolic execution, static analysis, and taint tracking**—rely heavily on expert-defined rules. Tools like **Oyente** and **Securify** have been effective to a point, but suffer from:

- **High false-positive rates** due to over-approximation
- **Limited scalability** in complex cross-contract scenarios
- **Inability to understand high-level logic and semantics**

These techniques are constrained by rule-based heuristics and often fail to detect modern and context-dependent vulnerabilities.

### Rise of Deep Learning for Vulnerability Detection

To address these limitations, researchers have turned to **deep learning (DL)** approaches that can automatically learn semantic patterns in smart contract code. The typical DL-based vulnerability detection pipeline includes:

- **Data Collection & Labeling**
  Code samples (e.g., from Etherscan) are annotated with vulnerability types (e.g., reentrancy, arithmetic bugs).

- **Data Preprocessing**
  Code is transformed into machine-readable representations such as Abstract Syntax Trees (ASTs), Opcodes, and various program graphs (CFG, DFG, PDG).

- **Feature Extraction**

  - **Sequential Models** (e.g., LSTM, RNN, CodeBERT) process token sequences to capture syntactic patterns.
  - **Graph-based Models** (e.g., GCN, GAT) encode non-linear dependencies and contextual semantics.

- **Model Training & Optimization**
  Techniques include transfer learning, hyperparameter tuning, and attention mechanisms.

- **Model Evaluation & Deployment**
  Evaluated using metrics like **accuracy**, **precision**, **recall**, and **F1-score**, and integrated into auditing pipelines for real-time use.

### Sequential vs. Graph-Based Deep Learning Models

Deep learning approaches for smart contract vulnerability detection fall into two broad categories:

- **Sequential Models** (LSTM/RNN/Transformer-based):
  Treat code as a linear sequence of tokens. These are good at capturing local syntax and repetitive patterns but struggle with cross-function and global semantic dependencies.

- **Graph-Based Models** (e.g., GCN, GAT):
  Represent code via control flow graphs (CFG), data flow graphs (DFG), or program dependency graphs (PDG). These models excel at capturing non-linear control/data dependencies, making them better suited for detecting complex, logic-level vulnerabilities.

For example, a GCN can model smart contract functions and variables as **nodes**, and their interactions (e.g., calls, dependencies) as **edges**. Tools like **Surya** can be used to extract call graphs and adjacency matrices. The GCN then propagates information layer by layer using the update rule:

$$
H^{(l+1)} = \sigma(\tilde{A} H^{(l)} W^{(l)})
$$

Here, $\tilde{A}$ is the normalized adjacency matrix, $H^{(l)}$ is the feature matrix at layer $l$, $W^{(l)}$ is the weight matrix, and $\sigma$ is a non-linear activation function. This formulation allows the model to combine a node’s own features with those of its neighbors, capturing the **global context** of smart contract behavior.

### Role of Large Language Models (LLMs)

Large language models (LLMs) such as GPT are now being explored for vulnerability detection tasks. LLMs can perform:

- **Code summarization**
- **Code generation**
- **Test case generation**
- **Security audit simulations**

In vulnerability detection, LLMs are prompted as **security experts** and tasked with identifying and describing vulnerabilities in smart contract code. Instead of returning line numbers—which LLMs often miscount—they return the actual code snippets that contain vulnerabilities. These can then be mapped to line numbers with scripts.

In evaluations using datasets such as **SmartBugs Curated** and a set of **400 real-world labeled contracts**, LLMs demonstrated promising but still limited capabilities in detecting vulnerabilities in complex contracts. Their effectiveness improves with carefully designed prompts and role conditioning (e.g., defining the LLM as a smart contract security analyst and specifying DASP Top 10 categories for classification).

---

## Graph Convolutional Networks (GCN) for Detecting Unchecked External Call Vulnerabilities

In smart contracts, **functions and variables can be modeled as nodes**, while **their call relationships are represented as edges**. This naturally forms a _call graph_, a powerful representation for capturing interaction logic within and across contracts. The tool **Surya** plays a critical role in this process by converting Solidity source code into call graphs, which can then be used for feature extraction and vulnerability detection.

### From Source Code to Call Graph

**Surya** analyzes smart contract source code through **lexical analysis** (tokenizing keywords, operators, etc.) and **syntactic analysis** (constructing Abstract Syntax Trees, or ASTs). It identifies all function declarations and maps their interrelations. The output is a **call graph** (in DOT format), where each node represents a function and each edge represents a call between functions. This graph effectively captures both internal and cross-contract interactions.

### Graph Feature Extraction

Once the call graph is generated, the next step is **feature extraction** for both nodes and edges:

- **Nodes** represent functions and are embedded into a vector space based on their names and functionality. Functions involved in external calls (e.g., transfers, approvals, withdrawals) are especially important.
- **Edges** represent call relationships and are annotated with features such as whether the call is external or whether return values are checked.

This results in:

- A **node feature matrix** $H^{(0)}$, where each row is a feature vector for a function.
- An **adjacency matrix** $A$, which encodes the connectivity structure of the call graph.
- An **adjacency dictionary** to facilitate normalization and graph operations.

### GCN-Based Information Propagation

Traditional sequence models like **RNNs or LSTMs** are limited in their ability to model graph-structured data, often requiring the graph to be flattened into sequences, which loses structural information. In contrast, **Graph Convolutional Networks (GCNs)** excel in preserving and utilizing this structure.

GCNs update node features by aggregating information from neighboring nodes using the propagation rule:

$$
H^{(l+1)} = \sigma(\tilde{A} H^{(l)} W^{(l)})
$$

Where:

- $H^{(l)}$ is the node feature matrix at layer $l$
- $\tilde{A}$ is the normalized adjacency matrix
- $W^{(l)}$ is the trainable weight matrix at layer $l$
- $\sigma$ is a non-linear activation function (typically ReLU)

At each layer, a node's representation incorporates its own features and those of its neighbors. This multi-layer process enables the model to capture **global context and deep semantic dependencies**, which is crucial for detecting **interaction vulnerabilities**, especially those involving **multi-contract call chains**.

### Edge Prediction

To infer potential call relationships between functions that may not be explicitly defined, an **edge prediction module** is used. It predicts the presence and characteristics of an edge between two nodes $x_i$ and $x_j$ using:

$$
y_{ij} = \exp \left( 0.5 \times \left( f_{edge}(x_i, x_j) + f_{edge}(x_j, x_i) \right) \right)
$$

Here, $f_{edge}$ is a feed-forward neural network composed of a linear transformation and batch normalization, which outputs the likelihood or strength of a call relationship. The predicted edges are integrated into the adjacency matrix to form a symmetric graph:

$$
A = A + A^\top
$$

### Node Clustering

After GCN processing, **node clustering** is applied to group semantically similar functions. The similarity between node $x_{ij}$ and cluster center $c_k$ is computed as:

$$
d_{ijk} = \sum_{c=1}^{C} (x_{ijc} - c_{kc})^2
$$

The node is assigned to the cluster with the minimum distance:

$$
a_{ij} = \arg\min_k d_{ijk}
$$

This grouping helps abstract and organize complex function interactions, supporting the identification of vulnerability patterns.

### Conformer Block for Enhanced Representation

To further enrich feature learning, a **Conformer Block** is introduced. It integrates several neural mechanisms to capture both local and global patterns in the call graph:

- **Feedforward Network (FFN):** Two linear layers with GELU activation and dropout for regularization.
- **Multi-Head Self-Attention:** Learns long-range dependencies between nodes using:

$$
\text{Attention}(Q, K, V) = \text{softmax}\left( \frac{QK^\top}{\sqrt{d_k}} \right) V
$$

- **Scale Layer:** Stabilizes gradient flow and accelerates convergence.
- **Pre-Normalization:** Ensures consistent input distributions across layers.

### Convolutional Layers and Graph Connectivity

Convolutional layers are used to capture **local spatiotemporal features** of the graph. Using an enhanced adjacency matrix:

$$
\hat{A} = I + A
$$

(where $I$ is the identity matrix), the node features are updated using:

$$
X_{out} = \sum_{r=0}^{R-1} (X \cdot (I + adj_{sq} \cdot A_r)) \cdot \text{mask}
$$

The convolutional update in GCN is again defined as:

$$
H^{(l+1)} = \sigma(\tilde{A} H^{(l)} W^{(l)})
$$

This process enhances the model’s ability to capture both **structural patterns** and **semantic dependencies** across the graph.

### Overall Detection Pipeline

The end-to-end process for detecting **unchecked external call vulnerabilities** using GCN is summarized as follows:

- **Input**: Solidity source code files.
- **Call Graph Construction**: Use Surya to generate a DOT-format call graph.
- **Graph Feature Extraction**: Parse the DOT file to build the node feature matrix, adjacency matrix, and edge labels.
- **Edge Prediction**: Enhance the call graph by predicting missing or weakly defined call edges.
- **GCN Processing**: Learn node representations incorporating local and global context.
- **Node Clustering**: Organize semantically similar functions to abstract interaction patterns.
- **Conformer Module**: Apply attention and convolutional mechanisms for deeper semantic understanding.
- **Classification**: Pass the final representation through pooling and fully connected layers to classify whether vulnerabilities exist.

<details><summary>Code</summary>

```text
Algorithm 1: Identifying DApp vulnerabilities from source code

  Require: source files
  initialize a Graph and Feature G, F;
  for each file ∈ source files do
      (nodes, edges) = surya(file);
      G.append(edges, nodes);
  end for
  for each g ∈ G do
      (gLabel, (Ns, Ne, type), FCnames) = GraphInfo(g);
      Feature = convert2adj(gLabel, (Ns, Ne, type), FCnames);
      F.append(Feature);
  end for
  B, N, C = loaddata(F)
  for each b ∈ B do
      node_pair = find_node_pair(b);
      x_cat = concatenate_feature(node_pair);
      y = edge_pred(x_cat);
  end for
  Y = construct_adj_matrix(y);
  data = combine(Y, F[1]);
  x = gcn(data)[0];
  x = dropout(x);
  x = cluster_layer(x);
  x = gcn(x);
  x = x * mask.unsqueeze(-1);
  x = conformer_block(x);
  x = dropout(x);
  x = max_pooling(x);
  x = fullconnect(x);
  return x

```

</details>

---

## 2. Bytecode-Based Detection and Function Interface Inference

While smart contract vulnerability detection is essential to blockchain security, **most existing tools rely on access to source code**. However, only about **2% of Ethereum contracts are open source**, meaning that the vast majority of contracts are available only in **bytecode format**. This limitation presents a significant challenge for traditional tools, which typically focus solely on **execution logic** while overlooking the **function interface** — i.e., how external users or contracts interact with the contract.

Function interfaces, described by an Application Binary Interface (ABI), define the external callable methods of a smart contract. Many deployed contracts do **not have ABI information publicly available**, making it difficult to understand or predict how the contract is meant to be interacted with. To address this, researchers have proposed **automatically inferring function signatures from bytecode**, enabling vulnerability detection even in the absence of source code.

The proposed approach starts by building a **Control Flow Graph (CFG)** from the contract bytecode and then uses machine learning models to **learn the structure and patterns of function signatures** directly from the bytecode.

### Extracting Opcode

The first step is to extract two types of opcode sequences from the bytecode:

- **Raw Opcode Sequences**: Directly disassembled sequences from bytecode.
- **SSA-Formatted Opcode Sequences**: A more refined intermediate representation using **Static Single Assignment (SSA)** form.

SSA-form removes stack-specific operations (e.g., `PUSH`, `POP`, `SWAP`, `DUP`) and retains only the **semantic essence** of the instructions, making the code easier to analyze. This representation focuses on logical execution rather than low-level operational noise.

<details><summary>Code</summary>

```text

Algorithm 1: Functions Context and Ids Acquisition

  input: A deployed bytecode smart contract bc
  output: two global map: functions context OpSeq, functions hashes Ids

  BasicBlocks, eb ← CFG.countBasicBlocks(EVMAsm(bc));
  pushValue, prePushValue ← None;
  Function getFuncInfo(block, entry):
      foreach instruction i of the block.ins do Ops ← i;
      if entry then
          if end of block compatible with JUMPI then
              Assert length of block.ins > 2;
              dest ← oprand of block.ins[-2];
              OpSeq[dest] ← getFuncInfo(BasicBlocks[dest], false);
              Ids[dest] ← None;
              return Ops;
      for i in block.ins do
          if i compatible with PUSHs then
              prePushValue ← pushValue;
              pushValue ← oprand of i;
      if end of block compatible with JUMPI then
          if prePushValue then
              fnAddr, fnId ← pushValue, prePushValue;
          else
              fnAddr, fnId ← None;
          if fnAddr compatible with BasicBlocks then
              OpSeq[fnAddr] ← getFuncInfo(BasicBlocks[fnAddr], false);
              Ids[fnAddr] ← fnId;
      if end of block compatible with JUMPI then
          dest ← ((endPc ep of block) + 1);
          OpSeq[dest] ← getFuncInfo(BasicBlocks[dest], false);
      return Ops;
  foreach entryBlock address eb of the BasicBlocks do
      OpSeq[eb] ← getFuncInfo(BasicBlocks[eb], True);

```

</details>

### Embedding Semantics and Interface Features

Both the semantic opcode information and the inferred function interface characteristics are mapped into a shared **latent space**, where they are fused into a **composite contract feature vector**. A neural network equipped with a **decoder** and an **attention mechanism** is then trained to **classify vulnerabilities** based on these combined features.

The function signature recovery process uses both:

- **Known ABI/function signatures** from public databases, and
- **Predicted signatures** generated by a trained model that learns to infer function names, parameters, and return types from patterns in the bytecode.

This enables the system to generate a **synthetic ABI description** for a contract, which can then be used in vulnerability detection even when the actual ABI is unavailable.

### Multi-Label Classification of Function Signatures

Unlike traditional single-label classification tasks (e.g., identifying whether an image is a cat, dog, or bird), function signature inference requires **multi-label classification (MLC)**. For example, a single function may have multiple parameters of types such as `uint256`, `address`, and `bool`.

Given a label set $L = \{l_1, l_2, ..., l_m\}$, the goal is to infer an optimal subset $y$ for a given opcode sequence $x = \{w_1, w_2, ..., w_n\}$. This is formulated as:

$$
P(y|x) = \prod_{i=1}^{n} p(y_i | y_1, ..., y_{i-1}, x)
$$

This sequential dependency models the fact that **the prediction of each label (e.g., parameter type) depends on previous labels and the original input sequence** — much like real function parameters, where one parameter may influence the next.

### Neural Sequence Modeling with Attention

#### Input Processing

All tokens (opcodes, numbers, symbols) are collected into a **vocabulary**. Each token is converted to an ID and transformed into a **one-hot vector**, which is then passed through an **embedding layer** to obtain dense semantic vectors:

$$
c = \{ e_1, e_2, ..., e_n \}
$$

Each $e_i$ is an embedding of a specific opcode or token.

#### Bi-directional LSTM Encoder

A **Bi-directional LSTM** (BiLSTM) is used to encode contextual relationships by processing the sequence in both forward and backward directions:

```math
h_i = [\text{LSTM}_{\text{forward}}(h_{i-1}, c_i); \ \text{LSTM}_{\text{backward}}(h_{i-1}, c_i)]
```

This captures both past and future context, which is critical for semantic understanding.

#### Attention-Based Decoder

During the decoding process, an **attention mechanism** allows the model to focus on the most relevant parts of the encoded sequence when predicting each label:

$$
w_{ti} = \text{softmax}(W_a^T \tanh(U_a s_t + O_a h_i))
$$

Where:

- $s_t$ is the current decoder state,
- $h_i$ is the representation of the $i$-th token,
- $W_a, U_a, O_a$ are trainable weight matrices.

The attention-weighted context vector is:

$$
v_t = \sum_{i=1}^n w_{ti} h_i
$$

The decoder state is updated using the previous predicted label $g(y_{t-1})$ and the previous context vector $v_{t-1}$:

$$
s_t = \text{LSTM}(s_{t-1}, [g(y_{t-1}); v_{t-1}])
$$

### Handling Class Imbalance with Focal Loss

Since some parameter types (e.g., `bytes32`) are rare, **class imbalance** is a serious issue. A **focal loss function** is used to emphasize hard-to-classify samples:

$$
\text{loss} = -\alpha (1 - p_t)^\gamma \log(p_t)
$$

Where:

- $p_t$ is the probability of correctly predicting the sample,
- $\gamma$ controls the focus on hard samples (usually set to 2),
- $\alpha$ balances the influence of each class.

An adaptive version further adjusts $\alpha$ based on the frequency of each label:

$$
\alpha_i = \frac{\sum_{j=0}^{T} n_j}{n_i}
$$

Where $n_i$ is the number of samples in class $i$.

### Path Sequence Construction and Feature Fusion

Semantic information and function interface characteristics are **concatenated into path sequences**, where each function's signature and attributes are jointly embedded to provide a **comprehensive functional representation**.

Training leverages this combined representation to learn vulnerability patterns from large datasets. The model is enhanced with **MS-CAM (Multi-Scale Channel Attention Module)**, which merges:

- **Local features**: Captured via convolutions that focus on individual function details.

$$
\text{feature}_l = N(\text{Conv}_2(\zeta(N(\text{Conv}_1(G)))))
$$

- **Global features**: Obtained via global average pooling (GAP).

$$
\text{feature}_g = GAP(\text{feature}_l)
$$

The fused representation is:

$$
\text{output} = G \otimes (\sigma(\text{feature}_g \oplus \text{feature}_l))
$$

This allows the model to consider **both fine-grained function-level behavior and overall contract patterns**.

Finally, the **semantic features** $h^{(1)}, ..., h^{(k)}$ are concatenated with **function signature features** $G$ into:

$$
h_x = (h^{(1)}, ..., h^{(k)}; G)
$$

This unified vector is passed through an **attention-based RNN decoder**, which outputs a sequence of vulnerability predictions. A **masking mechanism** ensures each label is unique and not repeated across time steps.

---

## 3. Cross-Chain Bridge Security: Graph-Based Attack Detection

Cross-chain bridges differ fundamentally from typical decentralized applications (DApps). While regular DApps primarily rely on on-chain logic, cross-chain bridges require **coordination between on-chain and off-chain components** —such as validator nodes or external relayer services. This significantly expands the attack surface. Moreover, cross-chain bridges often manage large amounts of assets, and their complex logic involving multiple stages, contracts, and chains makes them particularly attractive targets for attackers.

Since 2021, cross-chain bridge attacks have resulted in over $5 billion in losses, with logic-based vulnerabilities causing far more damage than non-logic attacks. The three largest exploits listed in the Rekt database all involve cross-chain bridges: Ronin Network (\$624M), Poly Network (\$611M), and BNB Bridge (\$586M). These attacks typically exploit call structure anomalies —the way function calls are made during an attack deviates significantly from normal transactions. Vulnerabilities such as faulty message verification, signature bypasses, or miscalculated token amounts can lead the bridge contract to erroneously release funds.

A cross-chain bridge is a decentralized application that connects multiple blockchain networks, enabling the **transfer of assets and data across chains**. Common implementations include:

- **Atomic Swaps**: Direct peer-to-peer exchanges between chains without intermediaries—very secure but limited in functionality.
- **Sidechains**: Dependent blockchains that process transactions before relaying them back to the main chain—simpler but less scalable.
- **Relay Chains**: Independent third-party chains that validate and forward transactions between chains—highly functional but complex and risky.

A complete cross-chain transaction generally involves **three stages**:

#### Source Chain Phase

- **User Request**: The user initiates a deposit transaction to the bridge’s routing smart contract.
- **Routing**: The router forwards the request to the relevant token contract.
- **Asset Lock**: The token contract locks the assets in a vault and emits a lock event.
- **Event Verification**: The router verifies the lock event and emits a deposit event.

#### Off-Chain Phase

- **Message Relay**: The source chain event is relayed to an off-chain system.
- **Verification**: The off-chain system verifies the authenticity of the source event and sends the information to the target chain.
- **Verification Methods**: Can be native, local, or external, depending on the bridge design.

#### Target Chain Phase

- **Request Forwarding**: The router on the target chain forwards the verified request to the token contract.
- **Asset Release**: The token contract either releases locked funds or mints new tokens, emitting a release event.
- **Finalization**: The router receives the release event and emits a corresponding withdraw event.

### Types of Attacks

Attacks on cross-chain bridges fall into two main categories:

- **Business Logic Attacks**: Exploits targeting core processes, such as verification bypass, consensus manipulation, or double-minting.
- **Non-Logic Attacks**: Include private key compromises, flash loan exploits, or rug pulls by malicious developers.

### Graph-Based Modeling and Detection

To detect such attacks, researchers propose modeling cross-chain transactions as **graph structures**, where:

- **Nodes** represent contracts, accounts, transaction steps, or events.
- **Edges** indicate relationships such as function calls, fund transfers, or event emissions.

Using **two-stage graph mining**, the system identifies anomalies in both **global** and **local** structures:

#### The Problem with Traditional Detection

Conventional detection tools rely on **matched transaction pairs** (i.e., deposit on the source chain and corresponding withdrawal on the target chain). However, most real-world attacks—especially those exploiting off-chain authentication—**lack these complete transaction pairs**, making traditional methods ineffective. Fortunately, even isolated malicious transactions show distinctive structural patterns that differ significantly from benign ones.

### The xTEG: Cross-Chain Transaction Execution Graph

Each cross-chain transaction is transformed into an **xTEG (Cross-chain Transaction Execution Graph)**:

- **Vertices**: Include EOAs (externally owned accounts), contract addresses, contract functions, and emitted events.
- **Edges**: Represent execution operations, including:

  - **CALL / STATICCALL / DELEGATECALL / CALLCODE** – function calls
  - **CREATE / CREATE2** – contract creation
  - **SELFDESTRUCT** – contract destruction
  - **EMIT** – event triggers

xTEG captures both the **execution trace** and **event-driven interactions** involved in a cross-chain transaction, representing its full complexity.

### Global Graph Mining

This phase captures the **overall structure** of a transaction:

- **Graph2vec**: An unsupervised graph embedding technique inspired by Doc2vec. The graph is treated like a document, and neighborhood subgraphs act as "words".
- **Weisfeiler-Lehman (WL) Subtree Kernel**: Generates canonical labels for subgraphs.
- **Skip-Gram Model**: Learns a 16-dimensional graph embedding vector.

Additionally, several **statistical features** are calculated:

- Number of nodes and edges
- Number of event logs
- Graph density: $D = \frac{2|E|}{|V|(|V|-1)}$
- A binary indicator: deposit or withdrawal transaction

**Output**: A 21-dimensional global feature vector $F_{glo} \in \mathbb{R}^{21}$ (16 embedding + 4 stats + 1 label).

### Local Graph Mining

Global features alone are insufficient to pinpoint specific **attack types**. Local mining focuses on:

- **Network Motifs**: Small, recurring subgraph patterns revealing structural anomalies.
- 16 distinct directed motifs ($M_1 \sim M_{16}$) are counted for their frequency in the xTEG.

**Output**: A 16-dimensional local feature vector $F_{loc} \in \mathbb{R}^{16}$.

### Classification Model

The final **combined feature vector**:

$$
F = F_{glo} || F_{loc} \in \mathbb{R}^{37}
$$

is fed into a multi-class classification model (e.g., decision tree, random forest, neural net) to predict whether the transaction is **benign or malicious**, and if malicious, what **type of attack** it belongs to.

<details><summary>Code</summary>

```text

  Algorithm 1
  Input: Transaction hash tx
  Output: Transaction category c

  trace ← getTrace(tx)
  log ← getLog(tx)
  xTEG ← buildXTEG(trace)
  global_feature ← Concat(graph2vec(xTEG), statistic(xTEG), log)
  local_feature ← motif_count(xTEG)
  features ← Concat(global_feature, local_feature)
  c ← classifier(features)
  return c

```

</details>

---

## Other Safer Cross-Chain Swaps: Formal Verification, Value Equivalence, and AI-Powered Vulnerability Detection

Cross-chain bridges and decentralized applications (DApps) are at the heart of modern blockchain ecosystems—but they're also among the most vulnerable components. As the complexity of smart contract interactions grows, ensuring security and efficiency in cross-chain systems becomes a critical challenge. Therefore, we explore emerging solutions in formal verification, innovative AMM design, and AI-driven vulnerability detection that offer a promising path toward secure, bridge-free cross-chain swaps.

### Formal Verification for Cross-Chain Bridges

**Formal verification** uses mathematical methods to rigorously prove that a smart contract behaves exactly as intended. Rather than relying solely on testing or auditing, it provides a "mathematical proof" that the system always adheres to specified safety and liveness properties.

The process typically includes:

- **Specification**: Clearly defining how the contract should behave and what outcomes are expected.
- **Mathematical modeling**: Abstracting the contract into a formal model that captures all possible states and transitions.
- **Proof construction**: Demonstrating that the system maintains the desired properties in all possible scenarios.

Finite State Machines (FSMs) are often used in this process due to their well-defined structure:

- **Finite states** allow exhaustive exploration.
- **Clear transitions** represent function calls and events.
- **Compatibility** with model checkers (e.g., Promela for SPIN, NuSMV, UPPAAL) makes them ideal for formal verification.

By applying formal verification to cross-chain bridge logic, developers can eliminate entire classes of vulnerabilities and ensure critical properties like atomicity and consistency across chains.

---

## Beyond Bridges: Bridge-Free Cross-Chain Swaps

Traditional cross-chain swaps depend on **bridges** and **intermediary tokens** (e.g., wrapped tokens), which introduce security risks, high gas fees, and fragmented liquidity. A new generation of **bridge-free cross-chain AMMs** eliminates these inefficiencies through mathematical and protocol-level innovation.

### The Problems with Traditional AMMs

- **Bilateral liquidity pools** fragment liquidity across chains.
- **Bi-state dependency**: Price and liquidity are tied to two assets simultaneously, increasing volatility and risk.
- **Wrapped tokens and bridges** expose users to additional attack surfaces.

### A New Paradigm: Value-Based Swaps without Bridges

A novel AMM mechanism solves these issues by introducing:

- **Single-sided state dependency**: Each chain computes its local transaction state independently.
- **Global value invariance**: The swap protocol ensures that the total value across chains remains constant.

#### Core Invariant: Value Equivalence

The protocol's invariant guarantees that:

> **The total value change across all pools remains zero.** $\sum_{x=a}^{z} \int_{x_0}^{x_n} P(x)dx = 0$

This means:

- The value added to one pool equals the value removed from another.
- Cross-chain atomicity is achieved without relying on synchronized state or bridges.

#### Swap Equation

The **value equivalence** for cross-chain swaps is formalized as:

> $\int_{i_n}^{i_{n+1}} P_x \, dx = - \int_{j_n}^{j_{n+1}} P_x \, dx$

Where:

- The left side represents the input asset value change on Chain A.
- The right side represents the output asset value on Chain B.
- Both are derived locally, while **relayed messages** ensure they match in value.

#### Liquidity Management Innovation

In contrast to traditional AMMs, where liquidity operations modify the pool invariant (and thus require costly synchronization), the new system introduces:

- **Custom LP tokens** that represent a user’s share of the pool value.
- **Invariant-preserving LP actions**: Adding/removing liquidity no longer affects the core value invariant directly.
- Initial supply of LP tokens is calculated using a **geometric mean**:

> $Initial = \sqrt[n]{a_0 \times b_0 \times c_0 \times \ldots \times z_0}$

This approach keeps the AMM scalable and suitable for multi-chain environments without coordination overhead.

#### Price Curve Optimization for Stablecoins

Traditional AMM price functions often lead to high slippage for stablecoin pairs. To address this, a new **Witch of Agnesi**-inspired pricing curve improves trade efficiency:

> $P_x = \frac{w}{x} \times \left(1 - \frac{A^2}{(x - x_{stable})^2 + A^2}\right) + \frac{w}{x_{stable}} \times \frac{A^2}{(x - x_{stable})^2 + A^2}$

Where:

- $x_{stable}$ is the ideal stablecoin balance.
- $A$ is an amplification factor controlling the curve's flatness.
- The result is a **bell-shaped** curve that reduces slippage near equilibrium points—ideal for USDC, DAI, etc.

#### Smarter Vulnerability Detection: AI + Multi-Modal Semantics

Most current smart contract vulnerability detection tools rely on static or symbolic analysis, often ignoring the full semantic context. This leads to:

- **High false positive/negative rates**
- **Limited understanding** of cross-contract or bytecode-level behavior

Emerging AI-based approaches leverage **multi-modal inputs**:

- **Source code**
- **Bytecode**
- **Opcode sequences**

Combining different deep learning models can address the full complexity of smart contract behavior:

- **Sequential models** (e.g., Transformers, LSTMs) capture the execution flow.
- **Graph-based models** (e.g., GNNs) analyze control flow graphs and dependency graphs.

These hybrid architectures are particularly suited for detecting sophisticated attack patterns in **cross-chain bridges**, where multiple contracts and chains are involved.
