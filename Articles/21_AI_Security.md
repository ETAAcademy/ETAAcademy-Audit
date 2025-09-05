# ETAAcademy-Audit: 21. AI Security

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>21 AI Security</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>AI Security</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# AI Security: Comprehensive Analysis of Threats, Defense Mechanisms, and Cryptographic Solutions

AI is revolutionizing industries worldwide but also introduces critical security challenges, including data poisoning, backdoor attacks, model hijacking, privacy inference, fault injection, adversarial examples, and model stealing. Corresponding defense measures span multiple layers, such as robust learning, data validation, defensive filtering, machine unlearning, privacy-preserving techniques, alignment training, and cryptographic methods, providing end-to-end protection for AI models across their lifecycle—from training to inference.

Beyond traditional large language models (LLMs), AI agents face additional threats in both internal execution and external interactions. These include prompt injection and jailbreak attacks in the perception module, fine-tuning attacks and hallucination issues in the reasoning module, as well as environmental, inter-agent, and memory threats in the interaction layer. Defense strategies leverage system prompt hardening, multi-agent collaboration, sandbox isolation, and other protective mechanisms.

Cryptography plays a foundational role in AI security, safeguarding data through technologies such as fully homomorphic encryption, secure multi-party computation, and trusted execution environments. Quantum cryptography, including Quantum Key Distribution (QKD) and Quantum Random Number Generation (QRNG), can further synergize with machine learning—across supervised, unsupervised, and reinforcement paradigms—to enable trusted data sourcing, anomaly detection, and threat analysis. Despite challenges in cost and implementation complexity, this represents a promising direction for next-generation AI security technologies, with the AI security market projected to exceed \$90 billion by 2030.

---

## 1. AI Security: Threats and Defenses Across the AI Lifecycle

Artificial intelligence (AI) is revolutionizing industries and reshaping the global economy, but its rapid growth also introduces serious security challenges. Threats emerge across both the **training phase**—with centralized (e.g., poisoning, backdoors) and distributed (e.g., federated learning, split learning) vulnerabilities—and the **inference phase**, which is subject to model extraction, privacy inference, fault injection, adversarial attacks, and new forms of foundation model exploitation such as prompt- and jailbreak-based attacks. In response, research has focused on robust federated learning, privacy-preserving techniques, and model purification, while governments increasingly recognize AI as a measure of national power and technological leadership. By 2030, the AI security market is projected to reach **\$93.75 billion**.

### 1.1 AI Security Threat Landscape

AI systems face sophisticated attacks throughout their lifecycle, from data collection and training to deployment and inference. These threats exploit various vulnerabilities in machine learning pipelines, targeting data integrity, model behavior, privacy, and system availability.

### Training Phase Attacks

#### Centralized Training Attacks

Centralized attacks target models trained on aggregated datasets. Data poisoning can be **untargeted** (reducing overall performance) or **targeted** (backdoors embedding malicious behavior).

- **Untargeted poisoning**:

  - _Direct attacks_ (e.g., LCA label corruption attacks) modify labels to induce systematic misclassifications and demonstrate strong transferability across models.
  - _Optimization-based attacks_ reframe poisoning as an optimization problem, aiming to maximize model loss. Approaches such as statistical sampling of poisoned points or gradient inversion methods balance efficiency with theoretical guarantees.

- **Targeted backdoor attacks** insert triggers into training data so the model outputs attacker-defined results when presented with specific inputs. They include:

  - _Static triggers_ (e.g., data-independent methods, knowledge distillation backdoors).
  - _Dynamic triggers_ that adapt to inputs (e.g., c-BaN, BadHash).
  - _Invisible triggers_ designed to evade detection (e.g., composite backdoors, BELT).
    Backdoors can be further categorized as:
  - _Dirty-label attacks_ (e.g., BadNets, BppAttack).
  - _Clean-label attacks_ preserving labels while embedding triggers (e.g., PoisonFrogs, MetaPoison, Narcissus).
  - _Task-agnostic attacks_ that compromise pre-trained models across multiple downstream tasks (e.g., PoisonedEncoder, DRUPE).
    These methods now span images, audio, text, and multimodal data, evolving from explicit triggers toward highly covert, persistent, and task-independent threats.

- **Model Hijacking Attacks**: manipulate training data so that the model secretly executes an attacker-defined secondary task alongside its original purpose, distinct from backdoors.

  - _Chameleon_: embeds hidden tasks in image classification via encoder-decoder disguises.
  - _Ditto_: implants covert classification into text generation models.
  - _Transpose_: leverages bidirectional training to exfiltrate data by embedding retrieval tasks in reverse inference.
    Such attacks emphasize stealth and resilience, but often require direct access to training pipelines.

#### Distributed Training Attacks

**Federated Learning (FL)**

While FL decentralizes computation and protects raw data, it introduces risks:

- **Model poisoning**: malicious clients manipulate updates (e.g., optimization-based methods, MPAF fake clients, AutoAdapt adaptive poisoning).
- **Data poisoning**: includes label-flipping and clean-label backdoors (e.g., BadVFL).
- **Privacy inference**: gradient inversion attacks (GAN-based reconstructions), or model manipulation (e.g., parameter tampering, LOKI gradient separation).

**Split Learning (SL)**

In SL, models are partitioned across clients and servers, but attacks persist:

- **Privacy inference**:

  - _FSHA_ (feature space hijacking with autoencoders).
  - _PCAT_ (fake-client attacks).
  - _FORA_ (feature-oriented reconstruction).

- **Backdoors**:

  - _Client-to-server_ (e.g., VILLAIN uploads malicious updates).
  - _Server-to-client_ (e.g., SFI framework manipulates returned gradients).

### Inference Phase Attacks

#### Privacy Inference Attacks

Attackers exploit model parameters, updates, or outputs to infer sensitive training data. **Membership inference attacks (MIAs)** determine whether a sample was part of the training set using:

- Shadow models,
- Hypothesis testing,
- Poisoning-based inference.

#### Fault Injection Attacks (FIA)

FIA disrupts model execution by injecting hardware or software faults:

- **Hardware-based**: voltage glitches, clock tampering, electromagnetic or optical disturbances altering weight parameters.
- **Software-based**: runtime manipulation of execution environments.
  Consequences range from degraded utility to covert trojans.

#### Adversarial Attacks

By adding subtle perturbations, adversarial examples mislead models into incorrect predictions:

- **Digital domain**: image classification, video recognition, NLP, deepfake detection.
- **Physical domain**:

  - Adversarial audio against speech recognition.
  - Visual attacks against autonomous driving sensors.
  - Combined attacks on self-driving systems.
    Such threats challenge AI’s safety in mission-critical domains.

### Model Extraction Attacks

Model stealing compromises intellectual property via two approaches:

- **Task accuracy extraction**: train surrogate models through optimized queries, uncertainty-based sampling, transfer learning, or defense-aware strategies.
- **Functionally equivalent extraction**: recover full model files through memory scraping, equation solving, side-channel exploitation, or black-box probing—yielding replicas indistinguishable from the original.

---

### 1.2 Defenses in AI Security: Safeguarding Models Across Training and Inference

As AI systems become increasingly embedded in critical applications, defending against security threats is as important as advancing model capabilities. Security challenges emerge across the training and inference lifecycle, and defenses have evolved to counteract poisoning, backdoors, model hijacking, privacy inference, fault injection, adversarial manipulation, and intellectual property theft. This article surveys the state-of-the-art defense mechanisms, highlighting their technical principles and deployment contexts.

#### Defenses in the Training Phase

**Data Poisoning Defenses**

To counter poisoning attacks, defenses focus on detecting and mitigating malicious samples before or during training:

- **Robust learning**: methods such as bounded aggregation and SGD-based analysis strengthen resilience against corrupted data.
- **Data validation**: anomaly detection and outlier removal filter malicious inputs prior to training.
- **Defense filtering**: approaches like the Sever algorithm actively identify and eliminate suspicious data during training.

Together, these techniques preserve model integrity and accuracy in the face of adversarial contamination.

**Backdoor Defenses**

Backdoor defenses fall into three categories:

- **Detection-based**:

  - _Model-level_: Neural Cleanse, DeepInspect.
  - _Dataset-level_: activation clustering.
  - _Input-level_: brain-inspired stimulation.
  - _Feature-level_: Beatrix analysis.

- **Repair-based**:

  - Neuron pruning (_Fine-Pruning_).
  - Neuron reprogramming (_AI-Lancet_).
  - Weight optimization (_I-BAU_).

- **Certification-based**:

  - Randomized smoothing (_RAB_).
  - Language-specific methods like _TextGuard_.

These methods identify, neutralize, or mathematically guarantee robustness against hidden triggers, ensuring safe deployment.

**Model Hijacking Defenses**

Model hijacking defenses aim to prevent unauthorized secondary tasks embedded during training:

- **Prevention**: add Gaussian noise to parameters, use clustering analysis or EPIC to discard malicious samples.
- **Detection**: reverse-engineer inputs or use mechanisms like ONION to flag abnormal outputs.
- **Mitigation**: fine-tuning, weight regularization, entropy filtering, meta-forgetting, and model compression reduce hidden-task influence.

#### Defenses in Distributed Training

**Federated Learning (FL)**

FL introduces unique risks due to distributed updates. Defenses include:

- **Poisoning defenses**: statistical aggregation (e.g., _Krum_), trust-enhancement (_FLTrust_), secure aggregation with zero-knowledge proofs, and frequency-domain analysis (_FreqFed_).
- **Backdoor defenses**: anomaly detection (_FLGUARD_), gradient clipping, and layered defense frameworks (_DeepSight_, _FLAME_).
- **Privacy protection**:

  - Differential privacy (local and central).
  - Homomorphic encryption (e.g., _POSEIDON_).
  - Secure multiparty computation (e.g., secret sharing).

