# ETAAcademy-Audit: 40. Trusted Execution Environment

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>40 TEE</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>TEE</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# Runtime Trust in Confidential Computing: TRAF, Intel SGX, AMD SEV-SNP, and Intel TDX

Confidential computing tries to make a difficult cloud promise credible: sensitive workloads should remain protected even when the operating system, hypervisor, firmware, or cloud operator infrastructure cannot be fully trusted. Trusted Execution Environments (TEEs) are the hardware-backed answer to that problem, but their designs differ sharply in what they protect, what they leave to the host, and where they place trusted enforcement.

TEE Runtime Architectural Framework (TRAF) explains runtime protection as a question of who controls security-sensitive resources. TEEs are best understood as runtime trust architectures, not just encryption features. TRAF makes this clear by asking which component controls each security-sensitive task: the untrusted host, a trusted runtime protection module, or the protected instance itself. That framing explains why similar marketing claims can hide very different security properties. It is applied to three major confidential-computing architectures: Intel SGX for application enclaves, AMD SEV-SNP for confidential virtual machines, and Intel TDX for Trust Domain based VM isolation.

Intel SGX shows how enclave-scale isolation depends on measurement, EPC page protection, key derivation, attestation, and launch policy. AMD SEV-SNP shows why confidential VMs need memory integrity and ownership validation, not only encrypted DRAM. Intel TDX shows a different VM-scale path, centralizing page ownership and CPU-state protection in the SEAM-hosted TDX module while accepting trade-offs around attestation traceability and physical replay scope.

Across all three systems, the recurring lesson is the same: the most important security questions are usually at the runtime boundary. A TEE review should examine not only what is encrypted, but also what is measured, what is integrity protected, who validates resource transitions, how attestation binds configuration to identity, and where the host still has influence over execution.

The main takeaway is that TEE security does not come from encryption alone. Strong designs must bind launch measurements, memory ownership, address translation, CPU state, attestation evidence, and runtime transitions into one coherent trust model. Where those bindings are incomplete, attacks usually appear at the edges: page faults, context switches, emulation paths, migration, shared memory, and I/O.

## 1. Understanding Server-Side TEE Design Through TRAF

Trusted Execution Environments (TEEs) are now central to confidential computing: they promise to run sensitive cloud workloads even when the cloud provider's privileged software is not trusted. But the TEE ecosystem is difficult to reason about because designs differ sharply across Intel SGX/TDX, AMD SEV, ARM CCA, IBM PEF, and RISC-V systems such as Keystone, Sanctum, CURE, and Penglai. Attacks also vary widely, from incomplete attestation measurements to runtime resource-management corner cases.

The key insight behind the TEE Runtime Architectural Framework (TRAF) is that many TEE failures are not best understood as isolated bugs. They emerge from architectural decisions about who controls runtime resources: the untrusted host OS, the TEE instance itself, or trusted manufacturer components. TRAF turns that question into a structured framework for comparing designs, explaining trade-offs, and identifying where attack surfaces appear.

At the highest level, the design problem is to maximize security while preserving efficient resource management under an untrusted host OS. This explains that objective from the ground up: what TEEs assume, how remote attestation establishes initial trust, how TRAF classifies runtime protection modes, and why AMD SEV's evolution shows the security consequences of shifting responsibility away from the host.

### Why Server-Side TEEs Need a Runtime Framework

Traditional cloud computing asks users to trust the cloud service provider (CSP), including the hypervisor, host OS, operators, and privileged management software. That assumption blocks adoption for workloads in finance, healthcare, government, and other domains where runtime data exposure is unacceptable. Encryption at rest and in transit do not solve the problem because data must still be processed while in use.

Server-side TEEs address this gap by creating hardware-backed isolated execution contexts. A TEE protects code and data confidentiality and integrity even when privileged software is malicious or compromised. In cloud settings, TEEs support secure remote execution (SRE): a user should be able to verify the remote platform, send secrets only to the expected workload, and rely on hardware-backed protection during execution.

That goal creates a systems problem. Cloud platforms still need efficient CPU scheduling, memory allocation, page-fault handling, and I/O management. Moving all of that logic into trusted hardware or firmware would bloat the Trusted Computing Base (TCB). Leaving all of it to the host would preserve performance and manageability but undermine the security model. TRAF studies the coordination boundary between those two extremes.

### Scope, Threat Model, and Lifecycle

This focuses on hardware-based, server-side TEEs on general-purpose processors that support SRE. It includes commercial and academic designs such as Intel SGX/TDX, AMD SEV, ARM CCA, IBM PEF, and RISC-V systems. It excludes non-cloud embedded TEEs, purely software enclaves, and software extensions built on top of existing TEEs.

The adversary is powerful: it controls privileged software, can execute arbitrary privileged instructions, can manage lifecycle events, and controls non-TEE resource management such as memory mapping, I/O orchestration, and CPU scheduling. Many designs also consider a physical adversary for off-chip memory and peripherals. However, commercial TEE threat models typically exclude denial of service, because a hostile platform manager can always terminate workloads. Many also exclude microarchitectural side channels from their core guarantees.

The TCB has two major parts:

- **Manufacturer TCB:** trusted hardware, firmware, secure monitors, coprocessors, and manufacturer-controlled enclaves that enforce TEE guarantees.
- **ISV TCB:** application code, guest OS code, libraries, and other software inside the protected environment.

Smaller TCBs usually imply lower attack surface, but comparisons are not simple. Process-based TEEs and VM-based TEEs put different software and hardware into the trusted boundary, and many commercial components are closed-source.

A generalized TEE lifecycle looks like this:

```text
TEE Executable → Verifiable Initial States → Runtime TEE Instance → Delete TEE Instance
                      ↑ Remote Attestation (§III)          ↑ Runtime Management (§IV, §V)
```

Remote attestation establishes trust in the initial state. Runtime management is the harder ongoing problem: the TEE must keep protecting confidentiality and integrity while the untrusted host continues to manage shared resources.

### Remote Attestation: Trust Before Runtime

Remote attestation is the bootstrap phase. A remote verifier receives cryptographic evidence that a TEE instance was launched on an authentic platform with the expected initial software and configuration. Only after this check should the verifier provision secrets.

Most TEEs follow a three-step pattern:

```text
Step 1: Request  ← TEE instance or loader → attestation supervisor (Manufacturer TCB)
Step 2: Sign     ← supervisor produces Report = Sign(M || platform || config || PK_instance)
Step 3: Verify   ← remote user checks Report; if accept, open secure channel with PK_instance
```

The attestation supervisor is a trusted manufacturer component. It may be implemented in hardware, firmware, a secure coprocessor, or a privileged enclave. It signs an attestation report using a platform attestation key, which is itself endorsed by a manufacturer root key. This establishes a chain of trust from the hardware vendor to the report.

The central measurement relation is:

$$
M = H(C_{\text{init}})
$$

Here, $M$ is the fixed-length secure measurement, $H(\cdot)$ is a collision-resistant hash function, and $C_{\text{init}}$ is the TEE instance's initial content. In a process-based TEE, this may be application code and data. In a VM-based TEE, it may begin with a secure BIOS and then extend through a longer boot chain.

Attestation relies on collision resistance, but collision resistance is not enough. The measurement must include all security-relevant metadata. If virtual addresses, access permissions, layout choices, or other relevant state are omitted, an attacker may change execution semantics without changing the measured hash. The AMD SEV-ES BIOS layout attack illustrates this class: the issue was not a broken hash function but an incomplete measurement scope.

The report-signing relationship can be represented as:

$$
\text{Report} = \mathrm{Sign}_{K_{\text{attest}}}\big(M \,\|\, \text{platform} \,\|\, \text{config} \,\|\, \text{PK}_{\text{instance}}\big)
$$

The verifier then checks the report before provisioning secrets over a secure channel. If the verification key is public, the user can validate the report directly. If the manufacturer keeps verification keys private, verification is delegated to an attestation service, which expands the practical trust boundary.

