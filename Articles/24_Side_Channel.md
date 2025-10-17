# ETAAcademy-Audit: 24. Side-Channel Security

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>24 SCA</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>SCA</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# Side-Channel Attacks in the Age of AI: Evolution, Threats, and Countermeasures

Side-Channel Attacks (SCA), first introduced by Paul Kocher in 1996, have emerged as a critical threat to modern cryptographic systems. These attacks exploit physical information leakage during device operation—such as power consumption patterns, timing variations, electromagnetic emissions, or fault responses—to compromise encryption systems, including both traditional AES and emerging Post-Quantum Cryptography (PQC) implementations. While PQC algorithms are designed to withstand quantum computing attacks, their hardware implementations remain vulnerable to various side-channel vulnerabilities, including power analysis, timing attacks, electromagnetic leakage, and fault injection.

In the SCA landscape, Artificial Intelligence serves a dual purpose. As an offensive tool, deep learning architectures such as CNNs and RNNs excel at extracting subtle patterns from vast amounts of side-channel data, enabling more sophisticated and stealthy attacks through cross-device generalization. Defensively, machine learning technologies power countermeasures including anomaly detection systems, intelligent noise injection mechanisms, and optimized masking schemes with dynamic power management. However, these AI-driven defensive measures face significant challenges: they often incur substantial performance overhead, remain vulnerable to adversarial attacks, and lack standardized evaluation frameworks.

Notably, AI systems themselves have proven vulnerable to side-channel attacks. Researchers have successfully extracted intellectual property—including model architectures, weights, and input data—from neural networks across diverse platforms, from microcontrollers and FPGAs to specialized AI accelerators (TPUs, NCS, and NVDLA). The vulnerability is particularly acute in machine learning inference scenarios, where embedding table lookups can leak sensitive information about model structure, parameters, and user data through memory access patterns. Traditional countermeasures like Oblivious RAM (ORAM), address space randomization, and cache partitioning offer protection but at the cost of significant performance overhead or hardware modifications. In response, novel approaches like Deep Hash Embedding (DHE) have emerged, offering an elegant solution by combining deterministic hash functions with neural networks. DHE's data-independent access patterns and deterministic execution paths provide natural resistance to certain side-channel attacks while maintaining efficiency and reducing memory footprint.

The role of AI in side-channel security thus represents a complex duality—it both enhances attack capabilities and strengthens defensive measures. However, implementing robust and verifiable protections in real-world systems requires a holistic approach: coordinated defenses across hardware and software layers, careful balancing of security benefits against resource costs, improved transparency in AI-based security measures, and the development of industry-wide standards and benchmarking methodologies.

---

## 1. **AI-Driven Side-Channel Attacks and Defenses in the Post-Quantum Era**

The increasing integration of embedded systems and Internet of Things (IoT) devices into **Cyber-Physical Systems (CPS)** has raised growing concerns over cybersecurity threats. Among these, **Side-Channel Attacks (SCAs)** represent a particularly serious challenge. First introduced by **Paul Kocher in 1996**, SCAs exploit unintended information leakage—such as power consumption, electromagnetic emissions, and execution timing—to extract sensitive cryptographic data. Unlike classical cryptanalysis, which targets the mathematical foundations of cryptographic algorithms, SCAs focus on their physical implementation to recover secret information such as encryption keys.

With the advent of **quantum computing**, traditional cryptographic algorithms face increasing vulnerability, accelerating the rise of **Post-Quantum Cryptography (PQC)**. However, PQC implementations remain susceptible to SCAs. Over the years, it has been consistently demonstrated that any device performing cryptographic operations inherently leaks information in the form of power, electromagnetic radiation, or timing variations. More recently, practical attacks have even been demonstrated against **machine learning (ML) accelerators**, allowing adversaries to recover neural network architectures, weights, inputs, and other parameters. This highlights SCAs as a severe and evolving threat.

#### **AI’s Dual Role in Side-Channel Security**

The integration of **Artificial Intelligence (AI)**—including **Machine Learning (ML)** and **Deep Learning (DL)**—has had a profound impact on side-channel analysis.
On one hand, AI empowers attackers by enabling the efficient analysis of complex data patterns, making SCAs more effective and harder to detect. On the other hand, AI-driven defense mechanisms can strengthen resilience by detecting anomalies, dynamically masking sensitive operations, optimizing hardware-level security, and enhancing cryptographic implementations.

