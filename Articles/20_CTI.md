# ETAAcademy-Audit: 20. Cyber Threat Intelligence (CTI)

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>20 CTI</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>CTI</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# Cyber Threat Intelligence in the Era of AI, Blockchain, and Web3: Toward Proactive and Adaptive Defense

Cyber Threat Intelligence (CTI) is a systematic discipline that enables organizations to move from reactive defense to **proactive, risk-informed security management**. By collecting, analyzing, and distributing information on cyberattacks, CTI provides actionable insights that strengthen both operational defenses and strategic decision-making.

Modern CTI integrates data from both open and closed sources, covering indicators, tactics, and strategic intelligence. It leverages **standardized protocols, machine learning, behavioral analysis, and big data technologies** to deliver multi-layered, multidimensional protection. The global CTI market is expected to expand from **\$5.8 billion in 2024 to \$24.85 billion by 2032**, reflecting a compound annual growth rate of nearly **20%**.

In contemporary cybersecurity and Web3 environments, the **convergence of AI, blockchain, and smart contracts** is reshaping CTI into an auditable and adaptive defense framework.

- **AI** enables real-time analysis of CTI data and dynamic compliance rule-matching.
- **Smart contracts** transform abstract security standards into executable policies, automatically triggering countermeasures.
- **Blockchain** ensures tamper-proof auditability, forming a **self-healing security management loop**.

Consortium blockchain-based CTI platforms further enhance service quality and market growth by fostering **three-party collaboration** among requesters, platforms, and service providers. Incentive mechanisms encourage data sharing while maintaining trust.

To address data sensitivity, **federated learning** enables collaborative, cross-organizational, and even cross-border threat detection without exposing raw data. Instead, only encrypted parameters are exchanged, secured through **aggregation protocols, anti-poisoning defenses, and differential privacy**. This allows joint progress in intrusion detection, malware classification, and zero-day attack identification—while safeguarding data sovereignty.

For **Web3 ecosystems**, where threats often emerge from malicious smart contracts, CTI must evolve toward **contract-centric detection**. Through techniques such as **bytecode semantic analysis, Pruned Semantic-Control Flow Tokenization (PSCFT), and Transformer-based ensemble classifiers**, it is possible to predict attack probabilities even **after a contract is deployed but before an attack is executed**. Crucially, this ensures that **proactive defenses can be triggered in advance**, even when private transaction pools bypass traditional monitoring mechanisms.

---

## 1. AI + Blockchain + Smart Contracts: Toward Automated Compliance and Adaptive Threat Response

Modern organizations face significant challenges in **compliance** and **threat response**. Security policies are complex and numerous, leading to compliance fatigue, misconfigurations, and human error. Traditional approaches require large teams to manually manage compliance, costing enterprises an average of **\$5M annually** (IBM & Ponemon Institute). This problem is most acute in heavily regulated industries such as finance, where regulations change frequently, creating administrative overhead and inefficiency.

At the same time, **Cyber Threat Intelligence (CTI)** evolves rapidly. CTI involves the collection and analysis of data related to potential and active cyberattacks, helping organizations anticipate and mitigate threats. CTI has shifted cybersecurity strategies from **reactive defense** to **proactive defense**. While advances such as deep learning applied to vulnerability analysis or machine learning for hacker forum classification exist, the dynamic nature of threats remains poorly addressed. Current frameworks fail to translate CTI insights into **actionable, automated security controls**, leaving a gap between detection and execution.

### The AI + Blockchain + Smart Contract Integration Framework

An integrated framework combining **Artificial Intelligence (AI)**, **Blockchain**, and **Smart Contracts** addresses these shortcomings by enabling automated compliance and adaptive threat response:

- **AI** continuously analyzes CTI data to predict attack vectors and validate compliance status.
- **Smart Contracts** encode standards (ISO 27001, GDPR, NIST CSF, MITRE ATT\&CK) into executable rules that can be enforced automatically and immutably.
- **Blockchain** provides a transparent, tamper-proof audit trail of compliance checks, threat responses, and policy updates.

When AI detects a threat or noncompliance, a smart contract automatically enforces controls—such as firewall updates, endpoint isolation, or credential revocation—creating a **self-healing security environment** that minimizes human intervention, reduces decision delays, and ensures continuous compliance.

### Smart Contract Development Layer