Remote attestation is therefore necessary but insufficient. It proves a launch-time claim. TRAF focuses on what happens next: runtime resource management under an adversarial host.

### TRAF: Decomposing Runtime Resource Management

TRAF analyzes TEEs by breaking runtime behavior into OS-like resource-management tasks and then asking how each task is protected. The core runtime categories are CPU management, memory management, and I/O management.

```
CPU Management:    Scheduling | Context Switch | IRQ/INS Emulation
Memory Management: Virtual Address | Allocation | Page Fault | (Physical Address backup)
I/O Management:    I/O Transmission | I/O Operations
```

The key trusted component in TRAF is the **TEE Runtime Protection Module (RTPM)**. An RTPM is a manufacturer-TCB mechanism that enforces secure runtime management. Depending on the design, it may be a secure coprocessor such as AMD PSP, CPU extensions such as Intel TDX mechanisms, firmware, or a secure monitor.

TRAF classifies every runtime task into one of four protection modes:

$$
\phi(t) \in \{\text{Unprotected},\ \text{RTPM-only},\ \text{RTPM-guarded},\ \text{Instance-assisted}\}
$$

In this relation, $t$ is a runtime task and $\phi(t)$ is the selected coordination mode. The modes are:

| Mode                  | Host role              | RTPM role                  | TEE instance role            |
| :-------------------- | :--------------------- | :------------------------- | :--------------------------- |
| (a) Unprotected       | Full resource control  | Absent / non-participating | Passive                      |
| (b) RTPM-only         | Non-TEE resources only | Full TEE resource control  | Passive                      |
| (c) RTPM-guarded      | Configure/manage       | Validate security          | Passive                      |
| (d) Instance-assisted | Limited                | Limited                    | Direct resource manipulation |

This vocabulary matters because it prevents vague claims such as "the TEE protects memory" from hiding crucial implementation choices. A resource may be encrypted but not integrity protected. A page-table path may be guarded in common cases but fail in corner cases. A TEE may rely on application-level checks even though the manufacturer TCB does not supervise the relevant host behavior.

### CPU Management: Scheduling, Switching, and Emulation

CPU scheduling is usually **unprotected**. CSPs need policy control over placement, multiplexing, and fairness, and moving the scheduler into the TCB would be complex. This preserves cloud efficiency but gives the host control over interruption patterns, which creates controlled-channel leakage opportunities.

Context switching is usually more sensitive. Most mature TEEs move context-switch protection toward **RTPM-only** handling because register state, address-space identifiers, encryption-key selection, and TLB behavior all affect isolation. Baseline AMD SEV left more context-switch state under host control, while SEV-ES and SEV-SNP progressively hardened this path.

Instruction and interrupt emulation are especially difficult for VM-based TEEs. Hypervisors often emulate instructions such as CPUID, RDMSR, and RDTSCP. If the path is unprotected, the host can feed incorrect values to the guest. TDX and IBM PEF move some security-relevant events toward RTPM-guarded handling, while SEV-family designs retain more host involvement.

TLB tagging captures the performance-security trade-off. Tagged TLBs avoid expensive flushes by associating cached translations with an address-space identifier. But the performance gain depends on correct domain labeling, correct ASID discipline, and secure enforcement during context switches. If stale translations can be reused across domains, TLB poisoning becomes possible.

### Memory Management: Mapping, Allocation, Faults, and Encryption

Memory management is where TRAF exposes some of the most important differences between TEE designs.

Virtual memory is commonly **instance-assisted**: the guest or protected process controls its own virtual address space. Physical allocation is usually **RTPM-guarded**: the host proposes physical pages, but trusted metadata or hardware checks enforce ownership and isolation.

That guarded allocation relationship can be written as:

$$
\text{Alloc}(v) \xrightarrow{\text{host}} p \;\xrightarrow{\text{RTPM check}} \text{Permit}(p, \text{owner}=\text{TEE})
$$

The host selects a system physical page $p$ for a virtual address $v$, but the RTPM must approve the mapping before it is used. Different systems implement this through structures such as SGX's EPCM, SEV-SNP's RMP, or other ownership metadata.

Page-fault handling varies more widely. SGX and SEV-SNP use RTPM-guarded approaches where the host maintains page tables but checks occur during translation. TDX and Penglai move more control into RTPM-only page-table handling. Keystone and CURE use instance-assisted approaches. Hybrid designs also exist: private memory may be RTPM-only while shared memory remains RTPM-guarded.

Memory encryption protects data leaving the SoC:

$$
\text{mem}_{\text{bus}} = E_{K_{\text{MEE}}}(\text{plaintext}),\quad K_{\text{MEE}} \in \text{RTPM only}
$$

This protects confidentiality against some physical threats, but encryption does not automatically imply integrity or freshness. At server memory scale, several designs omit full integrity/freshness protection for performance reasons. That leaves replay and remapping risks even when raw DRAM contents are encrypted.

### I/O Management: The Persistent Weak Point

I/O is usually **unprotected**. SGX relies on outside calls and application-level protocols. VM-based TEEs often use emulated virtio devices, shared memory, and host-mediated DMA paths. Confidentiality and integrity are frequently delegated to application-layer TLS, disk encryption, or protocol-specific defenses.

This design choice is practical because fully trusted I/O would require substantial hardware, driver, and device ecosystem support. But it also means that I/O remains a broad attack surface. Emerging trusted-I/O proposals, including TDX 2.0-style protected device channels, PCIe IDE, and SR-IOV-based designs, move toward stronger hardware-backed channels, but they are not the default pattern across deployed TEEs.

### What TRAF Reveals About Security Risk

TRAF is not a numerical scoring system. Its value is causal: it identifies where host influence remains and whether the trusted guard is strong enough to validate the relevant invariants.

Risk rises when the host can manipulate or observe a task and the trusted validation logic omits security-relevant state. This explains why many attacks do not break cryptographic primitives directly. Instead, they exploit host-controlled transition states, stale metadata, incomplete checks, or under-specified measurements.

Controlled channels are a recurring example. If the host can trigger interrupts or page faults and observe the resulting trace, the trace may leak information about secret-dependent execution. Even when confidentiality and integrity of memory contents hold, execution patterns can reveal sensitive behavior.

The four modes imply different trade-offs:

- **Unprotected mode** gives the best performance and operational flexibility, but it maximizes host control and observability.
- **RTPM-only mode** offers the strongest isolation for the task, but it can increase TCB complexity and may be impractical for OS-scale functions.
- **RTPM-guarded mode** balances host management with trusted checks, but it is only as strong as the completeness of those checks.
- **Instance-assisted mode** gives the TEE instance more autonomy, but it can reduce transparency and requires careful handling of malicious or non-compliant tenant behavior.

The crucial lesson is that "guarded" is not a binary property. A weak guard that validates encryption-key ownership but not mapping integrity may still allow serious attacks.

### Practical Lessons for TEE Design and Review

TRAF leads to several practical lessons for anyone studying or auditing TEE systems.

First, attestation must measure complete security-relevant state. A hash over incomplete launch metadata gives false confidence. Measurements must cover not only software bytes but also layout, permissions, configuration, and any metadata that changes execution meaning.

Second, memory encryption should not be treated as full memory security. Confidentiality, integrity, freshness, and mapping correctness are separate properties. A design can satisfy one while failing another.

Third, runtime transitions deserve special attention. Context switches, page faults, interrupt delivery, instruction emulation, and shared-memory transitions are where host control often persists. These boundaries are more likely to hide exploitable corner cases than steady-state execution.

Fourth, TCB minimization is not automatically superior if it pushes critical validation back to an untrusted host. The right question is task-specific: what must be trusted for this resource-management event, and what state does the trusted component actually validate?

