# ETAAcademy-Audit: 26. Physical Unclonable Function

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>26 PUF</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>PUF</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# Quantum PUF, AI, and Blockchain: Unclonable Hardware Security

Physical Unclonable Function (PUF) is a hardware security primitive that uses random manufacturing variations to create a device‑unique “fingerprint,” allowing a root key to be reconstructed on demand—without storing secrets—via a challenge–response protocol plus error correction and privacy amplification.

Common implementations include SRAM, Ring Oscillator (RO), and Arbiter PUFs. Engineering involves selecting an entropy source, designing readout/comparison circuits, validating with Monte Carlo statistics, and enforcing symmetric, well‑isolated layouts; newer directions explore memristor and optical PUFs, hybrid designs, and integration with blockchain, quantum sensing, and AI.

Quantum PUFs (QPUFs) leverage the no‑cloning principle and hardware noise: on real quantum chips, X/Ry/H gates with CNOT entanglement yield distinctive output distributions driven by T1/T2 coherence, gate fidelities, and readout errors, and can pair with blockchain/IIoT authentication and session protocols.

For secure AI at the edge and in IIoT, systems must boost robustness while resisting CRP‑based modeling attacks. On low‑power ReRAM compute‑in‑memory (CIM), a PUF‑guided scheme ensures models run correctly only on matching devices: inference protection (selects real vs. fake weights), weight protection (Bipartite‑sort scrambles and fragments positive/negative parts), and input protection (PUF‑driven MSB/LSB reordering with bit‑serial injection)—delivering low energy, no stored keys, and suitability for resource‑constrained environments.

---

## 1. Physical Unclonable Functions (PUFs): Principles, Implementations, and Modern Applications

A **Physical Unclonable Function (PUF)** is a hardware security primitive that acts as a **digital fingerprint** of a semiconductor device. It derives uniqueness from the inherent manufacturing variations present in integrated circuits (ICs), enabling each chip to generate device-specific responses that are extremely difficult—practically impossible—to clone. PUFs have become an important foundation for hardware security, with applications ranging from anti-counterfeiting and authentication to key generation, secure communication, oblivious transfer, key exchange, key derivation, and even verifiable computation.

### The Origin of Uniqueness: Semiconductor Process Variation

Deep-submicron semiconductor manufacturing introduces inevitable random variations in device parameters such as:

- Transistor threshold voltage ($V_{th}$)
- Mobility and gain factor
- Oxide thickness
- Channel length/width variations
- Interconnect resistances
- Path delays

Although these variations are undesirable for traditional circuit design, they provide high-entropy randomness for security.

Each transistor behaves slightly differently even when designed to be identical. These microscopic differences accumulate to create measurable variations in circuit behavior. PUF circuits intentionally amplify these variations to generate **device-unique bit patterns**, which serve as the “biometric fingerprint” of each IC.

### Challenge–Response Mechanism

Unlike traditional cryptographic systems that store secret keys in non-volatile memory, a PUF generates keys **on demand**. A PUF takes an input stimulus called a **challenge**, and produces an output called a **response**:

- **Challenge (C)** → input stimulus
- **Response (R)** → device-unique output pattern based on physical variations

The mapping ( $C \rightarrow R$ ) is unique per device and extremely difficult to duplicate.

Two categories of PUFs exist:

**Weak PUFs**

- Produce only a small number of responses (e.g., 128–256 bits)
- Used mainly for **key generation**
- Examples: SRAM PUF, Ring Oscillator PUF

**Strong PUFs**

- Provide an exponential number of challenge–response pairs (CRPs)
- Suitable for **authentication protocols**
- Examples: Arbiter PUF, XOR-Arbiter PUF, FF-PUF, lightweight PUFs

Because the key is not stored anywhere on the chip—only **reconstructed when needed**—PUFs offer excellent resistance against invasive attacks.

### PUF Output Processing: Error Correction and Privacy Amplification

The raw PUF response contains slight noise due to temperature, voltage, and aging effects. Therefore, PUF-based key generation must include:

- **Error Correction (Fuzzy Extractor):**
  Ensures reproducibility: the same key is reconstructed each time despite noisy PUF outputs.

- **Privacy Amplification:**
  Converts the partially random PUF pattern into a **fully uniform cryptographic key**, removing statistical bias.

These two steps ensure that PUF-derived keys meet modern cryptographic standards.

### Major Types of PUFs

**SRAM PUF (Most Widely Used)**

When powered on, each SRAM cell preferentially settles to either 0 or 1.
This preference is determined by tiny transistor mismatch in the cross-coupled inverters.
Reading the SRAM at boot produces a stable device-unique bitstring.

