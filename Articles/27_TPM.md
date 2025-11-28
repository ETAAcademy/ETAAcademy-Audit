# ETAAcademy-Audit: 27. Trusted Platform Module

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>27 TPM</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>TPM</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# A Unified Trust Architecture: TPM-Based Attestation, Cloud FPGA Security, and Quantum-Enhanced Entropy

TPM (Trusted Platform Module) is the hardware root of trust for computing platforms. TPM 2.0 introduces pluggable cryptographic algorithms and richer policy controls, combining dedicated crypto engines, isolated secure storage (PCRs/NV), and trusted execution to deliver measured boot with policy binding.

PCRs track the boot chain through irreversible extends, while remote attestation uses an Attestation Key (AK) to sign selected PCRs plus a nonce; verifiers validate the AK/EK certificate chain, signature, allowlisted PCR values, and nonce to establish device integrity. The key hierarchy comprises the Endorsement Key (EK) for factory identity, the AK for attestation, and the Storage Root Key (SRK) for protecting application keys. TPMs are available as discrete chips (dTPM), firmware implementations (fTPM), and virtual instances (vTPM).

For cost‑sensitive deployments, a hybrid architecture pairs server‑side dTPM with client‑side vTPM to combine measurement, attestation, behavior monitoring, and strict startup checks for end‑to‑end trust. In cloud‑FPGA environments, a user‑controlled vTPM leverages PUF‑based identity, Secure Boot, session‑key rotation, and extended commands to measure deployment and invocation into PCR8/9/10, enabling verifiable, dynamic customization.

To address weak entropy, a QRNG‑enhanced software TPM (QTPM) and Quantum Entropy as a Service (QEaaS) override the random source at the TSS layer, add post‑quantum, end‑to‑end encryption and authentication, and provide a cloud entropy engine with multi‑source aggregation and health checks—so one trustworthy source secures the whole system. This materially improves randomness and availability, strengthening EK/SRK generation, session keys, and the reliability of remote attestation.

---

## 1. Trusted Platform Module (TPM): Architecture, Components, and Security Functions

A **Trusted Platform Module (TPM)** is a dedicated hardware security component designed to provide secure key storage, attestation, measured boot, and cryptographic operations. In modern computing platforms, the TPM serves as a **root of trust**, anchoring system integrity and safeguarding sensitive cryptographic material. It underpins security features such as secure boot, remote attestation, disk encryption, and platform identity.

TPM technology has evolved significantly over the past two decades. The industry often distinguishes between:

- **TPM 1.2** — the legacy standard (circa 2009), fixed to RSA and SHA-1, limited functionality, and primarily designed for traditional PCs.
- **TPM 2.0** — the modern, extensible standard (2014–present), supporting algorithm agility (RSA, ECC, SHA-256, SM-series algorithms), flexible policies, and expanded authorization mechanisms.

Today, TPM 2.0 is the dominant and actively evolving foundation for hardware-based trust in personal devices, servers, and cloud environments.

---

### TPM 2.0 Architecture

TPM 2.0 consists of three major subsystems that together provide strong cryptographic isolation and verifiable system integrity.

#### Cryptographic Engine

The internal crypto engine implements:

- RSA (usually 2048-bit)
- ECC, especially **NIST P-256**
- SHA-256 hashing
- HMAC
- A hardware True Random Number Generator (TRNG)

This engine executes all cryptographic operations, including signing, hashing, key derivation, and random number generation. Private keys generated inside the TPM **never leave the secure boundary**.

#### Isolated Secure Storage

TPM internal storage holds:

- Key hierarchy roots (e.g., **SRK**, **EK**, **AK**)
- **Platform Configuration Registers (PCRs)**
- Policy digests
- Non-volatile (NV) storage objects

Key materials inside the storage hierarchy are sealed to the chip and cannot be extracted.

#### Trusted Execution Logic

This subsystem enforces:

- PCR extend rules
- Key authorization policies
- Command access control
- Attestation workflows

It ensures that even software with high system privileges cannot compromise keys or manipulate measurements.

---

### Platform Configuration Registers (PCRs)

PCRs are one of the most crucial TPM structures. They store **irreversible, cumulative hash measurements** representing the system’s boot chain.

