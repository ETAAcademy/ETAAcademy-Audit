# ETAAcademy-Audit: 32. Hardware-Software Security

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>32 HSC</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>HSC</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# Hardware-Software Security Contracts and Constant-Time Implementation Strategies

Hardware-Software Contracts (HSC) provide a formal interface defining security guarantees and operational requirements between hardware and software, bridging the gap between architectural and microarchitectural states to mitigate side-channel attacks like Spectre. These contracts use information flow control and lattice theory to rank hardware security, ensuring that software verified as non-interfering against a contract remains secure on compliant hardware.

Modern architectural extensions, such as Arm's Data-Independent Timing (DIT) and Intel's Data Operand Independent Timing (DOIT), offer hardware-level support for constant-time programming by ensuring instruction latencies are independent of data values.

Domain-specific languages like FaCT automate the conversion of high-level cryptographic code into constant-time executable bitcode using confidentiality type systems, while tools like Escort address timing leaks in floating-point operations by masking subnormal latencies through SIMD techniques. Practical cryptographic implementations utilize strategies like bitslicing, Montgomery ladders, and carry spilling to maintain constant-time behavior across various architectures, often requiring careful management of compiler optimizations and big integer arithmetic.

Verification methodologies, including Contract Shadow Logic for RTL and coverage-guided fuzzing with the Self-Compositional Deviation metric, enable the detection of microarchitectural security violations in complex processors, facilitating pre-silicon validation of hardware-software security properties.

## 1. Hardware-Software Contracts (HSC)

**Speculative Execution** allows a CPU to "guess" a program's execution path (e.g., branch prediction) and execute instructions in advance. Side-channel attacks (e.g., Spectre) exploit changes in the CPU's internal state (e.g., cache updates) during speculative execution that are not fully rolled back, thereby inferring secret data. **Timing side-channel attacks** are security vulnerabilities where a program's execution time varies based on secret data (e.g., cryptographic keys), allowing an attacker to recover these secrets by measuring program performance. **Constant-time programming** is a programming discipline that avoids using secret-dependent branches, memory accesses, and variable-time instructions to ensure consistent execution time. **Information Flow Control (IFC)** is a security mechanism (often implemented via type systems) used to track the flow of sensitive data within a program to prevent data leaks. A **confidentiality type system** is a specific IFC implementation that marks variables as `public` or `secret` and enforces rules to ensure that `secret` values do not influence observable behavior.

A Hardware-Software Contract (HSC) provides a formal interface defining the security guarantees provided by hardware and the operations software must perform to remain secure; simply put, it can be viewed as the ISA language and semantics between hardware and software. **Operational Semantics** is a method for describing the execution of a computer program by defining transitions between states. It distinguishes between **Architectural State** (programmer-visible registers and memory) and **Microarchitectural State** (internal components that are typically hidden but may leak information, such as caches and predictors).

The architectural state $\sigma$ consists of current memory content ($m$) and current values in all CPU registers ($a$), $\sigma = \langle m, a \rangle$. The microarchitectural state $\mu$ includes predictors, caches, reorder buffers, etc. The hardware state $\langle \sigma, \mu \rangle$ is composed of architectural and microarchitectural states. An execution trace is a sequence of hardware states; a hardware execution step transitions from one combined state (architectural + microarchitectural) to another $\langle \sigma, \mu \rangle \Rightarrow \langle \sigma', \mu' \rangle$. Hardware trace semantics $\{|p|\}(\sigma) = \mathcal{A}(\mu_0) \cdot \dots \cdot \mathcal{A}(\mu_n)$ define the hardware trace of program $p$ starting from state $\sigma$ as the sequence of all observations made by an attacker throughout the hardware-level execution. $\mu_i$ is the microarchitectural state at step $i$; the attacker cannot see the full $\mu$, only a "projection" $\mathcal{A}(\mu_i)$, which precisely defines what information "leaks" to the observer. If two different secret values produce the same execution trace (the sequence formed over time by the attacker-observable microarchitectural behavior when the program runs on hardware), then the secret is secure against this specific attacker. If the traces differ, the attacker can infer the secret.

Hardware-software contracts act as an "intermediate layer" specifying how much information a program is allowed to leak. **Information Flow Control (IFC)** uses **Lattice Theory** to compare the strength of different contracts (where "stronger" means "leaking less information"). The goal of a contract is to divide responsibility between hardware and software. Hardware must ensure it leaks no more information than allowed by the contract, while software must ensure it is secure within the bounds allowed by the contract. **Observation modes** (what data is leaked, e.g., addresses or values) and **execution modes** (how execution paths are explored, e.g., sequential or speculative). The core formal requirement for a hardware platform to "satisfy" a contract

$$
\{| \cdot |\} \vdash ⟦ \cdot ⟧
$$

$$
⟦ p ⟧ (\sigma) = ⟦ p ⟧ (\sigma') \implies \{|p|\}(\sigma) = \{|p|\}(\sigma')
$$

where $\{|p|\}(\sigma)$ represents the hardware trace semantics of program $p$ under state $\sigma$, and $⟦ p ⟧ (\sigma)$ represents the contract trace semantics. If hardware satisfies a contract, then when the contract dictates that two executions are indistinguishable (identical contract traces), the hardware itself must also make them indistinguishable (identical hardware traces). It ensures that if a programmer audits code according to a contract and finds it secure, that code will be secure on any hardware satisfying that contract. Contract refinement ($⟦ \cdot ⟧_1 \sqsubseteq ⟦ \cdot ⟧_2$) links abstract contracts to physical hardware execution:

$$
⟦ p ⟧_2(\sigma) = ⟦ p ⟧_2(\sigma') \implies ⟦ p ⟧_1(\sigma) = ⟦ p ⟧_1(\sigma')
$$

If everything indistinguishable under Contract 2 is also indistinguishable under Contract 1, then Contract 1 is stronger than Contract 2. This allows us to rank hardware. A CPU satisfying ⟦·⟧⊤ (no information leakage) is the strongest, while a CPU satisfying ⟦·⟧⊥ (all states leaked) is the weakest. This forms the "Lattice of Contracts", where ⟦ \cdot ⟧{ct}^{seq} is a basic baseline. Modern out-of-order processors **do not** satisfy the basic ⟦ \cdot ⟧{ct}^{seq} contract (sequential constant-time), which is why Spectre vulnerabilities are so difficult to address. In the lattice direction, $\sqsubseteq$ points from weaker (more permissive) contracts to stronger (more defensive) contracts. This aligns with security conventions where "top" ($\top$) represents the most secure state. The `arch` observer mode exposes _values_ loaded from memory. This is crucial for formalizing "sandboxing" (where the policy is "do not read outside my boundaries") as it allows the contract to "see" if forbidden values are accessed.

**Non-Interference (NI)** (p ⊢ NI(𝝅, ⟦·⟧)) is a standard security property asserting that secret (High) inputs do not affect public (Low) outputs or observations: $\sigma \approx_\pi \sigma' \implies ⟦ p ⟧ (\sigma) = ⟦ p ⟧ (\sigma')$. A **policy $\pi$** classifies data as public ($L$) or secret ($H$). If a program is started from two different states that share the same "public" data (but may have different "secret" data), and the contract-produced observable traces are identical, then the program is non-interfering. This means an observer cannot learn anything about secret data by looking at the contract trace (e.g., memory access patterns or timing). Using abstract contracts to guarantee software security covers two main scenarios: **Constant-time programming**, ensuring program execution time (and side-channel traces) does not depend on secret data; and **Sandboxing mechanisms**, ensuring malicious or untrusted programs cannot access memory locations they should not (high-privilege memory).

The key lies in the "security transfer" property:

$$
p \vdash NI(\pi, ⟦ \cdot ⟧) \land \{| \cdot |\} \vdash ⟦ \cdot ⟧ \implies p \vdash NI(\pi, \{| \cdot |\} )
$$

If you prove your software is secure against a _contract_, and the hardware correctly implements that contract, then the software program is guaranteed to be secure on actual hardware. This is the "killer app" of the contract framework. It allows software auditors to ignore complex hardware details (like pipelines and out-of-order buffers) and focus on simpler contracts. **SNI (Speculative Non-Interference)** and **wSNI (weak Speculative Non-Interference)** bridge the gap between traditional security proofs and the reality of speculative execution. Speculative Non-Interference (SNI):