Using **Hyperledger Fabric** and **Node.js chaincode**, compliance policies are transformed into executable rules within blockchain-based smart contracts. This layer operates through seven stages:

- **Policy Encoding** – Compliance requirements (ISO 27001, GDPR, MITRE ATT\&CK) are translated into `ComplianceRule` objects.
- **Event-Driven Triggers** – Chaincode listens for events (e.g., new application deployment) and automatically launches compliance checks.
- **CTI Integration** – External CTI feeds (e.g., IBM X-Force Exchange) update policies dynamically against evolving threats.
- **Automated Compliance Verification** – The system verifies system, application, and network configurations in real time.
- **Enforcement & Remediation** – The `EnforcementEngine` executes countermeasures (e.g., blocking malicious IPs, isolating infected devices) with blockchain consensus ensuring atomic, consistent actions.
- **Immutable Audit Trails** – Every compliance check and remediation action is recorded on-chain for regulatory transparency.
- **Dynamic Policy Updates** – Fabric’s Chaincode Lifecycle manages secure updates to compliance logic with governance safeguards.

### AI-Driven Compliance and Threat Response

The **AI engine** acts as the system’s brain, bridging CTI insights with blockchain enforcement. It follows a structured pipeline:

- **CTI Ingestion** – CTI data is collected via APIs (e.g., IBM X-Force), normalized with `pandas`, and prepared for analysis.
- **Threat Analysis & Classification** – SecureBERT generates semantic embeddings from threat reports; a Random Forest classifier evaluates severity levels.
- **Compliance Mapping** – Classified threats are mapped to organizational policies in a PostgreSQL compliance database, cross-referenced with MITRE ATT\&CK for industry alignment.
- **Decision-Making** – A transparent decision tree determines appropriate responses (no action, standard mitigation, or immediate intervention).
- **Blockchain Enforcement** – AI triggers Hyperledger Fabric smart contracts that execute predefined controls.
- **Adaptive Learning** – Reinforcement learning monitors the outcomes of actions and continuously refines threat classification models.

### Blockchain’s Dual Role

Blockchain performs **two critical functions**:

- **Immutable Auditability** – Every threat detection, AI decision, and contract execution is recorded immutably, providing trusted logs for internal security teams and external regulators.
- **Execution Platform** – Smart contracts enforce compliance rules in real time, ensuring consistent, tamper-proof security actions such as access revocation, protocol updates, or encryption enforcement.

Each blockchain transaction includes rich **metadata** (threat type, attack vectors, recommended changes, severity levels), ensuring that every enforcement action is both **auditable and context-aware**. This creates a **closed adaptive loop**:
AI proposes → Blockchain validates & executes → Results feed back into AI → AI improves decision-making.

### Challenges and Future Directions

Despite its promise, real-world deployment faces hurdles:

- **Performance** – Blockchain throughput is limited; techniques like sharding or off-chain channels may mitigate bottlenecks.
- **Integration** – Legacy IT systems require APIs or adapters to integrate with AI-blockchain frameworks.
- **Talent Shortage** – Expertise in both blockchain and AI is scarce, raising adoption barriers.
- **Legal & Ethical Issues** – Automated enforcement involves sensitive data and must respect privacy, fairness, and regulatory boundaries.

Future research directions include:

- **Stronger AI models** for predicting **unknown attack vectors**, not only recognizing known threats.
- **Privacy-preserving computation** (e.g., federated learning, secure multiparty computation) to enable safe CTI sharing.
- **Next-generation cryptography** such as zero-knowledge proofs and quantum-resistant protocols for stronger guarantees.
- **Policy harmonization** to account for regulatory variations across jurisdictions and industries.

<details><summary>Code</summary>

```Algorithm
AI-Driven Compliance and Response Algorithm

  Algorithm: AI-Driven Compliance and Threat Response
  Input: CTI_feed (Cyber Threat Intelligence feed)
  Output: decision, action_result

  1: Initialize SecureBERT model M, Random Forest classifier RF
  2: Load policy database P (PostgreSQL)
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

  Core Algorithm Flow

  1. CTI Data Cleaning & Structuring (Line 5)
  2. SecureBERT Semantic Analysis (Line 6)
  3. Random Forest Threat Classification (Line 7)
  4. Policy Matching (Line 8)
  5. Decision Tree Judgment (Line 9)
  6. Smart Contract Execution (Line 10)
  7. Reinforcement Learning Update (Line 11)
```