Finally, cross-TEE comparisons should be made at the task level, not only at the product level. Saying that a platform is "VM-based" or "process-based" is less precise than asking how it handles scheduling, context switching, page faults, physical allocation, I/O, and attestation measurement.

---

## 2. SGX Architecture: From Enclave Creation to Attestation and Launch Control

Intel SGX is built around a compact but dense set of hardware-enforced mechanisms: enclave construction, measurement, thread entry and exit, EPC page eviction, key derivation, software attestation, and launch control. These mechanisms work together to let software run in an isolated address space while the operating system remains outside the trust boundary.

The central design pattern is consistent across SGX: system software may orchestrate enclave resources, but the processor enforces the security-critical invariants. A malicious kernel can schedule, interrupt, page, and attempt to manipulate enclave state, yet SGX tries to ensure that enclave identity, sealed keys, attestation reports, and EPC page integrity remain bound to hardware-verified state.

This turns the SGX architecture analysis into a standalone walkthrough of the lifecycle: how an enclave is built, how threads execute, how pages leave and re-enter the Enclave Page Cache (EPC), how `MRENCLAVE` and `MRSIGNER` define identity, how keys are derived, how attestation works, and why launch control was controversial.

### The SGX Lifecycle at a Glance

SGX enclaves move through two broad phases: build time and runtime. Build time establishes identity and freezes the enclave image. Runtime handles controlled entry, exit, interruption, paging, key use, and attestation.

The core lifecycle is compact: `ECREATE`, `EADD`, `EEXTEND`, and `EINIT` construct and finalize the enclave; `EENTER`, AEX, and `EEXIT` handle execution transitions; `EBLOCK`, `EWB`, `ELDU`, and `ELDB` protect EPC paging; `EREPORT` supports local attestation; and `EINIT` also enforces launch-token checks. The rest of the architecture can be understood as a set of mechanisms that preserve these invariants even when privileged software is hostile.

### Enclave Construction and Initialization

SGX enclaves are constructed by system software using four privileged instructions: `ECREATE`, `EADD`, `EEXTEND`, and `EINIT`. The system software creates the enclave's control structure, adds pages to the EPC, extends the cryptographic measurement, and finally asks the CPU to initialize the enclave.

This division is security-sensitive. Before `EINIT`, the enclave is mutable and the OS can load arbitrary content. After `EINIT`, the enclave is locked into its measured identity and can access SGX-protected resources. In practice, `EINIT` is the hard trust boundary.

SGX uses a `SIGSTRUCT` certificate to bind the enclave's build-time identity to the author's signing key. The certificate is verified using RSA-3072 and SHA-256, and the author identity, `MRSIGNER`, is derived from the signing modulus.

This means SGX tracks both what enclave code was built (`MRENCLAVE`) and who signed it (`MRSIGNER`). That split becomes important for key derivation, update policy, and launch authorization.

The fact that `ECREATE` and `EADD` run under system software is not a weakness by itself. SGX assumes the OS may be malicious. The defense is measurement: if the OS loads the wrong content, the final `MRENCLAVE` will not match the expected value in `SIGSTRUCT.ENCLAVEHASH`, and `EINIT` should fail.

### Thread Execution and Asynchronous Exit

Each SGX thread is anchored by a Thread Control Structure (TCS) inside the EPC. The TCS points to a Stack Save Area (SSA), which stores CPU state during enclave exits. The basic thread lifecycle is handled by `EENTER`, `EEXIT`, and Asynchronous Enclave Exit (AEX).

When a thread enters the enclave, hardware locks the TCS so the same TCS cannot be re-entered concurrently. When the enclave exits voluntarily, `EEXIT` returns control to the host. When an exception, interrupt, or fault occurs, AEX saves enclave register state into the SSA and exits safely.

AEX uses the current SSA frame selected by the TCS, saves general-purpose registers there, increments the CSSA nesting level, and exits the enclave. Because the SSA lives inside the EPC, saved register contents remain encrypted and integrity protected from the hypervisor's direct view. The host can force exits, but it should not be able to read the saved register state.

The main architectural limitation is stack depth. `TCS.NSSA` is fixed at build time. A malicious hypervisor can induce nested exceptions until the SSA stack overflows, causing enclave termination. SGX treats this as denial of service, which is generally outside the enclave confidentiality and integrity guarantees.

### EPC Page Eviction and Replay Protection

The Enclave Page Cache is limited, so SGX must allow EPC pages to be evicted to ordinary untrusted DRAM. Eviction is dangerous because the OS controls paging, memory pressure, and storage locations. SGX therefore uses a hardware protocol that encrypts, MACs, versions, and tracks pages as they leave and re-enter the EPC.

The eviction flow is:

```text
EBLOCK → ETRACK → shootdown → EWB
```

`EBLOCK` marks a page as blocked. `ETRACK` begins tracking stale translations. The system performs TLB shootdown so no CPU continues using the page. `EWB` then writes the page out of the EPC in encrypted and authenticated form.

The write-back protection is modeled as:

```math
(\text{Ciphertext}, \text{MAC}) = \text{AES-GCM}_{K_{\text{EPC}}}\big(\text{Plaintext}, \text{GPA} \| \text{Epoch} \| \text{VA\_slot}\big)
```

Replay protection uses Version Arrays. Each evicted page is associated with a version slot, and the slot is incremented on every write-back:

$$
\text{VA}[\text{slot}]_{\text{new}} = \text{VA}[\text{slot}]_{\text{old}} + 1 \quad \text{on each EWB}
$$

On reload, SGX detects stale data by comparing the stored version with the current Version Array entry:

```math
\text{Version}_{\text{stored}} \neq \text{Version}_{\text{VA slot}} \Rightarrow \text{ELDU: PF (replay detected)}
```

This design mitigates replayed EPC pages, corruption, and aliasing attacks. The EPC Map also enforces one-to-one relationships that prevent two guest pages from being aliased to the same EPC page.

The cost is complexity. Version Arrays themselves live in the EPC and can be evicted, creating nested eviction chains. If the OS cannot load the required Version Array before completing an eviction decision, the system can deadlock. The `ETRACK` and shootdown path also creates significant performance pressure, especially on large multi-socket systems where inter-processor interrupts are expensive.

### Enclave Measurement: What `MRENCLAVE` Really Captures

`MRENCLAVE` is SGX's deterministic fingerprint of the enclave image. It is a running SHA-256 hash over the ordered sequence of build operations and selected metadata. The measurement captures the enclave's construction process, not merely a flat hash of a binary.

`ECREATE` incorporates the SSA frame size and enclave size, `EADD` incorporates page layout metadata such as relative virtual address and permissions, and `EEXTEND` incorporates page content in 256-byte chunks. `EINIT` finalizes the running hash and checks that the finalized measurement matches the signed enclave hash:

$$
\text{EINIT checks:} \quad \text{MRENCLAVE}_{\text{final}} == \text{SIGSTRUCT.ENCLAVEHASH}
$$

The subtle point is that `EADD` measures page metadata but not page content. Page content enters the measurement only through `EEXTEND`. If security-sensitive pages are added but not extended, the final measurement may validate layout and permissions while omitting actual bytes. Enclave authors and auditors must verify that all security-relevant pages are extended.

SGX also measures relative guest virtual addresses rather than absolute addresses. This makes enclave measurements relocatable and supports address-space layout choices, but it also means changing the base address does not change `MRENCLAVE`.

### Key Derivation and Versioning

SGX derives enclave-specific keys from hardware secrets using AES-CMAC. The derivation binds keys to selected identity fields, version fields, CPU state, attributes, and owner epoch values. The result is a flexible hierarchy that supports sealing, rollback protection, attestation, and launch control.

The universal `EGETKEY` derivation can be summarized as:

$$
K = \text{AES-CMAC}_{K_{\text{master}}}\big(\text{KEYNAME} \| \text{MRENCLAVE/MRSIGNER} \| \text{ISVPRODID} \| \text{ISVSVN} \| \text{CPUSVN} \| \text{MASKEDATTRIBUTES} \| \text{KEYID} \| \text{OWNEREPOCH}\big)
$$

