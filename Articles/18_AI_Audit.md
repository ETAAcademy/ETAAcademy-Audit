# ETAAcademy-Audit: 18. AI Audit

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>18 AI Audit</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>AI Audit</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# AI-Driven Frameworks for Smart Contract Vulnerability Detection and Auditing

The evolution of smart contract vulnerability detection technologies has progressed through several distinct phases. Initially, traditional program analysis methods such as **symbolic execution**—which explores potential execution paths by analyzing program paths to uncover vulnerabilities—were widely used. Alongside this, **pattern matching** techniques compared contract code against known vulnerability signatures, and **formal verification** employed manually defined compliance patterns to rigorously validate contract correctness.

Subsequently, the field entered the **machine learning era**, where **deep learning models** significantly improved detection generalization and accuracy. For example, attention-enhanced **Bi-LSTM** (Bidirectional Long Short-Term Memory) networks were applied to identify specific vulnerability patterns within smart contracts.

The next advancement involved **Graph Neural Networks (GNNs)**, which captured structural information inherent in smart contracts by modeling their code as graphs (such as control flow or call graphs), enabling more context-aware vulnerability detection. This was complemented by the rise of **pretrained models** like Peculiar, which demonstrated strong generalization capabilities, and techniques such as **prompt tuning**, which leveraged prompt engineering to adapt pretrained language models to vulnerability detection tasks. Furthermore, **cross-modality learning** approaches emerged, integrating bytecode and source code information to enhance detection performance.

Currently, the state-of-the-art in smart contract security centers on **large language models (LLMs)**, with specialized frameworks such as GPTScan, FTSmartAudit, and iAudit leading through three key approaches: LLM-based auditing using structured prompting and five-stage pipelines with Chain-of-Thought reasoning; enhanced decompilation via chunked processing with domain-specific prompts for platforms lacking source code; and specialized model training using CPT, SFT, and DPO strategies to address semantic misinterpretation through expert-preferred explanations. Complementary research enhances Cyber Threat Intelligence (CTI) for real-time, verifiable security enforcement via smart contracts.

In parallel, **reinforcement learning (RL)** has been increasingly applied within software engineering domains, including API testing (e.g., ARAT-RL), code completion (such as RLCoder and IRCoCo), and code review processes (e.g., CodeMentor, which integrates LLMs with RLHF—reinforcement learning from human feedback). These explorations highlight the growing intersection of advanced AI techniques and smart contract security.

---

## 1. Prompting and Agentic Workflow for Reliable Smart Contract Auditing

Traditional tools such as static analysis and symbolic execution have been valuable in the early stages of smart contract auditing. However, as smart contracts become more complex and vulnerabilities increasingly subtle—often involving intricate inter-contract interactions—these conventional methods struggle to keep pace. Large Language Models (LLMs) have emerged as promising tools that can bridge the gap between **design intent** and **actual code implementation**, enabling the detection of complex and novel vulnerabilities. Nonetheless, LLMs inherently suffer from output uncertainty and immature verification mechanisms. Therefore, a critical research challenge lies in building **trustworthy, verifiable, and structured LLM-based auditing systems**.

Simply invoking an LLM to produce results is insufficient for the high-complexity task of smart contract auditing. To harness the reasoning capabilities of LLMs more reliably, researchers have proposed the following approaches:

### Structured Prompting Strategies

Structured prompting methods improve reasoning quality and consistency by guiding LLMs through more interpretable and stable thought processes. Common examples include:

- **Chain-of-Thought (CoT)** prompting, which encourages the model to reason step-by-step rather than outputting a final answer directly.
- **Few-shot learning**, which provides multiple example prompts to help the model mimic desired reasoning patterns.
- **Tree of Thought (ToT)**, which organizes the reasoning process as a tree structure, exploring multiple reasoning paths.
- **Graph of Thought (GoT)**, an extension of ToT, that allows for more complex logical structures representing concurrency and interaction.

### Autonomous Agent Systems

These systems empower LLMs to autonomously learn, plan, and act by perceiving their environment, making decisions, and executing tasks. Such agents can independently plan audit workflows, invoke external tools, and adjust strategies dynamically. Designing these systems requires careful specification of action spaces and decision-making processes. A typical example is an LLM-enabled agent that autonomously researches, writes code, tests it, and revises its approach based on feedback.