</details>

---

## 2. Cyber Threat Intelligence (CTI) Marketplace: Blockchain-Enabled Trust and Incentives

The **Cyber Threat Intelligence (CTI) marketplace** is an emerging platform model designed to facilitate the secure exchange of threat intelligence services among multiple stakeholders. Typically, three parties participate:

- **CTI Requesters** purchase intelligence services to defend against advanced cyberattacks.
- **CTI Providers** supply these services and receive financial rewards.
- **The Blockchain Network** acts as a trusted intermediary, ensuring that all transactions are tamper-proof, auditable, and incentive-compatible.

By leveraging **consortium blockchains**, the CTI marketplace fosters trust among organizations, ensures transparency, and enforces incentive mechanisms through smart contracts. This structure motivates providers to deliver high-quality services while guaranteeing requesters fair and secure access to threat intelligence.

The global CTI market is projected to grow from **\$5.8 billion in 2024** to **\$24.85 billion in 2032**, with a compound annual growth rate (CAGR) of approximately **20%**, underscoring both demand and strategic importance.

#### Consortium Blockchain-Based CTI Service Platform

A blockchain-powered CTI marketplace typically consists of three main roles:

- **CTI Requesters**

  - Organizations facing cyberattacks beyond their internal defensive capacity (e.g., DDoS, Advanced Persistent Threats).
  - They register on the platform and submit service requests with associated fees.
  - Registration and service requests are secured using **asymmetric encryption**, ensuring confidentiality and integrity.

- **CTI Service Platform**

  - Acts as the central coordinator, bridging requesters and providers.
  - Receives service requests and broadcasts them to registered providers.
  - Evaluates proposed service plans based on **quality and cost**, selecting the optimal provider.
  - Uses **smart contracts (chaincode)** to automate transactions and settlement.
  - Employs **zero-knowledge proofs (zk-SNARKs)** to guarantee transaction credibility while preserving data privacy.

- **CTI Providers**

  - Respond to broadcasted service requests by offering tailored service plans.
  - Their selection depends on service quality, cost-efficiency, and reputation.
  - Providers are rewarded proportionally to the value and effectiveness of the intelligence delivered.

#### Incentives and Market Dynamics

Traditional CTI platforms often rely on **reputation systems**, assigning ratings to providers. While reputation enhances trust, it lacks direct financial incentives. High-quality CTI services are costly to produce, and reputation alone does not guarantee long-term sustainability for providers.

The blockchain-enabled CTI marketplace introduces **economic incentives** alongside reputation mechanisms, aligning service quality with profitability.

- **Positive feedback**: An increase in requesters combined with higher-quality services raises overall market revenue, attracting more participants and strengthening platform sustainability.
- **Negative feedback**: Excessive provider competition or rising operational costs may drive prices and service quality downward, reducing profitability and weakening the marketplace.

At equilibrium, the marketplace functions as a cyclical mechanism:

- Requesters submit payments.
- The platform allocates tasks.
- Providers deliver CTI services.
- Quality feedback influences pricing and reputation.
- Demand adjusts accordingly, reinforcing the cycle.

#### Strategic Considerations for Platform Sustainability

For long-term viability, the CTI marketplace must carefully balance incentives among requesters, providers, and the platform itself. Key strategies include:

- **Provider Management** – Avoid oversupply, which dilutes profitability and reduces quality.
- **Quality Assurance** – Establish robust evaluation metrics to reward providers delivering effective intelligence.
- **Demand Expansion** – Prioritize attracting more requesters, as their participation drives overall market growth.
- **Equitable Revenue Sharing** – Ensure fair distribution of revenue between the platform and providers to sustain incentives.

Ultimately, **service quality is the central determinant of ecosystem health**. A high volume of engaged requesters combined with strong provider performance elevates collective revenue and market vitality. Conversely, unchecked provider competition and rising costs risk undermining service quality and discouraging participation.

---

### 2.1 Types and Classifications of Cyber Threat Intelligence (CTI)

**Cyber Threat Intelligence (CTI)** is derived from the analysis of current and potential cyber threats, including attack context, mechanisms, and indicators of compromise (IoCs). By understanding attackers’ **tactics, techniques, and procedures (TTPs)**, CTI enables organizations to **proactively detect, defend against, and respond to cyberattacks**.