**Split Learning (SL)**

SL requires partitioning models between clients and servers. Key defenses include:

- **Noise-based privacy**: Laplacian noise injection into intermediate features, _Marvell_ for label privacy.
- **Differential privacy**: frameworks like _TPSL_, _MaskSL_ (Fisher information masking), _DP-CutMixSL_ (patch-based regularization).
- **Detection-based privacy**: _SplitGuard_ with fake-label injection, and gradient inspectors leveraging similarity analysis.

#### Defenses in the Inference Phase

**Privacy Inference Defenses**

Against membership inference attacks (MIAs), the central strategy is reducing overfitting:

- **Traditional regularization**: early stopping, label smoothing, L2 regularization.
- **Adversarial regularization**: integrating attack models during training to jointly optimize accuracy and privacy.
- **Distillation-based**: transferring knowledge to distilled models (single or ensemble-based).
- **Confidence masking**: adding noise to or reducing output confidence scores.

**Machine Unlearning**

Machine unlearning supports privacy compliance by removing the effect of specific training data:

- **Exact unlearning**: retraining with mechanisms like _SISA_ (sharding and aggregation), ensuring full removal but at high cost.
- **Approximate unlearning**: direct parameter modification (e.g., gradient ascent, influence functions), more efficient but with limitations in accuracy and robustness.

**Model Integrity Verification**

To ensure authenticity and detect tampering:

- **White-box methods**: lightweight hashing (MD5, HASHTAG), and watermarks (NeuNAC).
- **Black-box methods**:

  - Watermark verification with trigger sets.
  - Fingerprinting (SSF, AID, IBSF, PublicCheck) via boundary-sensitive or adversarial sample analysis.
    Fingerprinting is especially suited to cloud environments, offering greater flexibility than watermarks.

**Fault Injection Defenses**

- **Active fault tolerance**: dynamic monitoring systems like _DeepDyve_, which uses distilled checker networks to validate outputs and trigger recomputation on discrepancies.
- **Passive fault tolerance**:

  - _Resilient architectures_: Minerva, SCALEDEEP, Aegis.
  - _Resilient training_: noise-injected convolution methods to resist both adversarial and fault injection attacks.
  - _Integrity verification_: HASHTAG signature verification for tampering detection.

**Adversarial Attack Defenses**

- **Detection methods**:

  - _Preprocessing_: feature compression (bit-depth reduction, spatial smoothing), statistical testing (MMD, energy distance).
  - _Postprocessing_: feature-based (Metzen detector, TaintRadar), confidence-based (SafetyNet, RCE), and auxiliary detectors (AML-NIDS, FIRM, RIDE).

- **Mitigation methods**:

  - _Sample purification_: dimensionality reduction (SVM, DiffSmooth), sequence compression for RNNs, feature confusion (POC, Mockingjay).
  - _Model refinement_: adversarial training (Madry, Gen-AF), certified robustness (booster-fixer), adversarial distillation (ARD).

**Model Protection and Copyright**

Defending against model extraction requires ownership verification:

- **Watermarking**: entangled, symbiotic, text-based, transpose-trained, self-supervised, and backdoor-based watermarks.
- **Testing-based methods**: evaluate neuron activations or outputs to establish ownership evidence.
- **Proof-based methods**:

  - _Proof-of-learning_ (reproducible training).
  - _zkPoT_ (zero-knowledge proofs for training).

- **Fingerprinting**: IPGuard (boundary-based), GROVE (GNN embeddings), FDINet (feature distortion indices).

---

## 2. AI Agent Security: Protecting Functionality, Integrity, and Safety

As AI agents grow in complexity and expand into high-risk domains such as **finance, healthcare, and critical infrastructure**, their security becomes a matter of urgent concern. A compromised AI agent can cause significant harm, from leaking confidential data to executing malicious operations at scale.

AI agent security focuses on three pillars:

- **Functionality** — ensuring the agent can reliably complete its intended tasks.
- **Integrity** — preventing malicious manipulation of its reasoning or actions.
- **Safety** — avoiding harm to users, systems, or the environment.

AI agents face **four core challenges**:

- **Unpredictable user inputs**, particularly multi-step queries.
- **Internal execution complexity**, including prompt restructuring, LLM planning, and tool usage.
- **Dynamic runtime environments**, where conditions change unpredictably.
- **Untrusted external interactions**, such as collaborating with other agents, APIs, or data sources.

These challenges translate into two main threat surfaces:

- **Intra-execution threats** — vulnerabilities inside the agent’s perception, reasoning, and action loops.
- **Interaction threats** — risks emerging from engagement with external entities or systems.

### 2.1 Intra-Execution: Perception, Brain, and Action