### Agentic Workflows

Agentic workflows combine human-designed task decomposition and control with collaborative multi-LLM cooperation to handle complex analyses. Compared to fully autonomous agents, agentic workflows are better suited for **high-reliability and high-security tasks** such as smart contract auditing. They decompose a large auditing task into a sequence of ordered subtasks, which can be static or dynamically generated. Each subtask is assigned to an LLM playing a specific role—e.g., “vulnerability identifier,” “logical analyst,” or “risk summarizer.” This structure enables intermediate review, feedback, and iterative improvement. External tools like static analyzers or knowledge bases may be integrated to bolster analysis quality.

Additionally, agentic workflows incorporate confidence scoring mechanisms: only findings above a predefined confidence threshold are accepted, preventing low-quality or uncertain results from polluting the final audit. The filtered high-confidence findings are aggregated into a verified discovery set, emphasizing human design and control over the entire workflow. This approach suits scenarios demanding rigor, traceability, and verifiability, such as vulnerability detection, legal review, and medical decision-making.

---

### Five Stages of a Comprehensive Smart Contract Audit Pipeline

#### Stage 1: Understanding

Analyze contract code and documentation to build foundational knowledge. The LLM develops an initial understanding (`s1`) that includes:

- Business logic: What is the contract’s purpose? Which modules are involved (e.g., ERC20 token, decentralized exchange, DAO)?
- Contract roles: Key modules and interfaces (token logic, governance, upgrade mechanisms, security features).
- Asset exposure: Whether funds, permissions, or sensitive data are involved, and which functions might be attack vectors.
- Implementation structure: Are main functions, data structures, and inheritance relationships well-designed? Are there potential architectural vulnerabilities?

For example, for an ERC-721 NFT trading platform, the LLM identifies mint, burn, transfer functions; NFT pricing and trading logic; and sensitive token-related functions needing careful audit. This stage can also incorporate static analysis outputs (e.g., Slither’s warnings about reentrancy or uninitialized variables) to provide concrete data for enhanced understanding.

#### Stage 2: Adaptive Audit Planning

Once the initial understanding (`s1`) is established, the LLM generates a customized audit plan (`s2`) via prompt `PA2`. This plan decomposes the audit into subtasks (e.g., reentrancy checks, access control verification, mint/burn logic review, governance assessment), prioritized by risk levels. Complex functions may be recursively decomposed to ensure granular analysis.

<details><summary>Code</summary>

```code

Algorithm 1: Dynamic Smart Contract Auditing Workflow

  Require: Smart contract code c, Contextual documents Dctx, LLM M,
           Set of specialized prompts P = {PA1, PA2, {PiA3e}, {PiA3v}, PA4}
  Ensure: Comprehensive audit report R

  // A1: Context-Aware Initial Analysis
  1: s1 ← M(PA1(c, Dctx))                           ▷ Generate initial analysis and understanding

  // A2: Adaptive Audit Planning
  2: s2 ← M(PA2(s1, c))                             ▷ Create a prioritized audit plan (list of sub-tasks t)
  3: t = {t1, t2, ..., tn} ← extract_subtasks(s2)

  // A3: Multi-faceted Vulnerability Execution and Validation
  4: Sv = ∅                                         ▷ Initialize set of validated findings for s3
  5: for each sub-task tj ∈ t do
  6:     ej ← M(Pj_A3e(tj, c, s1))                  ▷ Preliminary execution for sub-task tj
  7:     (vj, confidencej) ← M(Pj_A3v(ej, c))       ▷ Validate ej; output validated finding vj and confidence
  8:     if confidencej ≥ THRESHOLD_CONFIDENCE then
  9:         Sv ← Sv ∪ {vj}
  10:    end if
  11: end for
  12: s3 ← Sv                                       ▷ Aggregated validated findings from execution stage

  // A4: Cross-Cutting Findings Synthesis
  13: s4 ← M(PA4(s3))                               ▷ Correlate, prioritize, assign severity to findings in s3

  // A5: Comprehensive Report Generation
  14: s5 ← {s1, s2, s4}
  15: return R = s5

```

</details>