The challenge, however, lies in the **frequency and complexity of cyberattacks**, which makes classification and layering of threat intelligence difficult. Moreover, establishing **trust among organizations** to securely share intelligence remains a persistent obstacle. Advanced methods such as **machine learning and behavioral analytics** can reduce detection and response times by nearly **60%**. Trust-based sharing mechanisms—such as **Information Sharing and Analysis Centers (ISACs)** and standardized frameworks like **STIX (Structured Threat Information eXpression)** and **TAXII (Trusted Automated eXchange of Indicator Information)**—have been shown to improve collaborative defense capabilities by about **65%**. Combining **standardized sharing protocols, advanced analytics, and machine learning** significantly strengthens cybersecurity resilience.

#### Categories of Threat Intelligence

Threat intelligence is commonly divided into **three main categories**, each serving distinct layers of security needs:

- **Indicator-Based Intelligence (IoCs)**

  - Consists of technical “footprints” left by attackers, such as IP addresses, domain names, file hashes, registry keys, and malware signatures.
  - Highly **actionable and immediate**, enabling quick detection and blocking of attacks.
  - Best suited for **short-term defense**, allowing security teams to determine whether an attack has already occurred.

- **Tactical Intelligence**

  - Focuses on attackers’ **TTPs** (Tactics, Techniques, and Procedures), including reconnaissance, initial intrusion, lateral movement, and data exfiltration.
  - Helps organizations **predict adversary behavior**, understand their capabilities and intent, and design targeted defenses.
  - Supports **medium-term defense** by guiding Security Operations Centers (SOCs) in building detection rules and monitoring strategies.

- **Strategic Intelligence**

  - Examines **macro-level factors**, such as long-term threat trends, emerging attack vectors, and geopolitical risks.
  - Informs **executive decision-making**, resource allocation, and security investment strategies.
  - Essential for **long-term defense planning** at the organizational and national level.

#### Sources of Threat Intelligence

CTI typically originates from two types of sources:

- **Open-Source Intelligence (OSINT)**

  - Drawn from publicly available data such as websites, social media, forums, and blogs.
  - Collected using crawlers, data mining, and social network analysis.
  - Often integrated into **Threat Intelligence Platforms (TIPs)** for automation.
  - Advantages: free, abundant, and diverse.
  - Limitations: variable accuracy, requiring careful validation.

- **Closed-Source Intelligence (CSINT)**

  - Proprietary or restricted information, such as vendor datasets, private databases, and closed communities.
  - Often provided by **security vendors, CTI platforms, and industry sharing groups**.
  - Typically higher in reliability and tailored to specific sectors or organizations.

Organizations achieve the best results by **combining OSINT and CSINT**, gaining both breadth and depth of coverage.

#### Methods of Collection, Analysis, and Distribution

The CTI lifecycle follows a **collect → analyze → distribute** model. Key techniques include:

- **Passive DNS Monitoring** – Identifying malicious domains and command-and-control communications through DNS traffic analysis.
- **Behavioral Analytics** – Monitoring user, application, and entity behaviors to spot anomalies, effective for zero-day and unknown threats.
- **Information Sharing** – Distributing actionable intelligence among trusted partners to improve collective defense.

To facilitate sharing and interoperability:

- **STIX and TAXII** provide standardized formats and exchange protocols.
- **Threat Intelligence Platforms (TIPs)** and **Security Information and Event Management (SIEM)** systems automate ingestion, correlation, and response.

**Analytical approaches** include:

- **Machine Learning** to recognize attack patterns and anomalies.
- **Big Data Analytics** to uncover hidden threats in large volumes of logs and network traffic.
- **Behavioral and Anomaly Detection** to detect unknown or zero-day attacks.
- **Human Expertise** to provide context, judgment, and validation of automated results.

#### Integrating CTI into Cybersecurity Frameworks

CTI can be mapped to existing frameworks for stronger resilience:

- The **NIST Cybersecurity Framework (CSF)** provides structured risk management processes.
- The **MITRE ATT\&CK matrix** offers a detailed mapping of adversary tactics and techniques, helping organizations align CTI findings with operational defenses.