Version rollback protection is enforced through constraints on requested software and CPU security versions:

```math
\text{KEYREQUEST.ISVSVN} \leq \text{SECS.ISVSVN} \quad \text{and} \quad \text{KEYREQUEST.CPUSVN} \leq \text{CR\_CPUSVN}
```

During an enclave upgrade, version-bound keys change as the ISVSVN changes. The `KEYPOLICY` choice is one of the most important design decisions for sealed data. An `MRENCLAVE`-bound key is tied to one exact enclave build. An `MRSIGNER`-bound key can be shared across enclaves signed by the same author, which supports upgrades and migration but also allows future versions from that signer to access older sealed data. That is a feature when updates are trusted and a risk when future code is not yet reviewed.

The `KEYID` field allows multiple independent key families for the same enclave identity, which supports key rotation without changing the enclave binary. `OWNEREPOCH` lets platform ownership changes invalidate many derived keys, although provisioning-related keys deliberately treat this field differently.

### Software Attestation

SGX attestation has two layers. Local attestation uses `EREPORT` to produce a MACed report for another enclave on the same platform. Remote attestation relies on a Quoting Enclave that converts local evidence into a remotely verifiable Quote.

The Report Key is derived for the target enclave:

```math
K_{\text{Report}} = \text{AES-CMAC}_{K_{\text{master}}}\big(\text{REPORT\_KEY} \| \text{MRENCLAVE}_{\text{target}} \| \text{CPUSVN}_{\text{current}} \| \text{CR\_REPORT\_KEYID} \| \text{OWNEREPOCH}\big)
```

`EREPORT` then authenticates the reporting enclave's identity and caller-provided report data:

$$
\text{MAC}_{\text{REPORT}} = \text{AES-CMAC}_{K_{\text{Report}}}\big(\text{MRENCLAVE} \| \text{MRSIGNER} \| \text{ISVPRODID} \| \text{ISVSVN} \| \text{ATTRIBUTES} \| \text{CPUSVN} \| \text{KEYID} \| \text{REPORTDATA}\big)
$$

Only the target enclave can derive the matching Report Key, so it can verify the report while outsiders cannot forge it.

For remote attestation, Intel's Quoting Enclave signs evidence using a provisioned Attestation Key. SGX's original design used EPID group signatures, which let a verifier confirm that a quote came from a legitimate group member without learning the exact CPU identity.

The key hierarchy separates sealing, reporting, provisioning, launch, and attestation responsibilities. Seal, Report, Provisioning Seal, and Launch keys depend on platform secrets that Intel should not be able to recreate after provisioning. The main exception is the Provisioning Key, which is derivable by Intel and is used during provisioning to authenticate the Provisioning Enclave to Intel's servers. After provisioning, the Attestation Key is stored under a Provisioning Seal Key that Intel should not be able to derive. This two-stage handoff limits Intel's ability to recreate already-provisioned attestation keys.

### Launch Control

Launch control is one of SGX's most controversial mechanisms. In the original SGX model, a Launch Enclave acted as a mandatory gatekeeper for non-Intel enclaves. The Launch Enclave issued `EINITTOKEN`s, and `EINIT` would initialize an enclave only if the token authenticated the enclave identity and attributes.

The Launch Key derivation excludes the target enclave's `MRENCLAVE` and `MRSIGNER`:

```math
K_{\text{Launch}} = \text{AES-CMAC}_{K_{\text{master}}}\big(\text{LAUNCH\_KEY} \| \text{ISVPRODIDLE} \| \text{ISVSVNLE} \| \text{CPUSVN} \| \text{MASKEDATTRIBUTESLE} \| \text{KEYID} \| \text{OWNEREPOCH}\big)
```

The Launch Enclave uses that key to MAC an `EINITTOKEN` over the vetted target:

$$
\text{MAC}_{\text{TOKEN}} = \text{AES-CMAC}_{K_{\text{Launch}}}\big(\text{MRENCLAVE}_{\text{vetted}} \| \text{MRSIGNER}_{\text{vetted}} \| \text{ATTRIBUTES}_{\text{vetted}} \| \text{LE metadata}\big)
$$

SGX also includes a bootstrap exception for Intel-signed Launch Enclaves, and debug and production launch namespaces are separated so debug Launch Enclave tokens cannot initialize production enclaves.

The security critique is direct: the Launch Enclave adds little for the computer owner because system software already has broad visibility into launch parameters. Its practical role was closer to licensing and ecosystem control. Intel later introduced Flexible Launch Control on Ice Lake processors, allowing platform owners to configure a custom `MRSIGNER` for launch approval instead of relying exclusively on Intel's Launch Enclave authority.

### Trust Hierarchy

<details><summary>The SGX trust hierarchy flows from hardware fuses into derived keys, then into sealing and attestation:</summary>

```text
Hardware E-Fuses (Provisioning Secret + Seal Secret)
         │
         ▼
   SGX Master Derivation Key (K_master)
         │
         │
    ┌────┼────────────────────┐
    │    │                    │
    ▼    ▼                    ▼
 Seal  Report  Provisioning  Launch
 Key   Key     Key           Key
    │              │
    │    ┌─────────┘
    │    ▼
    │  Intel Provisioning Service
    │    (EPID Member Key)
    │         │
    │    Provisioning Seal Key
    │         │ (encrypted storage)
    │    Quoting Enclave
    │         │ EPID Signature (Quote)
    │         ▼
    │    Remote Verifier
    │
    └──> Enclave-specific sealed data
         (Seal Key family)
```

</details>

This hierarchy explains why SGX is not a single mechanism. It is a composition of measurement, key derivation, local attestation, remote attestation, launch authorization, and page-protection machinery.

### What SGX Defends Against

The SGX mechanisms map cleanly to common attack classes. `MRENCLAVE` measurement prevents the host from silently loading different code. Version Arrays, AES-GCM write-back metadata, and the EPC Map defend against replay and aliasing of evicted EPC pages. Version constraints in `KEYREQUEST` reduce sealed-data rollback risk. Report Keys prevent outsiders from forging local attestation reports, while EPID hides the exact CPU identity in the original remote-attestation design. Launch tokens and debug/production namespace separation enforce SGX's launch-control policy.

These defenses also show SGX's boundary. The architecture addresses identity, integrity, replay, sealing, and attestation within its model. It does not fully solve denial of service, controlled-channel leakage, or every microarchitectural side channel.

---

## 3. AMD SEV-SNP: Integrity Protection for Confidential Virtual Machines

AMD SEV-SNP is the third major step in AMD's Secure Encrypted Virtualization line. The original SEV design protected VM memory confidentiality with per-VM encryption keys. SEV-ES then protected guest CPU register state during VM exits. SEV-SNP adds the missing property that confidentiality alone cannot provide: memory integrity under a malicious hypervisor.

The core problem is simple. An attacker does not need to decrypt a VM's memory to damage the VM. A malicious hypervisor may replay old ciphertext, corrupt encrypted memory, alias pages, or remap guest physical addresses to unexpected system physical pages. These attacks can break workload assumptions even when AES encryption remains intact.

SEV-SNP responds by treating the hypervisor as fully malicious and moving key integrity decisions into hardware and AMD secure firmware. Its main mechanisms are the Reverse Map Table (RMP), guest-driven page validation through `PVALIDATE`, intra-VM isolation through Virtual Machine Privilege Levels (VMPLs), and VCEK-signed attestation tied to the platform TCB version.

### From Confidentiality to Integrity

SEV and SEV-ES reduced what the hypervisor could read. SEV encrypted guest memory with a key that software cannot directly access. SEV-ES encrypted CPU register state on hypervisor transitions. Together, these mechanisms protected important confidentiality surfaces.