Despite these advances, **AI-driven security** still faces major challenges. These include computational overhead, susceptibility to **adversarial AI attacks**, and the lack of standardized frameworks for evaluation. Furthermore, the **transferability of AI-based attacks** across different cryptographic implementations complicates mitigation efforts, perpetuating an ongoing “arms race” between attackers and defenders. Ultimately, proactive strategies that combine **AI-based anomaly detection, adaptive countermeasures, and secure hardware design** will be crucial to safeguarding cryptographic systems in the post-quantum era.

#### **Side-Channel Threats to AES Implementations**

The **Advanced Encryption Standard (AES)** remains a foundational component of modern cryptography, securing a vast range of digital communications and data storage systems. Its architecture—based on substitution–permutation networks with a fixed number of rounds determined by key length—balances security with performance efficiency. However, AES implementations are particularly vulnerable to SCAs, which exploit physical leakage from the environment rather than weaknesses in AES’s mathematical design.

Leakage sources such as **power consumption**, **electromagnetic (EM) emissions**, **execution time**, and **cache behavior** can reveal sensitive intermediate values during encryption or decryption, ultimately exposing secret keys.

One common example is **Power Analysis Attacks**, which analyze a device’s power consumption during cryptographic operations. The resulting **power traces** can be examined visually or statistically to infer correlations between power fluctuations and secret data.

- **Simple Power Analysis (SPA)** typically requires only a single trace and relies on visual or template-based inspection.
- **Timing Attacks**, on the other hand, exploit variations in execution time—caused by cache hits/misses, conditional branching, or memory access patterns—to deduce sensitive information. Preventing such attacks requires **constant-time implementations** that eliminate data-dependent timing behavior.

#### **Emerging Threats: ML Accelerator Attacks**

In the context of ML hardware, **power and EM side channels** can be exploited for **Intellectual Property (IP) theft**. By identifying layer types and hyperparameters (e.g., kernel size, number of filters), attackers can reconstruct neural network architectures. Since neural network training often requires substantial computational and temporal resources, recovering a model structure enables attackers to **retrain** it with reduced datasets and time, obtaining economic or competitive advantages.

Furthermore, the recovered model can serve as an initial step toward **Generative Adversarial Network (GAN)-based model reconstruction**. These so-called **gray-box model extraction attacks**, where partial model information is available, have been shown to outperform traditional black-box attacks—enabling model recovery and generalized inversion with fewer queries.

#### **AI-Based Attacks on AES via Deep Learning**

In recent years, **Deep Learning (DL)** has emerged as a transformative tool in **cryptanalysis**. While AI and ML have also been explored for enhancing AES security through adaptive defenses, much of the recent research emphasizes their **offensive potential**. Leveraging neural networks’ pattern-recognition capabilities, DL-based side-channel attacks can learn complex correlations from physical leakage data (e.g., power traces, EM emissions), achieving unprecedented accuracy in recovering AES keys. Consequently, DL-based SCAs represent a serious confidentiality threat to AES-based systems.

#### **Defensive Strategies and Open Challenges**

The development of **side-channel countermeasures for secure ML** remains an open and active research area. Unlike classical cryptographic systems, ML models operate on large, structured data, making it infeasible to directly transfer traditional countermeasures.

- **Masking techniques**—which use randomization to obscure sensitive values (e.g., inputs, weights)—can protect mathematical structures but not higher-level information such as layer count or architecture.
- **Shuffling and operation reordering** are ineffective in preventing architectural leakage, as operation sequences remain visible in power traces.
- **Obfuscation approaches**, such as layer widening, branching, or scheduling randomization, have shown potential but require further evaluation on real hardware platforms with power-trace analysis.

Ultimately, advancing **secure ML hardware design**, **adaptive randomization**, and **AI-optimized anomaly detection** will be essential for defending against future SCA threats.

---

## 2. **Post-Quantum Cryptography and Its Vulnerability to Side-Channel Attacks**

**Post-Quantum Cryptography (PQC)** refers to cryptographic algorithms designed to remain secure against attacks from quantum computers. Unlike classical schemes such as **RSA** and **Elliptic Curve Cryptography (ECC)**, which rely on integer factorization and discrete logarithm problems (both vulnerable to **Shor’s algorithm**), PQC is based on mathematical problems believed to be hard even for quantum adversaries.

Recognizing the urgency of the quantum threat, the **U.S. National Institute of Standards and Technology (NIST)** initiated the **PQC standardization process** to identify and promote cryptographic algorithms resilient in the post-quantum era. Major candidates include:

- **Lattice-based cryptography** (e.g., _CRYSTALS-Kyber_, _CRYSTALS-Dilithium_),
- **Code-based cryptography** (e.g., _Classic McEliece_),
- **Hash-based cryptography** (e.g., _SPHINCS+_),
- **Multivariate** and **isogeny-based** schemes.

Each of these categories offers distinct security guarantees, yet all remain potentially vulnerable to **Side-Channel Attacks (SCAs)**. While PQC algorithms are mathematically designed to resist quantum attacks, their **implementation complexity**—involving large key sizes and intricate mathematical operations—makes them susceptible to physical leakage-based exploits. For example, **lattice-based encryption schemes**, which rely heavily on matrix multiplications, may reveal sensitive information through timing variations or power consumption patterns during computation.

#### **Principles of Side-Channel Attacks**

SCAs exploit the **physical characteristics** of cryptographic implementations rather than weaknesses in the underlying algorithms. Common SCA techniques include:

- **Timing Attacks**, which infer secret keys from variations in execution time;
- **Power Analysis Attacks**, which analyze power consumption patterns to extract key information—such as **Simple Power Analysis (SPA)** and **Differential Power Analysis (DPA)**;
- **Electromagnetic (EM) Analysis**, which captures EM emissions during cryptographic operations;
- **Fault Injection Attacks**, which induce hardware faults (via voltage, temperature, or laser manipulation) to cause computational errors that leak sensitive data.

Among these, **power analysis attacks**—particularly SPA and DPA—have proven especially effective. SPA relies on direct observation of power traces to identify key-dependent operations, while DPA applies statistical analysis to a large set of traces to recover hidden correlations with secret data. **Electromagnetic** and **fault injection attacks** extend the reach of SCAs, enabling adversaries to compromise cryptographic systems with minimal physical access.

#### **AI-Enhanced Side-Channel Attacks**

Recent advances in **Machine Learning (ML)** and **Deep Learning (DL)** have significantly amplified the effectiveness of SCAs. Modern neural networks can process vast amounts of side-channel data and autonomously learn complex patterns that reveal cryptographic secrets. Compared to traditional statistical methods, AI-driven SCAs achieve **higher accuracy**, **faster convergence**, and **greater generalization** across devices and encryption schemes.

Deep learning models trained on power traces, electromagnetic emissions, or timing data can often recover secret keys even under noisy or partially randomized conditions. This adaptability underscores the urgent need for **robust and adaptive countermeasures** against AI-assisted SCAs.

#### **Attack–Defense Lifecycle in SCA Scenarios**

A typical AI-assisted SCA follows a chronological attack–defense sequence.

- **Attack Phase:** The adversary first collects power traces during cryptographic operations and trains a DL model to extract key-related features.
- **Defense Phase:** Defenders counter these efforts using layered techniques—starting with **virtual data augmentation** to introduce noise, followed by **hiding** and **masking** strategies, and finally employing **dynamic power management** to obscure operational patterns.
- **Outcome:** If implemented effectively, these defenses disrupt the DL model’s ability to correlate physical signals with secret keys, thereby preventing key extraction and preserving system security against AI-enhanced SCAs.

#### **Challenges in Countermeasure Effectiveness**

Over the years, a variety of countermeasures have been proposed to mitigate SCAs, including **hiding** and **masking** techniques. However, the rapid evolution of deep learning has exposed their underlying limitations. Traditional **hiding** countermeasures seek to randomize or obfuscate power consumption patterns, making it harder for attackers to derive meaningful information. Yet, modern **deep learning classifiers**—owing to their superior pattern recognition capabilities—can often distinguish between genuine cryptographic executions and obfuscated traces.

Consequently, DL-based attacks can successfully identify patterns even in artificially randomized or noise-augmented traces, **significantly reducing the effectiveness** of conventional hiding methods. This highlights the need for next-generation countermeasures that are **AI-aware**, **context-adaptive**, and **hardware-integrated**, ensuring resilience against both classical and machine-learning-enhanced SCAs.

---

#### The Transformation of Side-Channel Analysis through Machine Learning

Machine learning (ML) has revolutionized side-channel analysis (SCA) by enabling attackers to process and interpret large datasets of leakage traces with unprecedented efficiency. Traditional SCA relied heavily on manual feature engineering and statistical correlation analysis, which demanded expert knowledge and extensive preprocessing. In contrast, artificial intelligence (AI) and deep learning (DL) approaches automate this process—identifying subtle dependencies within power consumption, electromagnetic (EM) emissions, and timing signals that correspond to secret cryptographic information.

