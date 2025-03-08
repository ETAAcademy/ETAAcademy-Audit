# ETAAcademy-Adudit: 8. Dynamic Analysis and Deep Learning

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>08 Dynamic Analysis and Deep Learning</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>Dynamic_Analysis_Deep_Learning</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# Dynamic Analysis and Deep Learning

Dynamic Analysis provides powerful runtime insights but comes with computational overhead, the combination of **Static Analysis, Dynamic Analysis, and Deep Learning** offers automation and pattern recognition advantages. Leveraging deep learning, especially the Transformer architecture, LLMs have demonstrated capabilities in identifying vulnerabilities across various programming languages, including Solidity-based contracts. Techniques like RAG and fine-tuning have notably improved detection accuracy, although challenges persist, such as issues with cross-file detection, identifying zero-day vulnerabilities, and reducing false positives.

---

## Dynamic Analysis

Dynamic Analysis and Static Analysis are two core methods in software analysis and security. **Static Analysis** examines code without executing it to infer program behavior, while **Dynamic Analysis** observes a program’s behavior during execution, allowing it to detect runtime issues that Static Analysis might miss.

#### 1. Techniques in Dynamic Analysis

Dynamic Analysis encompasses various techniques, including **fuzzing, dynamic program slicing, runtime monitoring, and emulation**.

- **Fuzzing** is an automated testing method that generates random or semi-structured inputs to trigger program anomalies, such as crashes or memory errors, thus uncovering vulnerabilities. There are different types of fuzzing:

  - **Generation-based**: Constructs inputs based on predefined formats or protocols (e.g., Peach Fuzzer).
  - **Mutation-based**: Modifies existing inputs randomly (e.g., AFL - American Fuzzy Lop).
  - **Guided fuzzing**: Uses coverage feedback to optimize input generation (e.g., AFL++, LibFuzzer, honggfuzz).
  - Fuzzing is effective in detecting issues like buffer overflows and format string vulnerabilities but may struggle with deeper logical flaws. Tools like **Echidna**, **AFL**, **LibFuzzer**, and **honggfuzz** are widely used.

- **Dynamic Program Slicing** tracks data flow and control flow during execution to extract relevant code segments that influence specific variables. Compared to static slicing, this method is more precise as it only considers executed paths. It is useful for debugging and regression testing but limited by input data coverage. Examples include **JSlice (for Java)** and **DynSlice**.

- **Runtime Monitoring** involves inserting checks into a running program to ensure adherence to predefined rules, such as memory safety and concurrency locks. It can detect runtime errors such as memory leaks and race conditions:

  - **Memory safety detection**: Valgrind, AddressSanitizer.
  - **Concurrency bug detection**: ThreadSanitizer.
  - **Behavior enforcement**: AppArmor (restricts process permissions).
  - It directly captures runtime errors (e.g., use-after-free) but may introduce overhead.

- **Emulation** executes a program in a controlled environment, such as a virtual machine or sandbox, to monitor its behavior. This is particularly useful for security analysis and malware detection. Tools like **Cuckoo Sandbox** record file, network, and registry operations. However, some programs may use anti-VM techniques to evade detection.

#### 2. Auxiliary Dynamic Analysis Techniques

Several additional methods assist in runtime analysis and optimization:

- **Profiling**: Measures resource consumption (CPU, memory, I/O) to identify performance bottlenecks. Tools include **gprof, perf, and VisualVM**.
- **Coverage Analysis**: Evaluates test effectiveness by tracking executed code paths. It includes **line coverage** (tracks executed lines) and **branch coverage** (checks all conditional branches). Tools like **gcov and JaCoCo** help improve test suites.
- **Dynamic Binary Instrumentation (DBI)**: Inserts analysis code at runtime without modifying binaries, useful for behavior monitoring and vulnerability detection. Common tools include **DynamoRIO and Pin**.
- **Dynamic Reverse Engineering**: Uses debuggers to analyze execution flow and uncover program logic. Tools such as **GDB, x64dbg, and IDA Pro (with dynamic debugging plugins)** are widely used.
- **Hot Patching**: Allows real-time bug fixes or updates without restarting the application. This technique is commonly used in critical systems (e.g., Windows Live Patching).