**Ring Oscillator (RO) PUF**

- Fabricate arrays of ring oscillators
- Due to delay variations, each RO oscillates at a slightly different frequency
- Compare frequencies of RO pairs:

  - RO1 > RO2 → output 1
  - RO1 < RO2 → output 0

**Arbiter PUF**

- Two symmetric delay paths receive a simultaneous input pulse
- Manufacturing variations cause one path to arrive earlier
- An arbiter outputs which path was faster
- Challenges reconfigure the paths → exponential CRP space (strong PUF)

### How to Build a PUF: Practical Design Flow

**Step 1: Identify a Physical Entropy Source**

Examples:

- SRAM power-up imbalance
- RO frequency variation
- CMOS delay differences
- Interconnect resistance randomness
- Memristor (ReRAM) switching stochasticity

**Step 2: Design Readout and Extraction Circuitry**

- SRAM read circuitry
- Frequency counters and comparators for RO-PUF
- Delay measurement and arbiters for Arbiter-PUF
- ADC readout for analog PUFs (e.g., ReRAM PUF)

**Step 3: Monte Carlo Simulation**

Use Cadence ADE XL:

- Run 100–1000 Monte Carlo samples
- Observe bit probability (0/1 bias)
- Evaluate:

  - **Uniqueness** (inter-chip difference)
  - **Reliability** (intra-chip stability)
  - **Robustness** against PVT variation

**Step 4: Layout Design**

- Ensure symmetry (critical for Arbiter PUF)
- Minimize noise coupling
- Stabilize power supply and temperature sensitivity

**Step 5: Fabrication and Silicon Testing**

Real chips exhibit random behavior due to actual process variation—confirming uniqueness and stability.

### Beyond Classical PUFs: Emerging Architectures

- **XOR Arbiter PUFs** enhance security by combining multiple APUF outputs
- **Memristor / ReRAM PUFs** exploit stochastic ionic filament formation
- **Optical PUFs** use laser scattering in complex material structures
- **Hybrid PUFs** (e.g., SRAM + ReRAM) improve robustness
- **Blockchain-integrated PUF systems** provide Sybil-resistant device identity
- **Quantum-resilient PUF protocols (QPUF)** explore future-proof hardware security
- **AI-enhanced PUF frameworks** dynamically optimize reliability under PVT variation

A notable direction is **Adaptive PUF Systems**:
AI algorithms monitor temperature, voltage, and aging, then select the optimal PUF type and auto-tune error correction. This “Dynamic Error Correction Unit (DECU)” enables high reliability even under severe environmental changes.

---

## 2. Quantum Physical Unclonable Functions (QPUFs)

Quantum Physical Unclonable Functions (QPUFs) are emerging primitives designed to create **unique, unclonable fingerprints for quantum processors**, leveraging the inherent randomness of quantum-mechanical behavior and hardware-level variations. By exploiting the unique physical properties of quantum devices—such as qubit coherence times, decoherence rates, crosstalk, and gate errors—QPUFs provide a foundation for secure quantum information processing and authentication. They rely on fundamental physical laws: the **quantum no-cloning theorem**, which states that an unknown quantum state cannot be copied, and **Heisenberg’s uncertainty principle**, which limits the precision with which quantum variables can be simultaneously known. These principles make QPUFs intrinsically more secure than classical PUFs.

### Hardware Variation as the Source of QPUF Entropy

QPUFs exploit variations in quantum hardware parameters, such as:

- **Coherence and decoherence times (T1, T2)**
- **Qubit–qubit crosstalk**
- **Intrinsic resonance frequencies**
- **Gate fidelity and over-/under-rotation**
- **Readout noise and assignment error**

For example, in superconducting transmon qubits, crosstalk alters the absolute resonance frequency of qubits. A QPUF signature can be generated using **Ramsey interferometry**, where noise introduced by neighboring qubits shifts resonance frequencies in unpredictable ways, producing chip-specific responses.

Another architecture uses decoherence and entanglement to produce unique binary bitstreams. Evaluations show that QPUF responses generated using standard quantum gates—Ry, CNOT, Pauli-X, and Hadamard—are reliable and stable under repeated measurements.

Additional research includes:

- **NeoPUF**, which stores PUF characteristics in ultra-thin oxide layers utilizing manufacturing variation in oxide thickness.
- **Quantum-circuit PUFs**, which rely on tunable rotation angles in Ry gates to introduce unique device-specific randomness.
- **Blockchain-enabled IIoT frameworks**, where PUFs serve as unclonable device identities for secure consensus (HPCchain).
- **PUF-based protocols for secure machine-to-machine (M2M) communication** in Industrial IoT, such as the **PEASE protocol**, which supports low-power, low-overhead authentication.
- **Pseudo-PUFs combined with lightweight encryption** for ultra-low-power IIoT systems.
- **Quantum-inspired random number generators (QRNGs)** implemented on quantum simulators and real quantum hardware for secure communication.
- **Blockchain-based quantum IoT (QIoT)**, which uses entanglement to verify sensor data integrity.

### Eight-Qubit QPUF Architecture

A typical QPUF implementation uses an **eight-qubit quantum circuit** combining single- and two-qubit gates. Quantum Hadamard, Ry, and CNOT gates are used to evaluate the circuit’s sensitivity to hardware-level variations. The architecture operates as follows:

**(1) Random Initialization — X Gate**

The **X gate** acts as a NOT gate.

- If the qubit starts in (|0⟩), applying **X** produces (|1⟩).
- If it starts in (|1⟩), applying **X** produces (|0⟩).

Because real quantum hardware has device-specific imperfections, each qubit’s actual initial state after the X gate carries slight, unique physical variations. This becomes the **initial entropy source** for the QPUF.

**(2) CNOT — Entangling the First Four and Last Four Qubits**

A **CNOT gate** copies (controls) the state of the control qubit onto the target qubit, creating **entanglement**.
Once qubits are entangled, they no longer behave independently: noise, drift, and hardware variations propagate jointly through the entangled pair.

This _amplifies_ tiny hardware differences → making the device easier to uniquely identify.
Thus, entanglement enhances the discriminability needed for a strong QPUF.

**(3) Ry(θ) — Controlled Randomness Through Rotation**

The **Ry(θ)** rotation gate places a qubit into a rotated superposition. On real hardware, physical variations introduce:

- phase errors
- over-rotation / under-rotation
- crosstalk
- decoherence

So, even if two quantum chips run the same Ry(θ), the measured statistical distributions differ. This produces a hardware-unique “quantum fingerprint”.

$$
R_y(\theta) =
\begin{bmatrix}
\cos(\theta/2) & -\sin(\theta/2) \\
\sin(\theta/2) & \cos(\theta/2)
\end{bmatrix}
$$

**(4) Hadamard — Constructing an Equal 0/1 Superposition**

The **Hadamard gate** produces a balanced superposition:

$$
H|0\rangle = \frac{|0\rangle + |1\rangle}{\sqrt{2}}
$$

This increases entropy and greatly amplifies phase noise.
When a control qubit in superposition feeds into multiple CNOT operations, the 8 qubits form a **globally entangled superposition** that is _extremely sensitive_ to hardware-level differences.

**(5) Measurement — Extracting the Probability Distribution (PUF Response)**

Finally, all qubits are **measured**, producing an 8-bit binary string for each run.
Repeating the circuit (e.g., **1024 shots**) gives a probability histogram.

Different quantum devices exhibit _distinct_ output probability distributions because of their unique noise patterns and hardware characteristics.

This histogram is the QPUF’s **challenge–response pair (CRP)**.

### Why QPUFs Are Unclonable

The uniqueness of a QPUF is derived from deep physical characteristics of the quantum hardware:

- **T1 (Energy Relaxation)**
  Determines how long a qubit remains in the |1⟩ excited state; varies ±20% across qubits.

- **T2 (Phase Coherence Time)**
  Governs phase stability—crucial for Ry and H gates—and differs across chips.

- **Gate Fidelity Variations**
  CNOT fidelity varies widely (0.97–0.99), making entanglement behavior highly hardware-specific.

- **Qubit Resonance Frequencies (~5 GHz)**
  Small frequency offsets alter rotation accuracy and phase accumulation.

- **Anharmonicity Differences**
  Affects susceptibility to microwave crosstalk and off-resonant excitations.

- **Readout Noise (1%–5%)**
  Each qubit’s readout amplifier introduces unique measurement biases.

These factors create a fingerprint that is:

- **Unique:** No two quantum processors behave identically.
- **Unpredictable:** Based on microscopic, uncontrollable quantum-level imperfections.
- **Unclonable:** Guaranteed by both hardware physics and quantum no-cloning laws.
- **Robust:** Stable within the same device across repeated measurements.

<details><summary>Code</summary>