An **Iterative Prompt Optimization Framework** is employed during planning to evolve and optimize the prompt itself using an evolutionary algorithm. The goal is to find an optimal prompt $\rho^{*}$ that maximizes audit task performance while balancing prompt complexity:

$$
\rho^* = \arg\max_\rho \left( \frac{1}{|D_{\text{train}}|} \sum_{(c,A) \in D_{\text{train}}} f(\rho, c, A) - \lambda \cdot \text{Complexity}(\rho) \right)
$$

where:

- $c$ is contract code,
- $A$ is the expert-provided target output (audit plan),
- $f(\rho, c, A)$ is a scoring function combining semantic alignment and LLM confidence,
- $\lambda$ controls the penalty on prompt complexity.

The scoring function $f$ combines:

- Execution alignment $f_{\text{exec}}$: semantic similarity to expert outputs,
- Confidence $f_{\text{log}}$: LLM’s internal log probability-based confidence over generated tokens.

<details><summary>Code</summary>

```code
Algorithm 2: Iterative Prompt Optimization Algorithm

  Require: Training dataset Dtrain, validation set Dval;
           Max generations Tmax, population size PS, elite count ke, offspring count ko = PS - ke;
           Replay buffer capacity m, mutation parameters (τmax, β)
           Convergence criteria: min fitness improvement δfitness, stable generations Nstable, min diversity Dmin
  Ensure: Optimal instruction ρ*

  Phase 1: Initialization:
  1: Generate initial population U0 = {ρ1, ρ2, ..., ρPS} using mutation of seed prompts
  2: Initialize replay buffer Breplay ← ∅
  3: Initialize f̄0(ρ) ← EvaluateInitialFitness(ρ, Dtrain) for each ρ ∈ U0
  4: t ← 1; generations_without_improvement ← 0

  Phase 2: Evolutionary Loop
  5: while t ≤ Tmax and generations_without_improvement < Nstable do
      // A. Mini-Batch Sampling
  6:     Sample mini-batch Bt ⊂ Dtrain of size nt = ⌈0.1|Dtrain| · (1 + t/T)⌉

      // B. Stochastic Fitness Evaluation for Population Ut-1
  7:     for all ρ ∈ Ut-1 do
  8:         if |Breplay| > 0 then
  9:             freplay(ρ) ← ε · (1/|Breplay|) ∑(ρ',f')∈Breplay sim(ρ, ρ') · f'
  10:        else
  11:            freplay(ρ) ← 0
  12:        end if
  13:        fbatch(ρ) ← (1/|Bt|) ∑(c,A)∈Bt f(ρ, c, A)
  14:        ft(ρ) ← fbatch(ρ) + freplay(ρ)
  15:    end for

      // C. Update Smoothed Fitness and Select Elites from Ut-1
  16:    for all ρ ∈ Ut-1 do
  17:        f̄t(ρ) ← α · f̄t-1(ρ) + (1 - α) · ft(ρ)
  18:    end for
  19:    Select ke elites Et based on f̄t(·)

      // D. Offspring Generation
  20:    Compute mutation temperature: τt ← τmax · e^(-βt)
  21:    Select parents Pparents from Ut-1 (e.g., using tournament selection, or use Et)
  22:    Ot ← ∅
  23:    for i = 1 to ko do                                    ▷ Generate ko = PS - ke offspring
  24:        ρparent ← SelectParent(Pparents)
  25:        ρoffspring ← Mutate(ρparent, τt, Bt)              ▷ Guided mutation using Bt
  26:        Ot ← Ot ∪ {ρoffspring}
  27:        f̄t-1(ρoffspring) ← f̄t-1(ρparent)                ▷ Initialize smoothed fitness for new offspring, or use parent's
  28:    end for

      // E. Form New Population for Next Generation
  29:    Ut ← Et ∪ Ot                                         ▷ New population of size PS = ke + ko

      // F. Update Replay Buffer
  30:    Update Breplay with top-m performers from Ut

      // G. Convergence Check
  31:    f̄best,t ← maxρ∈Ut f̄t(ρ)
  32:    if t > 1 and |f̄best,t - f̄best,t-1| < δfitness then
  33:        generations_without_improvement ← generations_without_improvement + 1
  34:    else
  35:        generations_without_improvement ← 0
  36:    end if
  37:    D(Ut) ← 1 - MeanPairwiseSimilarity(Ut)
  38:    t ← t + 1
  39: end while

  Phase 3: Final Selection
  40: Using Eq. 1 to select ρ* on a held-out validation set Dval
  41: return ρ*

```