Key advances include:

- **Pattern Recognition via Deep Learning:** Neural networks can detect minute correlations in side-channel data, making AI-driven SCAs significantly more effective than traditional statistical approaches.
- **Automated Attacks:** AI systems can automatically adapt to varying cryptographic implementations without manual feature selection, thereby improving attack efficiency and generalization across devices.

AI has thus enhanced SCA by automating and optimizing the extraction of sensitive cryptographic information. Convolutional Neural Networks (CNNs) can recover encryption keys directly from power traces or EM emissions without prior leakage modeling, while Recurrent Neural Networks (RNNs) and Long Short-Term Memory (LSTM) models improve time-series analysis for timing attacks. Moreover, AI systems can dynamically tune attack parameters—such as the number of traces or sampling rates—to maximize success. By understanding how countermeasures alter side-channel leakage, attackers can even train AI models to circumvent defenses such as noise injection or masking.

#### Countermeasures Against AI-Enhanced Side-Channel Attacks

Several defensive strategies have been developed to mitigate the risk posed by SCAs:

- **Hiding Techniques** aim to randomize power consumption patterns through noise insertion or instruction reordering, making meaningful correlations harder to detect.
- **Blinding Methods** mathematically alter intermediate encryption computations to obscure relationships between intermediate and final results.
- **Masking Techniques** introduce random variables during computation to disrupt the statistical dependencies exploited by attackers.

While these countermeasures are effective against conventional SCAs, they are increasingly challenged by deep learning–based attacks capable of adapting to complex and nonlinear leakage patterns. Recent studies propose **hybrid countermeasures** that combine multiple layers of defense to improve resilience against AI-driven SCAs. Examples include **dynamic power management**, **randomized instruction execution**, and **adaptive noise injection**, each complementing traditional hiding and masking approaches. The overall effectiveness of such composite defenses depends on implementation efficiency, resource overhead, and adaptability to evolving attack methodologies.

#### AI as a Defensive Tool Against Side-Channel Threats

AI itself is also emerging as a powerful ally in defending against SCAs. Key applications include:

**(A) AI-Driven Anomaly Detection**

Machine learning models can monitor cryptographic operations for deviations from expected execution patterns, signaling potential attacks.

- **Supervised Learning:** Models trained on normal encryption traces can detect anomalies introduced by SCAs.
- **Unsupervised Learning:** Clustering algorithms such as _k-means_ and _autoencoders_ can uncover unknown attack patterns within power or timing data.

AI-based Intrusion Detection Systems (IDS) leverage these methods to monitor power and EM signals, providing early warnings of side-channel exploitation attempts.

**(B) AI-Based Behavioral and Adaptive Defense**

AI can model and track cryptographic behavior, identifying irregularities indicative of malicious interference. Reinforcement learning–based defense systems can dynamically adjust encryption parameters in real time to counter ongoing attacks.

#### AI for Data Obfuscation and Leakage Prevention

AI can be leveraged to dynamically modify cryptographic implementations, increasing the difficulty for attackers to extract sensitive data.

- **AI-Assisted Noise Injection:** ML models can inject targeted randomness into timing, power, or EM signals to disrupt learned attack patterns. Adversarial AI techniques can generate carefully crafted noise to confuse deep learning–based attackers.
- **Adaptive Randomization of Execution:** AI-driven control systems can dynamically vary execution order or memory access patterns, reducing the consistency of side-channel emissions. For example, AI-controlled execution jitter can modify encryption operations at runtime to resist timing-based attacks.
- **AI-Optimized Masking:** Machine learning algorithms can tune masking schemes automatically to balance between computational overhead and security strength, optimizing random variable insertion for maximum protection.

#### AI-Optimized Hardware Security

AI can also enhance hardware-level defenses against SCAs by optimizing power and design configurations:

- **Reinforcement Learning for Power Control:** AI systems can dynamically configure voltage and clock frequencies (e.g., through Dynamic Voltage Scaling, DVS) to minimize information leakage and resist power analysis attacks.
- **AI-Assisted Physical Security Audits:** Neural network–based layout analysis can identify potential hardware design vulnerabilities that may leak cryptographic information, aiding in the prevention of side-channel leakage in post-quantum cryptographic (PQC) implementations.

#### Real-Time AI-Enabled Monitoring and Response

AI can enable **real-time detection, mitigation, and recovery** mechanisms to proactively defend against SCAs:

- **AI-Driven Intrusion Detection Systems (IDS):** These systems continuously monitor cryptographic operations and trigger automated countermeasures—such as halting encryption or switching to backup algorithms—upon detecting suspicious activity.
- **Automated Vulnerability Patching:** AI can autonomously identify weaknesses in cryptographic software or hardware and propose or apply targeted security patches.

---

#### Simulation and Modeling Tools for Side-Channel Analysis

Artificial intelligence (AI)–driven anomaly detection offers scalability and adaptability, making it a promising candidate for securing post-quantum cryptography (PQC) implementations. As quantum computing advances, schemes such as **CRYSTALS-Kyber** and **SPHINCS+**, selected by NIST for PQC standardization, must also withstand emerging _AI-augmented side-channel attacks (SCA)_. Researchers have demonstrated that integrating **AI-optimized masking and shielding** techniques can increase attack complexity—requiring up to **five times more traces** for successful key extraction—while maintaining negligible computational overhead. Moreover, reinforcement learning has enabled **AI-controlled dynamic voltage scaling (DVS)** on PQC processors, allowing real-time power adjustments that minimize side-channel leakage.

A critical tool for evaluating SCA vulnerabilities is **ELMO**, an open-source simulator specifically designed for modeling power consumption in embedded systems. ELMO enables researchers to analyze the impact of different cryptographic implementations under controlled environments, providing precise insights into potential leakage sources. By categorizing assembly instructions—such as arithmetic logic unit (ALU) operations, load/store operations, and shift operations—ELMO enhances the accuracy of power trace prediction.

Recent updates have integrated **machine learning–based analysis** into ELMO, allowing it to simulate realistic side-channel behaviors more effectively. These AI-enhanced predictive models facilitate the evaluation of both **attack methods and defense strategies**, accelerating the development of adaptive countermeasures. By leveraging ELMO and similar simulation frameworks, researchers can refine cryptographic implementations and test mitigation strategies against dynamically evolving threat landscapes.

#### FPGA and Dynamic Hardware Defenses

From a hardware perspective, traditional **static defense mechanisms** often fail to fully mitigate SCAs, as they lack the ability to adapt dynamically to evolving attack patterns. **Field-Programmable Gate Arrays (FPGAs)**, known for their flexibility and reconfigurability, are particularly vulnerable to SCA but also provide unique opportunities for defense. The **Dynamic Partial Reconfiguration (DPR)** feature of FPGAs allows different hardware circuits to be loaded at runtime, effectively randomizing side-channel signatures.

Furthermore, FPGAs excel in **accelerating AI and ML models**, providing enhanced performance and energy efficiency. This synergy enables **AI-driven adaptive hardware security**, where deep learning models can monitor and reconfigure FPGA logic in real time to obscure or disrupt leakage patterns.

#### Hyperparameter Optimization in Deep Learning–Based SCA Research

In deep learning–based SCA studies, **hyperparameters**—including neural architecture parameters, activation functions, and learning rates—play a critical role. Architectural hyperparameters define the model’s structure (e.g., depth, layer type, kernel size, stride, and connectivity patterns such as residual or multi-head attention). These parameters govern the model’s representational capacity and inductive bias, directly influencing feature extraction and hierarchical learning.

Hyperparameter optimization is thus a major focus in the SCA research community.

- **Perin et al.** employed **grid search** for hyperparameter tuning.
- **Wu et al.** introduced a hybrid approach combining **Bayesian optimization** and **random search** to improve efficiency.

However, each method has trade-offs: Bayesian optimization struggles with high-dimensional or noisy evaluations, random search lacks sample efficiency, and grid search becomes computationally infeasible in constrained environments. The search for optimal hyperparameter tuning remains an open challenge, especially for mixed discrete–continuous parameter spaces typical in SCA analysis.

#### Balancing Security and Performance

Integrating AI into cryptographic systems introduces new challenges related to **latency, power consumption, and resource utilization**, particularly for embedded and IoT devices. Achieving a balance between **security and performance** is critical. Moreover, **AI models trained on one cryptographic implementation** often transfer effectively to similar systems—meaning that even minor hardware variations (e.g., between microcontrollers) may not prevent AI-driven SCAs.
This underscores the need for **adaptive and generalizable defenses** capable of resisting **transfer learning–based attacks**.

#### The Need for Standardization and Explainable AI

A major limitation in current research is the **lack of standardized AI security frameworks** for PQC. Without consensus on how to integrate AI into post-quantum security architectures, defenses remain fragmented and inconsistent. To ensure trust and transparency, the field must embrace **Explainable AI (XAI)**—techniques that make AI decisions interpretable and auditable.