#### PCR Characteristics

- TPM 2.0 provides **24 PCRs**, each typically **256 bits** (SHA-256).
- PCRs cannot be directly written; they are updated via:

```
PCR[i] = SHA256( PCR[i] || new_measurement )
```

This creates an **append-only hash chain**.

#### **Role of PCRs**

PCRs record measurements from:

- BIOS/UEFI firmware
- Secure Boot configuration
- Bootloader
- Kernel and initrd
- Critical system drivers

These values form the basis for **Measured Boot** and **remote attestation**.

#### Example PCR Usage (common conventions)

| PCR | Typical Contents              |
| --- | ----------------------------- |
| 0   | BIOS/UEFI boot block hash     |
| 1   | Firmware & configuration      |
| 7   | Secure Boot certificate chain |
| 8+  | Bootloader, kernel, initrd    |

---

### Non-Volatile Storage (NV Index / NVRAM)

The TPM also includes a controlled **non-volatile memory region** used for:

- Storing policy digests
- Maintaining TPM lock/unlock state
- Application-specific secrets or counters

NV indices support fine-grained permissions and can require a password, an authorization policy, or specific PCR states to permit reads or writes.

---

### TPM Key Hierarchy: EK, AK, SRK

TPM organizes its keys into functional roles. The three most important are:

#### EK — Endorsement Key

- A unique, non-exportable identity key burned into the TPM at manufacture.
- Typically RSA-2048 or ECC P-256.
- The manufacturer issues an **EK certificate**, proving the key originated from a genuine TPM.
- Used **only for identity and provisioning**, not for routine signatures, to avoid privacy leaks.

#### AK — Attestation Key

- A TPM-generated key pair used exclusively for **attestation**.
- Public portion can be exported; private key never leaves the TPM.
- Tied to the EK chain to prove it originated from an authentic TPM.
- Used to sign PCR values during **remote attestation**.

#### SRK — Storage Root Key

- Root of the TPM’s internal key hierarchy.
- Used to encrypt and seal all subordinate keys.
- Critical in disk encryption systems like Microsoft BitLocker: the Volume Master Key (VMK) can be sealed to TPM state and policies.

---

### Remote Attestation (Quote Operation)

Remote attestation allows an external verifier to confirm that a machine is running in a trusted, unmodified state. The TPM’s **Quote** command signs the system’s PCR values using the AK.

#### Attestation Workflow

- **Verifier sends a challenge nonce**

  ```
  Give me a quote of PCR[0,1,7] using your AK.
  nonce = random(256-bit)
  ```

- **TPM constructs the quote data**

  Includes:

  - selected PCR values
  - nonce
  - optional timestamp

- **TPM signs the quote with the AK**

- **Device returns**

  - quote data
  - signature
  - AK public key
  - optional EK certificate

- **Verifier validates**

  - AK → EK → OEM root certificate chain
  - Signature integrity
  - PCR values against an allowlist
  - Nonce freshness (prevents replay)

If all checks pass, the device is proven to be in a trusted configuration.

---

### Types of TPM Deployments

TPMs come in three major deployment models:

#### dTPM — Discrete TPM

- Physically separate chip
- Highest isolation and strongest security
- Common in enterprise servers

#### fTPM — Firmware TPM

- Implemented in CPU firmware (e.g., Intel PTT, AMD fTPM)
- Most laptops and desktops use this
- Lower isolation but highly available

#### vTPM — Virtual TPM

- Software-emulated TPM provided to virtual machines
- Used in cloud platforms and embedded virtualization
- Security depends heavily on the underlying hypervisor and host integrity

```
                ┌──────────────────────────┐
                │      TPM 2.0 Module      │
                ├──────────────────────────┤
                │  Crypto Engine           │
                │  True RNG                │
                │  Secure Storage          │
                ├──────────────────────────┤
                │  EK  (identity)          │
                │  AK  (attestation)       │
                │  SRK (key hierarchy)     │
                │  PCRs (boot state)       │
                │  NV Storage              │
                └──────────────────────────┘

        Quote = Sign_AK( PCR_values || nonce )
```

---

## 2. Virtual TPM Architectures and Their Evolution in Modern Trusted Computing