$$
\sigma \approx_\pi \sigma' \land ⟦ p
⟧_{ct}^{seq}(\sigma) = ⟦ p
⟧_{ct}^{seq}(\sigma') \implies ⟦ p
⟧(\sigma) = ⟦ p
⟧(\sigma')
$$

Here, $⟦ p ⟧{ct}^{seq}$ is the sequential CT contract (traditional non-speculative leakage trace), and $⟦ p ⟧$ is the target contract, typically a speculative one like $⟦ \cdot ⟧{ct}^{spec}$. SNI means that if two executions look the same on a _sequential_ processor, they must also look the same on a _speculative_ processor (relative to the given contract). It specifically isolates leakage caused by speculation. If a program is "ordinary constant-time" and satisfies SNI, it is "generally constant-time". This is the property checked by tools like Spectector to find Spectre vulnerabilities. Sandboxing depends on values; sandboxing is equivalent to non-interference regarding the `arch` contract because only by seeing the _values_ of accessed memory can one know if the sandbox has been "escaped". The bridging properties, wSNI and SNI, upgrade traditional security proofs (ordinary) to modern speculative environments (general). In Spectre-style attacks, secret data is typically accessed via architecturally valid load instructions, but speculative execution allows transient instructions to propagate that secret data into microarchitectural side effects (e.g., cache state), creating distinguishable hardware traces despite identical architectural execution. This means current hardware protections (e.g., STT or NDA) are insufficient for achieving constant-time programming because they still allow secret data to be loaded non-speculatively and then leaked speculatively.

Hardware countermeasures include secure speculation mechanisms like **taint tracking**, which "marks" (taints) data from untrusted sources (e.g., speculative loads), and **speculative shadows**, referring to the state uncertainty before a branch instruction is resolved. `seq` (sequential execution) is a baseline mechanism that simply disables speculative execution; it is secure but slow. `loadDelay` delays all memory loads until they are no longer speculative. Surprisingly, this mechanism **still violates** the standard constant-time contract (`ct-seq`) because it fails to protect the _instruction cache_ or _branch predictor_ from updates based on speculative branches. `tt` (taint tracking) is based on STT and NDA. It allows speculative loads but taints the data. Only when an instruction depends on tainted data are instructions that might leak data (like other loads or branches) delayed.

The `loadDelaySTEP-EAGER-DELAY` rule: ∀ pc←ℓ@T∈buf[0..i−1], T=ε. When the $i$-th instruction in the ROB is a load, it is only allowed to execute if all preceding branch instructions have been "de-speculated" (tag is empty $\varepsilon$). Here, `buf[0..i-1]` is the Reorder Buffer prefix (instructions before the current one), $T$ is a tag indicating if the instruction is speculative, $\varepsilon$ is an empty tag meaning non-speculative, and $\ell$ is the branch address. The "delay all loads" mechanism has a subtle flaw: if a speculative branch depends on secret data, it can still change the state of the **branch predictor** or **instruction cache**. Even if no speculative data loading occurs, an attacker can later "probe" these components to obtain secret data. Taint tracking uses tagged instructions $i@T^L$; each instruction in the ROB now carries a tag of an index set ($L$). This set tracks which preceding speculative instructions "influenced" this instruction, allowing for fine-grained hardware control. It doesn't delay all instructions, only those where $L$ is non-empty (i.e., tainted), forming the basis for formalizing STT (Speculative Taint Tracking).

STT security guarantee: $\{| \cdot |\}{tt} \vdash ⟦ \cdot ⟧{arch}^{seq}$. Hardware with taint tracking $\{| \cdot |\}{tt}$ satisfies the `arch-seq` contract. Although STT and NDA use different marking strategies, they provide the **same** security guarantees at the contract level (`arch-seq` and `ct-spec`). **All** proposed pure hardware mechanisms (loadDelay, STT, NDA) are insufficient to directly support traditional constant-time programming. Software still needs to be "speculation-aware" or use barrier instructions. The **generality** of formal models involves **microcode assistance** (internal CPU routines for complex tasks), **memory aliasing** (different addresses pointing to the same physical location), and the distinction between **control flow** and **data flow** speculation. To handle Spectre variants exploiting indirect jumps, the contract semantics just need modification to explore not just two paths (taken/not taken) but all possible targets the hardware might predict. This shows the "contract" approach is not limited to one attack type; it is a universal methodology. Hardware vendors could provide "security contracts" in their datasheets, similar to how they currently provide instruction set manuals. The suggestion is not to try fixing flawed hardware but to start from the desired contract (e.g., "I want a CPU satisfying $⟦ \cdot ⟧{ct}^{seq}$") and design the microarchitecture to achieve that goal. It **does not** cover "Meltdown-style" attacks (involving instruction faults) because those would require exposing most of the memory space to the contract, which is theoretically difficult.

---

## 2. Hardware and Language Support for Security

### 2.1 Arm Data-Independent Timing (DIT)

The DIT (Data-Independent Timing) register is a 64-bit control mechanism in the Arm architecture designed to achieve data-independent timing for instructions, thereby mitigating side-channel attacks. Its primary purpose is to allow software to toggle the data-independent timing state. In cryptographic operations, instruction execution time sometimes depends on the data being processed (e.g., whether a bit is 0 or 1). Attackers can observe these timing variations to infer sensitive information, such as private keys.

**Constant-time programming** is a critical defense-in-depth measure for cryptographic software. In modern processors, optimizations like data-dependent execution cycles (e.g., faster multiplication for smaller operands) can leak secrets. The **DIT bit** acts as a global architectural hint/requirement for the processor to disable these optimizations for specific instruction classes, ensuring execution time depends only on the instruction itself, not the operand values. The DIT specification provides a standardized mechanism to enforce data-independent execution time for a large set of AArch64 instructions, including base instructions, SIMD, SVE, and SME operations.

The DIT register is available when `FEAT_DIT` and `FEAT_AA64` are implemented. **FEAT_DIT** (Data-Independent Timing) is an architectural extension ensuring certain instructions execute in a constant number of cycles regardless of data values, eliminating this information leakage. It contains a functional bit at position [24]. Bit [24] is the DIT bit, enabling data-independent timing. Bits [63:25] and [23:0] are reserved as `RES0`. The register is a sparse 64-bit control word where bit 24 is the active toggle bit. This bit is the entry point for the "Data-Independent Timing" state machine, affecting the CPU's execution engine.

Extended instruction ranges (SME, SVE, and SIMD) strictly cover Scalable Vector Extension (SVE/SVE2) and Scalable Matrix Extension (SME/SME2), ensuring constant-time execution across a vast array of scalar, vector, and matrix operations. SVE (Scalable Vector Extension) allows vector register sizes to vary by hardware implementation (from 128 to 2048 bits) while allowing the same binary code to run on all hardware. SME (Scalable Matrix Extension) adds 2D matrix tile registers and instructions on top of SVE to accelerate matrix operations (e.g., outer products), which are vital for AI and ML workloads. Distinguishing between data-value invariance and address invariance is crucial for understanding what DIT protects. DIT ensures timing is independent of data `values` (e.g., `0 + 0` takes the same time as `0xFFFF + 0xFFFF`). It typically `does not` protect against timing differences caused by the memory hierarchy (cache hits vs. misses) due to memory `addresses`, but it does ensure that stored `values` do not affect store timing.

The definition of `0b1` (DIT enabled) places two strict requirements on the microarchitecture: **Load/Store invariance**, where the execution time of load/store instructions must be independent of the data value being transferred; and **Data-processing invariance**, where for listed instructions, execution time must be independent of input data values and the state of condition flags (NZCV). Current coverage has expanded to include base AArch64 standard arithmetic (ADD, SUB), logical (AND, ORR), and cryptographic (AES, SHA) instructions; Advanced SIMD (Neon) vectorized versions of these; SVE/SVE2 instructions including predicate operations, gather/scatter (e.g., `LD1B`, `ST1W`), and complex integer arithmetic; and SME/SME2 matrix outer products (`FMOPA`, `BMOPA`), tile moves (`MOVA`), and multi-vector operations. Including SME instructions like `SMOPA` (Streaming Matrix Outer Product) means even high-throughput matrix math engines must support constant-time execution for security-oriented anti-timing attack modes.

DIT invariance property: $\forall i \in I_{DIT}, \forall v_1, v_2 \in \mathbb{D} \implies T(i, v_1) \approx T(i, v_2)$. For any instruction $i$ in the protected set ($I_{DIT}$), regardless of whether input data $v$ is "simple" (e.g., zero) or "complex" (e.g., large integers, specific bit patterns, operands, registers, flags), execution time $T$ must remain constant. "≈" means architecturally indistinguishable with no observable timing difference. If this condition is not met, an attacker can infer the value of $v$ by observing $T$. For example, in RSA encryption, if a modular exponentiation takes longer for a key bit "1" than "0", the key could be stolen.

SVE gather load instructions (e.g., `LD1B (scalar plus vector)`) access non-contiguous memory addresses. While DIT protects data value timing, address patterns will strictly determine cache performance. Developers should not mistake DIT for a "memory access obfuscation" feature; it does not hide memory access patterns. Explicit cryptographic instructions like `AESD` and `SHA256H` are hardware-accelerated operations. Their inclusion confirms that even fixed-function hardware units are coupled with DIT control logic. The inclusion of SME2 instructions (e.g., `SMLAL` multi-vector) indicates that DIT is a future-proof feature integrated into the latest cutting-edge IP.

The DIT register is architecturally initialized to zero on reset and managed via standard system register instructions (MRS/MSR), with its state directly integrated into Processor State (PSTATE). In AArch64, PSTATE is a collection of fields defining the current processor state (e.g., condition flags, execution state, exception level). Many system registers, including `DIT`, are essentially views of specific PSTATE fields. MRS/MSR instructions (Move to System Register from Scalar and vice versa) are the primary way software interacts with architectural control registers. A warm reset preserves some system state (unlike a cold reset) but clears architectural control bits like DIT to ensure the system starts in a known, predictable, and typically "least-privileged" or "standard-performance" mode. On warm reset, the DIT field resets to "0". This ensures data-independent timing is disabled by default, prioritizing performance over security during the boot process.

The register can be accessed via standard encoding (`op0=0b11, op1=0b011, CRn=0b0100`). Architectural pseudocode shows that the `DIT` register is a 64-bit projection of the `PSTATE.DIT` bit. During a read (`MRS`), the bit is at position [24] with all other bits zeroed. During a write (`MSR`), only bit [24] of the source register is used to update `PSTATE.DIT`. The immediate version of MSR (`MSR DIT, #<imm>`) allows software to toggle the bit without using a temporary general-purpose register.

Register read mapping (MRS): $X[t] = \text{Zeros}(39) : \text{PSTATE.DIT} : \text{Zeros}(24)$. When software reads `DIT` into a 64-bit general-purpose register X[t], the processor takes the 1-bit `PSTATE.DIT` value (the internal architectural source of truth), appends 24 zeros (bits 23:0), prepends 39 zeros (bits 63:25), and stores the result. This explains the choice of bit [24]—it aligns with how the bit is stored internally. Using zero for reserved bits ensures software doesn't see "garbage" values. The pseudocode `if !(IsFeatureImplemented(FEAT_DIT) && IsFeatureImplemented(FEAT_AA64)) Undefined();` confirms the register is entirely absent (triggering an exception) if the extension isn't supported. The fact that `DIT` is part of `PSTATE` is crucial; it means DIT state is automatically saved and restored during context switches (e.g., when an exception occurs and the processor saves `SPSR`), ensuring security settings are maintained even if a process is interrupted. The `MSR DIT, #<imm>` encoding is a performance optimization for kernel code, allowing security state toggling with a single instruction.

---

### 2.2 Intel's Data Operand Independent Timing (DOIT)

Data Operand Independent Timing (DOIT) aims to ensure constant-time execution of security-critical cryptographic code by disabling certain hardware optimizations that could lead to data-dependent timing biases. Attackers infer sensitive information (like encryption keys) by measuring how long a processor takes to execute specific instructions. To mitigate this, developers use **constant-time programming**, ensuring execution time does not depend on secret data. DOIT is a hardware feature assisting this practice by providing architectural guarantees of **data-independent timing**, meaning the processor cycle count for instructions remains constant regardless of input data values ("operands").

DOIT is a tool for "constant-time" execution, primarily used in cryptographic algorithms. It is not a standalone security solution but a complement to existing software mitigations. Enabling DOIT may disable hardware optimizations (e.g., data-dependent prefetching), impacting performance. Thus, it is recommended to enable DOIT only in specific software components that need it, rather than globally. While DOIT addresses data-dependent timing, it does not necessarily mitigate side-channel attacks related to memory addresses, instruction encoding, power consumption, or temperature changes.

DOIT focuses on the microarchitectural behavior (timing) of operands. **Masking operations** (common in SIMD/AVX) use a "mask" to determine which elements in a vector are processed. **Immediates** are constants embedded directly in instruction opcodes rather than loaded from registers. DOIT only guarantees instruction latency is independent of _data values_ (operands). It explicitly excludes:

- **Memory Addresses**: Addresses used to load/store data can still leak information through cache timing or other address-based side channels.
- **Instruction Encoding**: The bytes of the code itself (including immediates) affect latency. Sensitive information must never be placed in immediates.
- **Masking Operations**: For operations _not_ accessing memory, latency is mask-independent. However, for those _that do_ access memory, masks affect accessed addresses, which can lead to data-dependent timing issues if addresses themselves are time-sensitive.

DOIT mode is enabled via a Model Specific Register (MSR), ensuring specified instructions execute with timing independent of data operands and preventing those values from affecting the execution time of subsequent instructions. **Model Specific Registers (MSRs)** are system registers used to toggle processor features not part of the standard instruction set. It also involves **speculative execution**, where the CPU executes instructions in advance for performance. Furthermore, **Telemetry** and **Power Management (RAPL)** are how the CPU tracks power consumption, which itself can be a side channel. When DOITM is enabled:

- **Direct Timing Independence**: Instructions in the DOIT subset execute in a constant number of cycles, independent of operand values (ignoring power/temperature changes).
- **Indirect Timing Independence**: Data values from DOIT instructions do not affect the execution time of _other_ (subsequent) instructions. This is a key feature for preventing "transient execution" or "microarchitectural state" leaks.
- **Outside the DOIT Subset**: Instructions not in the subset, or executed when DOITM is off, may still leak data-dependent timing information, both for themselves and other instructions.
- **Side-channel Residue**: Even in DOIT mode, power consumption, CPU frequency, and RAPL telemetry may still vary based on data values. This shows "constant-time" (cycles) = "constant-power".
- **Speculative Execution**: Developers must still consider speculative execution vulnerabilities (e.g., Spectre), as secret data might be processed in a data-dependent manner during the speculative execution window before DOIT protection is applied by the core.

**Cycles vs. Power**: DOIT guarantees cycle count independence but explicitly _not_ power consumption independence. **"Subset" Limitation**: Constant-time only applies to a _subset_ of instructions. If a developer accidentally uses instructions outside this subset for secret data, DOITM provides no protection. **Data Leakage to Subsequent Instructions**: The guarantee that DOIT data doesn't affect subsequent instruction execution time is vital for stopping complex side-channel attacks where one instruction's data influences seemingly unrelated later instructions.

Software enables DOIT by setting a bit in the `IA32_UARCH_MISC_CTL` MSR. This process requires architectural support verification and careful evaluation of performance-security tradeoffs. **Feature Enumeration** is the process where software queries the CPU (typically via `CPUID`) to see if a feature exists. `IA32_ARCH_CAPABILITIES` is an MSR reporting various architectural attributes and mitigations. **Microcode updates** are patches used to update CPU internal logic after manufacturing.

- **Control Mechanism**: The `IA32_UARCH_MISC_CTL[DOITM]` bit (bit 0 of MSR 0x1B01) is the primary switch.
- **Enumeration**: Support is indicated if bit 12 of the `IA32_ARCH_CAPABILITIES` MSR is set to 1.
- **Processor Evolution**:
  - _Pre-Ice Lake / Pre-Gracemont_: These processors may not enumerate DOITM; developers can assume relevant instructions _already_ behave as if DOITM is enabled (i.e., they are inherently constant-time).
  - _Ice Lake and later / Gracemont and later_: These processors _will_ enumerate DOITM and must have it explicitly enabled for constant-time guarantees.

**Implicit vs. Explicit**: The shift from "always constant-time" (older CPUs) to "optional constant-time" (newer CPUs) indicates that hardware optimizations are becoming more aggressive and data-dependent, necessitating an explicit "slow down for security" switch. **MSR Serialization Behavior**: The `WRMSR` instruction to `IA32_UARCH_MISC_CTL` is _not_ defined as a serializing instruction. This means if software needs to ensure DOIT is active immediately after writing, additional serialization (like `LFENCE` or `ISYNC` equivalents) may be required. Performance impact "may increase in future processor generations," so DOIT code paths should be kept concise and efficient.

To maintain security, DOIT is automatically forced enabled in Intel SGX enclaves regardless of system settings; Intel TDX allows Trust Domains to manage the mode independently via context switching support. In **Trusted Execution Environments (TEEs)**, Intel Software Guard Extensions (SGX) allows software to execute in private "enclaves" isolated from the rest of the system, including the OS. Intel Trust Domain Extensions (TDX) isolates VMs ("Trust Domains") from the VMM and other hardware components. The CPU process of saving and restoring state for different execution environments (e.g., switching between two VMs) is part of this.

- **Intel SGX**:
  - **Forced Enablement**: If the processor supports DOIT, it is always enabled when executing within an enclave.
  - **Trust Model**: SGX does not trust the OS to manage DOIT. Since enclaves are often used for secret processing (where constant-time is vital), hardware enforces DOIT to mitigate timing attacks. This avoids the complexity of controlling DOIT via existing SGX ISA but at the cost of potential performance reduction within the enclave.
- **Intel TDX**:
  - **Independent Control**: A Trust Domain (TD) can view and set DOIT control bits based on its own threat model.
  - **Context Switching**: The TDX module ensures DOIT settings are correctly saved and restored when switching between TDs. Unlike SGX, it is not forced enabled, allowing TD owners more flexibility.

The choice to force enable DOIT in SGX reflects the enclave's "secure by default" philosophy where timing leaks are a primary threat. The TDX approach is more flexible, treating DOIT as another piece of virtualized architectural state controlled by the TD owner. Beyond standard context switching, TDX has no special interaction with Secure Arbitration Mode (SEAM) or VMX.

The DOIT feature is architecturally defined via bit 12 of the `IA32_ARCH_CAPABILITIES` register and bit 0 of the `IA32_UARCH_MISC_CTL` register, enabling precise hardware discovery and control. MSRs like `IA32_ARCH_CAPABILITIES` (0x10A) are read-only and inform software of hardware capabilities. `IA32_UARCH_MISC_CTL` (0x1B01) is a read/write register for microarchitectural control. **Logical processor scope** means the setting applies to an individual CPU thread rather than the entire physical package or core.

- **Feature Discovery**:
  - **Register**: `IA32_ARCH_CAPABILITIES` (Address: 10AH / 266).
  - **Bit**: 12.
  - **Meaning**: If 1, the processor supports DOITM.
- **Control Bit**:
  - **Register**: `IA32_UARCH_MISC_CTL` (Address: 1B01H).
  - **Bit**: 0 (DOITM).
  - **Permissions**: Read/Write (R/W).
  - **Reset Value**: 0.
  - **Scope**: Logical Processor.

On some processors, configuring `MXCSR` (Floating-Point Control/Status Register) might also be necessary to avoid data-dependent timing issues for certain instructions. **Discovery Order**: Software _must_ check `IA32_ARCH_CAPABILITIES[12]` before attempting to write to `IA32_UARCH_MISC_CTL[0]`, otherwise a General Protection Fault (#GP) may be triggered. **Floating-point Interaction**: `MXCSR` is vital for developers using SIMD instructions; it indicates that if the FPU is configured to handle subnormals or exceptions in a data-dependent manner, just using DOIT might not suffice. **MSR Address 1B01H** is the specific address needed for `RDMSR` and `WRMSR` instructions.

---

### 2.3 FaCT: A Domain-Specific Language for Constant-Time Programming

FaCT is a domain-specific language and compiler that uses a confidentiality type system to eliminate secret-dependent control flow, automatically converting readable, high-level cryptographic code into time-safe, constant-time executable code. Writing secure cryptographic code in C is difficult and error-prone. Developers often avoid high-level constructs (if statements, loops) when handling sensitive data, manually applying "tricks" to create obfuscated constant-time code. This manual process is prone to error and hard to audit. FaCT addresses this by allowing developers to write high-level code with `secret` annotations. The FaCT compiler automatically applies these constant-time transformations (tricks) to LLVM bitcode during compilation, ensuring security while maintaining code readability and performance.

Constant-time selection trick: x=(−secret & e) ∣ (secret−1) & x. Here, `secret` is a secret boolean (0 or 1) used as a selection mask, $e$ is the expression to be assigned when `secret` is 1, and $x$ is the variable to be updated, retaining its original value if `secret` is 0. It exploits properties of two's complement arithmetic where -1 is represented as all bits set to 1 (0xFF...FF) and 0 as all bits set to 0 (0x00...00). This method avoids hardware branch instructions. In traditional C, `if (secret) x = e;` would compile into a branch instruction whose execution time (or impact on the branch predictor) depends on the value of `secret`. This bitwise sequence executes in the same number of clock cycles regardless of input. This is the manual "method" FaCT aims to automate. FaCT allows programmers to write `if (secret) x = e;` and automatically generates equivalent constant-time bitcode.

If `secret` is any other non-zero value, the bitmask logic will fail unless first normalized (e.g., `!!secret`). The expression $e$ is always evaluated. If $e$ has side effects or a long computation time, this transformation could introduce new issues or performance bottlenecks, though in cryptography, it is usually a simple value or arithmetic expression. While bitwise operations are typically constant-time, some architectures might have variations; FaCT relies on the compiler backend (LLVM) to maintain these properties.

**Padding Oracle Attacks (e.g., Lucky13)** exploit timing differences between responses for "valid padding" and "invalid padding" to decrypt ciphertext. If a loop or function returns immediately upon finding a mismatch (typical C style), the execution time leaks the mismatch position, which is often secret information (e.g., in string/buffer comparisons). Since cache hits are faster than misses, memory access patterns dependent on secret indices (like S-boxes) leak secret information. Attackers can recover indices by observing AES encryption. In multi-precision arithmetic, a "limb" refers to one word (e.g., 64 bits) of a large integer. Three main timing vulnerabilities:

- **Secret-dependent Branching**: Branch statements in C typically require different execution times. FaCT replaces `if` statements with `ctselect` bitmask operations.
- **Early Termination**: Early `return` or `break` based on secret values leaks information. FaCT's compiler converts these into "deferred returns," where the function continues to the end but suppresses side effects after the logical "return."
- **Memory Access**: Accessing `array[secret_index]` leaks the index via the cache. FaCT ensures memory access patterns are independent of secret data.

Libsodium constant-time buffer comparison: return (1 & ((d−1)≫8))−1. Here, $d$ is the cumulative XOR sum of buffer differences ($d=0$ if buffers match, $d>0$ otherwise). $n$ is the number of bytes to compare, the loop boundary. It converts the cumulative difference $d$ into a return value of $0$ (success/match) or $-1$ (failure). It ensures the loop always executes $n$ iterations regardless of when a mismatch occurs. The final expression is a constant-time way to check if $d$ is non-zero without using an `if` statement. In FaCT, users write simple `if (x[i] != y[i]) return -1;`, and the compiler generates this (or similar) logic.

Curve25519-donna constant-time swap: `swap` is a selection mask (all 1s or all 0s). If `swap` is all 1s, then $x = a[i] \oplus b[i]$, and subsequent XORs make $a[i]$ become $b[i]$ and $b[i]$ become $a[i]$. If `swap` is 0, then $x = 0$, and values remain unchanged. This avoids branch statements like `if (do_swap) swap(a, b);`. Both paths (swap or no swap) execute exactly the same instruction sequence and memory accesses. This is a manual implementation of "conditional swap" (CSWAP), a primitive widely used in elliptic curve cryptography to prevent timing side channels during scalar multiplication. Manual implementations are unintuitive and error-prone (e.g., using wrong shift amounts or confusing logical vs. arithmetic shifts). If compilers (e.g., `gcc` or `clang`) recognize this pattern, they might "optimize" it back to branch statements unless specific precautions are taken. FaCT operates at the LLVM level to ensure constant-time properties are preserved.

x=swap & (a[i]⊕b[i])

a[i] = a[i] ⊕ x

b[i] = b[i] ⊕ x

FaCT's type system uses confidentiality labels and tracking contexts (pc and rc) to prevent implicit information leaks and enforce "public safety," ensuring high-level code can be safely converted to constant-time bitcode. **Confidentiality labels ($\ell$)** classify data as `Pub` (public) or `Sec` (secret). These labels form a lattice where $Pub \sqsubseteq Sec$. The path context ($pc$) tracks if current execution is within a branch dependent on a `secret` value. The return context ($rc$) tracks if current execution might be skipped by an early `return` within a `secret` branch. Implicit flow is an information leak where _control flow_ (e.g., which branch is taken) leaks info about a secret, even if that secret is never directly assigned to a public variable. Any operation that could cause undefined behavior (e.g., array indexing) must be secure based _only_ on public information.

FaCT is designed as a C-like DSL that can be embedded in existing projects. It supports standard constructs but adds primitives like `ctselect` (constant-time selection), `declassify` (explicitly making a secret public), and `assume` (providing hints to the safety checker). The type system is the "core" of FaCT, rejecting programs that leak secrets through timing or memory access patterns. It also guides the compiler's transformation. Specifically, "public safety" checks use the Z3 SMT solver to prove array indices are always within valid ranges regardless of secret conditions, preventing constant-time transformations from introducing out-of-bounds accesses.

Confidentiality label lattice and join: $\ell$ labels are $\{ 	\text{Pub}, 	\text{Sec} \}$, defining the information flow policy. Public data can be used for secret computations, but secret data cannot be "demoted" to public without explicit declassification. This provides the mathematical basis for tracking "taint." If an expression uses one secret variable and ten public ones, the result is still `Sec`.

$$ Pub⊑ℓandℓ⊑Sec $$
$$ ℓ1⊔ℓ2 =Sec if either is Sec, else Pub $$

Statement typing judgment (simplified) evaluates if statement $S$ is correctly typed under given confidentiality contexts and how it updates these contexts and environments for the next statement. This is how FaCT tracks direct and implicit flows. For example, if $pc = 	\text{Sec}$, any assignment in $S$ must be to a `Sec` variable (Rule `T-Asgn`).

$$ ω,pc,βr ⊢S:Γ,rcoΓ′ ,rc′ $$

- $\omega$: Procedure context, mapping procedures to their initial $pc$.
- $pc$: Path context label, current secrecy level of control flow.
- $\beta_r$: Procedure return type, expected return value type.
- $S$: Statement being typed, e.g., assignment, if statement, loop.
- $\Gamma$: Type environment, mapping variables to types.
- $rc$: Return context label, secrecy of the "already returned" status.

Assignment rule (T-Asgn): To assign $e_2$ to reference $e_1$, the combined secrecy of control flow ($pc$) and return status ($rc$) cannot exceed the target variable's confidentiality label $\beta$. This rule prevents "implicit flow." If you are inside an `if (secret)` block ($pc = 	\text{Sec}$), you cannot update a public variable because that update would leak the secret's value.

$$ \frac{\Gamma \vdash e_1 : \text{Ref}\_W[\beta] \quad \Gamma \vdash e_2 : \beta \quad pc \sqcup rc \sqsubseteq \beta}{\omega, pc, \beta_r \vdash e_1 := e_2 : \Gamma, rc o \Gamma, rc} $$

**SMT Dependency**: The validity of "public safety" checks depends on the SMT solver's ability to reason about program constraints. If logic is too complex for Z3, users must add `assume` statements. **Pointer-free**: FaCT avoids C-style pointers, using `Ref` and `Arr` types and "views" to maintain memory safety and enable static analysis. **Static vs. Dynamic**: While many IFC systems are dynamic, FaCT is purely static, meaning all safety checks happen at compile time, and labels themselves incur no runtime overhead.

The FaCT compiler uses a two-stage process—return deferral and branch elimination—to eliminate secret-dependent control flow, converting high-level idiomatic code into straight-line, constant-time code. **Return Deferral**: In constant-time programming, you cannot exit a function early based on secrets. This process ensures every function executes to the end by replacing `return` statements with state updates. **Branch Elimination (Conditional Transformation)** is the process of converting conditional branches into a sequence of selections (multiplexing). Both `then` and `else` paths execute, but only results based on the mask are committed. **Instrumented Semantics** are used to track "information leaks" $\psi$. $\psi$ represents observable code behavior, like accessed memory addresses and branch directions taken. The **Correctness Theorem** proves transformed code produces the same results as original code. The **Security Theorem** proves transformed code is "constant-time," meaning its information leakage $\psi$ is independent of secret inputs. The security theorem only applies if the code contains no `declassify` operations. If a programmer explicitly declassifies secret data, the constant-time guarantee for that data is (intentionally) broken. **Code Bloat**: Branch elimination executes _both_ sides of an `if-else`. If both sides are large, code size and execution time double. However, in cryptographic code, branches are usually small. The transformation assumes expressions themselves do not leak info. FaCT's type system ensures expressions are simple and constant-time.

The compiler operates in two phases. First, **Return Deferral** handles early returns. It introduces two variables: `rval` (storing return value) and `notRet` (a boolean flag indicating if the function is still logically executing). All statements following potentially secret return statements are guarded by `if (notRet)`. Second, **Branch Elimination** eliminates `if` statements. It evaluates both branches and uses `ctselect` to update memory or variables. For example, an assignment `x = y` in a secret `if` statement becomes `x = ctselect(mask, y, x)`. It is proven that for "no-declassify" procedures, this compiler generates constant-time code.

Return deferral for return statements (Tr-Ret): If a `return` statement is encountered in a secret context ($pc$ or $rc$ is `Sec`), it is replaced by an assignment to a temporary return value variable and the clearing of the "not returned" flag. This prevents the CPU from executing jump or return instructions that might leak exit time. The rest of the function continues to run, but subsequent side effects are suppressed because `notRet` is now `false`.

$$ \frac{pc \sqcup rc = \text{Sec}}{\Phi, pc, rc \vdash \text{return} \ e o \text{rval} := e; \text{notRet} := \text{false}} $$

rval: Variable storing the final return value, introduced by Tr-Ret-Dec

- `rval`: Variable storing the final return value, introduced by `Tr-Ret-Dec`.
- `notRet`: True if the function hasn't yet returned, initialized to `true`.
- $e$: Return expression.

Branch removal for assignment (Tr-Br-Assign) converts an assignment inside an `if (p)` block into an unconditional `ctselect` operation. If $p$ is true, $e_1$ gets new value $e_2$. If $p$ is false, $e_1$ "gets" its own current value (effectively a no-op). This is the core of branch removal. It converts conditional control flow into conditional data flow, which can be implemented in constant time using instructions like `CMOV` or bitmasking.

$$ \Phi, p \vdash e_1 := e_2 o e_1 := \text{ctselect}(p, e_2, e_1) $$

- $p$: Control predicate (mask) containing branch condition.
- $e_1$: Target variable/reference.
- $e_2$: Source expression.

Big-step instrumented semantics define procedure $F$ execution. It maps inputs and initial state to a final result while generating "leakage" information $\psi$. This is a mathematical tool for proving security. If for any two executions with different keys but identical public inputs, the leakage $\psi$ remains the same, the procedure is "constant-time."

$$F : (\vec{v}, h) \xrightarrow{\psi} (v, h')$$

- $F$: Procedure.
- $\vec{v}$: Input parameters.
- $h, h'$: Initial and final heaps, mapping pointers to values.
- $v$: Final return value.
- $\psi$: Leakage trace, including branch directions, memory addresses.

FaCT's limitations in polymorphism and cross-layer verification place it within the evolving landscape of cryptographic DSLs, providing a formal grammar for the language. **Polymorphism** is the type system's ability to handle different data types with a single interface (e.g., one function for both `Int` and `Float`). FaCT currently lacks this. **Flow-sensitivity** is a type system property where a variable's type (or label) can change based on control flow paths (e.g., a variable is initially `Pub`, then becomes `Sec` after assignment). **Lowering** is the process of converting high-level code to low-level representations (e.g., LLVM IR to machine code). A **Verified Compiler** (like CompCert) has a mathematical proof that source semantics are precisely preserved in output. FaCT's backend (LLVM) is not verified in this way.

FaCT is compared to other tools. `Jasmin` and `Vale` provide higher security guarantees but are lower-level (akin to assembly). `Low*` and `CT-Wasm` are similar high-level approaches but typically require more manual effort than FaCT's automated method to ensure constant-time behavior. `SC-Eliminator` works on existing C code but lacks FaCT's high-level security guarantees.

---

### 2.4 Managing Floating-Point Timing Leaks

Floating-point computations on modern processors leak sensitive information via variable-latency instructions (especially subnormals) and software-level control/data flow, requiring comprehensive defenses like "Escort" to ensure fixed-time execution and uniform access patterns.

**Side-Channel Attack**: An attack where an attacker infers secret information by observing physical or microarchitectural side effects of program execution (e.g., timing variations, power consumption, or cache behavior). **Fixed-time Operation**: A security requirement that an operation's execution time must be independent of its operand values to prevent timing side-channel attacks. **IEEE 754 Floating-Point Standard**: The standard for floating-point representation and arithmetic, including special values like **Zero**, **Infinity**, **NaN** (Not a Number), and **Subnormals**. **Subnormal numbers (denormalized numbers)** are numbers smaller than the smallest representable "normal" floating-point number. Due to leading zeros in their significand, many processors handle them using microcode or traps, leading to significantly higher execution latency.

Floating-point operations are a major source of side-channel leakage from two primary sources:

- **Hardware Level**: Variable-latency instructions, especially those handling denormalized operands (e.g., an `SQRTSS` instruction taking 153 cycles for denormalized operands vs. 11 cycles for normalized ones).
- **Software Level**: High-level operations in libraries (e.g., sine/cosine functions) can leak data through exceptions, conditional branches, and memory access patterns (lookup tables).

A compiler-based solution, **Escort**, aims to provide four features: fixed-time operation, disabled exceptions, sequential control flow, and uniform data access. Escort uses SIMD lanes to perform "dummy" calculations on denormalized operands while processing normalized ones to mask timing differences, using code transformations for software-level security.

Timing variation represents a core mathematical vulnerability: instruction $I$ latency $L$ is a function of its operand $x$, $L(I, x)$. The time required to execute a floating-point instruction varies based on whether input $x$ is normal, zero, denormal, infinity, or NaN. In a secure environment, if an attacker can measure $L$, and $L$ for a denormal $x$ (e.g., 153 cycles) differs significantly from a normal $x$ (e.g., 11 cycles), the attacker can infer if $x$ is denormal. If these bits affect whether an operand becomes denormal, this could leak secret bits of a key used in a cryptographic algorithm. This timing difference ($L_{subnormal} \gg L_{normal}$) is precisely the "timing side channel" Escort aims to eliminate. Latency values are specific to tested architectures (like x86 processors). The four properties defined for a "strong solution" are considered necessary and sufficient to eliminate digital side channels in this context. The method adopts a SIMD strategy, using SIMD lanes, thus focusing on improving timing performance via masking operations.

- $L$: Latency (in cycles), the dependent variable.
- $I$: Instruction (e.g., SQRTSS), the independent variable.
- $x$: Operand value, the input to the instruction.

Escort is designed to defend against external and co-resident attackers monitoring various digital side channels. It improves upon existing fixed-point libraries (e.g., FTFP), providing higher precision and plugging more leak vectors with lower overhead. **Observation-based side channels** are those where an external entity monitors system-wide signals (e.g., total execution time, power, or memory address traces via `/proc`). **Contention-based side channels** are those where other processes on the same physical hardware (e.g., another VM) compete for shared resources (e.g., CPU cache or branch predictors) to infer victim activity. **Fixed-point arithmetic** is a method for representing fractional numbers using a fixed number of bits with a set number of digits before and after the decimal point. It avoids variable-latency issues of floating-point arithmetic but typically loses precision and dynamic range. An **Irreducible Control Flow Graph (CFG)** is a representation of possible program execution paths where some loops have multiple entry points. Most modern compilers require reducible CFGs for optimization.

Attackers can monitor time, memory traces, cache contention, and branch predictor state. However, physical side channels (e.g., heat or EM radiation) are excluded (assuming the CPU is in a "sealed and tamper-resistant chip"). The **Trusted Computing Base (TCB)** includes the CPU and compiler. This means we trust the hardware's SIMD behavior and the compiler's transformation logic. Escort is compared to the previous state-of-the-art, `libfixedtimefixedpoint` (FTFP), which was criticized for manual operation, slowness, low precision (due to conversion to fixed-point), and failure to plug cache or address trace channels. Escort is fully automated via the compiler and supports a much larger set of features than FTFP's 19 (112 math functions). By excluding heat/power/EM, it focuses on "digital" leaks capturable via software or high-level system monitoring. The Escort compiler successfully bridges the gap between side-channel security and numerical precision, leveraging existing SIMD hardware and automated code linearization to protect a wide array of floating-point libraries. **Hardware-Software Co-design**: Security is best achieved when compiler techniques and hardware features (like SIMD instruction latency properties) work together. This encourages hardware vendors to provide more explicit support for constant-time instructions rather than forcing software to use tricks like SIMD subnormal masking. The Escort method is limited by specific features of current x86 processors (e.g., variable-latency denormalized floats). If future processors provide native constant-time floating-point operations, SIMD tricks would become obsolete, but compiler linearization would still be necessary for software-level leaks. **Range extension**: If integer instructions also exhibit operand-dependent execution time, Escort's method can be extended to them.

If program $P$ fits Escort's constraints (no I/O, no recursion, etc.), the transformed program $P'$ satisfies: `Results` (calculated output must be identical/precision), and `Leakage` (information leaked via channels must be null). Transformations must preserve precise numerical output (precision) while ensuring no information about secret inputs is leaked through timing or other digital channels (like cache or memory addresses).

$$ \text{Results}(P) = \text{Results}(P') \land \text{Leakage}(P', \text{Timing}) = \emptyset \land \text{Leakage}(P', \text{Digital}) = \emptyset $$

Variable latency in floating-point instructions is caused by microcode handling denormalized numbers (vital for gradual underflow), and the precision loss of alternatives is quantified using Units in the Last Place (ULP). **Gradual underflow** is an IEEE 754 feature allowing floats to lose precision "gracefully" as they approach zero rather than abruptly "flushing to zero." This is achieved using **denormalized numbers**. The **significand (mantissa)** is the part of a floating-point number representing its significant digits. **Microcode** consists of low-level instructions stored in processor control memory to implement complex machine instructions. Arithmetic on denormalized numbers is often too complex for hardware circuits, thus requiring microcode handling. **ULP (Unit in the Last Place)** measures the spacing between two consecutive floating-point numbers, often used to quantify error or distance between two float values.

The root cause of timing vulnerabilities is the very small exponent of **denormalized numbers**. For single-precision floats, $|x| < 10^{-38}$. Without them, a large gap exists between zero and the smallest positive number. With them, the gap is uniform, enabling gradual underflow. **Vulnerability**: Hardware logic for normalized numbers is fast, but since denormals are rare, vendors often use microcode to implement them, which is much slower. Attackers can measure execution time to infer if a value is denormalized. **Measurement**: ULP is introduced as a precision metric. If the ULP distance between two floats is zero, they are identical.

The denormalized range (single precision) defines the numerical range where a single-precision float is considered "denormalized." Values in this range trigger slower microcode paths, creating a timing side channel.

$$ 10^{-45} < |x| < 10^{-38} $$

ULP distance calculation: The ULP distance between floats $f_1$ and $f_2$ interpreted as integers (IntRep bit pattern interpreted as an integer) is:

$$ \text{Dist}\_{ \text{ULP}}(f_1, f_2) = | \text{IntRep}(f_1) - \text{IntRep}(f_2)| $$

The distance between two floats is the number of representable values between them. Since IEEE 754 representation (for positive numbers) is lexicographical, subtracting their integer representations gives this count. If the Escort output $f_{escort}$ and standard library output $f_{math}$ have `Dist_ULP = 0`, Escort is "perfectly precise." **Necessity of Gradual Underflow**: Subnormals are "indispensable" for reliable equation solving, explaining the choice to safely support them rather than simply flushing to zero (which is faster but less precise). **Microcode Overhead**: The overhead is described as an "order of magnitude" slowdown, 153 cycles vs. 11.

Escort achieves side-channel security by masking variable-latency instructions using dummy subnormal lanes in SIMD registers and linearizing high-level code through predicate-based control flow and memory access obfuscation.

Escort consists of two main components. The SIMD technique is clever as it exploits existing hardware features (slowest lane wins) to provide security without new CPU instructions. Linear scanning (Step 5) and constant-time loops (Step 6) have high overhead. Math libraries typically use small lookup tables and loops. Escort requires programmers to disable exceptions (`feclearexcept`) because exceptions are a side-channel "termination" leak.

- **Secure Base Operations**:
  - **SIMD Trick**: x86 SIMD instruction latency (e.g., `mulpd`) depends on the _slowest_ lane. Escort loads real data into one lane and a "dummy" denormalized number into another. This ensures the instruction always takes the (longer) denormalized latency, regardless of the real data's value.
  - **Optimization**: Escort uses Z3 (an SMT solver); if it can prove an actual operand will never be denormalized, it skips this high-overhead masking operation.
- **Compiler Transformations**:
  - **Step 1**: Replace vulnerable instructions with SIMD-masked versions.
  - **Step 2 & 3**: "Flatten" the program. Remove all conditional branches. Every basic block executes, but its results are only "committed" (written to memory/registers) if its associated predicate is true.
  - **Step 4 & 5**: Control side effects. Memory stores become conditional. Array accesses are replaced by linear traversal of the entire array to hide indices.
  - **Step 6**: Loops execute with a fixed number of iterations (relative to the next power of 2 of the maximum iterations).

SIMD Latency Masking: Let $L(v)$ be the instruction latency for operand $v$. For a SIMD instruction with lanes $v_1, v_2$:

$$ \text{Latency}\_{ \text{SIMD}}(v_1, v_2) = \max(L(v_1), L(v_2)) $$

Escort ensures $v_2 \in 	\text{Subnormals}$, so $L(v_2) = L_{max}$.
Thus, for any $v_1$, $Latency_{ SIMD}(v_1, v_2) = L_{max}$. By always including a subnormal "anchor" in an idle SIMD lane, the total execution time is bound to the worst case. This keeps execution time constant (fixed-time) relative to actual data $v_1$, eliminating the timing side channel.

- $v_1$: Actual operand, real data.
- $v_2$: Dummy operand, forced subnormal.
- $L_{max}$: Maximum instruction latency, the latency for subnormals.

Predicate Propagation: For basic block $s$, which is a successor block derived from $bb$ via a branch with condition $p$. If the predecessor node is active and the condition to reach node $s$ is met, node $s$ should "logically" execute. This allows the compiler to convert a branch control flow graph into a linear sequence of blocks where each block's impact is controlled by its predicate.

$$ \text{pred}[s] = \text{pred}[s] \lor ( \text{pred}[bb] \land p) $$

- pred[bb]: Predicate of basic block $bb$, a boolean.
- $p$: Branch condition from `if (p)`.
- $pred[s]$: Predicate of successor node $s$, or-accumulated.

---

### 2.5 Cryptographic Implementation and Constant-Time Practices

The historical context of timing attacks dates back to Paul Kocher's 1996 attack on RSA. While timing attacks are not common in practice, even remote attacks are feasible, necessitating systematic defenses. Comparing constant-time code (execution time independent of secrets) and masking (randomizing inputs/operations), masking's limitation is its algorithm-dependence, lack of conclusive proof of effectiveness, and need for high-quality randomness.

The "execution model" identifies five potential non-constant-time hardware-level operations:

- **Memory Access**: Caches create timing differences based on addresses, potentially leaking secret indices.
- **Conditional Jumps**: Jump prediction and opcode fetching might leak conditions (if secret-dependent).
- **Integer Division**: Often implemented via microcode or subroutines optimized for small operands.
- **Shifts and Rotates**: On CPUs lacking barrel shifters (e.g., Pentium IV), these can take variable time.
- **Multiplication**: While most modern CPUs have constant-time multiplication, older or specialized architectures (e.g., ARM9) might have variable-time implementations.

RSA masking: Instead of directly calculating $m^d \mod n$, message $m$ is first masked by multiplying it with $r^e \mod n$. Then the private exponent $d$ is applied to the product. Finally, the result is multiplied by $r^{-1} \mod n$ to unmask it. Since modular exponentiation is performed on $(mr^e)$, which is effectively a random value to an observer, any timing variation in the $d$ exponentiation step cannot be correlated with original message $m$ or private key $d$. This illustrates another way to achieve constant-time code: instead of making execution time constant, we make its dependence on secret data look random. While constant-time multiplication is common now, it's an "important note" for systems like ARM7 or ARM9. It's crucial to distinguish that timing leakage is only an issue when conditions/addresses/operands are _secret_ data; non-secret data (e.g., round counts in AES-128) does not require constant-time handling.

$$ r^{-1}(mr^e)^d \mod n $$

- $m$: Message to be decrypted or signed, secret data.
- $n$: Modulus, public value.
- $d$: Private exponent, the key.
- $e$: Public exponent, used for masking.
- $r$: Random integer, a randomly generated mask modulo $n$.

Standard C compilers can break constant-time code or even functionality through optimizations based on the "as-if" rule and the assumption that undefined behavior (e.g., signed overflow) never occurs, requiring defensive techniques like using unsigned types and checking assembly output. The **As-If Rule** in the C standard allows any transformation preserving "observable behavior"; notably, execution time is not considered observable behavior. **Undefined Behavior (UB)**: The C standard prescribes no behavior for certain operations (like signed integer overflow). Compilers often assume UB cannot happen, enabling aggressive optimizations. **Modular Arithmetic**: Arithmetic where numbers wrap around after reaching a modulus. C guarantees unsigned operations follow modular rules, but not signed ones.

Modern compilers are more than "portable assemblers"; they use aggressive optimizations that might remove parts intended to ensure constant-time execution. For example, a function `add` uses `int32_t` arrays for big integer addition. The code uses subtraction from `0x80000000` (min value for signed 32-bit int) to simulate unsigned comparison in a signed context. At basic optimization (`-O`), the logic is preserved. However, at higher optimization (`-O9`), the compiler notices the same constant is subtracted from both sides. Assuming signed overflow is UB (and thus won't happen), it optimizes away the "out-of-range" operation, breaking carry detection logic. Guidelines for constant-time development: avoid booleans, prefer unsigned types, and verify generated assembly.

Perform element-wise addition on two big integers represented as signed 32-bit arrays.

- **Carry Detection Logic**:
  - `cc &= (xw == zw);`: If `cc` is 1 and `xw == zw`, it means adding `yw` and old `cc` didn't change `xw`, implying `yw + cc` was 0 (overflow from a specific state).
  - `cc |= (zw - 0x80000000) < (xw - 0x80000000);`: The key line. Shifting the range allows signed comparison to behave like unsigned. If in an unsigned sense, sum `zw` is "less than" operand `xw`, a carry occurred.
- **Compiler Dilemma**: Because it uses signed integers (`int32_t`), the compiler simplifies `zw - C < xw - C` to `zw < xw`. But if `xw + yw + cc` overflows (signed UB), the compiler assumes `zw` is a valid signed integer, ignoring overflow and failing to detect the carry correctly. "Control words" (0 or 1) often enable/disable branches bitwise, but if the compiler identifies the 0/1 pattern, it might replace bitwise ops with conditional jumps. Using `-1` (all-1s bitmask) for "enabled" instead of `1` is recommended to prevent some optimizations.

<details><summary>Big Integer Addition `add()`</summary>

```c
int32_t
add(int32_t *z, const int32_t *x, const int32_t *y, size_t len)
{
    int32_t cc;
    size_t u;
    cc = 0;
    for (u = 0; u < len; u ++) {
        int32_t xw, yw, zw;
        xw = x[u];
        yw = y[u];
        zw = xw + yw + cc;
        z[u] = zw;
        cc &= (xw == zw);
        cc |= (zw - (int32_t)0x80000000) < (xw - (int32_t)0x80000000);
    }
    return cc;
}
```

</details>

Bitslicing, or "data orthogonalization," replaces secret-dependent table lookups with a sequence of bitwise operations mimicking logic circuits, providing inherent constant-time execution and high parallel performance at the cost of increased code size and register pressure. **Bitslicing** is a data representation technique where data is distributed across multiple registers by bit. Instead of one register storing one 32-bit word, 32 registers store one bit each of 32 different words. **Data Orthogonalization**: The transformation (effectively a matrix transpose) required to convert standard data to bitsliced format. **Logic Gates in Software**: Using bitwise instructions (`&`, `|`, `^`, `~`) as software equivalents of hardware logic gates (AND, OR, XOR, NOT). **Constant-time Table Lookup** is replaced by logic circuits, ensuring no memory access index depends on secret data.

`br_ccopy()` is a conditional copy tool for windowed elliptic curve point multiplication. To avoid timing leaks, Step 3 (adding P to Q if a bit is set) must execute every time, requiring a 2-bit window and constant-time table lookup. Bitslicing technique: By treating bitwise operators as logic gates, algorithms (e.g., DES S-boxes) can be implemented as boolean circuits. With no data-based conditional branches or memory accesses, this approach is naturally constant-time. Moreover, 64-bit CPUs can process 64 algorithm instances in parallel using the same instruction sequence. However, bitslicing requires many registers, generates massive code, and incurs orthogonalization overhead. It's most effective when the algorithm itself allows parallel execution (e.g., DES decryption in CBC mode).

Implementing the first DES S-box: It takes a 6-bit input (distributed over six 64-bit unsigned integers `a1` to `a6`) and produces a 4-bit output. The function is a "straight-line" sequence of 56-63 bitwise operations. No `if` statements or array lookups. This is a concrete implementation of the logic circuit approach. Each line represents a gate in the circuit. If `unsigned long` is 64-bit, this function computes results for 64 different S-boxes simultaneously. Bit $i$ of input `a1` represents the first bit of the $i$-th S-box input.

**Register Pressure**: The function uses 63 intermediate variables (`x1` to `x63`). On x86 CPUs with only 16 registers, most will be "spilled" to the stack. **Superscalar Execution**: On modern CPUs, the cost of stack-to-register moves might be negligible as the ALU is busy with bitwise ops and the CPU can interleave memory accesses. **Parallelism Limits**: CBC encryption cannot be effectively bitsliced because each block depends on the previous one, whereas CBC decryption can.

<details><summary>DES S-box bitslice function `s1`</summary>

```c
static void s1(unsigned long a1, ..., unsigned long *out4) {
    unsigned long x1, x2, ..., x63;
    x1 = ~a4;
    x2 = ~a1;
    x3 = a4 ^ a3;
    // ... (many more operations)
    x61 = x58 ^ x60;
    x62 = a5 & x61;
    x63 = x56 ^ x62;
    *out3 ^= x63;
}
```

</details>

Constant-time AES in BearSSL: BearSSL uses multiple strategies for AES, notably a "hybrid bitslicing" approach leveraging the algebraic structure of the S-box for constant-time performance across architectures.

- **AES S-box**: The non-linear component of AES, defined as the multiplicative inverse in finite field $GF(2^8)$ followed by an affine transformation.
- **Affine Transformation ( $A$ or $A^{-1}$ )**: Linear mapping plus a constant ( XOR in $GF(2^n)$ ).
- **Involution**: A function that is its own inverse. The inversion function $I(x)$ in $GF(2^8)$ is an involution ( $I(I(x)) = x$ ).
- **AES-NI**: Hardware-accelerated AES instructions on modern x86 CPUs, which are inherently constant-time and high-performance.

BearSSL provides five AES implementations: `big` (fast but side-channel risky), `small` (compact but slow), `ct` (32-bit bitsliced), `ct64` (64-bit bitsliced), and `x86ni` (hardware accelerated). The `ct` implementation uses a bitsliced S-box circuit optimized for 32 parallel instances. To reduce code footprint, a key optimization is reusing the forward S-box circuit for decryption. By algebraically representing the S-box, a derivation allows calculating the inverse S-box using the forward circuit supplemented by extra affine transformations. This approach acknowledges the performance overhead for decryption but notes that CBC decryption is parallelizable, offsetting some of the cost. `x86ni` is much faster than `ct` (up to 30x in CTR mode), highlighting the benefits of hardware-supported constant-time operations. **Parallelism Difference**: CBC decryption implementations incur extra overhead for the decryption engine since decryption is parallel while encryption is not.

AES S-box definition: The S-box is a combination of field inversion and affine mapping. This algebraic decomposition is the basis for bitsliced implementations, allowing complex S-boxes to be broken down into smaller logic gates.

$$ S(x) = A(I(x)) \oplus 0x63 $$

- $x$: Input byte (8 bits), element of $GF(2^8)$.
- $S(x)$: S-box output.
- $I(x)$: Multiplicative inverse in $GF(2^8)$, $I(0) = 0$.
- $A(\cdot)$: Affine transformation, the linear part of the S-box.
- $0x63$: Constant XOR, part of the affine mapping.

Inverse S-box reuse: To calculate the inverse S-box, one applies the inverse affine transformation, then the forward S-box circuit, and finally the inverse affine transformation again (with appropriate XOR adjustments). This allows software to contain only one large bitsliced circuit (the forward S-box), saving significant code space. The derivation is based on $I(x)$ being its own inverse. This relates to the earlier observation about large code size; reusing circuits is a key measure to reduce bitsliced code size.

$$ S^{-1}(x) = A^{-1}(S(A^{-1}(x \oplus 0x63)) \oplus 0x63) $$

- $S^{-1}(x)$: Inverse S-box output, used for decryption.
- $A^{-1}(\cdot)$: Inverse affine transformation.

Constant-time DES in BearSSL: BearSSL achieves constant-time DES by decomposing its eight unique S-boxes. These are broken down into 32 single-bit output functions (T-boxes), which share a unified recursive multiplexer tree circuit, enabling parallel execution and minimizing code redundancy. **DES S-box**: DES uses eight unique S-boxes, each taking 6-bit input and producing 4-bit output. Compared to AES using only one, this is a massive challenge for bitslicing. **Multiplexer (MUX)**: A logic operation selecting one of two inputs based on a control bit. In bitwise logic: `MUX(s, x, y) = x ^ (s & (x ^ y))`. **Shannon Decomposition**: A method for representing a boolean function as a sum of two sub-functions based on one input variable, forming the theoretical basis for the "recursive tree."

DES is difficult to bitslice because of its eight unique S-boxes. Implementing each as a separate circuit would be massive. The solution is decomposing the four 6-in-1-out "T-boxes" per S-box, totaling 32 T-boxes. Each is implemented as a recursive MUX tree with 6 layers (for 6 input bits). The 64 leaves of the tree contain the 1-bit constants defining the specific S-box function. Since the tree structure is identical for all 32 T-boxes, the code implements a universal tree circuit. These 64 constants are treated as extra inputs to the circuit. By bitslicing 32-bit words, all 32 functions are computed in parallel. This method is slower than table-based DES but ensures constant-time behavior. **Parallel Efficiency**: Despite lower performance than lookup tables (6.53 MB/s vs. 20.26 MB/s), the implementation is efficient in _conceptual_ parallelization—32 different functions are computed in one pass. **Register/Memory Tradeoff**: This approach uses 70 inputs (6 bits + 64 constants), requiring careful bitmask management to ensure "constants" are accessible in a bitsliced manner.

Recursive Multiplexer Tree: A 6-bit function is defined by using the first bit $a$ to choose between two 5-bit functions. This proceeds recursively until reaching 0-bit functions (constants). It provides a completely uniform way to implement _any_ boolean function. Uniformity is crucial for parallel processing of multiple different functions. This achieves the "straight-line code" requirement for bitslicing while solving the DES "multiple S-box" problem.

$$ T(a, b, c, d, e, f) = MUX(a, T(0, b, c, d, e, f), T(1, b, c, d, e, f)) $$

- $a, b, c, d, e, f$: 6-bit input to S-box.
- $T(\cdot)$: T-box function, 6-bit to 1-bit mapping.
- $MUX(s, x, y)$: Multiplexer, $s$ selects $x$ (if 0) or $y$ (if 1).

Constant-time GHASH (for GCM) in BearSSL: BearSSL implements constant-time GHASH by simulating carryless multiplication using standard integer multiplication with "gaps" (spaced-out bits) to prevent carry propagation between data bits. GHASH is the hash function used in GCM mode, involving multiplication in finite field $GF(2^{128})$. Carryless multiplication is a special multiplication where addition is replaced by XOR (no carry). **Polynomials over $GF(2)$**, carryless multiplication of integers is equivalent to multiplication of polynomials with bit coefficients. **Karatsuba Decomposition**: An algorithm computing the product of two $N$-bit numbers using three $N/2$-bit multiplications, recursively reducing complexity.

GHASH requires 128-bit carryless multiplication. While modern x86 CPUs have the `pclmulqdq` instruction, portable C code typically falls back to non-constant-time lookup tables. BearSSL introduces a novel technique: spacing out bits with zeros ("gaps") so that standard CPU integer multiplication (with carries) behaves like carryless multiplication at "data" bit positions.

The `bmul` implementation splits a 32-bit word into four parts, each containing 8 data bits with 3-zero bit gaps between them. it performs 16 multiplications on these parts and recombines the results. For full 128-bit GHASH, BearSSL uses Karatsuba decomposition to reduce the operation to nine 32-bit multiplications. On 64-bit systems where 128-bit results aren't directly available, an extra "bit reversal" property is used to obtain the full product.

Compute the carryless product of two 32-bit integers. It extracts every fourth bit of `x` and `y` into `x0..x3` and `y0..y3`. This creates 3-bit "gaps" between active bits. It uses standard integer multiplication (`MUL`). Due to gaps, bits can be added (represented by `^` in code, though internal `MUL` uses integer `+`), and carries won't reach the next active bit position within the 4-bit block. It masks out "overflow" carries using `0x1111...` masks and recombines. This is a practical implementation of the "multiplication with gaps" concept, mapping algebraic $GF(2)$ multiplication to standard ALU integer multiplication.

**Performance vs. Native Instructions**: `ctmul32` (this technique) is slower than native `pclmulqdq` but provides a vital constant-time fallback for older/smaller ARM or x86 systems. This method relies entirely on the assumption that the CPU's `MUL` instruction itself is constant-time.

Bit Reversal Property: $REV_N(x)$ Bit reversal of an $N$-bit word, where bit $i$ becomes bit $N-1-i$.

$$ REV_N(a \times b) = REV_N(a) \times REV_N(b) $$

If operands of a carryless multiplication are bit-reversed, the result is the bit-reversed product of the original operands. In standard C, carryless multiplication (simulated via bitwise ops) might only yield the low bits of the result. To get high bits, one can bit-reverse inputs, perform the same operation, and bit-reverse the result again. This provides a portable way to obtain the "full" carryless product. This property enables BearSSL to implement GHASH on architectures without 128-bit carryless multiply instructions.

<details><summary>Carryless Multiplier `bmul()</summary>

```c
static inline void
bmul(uint32_t *hi, uint32_t *lo, uint32_t x, uint32_t y)
{
    // ...
    x0 = x & (uint32_t)0x11111111;
    x1 = x & (uint32_t)0x22222222;
    // ... similar for y ...
    z0 = MUL(x0, y0) ^ MUL(x1, y3) ^ MUL(x2, y2) ^ MUL(x3, y1);
    // ...
    z0 &= (uint64_t)0x1111111111111111;
    // ...
    z = z0 | z1 | z2 | z3;
    *lo = (uint32_t)z;
```

</details>

ChaCha20, Poly1305, Hash Functions, HMAC: While many modern algorithms like ChaCha20 and SHA-2 are naturally constant-time due to their reliance on ARX (Addition-Rotate-XOR) operations, constructions like Poly1305 and HMAC require careful carry management and uniform instruction execution to remain timing-neutral.

**ARX Algorithms**: Cryptographic designs using only modular addition, rotation, and XOR (e.g., ChaCha20, BLAKE2). On CPUs with constant-time addition and shifts, these are inherently constant-time. **Poly1305**: A Message Authentication Code (MAC) computing a polynomial in prime field $GF(2^{130}-5)$. **HMAC**: A MAC construction using a cryptographic hash function. **Carry Spilling**: In big integer arithmetic, using more bits than strictly needed (e.g., 26 bits in a 32-bit register) to allow carries to accumulate without overflow, reducing normalization frequency.

ChaCha20 and standard hash functions (SHA-256 etc.) are "naturally" constant-time as they don't use secret-dependent conditional branches or memory indices. Poly1305 is more complex, involving big integer multiplication. BearSSL uses a "carry spilling" strategy: splitting a 130-bit value into five 26-bit words. A 6-bit "gap" in 32-bit registers allows multiple additions and multiplications before strictly needing carry propagation. For systems with limited multiplication (e.g., ARM Cortex M), BearSSL uses 13-bit words to ensure 32-bit results. For HMAC, the challenge is that standard implementations might leak message or key length. BearSSL follows Adam Langley's approach, ensuring the hash function executes exactly the same instruction sequence for any input length within a specified range, effectively masking the true length.

Poly1305 prime field computation is modulo $2^{130} - 5$. This prime's structure (close to a power of 2) allows for extremely efficient modular reduction using only shifts and additions, avoiding expensive division instructions. This efficiency makes Poly1305 a preferred constant-time MAC on various hardware. **HMAC Timing Leaks**: "MAC-then-Encrypt" (as used in old SSL) is flawed, and constant-time HMAC is vital for defending against padding oracle attacks. **Incompatible Architectures**: ARM Cortex M0/M3 highlights the need for specialized `ctmul32` implementations when standard 32x32->64 multiplication is unavailable or slow. The core of constant-time HMAC is not just avoiding branches but ensuring "exactly the same instructions" execute regardless of length.

$$ p = 2^{130} - 5 $$

, where $p$ is the Poly1305 prime, a large prime used for modular reduction.

CBC Padding and RSA: BearSSL defends against padding oracle attacks and RSA timing leaks using constant-time buffer rotation for MAC extraction and uniform modular exponentiation (executing all operations regardless of secret bit values). Padding Oracle Attack: An attack exploiting how a system handles padding errors to gain info about ciphertext (e.g., Lucky Thirteen). **Montgomery Multiplication**: An efficient modular multiplication algorithm avoiding division but potentially requiring a final "conditional" subtraction to keep the result in $[0, n-1]$. **Square-and-Multiply**: A classic modular exponentiation algorithm ($m^e \mod n$). In constant-time form, the multiplication step always executes, but results are only stored if the current exponent bit is 1. **Buffer Rotation**: Rearranging a byte array. For constant-time, the shift amount must not be exposed via memory access patterns.

To stop padding oracle attacks, BearSSL handles incoming CBC-encrypted records in strictly constant-time. This includes reading all potential padding bytes, performing constant-time buffer rotation (using an $O(n \log n)$ algorithm instead of $O(n^2)$ ) to extract the MAC, and using constant-time memory comparison. For RSA, BearSSL avoids the standard "masking" approach (which requires a good RNG) and uses a fully constant-time big integer implementation. The key is the "declared bit length"—operations depend only on the public key size (e.g., 2048 bits), not the integer's actual value. A clever one-line C snippet for conditional subtraction in Montgomery multiplication performs subtraction logic twice to ensure constant-time while only committing the result if necessary.

**Rotation Optimization**: The $O(n \log n)$ rotation algorithm is a significant improvement over $O(n^2)$ nested loops used in some other "constant-time" libraries. **RSA Prime Length Assumption**: For a 2048-bit RSA key, primes are assumed to be exactly 1024 bits. While not strictly secret, this "public length" strategy enables constant-time big integer operations.

Constant-time Montgomery Subtraction: After Montgomery multiplication, result $d$ might be in $[m, 2m-1]$. If so, $m$ must be subtracted. A simple `if (d >= m) d -= m;` leaks timing. The code `br_i31_sub(d, m, NEQ(dh, 0) | NOT(br_i31_sub(d, m, 0)))` achieves this. The "inner" call performs subtraction to check for carry/borrow (the `d >= m` condition) without modifying $d$. The "outer" call uses that result as a control signal to decide whether to actually write back the subtraction result to memory.

Conditionally subtract `m` from `d` in constant time. `br_i31_sub(d, m, 0)`: The third argument `0` means "do not write result to `d`." It just returns the carry (1 if $d < m$, 0 if $d \ge m$). `NOT(...)` flips the carry. Now it's 1 if $d \ge m$. `NEQ(dh, 0)` checks if the high word of $d$ is non-zero (indicating $d > m$ even before the subtraction check). The outer `br_i31_sub(..., control)` executes subtraction again. If `control` is 1, the result is stored in `d`. This perfectly fulfills the requirement that code paths must be identical whether or not subtraction "needs" to happen. Both subtraction operations always execute.

$$ d \leftarrow d - m \text{ if } (d \ge m) $$

- $d$: Intermediate result, product of Montgomery multiplication.
- $m$: Modulus, public value.

```c
br_i31_sub(d, m, NEQ(dh, 0) | NOT(br_i31_sub(d, m, 0)));
```

Elliptic Curves and Future Directions: BearSSL implements ECC using Jacobian coordinates and the Montgomery ladder to eliminate secret-dependent branching, while planning future architecture-specific optimizations and more efficient field operations. **Jacobian Coordinates**: A system representing point $(x, y)$ as $(X, Y, Z)$ where $x = X/Z^2$ and $y = Y/Z^3$. This allows point addition and doubling using only field multiplication and squaring, deferring modular inversion to the end. Modular inversion (needed for dividing by $Z$) is extremely slow. By using this mapping, we only need one inversion at the very end of the scalar multiplication. Since inversion is typically implemented with variable-time algorithms (like Extended Euclidean Algorithm), minimizing its count and ensuring it only happens on a final non-secret result is a key constant-time strategy. **Projective Coordinates**: Similar to Jacobian, used to avoid modular inversion. **Montgomery Ladder**: A scalar multiplication algorithm performing the same sequence of operations for every bit of the scalar, making its time complexity constant. **Fixed-point Optimization**: Precomputing multiples of a fixed base point $G$ to accelerate $kG$ computation.

For NIST curves, BearSSL uses projective Jacobian coordinates. The implementation carefully handles special cases (like point at infinity or doubling) avoiding secret-dependent conditional branches. Point addition uses a 2-bit window strategy, selecting the correct precomputed point with constant-time lookups.

Curve25519 is significantly simplified by its Montgomery ladder and "naturally" achieves constant-time code. Specialized optimizations exist for Curve P-256, like fixed-point multipliers for the traditional generator $G$. Future goals include native AES instruction sets (AES-NI), more efficient carryless multiplication (Cantor-Kaltofen), and more efficient big integer code designed for fewer registers on CPUs like Cortex M0.

While input points are validated, computations proceed even if a point is invalid, recording the validity status in a "flag." This prevents "invalid curve attacks" from leaking info via timing. **Stack Usage vs. Window Size**: For ECDSA, a 2-bit window is chosen over larger ones to keep stack usage below 3 kB, prioritizing memory-constrained embedded systems. **Superiority of Montgomery Ladder**: Curve25519's ladder is inherently more secure and easier to implement than NIST curve point addition/doubling logic.

---

## 3. Microarchitectural Security Verification

### 3.1 RTL Security Speculation Verification Based on Contract Shadow Logic

A scalable formal verification technique—Contract Shadow Logic—leverages architectural insights to verify hardware-software security contracts on RTL (Register-Transfer Level) designs of out-of-order processors by extracting ISA (Instruction Set Architecture) traces via non-intrusive auxiliary logic. **Speculative Execution Attacks**: Vulnerabilities like Spectre and Meltdown exploit microarchitectural side effects (e.g., cache state) of "transient" instructions—those executed speculatively but subsequently discarded—to leak secrets. **RTL (Register-Transfer Level)**: The stage where microarchitectural details are implemented; verification at this level is crucial but difficult due to complexity. **Hardware-Software Contract**: A formal agreement where hardware guarantees no information leak if software follows certain constraints (e.g., no secrets in addresses). **Shadow Logic (Ghost Code)**: Auxiliary logic added to a design for verification purposes that monitors system state without altering system behavior.

Current RTL verification tools (like UPEC) require massive manual effort (20,000+ lines of invariants) and deep formal expertise to handle Out-of-Order (OoO) processors. Contract Shadow Logic shifts the burden from complex logical invariants to "shadow logic" that architects can easily write, significantly reducing manual effort (hundreds of lines vs. thousands). **Contract-Centric**: Unlike previous methods that just "checked for leaks," this approach integrates _contract constraint_ checking directly into microarchitectural verification.

Security properties are formalized as conditional non-interference contracts, where if software traces are indistinguishable at the ISA level ($O_{ISA}$), hardware must ensure microarchitectural indistinguishability ($O_{\mu Arch}$), introducing model checking and shadow logic as verification mechanisms. **Non-interference**: A security property ensuring secret inputs do not influence public outputs or observable side channels. **Hyperproperty**: A property that cannot be verified on a single execution trace but requires comparing multiple traces (e.g., verifying results are the same for two executions using different secrets). **Bounded Model Checking (BMC)**: Verifying all possible execution paths of a system within a fixed number of steps ($k$). **Shadow Logic**: Auxiliary circuits added to RTL for verification that monitor state (e.g., tracking instruction commit status) without affecting the actual design.

The Contract Shadow Logic approach for RTL security verification exploits the insight that out-of-order processors contain the necessary information to reconstruct ISA execution traces at commit time, simplifying the baseline four-machine problem into a two-machine problem with auxiliary "shadow logic." **Public vs. Secret Data**: Memory is partitioned; attackers try to infer secret partitions via microarchitectural side channels. **Cycle-level Observation**: Assumes attackers can observe cycle-by-cycle events like memory bus addresses and instruction commit times. **Baseline Verification (Four-Machine)**: Standard approach requires two ISA machine copies (for software compliance) and two OoO hardware copies (for hardware security) running in parallel. **Trace Reconstruction**: The process of extracting the sequential stream of committed instructions from an out-of-order pipeline.

**Redundancy Reduction**: The "baseline" uses 4 machines. "Contract Shadow Logic" reduces it to 2. Since an OoO processor _must_ correctly implement its ISA to function, the committed instruction stream _is_ the ISA execution trace. Thus, we can derive $O_{ISA}$ directly from the Out-of-Order processor's Reorder Buffer (ROB) commit stage. This assumes functional correctness (i.e., the processor eventually commits correct instructions). This allows the security proof to focus on _timing_ and _microarchitectural side effects_ related to those commits. Shadow logic monitors the ROB and other modules (like register files or load/store queues) to capture information that might be "lost" or overwritten when an instruction commits. The number of state machines is a primary driver of model checking complexity; reducing from 4 to 2 is a significant scalability gain.

Contract Shadow Logic implements a two-phase verification scheme to overcome synchronization and instruction inclusion challenges inherent in out-of-order processors, using shadow logic to monitor architectural state, pausing clocks to realign diverging traces, and verifying hardware-software contracts via a combined assertion-assumption framework. Sound verification must ensure that if an instruction's side effects are observed (leaked), that instruction must eventually commit and be checked against the software contract. In Out-of-Order (OoO) processors, two instances with different secrets might commit instructions at different cycles (e.g., due to cache hit vs. miss). This "skew" makes cycle-by-cycle comparison of ISA traces impossible without realignment. **Two-Phase Shadow Logic**:

- **Phase 1**: Monitors for microarchitectural trace divergence ($O_{\mu Arch}$ divergence).
- **Phase 2**: Once divergence is detected, it enters a state to verify if instructions causing the divergence violate the contract.
- **ROB Tail/Head**: Metadata pointers tracking instruction progress through the pipeline, ensuring all relevant instructions are checked.

**Gated Clocks**: Shadow logic can "pause" the clock of either CPU instance. This is the mechanism used for realignment. **Phase 1 Detection**: If a microarchitectural difference occurs (`uarch_diff`), it's recorded (`uarch_diff_phase1 <= 1`) and current ROB tails are captured (`tail1`, `tail2`). **Realignment**: If one CPU commits an instruction while the other doesn't (due to skew), the committed CPU is _paused_ until the other catches up. This ensures ISA traces (`O_ISA`) stay synchronized for comparison. **Completion (`drained`)**: Phase 2 lasts until both CPUs have committed the instructions that were in flight when divergence was first detected. **Verification**: `assume (isa_diff == 0)` tells the model checker to only consider programs that behave identically architecturally (satisfy the contract). `assert (!(uarch_diff_phase1 && drained))` means if the contract is met, there should be no microarchitectural divergence once all relevant instructions have committed.

While shadow logic is usually passive, the "pause" mechanism actively modifies clocks. This is valid as it only happens _after_ a security-relevant event (divergence) is detected and doesn't change execution results, only their timing relative to each other. **Scalability Trick**: By using `assume` on ISA differences, the model checker's search space is pruned to only focus on contract-compliant programs. The **drained state** is vital for correctness; without it, the property might "fail" before the contract-violating instruction actually reaches the commit stage. The logic for **superscalar processing** assumes one instruction per cycle; for superscalar processors, shadow logic needs more complexity to handle partial commits and unaligned traces within a single cycle.

<details><summary>Verilog Pseudo-code</summary>

```verilog
1 cpu cpu1 (.clk(pause1 ? 0: clk), .rst(rst));
2 cpu cpu2 (.clk(pause2 ? 0: clk), .rst(rst));
...
6 uarch_diff = (O_uarch(cpu1) != O_uarch(cpu2));
7 isa_diff = (O_ISA(cpu1) != O_ISA(cpu2));
...
18 if uarch_diff_phase1 then // Entering Phase 2
20     drained = (tail1 == cpu1.head-1) && (tail2 == cpu2.head-1);
...
24     if (cpu1.commit == cpu2.commit) { pause1 <= 0; pause2 <= 0; }
27     else if (cpu1.commit) pause1 <= 1; // Realignment Logic
29     else if (cpu2.commit) pause2 <= 1;
...
34 assume (isa_diff == 0); // Contract Check
36 assert (!(uarch_diff_phase1 && drained)); // Leakage Check
```

</details>

### 3.2 Pre-silicon Coverage-Guided Fuzzing for Open-Source Processors Based on Leakage Contracts

A scalable coverage-guided fuzzing approach for pre-silicon hardware-software contract verification uses a novel "Self-Compositional Deviation" (SCD) metric to detect microarchitectural state differences caused by secret data. There's a gap between formal contract verification (rigorous but non-scalable) and traditional hardware fuzzing (scalable but unable to detect side-channel leaks like Spectre). To bridge this, a self-compositional testing framework runs two processor instances in synchronization with different secret data. The core innovation is the **Self-Compositional Deviation (SCD)** coverage metric, which tracks microarchitectural state differences to guide the fuzzer toward execution paths potentially violating security contracts. Evaluations on open-source RISC-V cores: Rocket (sequential) and BOOM (out-of-order). The method relies on microarchitectural state visibility common in pre-silicon RTL simulation but unavailable in post-silicon commercial hardware. The "contract-aware" feature distinguishes this from traditional differential fuzzing tools that only compare architectural state (registers and memory). The SCD metric specifically prioritizes exploration of security-relevant state space.

Coverage-guided contract fuzzing is a feedback-driven workflow using executable leakage contracts in Sail and a novel SCD metric to guide microarchitectural state exploration for potential security vulnerabilities. **Sail**: A DSL for specifying ISA semantics used to implement executable contracts. **Grey-box Fuzzing**: A technique using some internal system state info (like code coverage) to guide input generation. **Self-Compositional Deviation (SCD)**: A metric measuring microarchitectural state differences between two processor instances running different data. **Rolling Hash**: An efficient hashing technique for data sequences used to track state transitions over time.

Five-stage fuzzing pipeline:

- Generation/Mutation
- Contract
- RTL Simulation
- Leakage/Coverage Analysis
- Processing/Prioritization

The system uses a **Sail-based contract** as the golden model. Leakage is found if two inputs yield identical contract traces but differing hardware behavior (detected via self-composition in RTL simulation). The **SCD metric** is introduced; instead of traditional code coverage, SCD tracks which register state differences ($\Delta$) are triggered by secret data during execution cycles. To manage the vast state space, it uses a hashing mechanism to store state transitions in a 2 MiB bit-vector.

`Seq-ct` - Load instruction leakage: Under the `seq-ct` contract, a load instruction leaks the instruction address (PC) and the address of data being loaded. This formalizes the constant-time assumption that data addresses must not depend on secrets. This rule is implemented in the Sail simulator for generating "allowed" leakage traces.

$$ \sigma, i \ rd \ rs1 \ imm →*p \sigma' \quad l=\{(lAddr, a(rs1)+imm), (pc, a(pc))\} $$
$$ \sigma, i \ rd \ rs1 \ imm \rightharpoonup^l*{p, \text{seq-ct}} \sigma' $$

- $lAddr$: Load address label, leakage category.
- $a(rs1)+imm$: Memory address, the actual accessed address.
- $pc$: Program counter label.
- $a(pc)$: PC value.

State space deviation ($\Delta$): Deviation $\Delta$ is the set of all registers with differing values in two synchronous simulations. In a secure system, if only secret data differs, microarchitectural state shouldn't diverge in ways affecting timing. $\Delta$ captures this divergence.

$$ \Delta(s_A, s_B) = \{r \mid r \in Registers, s_A[r] = s_B[r]\} $$

- $s_A, s_B$: States of two processor instances running the same program but with different data.
- $r$: Registers in the design, including microarchitectural ones.
- $\Delta(s_A, s_B)$: Deviation set, the collection of differing registers.

SCD Coverage Hashing: A bit in the coverage vector is set by hashing the current state deviation and that of the previous cycle. This tracks _transitions_ between different types of state divergence, providing more meaning than just tracking which registers differ; it tracks the sequence of how differences propagate. This provides the feedback signal for the fuzzer. A new bit set in `cov` indicates a previously unexplored execution path.

$$ cov[hash(\Delta(s_A, s_B)) \oplus hash(\Delta(s_A, s_B)_{t-1}) \gg 1] := 1 $$

- $cov$: Coverage bit-vector, 2 MiB ($2^{24}$ bits).
- $\Delta(s_A, s_B)_{t-1}$: Deviation from the previous cycle.
- $\oplus$: XOR operation.
- $\gg 1$: Right shift, part of rolling hash logic.

Cumulative Coverage: Total coverage is the logical OR of coverage vectors from all explored test cases, allowing the fuzzer to determine if a new test case provides "new" coverage (i.e., hits a bit not yet set in `cumulative-cov`).

$$ \forall i : cumulative-cov[i] = \bigvee*{tc \in TC} cov*{tc}[i] $$

A hashing mechanism (SHAKE128) maps the potentially massive state space of all design registers to a fixed-size bit-vector. The SCD metric is cycle-accurate, meaning it captures timing-related differences occurring anywhere in the pipeline. The `seq-arch` contract is significantly more permissive (leaking data values), used for sandboxing untrusted code where all data is treated as public.

Fuzzing pipeline implementation and design decisions: Iterative fuzzing loop, specialized mutation and merging strategies for programs and data, and prioritization algorithms favoring test cases with rare microarchitectural coverage. **Mutator**: A fuzzer component modifying existing "seed" inputs to create new test cases. **Test Case Prioritization**: The process of selecting seeds from the corpus most likely to yield new results after mutation. **Verilator**: A tool compiling Verilog code into cycle-accurate C++ models for fast simulation. **Weighted Feedback**: A strategy giving higher priority to test cases covering "rare" features (bits in the coverage vector). **Corpus**: A set of interesting test cases maintained by the fuzzer used as seeds for further mutation.

The fuzzing loop implementation uses a **Sail-based contract checker** to filter out "contract-distinguishable" inputs (those already leaking at the ISA level) before running time-consuming RTL simulations. **Program Generation** is based on DifuzzRTL but limited to forward jumps to ensure termination. **Data Generation** uses three strategies, including a "50-50 random equal" method balancing randomness and data similarity to increase chances of passing contract checks. **Prioritization** logic is a key contribution, implementing four strategies: pass-through, new-coverage, weighted, and size-constrained pass-through feedback. **Weighted Feedback** calculates scores based on the inverse frequency of coverage bits triggered by each test case.

Coverage element weight: A coverage bit's weight is inversely proportional to the number of test cases in the corpus triggering that bit. Rare bits (low $n_c$) get high weights, incentivizing the fuzzer to focus on "unusual" microarchitectural behavior rather than common ones.

$$ W(c) = \frac{1}{n_c} $$

- $c$: Coverage element (bit in the vector).
- $n_c$: Number of test cases in the corpus covering $c$, $n_c = |\{tc \in Corpus \mid cov_{tc}[c]=1\}|$.
- $W(c)$: Weight of element $c$.

Test case score: A test case's score is the sum of weights for all coverage bits it triggers. Test cases triggering many rare bits get higher scores, making them better candidates for mutation.

$$ Score(tc) = \sum\_{c \in C(tc)} W(c) $$

- $C(tc)$: Set of coverage bits triggered by test case $tc$.
- $Score(tc)$: Total score of test case $tc$.

Prioritization probability: The probability of selecting test case $tc$ for mutation is its normalized score relative to the whole corpus. This implements a "power-law schedule" biasing search toward more valuable seeds.

$$ p*{tc} = \frac{Score*{tc}}{\sum\_{t \in Corpus} Score_t} $$

- $p_{tc}$: Probability of selecting test case $tc$ for mutation.
- $Score_{tc}$: Score of test case $tc$.
- $\sum_{t \in Corpus} Score_t$: Total score sum of the corpus.

The `Fuzzing Loop` is the main coordination loop. it uses the contract simulator (`run_contract_check`) to check inputs; if the program is "contract-indistinguishable," it runs cycle-accurate RTL simulation. If RTL simulation detects a leak (difference in termination time), the error is saved. Finally, global coverage is updated. `run_contract_check` implements the Contract Trace and filters out programs satisfying the Contract Satisfaction precondition. `run_rtl_sim` generates the coverage vector $cov$ defined in SCD Coverage Hashing.

Using `~cum_coverage & cov` is a standard grey-box fuzzing method to detect "new" bits. `update_data_seed_energy` is used to discard invalid data segments, crucial for contracts like `seq-arch` which are hard to satisfy. The "weighted feedback" method directly borrows from AFL's evolutionary algorithms but is tuned for microarchitectural state differences.

<details><summary>Fuzzing Loop</summary>

```python
def fuzz(num_iter):
    cum_coverage = bitarray(repeat(0, 2 ** 24))
    for i in range(0, num_iter):
        (program, (data_a, data_b)) = mutator.get()
        (hsc_input, rtl_input) = compile(program, data_a, data_b)

        # Contract-distinguishability check
        ret = run_contract_check(hsc_input)
        mutator.update_data_seed_energy(program.get_seed(), ret)
        if ret == CONTR_DIST:
            save_mismatch(i)
            continue

        # RTL simulation
        (ret, cov) = run_rtl_sim(rtl_input)
        if ret == LEAK:
            save_leak(i)

        # Check for new coverage
        new_coverage = ~cum_coverage & cov
        if new_coverage.any():
            write_cov_log()
            program.save()
            mutator.add_corpus(program)
```

</details>

Coverage-guided fuzzing, particularly "weighted feedback," significantly improves microarchitectural state exploration efficiency and faster detection of security contract violations in complex out-of-order processors. **Rocket Core**, a five-stage sequential RISC-V processor, serves as a "hard target" expected to resist basic timing side-channel attacks. **BOOM (Berkeley Out-of-Order Machine)**, a complex ten-stage out-of-order RISC-V processor, is a "bug-finding" benchmark known to be susceptible to speculative execution side channels. **Mann-Whitney U test**: A non-parametric statistical test used to determine if the distributions of two datasets differ significantly. **p-value** < 0.05 indicates the performance gain of one fuzzing strategy over another is statistically significant.

The evaluation addresses two main questions: impact on **cumulative coverage** and impact on **vulnerability detection speed**. For Rocket, the "weighted feedback" strategy's median coverage is nearly double that of the unguided "pass-through feedback" strategy. For BOOM (vulnerable to `seq-arch` violations), "weighted feedback" detected the first vulnerability in 194 test cases on average, significantly faster than the 1155 required by the baseline. Results highlight a **negative correlation** between coverage and vulnerability detection time: strategies exploring more microarchitectural state find vulnerabilities faster.

The method is compared to three main areas:

- **Contract Verification**: Formal proofs exist but lack scalability.
- **Post-silicon Fuzzers (e.g., Revizor)**: These treat hardware as a black box and use side-channel "gadgets" to detect vulnerabilities. Accessing internal RTL state is a more direct approach.
- **Pre-silicon Fuzzers**:
  - **DifuzzRTL**: Focuses on functional bugs (correctness).
  - **SIGFuzz**: Searches for time-dependent behavior but is not "contract-aware."
  - **SpecDoctor**: Targets Spectre-class vulnerabilities but uses static analysis to identify vulnerable components.