But confidentiality does not imply integrity. If a VM writes value `A` to location `X`, encryption alone does not prove that a later read from `X` will return `A`. A malicious hypervisor may be unable to choose the decrypted plaintext, but it may still replay stale ciphertext, corrupt data, or change address translations in ways that cause the guest to consume unintended values.

SEV-SNP's defining invariant is a read-after-write consistency guarantee:

$$
\forall X, \forall A: \quad \text{Write}(X, A) \Rightarrow \text{Read}(X) \in \{A, \perp\}
$$

Here, `X` is a private guest memory location, `A` is the value last written by the VM, and $\perp$ means an exception. In plain terms, a guest read must either return the last value written by that guest or fail. It must not silently return stale, corrupted, or remapped data.

This guarantee has to survive normal VM operations such as page swapping and live migration. SEV-SNP therefore combines CPU hardware checks, AMD Secure Processor (AMD-SP) firmware services, page ownership metadata, and cryptographic authentication for swapped state.

### A Stronger Threat Model

SEV-SNP changes the trust model from "the hypervisor may be buggy" to "the hypervisor may be actively malicious." The trusted components are the AMD SoC hardware, AMD-SP firmware, and the guest VM components that the owner chooses to trust. The untrusted side includes the BIOS, hypervisor, host device drivers, PCI devices, other VMs, and cloud operator-controlled software.

This shift explains why SEV-SNP cannot rely on the hypervisor to honestly report memory ownership, CPU features, interrupt behavior, or migration policy. Instead, SEV-SNP gives the hypervisor a management role while requiring hardware or guest-owned mechanisms to validate security-critical transitions.

The most important design theme is separation of proposal and acceptance. The hypervisor can propose a page assignment, but the guest must validate it. The hypervisor can still control nested page tables, but hardware checks the result against RMP metadata. The platform can report a TCB version, but the attestation key is cryptographically tied to that version.

### The Reverse Map Table

The Reverse Map Table is the central hardware structure behind SEV-SNP memory integrity. It contains one entry per 4 KB system physical page and is indexed by System Physical Address (SPA). Each RMP entry records ownership, the assigned Guest Physical Address (GPA), validation state, and VMPL permissions.

The RMP is not directly writable by normal software. It is modified through controlled paths:

- `RMPUPDATE`: called by the hypervisor to assign a page to a guest, with `Validated=0`.
- `PVALIDATE`: called by the guest to accept a GPA-to-SPA mapping, setting `Validated=1`.
- `RMPADJUST`: used by higher VMPL code to adjust permissions for lower VMPLs.
- AMD-SP APIs: used for firmware-managed state transitions such as metadata and migration handling.

In native mode, the CPU checks the RMP after a standard page-table walk. In SEV-SNP VM mode, the CPU checks that the SPA found through nested paging matches the GPA and ownership stored in the RMP entry. Write accesses require RMP checks. Hypervisor reads are exempt because private guest data remains encrypted, but that exemption assumes the memory encryption mode does not leak useful plaintext information through ciphertext behavior.

### Page Validation and Re-Mapping Defense

Page validation is the mechanism that prevents a malicious hypervisor from silently changing a guest's GPA-to-SPA mapping. It uses a two-step protocol.

First, the hypervisor assigns a page using `RMPUPDATE`. This creates an RMP entry for the guest but leaves it unvalidated. Second, the guest accepts the mapping using `PVALIDATE`, which sets the RMP entry's `Validated` bit. If the hypervisor later points the nested page table at a different SPA for the same GPA, the CPU sees that the substituted page is not validated and raises a `#VC` exception.

The uniqueness rule can be expressed compactly:

$$
\nexists \text{SPA}_i \neq \text{SPA}_j : \text{RMP}[\text{SPA}_i].\text{GPA} == G \land \text{RMP}[\text{SPA}_i].\text{Validated} = 1 \land \text{RMP}[\text{SPA}_j].\text{GPA} == G \land \text{RMP}[\text{SPA}_j].\text{Validated} = 1
$$

No two validated SPAs may claim the same GPA. This prevents aliasing and re-mapping attacks when the guest follows the required validation discipline. If the nested page table points a GPA at an unvalidated SPA, the CPU raises `#VC`.

<details><summary>A typical re-mapping attack detection flow looks like this:</summary>

```text
Guest                   Hypervisor                AMD-SP Hardware
  |                          |                          |
  |  PVALIDATE(GPA=A -> SPA=X)                          |
  |-------------------------->|   RMP[X].Validated=1    |
  |                          |                          |
  |                    RMPUPDATE(SPA=Y, GPA=A)           |
  |                          |   RMP[Y].Validated=0     |
  |                    NPT[A] := Y (malicious remap)     |
  |                          |                          |
  |  Read(GPA=A)             |                          |
  |   -> NPT walk: A -> Y    |                          |
  |   -> RMP[Y].Validated=0  |                          |
  |   -> #VC exception       |                          |
  |  Guest detects attack    |                          |
```

</details>

This mechanism is powerful, but it has an important software requirement: the guest must not validate the same GPA twice in a way that accepts a malicious remap. The hardware provides the validation state and exception path, but guest software must maintain correct `PVALIDATE` discipline.

### Page States, Swapping, and Authenticated Metadata

The SEV-SNP white paper defines several RMP page states, including `HYPERVISOR`, `GUEST-INVALID`, `GUEST-VALID`, `PRE-GUEST`, `PRE-SWAP`, `FIRMWARE`, `METADATA`, and `CONTEXT`. These states control which entity owns a page and which transitions are legal.

Swapping creates a special challenge because a page may leave DRAM and later return. SEV-SNP handles this by storing swapped pages with AMD-SP-generated AES-GCM authentication tags and GPA metadata. On swap-in, the platform can verify that the restored page corresponds to the correct guest and address. This extends the integrity guarantee beyond always-resident DRAM.

Live migration has a similar requirement. The VM must not silently resume with stale, substituted, or unauthorized state. SEV-SNP addresses this with a migration architecture built around a trusted Migration Agent.

### VMPLs: Isolation Inside the VM

SEV-SNP introduces four Virtual Machine Privilege Levels, VMPL0 through VMPL3. VMPL0 is the most privileged level and can serve as an internal trusted manager. It can emulate sensitive services, handle `#VC` exceptions for unenlightened guests, enforce migration policy, or mediate interrupt behavior.

VMPL permissions are restrictive. A write succeeds only when all relevant permission planes allow it:

$$
\text{Writable}(P, L) \iff \text{GPT}[P].\text{Write} \land \text{NPT}[P].\text{Write} \land \text{RMP}[P].\text{VMPL}[L].\text{Write}
$$

Here, `GPT` is the guest page table, `NPT` is the nested page table controlled by the hypervisor, and the RMP VMPL permission is set by a higher VMPL through `RMPADJUST`. This means that even if the hypervisor grants write permission in the nested page table, VMPL0 can still deny write access to a lower VMPL by withholding the RMP permission.

The VMPL hierarchy is useful only if VMPL0 remains trustworthy. If VMPL0 is compromised, it can grant broad permissions to lower VMPLs and collapse the intended isolation structure.

SEV-SNP also includes the Virtual Top of Memory (vTOM) mechanism. vTOM gives VMPL0 a way to classify private and shared memory without requiring standard guest page tables to manipulate the C-bit directly. This helps support guest operating systems that were not originally designed around SEV-SNP's private/shared memory model.

### Interrupts, Exceptions, and CPU Feature Honesty

A malicious hypervisor can attack a VM by injecting interrupts or exceptions at unsafe times or by lying about CPU capabilities. SEV-SNP adds optional defenses for both areas.

Restricted Injection limits the hypervisor to injecting only the `#HV` doorbell exception. Other events are delivered through a shared memory event queue, giving trusted guest-side code more control over event interpretation.