By embedding CTI into security operations, organizations can **anticipate attacker behavior**, craft tailored defense strategies, and enhance overall resilience.

---

### 2.2 Federated Learning (FL)-Based Threat Intelligence Models: Privacy-Preserving Collaborative CTI Across Geopolitical Boundaries

In today’s digital landscape, **multinational corporations, financial institutions, and government agencies** must collaborate to counter increasingly sophisticated cyberattacks. Effective defense requires the **sharing of threat intelligence (TI)** across borders and organizational boundaries. However, **traditional centralized threat modeling** relies on aggregating sensitive data in a single platform, which introduces critical risks:

- **Data sovereignty and compliance** – regulations such as the **EU’s GDPR**, **California’s CCPA**, **Russia and India’s data localization laws**, and **China’s Cybersecurity Law** impose strict limits on cross-border data flows.
- **Geopolitical sensitivities** – cross-border intelligence sharing may touch on national security concerns.
- **Corporate competition and espionage risks** – sharing data between rival firms can create exposure.
- **Privacy and surveillance issues** – centralized storage increases the chance of large-scale leaks.

To address these challenges, **Federated Learning (FL)** has emerged as a promising approach. FL enables multiple organizations to **jointly train threat detection and intelligence models without sharing raw data**. Instead, participants retain their data locally and only share **model parameters or gradients**, which are then aggregated by a central coordinator. This model preserves **data sovereignty**, avoids cross-border raw data transfers, and leverages diverse global threat datasets for improved accuracy.

#### Applications of FL in Cybersecurity

- **Intrusion Detection Systems (IDS):** organizations share traffic feature updates without disclosing raw logs.
- **Malware Classification:** research labs in different countries collaboratively build models without exchanging actual malware samples.
- **Fraud and Anomaly Detection:** banks and stock exchanges share models to identify cross-border fraud without exposing sensitive financial records.

#### Challenges in Applying FL to Cybersecurity

Deploying FL in adversarial, cross-border cybersecurity environments introduces unique challenges:

- **Adversarial Participants (Model Poisoning):** Malicious nodes may upload poisoned updates to corrupt the global model. Countermeasures include robust aggregation methods (**Krum, Trimmed Mean**) and **reputation scoring**.
- **Inference Attacks:** Adversaries may attempt to infer sensitive data distributions from shared model parameters. Solutions include **Differential Privacy (DP)**, where noise is added to updates to obscure individual data contributions.
- **Secure Aggregation:** Parameters must remain protected during transmission and aggregation. Techniques such as **Homomorphic Encryption (HE)** and **Secure Multi-Party Computation (SMPC)** allow computations to occur on encrypted data, preventing leaks.
- **Unequal Trust Environments:** Participants may have different levels of mutual trust, especially across nations. Mechanisms like **tiered authentication, auditing, and accountability frameworks** ensure integrity and transparency.

#### Federated Learning Architectures

FL is a **decentralized machine learning paradigm**, where data remains at the source while only encrypted updates are exchanged. Key forms include:

- **Horizontal FL:** Participants share the same feature space but different users (e.g., hospitals with similar patient fields, distributed security sensors).
- **Vertical FL:** Participants share the same users but different features (e.g., telecoms + banks combining call records and financial transactions for fraud detection).

#### Deployment Modes

- **Cross-Device FL:** Involving IoT, mobile, and edge devices for endpoint anomaly and malware detection. Challenges: limited device computation, bandwidth constraints, and hardware heterogeneity.
- **Cross-Silo FL:** Involving fixed organizations such as **national cybersecurity centers, intelligence agencies, or multinational enterprises**. With greater computational capacity and stability, this model is suitable for critical industries (energy, power, finance, healthcare) that require high accuracy and low latency.

#### System Architecture for FL-Based Threat Intelligence

An FL-based distributed detection system can be conceptualized in three layers:

- **Clients (Participants):** Cybersecurity Operations Centers (CSOCs), such as government defense agencies, hospitals, and enterprises, retain local logs, anomalies, and forensic signals. Each trains a **local model** on-site.
- **Federated Server:** Collects encrypted parameters/gradients, performs **model aggregation** (e.g., FedAvg), and broadcasts the updated global model back to participants.
- **Secure Relay Layer (Optional):** Employs cryptographic protections (HE, SMPC) to secure data exchanges against inference or poisoning attacks.
- **Policy Control Plane:** Manages communication frequency, trust levels, and differential privacy thresholds to ensure transparency, auditability, and compliance across borders.