Dynamic Analysis helps capture runtime-specific issues (e.g., memory errors, race conditions), but it is constrained by execution coverage and performance overhead. **In practice, Static and Dynamic Analysis are often combined**: Static Analysis for initial filtering and Dynamic Analysis for deeper vulnerability verification. For instance, static checks ensure coding standards, while fuzzing uncovers deep vulnerabilities.

---

## Deep Learning

Deep Learning (DL) has achieved significant breakthroughs in **computer vision (CV) and natural language processing (NLP)**, leading researchers to explore its application in **software vulnerability detection**. Traditional methods like Static and Dynamic Analysis have inherent limitations:

- **Static and Dynamic Analysis** may suffer from **false positives (incorrect alerts) and false negatives (missed vulnerabilities)**.
- **Dynamic Analysis, such as Fuzzing**, requires **substantial computational resources and time**, and it may still fail to detect certain vulnerabilities due to dependency on input generation.

In contrast, deep learning offers greater automation capabilities, reducing the need for human intervention and improving **detection accuracy** and **generalization capability**.

#### 1. Vulnerability Detection Granularity

Vulnerability detection tasks can be classified into four different granularity levels: **file-level (Level 1), function-level (Level 2), code snippet-level (Level 3), and token-level (Level 4)**. Each level has its unique challenges:

- **File-level detection** focuses on determining whether a vulnerability exists in an entire file. However, this approach struggles with pinpointing the exact location of the vulnerability, resulting in the **"Lost in the Woods (LIW)" problem** due to the overwhelming volume of code.

- **Function-level detection** reduces the search space by analyzing individual functions but still suffers from the LIW problem and high computational costs.
- **Code snippet-level detection** extracts relevant code snippets to improve detection accuracy. Prominent models like **VulDeePecker** and **DeepWukong** leverage deep learning to enhance code representation. However, they encounter the **"Lost in Translation (LIT) problem"**, where global contextual information may be lost during the segmentation process.
- **Token-level detection** focuses on the smallest code units such as operators, keywords, and variables, providing detailed structural information. However, the LIT problem persists, leading to potential false positives or false negatives.

A promising research direction involves integrating multi-granularity strategies. For example, combining file or function-level methods for coarse-grained detection with snippet or token-level methods for fine-grained detection can significantly enhance detection accuracy. Additionally, improving the fusion of **syntactic and semantic information** can mitigate the LIT problem, leading to more precise vulnerability detection. Furthermore, introducing **automated repair (Auto-Fix)** or **AI-assisted code analysis** can reduce human intervention, increasing the practicality and scalability of vulnerability detection systems.

#### 2. Code Representation for Deep Learning Models

Once the detection granularity is determined, the next challenge is how to represent code so that deep learning (DL) models can effectively learn crucial features. Effective code representation is critical for accurate vulnerability detection. Researchers have explored various methods to capture both **syntax** and **semantic features** of code, which can be broadly classified into five categories:

- **Abstract Syntax Tree (AST)**: This method captures the syntactic structure of the code in a tree format, where each node represents a syntactic construct. ASTs efficiently preserve the hierarchical structure of code but fail to capture deeper semantic information. Moreover, in large codebases, ASTs can become prohibitively large and complex to manage.

- **Graph-Based Methods**: Graph representations such as **Call Graphs (CG)** or **Program Dependency Graphs (PDG)** capture both **syntax and semantic relationships** within the code. While they enhance detection accuracy, the construction and analysis of such complex graphs incur high computational costs, limiting their scalability.

- **Hybrid Methods**: Combining multiple representation methods, such as ASTs, graph-based representations, and NLP techniques, provides richer code representations. Although this approach achieves high detection accuracy, it demands significant computational resources and increases implementation complexity.