Virtual Trusted Platform Modules (vTPMs) have become essential in virtualized and distributed environments where deploying a physical TPM chip on every node is impractical. Over the past two decades, vTPM architecture has evolved into three major categories—hardware-anchored vTPM, TEE-isolated vTPM, and cloud-managed vTPM services—each addressing different challenges in trust establishment, scalability, and deployment flexibility.

#### Hardware-Anchored vTPMs: Trust Injection from Physical TPMs

Early and foundational approaches—such as those by Perez et al. (2006) and Benedictis et al. (2024)—treat virtual TPMs as cryptographic state machines whose trustworthiness must ultimately derive from a **physical TPM (dTPM)**. Since a vTPM lacks an intrinsic hardware root of trust, these designs rely on “**trust injection**” from the physical TPM through mechanisms such as:

- **Endorsement Key (EK) Linking**
- **Attestation Key (AK) Linking**
- Using physical TPM storage keys or primary keys to **encrypt and seal vTPM state**
- Binding vTPM lifecycle to a single hardware node

In this model, the physical TPM remains the ultimate trust anchor: if the physical TPM is trusted, any vTPM instance cryptographically derived from it inherits that trust.
**Limitation:** Hardware anchoring fundamentally ties a vTPM to a single physical machine. This prevents **live migration**, complicates cluster deployments, and limits scalability in cloud environments.

#### vTPMs: Software TPM Within Hardware-Enforced Enclaves

When physical TPM anchoring is inflexible or unavailable—especially in large virtualized or heterogeneous environments—researchers turn to hardware-based **Trusted Execution Environments (TEEs)** such as **Intel SGX** or **Microsoft VSM/VBS**.

Examples include:

- **eTPM (Sun et al., 2018)**
- **svTPM (Wang et al., 2023)**
- Kim & Kim (2019)

The key idea:
A **software TPM** runs inside a TEE, leveraging enclave protections to provide confidentiality, integrity, and attestation services commonly associated with physical TPMs.

TEE-based vTPMs can support:

- Secure state sealing inside enclaves
- Remote attestation using TEE attestation keys
- Migratable vTPM instances (migrating the enclave state)
- TPM-like isolation without requiring hardware TPMs on each node

**Advantages:**

- Greater deployment flexibility
- Hardware-level isolation without physical TPM dependency
- Better support for vMotion / live migration and virtualized cloud settings

**Limitations:**

- TEEs are still **single-machine roots of trust**
- SGX/VBS availability depends on CPU vendor, firmware, memory model
- Not suitable for high-assurance multi-party distributed trust

This trend reflects a growing shift toward **“soft TPM + hardware isolation”** instead of relying on actual physical TPMs.

#### Cloud-Managed / Centralized vTPM Services

More recent approaches—including CoCoTPM (2022) and cloud vendor implementations such as **AWS NitroTPM** and **Azure vTPM**—treat the TPM not as a local device, but as a **cloud-native security service**.

Key characteristics:

- vTPMs operated as **multi-tenant services**
- Central certificate authority (CA) for EK/AK issuance
- API-based provisioning, attestation, and key management
- Separation of trust anchor from local hypervisors
- Integration with cloud orchestration and identity systems

This architecture is the natural progression toward **TPM-as-a-Service**, enabling:

- Scalability across thousands of VMs
- Uniform security policy management
- Centralized auditing & lifecycle control
- Simplified key provisioning for cloud-native workloads

---

### 2.1 Trusted Computing in Embedded Networks: Combining dTPM, vTPM, and Attestation

In embedded or industrial IoT systems—robotics, drones, industrial controllers—devices are often **low-cost, resource-constrained, and physically exposed**. Deploying a hardware TPM on every node is prohibitively expensive. A practical architecture therefore uses:

- **A few central nodes with hardware TPMs (dTPM)**
- **Most endpoint nodes using vTPMs**
- **A central attestation and anomaly detection backend**

A typical system architecture:

```
Server (with hardware TPM)
   |
   |-- IBM Attestation Client/Server (IBMACS)
   |
   +-- Integrity Violation Handler (AD Agent)
   |
   +-- Network Traffic Anomaly Detection
   |
Client Nodes (mostly vTPM + Custom Measurement Kernel)
```

#### **Layered Security Strategy**