Alternate Injection moves interrupt queuing and injection control into the encrypted VM Save Area (VMSA), where only VMPL0 can manipulate it. This prevents the hypervisor from injecting interrupts at prohibited privilege levels or violating guest OS assumptions about exception delivery.

For CPU feature reporting, SEV-SNP supports AMD-SP-vetted CPUID values. A malicious hypervisor could otherwise lie about CPU features, such as XSAVE buffer sizes, and induce guest memory corruption. SEV-SNP can validate CPUID values either on demand or through a pre-vetted boot-time CPUID page.

### TCB Versioning and VCEK-Based Attestation

Remote attestation lets a relying party verify that a VM is running on authentic AMD hardware with the expected initial state and trusted firmware version. SEV-SNP ties this report to the Versioned Chip Endorsement Key (VCEK).

The VCEK is derived per chip and per TCB version:

$$
\text{VCEK} = \text{ECDSA-KeyDerive}\big(\text{H}(\text{ChipEndorsementKey} \ \| \ \text{TCBVersion})\big)
$$

Different TCB versions produce different VCEKs. This gives attestation anti-rollback force. A remote verifier can reject reports signed under a VCEK corresponding to a vulnerable or outdated TCB version.

The attestation report binds launch measurement, platform version, identity information, and guest-supplied data:

$$
\text{Quote} = \text{ECDSA-Sign}_{\text{VCEK}}\big(\text{LaunchDigest} \ \| \ \text{TCBVersion} \ \| \ \text{IDB} \ \| \ \text{GuestData}\big)
$$

Verification gives the relying party a platform and launch-state claim. The launch digest covers initial guest pages and GPA layout. The Identity Block can carry the guest owner's expected launch digest. `GuestData` can include a hash of a public key, allowing a remote party to verify the report and then provision secrets over a secure channel bound to that key.

<details><summary>A typical attestation flow is:</summary>

```text
Remote Party         Guest VM              AMD-SP
     |                  |                     |
     |  Challenge       |                     |
     |----------------->|                     |
     |                  |  Request Report     |
     |                  |  GuestData=HASH(PubKey)
     |                  |-------------------->|
     |                  |                     | Measure and sign
     |                  |  Attestation Report |
     |                  |<--------------------|
     |  Report + PubKey |                     |
     |<-----------------|                     |
     | Verify VCEK sig  |                     |
     | Check LaunchDigest                    |
     | Provide Secrets  |                     |
     |----------------->|                     |
```

</details>

This attestation model is stronger than a simple launch hash because it also binds the platform's TCB version. It still does not solve every physical threat. The white paper explicitly leaves some offline DRAM replay and active bus manipulation scenarios out of scope.

### Migration Agent as a TCB Extension

SEV-SNP replaces older static migration-policy assumptions with a Migration Agent (MA). The MA is itself an SEV-SNP VM. It is bound to the primary VM at launch, appears in the primary VM's attestation report, authenticates the destination platform's MA, and transfers guest state over a protected channel.

This design makes migration policy flexible. It can support online and offline migration, including cases where a VM is paused until a suitable destination is available. But it also expands the effective TCB. A compromised or incorrectly chosen MA could enable unauthorized migrations. Guest owners therefore need to pin and audit the MA measurement they accept, just as they would for other trusted launch components.

### Side-Channel Scope and Optional Mitigations

SEV-SNP improves isolation against several hypervisor-controlled attacks, but it does not claim to eliminate all side channels. Optional BTB flush protection can reduce branch target injection risk. The `SPEC_CTRL` MSR is virtualized so a guest can choose IBRS and IBPB behavior. SMT-sensitive VMs can request execution policies that require SMT to be disabled.

These controls are policy-dependent. A guest that does not opt in may remain exposed to hypervisor-influenced branch predictor behavior. Other attacks, including cache PRIME+PROBE, page-fault-based fingerprinting, and performance counter tracking, remain outside the core protection scope.

### Practical Lessons

SEV-SNP shows that confidential computing must distinguish confidentiality from integrity. Memory encryption is valuable, but it does not prove that a guest is reading the memory it last wrote.

The RMP and `PVALIDATE` split the memory-management contract into host proposal and guest acceptance. That is the right direction under a malicious hypervisor model, but the model still depends on correct guest validation discipline.

VMPLs provide useful in-VM isolation, especially for a trusted VMPL0 service layer, but they concentrate trust in VMPL0. If VMPL0 fails, lower-level protections can be weakened from inside the VM.

VCEK-based attestation gives remote verifiers a way to bind VM identity to chip identity and TCB version. This is essential for anti-rollback decisions, but migration agents and optional side-channel mitigations remain part of the relying party's policy surface.

---

## 4. Intel TDX: Trust Domains, Memory Integrity, and VM-Scale Confidential Computing

Intel Trust Domain Extensions (Intel TDX) extend confidential computing from application enclaves to full virtual machines. Where Intel SGX isolates ring-3 enclave code, TDX isolates an entire VM, called a Trust Domain (TD), from the hypervisor, Virtual Machine Manager (VMM), system firmware, and other non-TD software on the host platform. The result is a VM-scale security architecture designed for cloud tenants that need to process sensitive workloads without trusting the cloud operator's privileged software stack.

TDX combines Intel VMX extensions, Total Memory Encryption Multi-Key (TME-MK), and an Intel-signed TDX module running in Secure-Arbitration Mode (SEAM). The TDX module becomes the trusted intermediary between the untrusted VMM and each protected TD. It controls private page ownership, programs per-TD memory-encryption keys, protects CPU state during exits, and produces attestation evidence that remote verifiers can check before sending secrets.

This explains Intel TDX as a complete architecture: its trust model, memory encryption and integrity mechanisms, address-translation protections, CPU-state handling, interrupt model, remote attestation flow, update and migration features, and the main security trade-offs that follow from the design.

### From Enclaves to Trust Domains

Intel TDX is best understood as the VM-scale evolution of SGX. SGX protects individual application enclaves, while TDX protects a complete VM boundary. This matters because many cloud workloads are not easily refactored into enclave-style applications. TDX lets tenants run a conventional VM while reducing the amount of cloud-provider software that must be trusted.

The core protected object is the **Trust Domain (TD)**. A TD contains private memory, CPU register state, VM control state, measurements, and attestation identity. The VMM still creates and manages TDs, but it cannot directly read or modify TD-private memory or TD-private control structures. The VMM remains responsible for many operational tasks, but those tasks are mediated by the TDX module.

The TDX module runs in **Secure-Arbitration Mode (SEAM)**. SEAM memory is protected by the SEAM Range Register (SEAMRR), and the module is digitally signed by Intel. The design explicitly excludes the VMM, hypervisor, SMM, SGX, and other host software from reading or writing SEAM memory.

TDX introduces several instruction interfaces around this boundary:

- **SEAMCALL:** VMM-to-SEAM entry for TDX module services.
- **TDCALL:** TD-to-SEAM entry for TD requests.
- **SEAMRET:** SEAM return to the VMM or TD.
- **SEAMREPORT:** SEAM-mode generation of a platform-bound attestation report.
- **EVERIFYREPORT2:** SGX-enclave verification of a SEAMREPORT-generated report.

This separation lets the VMM retain cloud management responsibilities while moving security-critical decisions into an Intel-controlled trusted module.

### Threat Model and Security Goal

TDX addresses two major adversary classes.

The first is a **system software adversary**: VMM, hypervisor, BIOS, SMM, and other privileged software with broad access to system memory and hardware programming interfaces. TDX aims to prevent this adversary from reading or tampering with TD-private memory, CPU state, page ownership, and attestation-relevant state.

The second is a **hardware adversary** with physical access to the DDR memory bus, including attacks such as cold boot, injection, replay, aliasing, and rowhammer-style corruption. TDX provides stronger hardware attack coverage than plain TME or TME-MK, especially when cryptographic-integrity mode is enabled.