</details>

Furthermore, integrating static analyzers like Slither into this pipeline allows cross-validation and helps reduce false positives from LLM outputs, producing a hybrid system of **structure-aware plus semantic-aware auditing**.

#### Stage 3-5: Execute, Integrate, and Report

- **Execution**: LLMs perform detailed security analyses on each subtask, outputting preliminary findings which are self-verified or rule-checked. Only high-confidence results proceed.
- **Integration**: Aggregation and classification of verified vulnerabilities to evaluate severity, impact scope, and relationships between issues, synthesizing a holistic security posture.
- **Reporting**: Producing structured, actionable audit reports tailored to developers and managers, highlighting prioritized findings and recommended mitigations.

These stages emphasize accuracy, risk prioritization, and clarity to realize practical, automated smart contract audits.

---

### Enhancing Smart Contract Decompilation with Large Language Models

Many blockchain platforms, such as Sui, lack publicly available source code for their smart contracts, posing significant challenges for security auditing. Existing decompilers, which aim to reconstruct source code from compiled bytecode, often produce output that is difficult to understand and cannot be directly recompiled. This limitation substantially hinders practical use.

Traditional decompilers face inherent constraints—they cannot fully recover the original developer’s code, as critical elements such as comments, variable names, and precise type information are typically lost during compilation. The resulting code is often obfuscated and less readable, impeding thorough program comprehension and security analysis.

To address these issues, AI-augmented solutions leverage Large Language Models (LLMs) to enhance decompilation outputs. LLMs have demonstrated strong capabilities in code understanding and generation. For example, LLM4Decompile achieves an impressive 80.49% re-execution rate on C language decompilation tasks by effectively using **prompt engineering** techniques to guide the decompilation process.

However, feeding an entire contract’s source code at once into an LLM yields suboptimal results. Due to LLMs’ limitations in processing long text sequences, this approach often leads to omissions, inaccurate or hallucinated code (e.g., fabricated functions or erroneous content), and incomplete annotations.

To overcome these challenges, a **chunked processing** strategy is adopted. The bytecode is split function-by-function, and each function’s bytecode segment is input separately into the LLM, which then generates the corresponding source code for that function. This modular approach improves accuracy and maintainability of the output.

The prompt design guiding the LLM’s decompilation output consists of three core components:

- **Domain Knowledge Input:** This section explains language-specific features relevant to the target smart contract language, such as syntax rules, variable mutations, and ownership management mechanisms. It also highlights common decompilation pitfalls to prompt the model to avoid or correct typical errors.

- **Do’s and Don’ts Instructions:** Explicit directives are provided to ensure the output adheres to strict formatting and coding standards. For example, the model is instructed to produce clean, well-structured code with clear variable names, complete type annotations, and no hallucinated or fabricated code segments.

- **Few-Shot Learning Examples:** A small set of input-output pairs, consisting of decompiled bytecode snippets and their corresponding original source code, is included to help the model learn the expected input-output mapping and language nuances.

By combining chunked function-level processing with carefully engineered prompts grounded in domain knowledge and exemplars, LLM-enhanced decompilation significantly improves the quality, readability, and utility of smart contract source recovery. This advancement facilitates more effective auditing, vulnerability detection, and code analysis in ecosystems where source code disclosure is limited.

---

## 2. Dataset: CPT, SFT, and DPO

General-purpose large language models (LLMs) often misinterpret critical semantics in the smart contract domain. For example, they may incorrectly identify nonexistent reentrancy vulnerabilities or fail to accurately understand the order of external calls and state updates within functions such as `buyInternal()`. Even with continued pre-training (CPT) on large-scale smart contract code for domain adaptation and supervised fine-tuning (SFT) on high-quality annotated explanations, these issues are not fully resolved. This highlights the need for **stronger task alignment techniques**.

Building upon CPT and SFT, a **three-stage training strategy** incorporates **Direct Preference Optimization (DPO)**, enabling the model to learn from expert preferences and generate higher-quality explanations. The core of this approach lies in constructing a high-quality dataset comprising:

- Four major vulnerability categories: reentrancy, time dependency, integer overflow, and delegatecall;
- Complex vulnerability types that are difficult for automated tools to detect;
- Precisely annotated vulnerability locations and multiple pairs of high- and low-quality explanations for DPO training.

#### Seven Complex Vulnerabilities Difficult for Automated Detection (MU)

These vulnerabilities are challenging for traditional tools to detect and require domain knowledge and contextual understanding:

| Abbreviation | Vulnerability Name               | Description                                                  |
| ------------ | -------------------------------- | ------------------------------------------------------------ |
| PO           | Price Oracle Manipulation        | Exploiting faulty or insecure price oracles                  |
| EA           | Erroneous Accounting             | Accounting errors caused by flawed business logic            |
| IU           | ID Uniqueness Violation          | Failure to correctly validate unique identifiers             |
| IS           | Inconsistent State Updates       | Conflicting state variable updates leading to logic errors   |
| PE           | Privilege Escalation             | Insufficient access control                                  |
| AV           | Atomicity Violation              | Broken atomicity causing concurrent interference             |
| CI           | Contract Implementation Specific | Vulnerabilities dependent on contract implementation details |

#### Four Key Stages: Data Construction, Continued Pre-Training, Supervised Fine-Tuning, and Preference Optimization

The training pipeline leverages open-source smart contract vulnerability datasets to train and evaluate the model’s explanation generation capabilities. Using LLaMA-3.1-8B as the base model, the approach focuses not only on vulnerability detection accuracy but also on explanation quality.

In the **data construction** phase, all Ethereum smart contract addresses with transaction records are collected via Google BigQuery. Corresponding source codes are crawled from Etherscan, with duplicates filtered out using Jaccard similarity metrics to retain only unique business logic. The supervised fine-tuning and preference optimization stages incorporate multiple high-quality annotated datasets such as SmartBugs and Code4rena, as well as extensive real-world contract samples collected from Etherscan, GitHub, and blogs spanning 2020 to 2024.

During the **SFT data curation** phase, high-scoring explanations undergo thorough review and correction based on detailed annotation guidelines. For example:

- For reentrancy (RE) vulnerabilities, reviewers analyze not only the existence of reentrancy but also the order of external calls, cross-contract reentrancy risks, inheritance structure, and the effectiveness of Reentrancy Guards.
- For delegatecall (DE) vulnerabilities, emphasis is placed on storage layout safety and access control in proxy contracts.

In cases of internal disagreement, third-party experts arbitrate to reach consensus. To mitigate bias further, additional external experts with practical development experience are engaged for supplementary review.

#### Preference Optimization (DPO) Dataset Construction

The final DPO stage constructs paired examples of **preferred** (high-quality) and **rejected** (lower-quality) explanations for each vulnerability. Preferred explanations are drawn from the SFT-reviewed versions. Rejected explanations are rewritten by experts based on lower-scoring LLM outputs, maintaining basic correctness but deliberately simplifying analytical depth. For instance, in the case of reentrancy, a low-quality explanation might merely note the presence of external calls without addressing call order, inheritance relations, or token transfer reentrancy risks.

This contrastive design enables the model to learn expert preferences and improve the generation of more nuanced and reasonable explanations.

#### DPO Loss Function

The DPO training optimizes the following loss function:

```math
L_{DPO}(\pi_\theta; \pi_{\text{ref}}) = - \mathbb{E}_{(x, y_w, y_l) \sim D} \left[ \log \sigma\left( \beta \log \frac{\pi_\theta(y_w|x)}{\pi_{\text{ref}}(y_w|x)} - \beta \log \frac{\pi_\theta(y_l|x)}{\pi_{\text{ref}}(y_l|x)} \right) \right]
```

Where:

- $x$ denotes the input smart contract code;
- $y_w$ is the expert-preferred high-quality analysis;
- $y_l$ is the less preferred, lower-quality analysis;
- $\pi_\theta$ is the policy model being optimized (initialized from the SFT-trained model);
- $\pi_{\text{ref}}$ is a fixed reference model;
- $\beta$ is a temperature hyperparameter controlling preference strength.

This formulation encourages the model to assign higher likelihood to preferred explanations over less preferred ones, aligning output quality with expert judgment.

---