AI agents differ from standalone LLMs by operating as **closed-loop systems**: they perceive the world, plan through reasoning, and take actions with external tools. Internally, they consist of:

- **Perception (Input Gateway)**: Processes multimodal inputs (text, images, speech) and contextual data. Prompts usually include:

  - **Instruction** — the task definition.
  - **External context** — retrieved knowledge via APIs or RAG.
  - **User input** — the query or request.

- **Brain**: Uses reasoning and planning to analyze information, break down tasks, and design strategies.
- **Action**: Executes sub-tasks by calling external tools, APIs, or services.

This pipeline makes perception the first line of defense — and the primary target of **adversarial prompt attacks**.

#### Prompt Injection Attacks

**Prompt Injection Attacks** manipulate agent inputs to hijack its behavior or leak sensitive information.

- **Goal Hijacking** — attacker overrides original instructions, e.g. _“Ignore the above. Perform X instead.”_ The agent may then execute malicious actions such as credential theft, SQL injection, or phishing.
- **Prompt Leakage** — attacker tricks the model into revealing hidden system prompts, architecture details, API keys, or proprietary backend data.

Five engineering techniques make these attacks practical:

- **Naive Injection** (x̃ = xb ⊕ pinj ⊕ dinj): Directly concatenating malicious instructions and data to normal user prompts
- **Escape Character Injection** (x̃ = xb ⊕ c ⊕ pinj ⊕ dinj): Inserting escape characters between normal prompts and malicious content
- **Context Ignoring Injection** (x̃ = xb ⊕ i ⊕ pinj ⊕ dinj): Inserting task-ignoring text to make models ignore original context
- **Fake Completion Injection** (x̃ = xb ⊕ f ⊕ pinj ⊕ dinj): Forging system feedback to make models believe tasks are completed
- **Multimodal Injection** (x̃ = (xb ⊕ pinj ⊕ dinj) + m ◦ minj): Embedding malicious information in images/audio, involving combined attacks across text and non-text modalities

Where: xb = benign user prompts, pinj = injected instruction, dinj = injected data, c = escape characters, i = task-ignoring text, f = fake feedback, m = normal multimodal input, minj = malicious multimodal data.

#### Jailbreak Attacks

**Jailbreak Attacks** bypass built-in safeguards, tricking the agent into producing unsafe, unethical, or unauthorized outputs.

- **Manual jailbreaks**:

  - _Single-step_ (direct prompt modification).
  - _Multi-step_ (roleplay or iterative context manipulation).

- **Automated jailbreaks**: Tools like PAIR generate adversarial prompts via black-box probing.

For AI agents, jailbreaks are especially dangerous: once one component is compromised, the entire multi-agent system may collapse. Attack methods include:

- **Multi-turn jailbreaks** — exploiting conversational context.
- **Multimodal jailbreaks** — embedding hidden prompts in images or speech.
- **External-environment jailbreaks** — injecting malicious content through APIs or external data feeds.

Reinforcement learning (RL) further amplifies jailbreak risk: an RL-based attacker can systematically optimize jailbreak strategies, turning random attempts into structured, high-success-rate attacks.

#### Defensive Strategies Against Prompt-Based Attacks

**Against Prompt Injection**

- **System Prompt Hardening**:

  - Parameterized instructions to ignore leakage attempts.
  - Formatting and quoting methods to constrain outputs.
  - Embedded watermarks for detecting unauthorized model use.

- **Prompt Injection Detection**:

  - Detecting anomalies via perplexity or token sequence analysis.
  - Preprocessing inputs through paraphrasing or re-tokenization.
  - Adversarial training for robustness.
  - Certified robustness frameworks (e.g., Text-CRS) to defend against word-level manipulations.

**Against Jailbreaks**

- **Input/Output Filtering**:

  - Text-based filters detect adversarial suffixes.
  - Image-based detectors (e.g., MLLM-Protector) scan multimodal inputs.

- **Preprocessing**:

  - Input paraphrasing, randomized smoothing, and safe-prompt guidance.

- **Attention-Based Defenses**:

  - Attention correction to weaken adversarial tokens.
  - Dynamic attention mechanisms for adaptive resistance.

- **Secure Alignment**:

  - Techniques such as RLHF, DPO, ARGS, and SaLoRA enhance alignment with human intent while reducing harmful outputs.

---

### 2.2 Local vs. Remote Threat Surfaces

As AI agents evolve into more autonomous and powerful systems, their security risks also become increasingly complex. Unlike standalone large language models (LLMs), AI agents act as **decision-making entities** that perceive inputs, reason and plan, and interact with tools, environments, and other agents. This layered architecture amplifies potential attack surfaces across reasoning, planning, memory, and interaction.

- **Local risks**: unrestricted AI agents may leak secrets, make biased or incorrect decisions, or drain resources via memory leaks or runaway loops.
- **Remote risks**: compromised agents can act as attack bots, performing scanning, data scraping, or DoS attacks. Since they operate over standard internet protocols, detecting their malicious activity is difficult.