XAI can enhance anomaly detection and adaptive mitigation by revealing _why_ certain operations are flagged as potential attacks. Responsible AI development must also account for **privacy, fairness, and transparency**, ensuring that AI-based tools for cryptographic protection cannot be easily repurposed by adversaries.

---

## 3. Side-Channel Attacks on Neural Networks: From Key Recovery to Model Extraction

Side-channel attacks have long been a practical threat to cryptographic implementations—exploiting physical leakage (timing, power, electromagnetic emissions, etc.) to recover secret keys. In recent years, researchers have extended these techniques beyond cryptography to target machine learning models running on edge and embedded hardware. Work in this area falls broadly into two categories: **neural network architecture recovery** and **input/weight recovery**. Below we summarize representative advances and attack vectors.

#### Memory and Access-Pattern Leakage

One early line of work demonstrated how memory access patterns leak structural details about a model even when off-chip memory is encrypted. Hua et al. built a proof-of-concept using a custom CNN accelerator synthesized with Vivado HLS and showed that observing off-chip memory accesses allows an attacker to infer the network’s architecture. Their experiments highlight that encryption alone is not sufficient when access patterns themselves carry model information.

#### Timing and EM Attacks on Microcontrollers

Batina et al. explored timing and electromagnetic (EM) side channels in a gray-box setting to extract neural network structure and weights from models running on microcontrollers. Their attacks were validated on two widely used modern microcontrollers and demonstrated that even on constrained devices, timing and EM traces reveal rich information that can be exploited to reconstruct networks.

#### EM and Adversarial-Probe Techniques for BNNs

Subsequent work targeted binary neural network (BNN) accelerators using EM measurements combined with margin-based adversarial techniques. By crafting specific inputs designed to elicit distinguishing side-channel responses, attackers were able to recover the architecture and weights of BNN accelerators. These experiments also showed that carefully constructed adversarial queries can be used to reconstruct surrogate models with high fidelity.

#### SPA and Timing for Diverse Numeric Formats

Maji et al. extended side-channel recovery approaches to cover a range of numeric precisions. Using timing analysis and simple power analysis (SPA), they recovered information about inputs and models implemented in fixed-point, floating-point, and binary representations. Their experiments targeted multiple microcontroller platforms, demonstrating the broad applicability of SPA- and timing-based recovery across deployment scenarios.

#### FPGA and RO-Based Power Monitoring Attacks

Other teams focused on FPGA-based implementations. Yoshida et al. demonstrated weight recovery against custom neural-network implementations on FPGAs built from systolic-array primitives. A particularly creative attack used a local, on-chip ring-oscillator (RO) power monitor to capture subtle power curves: on a Zedboard, attackers used this remote power-sensing approach to distinguish different networks when running the CMSIS-NN-backed NNoM library on 32-bit microcontrollers. The implication is that even modest on-chip sensors or observables can betray model identity.

#### Model Extraction on Standard Networks

Across multiple studies, attacks have successfully reconstructed surrogate models for classical networks such as LeNet and AlexNet. These reconstructions often reach useful accuracy levels, showing that side-channel methods can yield practically useful models—not only theoretical leaks.

#### Target Platforms and Commercial Accelerators

So far most published attacks have targeted microcontrollers or custom accelerators implemented via Vivado HLS. However, the same techniques generalize to widely deployed commercial edge accelerators—Google’s TPU, Intel’s NCS, and NVIDIA’s NVDLA are natural future targets. The underlying taxonomy of exploitation—observing timing, power, EM, and memory-access behavior—applies across diverse hardware platforms.

#### Two Categories of Side-Channel Threats

In sum, side-channel attacks on machine learning systems fall into two broad and complementary classes:

- **Neural network architecture recovery** — inferring the model’s topology (layers, dimensions, sparsity patterns) from observable execution traces or memory-access patterns.

- **Input and weight recovery** — extracting either the model parameters or sensitive inputs by analyzing power, EM, timing, or other leakages, often augmented by carefully crafted probe inputs.

The body of work surveyed here shows that side channels are a real and practical threat to machine learning deployments at the edge. Even when standard protections—such as off-chip memory encryption—are in place, the patterns of computation and memory usage, along with analog leakages, can enable reconstruction of models and inputs. Defending against these attacks will require a combination of hardware-level countermeasures (noise injection, access-pattern obfuscation, constant-time implementations), system-level strategies (partitioning and isolation), and algorithmic robustness (privacy-preserving inference techniques).