The architecture does not claim to solve every physical attack. In particular, replay of physical memory through direct physical access is explicitly out of scope. This is an important distinction from SGX's EPC eviction model, which uses versioning and AES-GCM metadata for stronger replay protection in that narrower memory domain.

### Per-TD Memory Encryption

TDX uses Intel TME-MK to assign each TD a unique private KeyID. The TDX module programs the associated encryption key into the memory-encryption engine using the SEAM-mode-only PCONFIG instruction. Keys are CPU-generated, ephemeral, and inaccessible to software.

Each TD's private memory is encrypted using AES in XTS mode with a unique ephemeral key bound to the TD's private KeyID:

```math
\text{Ciphertext}[PA] = \text{AES-XTS-128}_{K_{\text{TD\_private}}}(\text{Plaintext}[PA], \text{Tweak}(PA))
```

The private key is generated by the CPU for the TD's KeyID and is never accessible to software. The physical address acts as part of the AES-XTS tweak, so identical plaintext blocks at different physical addresses encrypt differently. This protects against simple ciphertext comparison and prevents the hypervisor from learning TD-private plaintext by reading DRAM.

Memory encryption alone, however, is not memory integrity. AES-XTS is malleable, and repeated writes of identical plaintext to the same address can still produce observable patterns for a physical bus observer. TDX therefore defines a stronger cryptographic-integrity mode.

### Cryptographic and Logical Memory Integrity

TDX supports two memory-integrity modes.

In **cryptographic-integrity mode**, each cache line receives a truncated MAC computed over the ciphertext, physical address, TD-ownership bit, and TD-private key material. In **logical-integrity mode**, no MAC is stored; protection relies on a one-bit ownership tag that prevents cross-domain access but does not provide the same corruption-detection coverage.

In cryptographic-integrity mode, a 28-bit MAC is stored alongside each cache line:

```math
\text{MAC}_{28b}[\text{CL}] = \text{Truncate}_{28}\Big(\text{SHA-3-256}\big(\text{Ciphertext}[\text{CL}] \| K_{\text{TD\_private}} \| \text{TD\_Ownership\_bit} \| \text{PA}\big)\Big)
```

where SHA-3-256 uses the KECCAK[512] permutation. On read, hardware recomputes the MAC, compares it with the stored value, and terminates only the affected TD if verification fails.

The TD-ownership bit is included in the MAC. This prevents a cross-KeyID disclosure pattern in which private memory is accessed through the shared memory path. If a private-KeyID cache line is read with the shared KeyID, hardware returns a fixed pattern rather than exposing ciphertext.

The 28-bit MAC is a practical engineering trade-off. It gives TDX cache-line-level corruption detection while limiting metadata overhead. But it is probabilistic rather than absolute: a 28-bit MAC has only $2^{28}$ possible values. Under sustained physical attack with $N$ writes, the rough chance of an undetected corruption attempt is proportional to $N / 2^{28}$. For many cloud threat models, the attack cost and failure behavior make this acceptable, but high-assurance deployments should treat it as bounded integrity rather than perfect integrity.

### Address-Translation Integrity

TDX protects address translation with a dual-EPT model. A TD has both a **Secure EPT** and a **Shared EPT**. The Secure EPT is managed by the TDX module and maps private guest physical addresses to physical pages encrypted under the TD's private key. The Shared EPT is managed by the VMM and maps shared guest physical addresses through the shared key.

Memory access routing is determined by the most-significant bit of the GPA:

```math
\text{EPT}(GPA) = \begin{cases} \text{Secure EPT} \rightarrow K_{\text{TD\_private}} & \text{if } GPA[\text{MSB}] = 0 \text{ (private)} \\ \text{Shared EPT} \rightarrow K_{\text{shared}} & \text{if } GPA[\text{MSB}] = 1 \text{ (shared)} \end{cases}
```

This shared-bit routing is security-critical. It makes private and shared memory a hardware-enforced distinction rather than a convention maintained only by software. The CPU also blocks page-table structures and executable code from residing in shared memory, preventing the VMM from redirecting TD execution through attacker-controlled shared pages.

### PAMT and Page Ownership

The **Physical Address Metadata Table (PAMT)** is the authoritative TDX ownership table. Maintained by the TDX module in SEAM memory, PAMT records which TD owns each physical page, the page size, page type, and initialization state. It prevents aliasing across TDs and double-mapping within a TD.

**No aliasing:** A physical page may appear in at most one TD's Secure EPT:

$$
\nexists T_1 \neq T_2: \quad PA \in \text{SecureEPT}(T_1) \land PA \in \text{SecureEPT}(T_2)
$$

**No double-mapping:** Each physical page maps to at most one GPA in a TD's Secure EPT:

$$
\nexists GPA_1 \neq GPA_2: \quad \text{SecureEPT}(T)[GPA_1] = PA \land \text{SecureEPT}(T)[GPA_2] = PA
$$

PAMT is analogous to AMD SEV-SNP's Reverse Map Table (RMP), but the trust split differs. SEV-SNP uses guest-side validation through mechanisms such as PVALIDATE. TDX centralizes page ownership management inside the SEAM-hosted TDX module, which simplifies the guest software model but increases reliance on TDX module correctness.

### CPU-State Confidentiality and Integrity

TDX protects CPU state when a TD exits. The TDX module initializes each TD's VMCS, state-save area, and virtual APIC page under the TD's private key. On TD exit, the module saves registers and related CPU state into protected memory, then scrubs registers before returning control to the VMM.

The protected state includes general-purpose registers, control registers, model-specific registers, debug registers, performance monitoring counters, and XSAVE state. This prevents an untrusted hypervisor from learning TD secrets through register residue after a VM exit.

This mechanism is one of the main differences between confidential VMs and ordinary encrypted memory systems. Encrypting DRAM is not enough if CPU state leaks at the hypervisor boundary. TDX treats the VM exit path as a first-class confidentiality and integrity boundary.

### Interrupt and Exception Handling

TDX builds on VMX virtual APIC and virtual interrupt architecture. The VMCS and virtual APIC page are encrypted with the TD's private key, preventing the VMM from directly manipulating virtual interrupt state.

Posted interrupts can be delivered through hardware-processed posted-interrupt descriptors without forcing a VM exit. Exception injection by the VMM into a TD is architecturally disallowed. Non-maskable interrupts are handled only through a TDX-module API that preserves x86 NMI semantics.

The security goal is to preserve TD control-flow integrity against malicious hypervisor event injection while still letting the platform deliver required architectural events.

### Remote Attestation

Remote attestation lets a challenger verify that a TD is running on genuine Intel TDX hardware with the expected measurements and TCB version before provisioning secrets. TDX reuses SGX DCAP-style infrastructure: a TD-quoting enclave, a Provisioning Certification Enclave (PCE), a Provisioning Certification Key (PCK), and an Intel CA-backed certificate chain.

The SEAMREPORT instruction produces a Report structure binding all TD identity, measurements, and challenger data:

```math
\text{Report} = \big\{ \underbrace{\text{TDMR} \| \text{RTMR}[0..3]}_{\text{static+runtime measurements}} \| \underbrace{\text{TD\_Attributes} \| \text{MROWNER}}_{\text{identity metadata}} \| \underbrace{\text{TCB\_SVNs}}_{\text{CPUSVN+SEAMLDR\_SVN+Module\_SVN}} \| \underbrace{\text{GuestData}_{64B}}_{\text{challenger nonce/pubkey}} \big\}
```

The Report is MAC-protected, and the Quote is produced by the TD-quoting enclave via ECDSA signing:

```math
\text{Quote} = \text{ECDSA-Sign}_{K_{\text{attestation\_private}}}\big(\text{Report}\big)
```

The static TD Measurement Register (TDMR) reflects initial pages added during TD creation. Runtime Measurement Registers (RTMRs) let TD software extend measurements after boot, similar to TPM PCRs. This is powerful because a TD can attest dynamically loaded code, configuration, or policy data. It also means the quality of RTMR evidence depends on the integrity of the TD software performing the extensions.