## 3. Machine Learning, Deep Learning, Graph Neural Networks, and Transformers in Smart Contract Vulnerability Detection

Artificial intelligence methods for detecting vulnerabilities in smart contracts can be broadly categorized into four main approaches. Hybrid models that combine multiple methodologies have emerged to leverage the strengths of each paradigm, enhancing robustness and overall detection accuracy. Each approach presents trade-offs in scalability, interpretability, learning depth, and computational efficiency.

### Machine Learning (ML)

Traditional machine learning provides fast and interpretable models, primarily applied to anomaly detection and rule-based classification tasks. Its advantages include scalability and low computational cost, but it struggles with capturing complex semantic patterns inherent in smart contract logic. ML models often rely on handcrafted features extracted from contract metadata, opcode sequences, or execution traces. Examples include decision trees, support vector machines, and ensemble methods.

Dynamic analysis methods like **Sereum** use random forests—an ensemble of decision trees—to monitor smart contract states during execution for detecting reentrancy attacks. By aggregating multiple decision trees trained on different data subsets, random forests improve robustness and reduce overfitting compared to single decision trees.

Static analysis tools such as **MadMax** detect gas-related vulnerabilities (e.g., infinite loops, unbounded array iterations) in Ethereum smart contracts. MadMax models contract behavior as a finite state machine (FSM) $M = (S, S_0, T)$, where:

- $S$ is the set of symbolic program states,
- $S_0 \subseteq S$ is the set of initial states,
- $T: S \times \text{Opcode} \to S$ defines state transitions based on opcode execution.

Symbolic execution tracks symbolic variables representing storage and memory states, analyzing whether repeated transitions lead to gas exhaustion or non-terminating states. This approach enables predicting gas-related errors without on-chain execution.

While foundational, traditional ML and symbolic methods have limitations in understanding deep semantic dependencies, paving the way for advanced AI approaches.

### Deep Learning (DL)

Deep learning models excel at automatically learning hierarchical representations from raw code, obviating the need for manual feature engineering. Common architectures include Convolutional Neural Networks (CNNs), Recurrent Neural Networks (RNNs), particularly Long Short-Term Memory networks (LSTMs), and attention mechanisms.

- **Embedding-based Models** convert smart contract bytecode or opcode sequences into vector embeddings for downstream analysis. For example, **SmartEmbed** tokenizes bytecode sequences, applies word embeddings $e_i = E(t_i)$ mapping tokens $t_i$ to vectors in $\mathbb{R}^d$, and uses CNNs to capture local vulnerability patterns:

$$
c_j = \text{ReLU}(w \cdot E_{j:j+k-1} + b)
$$

The model proceeds through embedding, convolutional feature extraction, max pooling, and dense layers to classify vulnerabilities like stack misuse or unsafe opcode sequences.

- **Sequence Modeling with RNN/LSTM** is adept at capturing temporal dependencies in opcode sequences, important because certain instruction effects (e.g., write operations) manifest vulnerabilities only after several steps. LSTM’s gating mechanisms maintain and update long-term memory:

$$
h_t, c_t = \text{LSTM}(x_t, h_{t-1}, c_{t-1})
$$

where $x_t$ is the input token at time $t$, $h_t$ the hidden state, and $c_t$ the cell state.

Applications include delayed effect vulnerability detection, unprotected writes, complex control flow analysis, reentrancy patterns, uninitialized storage use, and inconsistent function calls.

- **Attention Mechanisms** provide selective focus on critical sequence elements, enhancing both prediction accuracy and interpretability. For instance, **VulnSniffer** combines bidirectional LSTM (Bi-LSTM) with attention to capture global context and highlight contract sections contributing most to vulnerability classification. The attention weights $\alpha_t$ are computed as:

$$
e_t = \text{score}(h_t), \quad \alpha_t = \frac{\exp(e_t)}{\sum_j \exp(e_j)}, \quad c = \sum_t \alpha_t h_t
$$

where $c$ is the context vector fed into a classifier.

### Graph Neural Networks (GNNs)

Unlike linear models, smart contracts possess rich structural information—multiple functions, control flow, storage variables, and call relationships—which naturally form graphs. GNNs are designed to operate on such graph-structured data, capturing global structure, local dependencies, and relational paths.