<details><summary>Code</summary>

```ALGORITHM
ALGORITHM 1: Max-Min sliding window downsampling
Input: T    // array consisting of captured trace data
Output: S   // array consisting of downsampled trace data
Data: tlen  // denotes the length of captured trace
      slen  // denotes the length of downsampled trace

1  i := 0
2  final_samples := slen/2    /* denotes the length of samples considered in a window */
3  samples_per_block := tlen/final_samples
4  current_block_pos := 0
5  while i < (final_samples * 2 - 2) do
6      j := 0
7      Wmax := INT_MIN
8      Wmin := INT_MAX
9      /* find maximum and minimum in the current window */
10     while j < samples_per_block do
11         current_value := T[current_block_pos + j]
12         if current_value > Wmax then
13             Wmax := current_value
14         if current_value < Wmin then
15             Wmin := current_value
16         j := j + 1
17     /* save the datapoints */
18     if i % 2 == 0 then
19         S[i] := Wmax
20     else
21         S[i] := Wmin
22     /* slide window with overlapping half points */
23     current_block_pos := current_block_pos + (samples_per_block/2)
24     i := i + 1
25  return S
```

---

### Side-Channel Vulnerabilities in Machine Learning Models with Embedding Tables and the Security Promise of Deep Hash Embedding

Recent research has demonstrated that side-channel attacks, traditionally used to extract cryptographic keys, can also be applied to **machine learning (ML)** models. For example, during ML inference, **memory access patterns**—especially those optimized by dynamic pruning—can leak sensitive information about a model’s **architecture** and **parameter values**. In some implementations, such access patterns may even expose **output labels**. These findings highlight the potential risks of side-channel leakage in ML systems and have motivated new defense mechanisms such as **oblivious computation**. However, most prior work has focused on ML models **without embedding tables**, such as fully connected (FC) or convolutional neural networks (CNNs).

Modern ML models, by contrast, must handle both **continuous** and **categorical (discrete)** feature values. For instance, **deep learning recommendation models (DLRMs)** rely on categorical user features to generate personalized recommendations, while **large language models (LLMs)** process discrete word or subword tokens. To represent such categorical inputs in continuous vector spaces, ML models use **embeddings**, which are high-dimensional vectors capturing semantic relationships among features. These embeddings are typically retrieved through **lookup operations** from large **embedding tables**.

Unfortunately, embedding lookups introduce **memory-based side-channel vulnerabilities**. Specifically, an attacker observing the memory access pattern can infer which table indices were queried—indices that directly encode input feature values. In LLMs, this means that if token IDs leak, so do the corresponding words or subwords; in DLRMs, the leakage of embedding table indices can expose private user attributes.

#### Embedding in LLMs and DLRMs

LLMs have become the dominant architecture for natural language processing tasks such as semantic search and text generation. A typical **Transformer**-based LLM first converts text into discrete **subword tokens** using a tokenizer. Each token ID is then mapped to a vector through an **embedding table lookup**, summed with **positional encodings**, and passed through multiple layers of attention and feedforward networks. The output can represent semantic features or, after being projected through an output layer (often weight-shared with the embedding table), produce logits for the next token in autoregressive text generation.

This generation process proceeds in two phases:

- **Prefill phase** — the model processes the full input prompt $([t_1, …, t_N])$ and predicts the next token’s probability distribution $(P(t_{N+1}|t_1,…,t_N))$.
- **Decoding phase** — new tokens are generated iteratively, each time conditioning on all previously generated tokens.

Because the decoding phase re-feeds the growing sequence through the model, it is **computationally expensive**, and its embedding layer operates at a smaller batch size but higher frequency.

By contrast, DLRMs employ multiple embedding tables of different sizes to represent sparse categorical features, while LLMs typically maintain a single large token embedding table—often containing tens of thousands of entries with embedding dimensions ranging from 768 to 8192.

#### Lookup-Based vs. Computation-Based Embeddings

Most production systems still use **storage-based embedding lookup**, where each feature ID directly indexes a pre-trained embedding vector. In DLRMs, however, these embedding tables can grow to **gigabytes or even terabytes**, leading to inefficiencies in both memory and latency.

To address these issues, researchers have proposed **computation-based embeddings**, notably **Deep Hash Embedding (DHE)**. Instead of direct table lookup, DHE computes embeddings through **parameterized hash functions** followed by a **fully connected (FC)** layer. This approach significantly reduces memory footprint but introduces additional computation overhead, which has limited its deployment in real-world systems. Moreover, systematic studies of DHE in LLMs remain scarce.