<details><summary>The full 12-step attestation sequence:</summary>

```text
Challenger        TD Software       TDX Module (SEAM)      TD-Quoting Enclave   Intel Attest. Svc.
    |                  |                   |                       |                    |
(1) |---Request------->|                   |                       |                    |
    |                  |                   |                       |                    |
(2) |          TD calls TDCALL             |                       |                    |
    |                  |--TDCALL---------->|                       |                    |
    |                  |                   |                       |                    |
(3) |                  | Module calls      |                       |                    |
    |                  | SEAMREPORT:       |                       |                    |
    |                  |  TDMR+RTMR       |                       |                    |
    |                  |  GuestData       |                       |                    |
    |                  |  TCB SVNs         |                       |                    |
(4) |                  |<-- Report (MAC)---|                       |                    |
    |                  |                   |                       |                    |
(5) |                  |                   |                       |                    |
    |<-- Report --------|                   |                       |                    |
    |                  |                   |                       |                    |
(6) | TD asks VMM to convert to Quote       |                       |                    |
    |                  |-->VMM------------>|                       |                    |
    |                  |                   |                       |                    |
(7) |                  |   VMM sends Report to TD-Quoting Enclave  |                    |
    |                  |                   |-------Report--------->|                    |
    |                  |                   |                       |                    |
(8) |                  |                   |      EVERIFYREPORT2   |                    |
    |                  |                   |      (verify MAC)     |                    |
    |                  |                   |                       |                    |
(9) |                  |                   |  ECDSA-Sign → Quote   |                    |
    |                  |                   |<------- Quote --------|                    |
    |                  |                   |                       |                    |
(10)|<-- Quote ---------|                   |                       |                    |
    |                  |                   |                       |                    |
(11)|------------ Quote Verification -------------------------------->|                    |
    |                  |                   |                       |    PCK cert chain  |
(12)|<--- Verify OK ---------------------------------------------------|                    |
```

</details>

The local report MAC proves the report was generated on the same physical platform. The ECDSA Quote makes the report remotely verifiable through Intel's PKI.

### PCK Certificate Chain

The Quote's trust anchor is the PCK certificate chain: the attestation key is certified through the PCE and PCK certificate, and the PCK chain terminates at Intel's CA. A verifier checks the ECDSA quote signature, the PKI chain, and PCK revocation collateral. The PCK certificate is device- and TCB-specific. This gives TDX a simpler offline-verifiable certificate-chain model, but it also means TDX attestation is traceable to a physical platform. That differs from SGX EPID, which used anonymous group signatures.

<details><summary>PCK Certificate Provisioning:</summary>

```text
TD-Quoting Enclave          PCE                    Intel CA
       |                     |                         |
  (1) Generate ECDSA         |                         |
      attestation keypair    |                         |
       |                     |                         |
  (2) Send AttestPublicKey-->|                         |
       |                     |                         |
       |             (3) PCE signs using PCK           |
       |             PCK Certificate ←————————————————— |
       |<-- Cert struct ------|                         |
       |                     |                         |
  (4) Quote = ECDSA-Sign(Report) [with cert chain]     |
       |                                               |
  Anyone can verify: Quote → AttestKey → PCK → Intel CA
```

</details>

Because PCK certificates and CRLs can be distributed, quote verification can be performed without an online round trip to Intel for every attestation event, assuming the verifier has the relevant collateral.

### Migration, Partitioning, and Updates

TDX includes several operational features needed for cloud deployment.

**Live migration** is intended to move an active TD between TDX-compatible physical hosts while preserving confidentiality and integrity of TD memory and CPU state during transfer. The analyzed white paper defers details to a future specification, so migration should be treated as an architectural commitment rather than a fully specified mechanism in this document.

**TD Partitioning** allows one TD to host nested VMs: one L1 VM acting as a VMM and up to three L2 VMs. This supports lift-and-shift scenarios for legacy guest operating systems, but it changes the trust boundary. L2 VMs must trust not only the TDX module but also the L1 VMM. That expansion should be visible in attestation policy and verifier expectations.

**VM-preserving updates** allow the TDX module to be updated at runtime while preserving existing TD state. Existing TDs are paused and saved to ACM-owned encrypted memory during the update, then resumed under the updated module. This avoids long downtime, but it temporarily extends the effective TCB to include the Authenticated Code Module used during the update.

### Key Security Trade-Offs

TDX makes several deliberate trade-offs.

First, it favors ECDSA/PCK attestation over anonymous EPID-style attestation. This simplifies data-center verification and aligns with SGX DCAP infrastructure, but it sacrifices CPU anonymity because the PCK is device-specific.

Second, the 28-bit memory-integrity MAC is space-efficient but probabilistic. It improves protection against corruption, rowhammer, and injection, but high-assurance deployments must account for the finite MAC space.

Third, physical memory replay is outside the current protection scope. TDX can detect many forms of modification in cryptographic-integrity mode, but replaying previously captured memory to the same physical addresses remains a stated limitation.

Fourth, SEAM centralization simplifies the TD software model. Unlike designs that require guest-side ownership validation, TDX keeps PAMT and Secure EPT ownership enforcement inside the TDX module. This reduces guest complexity but makes TDX module firmware a highly critical TCB component.

Fifth, AES-XTS encryption protects confidentiality but does not hide all access patterns or repeated writes to the same physical address. Per-TD ephemeral keys reduce cross-domain key-wearout risks, but a hardware observer may still see repeated ciphertext behavior within one TD's memory domain.

### TDX, SGX, and SEV-SNP

TDX sits between SGX's enclave-scale protection and AMD SEV-SNP's confidential-VM model. The major comparison points are:

| Property                        | Intel SGX                    | AMD SEV-SNP                           | Intel TDX                                     |
| ------------------------------- | ---------------------------- | ------------------------------------- | --------------------------------------------- |
| Isolation granularity           | Application enclave (ring 3) | Full VM                               | Full VM (TD)                                  |
| Memory encryption               | AES-XTS (EPC only)           | AES-XTS (per-VM key)                  | AES-XTS-128 (per-TD key, TME-MK)              |
| Memory integrity                | AES-GCM (eviction only)      | AES-GCM (swap) + RMP                  | SHA-3-256 28-bit MAC (every cache line)       |
| Page ownership tracking         | EPC Map + Version Array      | Reverse Map Table (RMP)               | PAMT                                          |
| Hypervisor exclusion from TCB   | ✓ (enclave level)            | ✓                                     | ✓                                             |
| Replay protection               | ✓ (Version Array + epoch)    | ✓ (AES-GCM tag + RMP Validated bit)   | ✗ (explicit out of scope)                     |
| Attestation scheme              | EPID (anonymous group sig)   | ECDSA (VCEK, TCB-versioned)           | ECDSA (PCK certificate chain)                 |
| CPU anonymity in attestation    | ✓ (EPID)                     | ✗ (VCEK is device-specific)           | ✗ (PCK is device-specific)                    |
| Guest-side ownership validation | Not required                 | PVALIDATE                             | Not required (PAMT in SEAM)                   |
| Interrupt injection by VMM      | Blocked via EINITTOKEN       | Blocked via VMSA/Restricted Injection | Blocked architecturally (exception injection) |
| CPUID integrity                 | CPUID page (optional)        | CPUID filtering (optional)            | VMX enumeration enforcement                   |
| Live migration                  | Via SIGSTRUCT + MRSIGNER     | Migration Agent (MA)                  | Live Migration (future module)                |
| Nested VM support               | No                           | Via VMPLs (4 levels)                  | TD Partitioning (1 L1 + 3 L2 VMs)             |

The table highlights the architecture's distinctive position: TDX offers full-VM isolation, SEAM-managed page ownership, cache-line-level MAC support, and DCAP-style certificate-chain attestation, while explicitly accepting traceable attestation and limited physical replay coverage.