- **Control Flow Graph (CFG)-based GNNs**, such as **ContractGraph**, transform bytecode into CFGs where nodes represent basic blocks or instructions, and edges denote control jumps and logic flow. GNNs perform message passing to learn node representations, enabling detection of path-sensitive vulnerabilities such as unreachable code, infinite loops, and complex conditional bugs (e.g., call injection). The node update formula is:

$$
h_v^{(k)} = \sigma \left( W^{(k)} \cdot \text{AGG} \left( \{ h_u^{(k-1)} : u \in \mathcal{N}(v) \} \right) + b^{(k)} \right)
$$

where $\mathcal{N}(v)$ denotes neighbors of node $v$, $W^{(k)}$ learnable weights, and $\sigma$ an activation function.

- **Heterogeneous Graph GNNs**, exemplified by **ETH2Vec**, model multi-typed nodes (Contracts, Transactions, Addresses, StorageSlots) and multi-typed edges (Transfers, Calls, Emits, DelegateCalls), reflecting real blockchain interactions and ecosystem relationships. This approach suits detecting ecosystem-level vulnerabilities such as transaction ordering dependence (TOD), proxy contract misuse, and cross-contract issues by learning relation-aware embeddings.

- **Attention-based GNNs (GATs)**, like **SolGraph**, assign attention weights to neighbors rather than uniform aggregation. Attention coefficients $\alpha_{vu}$ quantify the importance of neighbor $u$ to node $v$:

$$
\alpha_{vu} = \text{softmax}(\text{LeakyReLU}(a^\top [W h_v \| W h_u]))
$$

leading to refined node updates:

$$
h_v^{(k)} = \sigma \left( \sum_{u \in \mathcal{N}(v)} \alpha_{vu} W h_u \right)
$$

This enhances the ability to detect subtle, context-dependent vulnerabilities and provides interpretable insights into critical paths causing bugs.

### Transformer Models

Transformers, such as **BERT**, are currently among the most powerful sequence modeling architectures, adept at capturing long-range dependencies and contextual information. Smart contracts, though structured code, share language-like semantics with function calls, variable passing, and logical conditions, making them well-suited to Transformer modeling.

- **SmartBERT** is a BERT variant fine-tuned on Solidity source code and EVM bytecode. It tokenizes code and applies self-attention to generate context-aware embeddings that capture complex inter-token relationships. For instance, it learns token patterns combined with contextual cues that indicate vulnerabilities like reentrancy or integer overflow.

The core attention mechanism computes:

$$
\text{Attention}(Q, K, V) = \text{softmax}\left(\frac{Q K^\top}{\sqrt{d_k}}\right) V
$$

where $Q, K, V$ are query, key, and value matrices derived from token embeddings. Multi-head attention enables the model to analyze the code from multiple perspectives simultaneously.

- **SolTrans** is a Solidity-specific Transformer, addressing the limitations of generic models pretrained on natural language corpora. It incorporates domain-specific tokens (e.g., `msg.sender`, `require()`, `fallback`, `modifier`, `gas`) to better capture Solidity’s unique semantics.

#### Hybrid Models

The limitations of single-method approaches (e.g., classical ML or pure GNN) have motivated the development of hybrid models that combine multiple techniques. Traditional ML models provide interpretability and speed but lack semantic depth. DL and GNNs capture structure and context but demand more resources and are less transparent. Hybrid approaches represent a mature stage in smart contract security, integrating diverse strengths to build robust, scalable, and practical vulnerability detection systems for real-world deployment.

---

## 4. Cyber Threat Intelligence

Cyber Threat Intelligence (CTI) involves the systematic collection and analysis of information related to existing and emerging cyberattacks. By aggregating data from multiple sources, CTI provides insights into attackers’ motivations, targets, and methodologies. This intelligence enables organizations to transition from reactive defense strategies to proactive security postures.

Existing research has explored various applications of CTI. For example, Samtani et al. leveraged deep learning techniques to correlate attack methods with known vulnerabilities, supporting risk management efforts; however, their approach did not extend to dynamic adjustment of security controls in response to evolving threats. Similarly, Kure and Islam demonstrated the value of CTI in risk management frameworks but stopped short of detailing mechanisms for dynamically adapting security measures based on intelligence inputs. Gautam et al. applied machine learning to classify data from hacker forums, enriching CTI data sources, though practical applications for real-time security control were not discussed. Serketzis et al. integrated CTI into digital forensics to enhance investigation readiness, yet this approach did not scale to active defense scenarios.