Despite its performance advantages, the lookup-based method suffers from a critical **security flaw**: the embedding index lookup process inherently reveals **memory addresses**, which can be exploited by **side-channel attacks** (e.g., cache timing analysis).

Efforts to mitigate this include **Tensor Train (TT) decomposition**, which factorizes large embedding tables into smaller matrices to combine lookup and computation. Yet, since lookups still occur, **address information remains exposed**—rendering the method insecure against side-channel threats.

#### ORAM and Hardware-Level Defenses

A classical countermeasure is **Oblivious RAM (ORAM)**, which randomizes memory access patterns to conceal which addresses are being accessed. Variants such as **Look-Ahead ORAM** optimize for DLRM training, where data order is known in advance. However, during **inference**, inputs are unpredictable, making effective ORAM deployment challenging. While ORAM guarantees strong privacy, it typically incurs **severe performance overheads (10–50× latency increase)**.

At the hardware level, several mitigation strategies have been explored:

- **Cache and TLB defenses**: static/dynamic partitioning and randomized set mapping to prevent cache-based inference.
- **Processor frontend protections**: partitioning or randomizing control-flow execution to resist speculative-execution leaks.
- **Memory controller and DRAM countermeasures**: pattern shaping or hardware-level ORAM implementations, though these also introduce large delays.
- **Intel SGX (TEE) vulnerabilities**: even trusted execution environments can leak access patterns via page faults. Countermeasures such as isolation and obfuscation offer partial protection.
- **Address-space randomization**: static randomization (offsets) offers limited entropy; dynamic schemes like **Morpheus** or systems such as **ObfusCuro**, **Raccoon**, and **MoLE** introduce frequent relocations or ORAM-based protections, but with **10×–400× performance penalties**.

Overall, existing defenses cover only specific attack classes and often require **hardware modifications**, making them impractical for most production environments.

#### Deep Hash Embedding: A Security-Oriented Alternative

Originally proposed to **reduce memory usage and accelerate inference** in DLRMs (e.g., in the MP-Rec system), **Deep Hash Embedding (DHE)** offers inherent side-channel resistance. DHE uses a **deterministic hash function** to map an input ID to a fixed set of numeric values, which are then passed through an **FC network** to generate the embedding vector.

Formally:

$E(x) = f(W_2 \cdot \sigma(W_1 \cdot h(x)))$

where $(h(x) = (((a x + b) \bmod m) \bmod p))$ is a deterministic hash function composed of arithmetic operations only (multiplication, addition, modular reduction), independent of input data branching.

Because the entire computation path—hashing, scaling, and matrix multiplications—uses **deterministic and data-independent operations**, its **memory access pattern remains constant** across inputs. Modern deep learning libraries (e.g., PyTorch) implement activation functions such as ReLU using **SIMD/AVX intrinsics**, which execute without conditional branching (`if` statements). As a result, the execution flow is **data-oblivious** and **constant-time**, making DHE naturally resistant to side-channel analysis.

<details><summary>Code</summary>

```Algorithm
Algorithm 2: Dataset Synthesis
Input: The probability of noise p, the standard deviation of duration σ,
       the corpus dataset C, and the size of LLM vocabulary |V|
Output: The synthesized training dataset T
       // U denotes uniform distribution, N is normal distribution
       // s(·) is a sample

1  for c ∈ C do
2      Init L ← φ                  // Generated cache trace
3      Init ctime ← 0              // Current timestamp
4      Init mtime ← |c| + 1        // Maximum length
5      for token ∈ c do
6          if s(U[0,1]) ≥ p then
7              L ← L ∪ {(ctime, token)}
8          end
9          if s(U[0,1]) < p then
10             L ← L ∪ {(s(U[0,mtime]), s(U[0,|V|-1])}
11         end
12         ctime ← ctime + s(N(1,σ²))    // Simulate the periodicity of decode phases
13     end
14     T ← T ∪ {(L[0],L[1],c)}          // Get a (TD,KD,O)
15 end
```

</details>

---

[Nvdla](https://github.com/nvdla/hw/tree/nv_small)
[Nvdla_SW](https://github.com/nvdla/sw)
[Chipwhisperer](https://github.com/newaetech/chipwhisperer)
[SCAlib](https://github.com/simple-crypto/SCALib)
[SecEmb_DHE](https://github.com/bearhw/SecEmb_DHE)