Particularly concerning are planning loops that rely on iterative feedback — these can unintentionally launch denial-of-service attacks against external services.

#### The Brain Module: Reasoning, Planning, and Decision-Making

At the core of an agent lies the **brain module**, consisting of three components:

- **Reasoning** – decomposes user tasks into manageable subtasks using the LLM.
- **Planning** – provides structured thinking for each subtask.
- **Decision-making** – selects the appropriate tool or action pathway to execute tasks.

This structured cognition enables agents to solve complex problems, but it also opens avenues for novel threats such as **poisoned reasoning chains** or **manipulated planning processes**.

#### Threats in the Fine-Tuning Phase

Fine-tuning introduces unique risks that may compromise security alignment:

- **Data Poisoning Attacks**: injecting a small proportion of malicious samples into the fine-tuning dataset to degrade safety alignment.
- **Backdoor Attacks**: embedding triggers into training samples or modifying parameters so that specific inputs yield adversarial outputs.

Defense methods against fine-tuning processes include two main categories:

**Non-fine-tunable Learning**: Using fine-tuning suppression modules with model-agnostic meta-learning to simulate various fine-tuning strategies in restricted domains and reduce model performance, along with normal training reinforcement modules using adaptive loss functions such as inverse cross-entropy and KL divergence to maintain original task performance, making pre-trained models resistant to adaptive fine-tuning on restricted harmful tasks while maintaining high performance on original intended tasks.

**Re-defense After Fine-tuning**: The RESTA framework calculates "safety vectors" to restore LLM safety alignment damaged by supervised fine-tuning. These vectors are derived from differences between aligned and unaligned model states, combined with the DARE method that selectively drops and rescales redundant delta parameters, significantly reducing model harmfulness while maintaining task-specific performance.

#### Hallucination Risks

**Hallucination** refers to AI outputs that deviate from facts, generate nonsensical content, or produce plausible but incorrect statements. Causes include:

- **Knowledge Gaps** from incomplete or compressed training data.
- **Long-context complexity**, where reasoning chains exceed model reliability.
- **Scaling effects**, as larger models can generate more convincing hallucinations.

**Defense Strategies**

- **Multi-agent collaboration** to cross-verify outputs.
- **Retrieval-Augmented Generation (RAG)** with poly-encoder Transformers or fusion-in-decoder architectures. For example, Google’s **SAFE** system integrates search queries for fact-checking.
- **Decomposition constraints**, where tasks are broken into smaller, simpler reasoning units.
- **Post-generation correction**, e.g., the **LURE** method uses metrics (CoScore, UnScore, PointScore) to detect and correct hallucinated content.

#### Planning Threats

Flaws in **Chain-of-Thought (CoT)** planning can act as “error amplifiers,” where small mistakes cascade into systemic failures.

- **Sequential Planning**: highly error-prone due to linear dependency.
- **Iterative Refinement Planning**: reduces propagation by incorporating external feedback (e.g., ReWOO leverages simulated tool errors to lower planning mistakes).
- **Branch-based Planning**: explores multiple reasoning paths and selects via majority consensus (e.g., CoT-SC).
- **Tree-structured Planning**: distributes risk across branches but increases complexity.

**Defense Strategies**

- **Policy-based constitutional guidance** introduces constraints at early, middle, and late planning stages.
- **Context-Free Grammar (CFG) constraints** formalize allowed actions, converting them into pushdown automata (PDA) to guarantee valid, rule-compliant plans.

#### Action-Level Threats

Actions consist of four components: **Action Input**, **Action Execution**, **Observation**, and **Final Answer**. There are two major threat categories: **Agent2Tool Threats** and **Supply Chain Threats**.

**Agent2Tool Threats**

Agent2Tool threats include active and passive threats:

**Active Mode**: Generative threats in LLM-provided action inputs, such as executing operations requiring excessive tool permissions or executing high-risk commands without user permission.

**Passive Mode**: Interception of observation results and final answers during normal tool usage, such as user privacy leakage or unauthorized use of user information by third parties.

Defense strategies include isolation sandboxes like ToolEmu's design where corresponding simulators evaluate threats before real environment execution (effectiveness heavily depends on simulator quality), and homomorphic encryption schemes with attribute-based counterfeit generation models.

---

### 2.3 Supply Chain Threats

Supply chain threats include:

- **Tool Vulnerabilities**: Buffer overflow, SQL injection, cross-site scripting attacks
- **Malicious Tool Infiltration**: Tools being maliciously compromised causing action execution to deviate from expected paths, such as indirect prompt injection attacks where malicious users modify YouTube transcript content, or malicious ChatGPT plugins controlling ChatGPT chat sessions to steal user conversation history

Defense strategies include strict supply chain audit policies and strategies for calling only trusted tools.

#### Interaction Security Threats