This design ensures **“data never leaves its jurisdiction”** (raw logs remain local) while still enabling collaborative intelligence.

#### Models for Different Detection Tasks

- **Intrusion Detection (IDS):** RNNs / LSTMs capture temporal features in logs and traffic to detect slow scans, insider threats, and brute-force attacks.
- **Malware Classification:** CNNs transform binary/memory dumps into visual patterns for classification, while ensemble methods (Random Forest, XGBoost) handle structured log data.
- **Zero-Day Exploit Detection:** Autoencoders and Variational Autoencoders (VAEs) detect anomalies by measuring reconstruction error or shifts in latent feature distributions.

**Training workflow:**

- Each node trains locally.
- Encrypted weight updates are uploaded.
- The federated server aggregates results.
- The global model is redistributed for further training.
  This **iterative cycle** balances **global awareness** with **local contextual relevance**, allowing **multi-task, multi-model collaboration** across heterogeneous environments.

#### Secure Aggregation & Advanced FL Protocols

The **security of parameter aggregation** is central to FL in cybersecurity.

- **FedAvg (Federated Averaging):** The standard method that averages local updates weighted by dataset size. Vulnerable to poisoning attacks.
- **FedProx:** Introduces a proximal term to address **non-IID data**, where participant datasets (e.g., national defense vs. small enterprise logs) vary significantly.
- **Secure Aggregation Protocols (e.g., Bonawitz et al.):** Utilize additive secret sharing and homomorphic encryption so that only the aggregated results are visible, preventing inference of individual contributions.

In hostile or politically sensitive environments, **“secure aggregation = technological trust guarantee”**, mitigating political distrust and ensuring sustained collaboration.

---

## 3. Web3 Cyber Threat Intelligence (CTI)

In the Web3 landscape, cyber threats extend beyond traditional IT attack vectors. Smart contract vulnerabilities, on-chain exploits, and cross-chain bridge attacks introduce entirely new dimensions of threat intelligence. Traditional DeFi security solutions mainly focus on **transaction-level detection**. However, with the rise of **private transaction pools**, attackers can bypass the public mempool by submitting transactions directly to miners. This makes conventional monitoring systems blind to many attack attempts.

### Emerging Threat Vectors in DeFi

DeFi attacks typically fall into two categories:

- **Direct exploitation of vulnerable contracts**, where attackers invoke exposed functions to trigger vulnerabilities.
- **Adversarial Contract-based attacks**, in which attackers deploy a malicious intermediate contract that encapsulates the attack logic and then calls the victim contract.

Current detection systems suffer from two limitations:

- **Transaction-based detection** (heuristics or ML models analyzing mempool traffic) becomes ineffective in private mempool scenarios. Data shows that **over 56% of Ethereum attacks leveraged private pools**, rendering public mempool-based detection unreliable.
- **Contract-based detection**, such as Forta (NLP + Logistic Regression) and BlockWatchDog (limited to reentrancy), is either too narrow in scope or too weak in performance to capture the diversity of modern DeFi exploits.

This calls for a paradigm shift—from **passive transaction monitoring** to **proactive contract analysis**. Instead of waiting for an attack transaction, detection must identify **malicious contracts at deployment**, before exploitation occurs.

### Contract-Based Detection: Predicting Attacks Before They Happen

A contract-centric perspective enables proactive defense. By focusing on the **deployed contract itself**, rather than the eventual transactions, we can predict whether a contract is likely adversarial.

Detection relies on extracting three categories of features:

- **Attack Pattern Features**: e.g., reentrancy, flash loan behaviors, and unusual call sequences.
- **Code Semantics**: extracted via static analysis of function calls, event logs, and variable dependencies to uncover latent malicious intent.
- **Intrinsic Characteristics**: bytecode complexity, function density, and abnormal gas usage patterns.

Machine learning classifiers trained on labeled datasets of adversarial contracts can then predict the **likelihood and type of attack** (e.g., reentrancy, arbitrage, flash loan). Importantly, this approach works regardless of whether the contract is executed via a public or private pool.

### Defense Opportunity: The Time Window

Even with private pools, defenders retain a **critical time window**:

- When an adversarial contract is deployed, the **victim address is often hardcoded** and thus publicly visible.
- There is typically a short delay (≤100s) between deployment and the first malicious invocation.
- This creates an opportunity to **preemptively pause or protect the target contract** (e.g., ConicEthPool’s `shutdownPool` emergency method).

Thus, contract-based detection shifts defense from **reactive blocking** to **proactive prevention**.

### Behavior Signatures of Malicious Contracts

Despite differing vulnerabilities, adversarial contracts share consistent behavioral traits:

- Anonymous or obfuscated funding sources (mixers, non-KYC exchanges, bridges).
- Closed-source contracts to prevent reverse engineering.
- Frequent token-related function calls indicative of asset transfers.
- Abnormal interaction patterns with victim contracts (non-standard call sequences, callback dependencies, unusual state checks).

These invariant behavioral signals allow ML classifiers to generalize across different exploit types.

### System Model

The proposed detection system operates **at deployment time**:

- **Chain Monitoring** collects deployment metadata (deployer address, funding source, gas cost, bytecode).
- **Binary Lifting** converts EVM bytecode into an **Intermediate Representation (IR)** using the Gigahorse framework.
- **PSCFT (Pruned Semantic-Control Flow Tokenization)** extracts token-related operations, flash loan patterns, and external calls, yielding a text-like representation for ML models.
- **Classification** is performed via a hybrid ML architecture:

  - **Transformer models** analyze PSCFT semantic sequences.
  - **Feature-based models** (Logistic Regression, XGBoost, Random Forest) evaluate structured metadata (funding source, gas usage, function count).
  - **Meta Classifier** combines both outputs using stacking, reducing overfitting while improving prediction accuracy.

### Defense Mechanisms

Once flagged, multiple defenses can be triggered:

- **Notifications to target contracts** and their owners.
- **Emergency shutdown methods** to temporarily halt deposits/interactions.
- **On-chain blacklisting** of confirmed adversarial contracts.

This proactive framework extends CTI into Web3 by shifting the focus from **monitoring attack transactions** to **predicting attack contracts**.

### Dataset Construction

Training effective ML models requires carefully curated datasets:

- **Positive samples (Adversarial Contracts)**: derived from documented DeFi exploits (e.g., GitHub datasets). Categories include unsafe calls (e.g., reentrancy), access control flaws, and coding errors.
- **Negative samples (Benign Contracts)**: harder to obtain. Verified contracts on Etherscan are commonly used, but care must be taken to avoid duplicates or trivial internal transactions.

By balancing positive and negative datasets, and applying data augmentation techniques like ADASYN, classifiers are better able to detect adversarial patterns.

### Machine Learning–Based Detection of Malicious Smart Contracts

Machine learning (ML) models for detecting malicious smart contracts rely on two broad categories of features: **deployment-phase features** and **implementation-phase features**.

#### Deployment-Phase Features

These features capture the developer’s behavior during contract deployment, providing signals that distinguish between attackers and legitimate developers:

- **Nonce**: The number of transactions executed by the deployer before deployment, representing their level of activity.
- **Fund Source**: The origin of the deployment funds. Possible categories include:

  - **Safe** – centralized exchanges with KYC verification,
  - **Anonymous** – mixers or non-KYC exchanges,
  - **Bridge** – cross-chain sources.

- **Transaction Data**:

  - **Value** – whether native tokens are transferred during deployment; malicious contracts often transfer none.
  - **Input Data Length** – the length of the contract bytecode; malicious contracts tend to be shorter.
  - **Gas Used** – resources consumed during deployment, correlated with bytecode complexity.

- **Verification Status**: Whether the developer submitted verified source code and compiler information.

### PSCFT: Pruned Semantic-Control Flow Tokenization

To analyze implementation behavior, contract bytecode is transformed into a textual feature representation suitable for ML models. This is achieved through **PSCFT**, which works as follows:

- **Bytecode Lifting**: Using **Gigahorse**, EVM bytecode is elevated into an intermediate representation (IR).
- **Feature Extraction via Soufflé Datalog**:

  - **Opcode Features** – counts of sensitive instructions such as `DELEGATECALL` or `SELFDESTRUCT`.
  - **Function Features** – function entries and selectors are collected using `FunctionEntry` and `PublicFunctionSelector`.
  - **External Call Features** – extracted via `ExternalCallResolved`, including the total number, maximum per function, and average calls.