Security is enforced through a **three-layer defense model**:

**(1) Measured Boot (Device Integrity at Startup)**

Each software module is hashed before execution. The measurements are extended into PCR registers:

- Bootloader
- Kernel
- Drivers
- Mission-critical applications
- Config files and scripts

Because embedded systems often rely on custom applications, many deployments use a **Custom Measurement Kernel** to ensure all application components are included in the measurement chain.

**(2) Remote Attestation (Proof of Integrity to the Server)**

Using IBMACS (IBM Attestation Client/Server):

- The client reports PCR values and measurement logs
- The server validates against a **trusted baseline**
- The server determines whether the node is trustworthy

**Limitation addressed:** IBMACS typically attests only the OS; embedded deployments extend it with custom measurement to cover application components.

The **Activation Agent** further enforces policy during boot:
If an application’s measurement does not match the expected hash, the application is prevented from executing.

**(3) Behavioral / Network Anomaly Detection**

Even a device that passes integrity checks may behave maliciously after compromise. A separate anomaly detection system monitors:

- Traffic patterns and frequencies
- Unexpected endpoints
- DOS-like behavior
- Backdoor communication
- Deviations from normal service behavior

The **Database-Based Anomaly Detection Agent** queries known benign change patterns; if an unknown anomaly occurs, the node is quarantined.

Combined, these three layers provide a strong trust architecture for low-cost embedded networks using mixed dTPM + vTPM deployments.

---

### 2.2 User-Controlled vTPM for Secure and Verifiable Cloud FPGA Deployment

Cloud vendors such as **AWS, Azure, and Alibaba Cloud** now provide **Cloud FPGA** services (e.g., Xilinx Zynq UltraScale+). A Cloud FPGA places programmable logic hardware (FPGA fabric) in the cloud and allows users to rent hardware acceleration on demand. The core value of an FPGA is its **IP core** (hardware design). However, users often hesitate to upload valuable IP to the cloud because of two concerns:

- **Will the cloud provider (CSP) access or copy my FPGA bitstream / IP core?**
- **Can I trust the FPGA-SoC boot process and runtime when the CSP controls the hardware and software stack?**

Unlike CPUs, **FPGA-SoC** platforms contain both ARM processors and dynamically reconfigurable FPGA logic. Protecting the **Secure Boot**, **dynamic loading of user IP**, **measurement of bitstreams**, and **remote attestation** is far more complex than on traditional systems. Unfortunately, existing TPM technologies—**dTPM, fTPM, and conventional vTPM**—do **not** solve the FPGA-specific “dynamic IP deployment measurement” problem.

#### **Why Traditional TPMs Fail on Cloud FPGA**

**dTPM (Discrete TPM)**

A physical TPM chip works well on personal computers, but **data centers dislike** it:

- Hard to manage at scale
- Not virtualizable
- Vulnerable to physical probing
- Keys could be extracted when an attacker gains physical access

**fTPM (Firmware TPM inside ARM TrustZone)**

fTPM is feasible on FPGA-SoCs, but **controlled entirely by the CSP**, not the user.

- CSP can spoof TPM measurements
- Firmware TPM has no way to attest FPGA dynamic reconfiguration honestly

**Traditional vTPM (Virtual TPM in hypervisor)**

Classic vTPM is implemented in CSP-controlled virtualization layers.

- Completely untrusted from the user’s perspective
- Cannot prove anything about FPGA bitstream loading
- Cannot measure FPGA-side dynamic events

#### **FPGA-vTPM: A User-Controlled TPM for FPGA-SoC TEE**

The proposed solution is a **User-Controlled vTPM**, referred to as **FPGA-vTPM**.
This is different from traditional vTPMs used in VMs:

- It runs under **user control**, not CSP control.
- It is cryptographically bound to the FPGA-SoC TEE.
- It supports extended TPM commands that measure FPGA-specific runtime events.

This architecture integrates **SRAM PUF**, TPM measurement logic, and secure session key generation to provide full trusted execution for cloud FPGA workloads.

Key features include:

- PUF-based device identity
- TPM 2.0 measurement and attestation for bitstream deployment
- PCR extensions for IP invocation
- PUF-tied session keys for secure communication
- Fully verifiable FPGA dynamic reconfiguration