```Algorithm

Algorithm 1: QPUF circuit evaluation.
    Input: Qubits
    Output: Job String
    1:  initialize Qubits in QPUF circuit Randomly (Varying Initializations)
        Example:
        Qubit 0 → 0, Qubit 1→1, Qubit 2→1, Qubit 3→0, Qubit 4→0, Qubit 5→1, Qubit 6→1,
        Qubit 7→0, Qubit 8→0
    2:  Entangle Qubits using CNOT gate
        q0–>q4, q1–>q5, q2–>q6, q3–>q7
    3:  Apply Ry gate to control qubits with predefined angles
        qc.ry(𝑎𝑛𝑔𝑙𝑒𝑖)–> q0, qc.ry(𝑎𝑛𝑔𝑙𝑒𝑖)–> q1, qc.ry(𝑎𝑛𝑔𝑙𝑒𝑖)–> q2, qc.ry(𝑎𝑛𝑔𝑙𝑒𝑖)–> q3
    4:  Apply Hadamard gate to control qubits to create a superposition
        qc.h(q0), qc.h(q1), qc.h(q2), qc.h(q3)
    5:  Apply Measurement gate to measure the quantum states of qubits
    6:  Obtain IBM Quantum Application Programming Interface (API) token
    7:  Choose the quantum backend
    8:  Specify measurement counts for a job
    9:  Execute circuit
    10: Obtain jobs strings which a unique job string obtained from all 8 qubits for 1024 shots
        shot 1: 1010110, shot 2:0010101......

Algorithm 2: Workflow of proposed QPUF noise-suppression mechanism.
    Input: Initialization Parameters 𝑘𝐼
    Output: Most Reliable QPUF Response
    1:  initialize Qubits (Varying Initializations)
        Example:
        Qubit 0 → 0, Qubit 1→1, Qubit 2→1, Qubit 3→0, Qubit 4→0, Qubit 5→1,
        Qubit 6→1, Qubit 7→0, Qubit 8→0
    2:  Choose fixed initialization parameters for all the instances of jobs on backend 𝑏1
    3:  Choose Ry gate to all Control qubits with a predefined set of initialization angles after entangling
    4:  Execute 5 sets of angles for all sets of initializations
        Ry Angle→pi/4, pi/2, pi, 3*pi/2, 2*pi
    5:  Apply Measurement Gate(M) to measure the quantum states of all qubits
    6:  Obtain the most frequently occurring measuring outcome as job string
    7:  for For a job 𝑗𝑖 in an instance 𝑖1 with 1024 shots do
    8:  Choose the most frequent outcome obtained from all shots as job string (10101011:5, 11001101:6..)
    9:  Obtain the job strings for all jobs 𝑗1 in instance 𝑖1
    10: end for
    11: Extract results string from all job instances 𝑖𝑛
    12: Calculate the Reliability of all job strings in instances 𝑖𝑛
    13: if Job string 𝑗𝑖 obtained is frequently occurring in all instances 𝑖𝑛 then
    14:     Choose 𝑗𝑖 as the QPUF response 𝑟𝑖 for backend b and initialization parameter 𝑐𝑖
    15: end if
```

</details>

---

## 3. Federated Learning, Blockchain, PUF and AI Integration for Secure Edge Intelligence

Traditional machine learning (ML) techniques rely heavily on centralized cloud servers for model training. This raises significant concerns regarding data privacy and often faces resistance from data owners. **Federated Learning (FL)** has emerged as a privacy-preserving, distributed ML paradigm that enables multiple parties to collaboratively train models without sharing their raw data. Meanwhile, **blockchain**—with its decentralization, immutability, and traceability properties—provides a secure ledger layer for FL-driven **smart edge computing (EC)** environments.

However, blockchain’s distributed architecture is not naturally aligned with the constraints of EC. Massive data volumes increase the cost of managing blockchain nodes and storage. The resulting large number of data blocks leads to unacceptable verification and propagation delays in EC environments, and storing duplicated IoT data across all nodes causes inefficient use of storage resources.

To enhance security at the hardware level, the **Physical Unclonable Function (PUF)** has been widely adopted. A PUF exploits intrinsic manufacturing variations in electronic devices to generate unique, device-specific outputs—serving as cryptographic fingerprints for authentication, secure key generation, and related applications. PUFs can be integrated directly into edge devices as stand-alone ASIC components, submodules within system-on-chip (SoC) designs, or within field-programmable gate arrays (FPGAs). Reconfigurable architectures such as **FPGAs** are particularly well suited for PUF implementations due to their flexibility, rapid prototyping capability, and balanced trade-off between performance and power efficiency. ML techniques further enhance PUF reliability by improving noise tolerance, adjusting to resource constraints, enabling adaptive authentication, and providing lightweight cryptographic support and secure IoT device management.

### Machine Learning Attacks and PUF Modeling