- **CFG Construction and Pruning**: A control flow graph (CFG) is generated for each function, where nodes represent basic blocks and edges capture predecessor–successor relationships.
- **Semantic Recovery**: Improves interpretability of external calls:

  - **Address Label Recovery** – attempts to map addresses using known label libraries; if no match is found, tracks storage variables or marks as `UnknownTarget`.
  - **Function Selector Recovery** – uses the 4byte API to map function selectors to human-readable names; unresolved cases are marked as `UnknownFunc`.

The final output is a PSCFT text representation describing function control flows and external call chains—readable for humans, yet optimized for processing by Transformer models.

### Transformer-Based Semantic Modeling

The PSCFT textual representation is fed into a Transformer model:

- **Embedding Layer**: PSCFT tokens are converted into dense vectors using Word2Vec.
- **Encoder Layers**: Multi-head self-attention and feed-forward layers capture semantic dependencies.
- **Pooling & Dense Layers**: Aggregate encoded representations to predict contract classification.

This process allows the model to learn semantic and behavioral patterns of contracts.

#### Ensemble Classification with Meta-Learning

To strengthen predictive power, semantic features are combined with traditional engineered features in an ensemble framework:

- **Feature Models**: Classic ML algorithms—including Logistic Regression, Decision Trees, Random Forests, and XGBoost—are trained on engineered features such as fund source, function counts, and external calls.

  - To address class imbalance, **ADASYN** is used to generate adversarial samples.
  - Features are standardized or One-Hot encoded before training.

- **Meta Classifier**: Predictions from both the Transformer model and feature-based models are passed to a **Stacking ensemble**. Lightweight algorithms such as KNN, LR, SVM, and DT serve as meta classifiers, outputting the final adversarial probability score.

This two-level ensemble leverages both **semantic insights** from the Transformer and **statistical signals** from deployment/behavioral features, while reducing the risk of overfitting.

<details><summary>Code</summary>

```Algorithm
    // Raw IR

    function 0xd920755a() public {
        block 0x64, prev=[], succ=[0x6c, 0x70]
        0x65: v65 = CALLVALUE
        0x67: v67 = ISZERO v65
        0x68: v68(0x70) = CONST
        0x6b: JUMPI v68(0x70), v67
        ...
        block 0x268, prev=[0x24e], succ=[0x273, 0x27c]
        0x26a: v26a = GAS
        0x26b: v26b = CALL v26a,
            v211(0x5777d92f208679db4b9778590fa3cab3ac9e2168),
            v258(0x0), v253, v256(0xa4), v253, v24f(0x0)
        0x26c: v26c = ISZERO v26b
        0x272: JUMPI v26f(0x27c), v26e
        block 0x27c, prev=[0x268], succ=[0xbfcB0x27c]
        0x281: v281(0x2) = CONST
        0x283: v283(0x0) = CONST
        0x286: v286 = SLOAD v281(0x2)
        0x288: v288(0x100) = CONST
        0x28b: v28b(0x1) = EXP v288(0x100), v283(0x0)
        0x28d: v28d = DIV v286, v28b(0x1)
        ...
    }

    function uniswapV3FlashCallback(args)() public {
        Begin block 0x7b
        ...
    }
...

    // PSCFT

    [START]
    FUNCTION uniswapV3FlashCallback public
        BB0:
            Proxy.approve
            EDGES:
                BB1, BB2

        BB1:
            Proxy.balanceOf

        FUNCTION END
    FUNCTION UnknownFunction0 public
        BB0:
            UniswapV3Pool.flash
            EDGES:
                BB1

        BB1:
            internalFunc0

        FUNCTION END
    FUNCTION internalFunction0 private
        BB0:
            Token.balanceOf

        FUNCTION END
    [END]
```

</details>

[CTI Corpus](https://github.com/AutoCS-wyh/Automotive-cyber-threat-intelligence-corpus)
[LookAhead](https://github.com/zju-abclab/LookAhead)
[OpenCTI](https://github.com/OpenCTI-Platform/opencti)
[Awesome CTI](https://github.com/hslatman/awesome-threat-intelligence)
[OASIS](https://oasis-open.github.io/cti-documentation/)