### Integrating AI, Blockchain, and Smart Contracts for Automated Security

Modern cybersecurity architectures increasingly combine Artificial Intelligence (AI), blockchain technology, and smart contracts to realize **automated internal policy compliance**, minimizing human intervention while enabling systems to **dynamically respond and adapt to evolving cyber threats**. Such integration enhances overall security resilience and operational agility.

At the core, an **AI decision-making module** interprets an organization's internal security policies and translates them into enforceable rules. This module ingests inputs from Cyber Threat Intelligence feeds and system feedback loops, continuously refining its decision algorithms through supervised, unsupervised, or reinforcement learning techniques. These models detect anomalies and deviations from expected policies, facilitating timely and informed responses.

Blockchain technology serves as a distributed ledger that immutably records all decisions and rule executions made by the AI module. The inherent **immutability** and **transparency** of blockchain provide trustworthy audit trails and operational accountability. Implementations may utilize private or consortium blockchains to balance data privacy with transaction throughput requirements.

Smart contracts execute the AI-defined rules autonomously. Upon satisfying predefined triggering conditions—such as detection of suspicious network packets by an endpoint—smart contracts automatically initiate corresponding control actions like tightening access permissions, generating alerts, or blocking network connections. This automation enables **real-time compliance enforcement** and **adaptive security controls**.

### Key Components of Automated Compliance and Adaptive Security

- **Policy Definition and Encoding**: Organizations codify security policies and compliance requirements into smart contracts deployed on a blockchain, ensuring that rules are executed precisely and remain tamper-proof.

- **Threat Intelligence Integration and Continuous Monitoring**: The AI layer continuously collects and analyzes CTI data to forecast potential attack vectors and recommend proactive mitigations, while monitoring system adherence to established security policies.

- **Real-time Enforcement and Automated Response**: Smart contracts respond immediately to AI-generated alerts by executing predefined security operations—such as revoking privileges, updating firewall configurations, tuning intrusion detection parameters, or isolating compromised nodes—thus enabling dynamic security posture adjustments and automation.

<details><summary>Code</summary>

```code

Algorithm 1: AI-Driven Compliance and Response Using Smart Contracts and Blockchain

  Input: CTI_feed (Cyber Threat Intelligence feed)
  Output: decision, action_result

  1: Initialize SecureBERT model M, Random Forest classifier RF
  2: Load policy database P
  3: Establish blockchain connection B to Hyperledger Fabric network
  4: function ProcessThreatIntelligence(CTI_feed)
  5:     CTI_data = CleanAndStructure(CTI_feed)
  6:     embeddings = M.Encode(CTI_data)
  7:     threat_class = RF.Predict(embeddings)
  8:     relevant_policies = P.Query(threat_class)
  9:     decision = DecisionTree(relevant_policies)
  10:    action_result = TriggerSmartContract(decision)
  11:    UpdateModel(action_result)
  12:    return decision, action_result
  13: function DecisionTree(policies)
  14:    if policies is empty then
  15:        return "No action required"
  16:    else if max(policies.severity) > THRESHOLD then
  17:        return "Immediate action required"
  18:    else
  19:        return "Standard mitigation required"
  20: function TriggerSmartContract(decision)
  21:    contract = B.GetContract("compliancecontract")
  22:    response = contract.SubmitTransaction("executeDecision", decision)
  23:    return response
  24: function UpdateModel(result)
  25:    if result is success then
  26:        RF.Improve()
  27:    else
  28:        RF.Adjust()
  29: return ProcessThreatIntelligence(CTI_feed)

```

</details>

[DPO](https://gitlab.com/programmer-of-nansijie/smart-llama-dpo)
[Mad](https://suigpt.tools/mad)
[MAD_github](https://github.com/EasonC13/MAD_WWW)
[MCPSecurity](https://github.com/MCP-Security/MCP-Artifact)
[Audit Wizard](https://medium.com/@JohnnyTime/smart-contract-auditors-vs-ai-audit-wizard-overview-3b399627ea0c)
[ContractauditorAI](https://github.com/techaddict0x/contractauditorAI)