Despite their benefits, PUFs can be vulnerable to modeling attacks. By analyzing **challenge–response pairs (CRPs)**, ML algorithms can approximate a PUF’s behavior and emulate its responses. Prior studies have applied K-nearest neighbors (KNN), random forests (RF), logistic regression (LR), decision trees (DT), and particularly **support vector machines (SVMs)**—which are often recommended for PUF modeling.

In a typical modeling workflow, ML models are developed in a Python 3.12.3 environment with version control integration. CRP datasets undergo preprocessing, normalization, and cleaning. Using scikit-learn, pandas, and pypof libraries, models such as SVMs, LR, feed-forward ANNs, multilayer perceptrons (MLPs) for device classification, supervised RF clustering for anomaly detection, and ensemble gradient boosting approaches are implemented. Data is split into training and testing sets, model parameters are tuned, and performance metrics are evaluated to ensure adequate generalization without overfitting. The trained models then predict PUF outputs for unseen challenges, providing a measure of PUF resilience against ML-based attacks.

---

### Compute-in-Memory (CIM) and Security Challenges in AIoT Chips

**Compute-in-memory (CIM)** architectures reduce data movement costs by performing multiply-accumulate (MAC) operations directly within memory bit-cells. AIoT devices can store trained neural network parameters inside CIM modules to achieve low-latency and energy-efficient inference. Recent advances in **resistive RAM (ReRAM)** provide new opportunities for on-chip AI acceleration: ReRAM offers extremely low standby power and maintains stored data even when powered off.

However, AIoT chips face three major hardware security threats regarding:

- neural network architectures,
- on-chip weights, and
- user inputs.

Network structures and weights represent intellectual property (IP), while inputs are sensitive user data. An attacker with access to all chip I/O pins may read ReRAM-stored data directly or perform side-channel attacks such as power analysis. While traditional hardware security relies on CMOS-compatible PUFs (ReRAM, MRAM, PCM, SRAM, embedded DRAM, flash memory), **PUFs designed specifically for CIM-based AI computation remain scarce**. Existing work either focuses on PUF encoding/decoding performance or fails to protect neural network weights, structures, and intermediate activations. Prior SRAM-based CIM PUF designs rely on XOR-based encryption and incur density overheads due to large SRAM cells. ReRAM-based PUFs leverage filament-formation randomness but are designed for memory—not CIM—architectures, limiting direct applicability.

Thus, a secure and efficient integration of PUF and CIM is urgently needed to protect AI computation on edge devices.

#### PUF-Guided Secure CIM Architecture Using ReRAM Variations

To address these challenges, recent work leverages intrinsic **device-to-device resistance variations** in ReRAM arrays as entropy sources to design three mutually compatible protection mechanisms:

- **PUF-guided inference protection** (layer encryption)
- **PUF-guided weight redistribution** (weight encryption)
- **PUF-guided input obfuscation** (input encryption)

All PUF responses remain internal to the chip’s CIM module for security.

Neural network models—trained in the cloud or via distributed learning—are loaded into the ReRAM CIM array, and weights are rearranged according to each chip’s unique PUF. A CIM core can perform inference only when the correct PUF response is present, ensuring that computation is possible **only on authorized hardware**.

**Layer 1: PUF-Guided Inference Protection**

Each neural network layer is bound to a PUF bit:

- If **PUF bit = 1**, the layer uses the **real weights**.
- If **PUF bit = 0**, the layer uses **fake weights** generated on demand as

  $W_{fake} = f(address, PUF_{response}, W_{real})$

  requiring no additional storage.

An attacker who copies the model to a different device obtains mismatched PUF bits, causing real layers to be replaced randomly with fake ones. This produces completely incorrect outputs, preventing model theft.

**Layer 2: PUF-Guided Weight Redistribution Protection**

To protect weights against direct memory readout attacks, each weight is decomposed into positive and negative fragments. A **Bipartite-Sort (BS) code** permutes these fragments based on PUF bits. Without the correct PUF, the attacker only sees an unordered set of fragments, making weight reconstruction infeasible—even with full physical access to the ReRAM array.

**Layer 3: PUF-Guided Input Protection**

To protect user privacy, neural network inputs are split into MSB and LSB partitions (e.g., upper 4 bits, lower 4 bits). The input segments are permuted using PUF-guided BS codes and fed into the CIM core in a bit-serial fashion.

Without the correct PUF, the attacker cannot recombine partial sums into the original user input, defeating bus probing and side-channel attacks.

---

[Synopsys](https://www.synopsys.com/designware-ip/security-ip/cryptography-ip/puf.html)
[Pypuf](https://github.com/nils-wisiol/pypuf)
[Opentitan](https://github.com/lowRISC/opentitan)