- **Natural Language Processing (NLP) Methods**: These methods analyze code comments and documentation to capture semantic information, facilitating the understanding of code functionality. However, they lack sensitivity to syntax structures, limiting their ability to capture the actual code execution logic.

- **Embedding-Based Methods**: Techniques such as **Word2Vec**, **CodeBERT**, or other pre-trained language models convert code into low-dimensional vector embeddings. These embeddings efficiently capture semantic patterns but may overlook important syntactic structures, impacting precise vulnerability detection.

#### 3. Vectorization and Model Selection

After preprocessing and representing code, the next critical step is **vectorization** and **model selection**. Different models require specific data formats and exhibit varying sensitivity to feature granularity. Common vectorization techniques include:

- **One-hot encoding**
- **Word2Vec** (CBOW and Skip-gram)
- **Doc2Vec** (PV-DM and PV-DBOW)

The choice of vectorization directly impacts the learning performance and computational efficiency of the model.

#### 4. Deep Learning Models for Vulnerability Detection

Current deep learning models for vulnerability detection can be classified into three categories: **sequence-based models, graph-based models, and hybrid models**.

- **Sequence-Based Models**: These models capture the sequential patterns in code, akin to natural language text, and are widely used in vulnerability detection. Representative models include:

  - **RNN (Recurrent Neural Network)**
  - **LSTM (Long Short-Term Memory)**
  - **BLSTM (Bidirectional LSTM)**
  - **GRU (Gated Recurrent Unit)**
  - **BiGRU (Bidirectional GRU)**
  - **Transformers (Self-Attention Mechanism)**

  Notable research such as **VulDeePecker, SySeVR, and µVuldeePecker** use BLSTM or BiGRU models to detect multi-class vulnerabilities by capturing contextual patterns from code sequences.

- **Graph-Based Models**: These models exploit graph structures (AST, PDG, CFG) to capture both code syntax and semantic dependencies. Prominent graph-based models include:

  - **GNN (Graph Neural Network)**
  - **GCN (Graph Convolutional Network)**
  - **GAT (Graph Attention Network)**

  Examples like **Devign, DeepWukong, and DeepTective** have demonstrated superior performance in identifying vulnerabilities across different programming languages and vulnerability types, such as **SQL Injection (SQLi)** or **Cross-Site Scripting (XSS)**.

- **Hybrid Models**: Combining sequence and graph-based approaches can enhance detection performance. For instance, **Russell et al.** proposed a method that applies CNN for feature extraction, RNN for sequence modeling, and Random Forest for classification, achieving higher detection accuracy.

#### 5. Current Challenges and Future Directions

Despite promising results, deep learning models for vulnerability detection still face several challenges:

- **High Computational Costs**: Building complex models (such as GNN or Transformers) requires extensive computational resources, especially for large-scale code analysis.
- **Long Inference Time**: Real-time vulnerability detection remains a challenge due to the long inference time of deep learning models.
- **Limited Generalization**: Models often struggle to generalize across different programming languages or detect novel vulnerabilities, reducing their practical value.

Future research should focus on:

- **Optimizing Model Architectures**: Developing lightweight models with minimal inference time while maintaining high accuracy.
- **Enhancing Code Representation**: Creating hybrid embeddings that capture both syntax and semantic information with high efficiency.
- **Improving Generalization**: Using few-shot or zero-shot learning to improve the model's adaptability to unseen vulnerabilities.
- **Automated Repair (Auto-Fix)**: Incorporating AI-powered code repair mechanisms to not only detect but also automatically patch vulnerabilities.

By addressing these challenges, deep learning can significantly enhance the accuracy, scalability, and practical applicability of software vulnerability detection, advancing the security of modern software systems.

---

## Large Language Models (LLMs) in Vulnerability Detection

Large language models (LLMs) have made remarkable advancements in the field of natural language processing (NLP), showcasing exceptional capabilities in software development and vulnerability detection. Powered by deep learning techniques, especially the Transformer architecture, LLMs have been widely applied to tasks involving vulnerability detection across various programming languages. Recent developments have demonstrated notable progress, particularly in detecting vulnerabilities in C/C++, Java, and Solidity codebases.