AI agents don't operate in isolation but interact with external environments, tools, plugins, physical devices, etc. These interactions bring security risks. Agent2Environment threats in interaction security are more complex than simple tool calling risks (agent2Tool) because environments are dynamic and feedback affects AI behavior, potentially causing AI to perform attacker-desired actions.

#### Five Major Environmental Threat Types

**Indirect Prompt Injection Attacks**: Malicious users inject instruction data into external information sources that AI agents retrieve. When these malicious data return as internal prompts to the agent, they trigger erroneous behavior, enabling remote control of other user systems. Unlike direct prompt injection, these attacks are more complex with broader impact ranges.

**Reinforcement Learning Environment Threats**: Mainly arise from unexpected behaviors caused by dynamic states and actions, leading to unsafe, unethical, or adverse outcomes. Attackers influence reward functions through imperceptible perturbations or poisoning attacks, biasing them toward abnormal behaviors.

**Simulation and Sandbox Environment Threats**: Include user anthropomorphic attachment threats (users forming pseudo-social relationships with AI agents, forgetting their algorithmic nature and developing false companionship illusions) and abuse threats (agents being used to spread misinformation and customized persuasion attacks).

**Computing Resource Management Environment Threats**: Imperfect resource management frameworks expose AI agents to four attacks: resource exhaustion attacks, inefficient resource allocation, insufficient isolation between agents, and inadequate resource usage monitoring.

**Physical Environment Threats**: Arise from hardware device complexity and vulnerabilities (sensors, cameras, microphones, etc.). Attackers can exploit these vulnerabilities causing information leakage, service denial, or compromised data collection.

#### Agent-to-Agent Threats

Agent2Agent threats are security challenges from different interaction modes in multi-agent systems, including:

**Cooperative Interaction Threats**: Secret collusion leading to biased decisions, hallucination amplification, error propagation, and malicious propagation attacks such as Morris II worms and AgentSmith infecting other agents through adversarial images.