#### **System Architecture Overview**

A secure FPGA-SoC TEE for cloud environments consists of five major components:

| Component                     | Role                                                                                         |
| ----------------------------- | -------------------------------------------------------------------------------------------- |
| **User vTPM (local)**         | A TPM 2.0 instance controlled by the user; initiates commands and verifies cloud FPGA state. |
| **TPM-Agent (cloud REE)**     | A simple forwarder that relays encrypted TPM messages.                                       |
| **TMM (cloud TEE)**           | The _Trusted Management Module_: controls bitstream loading, IP invocation, PCR extensions.  |
| **SRAM PUF (cloud FPGA)**     | Provides an unclonable hardware identity (CRP set).                                          |
| **Trusted Third Party (TTP)** | Provides certificates, initializes devices, signs boot images, and anchors trust.            |

Communication flows:
**User Node → vTPM → TPM-Agent → FPGA-SoC (TMM + PUF)**

---

#### **Phase 1: Initialization (The Most Critical Stage)**

Initialization consists of three coordinated steps.

**(1) Device Enrollment**

Two parallel processes occur:

(a) FPGA-SoC Enrollment

- The vendor collects FPGA device identity.
- A full bootable image (FSBL, bitstream PUF, OP-TEE, Linux, etc.) is packaged and signed.
- All SRAM PUF CRPs are extracted and registered.
- The TTP’s public key (**PK_TTP**) is embedded into the secure boot chain.

This ensures that the entire FPGA boot flow becomes **measurable and verifiable**.

(b) vTPM Enrollment

- For each user, TTP generates a unique TPM identity pair `(PK_TPM, SK_TPM)`.
- The TPM public key is certified: `Cert_TTP(PK_TPM)`.
- Delivered to the user, forming a trusted anchor for subsequent remote attestation.

Now the cloud FPGA has a **PUF-based hardware identity**, while the user has a **TPM-based software identity**.

**(2) Launch**

- FPGA-SoC boots following the TTP-signed secure boot chain.
- User-side vTPM loads certificates and TTP anchors for verifying FPGA identity.

**(3) Mutual Remote Authentication + Session Key Establishment**

- vTPM authenticates the cloud FPGA’s identity (PUF-based).
- FPGA authenticates the user’s vTPM identity (certificate-based).

The FPGA generates a PUF response **R₁**, which guarantees:

- The hardware is genuine
- The session is bound to this unique FPGA
- Keys cannot be cloned or replayed

A session key is established:

```
SessKey = Hash(R1 || TPM_nonce || FPGA_nonce)
```

This key protects all future commands.

#### Phase 2: Secure Session Key Rotation

When the user requests it—or when a vTPM-internal counter reaches a threshold—the session key is refreshed:

- vTPM reads PCR0–PCR23.
- A new challenge C₂ is issued.
- FPGA uses a _fresh, unused_ PUF CRP.
- A new session key is generated:

```
SessKey_new = Hash(ShaVal || H1 || R2)
```

This ensures:

- Keys are unique and non-repeatable
- Security continuously evolves
- Keys remain bound to PUF identity & current system state

#### Phase 3: Secure Runtime FPGA Customization

This is the core value of the entire system: **making cloud FPGA reconfiguration verifiable and trustworthy**.

All sensitive FPGA actions are measured via TPM PCRs:

| Operation                 | PCR       |
| ------------------------- | --------- |
| Secure Boot               | PCR0–PCR7 |
| IP / Bitstream Deployment | PCR8      |
| IP Invocation (inputs)    | PCR9      |
| IP Invocation (outputs)   | PCR10     |

Thus, from system boot → bitstream upload → every hardware invocation is remotely attestable.

#### Phase 4: TPM 2.0 Extended Commands for FPGA

Custom TPM commands are added to support FPGA-specific functionality.

**(1) Update_CMD**

Triggers session key rotation.

**(2) Deploy_CMD**

Securely uploads the user’s bitstream (IP core).

- Returns `SHA3-384(bitstream)`
- Updates:
  `PCR8 = Hash(PCR8 || SHA3(bitstream))`

This proves that a particular user bitstream was loaded.

**(3) Invoke_CMD**

Executes a user IP.

Inputs include:

- Input data
- Flags
- Memory addresses (input/output buffers)

Measurements:

- PCR9 records the input
- PCR10 records the output

Outputs are returned securely to the user.

**A Comprehensive Security Framework for Cloud FPGA**

| Feature                     | Mechanism                 |
| --------------------------- | ------------------------- |
| Trusted Boot                | Secure boot + PCR0–PCR7   |
| FPGA Hardware Identity      | SRAM PUF                  |
| User-Side Control           | vTPM (not CSP-controlled) |
| Secure Bitstream Deployment | Deploy_CMD + PCR8         |
| Verifiable IP Execution     | Invoke_CMD + PCR9/10      |
| Confidential Communication  | PUF-derived session keys  |
| Ongoing Protection          | Session key rotation      |

---

## 3. Quantum-Enhanced TPM and QEaaS for Securing Low-Entropy IoT Devices

IoT devices typically operate under extreme resource constraints: tiny CPUs, limited memory, no disks, and very small batteries. More importantly, they lack strong entropy sources. With no keyboard, mouse, disk noise, or other classical sources of randomness, the entropy available to IoT devices is restricted to clock jitter, RF noise, ADC fluctuations, and simple noise sensors. These sources are weak, environment-dependent, and often predictable.

Yet **cryptography fundamentally depends on randomness**—key generation, IVs, nonces, authentication challenges, attestation tokens, and secure sessions all rely on unpredictable random numbers.
Low entropy in IoT → weak keys → predictable or replayable nonces → compromised cryptographic protocols → full-chain security collapse.

### The Randomness Crisis in IoT TPMs

TPMs inside IoT devices face three structural limitations:

**(1) Hardware TPMs Are Too Expensive**

A discrete TPM chip costs several dollars—far too expensive for a $1 IoT sensor.

**(2) Software TPMs (vTPMs) Depend on System Entropy**

vTPMs pull randomness from the OS entropy pool, but IoT devices have the _weakest_ entropy pools of any computing platform.

**(3) Classical Noise Is Not Quantum-Safe**

Traditional TRNGs rely on classical physical noise, which may become predictable with future quantum-sensing technologies.

As a result, **IoT TPMs frequently generate low-quality cryptographic keys**, undermining all TPM functions:
EK/SRK generation, nonce creation, sealing, session keys, PCR replay protection, and secure boot flow integrity.

---

### Quantum Randomness: The Ultimate Entropy Source

Quantum Random Number Generators (QRNGs) leverage _truly nondeterministic_ quantum events:

- single-photon beam-splitters (|0⟩ or |1⟩ outcomes)
- vacuum fluctuations
- spontaneous emission noise
- phase noise interference

Properties:

| Property            | Explanation                                      |
| ------------------- | ------------------------------------------------ |
| **True randomness** | based on indeterministic quantum measurements    |
| **Unpredictable**   | impossible to reproduce or bias                  |
| **Quantum-safe**    | quantum computers cannot invert the noise source |
| **Miniaturized**    | now available as compact chip-level modules      |

QRNGs are therefore **the ideal solution** to the IoT low-entropy problem.

---

### QTPM: A Software TPM Upgraded with Quantum Randomness

**QTPM (Quantum-Enhanced TPM)** integrates QRNG entropy into a software TPM (vTPM).
It replaces the weak system entropy with high-grade quantum entropy while preserving full TPM compatibility.

**How It Works**

- QTPM overrides the TSS/ESAPI layer implementation of `TPM2_GetRandom()`.
- A normal `TPM2_GetRandom` is invoked to maintain TPM-internal state.
- The output buffer is immediately overwritten with quantum entropy (e.g., `Quantis.Read(size)`).
- The TPM state machine remains correct: sessions, PCR logic, and nonces all behave normally.

This design guarantees:

- Strong, quantum-origin randomness
- Full compatibility with TPM 2.0 specifications
- Predictable system behavior even if QRNG is unavailable

**Runtime Source Switching (QRNG ↔ PRNG)**

QTPM implements fallback logic:

- If the QRNG fails (driver error, PCIe malfunction, unreachable API),
- Automatically switch to software DRBG,
- Ensuring **availability**—TPM never stalls due to lack of entropy.

This supports:

- Industrial-grade reliability
- Benchmarking
- Backwards compatibility with devices still migrating to QRNG

---

### From Software to Hardware: FPGA-Based Quantum Cryptographic Modules

Porting QTPM from software to FPGA allows creation of a **Quantum-Enhanced Hardware TPM**.

A QCM (Quantum Cryptographic Module) on FPGA:

- integrates QRNG hardware
- embeds TPM logic
- supports secure boot & attestation
- accelerates crypto instructions

This represents the next generation of IoT security chips.

---

#### QEaaS: Quantum Entropy as a Service

Not all IoT devices can host a QRNG (cost, size, power).
**QEaaS (Quantum Entropy as a Service)** solves this by distributing quantum entropy over the network.

IoT Device → QEaaS Cloud → Quantum Randomness Stream (secure, authenticated).

Key challenges addressed:

- Encryption of entropy in transit
- Authentication to prevent tampering
- Rate limiting
- Freshness proofs
- Fault-tolerant buffering for intermittent networks

QEaaS empowers even legacy devices with quantum-grade randomness.

#### QEaaS System Architecture

QEaaS consists of four core modules:

**(1) Quantum Entropy Engine (Cloud QRNG Pool)**

Implements quantum randomness generation using:

- optical QRNG (beam splitters, interference, de-biasing)
- noise-based QRNG (vacuum fluctuations, shot noise)

With:

- entropy extractors
- entropy pool management
- SP800-90B-style health tests

**(2) Cloud Control Plane**

Handles:

- authentication of devices
- PQC signature verification
- rate limiting & quotas
- load balancing between QRNG pools
- audit logs
- tenant isolation

**(3) IoT Client Runtime**

Runs inside device firmware and provides:

- local entropy buffer
- PQC signature verification
- automatic fallback when offline
- XOR/hashing for composite entropy
- reconnect logic & ultra-low-power modes

**(4) Distributed Multi-Source Entropy**

IoT firmware may combine entropy from multiple sources:

- AWS Quantum RNG
- Azure Quantum RNG
- NIST entropy services
- corporate QRNGs
- university research QRNG APIs

Combination functions:

- XOR of all sources
- Hash concatenation: `H(R1 || R2 || …)`
- HKDF over combined entropy

**If even one source is honest and unpredictable → the result is secure.**

This is the classical robustness property of combined random sources.

---

### Core Cryptographic Foundations

**Post-Quantum Signatures (PQC)**

Used to authenticate IoT↔QEaaS communication:

- Dilithium signatures for device requests
- Falcon signatures for server responses

**End-to-End Encryption**

Typically:

- Kyber KEM → derive session key
- AES-GCM → protect the quantum entropy
- Optionally layered over TLS 1.3 for defense-in-depth

**Local Entropy Buffer**

To maintain availability:

- device stores 4 KB of pre-fetched entropy
- refills when <25% remains
- batch processing used to reduce power cost

<details><summary>Code</summary>

```c
Listing 1. Enhanced tpm2_getrandom Function for Quantum Entropy Integration.
static tool_rc get_random_custom(ESYS_CONTEXT *ectx, unit8_t *
   custom_bytes, UINT16 custom_size){
   tool_rc rc = tpm2_getrandom(ectx, custom_size, &ctx.random_bytes,
       &ctx.cp_hash,  &ctx.rp_hash, ctx.aux_session_handle[0], ctx.
      aux_session_handle[1], ctx.aux_session_handle[2], ctx.
      parameter_hash_algorithm);
   if (rc != tool_rc_success){
      LOG_ERR (“Failed getrandom”);
      return rc;
   }

   // Replace the generated random bytes with custom_bytes
   memcpy(ctx.random_bytes->buffer, Quantis.Read(size));
   ctx.random_bytes->size = custom_size;

   return rc;
}
```

</details>

---

[Swtpm](https://github.com/stefanberger/swtpm)
[Libtpms](https://github.com/stefanberger/libtpms)
[Tpm2-tools](https://github.com/tpm2-software/tpm2-tools)
[Tpm2-tss](https://github.com/tpm2-software/tpm2-tss)
[TPM_Sharing_Scheme](https://github.com/CYCU-AIoT-System-Lab/TPM_Sharing_Scheme)