The application of LLMs in vulnerability detection has primarily focused on three major programming languages: C/C++, Java, and Solidity. Each language presents unique challenges: for example, C/C++ often encounters memory management issues, Python faces insecure deserialization, and Java suffers from object injection and reflection vulnerabilities. For Solidity, the security of blockchain smart contracts directly impacts the safety of financial transactions. Existing LLM-based vulnerability detection research tends to address specific challenges associated with these languages. Despite significant achievements by decoder-based models like GPT and CodeLlama, most existing datasets are limited to function-level and file-level vulnerabilities, lacking repository-level datasets that better reflect real-world development environments. Future research should aim to develop repository-level datasets, improve code semantic representation methods, and enhance cross-file vulnerability detection to increase the practical usability and reliability of LLMs.

#### 1. Key Techniques for LLM-Based Vulnerability Detection

The current application of LLMs in vulnerability detection still faces several critical challenges, including: (1) data leakage that may lead to artificially high performance, (2) difficulty in understanding complex code contexts, (3) information loss due to the context window length limitation, and (4) high false positive rates and insufficient capability in detecting zero-day vulnerabilities. To address these challenges, researchers have explored several key technical approaches to optimize LLM-based vulnerability detection, as outlined below:

##### 1.1 Code Data Processing Techniques

Optimizing code representation for LLMs is crucial to improving their vulnerability detection capabilities. Several data processing techniques have been widely adopted to enhance LLM performance, including graph representation, retrieval-augmented generation (RAG), and program slicing. However, despite their contributions, these methods remain limited in handling complex cross-file vulnerabilities.

- **Code Data Preprocessing:** The core goal of preprocessing is to optimize the use of LLM context windows and improve their understanding of code semantics. Abstract Syntax Trees (ASTs) are commonly used to strip unnecessary syntactic details while preserving semantic relationships between code components. However, since ASTs lack the ability to capture control and data flow, researchers have incorporated **Control Flow Graphs (CFGs)** and **Data Flow Graphs (DFGs)** to provide richer structural information, thereby enhancing vulnerability detection. **Call Graphs** have also been introduced to capture inter-function dependencies, allowing LLMs to better understand function call relationships.

- **Retrieval-Augmented Generation (RAG):** RAG combines information retrieval with generative models to mitigate LLMs' knowledge limitations in specialized domains. Some research has used the **Common Weakness Enumeration (CWE)** database as a knowledge base or incorporated vulnerability documentation, code snippets, and static analysis results to enhance LLMs' ability to detect vulnerabilities. In particular, GPT-4 has been leveraged to generate vulnerability report databases stored in VectorDBs, further boosting detection accuracy.

- **Program Slicing:** This technique extracts only the relevant code lines associated with a particular vulnerability, reducing the input size and improving LLM efficiency. For instance, research using program slicing has successfully isolated buffer overflow-related code segments by retaining only key functions (e.g., `strcmp`, `memset`). Some approaches also use fine-tuned LLMs to learn the automatic extraction of vulnerability-related code sections, further improving detection accuracy.

- **LLVM Intermediate Representation (LLVM IR):** This technique converts source code from different programming languages into a common intermediate representation (IR), enabling LLMs to detect vulnerabilities across languages. However, the limitation of LLVM IR is that it does not fully support Java and JavaScript, limiting its cross-language applicability.

##### 1.2 Prompt Engineering

Prompt engineering plays a crucial role in improving LLM performance in vulnerability detection tasks. By carefully designing prompts, researchers can guide LLMs to perform more accurate code analysis and vulnerability detection. Various prompt engineering strategies have been developed to optimize LLM performance.

- **Chain-of-Thought (CoT) Prompting:** CoT prompting guides LLMs through multi-step reasoning processes by first summarizing the code function, then analyzing potential vulnerabilities, and finally providing a vulnerability assessment. This method has been particularly effective for large models (>10B parameters).