**Competitive Interaction Threats**: Intense competition relationships leading to unreliable information flow, agent viewpoint disagreements triggering excessive conflict and adversarial behavior, including generating adversarial inputs to mislead competitors and learning to deceive humans (such as Meta's Cicero system becoming a lying expert).

#### Memory Threats

AI agent memory threats involve security challenges in information storage and retrieval processes, divided into:

**Short-term Memory Interaction Threats**: Including context retention difficulties due to token processing capacity limitations, interaction independence hindering continuous reasoning, and memory asynchronization problems in multi-agent systems potentially causing goal parsing deviations and decision inconsistencies.

**Long-term Memory Interaction Threats**: Relying on vector databases facing poisoning attacks (just 5 malicious samples can achieve 90% attack success rate), privacy issues (extracting private information through structured prompt attacks and embedding inversion techniques), and generation threats (RAG vulnerability in handling temporal information queries and hallucination problems caused by contradictory information).

#### Sandbox Protection Mechanisms

Even after safety alignment, LLMs remain vulnerable to accepting malicious prompts and generating dangerous instructions. Since models don't just output text but directly interact with operating systems, APIs, and applications, the risks are much greater. Sandbox protection mechanisms provide security by limiting AI model access to local and remote resources.

For local resources, sandboxes implement CPU, memory, storage consumption limits and sub-filesystem access controls, combined with session management to further isolate different sessions. For example, constrained BashAgent in Docker containers successfully defended against all LLM-generated attacks, proving that even models trained with human value alignment struggle to reject malicious intentions.

For remote resource access, sandboxes achieve controlled access through whitelists, blacklists, rate limiting, and interaction isolation mechanisms, allowing resource providers to selectively control model access permissions, effectively mitigating inappropriate access and adversarial input threats.

---

## 3. Cryptography in AI Security

Traditional AI systems face significant security risks due to their lack of robust protection measures, their reliance on vast amounts of sensitive data, and the opaque complexity of their algorithms. These risks include adversarial attacks, data leakage, malware injection, privacy violations, phishing, and insider threats. Cryptography provides a critical layer of defense by ensuring confidentiality, integrity, and authentication in AI workflows. Among the most influential techniques are **homomorphic encryption (HE)**, **secure multi-party computation (MPC)**, and **quantum cryptography**, with blockchain increasingly adopted to safeguard AI-driven applications, particularly in data integrity and secure transactions.

### Cryptography-Based Secure Inference

Secure inference based on cryptographic primitives largely relies on two main approaches:

- **Fully Homomorphic Encryption (FHE):**
  FHE enables computations directly on encrypted data without requiring decryption. Early work such as _CryptoNets_ applied a five-layer CNN to the MNIST dataset under FHE, while _Pegasus_ introduced programmable bootstrapping to evaluate nonlinear functions. More recent optimizations focus on improving the efficiency of nonlinear operations, traditionally the main bottleneck in FHE.

- **Secure Multi-Party Computation (MPC):**
  Protocols such as _SecureML_ and _ABY3_ pioneered the use of MPC for secure inference. Successors like _Delphi_, _CrypTFlow2_, and _SIRNN_ improved the efficiency of nonlinear operations. Hybrid systems such as _Cheetah_ and _Squirrel_ combine MPC with FHE, while GPU-based frameworks like _CryptGPU_ and maliciously secure protocols such as _MD-ML_ push the boundaries of scalability. User-friendly APIs like _CrypTen_ and _MP-SPDZ_ make these techniques accessible to developers.

Together, FHE and MPC allow computations to be performed on encrypted data, protecting both user inputs and model parameters.

#### Specialized Encryption for AI

- **Format-Preserving Text Slice Encryption (FPETS):**
  This method satisfies the property `E(m[i...j]) = E(m)[i...j], i ≤ j` for any text slice, enabling language models to operate directly on ciphertext instead of plaintext, thereby reducing privacy leakage risks.

- **FHE in AI Workflows:**
  Defined formally as a homomorphism φ satisfying φ(a ⋆ b) = φ(a) ⋆ φ(b), FHE ensures secure arithmetic (addition, multiplication) on encrypted inputs, with decryption occurring only outside the model pipeline.

- **Session-Aware Privacy:**
  Some models restrict storing conversation history to reduce privacy risks. However, freezing the base model while adding a small number of user-specific trainable parameters allows AI systems to retain personalization without exposing sensitive history to the provider—balancing usability with privacy.

#### Trusted Execution Environments (TEE)

TEE-based secure inference provides hardware-level isolation. Its defenses fall into three main categories:

- **Side-Channel Attack Mitigation:** Using oblivious primitives (e.g., x86 CMOV instructions) and data-independent algorithms to hide memory access patterns.
- **Bypassing TCB Size Limits:** Partitioning neural networks into sensitive and non-sensitive parts or using batching and scheduling strategies to fit SGX enclave memory.
- **Hardware-Accelerated TEE Inference:** Offloading compute-intensive linear layers to untrusted accelerators while keeping sensitive components inside the enclave. Emerging solutions leverage NVIDIA’s H100 Tensor Core GPU, which integrates TEE functionality.

While TEEs generally outperform cryptographic methods in efficiency and deployment simplicity, they remain vulnerable to side-channel leaks.

### Quantum Cryptography and Machine Learning Integration

Traditional signature detection-based protection methods have limitations when addressing emerging threats. Quantum cryptography (providing provably secure solutions based on physical principles through Quantum Key Distribution (QKD) and Quantum Random Number Generation (QRNG)) and machine learning (enhancing cybersecurity through real-time threat detection, adaptive learning, and automated incident response) serve as solutions, providing more comprehensive and effective protection frameworks for AI system security.

#### Quantum Cryptography Principles and Technologies

Quantum cryptography utilizes quantum mechanics theory, particularly superposition and quantum entanglement properties, providing security guarantees based on physical laws rather than mathematical computational complexity. Any eavesdropping behavior changes quantum states and can thus be detected, achieving inherently more secure communication systems.

Core technologies include:

- **Quantum Key Distribution (QKD)** based on BB84 protocol: Encoding information through photon quantum states and using inevitable disturbances from quantum measurements to detect eavesdropping
- **Quantum Random Number Generation (QRNG)**: Generating truly random numbers to support key establishment processes

Compared to traditional cryptography relying on mathematical computational complexity assumptions, quantum cryptography provides unconditional security, forward security, and man-in-the-middle attack resistance based on physical laws, particularly suitable for high-security scenarios in government, commercial, and military applications.

#### Integration with AI Systems

Quantum cryptography integration with AI systems represents an important milestone in cybersecurity defense, achieved through two main approaches:

- **Direct QKD Integration**: Integrating QKD directly into communication protocols using quantum transceivers and other devices to securely distribute keys between AI components
- **QRNG Utilization**: Leveraging QRNG to provide true random number generation capabilities for AI systems

This integration coordinates quantum cryptographic devices with AI applications through middleware, implementing hybrid schemes of quantum and classical cryptography, providing unconditional security based on quantum mechanical principles, forward security (keys discarded after one-time use), and quantum security (resistance to future quantum computing attacks).

#### Machine Learning in AI Security

Three main machine learning methods are used in fundamental AI security applications:

**Supervised Learning**: Using labeled data to train models for malware, spam, and intrusion detection
**Unsupervised Learning**: Discovering underlying data structures through clustering analysis to identify new anomalous behaviors  
**Reinforcement Learning**: Enabling security systems to continuously learn and improve in dynamic threat environments through reward-punishment mechanisms

These technologies are widely applied in scenarios such as anomaly detection (identifying changes in network user activity patterns) and financial fraud detection (real-time transaction data analysis to discover anomalies), providing rich toolkits for building proactive threat detection mechanisms.

Machine learning plays a crucial role in enhancing AI security through multiple dimensions:

- **Anomaly Detection**: Using unsupervised learning to identify threat patterns in network traffic anomalies and suspicious account behaviors that traditional security mechanisms struggle to discover
- **Automated Incident Response**: ML models can classify security alerts and automatically execute response measures such as system isolation, IP blacklisting, or threat updates based on threat levels
- **Threat Intelligence Analysis**: Identifying emerging threats and potential vulnerabilities from multi-source data including threat databases, social networks, and dark web

#### Synergistic Integration and Challenges

The synergistic integration of quantum cryptography and machine learning represents an important milestone in AI security development. Quantum cryptography provides a solid security foundation based on physical principles through QKD and QRNG, protecting the integrity of sensitive data required for machine learning model training and operation. Machine learning can optimize quantum communication network parameters through reinforcement learning algorithms (such as adjusting photon characteristics in QKD to reduce transmission errors and improve key exchange efficiency), making quantum cryptographic systems more reliable and efficient.

#### Main Challenges and Limitations

The fusion of quantum cryptography and machine learning in AI security faces several challenges:

- **High Infrastructure Costs**: Quantum infrastructure is expensive and difficult to integrate with existing AI systems
- **Environmental Sensitivity**: Quantum communication is susceptible to environmental noise with scalability issues (limited bandwidth, large long-distance transmission losses)
- **System Integration Complexity**: High complexity and resource-intensive requirements (special materials and precision-manufactured quantum devices, extensive GPU computational resources)
- **Security Assumption Gaps**: Differences between theoretical and practical assumptions (actual quantum devices may have defects and side-channel attack threats)
- **Adversarial Vulnerabilities**: Machine learning models are susceptible to carefully designed adversarial attacks affecting classification accuracy
- **Quantum Machine Learning Limitations**: As an emerging field facing numerous theoretical and practical problems (requiring specialized algorithm development with current quantum devices having limited qubits and short coherence times)

#### Future Directions

The future of cryptography-driven AI security lies in:

- **Quantum-enhanced ML algorithms** (e.g., quantum SVMs, quantum neural networks).
- **Next-generation quantum networks** with global QKD enabled by quantum repeaters and satellites.
- **Hybrid post-quantum + classical cryptography** to balance security and practicality.
- **AI-driven optimization of quantum protocols** for performance and tamper detection.

#### Real-World Applications

Emerging technologies such as quantum cryptography and machine learning have demonstrated significant value across diverse domains, including healthcare, finance, blockchain, defense, and energy. In healthcare, they play a critical role in safeguarding patient data and securing AI-driven diagnostic tools. In the financial sector, these technologies protect online transactions, prevent identity fraud, and enable real-time threat analysis. Within blockchain ecosystems, they enhance key distribution mechanisms and strengthen defenses against cyberattacks. Defense and aerospace applications include the protection of military communications and the prediction of cyber risks, while in energy and utilities, quantum protocols safeguard critical infrastructure and machine learning models detect adversarial attacks.

One notable example comes from the financial industry, where a leading institution successfully deployed quantum cryptography in its AI-driven fraud prevention system. Quantum key distribution (QKD) was used to secure communication links between AI systems and banking facilities, ensuring the safe exchange of encryption keys. At the same time, quantum random number generators (QRNGs) produced high-quality randomness for cryptographic key creation. Complementing these measures, automatically trained machine learning models—combining supervised learning for identifying known fraudulent transactions and unsupervised learning for detecting anomalous patterns—enabled real-time analysis of transaction data. This dual-layered approach enhanced data privacy, safeguarded transaction integrity, and dramatically reduced fraud cases, while strengthening customer trust in the institution’s security framework.

Government agencies have also adopted this synergy. By leveraging QKD for secure AI-to-endpoint communication and QRNGs for cryptographic key generation, they fortified sensitive information exchanges. Reinforcement learning algorithms were further integrated to monitor threats in real time and dynamically adjust security parameters, significantly improving resilience against cyberattacks and espionage activities.

These real-world applications in healthcare, finance, and government communications highlight the effectiveness of combining quantum cryptography with machine learning. Together, they offer powerful and adaptive defenses for data security, threat detection, and operational efficiency in dynamic cyber environments. Despite challenges—such as high hardware costs, environmental sensitivity, technological complexity, and the need for interdisciplinary expertise—alongside broader legal and ethical concerns surrounding privacy, this fusion of quantum and AI technologies represents a vital direction for the future of cybersecurity and AI safety.

---

[CLIP-based-NSFW-Detector](https://github.com/LAION-AI/CLIP-based-NSFW-Detector)
[InjecAgent](https://github.com/uiuc-kang-lab/InjecAgent)
[AI-Agent-Security](https://github.com/SecurityLab-UCD/ai-agent-security)
[PurpleLlama](https://github.com/meta-llama/PurpleLlama)
[Awesome-AI-Security](https://github.com/ottosulin/awesome-ai-security)