- **Few-shot Learning (FSL):** FSL embeds examples of vulnerable code along with CWE vulnerability classification into prompts, allowing LLMs to more accurately identify and categorize vulnerabilities. This approach has been particularly effective for smaller LLMs.

- **Hierarchical Context Representation:** This method organizes code into hierarchical structures such as module → class → function → statement, allowing LLMs to gradually process higher-level and lower-level contexts. This significantly reduces information loss caused by context window limitations.

- **Multi-level Prompting:** This technique divides the vulnerability detection task into multiple stages, such as generating a code summary, identifying potential vulnerabilities, and analyzing critical code fragments. This stepwise approach improves the model's systematic understanding of code.

- **Multiple Prompt Agents:** This approach leverages different LLM agents with distinct prompts to independently perform code summarization, vulnerability identification, and detailed code analysis. By aggregating results from multiple agents, overall detection performance is improved.

##### 1.3 Fine-tuning

Fine-tuning LLMs using task-specific data has proven to significantly enhance their vulnerability detection capabilities. Fine-tuning approaches are broadly categorized into full fine-tuning (FFT) and parameter-efficient fine-tuning (PEFT).

- **Full Fine-tuning (FFT):** This approach updates all model parameters using task-specific datasets, achieving high detection accuracy but at a high computational cost. Popular models for full fine-tuning include GPT-4, CodeLlama, and CodeT5.

- **Parameter-Efficient Fine-tuning (PEFT):** This approach minimizes the number of trainable parameters to reduce computational costs. Common PEFT methods include **Adapters** (which insert small trainable layers between transformer layers), **LoRA (Low-Rank Adaptation)** (which updates model weights through low-rank matrix decomposition), and **QLoRA (Quantized LoRA)** (which combines low-rank adaptation with 4-bit quantization to enable efficient fine-tuning on low-resource devices).

- **Discriminative vs. Generative Fine-tuning:** Discriminative fine-tuning focuses on classifying code as vulnerable or secure, while generative fine-tuning outputs structured information such as vulnerability descriptions or affected code segments. Large models like GPT-4 and CodeLlama have achieved F1 scores close to 0.9 using fine-tuning techniques. However, obtaining high-quality datasets (>10K samples) remains critical to fine-tuning success.

#### 2. LLMs for Smart Contract Auditing

Recent studies have explored LLM-based approaches for smart contract auditing, leveraging LLMs' code comprehension and generation capabilities to detect vulnerabilities that static analysis tools like Mythril and Slither often miss. LLMs can analyze code within a dynamic context and identify complex security patterns. Techniques such as **RAG** and **fine-tuning (e.g., QLoRA)** have shown promising results in adapting LLMs to evolving smart contract security threats.

For instance, a practical pipeline for smart contract auditing may involve preprocessing smart contract code (e.g., removing comments, normalizing format) and tokenizing it using LLaMA's tokenizer. The preprocessed data is then indexed in FAISS (a fast similarity search library) for efficient retrieval during inference. Fine-tuning using QLoRA on high-quality datasets enables efficient vulnerability detection with low computational resources. Such approaches have demonstrated significant improvements in vulnerability detection compared to traditional static analysis tools.

#### Future Directions

While fine-tuning and code preprocessing have enhanced LLM-based vulnerability detection, future challenges remain. As larger and more powerful LLMs emerge, their inherent capabilities may surpass the gains obtained through preprocessing. Furthermore, existing LLMs struggle with complex cross-file vulnerabilities and dependency analysis. Addressing these challenges will require the development of advanced code representation methods, larger and more realistic repository-level datasets, and improved hybrid approaches that combine static, dynamic, and LLM-based analysis for more accurate vulnerability detection.

---

<div  align="center">
<img src="https://github.com/ETAAcademy/ETAAcademy-Images/blob/main/ETAAcademy-Audit/08_DA_DL.gif?raw=true" width="50%" />
</div>
