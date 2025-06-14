# ETAAcademy-Adudit: 15. Logical Models

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>15 Logical Models </td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>Logical Models</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# The Landscape of First-Order Logical Theories: Completeness, Classification, and Structural Characterization

The first-order logical theories presents a coherent and rich landscape that spans from questions of completeness and model classification to the precise characterization of mathematical structures. At the foundation lies the theory of completeness, where Gödel's completeness theorem bridges syntactic provability and semantic truth. This connection reveals deep interdependencies among quantifier elimination, substructure completeness, and model completeness—key notions that structure the behavior of logical theories.

Countable model theory refines this picture by introducing the concept of types, transforming the problem of model isomorphism into one of type realization and omission. Within this framework, Vaught's conjecture captures a trichotomy in the number of countable models a theory may possess, while the existence of saturated and highly homogeneous (ultrahomogeneous) models deepens our understanding of model-theoretic classification.

Algebraically closed fields and real closed fields serve as classical examples where algebraic structures and logical theories seamlessly converge. Through quantifier elimination and model completeness, these theories not only exhibit structural elegance but also possess robust logical properties. Notably, Tarski’s proof of the decidability of real closed fields remains a landmark achievement in the history of mathematical logic.

Theories of addition over rational and integer domains illustrate different levels of logical complexity. The theory of addition over the rationals exhibits strong minimality and complete axiomatizability, whereas the theory of addition over the integers requires extensions with modular congruences to achieve completeness. In contrast, the arithmetic of natural numbers reveals the fundamental limitations of formal systems, as demonstrated by Gödel’s incompleteness theorems. While simple order and additive theories of natural numbers may be complete, the inclusion of multiplication introduces undecidable and independent statements, such as the Paris–Harrington principle.

Together, these developments form the theoretical backbone of modern model theory. They not only illuminate the internal complexity of mathematical structures but also delineate the expressive boundaries of logical systems. The impact of these insights extends well beyond pure logic, influencing foundational studies, computability theory, and the philosophy of mathematics.

---

## 1. Completeness in First-Order Logic

When we construct a logical theory, we often hope that it is strong enough to determine the truth or falsity of all statements we care about. However, not all theories can achieve this. **Completeness** refers to the ability of a first-order theory $T$ to decide the truth or falsehood of every sentence in its language.

A first-order theory $T$ is **complete** if, for every sentence $\theta$ in the language $\mathcal{L}_A(T)$, either $T \vdash \theta$ or $T \vdash \neg \theta$. That is, the theory can formally prove either the sentence or its negation. Completeness is equivalent to the theory being a **maximal consistent set**—a theory to which no further consistent sentences can be added without causing inconsistency. It is also equivalent to all models of the theory being **elementarily equivalent**—they satisfy exactly the same first-order sentences.

To determine whether a theory is complete, the key question is whether there exist **independent statements**—sentences that are neither provable nor refutable within the theory. A sentence $\theta$ is **independent of** a theory $T$ if both extended theories $T \cup \{ \theta \}$ and $T \cup \{ \neg \theta \}$ are consistent (i.e., each has a model).

### Using Gödel’s Completeness Theorem

Gödel’s Completeness Theorem connects semantic truth and syntactic provability: if a sentence $\theta$ is a logical consequence of a theory $T$ (i.e., $T \models \theta$), then it is also syntactically provable (i.e., $T \vdash \theta$). This allows us to use **model-theoretic tools**—such as constructing specific models—to test completeness.

For instance, if we can construct a model $M \models T \cup \{ \neg \theta \}$, then $T \nvDash \theta$, and by the Completeness Theorem, $T \nvdash \theta$. Thus, the sentence $\theta$ is not provable from $T$, indicating that $T$ is incomplete.

This perspective shifts the task of proving or refuting statements away from purely symbolic deduction and toward **semantic reasoning**. We can analyze the structure of models to determine whether certain sentences are necessarily true, thereby gaining insight into the logical strength of the theory.

### Equivalent Characterizations of Completeness

Let $T$ be a consistent first-order theory. The following are equivalent:

- A sentence $\theta$ is independent of $T$ if and only if both $T \cup \{ \theta \}$ and $T \cup \{ \neg \theta \}$ are consistent (have models).
- $T$ is complete if and only if for every sentence $\theta$, $T \vdash \theta$ if and only if $M \models \theta$ for all $M \models T$.
- $T$ is complete if and only if all models of $T$ are **elementarily equivalent**—they satisfy the same first-order sentences.
- $T$ is complete if and only if there exists a model $M \models T$ such that for every sentence $\theta$, $\theta \in T$ if and only if $M \models \theta$.

### Completeness and Undecidability

In general, whether a theory $T$ is complete is **undecidable**, especially if the theory has the expressive power to encode arithmetic (such as Peano Arithmetic). However, for countable languages and countable models, completeness can sometimes be verified using **model-theoretic arguments**, such as showing that all models of a certain cardinality are isomorphic.

One method to test whether a sentence $\theta$ is independent of a theory $T$ is to attempt to construct models of both $T \cup \{ \theta \}$ and $T \cup \{ \neg \theta \}$. If both models exist, then $\theta$ is independent, and $T$ is incomplete.

#### Examples

**Incomplete First-Order Theories:**

- **Group Theory:** The commutativity axiom $\forall x \forall y (xy = yx)$ is independent of the basic axioms of group theory.
- **Theory of Ordered Fields:** The statement “ $\sqrt{2}$ exists ” is independent.
- **Theory of Algebraically Closed Fields:** The statement “The characteristic is 17” is independent unless characteristic is fixed.

**Complete First-Order Theories:**

- **DLO (Dense Linear Orders without Endpoints):** The structure ($\mathbb{Q}, <$)serves as a model of the theory of dense linear orders without endpoints (DLO), which is known to be complete.
- **$T_{dlo}$ (Discrete Linear Orders without Endpoints):** The theory modeled by $(\mathbb{Z}, <)$ is also complete.

These theories are also examples of **$\omega$-categorical** theories, meaning that all countable models of the theory are **isomorphic**. For more complex theories like algebraically closed fields or real closed fields, completeness can be obtained by **adding specific axioms** (e.g., fixing the characteristic or ordering properties).

If a theory is **$\kappa$-categorical**, then all its models of cardinality $\kappa$ are isomorphic. In particular, $\omega$-categoricity implies that the theory is complete, since all countable models satisfy the same sentences.

### Quantifier Elimination, Minimal Substructures, and Completeness in First-Order Theories

In model theory, certain semantic and syntactic properties of first-order theories exhibit deep interconnections. Among these, **quantifier elimination**, the existence of **minimal substructures**, and **completeness** form a powerful triad. This article explores how these properties relate, particularly demonstrating that:

> **Any consistent first-order theory that admits quantifier elimination and possesses minimal substructures must be a complete theory.**

#### Quantifier Elimination

A theory $T$ in a language $\mathcal{L}$ is said to **admit quantifier elimination** if for every formula $\varphi(x_1, \dots, x_n)$, there exists a quantifier-free formula $\psi(x_1, \dots, x_n)$ such that:

$T \models \varphi \leftrightarrow \psi$

This means that every formula is provably equivalent (in $T$) to a Boolean combination of atomic formulas, making the theory syntactically tractable and logically transparent.

### Minimal Substructures

A **minimal substructure** of a theory $T$ is a structure $M \models T$ such that $M$ can be embedded into every model of $T$. That is, $M$ is a substructure of any model of $T$. Not every consistent theory admits minimal substructures. For example, the theory of algebraically closed fields (ACF) lacks a unique minimal model. However, the theory of algebraically closed fields of characteristic zero does have minimal models, such as:

- The ring of integers $\mathbb{Z}$
- The field of rational numbers $\mathbb{Q}$
- The field of algebraic numbers $\overline{\mathbb{Q}}$

Another prominent example is the theory $\text{ToDL}$ of dense linear orders without endpoints. It admits quantifier elimination in a language with a single binary relation symbol $<$, and all its countable models are isomorphic to $(\mathbb{Q}, <)$ (by Cantor’s theorem), making it both complete and categorical in the countable case.

In contrast, the theory $Th(\mathbb{Q}, <)$, though consistent, does not admit quantifier elimination. For example, existential formulas such as "there exists a square root of 2" cannot be expressed as quantifier-free formulas in $\mathbb{Q}$.

### Substructure Completeness and Quantifier Elimination

A key semantic notion equivalent to quantifier elimination is **substructure completeness**. A theory $T$ is substructure complete if for every model $M \models T$ and every substructure $M_0 \subseteq M$, the theory $T \cup \Delta_{M_0}$ is complete, where $\Delta_{M_0}$ is the atomic diagram of $M_0$. In other words, $T \cup \Delta_{M_0}$ decides the truth value of every sentence.

Quantifier elimination is equivalent to substructure completeness and is further characterized by certain structural and amalgamation properties of models:

- **Amalgamation property (Diamond Lemma):** Given a structure $C$ and embeddings $e: C \to M$, $f: C \to N$, with $M, N \models T$, there exists a structure $D \models T$ and an embedding $g: N \to D$ such that $M \subseteq D$ and $e(a) = g(f(a))$ for all $a \in C$.

This forms a **diamond-shaped commuting diagram**:

```
        C
       / \
      /   \
     V     V
     M     N
      \   /
       \ /
        D
```

This diagram is the structural core of the amalgamation theorem, which is a fundamental property of theories admitting quantifier elimination.

Additionally, the following equivalent characterizations hold:

- **Quantifier Elimination**: Every formula is equivalent to a quantifier-free formula.
- **Substructure Completeness**: Every substructure determines the truth of sentences.
- **Diamond Lemma**: Amalgamation property for embeddings over a common substructure.
- **Isomorphism Extension Property**: Given isomorphic substructures $M_0 \cong N_0$ of $M \models T$, $N \models T$, there exist extensions $M^{\*}, N^{\*} \models T$ with $M^{\*} \cong N^{\*}$ extending the isomorphism.

If the language $\mathcal{L}$ is countable, the above equivalences also hold for countable models, since finitely generated substructures remain countable.

### From Quantifier Elimination to Completeness

If a consistent theory $T$ admits quantifier elimination and has a minimal model $M$, then $T$ is **complete**. The reasoning is as follows:

- Quantifier elimination implies that all models of $T$ are **elementarily equivalent**, as truth of formulas is determined by their quantifier-free equivalents.
- A minimal substructure $M$ embeds into every model of $T$, acting as a common core.
- Together, these properties ensure that all models of $T$ satisfy the same sentences, i.e., $T$ is complete.

This connection highlights the interplay between syntactic reducibility (quantifier elimination) and semantic universality (completeness via minimal substructure).

### Model Completeness and Quantifier Elimination

A theory $T$ is said to be **model complete** if whenever $M, N \models T$ and $M \subseteq N$, then $M \prec N$, i.e., $M$ is an elementary substructure of $N$. Quantifier elimination implies model completeness, but not vice versa.

Model completeness ensures:

- Logical preservation across embeddings
- Robust behavior under substructure formation
- A strong form of definability within models

If $T$ admits quantifier elimination and has a minimal model, its model completeness and universality of substructures imply that $T$ is complete.

### Quantifier Simplification

A weaker notion than full quantifier elimination is **quantifier simplification**, which only requires that each formula $\varphi$ has a pair of quantifier-free formulas $\theta$, $\psi$ such that:

$T \models \varphi \leftrightarrow \theta, \quad T \models \neg \varphi \leftrightarrow \psi$

This is sufficient for definitional clarity, but not strong enough to guarantee model completeness or full syntactic tractability.

---

## 2. Countable Models and Types

The concept of _types_ provides a powerful lens for analyzing questions about _countable models_ and their _isomorphism classes_ by transforming them into questions about whether certain types are _realized_ or _omitted_ in a model.

At the heart of this approach lies the distinction between **principal types**—which represent common structural features realized in every model—and **non-principal types**, which represent special features that can be selectively omitted. This flexibility in realization allows us to construct non-isomorphic countable models of a theory.

### How Many Countable Models Can a Theory Have?

Given a first-order theory $T$, if it has an infinite model, how many _non-isomorphic_ countable models can it have? Is it necessarily unique? If not, how can we abstractly construct multiple, non-isomorphic countable models?

The answer is deeply tied to the theory's _types_. Consider the following examples:

- The theory of **dense linear orders without endpoints** (such as the order on $\mathbb{Q}$) has **only one** countable model up to isomorphism.
- The theory of **algebraically closed fields of characteristic 0** has **countably infinitely many** countable models (distinguished by their transcendence degree).
- **Peano Arithmetic (PA)** has **$2^{\aleph_0}$** countable models, an uncountable number.

### Types and Their Realizations

A **type** is a maximally consistent set of formulas with free variables (e.g., $x_1, ..., x_n$) over a theory $T$. Intuitively, a type represents the complete set of properties that a hypothetical tuple of elements might satisfy. If such a tuple exists in a model $\mathcal{M}$ of $T$, we say that the type is _realized_ in $\mathcal{M}$; otherwise, it is _omitted_.

Types can be viewed as extending the theory $T$ by adding constraints about specific elements or tuples. For example, in the standard model of arithmetic, the type $\{x > n \mid n \in \mathbb{N}\}$ is _not_ realized, but it can be realized in non-standard models.

A model $\mathcal{M}$ _realizes_ a type $p(\bar{x})$ if there exists a tuple $\bar{a} \in \mathcal{M}$ such that $\mathcal{M} \models \varphi[\bar{a}]$ for all $\varphi \in p$. Otherwise, the model _omits_ the type.

A type is _consistent_ with a theory $T$ if it does not contradict $T$, and by compactness, a consistent type is realized in some model of $T$ if and only if every finite subset is satisfiable.

### Principal vs. Non-principal Types

A **principal type** is one that can be generated by a single formula $\varphi(\bar{x})$, called a **complete formula**, such that for every formula $\psi$, either $T \models \varphi \rightarrow \psi$ or $T \models \varphi \rightarrow \neg \psi$. Principal types are realized in _every_ model of the theory.

In contrast, a **non-principal type** is a consistent type not generated by any single formula. Such types may or may not be realized in a given model, and the ability to _omit_ them is crucial for constructing non-isomorphic models.

#### Omitting Types and Constructing Models

The **Omitting Types Theorem** provides a method for constructing countable models that omit certain non-principal types. Given a complete theory $T$ in a countable language and a non-principal type $p(\bar{x})$, there exists a _countable_ model $\mathcal{M}$ of $T$ that _omits_ $p$.

The proof typically involves the **Löwenheim–Skolem Theorem** and the **Compactness Theorem**. By carefully adding new constants and extending the theory in a consistent and recursive way, one can build a maximally consistent set of sentences (a Henkin theory) whose corresponding model avoids realizing the given non-principal type.

This leads to the construction of:

- A model $\mathcal{M}_1$ that realizes $p$
- A model $\mathcal{M}_2$ that omits $p$

Since these models treat the type $p$ differently, they cannot be isomorphic. Thus, the existence of non-principal types implies the existence of **non-isomorphic countable models** of $T$.

### Principal Types, Type Spaces, and Model Counts

The number of non-isomorphic countable models of a complete theory $T$ is closely tied to the number of non-principal types:

- If _all_ types of $T$ are principal and there are only finitely or countably many, then $T$ has only finitely or countably many countable models.
- If $T$ admits non-principal types, then it must have **multiple**, and possibly **uncountably many**, non-isomorphic countable models.

**Examples:**

- The theory of dense linear orders without endpoints (DLO) has only finitely many principal types and exactly **one** countable model (up to isomorphism).
- The theory of algebraically closed fields of characteristic 0 has **countably infinitely many** non-isomorphic countable models, distinguished by transcendence degree.
- Peano Arithmetic admits many non-principal types, resulting in **uncountably many** countable models—specifically $2^{\aleph_0}$ of them.

### Vaught’s Conjecture

One of the central open problems in model theory is **Vaught’s Conjecture**. It states:

> For any complete first-order theory $T$ in a countable language, if $T$ has infinitely many non-isomorphic countable models, then it has **exactly $2^{\aleph_0}$** such models.

In other words, the conjecture asserts that for countable theories, the number of countable models can only be:

- Finite
- Countably infinite
- **Exactly $2^{\aleph_0}$** (the cardinality of the continuum)

No countable theory should have an _uncountable_ but _less than continuum_ number of countable models.

**Examples supporting the conjecture:**

- **DLO**: has exactly one countable model ⇒ finite case
- **PA (Peano Arithmetic)**: has $2^{\aleph_0}$ countable models ⇒ uncountable case

Despite extensive study since its proposal in 1961, **Vaught’s Conjecture remains unproven** and without counterexamples. It is one of the most significant unresolved problems in model theory. The intuition behind it is striking: once the space of types becomes “complex” enough to admit multiple models, it becomes so rich that it admits a _continuum_ of them—there is no middle ground.

### Type Spaces and Stability in Model Theory

In model theory, the **type space** $S_n(T)$ reflects the complexity of a theory $T$. Roughly speaking, the more intricate or “rich” the type space, the more complex the classification of models of $T$ becomes. If the type space is **finite** or **zero-dimensional and discrete**, the theory typically admits **finitely many or a unique countable model**. Conversely, a type space that is **uncountable**, **non-discrete**, or **topologically connected** often corresponds to theories with **continuum-many ($2^{\aleph_0}$)** non-isomorphic models.

#### Types and Type Spaces

A **type** is a set of formulas that describes the possible behavior or properties of a tuple of elements from a model. Given a consistent first-order theory $T$ in a language $L$, the **type space** $S_n(T)$ consists of all **complete and consistent $n$-types** over $T$—that is, maximal sets of formulas with $n$ free variables that are jointly consistent with $T$.

This set can be equipped with a natural **topological structure**. For any formula $\varphi(x_1, \dots, x_n)$, the set

$[\varphi] = \{ p \in S_n(T) \mid \varphi \in p \}$

is defined to be a **basic open set**. These sets form a basis for a topology on $S_n(T)$, making it a **compact Hausdorff space**. This well-behaved topological structure allows for tools from topology to be employed in the analysis of types.

### Stability and Type Count

The **cardinality of $S_1(T)$**, the space of complete 1-types over $T$, is particularly significant. If $S_1(T)$ is uncountable, then $T$ is said to be **unstable over $\omega$** (i.e., it is **not $\omega$-stable**). Conversely, $T$ is called **$\omega$-stable** if for every countable model $M$ of $T$, the type space $S_1(Th(M))$ is countable.

More generally, $T$ is said to be **$\kappa$-stable** if for every model $M$ of $T$ with $|M| = \kappa$, the type space $S_n(Th(M))$ has cardinality at most $\kappa$ for each $n$. If $T$ is stable in some infinite cardinality, it is called a **stable theory**.

**Examples:**

- The theory of arithmetic $(\mathbb{N}, +, \times)$ is **not $\omega$-stable**, as it has uncountably many 1-types.
- The theory of dense linear orders without endpoints $(\mathbb{Q}, <)$ is also **not $\omega$-stable**.
- The theory of **algebraically closed fields** of a fixed characteristic **is $\omega$-stable**, with the cardinality of its type space matching the cardinality of the field itself.

### Types as Ultrafilters on Boolean Algebras

Types can also be understood algebraically: as **ultrafilters on Boolean algebras**. Consider the Boolean algebra $B$ of equivalence classes of formulas modulo $T$-provable equivalence:

$\varphi \sim_T \psi \iff T \models (\varphi \leftrightarrow \psi)$

Then each **complete type** corresponds to an **ultrafilter** on $B$: a maximal consistent set of such formula classes.

Topologically, this correspondence is precise: the type space $S_n(T)$ is **homeomorphic** to the space of ultrafilters on $B$. Thus, the logical structure of types and the topological-algebraic structure of ultrafilters are not only analogous but **topologically equivalent**.

### Saturated Structures

A **saturated model** is one that realizes all types over small subsets of itself. Saturation provides a notion of maximality in terms of type realization and plays a central role in the theory of model construction, uniqueness, and classification.

There are various strengths of saturation:

- **Weak saturation**: Every complete 1-type over a finite set is realized in the model.
- **$\omega$-saturation**: Every complete 1-type over a finite set (or countable subset) of the model is realized.
- **$\omega_1$-saturation**: Every type over a **countable** set of parameters is realized.
- **$|M|$-saturation** (full saturation): Every type over a parameter set of size less than $|M|$ is realized.

The hierarchy follows:

$\text{Saturated} \Rightarrow \omega_1\text{-saturated} \Rightarrow \omega\text{-saturated} \Rightarrow \text{Weakly saturated}$

**Examples:**

- $(\mathbb{Q}, <)$ is a **saturated structure**.
- $(\mathbb{R}, <)$ is **$\omega$-saturated** but **not $\omega_1$-saturated**.
- $\mathbb{C}$ (the complex field) is **saturated**.
- For algebraically closed fields, $\omega$-saturation implies algebraic closure, but not vice versa.

In fact, for a complete theory $T$ in a countable language, the existence of a **countable saturated model** is equivalent to the condition that **$S_n(T)$ is countable for all $n$**.

### Construction and Uniqueness of Saturated Models

A saturated model can be built as the **union of an elementary chain** $M = \bigcup_n M_n$ where each $M_n$ realizes increasingly many types, constructed via **type extension**, **homogeneous chains**, and the **compactness theorem**, aided by **Tarski’s union theorem**.

Saturated models in a given cardinality are often **unique up to isomorphism**. If two countable models $M$ and $N$ are both **saturated** and **elementarily equivalent**, then they are **isomorphic**. This generalizes Cantor’s theorem on the uniqueness of countable dense linear orders, and the **back-and-forth method** provides a constructive proof of such isomorphisms.

For instance, let $T = Th(\mathbb{Q}, <, a_1, \dots, a_m)$, where $a_i$ are constant symbols for elements in $\mathbb{Q}$. Then the type space $S_1(T)$ has exactly $2m + 1$ complete types, each corresponding to a possible "position" relative to the named points. All such types are realized in $\mathbb{Q}$, highlighting the **realizability** and **saturation** of the model.

### Prime Models, Prime Types, and the Role of the Prime Model Uniqueness Theorem

In model theory, the **prime model** of a complete theory plays a foundational role in understanding the landscape of models. A **prime model** $\mathcal{M}$ of a complete theory $T$ is a minimal model into which every other model of $T$ admits an elementary embedding. One of the central results concerning such models is the **Prime Model Uniqueness Theorem**, which asserts that if $T$ has a prime model, then **any two prime models of $T$ are isomorphic**. Thus, when a prime model exists, it is **unique up to isomorphism** among all $T$-models.

But this raises a natural question: if the **prime type** (or **complete type**) that characterizes the theory is already unique, why do we still need a theorem to ensure the uniqueness of the prime model?

To address this, it is crucial to distinguish between **syntax** and **semantics**:

- A **prime type** $\Delta$ is a syntactic object: a complete and consistent set of formulas over some variables that can be realized in a model. It is a type that is **maximal consistent** and often seen as a “template” that defines the core behavior of some elements within models.
- A **model**, by contrast, is a semantic object: it includes a domain, interpretation functions, and actual element assignments.

Although a prime type is unique, **multiple models can realize this type in different ways**, depending on how Skolem functions are applied, how constants are interpreted, and how the domain is constructed. These models might differ in their structure even though they all realize the same prime type. Therefore, **the uniqueness of the prime type does not imply the uniqueness of the model** realizing it.

This is precisely the gap that the **Prime Model Uniqueness Theorem** fills: it ensures that **all models constructed to realize the prime type in a minimal way (i.e., all prime models) are isomorphic**. It bridges the gap between syntactic uniqueness (the prime type) and semantic uniqueness (the structure of models).

### Highly Homogeneous Models and Extreme Automorphism Groups

Beyond rigidity and minimality, another intriguing concept in model theory is **extreme symmetry**—models whose automorphism groups are vast. A particularly striking case is when a model is **highly homogeneous**, or **highly (ultra-)homogeneous**, meaning its automorphism group has the cardinality of the continuum.

For any consistent theory $T$ in a countable language that only has infinite models, it is possible to construct a **countable model of $T$** that is **extremely symmetric**, i.e., has a continuum-sized automorphism group. This construction proceeds through the careful identification of **indiscernible sets** and **Skolem closure**.

#### Step 1: Constructing an Indiscernible Set (Under Order)

We begin by constructing a countable set of elements $X = \{a_i : i \in I\}$, indexed by a countable linear order $(I, <)$, such that the sequence is **order-indiscernible**: for any increasing tuples $i_1 < \dots < i_m$ and $j_1 < \dots < j_m$, the corresponding tuples $(a_{i_1}, \dots, a_{i_m})$ and $(a_{j_1}, \dots, a_{j_m})$ satisfy exactly the same formulas in the language $L$.

To achieve this:

- Extend the language $L$ by adding a constant symbol $c_i$ for each $i \in I$.
- Define a theory $T^{\*}$ consisting of:

  - All sentences of $T$;
  - The inequalities $c_i \neq c_j$ for $i \neq j$, ensuring distinctness;
  - Axioms that enforce indiscernibility: for any two increasing sequences $i_1 < \dots < i_m$, $j_1 < \dots < j_m$, the tuples $(c_{i_1}, \dots, c_{i_m})$ and $(c_{j_1}, \dots, c_{j_m})$ satisfy the same formulas.

Using the **Compactness Theorem** and **Ramsey's Theorem**, we can prove that $T^{\*}$ is consistent. Thus, it has a model $\mathcal{M}^{\*}$ in which the constants $\{c_i\}$ interpret an indiscernible sequence as desired.

#### Step 2: Taking the Skolem Closure

Next, we move from this partially symmetric set to a fully symmetric model:

- Extend the language with **Skolem functions**, turning $T$ into a **Skolemized theory**, where every definable function is named.
- Take the **Skolem closure** $H(X)$ of the indiscernible set $X$, i.e., the smallest set containing $X$ and closed under all Skolem functions.

This closure ensures that every element of the model is **generated** from the indiscernible sequence $X$. Now, we leverage the high symmetry of $X$.

#### Step 3: Exploiting Order-Automorphisms of $(\mathbb{Q}, <)$

If we take the index set $I \cong (\mathbb{Q}, <)$, which is well known to be **extremely homogeneous** (every order-isomorphism between finite subsets extends to an automorphism), then any **order automorphism** $t: X \to X$ can be extended to an **automorphism of the entire Skolem closure** $H(X)$. This is possible because the closure only adds elements as outputs of Skolem functions applied to tuples from $X$, and automorphisms of $X$ permute these tuples in definable ways.

Since the automorphism group of $(\mathbb{Q},<)$ has cardinality $2^{\aleph_0}$, so does the automorphism group of the constructed model. Therefore, the resulting model is:

- **Countable**, because $X$ and the Skolem closure of a countable set are countable.
- **A model of $T$**, because it satisfies the original theory.
- **Extremely symmetric**, with automorphism group of size continuum.

#### Rigidity vs. Symmetry

This construction reveals the contrast between different kinds of models:

- A **rigid model** has no nontrivial automorphisms. A canonical example is $(\mathbb{N}, <)$, where the ordering pins down each element uniquely.
- A **non-rigid** model, such as $(\mathbb{Q}, <)$, admits many automorphisms.
- A **highly symmetric** (or extremely homogeneous) model is one whose automorphism group reaches the cardinality of the continuum, maximizing internal symmetry.

---

## 3. Theories of Algebraically Closed and Real Closed Fields

### 3.1 The Theory of Algebraically Closed Fields (ACF)

The **theory of algebraically closed fields**, commonly denoted **ACF**, is a foundational and classical example in model theory. It plays a central role due to its favorable model-theoretic properties: **completeness**, **quantifier elimination**, **model-completeness**, and the ability to classify uncountable models by their cardinality. Moreover, ACF serves as a deep connection point between model theory, algebraic geometry, and number theory.

**Key Variants**

- **ACF₀**: The theory of algebraically closed fields of **characteristic 0** (e.g., the field of complex numbers ℂ).
- **ACFₚ**: The theory of algebraically closed fields of **characteristic p** (e.g., the algebraic closure $\overline{\mathbb{F}_p}$ of the finite field $\mathbb{F}_p$).

These theories are not only central to pure logic but also deeply connected with **algebraic geometry**. For example:

- Definable sets in ACF correspond to **algebraic varieties**.
- Logical **types** correspond to **algebraic extensions** of points (i.e., closed points in schemes).
- The **Zariski topology** and **constructible sets** provide the geometric context for model-theoretic definability.
- **Chevalley’s theorem** functions as a geometric analogue of **quantifier elimination**, linking algebraic images to definable projections.

#### Logical Language and Axiomatization

- **Language**: $L_{\text{ring}} = {0, 1, +, -, \cdot}$ — the standard language of rings.
- **Axioms**:

  - **Field Axioms**: The theory includes axioms ensuring that the structure is a field.
  - **Algebraic Closure**: Every non-constant univariate polynomial has a root (expressed in first-order logic).
  - **Characteristic**:

    - For **characteristic 0**: Include the axioms
      $\forall n \in \mathbb{N}^+,\ \underbrace{1 + \cdots + 1}_{n\text{ times}} \neq 0$
    - For **characteristic p**:

    $\underset{p \text{ times}}{\underbrace{1 + \cdots + 1}} = 0\quad \text{and} \quad \forall 1 \leq n < p,\ \underset{n\text{ times}}{\underbrace{1 + \cdots + 1}} \neq 0$

#### Model-Theoretic Properties of ACFₚ and ACF₀

- **Completeness**: Each ACFₚ and ACF₀ is a **complete theory**—every sentence in the language is either provable or its negation is.
- **Categoricity in Countable Models**:

  - ACF₀ has a unique (up to isomorphism) countable model: $\overline{\mathbb{Q}}$.
  - ACFₚ has a unique countable model: $\overline{\mathbb{F}_p}$.
  - These models are **atomic** and often **prime**.

- **ω-Stability**: ACFₚ and ACF₀ are **ω-stable theories**. This means that for any countable parameter set, there are only countably many complete types, giving the theory a well-behaved classification.
- **Quantifier Elimination**: ACFₚ and ACF₀ eliminate quantifiers, meaning any formula is equivalent to a quantifier-free formula. This aligns their definable sets closely with **constructible sets** in algebraic geometry.
- **Model Completeness**: Every embedding between models of ACF is elementary (preserves all first-order properties).

#### On the Importance of Specifying Characteristic

While each **ACFₚ** or **ACF₀** is complete, the general theory **ACF** (without fixing the characteristic) is **not complete**. This is because certain first-order statements (e.g., involving the characteristic) differ in truth value depending on whether the model has characteristic 0 or some prime p. Hence, without specifying the characteristic, ACF cannot determine the truth of all first-order sentences.

#### Substructure Completeness and Quantifier Elimination

A proof sketch for quantifier elimination in ACFₚ often involves **substructure completeness**: Given two models $M_0 \subseteq M_1$, both models of ACFₚ, and a shared subfield $F$, one constructs an isomorphism between their definable sets over $F$. This field-theoretic approach to model-theoretic properties mirrors **Tarski’s method** in proving quantifier elimination for real closed fields, emphasizing algebraic structure as the foundation for logical properties.

#### Classification of Countable Models by Transcendence Degree

A key model-theoretic result is that in ACFₚ, a countable model $M$ is **ω-saturated** if and only if it has **countably infinite transcendence degree** over its prime subfield. This means:

- If $M$ contains a countable transcendence basis, it realizes every type over countable parameter sets ⇒ ω-saturation.
- If the transcendence degree is finite, then some 1-types cannot be realized.

This gives rise to a precise classification: all countable models of ACFₚ fall into unique isomorphism types indexed by their transcendence degree $0, 1, 2, \dots, \aleph_0$. The same holds for uncountable models as well (e.g., ℂ is ℵ₁-saturated).

#### Lefschetz Principle and Ax’s Theorem

A remarkable bridge between characteristic 0 and characteristic p models is the **Lefschetz Principle** in its first-order form:

> A first-order sentence φ holds in the complex numbers ℂ **if and only if** φ is a consequence of the axioms of ACF₀ **if and only if** φ holds in **ACFₚ for almost all sufficiently large primes p**.

This logical **transfer principle** underlies many surprising correspondences between number theory over finite fields and geometry over ℂ.

One famous application is **Ax’s Theorem**:

> Let $f: \mathbb{C}^n \to \mathbb{C}^n$ be a polynomial map. If $f$ is injective, then it is also surjective, hence a bijection.

This surprising result, non-obvious in complex geometry, is derived by combining the Lefschetz principle with the elementary fact that over finite fields, **injective polynomial maps are automatically surjective** (due to finite cardinality).

---

### 3.2 Theory of Real Closed Fields

The **theory of real closed fields (RCF)** arises from the classical problem of extending number systems, offering a precise first-order logical characterization of the real number field. It defines _formally real fields_, _real closed fields_, and _ordered real closed fields_, building deep connections among logic, algebra, and order theory. The central question is: **How can we capture the structure of the real numbers using first-order language?** Especially in relation to the existence of roots of polynomials and the behavior of linear orders. Beginning with the first-order language of fields, we can axiomatize abstract versions of the real numbers—**real closed fields**—by adding a finite number of well-chosen axioms.

#### Formally Real Fields

A field $F$ is called **formally real** if $-1$ is not a sum of finitely many squares in $F$. This can be expressed in first-order logic as:

$\forall x_1, \ldots, x_n \; \neg(x_1^2 + \cdots + x_n^2 + 1 = 0)$

Examples include the rational numbers $\mathbb{Q}$ and the real numbers $\mathbb{R}$, which are formally real. In contrast, the complex numbers $\mathbb{C}$ are not formally real, since $i^2 = -1$. This definition captures a key feature of "realness": the impossibility of expressing negative unity as a sum of squares.

#### Real Closed Fields

A field is **real closed** if it is formally real and **cannot be extended further while remaining formally real**—it is a _maximal_ formally real field with respect to algebraic extension. Equivalently, a field $F$ is real closed if:

- Every polynomial of odd degree has at least one root in $F$,
- For every $a \in F$, either $a$ or $-a$ has a square root in $F$.

These conditions ensure that $F$ behaves like the real numbers in terms of root existence and sign structure. Moreover, the complexification $F(i)$, where $i^2 = -1$, becomes algebraically closed, just like $\mathbb{C}$, if $F$ is real closed.

#### Orderable Fields and the Connection to Logic

A field $F$ is **orderable** if there exists a linear order $<$ on $F$ compatible with both addition and multiplication. The complex numbers $\mathbb{C}$ cannot be ordered this way, but $\mathbb{Q}$ and $\mathbb{R}$ can, and each admits only one such order. In contrast, $\mathbb{Q}(X)$ (the field of rational functions) is orderable in uncountably many distinct ways.

Crucially, a field is orderable if and only if it is **formally real**. This establishes a profound connection between algebraic and order-theoretic structures. A real closed field becomes **uniquely determined** up to isomorphism once a linear order is fixed. Indeed, for an **ordered field**, being real closed is equivalent to satisfying the _intermediate value property_:

$a < b, \; p(a) < 0 < p(b) \Rightarrow \exists x \in (a, b) \text{ such that } p(x) = 0$

This condition captures the intuition behind the "real root theorem" and underlies why $\mathbb{R}$ is considered the _minimal complete ordered field_.

#### Logical Characterization of the Real Numbers

Can the real numbers be completely characterized using first-order logic? The answer is subtle: while **completeness** (in the Dedekind or Cauchy sense) is inherently a second-order property, **the algebraic and order structure** of the reals can be captured via first-order axioms. The **first-order theory of real closed fields (RCF)** provides a complete, decidable, and axiomatizable description of the algebraic and order-theoretic behavior of $\mathbb{R}$, **excluding completeness**.

Thus, we can say:

$\text{Real Numbers} \approx \text{Ordered Real Closed Field} + \text{Completeness (Second-order)}$

So while RCF does not define the _complete_ real number system, it defines its **algebraic and order-theoretic essence**.

#### Logical Properties of RCF

- **Completeness**: All real closed fields satisfy the same first-order sentences. That is, **RCF is a complete theory**: for any sentence in the language of ordered fields, it is either true in all real closed fields or false in all.
- **Decidability**: RCF is **decidable**. This landmark result was proved by Alfred Tarski in the 1930s. Because RCF is both complete and recursively axiomatizable, there exists an algorithm to determine whether any first-order sentence is a theorem of RCF.
- **Recursive Axiomatizability**: Although RCF involves infinitely many polynomial conditions (e.g., “every odd-degree polynomial has a root”), these can be **effectively enumerated**, so the entire theory is **recursively enumerable** within a countable first-order language.
- **O-Minimality**: The models of RCF are **o-minimal**: every definable subset of the real line is a finite union of points and open intervals. This explains why **semi-algebraic geometry** (the geometry of polynomial inequalities) has such good structural properties (e.g., finiteness of connected components, dimension theory).
- **Model-Theoretic Applications**: The **compactness** and **completeness theorems** of first-order logic apply fully to RCF, enabling the construction of nonstandard models (e.g., real closed fields with infinitesimals) and logical deduction of algebraic properties.
- **Foundations of Real Algebraic Geometry**: RCF forms the **logical foundation of real algebraic geometry**, where solution sets of systems of polynomial inequalities—known as **semi-algebraic sets**—inherit desirable geometric properties, tightly linked to model-theoretic behavior.

---

## 4. First-Order Theories of Rational and Integer Addition

### 4.1 Theory of Rational Addition Arithmetic

The **Theory of Rational Addition Arithmetic** refers to the first-order logical theory satisfied by the structure $(\mathbb{Q}, 0, +, -)$, i.e., the field of rational numbers under addition and subtraction. This theory is known in model theory as **$T_{dag}$**, which captures the properties of **nontrivial, divisible abelian groups of characteristic zero**. It plays a foundational role in the logical analysis of additive number structures.

#### Axiomatization of $(\mathbb{Q}, 0, +, -)$

The structure $(\mathbb{Q}, 0, +, -)$ forms a **divisible abelian group of characteristic zero**, and its behavior is completely captured by a set of first-order axioms, which include:

- **Group Axioms** (denoted $\mathrm{Tag}$):

  - **Associativity**:
    $\forall x_1, x_2, x_3.\ F_+(F_+(x_1, x_2), x_3) = F_+(x_1, F_+(x_2, x_3))$

  - **Additive Identity**:
    $\forall x.\ F_+(x, 0) = x$

  - **Additive Inverse** (defined via subtraction function $F_-$):
    $\forall x, y.\ y = F_-(x) \leftrightarrow F_+(x, y) = 0$

  - **Commutativity**:
    $\forall x, y.\ F_+(x, y) = F_+(y, x)$

- **Characteristic Zero Axiom**:
  For all natural numbers $n > 1$,
  $\forall x.\ \underbrace{x + x + \cdots + x}_{n\ \text{times}} = 0 \Rightarrow x = 0$
  This ensures there are no nonzero torsion elements in the group.

- **Divisibility Axiom**:
  For each $n > 1 \in \mathbb{N}$,
  $\forall x.\ \exists y.\ ny = x$
  This ensures every element is divisible by any positive integer.

- **Nontriviality Axiom**:
  $\exists x.\ x \neq 0$
  To rule out the trivial group containing only the zero element.

#### Quantifier Elimination

Using standard model-theoretic techniques, such as the construction of prime models and $\Sigma_1$-elementary substructures, it can be shown that $T_{dag}$ admits **quantifier elimination**. This means every formula is equivalent (in $T_{dag}$) to a quantifier-free formula, allowing for a complete and computable understanding of definable sets.

#### Embeddings and Divisible Closure

Given any additive group $G$ of characteristic 0, one can construct an extension $H$ that is a divisible group (i.e., closed under division by all $n \in \mathbb{N}$). This **divisible closure** $H$ contains $G$ and satisfies the axioms of $T_{dag}$. In the categorical sense, $H$ is a minimal such extension with a universal (initial) property: it is the **free divisible abelian group** containing $G$.

#### Completeness and Model Theory

- **Model-Completeness**:
  The theory $T_{dag}$ is **model-complete**, meaning any embedding between models of $T_{dag}$ is elementary.

- **Completeness**:
  $T_{dag}$ is a **complete theory**: for any sentence $\varphi$ in its language, either $\varphi$ or $\neg \varphi$ is provable from $T_{dag}$. In other words, all models of $T_{dag}$ are elementarily equivalent.

- **Prime Model**:
  The structure $(\mathbb{Q}, 0, +, -)$ is the **prime model** of $T_{dag}$. Every other model of $T_{dag}$ ( e.g., $(\mathbb{R}, 0, +, -)$ ) contains a copy of it, but may not be isomorphic to it.

#### Strong Minimality

A deep structural property of $(\mathbb{Q}, 0, +, -)$ is that it is **strongly minimal**: any definable subset of the domain (with parameters) is either finite or cofinite. This follows from quantifier elimination and the fact that definable sets are essentially boolean combinations of linear equations, which have extremely low descriptive complexity in one variable.

#### Extended Theory $T_{dag}^{\*}$

To avoid degenerate models (like the trivial group), one can extend the language by adding a constant symbol $c_1$ and include an axiom such as $c_0 \neq c_1$ (where $c_0$ denotes 0), ensuring the model contains at least two distinct elements. This enriched theory, denoted **$T_{dag}^{\*}$**, better captures real-world structures like $(\mathbb{Q}, 0, 1, +, -)$ and ensures that the nontriviality of the group is explicitly guaranteed by the language.

---

### 4.2 The First-Order Theory of Integer Addition

The **theory of integer addition** in model theory focuses on the logical properties and models of the structure of integers when restricted to **languages involving only addition and its related symbols**. This branch examines what properties of the integers ℤ can be expressed using only addition (sometimes with negation, constants like 0 and 1, or even ordering `<`), and whether such structures admit **completeness**, **quantifier elimination**, or a **decidable and axiomatizable theory**.

A central result is that in the base language without the constant “1”, the theory of integers is **not suitable for quantifier elimination**. However, when “1” is added to the language—making it a so-called **level II₂ theory**—the resulting theory becomes much better behaved: systems such as **T\*** and **$T_{omrag}$** (the mod-congruence and ordered mod-congruence theories) **admit quantifier elimination**. With the help of **modulo-n congruence predicates**, one can formulate a **complete and model-complete axiomatization**. Interestingly, the **order relation `<` cannot be defined** in these structures purely using addition and mod predicates.

#### Strong Characteristic 0 Additive Groups

Consider the structure **(ℤ; 0, 1, +, −)**, also known as the **strong characteristic 0 additive group**. By introducing **predicates $P_n(x)$** for “n divides x,” one obtains the **mod-congruence theory**, denoted **$T_{mrag}$**. This theory has:

- A **complete**, **decidable**, and **quantifier-eliminable** axiomatization.
- A **minimal (prime) model**, which is ℤ itself.
- An **inexpressible order relation** `<`, demonstrated by showing that its presence fails to be preserved under isomorphisms to conjugate structures (e.g., ℤ\[√−1]).

When the order relation `<` is explicitly added, forming the structure **(ℤ; 0, 1, +, −, <)**, we get the **ordered mod-congruence theory** (**$T_{omrag}$**), which similarly admits a clean axiomatization with quantifier elimination.

#### The Three Fundamental Structures of Integer Addition

- **(ℤ; 0, +, −)**:
  The most basic additive group of characteristic 0.

  - Cannot define “1” or distinguish positive from negative integers.
  - **Does not admit quantifier elimination**.

- **(ℤ; 0, +, −, <)**:
  The ordered additive group.

  - “1” becomes definable, and positivity/negativity can be distinguished.
  - The complete theory **T = Th(ℤ; 0, +, −, <)**, but still **no quantifier elimination**.

- **(ℤ; 0, 1, +, −)** and **(ℤ; 0, 1, +, −, <)**:
  With the constant “1” explicitly included:

  - These are the **strong characteristic 0** and **ordered strong characteristic 0** additive groups, respectively.
  - Their theories **T\*** and its ordered version become quantifier eliminable and complete.
  - Moreover, the constant “0” can be removed without affecting essential properties, so the structures **(ℤ; 1, +, −)** and **(ℤ; 1, +, −, <)** are often studied.

**Five Key Theories and Their Characteristics**

| Name    | Language                                  | Structure                | Characteristics                                             |
| ------- | ----------------------------------------- | ------------------------ | ----------------------------------------------------------- |
| **T**   | {0, +, −, <}                              | Ordered ℤ-group          | Complete theory, **not** quantifier-eliminable              |
| **T\*** | {0, 1, +, −, <}                           | Ordered ℤ-group          | Complete and **quantifier-eliminable**                      |
| **T₀**  | {0, +, −, <, +congruence axioms}          | Arithmetic formalization | Complete and quantifier-eliminable, but more complex        |
| **T₀\***  | T₀ plus the constant 1                    | —                        | A **simpler level II₂** theory with quantifier elimination  |
| **Tₚᵣ** | Presburger arithmetic for (ℤ; 1, +, −, <) | —                        | Equivalent to the above; complete and quantifier-eliminable |

#### Tr: A Basic but Incomplete Theory

The **elementary theory Tr** includes axioms for:

- Addition (associativity, commutativity)
- Identity element (0)
- Inverses (−)
- Order (<)

The structure **(ℤ, 1, +, −, <)** is a **standard model** of Tr. However, **Tr is incomplete**—it cannot decide certain natural statements. For example, whether the equation $3x + 1 = (2,4)$ has a solution is undecidable within Tr, although it can be answered in the standard model.

Thus, **Tr is not suitable for quantifier elimination** and remains incomplete. To overcome this, **Presburger arithmetic Tₚᵣ** extends Tr by including an infinite collection of axioms that encode **modular arithmetic**, such as:

$\forall x\; \exists q, r \;(0 \leq r < m \land x = mq + r)$

These axioms ensure every integer can be expressed uniquely modulo _m_, giving a **full description** of ℤ in modular terms.

#### From Presburger to Mod-Congruence Theory

Despite this enrichment, **Tₚᵣ still does not fully support quantifier elimination**. For instance, some formulas such as:

$\forall x_1\; \neg(x_1 = \sigma_3(x_0))$

can be true in one model of Tₚᵣ and false in another, indicating **incompleteness**.

To fix this, a more powerful theory **T** is defined:

- Starting from **Tₚᵣ**, it **adds infinitely many new predicates** $P_{2^m}(x, y)$ to express congruence modulo $2^m$:

  $P_{2^m}(x, y) \leftrightarrow \exists k\; x = y + k \cdot 2^m$

- These predicates come with **defining axioms**, and the extended language is denoted **$\mathcal{L}_{A^{\*}}$**.

This extension is a **conservative one**—it doesn’t introduce any new theorems in the original language—and it **enables full quantifier elimination**, making the theory both **complete** and **decidable**.

---

## 5. Theories of Natural Number Order, Ordered Addition, and Arithmetic

### 5.1 Theory of Natural Number Order and Ordered Addition

The structure of natural numbers under order and addition can be precisely captured by **complete first-order theories**, and in some formulations, these theories even **admit quantifier elimination**. This stands in stark contrast to the case where both addition and multiplication are present—such combinations fall under the scope of Gödel’s incompleteness theorem and cannot form a complete theory.

If $X \subset \mathbb{N}$ is an **infinite proper subset** of $\mathbb{N}$, then the structure $(X, <) \cong (\mathbb{N}, <)$, i.e., it is order-isomorphic to the natural numbers. However, such isomorphic substructures are not **elementary substructures** of $(\mathbb{N}, <)$, and therefore do not support **quantifier elimination**.

To analyze the limitations of quantifier simplification in such structures, we examine two critical $\Pi_3$-level statements:

- $\theta_0$: Every element has a least greater element (discreteness).
- $\theta_1$: Every non-minimal element has a greatest smaller element (backward discreteness).

These statements, being $\Pi_3$ in the arithmetical hierarchy, cannot be equivalently rewritten as $\Pi_2$ formulas, demonstrating that the theory of $(\mathbb{N}, <)$ does **not admit quantifier elimination**.

#### The Theory $T_{\text{idlo}}$

The theory $T_{\text{idlo}}$ axiomatizes $(\mathbb{N}, <)$ as follows:

- $<$ is a strict linear order (irreflexive, transitive, total).
- There exists a **least element**.
- Every element has a **successor** (smallest element greater than it).
- Every non-minimal element has a **predecessor**.

Though this theory captures the structure of $(\mathbb{N}, <)$ completely, its quantifier complexity lies at the $\Pi_3$ level, making it unsuitable for quantifier elimination.

#### The Refined Theory $T_{\text{dlo}}$

To address the complexity, one can **extend the language** by introducing:

- A constant symbol $c_0$ for the minimal element $0$,
- A unary function symbol $F_s$ for the **successor function** $S(n) = n+1$,
- A binary predicate symbol $P_<$ for the order relation.

This gives rise to a new language $\mathscr{L}\mathscr{A}$ and an extended theory $T_{dlo}$, whose axioms include:

- $<$ is a linear order.
- $c_0$ is the least element.
- $F_s(x) = x + 1$.
- For all $x$, $x < F_s(x)$, and $F_s(x)$ is the immediate successor of $x$.
- Every element other than $c_0$ has a predecessor.

All non-logical axioms in $T_{\text{dlo}}$ are of **$\Pi_1$** or **$\Pi_2$** form, significantly simplifying the logical complexity compared to $T_{\text{idlo}}$.

Moreover, $T_{\text{dlo}}$ is a **conservative extension** of $T_{\text{idlo}}$: for any formula $\varphi$ in the language of $T_{\text{idlo}}$, if $T_{\text{dlo}} \vdash \varphi$, then $T_{\text{idlo}} \vdash \varphi$. This means the extension does not introduce new theorems in the original language—it merely simplifies the representation of existing truths.

**Proof Strategy**:

- Use the completeness theorem to argue that $\varphi$ holds in all models of $T_{\text{dlo}}$.
- From any model $M$ of $T_{\text{idlo}}$, identify a unique minimal element $c_0$ satisfying a defining formula $\phi_0$, and define a successor function using $\phi_s$.
- This allows the construction of a model $M^{\*}* = (M, <, S^{\*}, c_0)$ satisfying $T_{\text{dlo}}$, thereby ensuring $\varphi$ is true in $T_{\text{idlo}}$.

---

### The Theory $T_{\text{oasg}}$: Natural Numbers with Ordered Addition

The structure $(\mathbb{N}, 0, S, +, <)$—the **standard model of natural numbers with successor, addition, and order**—is the central model of the theory $T_{\text{oasg}}$ (the theory of ordered additive semigroups).

This structure satisfies:

- $0$ is the additive identity.
- $S(n) = n + 1$.
- $+$ is associative and commutative.
- $<$ is the standard order, compatible with addition and successor.

However, **$T_{\text{oasg}}$ is neither complete nor model complete**. For example, one can construct nonstandard models like $N_1 = \mathbb{N} \cup (\mathbb{Q} \times \mathbb{Z})$, which also satisfy $T_{\text{oasg}}$, illustrating that $T_{\text{oasg}}$ admits models not elementarily equivalent to $\mathbb{N}$.

#### Addressing Incompleteness: Modular Congruence Axioms

To resolve this incompleteness, we introduce a **family of modular congruence axioms** $\theta_m$ for each integer $m > 1$. These axioms capture modular behavior by asserting that every natural number has a unique residue class modulo $m$.

Introduce **unary predicates** $P_3^m(x)$ to denote "divisible by $m$", i.e., $x \in m\mathbb{N}$, and define:

- $T_{\text{oasg}}^{\*} = T_{\text{oasg}} \cup \{ \theta_m \mid m > 1 \}$

This augmented theory aims to eliminate “pseudo-natural” structures that satisfy the basic axioms but violate modular properties—thus narrowing the class of models to those more closely resembling $\mathbb{N}$.

#### The Extended Theory $T_{\text{omrasg}}$

To further develop the theory, we define $T_{\text{omrasg}}$ (the theory of **ordered modular congruence additive semigroups**) by:

- Extending the language $\mathcal{L}_{A^\le}$ to include **all** predicates $P_3^n$ for each $n > 1$,
- Asserting axioms for each such predicate corresponding to modular behavior.

This theory is a **conservative extension** of $T_{\text{oasg}}^{\*}$, maintaining all truths of the original theory while enabling **quantifier elimination** and **model completeness** through the richer language.

To accommodate broader model classes, **weakened versions** of $T_{\text{omrasg}}$ have been proposed by relaxing congruence definitions. These allow for generalized models including $\mathbb{Q} \times \mathbb{Z}$, $\mathbb{Z} \times \mathbb{Z}$, and other hybrid structures, striking a balance between expressive power and model-theoretic properties.

---

### 5.2 Arithmetic Theories of the Natural Numbers

The arithmetic of the natural numbers centers on **Gödel's incompleteness theorems**, which reveal the inherent limitations of formal systems capable of expressing elementary arithmetic. These limitations are illustrated through the construction of **nonstandard models**, such as the positive polynomial ring $\mathbb{Z}[X]^+$, and the examination of formal systems like **Peano Arithmetic ($T_{PA}$)** and **Elementary Arithmetic ($T_n$)**.

#### Gödel's Incompleteness Theorems and Peano Arithmetic

Peano Arithmetic ($T_{PA}$) is a formal system rich enough to express basic arithmetic operations and their properties, including induction. Gödel’s **First Incompleteness Theorem** establishes that $T_{PA}$ is **incomplete**: there exist true statements about the natural numbers that cannot be proved within the system. His **Second Incompleteness Theorem** shows that $T_{PA}$ **cannot prove its own consistency**—any such proof would require stronger assumptions than the system itself can provide.

A striking example of the limits of $T_{PA}$ is the **Paris-Harrington Principle**, a combinatorial statement independent of $T_{PA}$. It can neither be proven nor refuted within the system, yet it is true in the standard model of arithmetic. This demonstrates that even statements formulated in seemingly simple terms can lie beyond the provability of powerful formal systems like $T_{PA}$.

#### The Incompleteness of Elementary Arithmetic ($T_n$)

Elementary arithmetic, denoted $T_n$, is a weaker system than $T_{PA}$, omitting full induction. This makes $T_n$ even more susceptible to incompleteness. For instance, consider the $\Pi_2$ sentence $\theta$ defined as:

$\theta \equiv \forall x_1 \exists x_2 \left( \neg(x_1 < x_2) \land \left( x_1 = x_2 + x_2 \lor x_1 = S(x_2 + x_2) \right) \right)$

Intuitively, $\theta$ expresses the statement: **"Every natural number is either even or the successor of an even number"**, i.e., every number is either of the form $2n$ or $2n+1$.

While $\theta$ is **true** in the standard model of natural numbers $(\mathbb{N}, 0, S, +, \times, <)$, it is **not derivable** from $T_n$. Its provability in **Peano Arithmetic** can be shown by defining a formula $\varphi(x)$ stating the existence of an appropriate $x_2$ for each $x$, proving $\varphi(0)$ and $\varphi(x) \rightarrow \varphi(S(x))$ within $T_n$, and then applying mathematical induction (available in $T_{PA}$) to conclude $\forall x, \varphi(x)$, i.e., $\theta$.

Thus:

$(\mathbb{N},0,S,+,\times,<) \models \theta \quad \text{and} \quad T_{PA} \vdash \theta$

#### Nonstandard Models: The Case of $\mathbb{Z}[X]^+$

To show the **independence** of $\theta$ from $T_n$, one constructs a **nonstandard model** of arithmetic where $\theta$ fails, yet $T_n$ still holds. This is done using the **positive part of the polynomial ring** $\mathbb{Z}[X]$, denoted $\mathbb{Z}[X]^+$, under the following interpretation:

- $S(p) = p + 1$
- $<$ is defined via lexicographic (dictionary) order, with leading positive coefficients
- $+$ and $\times$ are inherited from polynomial operations

The structure $(\mathbb{Z}[X]^+, 0, S, +, \times, <)$ satisfies all axioms of $T_n$, but **not $\theta$**, because elements like the indeterminate $X$ cannot be expressed as either an even number or its successor. This establishes:

$T_n \nvdash \theta \quad \text{and} \quad T_n \nvdash \neg \theta$

Hence, $\theta$ is **independent of $T_n$**.

#### $\Sigma_1$-Completeness of $T_n$

Despite its incompleteness, $T_n$ retains a remarkable degree of expressive power at the **$\Sigma_1$ level**. A $\Sigma_1$ sentence has the form $\exists x, \varphi(x)$, where $\varphi$ is quantifier-free. For any such sentence $\sigma$ in the language of arithmetic, we have the equivalence:

$T_n \vdash \sigma \quad \Longleftrightarrow \quad (\mathbb{N},0,S,+,\times,<) \models \sigma \quad \Longleftrightarrow \quad (\mathbb{Z}[X]^+,0,S,+,\times,<) \models \sigma$

This means that **$T_n$ is complete with respect to all $\Sigma_1$ truths in the standard model**, even though it cannot capture all true arithmetic statements. This fact underpins the **constructive foundations of Gödel’s proof**, which relies on the existence of provably enumerable $\Sigma_1$ statements whose negations are not provable.

---

### Gödel's Incompleteness and the Formal Arithmetic of Natural Numbers

#### Completing Incomplete Theories? Not Always Possible

While some initially incomplete theories—such as the theories of rational arithmetic or algebraically closed fields—can be completed by adding independent axioms (e.g., induction or order axioms), the same approach does not work for Peano Arithmetic (PA). A natural question arises: can we add enough independent sentences to make PA complete? Gödel’s answer is a definitive **no**. Any consistent, effectively axiomatized theory that extends elementary arithmetic (such as PA) is inherently **incomplete**.

#### The Core Idea of Gödel’s Theorem

At the heart of Gödel’s incompleteness theorems lies the construction of a sentence that, in effect, **says of itself that it cannot be proved**. To express such self-referential statements within arithmetic, we need a way to talk about logical syntax—formulas, proofs, deduction, etc.—**from within number theory itself**. This requires encoding all syntactic objects of logic as **natural numbers**, a method known as **Gödel numbering**.

#### Gödel Numbering: Representing Syntax in Arithmetic

The Gödel numbering system translates logical constructs—terms, formulas, proof sequences—into natural numbers. Some foundational tools include:

- **Numerical Term Encoding:** Each natural number is encoded in PA using the successor function: `k₀ = 0`, `kₙ₊₁ = S(kₙ)`. Thus, the term `S(S(S(0)))` represents 3, and so on.

- **Model Construction (Nonstandard Extensions):** Nonstandard models of PA are constructed by introducing a new constant symbol `c` such that `kₙ < c` for all standard natural numbers `n`. By the **Compactness Theorem**, such models exist. They contain “nonstandard numbers” that lie beyond all standard naturals—intuitively explaining why certain sentences cannot be settled in PA.

- **The μ-Operator and Bounded Quantifiers:** The μ-operator (`μx φ(x)`) selects the least `x` satisfying a property `φ(x)`, allowing the formalization of bounded quantification:

  - `∃x < t φ(x)` ⇔ `φ(μx < t φ(x))`
  - `∀x < t φ(x)` ⇔ `¬∃x < t ¬φ(x)`

  These are critical for embedding meta-logical content into arithmetical formulas.

#### Encoding Proofs and Syntax: Pairing and β Functions

Gödel’s **pairing function** `OP(a, b) = (a + b)² + a + 1` allows us to encode a pair of numbers uniquely as a single number. This supports the creation of sequences, where each term can be extracted using the **β-function**: `β(a, i)` returns the `i`-th element of the sequence encoded by `a`. These tools make it possible to express within arithmetic that:

- “a number `a` encodes a formula”
- “a number `b` encodes a proof of that formula”
- “there exists a number which is unprovable”

This leads directly to **Gödel’s First Incompleteness Theorem**: in any consistent, sufficiently expressive system like PA, there exists a sentence that is **neither provable nor refutable** within the system.

#### Formalizing Logical Axioms as Arithmetic Predicates

Logical axiom schemas are represented arithmetically via **predicate functions** over Gödel numbers. For each logical axiom schema, we define a predicate `PrpX(a)` that holds iff the number `a` encodes a formula of that schema. Examples include:

- **Implication Distribution Law (PrpIA)**
  Form: `((φ₁ → (φ₂ → φ₃)) → ((φ₁ → φ₂) → (φ₁ → φ₃)))`.
  Defined via encoded tuples like `<5, …, 6>` representing formulas in sequence.

- **Reflexivity of Implication (PrpIB)**
  Form: `(φ → φ)`. The predicate checks for the existence of such φ.

Other logical axiom schemas are handled similarly:

- **First and Second Tolerance Laws**

- **Reductio ad Absurdum (Indirect Proof)**

- **Quantifier Rules**

  - **Specialization (SpP):** `(∀x φ) → φ[x / t]`, where the term `t` is safely substitutable.
  - **Universal Quantifier Distribution (UQD):**
    `(∀x (φ₁ → φ₂)) → ((∀x φ₁) → (∀x φ₂))`, with `x` appearing freely in both.
  - **Irrelevant Quantifier Introduction (IrUQ):**
    `φ → ∀x φ` when `x` does not appear free in `φ`.

- **Identity Axioms:**

  - **Equality Reflexivity (EQL):** `x = x`.
  - **Equality Substitution (EQQ):**
    `(x = y) → (φ₁ → φ₂)` when substituting `x` for `y` equates `φ₁` and `φ₂`.

We define the full set of logical axioms using a **predicate `LAX₀(a)`**, which holds if `a` satisfies any of these `PrpX` predicates. Logical deduction is modeled with:

- **Modus Ponens Predicate (MP):**
  Encodes the deduction from `φ`, `φ → ψ` to `ψ`.

- **Non-logical Axiom Predicates (`NLAXn`, `NLAXPA`):**
  Denote the axioms of **elementary arithmetic** and **Peano arithmetic**, respectively.

- **Proof and Theorem Predicates:**

  - `PRFn(a)`: `a` is a valid proof sequence in the system.
  - `PRVn(a, b)`: `b` is a proof of formula `a`.
  - `Thmn(a)`: Formula `a` is a theorem.

These definitions allow **arithmetization of syntax**, which is essential for proving Gödel’s second theorem.

#### Recursive Functions and Definability

To formalize the concept of computability within arithmetic, we define the class of **recursive functions**, built from:

- **Basic Functions:** zero, successor, addition, multiplication, projections
- **Closure under Composition:** function nesting
- **μ-Operator (Minimization):** allows the definition of partial functions

The class `R` of recursive functions is closed under these operations. All **total recursive functions** (also called **primitive recursive functions**) are computable, aligning with the **Church-Turing thesis**.

#### Portability to Nonstandard Models

A natural question is whether these coding techniques and predicates can be **transferred to nonstandard models** of arithmetic. The answer is **yes**—the syntactic definitions are purely formal and rely on the encoding mechanisms. Thus, if they are definable in the standard model ℕ, they remain meaningful in nonstandard models. Even though such models contain nonstandard elements (i.e., "fake naturals"), the definitions of β, OP, μ, and all logical axioms remain intact as **first-order expressible** formulas.

Furthermore, all these syntactic constructs are **definable inside arithmetic**, typically as Δ₀ or Σ₁ formulas. This allows one to represent logic within arithmetic, enabling meta-mathematical reasoning entirely within the language of PA—culminating in Gödel’s revolutionary theorems.

---

### Definitional Extensions and the Σ₁-Completeness of Recursive Extensions of Peano Arithmetic

Peano Arithmetic (PA) significantly enhances our formal reasoning capabilities by introducing powerful principles like mathematical induction. However, Gödel’s First Incompleteness Theorem establishes that PA is _incomplete_—there exist true statements about natural numbers that PA cannot prove. Moreover, it cannot be extended in a consistent and effectively decidable way to become complete. Gödel’s Second Incompleteness Theorem goes further: **PA cannot prove its own consistency within its own framework**.

To prove this second theorem, we must represent **PA’s own syntax and proofs within PA itself**. This requires the ability to define and manipulate more complex relationships and functions _within_ the theory, leading to the notion of **definitional extension**.

#### Definitional Extensions: Enriching Language Without Increasing Power

Let $T$ be a consistent first-order theory with language $L(T)$. Suppose we have a formula $\psi(x_1, \dots, x_n) \in L(T)$ with all variables free. We can introduce a new $n$-ary predicate symbol $P$, not in $L(T)$, and define it via the biconditional:

$P(x_1, \dots, x_n) \Leftrightarrow \psi(x_1, \dots, x_n)$

This yields a new language $L(T^{\*}) = L(T) \cup \{P\}$, and we define a new theory $T^{\*}$ by adding this defining axiom to $T$.

Similarly, if we have a formula $\varphi(x_1, \dots, x_n, y) \in L(T)$ such that:

- **Existence**: $T \vdash \exists y\, \varphi(x_1, \dots, x_n, y)$
- **Uniqueness**: $T \vdash \forall y\, \forall y'\, (\varphi(\bar{x}, y) \wedge \varphi(\bar{x}, y') \rightarrow y = y')$

then we may introduce a new function symbol $f(x_1, \dots, x_n)$ defined by:

$f(x_1, \dots, x_n) = y \Leftrightarrow \varphi(x_1, \dots, x_n, y)$

Such definitional extensions are **conservative**: they do not alter the original theory’s deductive power. For every formula $\theta \in L(T^{\*})$, there exists a natural translation $\theta^b \in L(T)$ such that:

- $T^{\*} \vdash \theta \leftrightarrow \theta^b$
- $T^{\*} \vdash \theta$ if and only if $T \vdash \theta^b$

This shows that definitional extensions are merely syntactic sugar: they enrich the language but do not introduce any new truths. From a model-theoretic perspective, any model $\mathcal{M} \models T$ can be expanded to a model $\mathcal{M}^{\*} \models T^{\*}$ by interpreting the new symbols according to their defining formulas.

#### Σ₁-Completeness of Recursive Extensions of PA

Let $T$ be a **recursive extension** of Peano Arithmetic $T_{PA}$, meaning that $T$ is recursively enumerable and contains all axioms of $T_{PA}$. A fundamental and powerful result in this setting is:

> **Every recursive extension $T$ of $T_{PA}$ is Σ₁-complete.**
> That is, if a Σ₁ sentence $\theta$ is _true in the standard model_ $\mathcal{M}_\mathbb{N} \models \theta$, then $T \vdash \theta$.

This Σ₁-completeness rests on several key ingredients:

- **Construction of T-Special Expressions**

  To reason about Σ₁ sentences within $T$, we define **T-special expressions**, which are structured formulas crafted to facilitate manipulation and translation. These include:

  - Basic atoms: $\text{co} = x_1$, $f(x_1, \dots, x_n) = y$, $P(x_1, \dots, x_n)$, $\neg P(x_1, \dots, x_n)$
  - Boolean combinations: $\phi \wedge \psi$, $\phi \vee \psi$
  - Restricted quantifiers: $\forall x_i (x_i < a_j \rightarrow \phi)$
  - Existential closure: $\exists x\, \phi$

- **Reduction of Σ₁ Sentences to T-Special Forms**

  Every Σ₁ sentence can be rewritten as a T-special expression via structural induction:

  - For atomic formulas $x = t$, $P(t_1, \dots, t_n)$, and their negations, intermediate variables are introduced to rewrite them using special forms.
  - Boolean operations (¬, ∧, ∨) are handled recursively.
  - For existential quantifiers, the inner formula is first transformed, then the quantifier is applied.

- **Equivalence With $T_{PA}$-Special Expressions**

  Since $T$ is a definitional extension of $T_{PA}$, all new symbols can be eliminated using the defining formulas. Hence, every T-special expression is provably equivalent in $T$ to a $T_{PA}$-special expression.

- **Numerical Instances Are Provable in $T_{PA}$**

  Any _true_ numerical instance (in $\mathbb{N}$) of a $T_{PA}$-special expression is provable in $T_{PA}$. This is a consequence of the original Σ₁-completeness of Robinson Arithmetic $T_N$, and the fact that $T_{PA}$ is stronger than $T_N$. Because the structure of special expressions is finite and well-controlled, such instances reduce to basic provable arithmetical statements.

#### Putting It All Together

We now sketch the reasoning that justifies Σ₁-completeness for recursive extensions of $T_{PA}$:

- Begin with a Σ₁ sentence $\theta$ true in $\mathbb{N}$.
- Translate it into an equivalent T-special expression $\phi$.
- Translate $\phi$ into an equivalent $T_{PA}$-special expression $\phi'$.
- Use the truth of $\theta$ to identify a true numerical instance of $\phi'$.
- This instance is provable in $T_{PA}$, hence so is $\phi'$, and therefore so is $\theta$ in $T$.

#### Limitations and Beyond: Toward Gödel Coding and Metamathematics

This Σ₁-completeness result is _optimal_: it fails to generalize to Π₁ sentences. Some Π₁ sentences true in $\mathbb{N}$ remain unprovable in _any_ consistent recursive extension of $T_{PA}$—this is the content of Gödel’s Second Incompleteness Theorem.

To express such meta-mathematical statements like “this formula is provable” or “this number encodes a proof,” we need a more powerful formal machinery inside the theory. This leads us to **formal Gödel numbering**, substitution functions (`Subst`), and coding of proof sequences—all of which are developed inside a suitable extension of PA, often denoted $PA_f$. These allow us to internalize syntax and metatheoretic notions within arithmetic, laying the foundation for formal incompleteness and provability logic.

---

### The Paris–Harrington Principle: True in the Standard Model, but Unprovable in Peano Arithmetic

The **Paris–Harrington Principle (PHPP)** is a strengthening of the finite Ramsey theorem that holds in the standard model of the natural numbers ℕ but is **unprovable within Peano Arithmetic (PA)**. This result demonstrates the **independence** of PHPP from PA: even though PHPP is arithmetically true (i.e., it holds in ℕ), it cannot be derived from the axioms of PA. This is a concrete instance of Gödel's incompleteness phenomena—specifically, it shows that there exist true arithmetic statements which are **formally undecidable** in PA.

#### PHPP and Its Weaker Form: RPP

To establish the unprovability of PHPP within PA, researchers have introduced a **weakened version of the principle**, known as the **Restricted Partition Principle (RPP)**. RPP is specially designed to be **formally expressible within PA** and more amenable to syntactic manipulation. The strategy involves showing that if PHPP were provable in PA, then so would RPP be. However, one can construct a **nonstandard model** of PA in which RPP **fails**. This contradiction implies that PHPP cannot be provable in PA—thus, it is **independent**.

### The Restricted Partition Principle (RPP)

RPP is defined as follows: Given natural numbers $(c, m, n, k)$, there exists a number $d > n$ such that for any collection of $k$ **compressed mappings** $f_i: [d]^n \to d$ (where $[d]^n$ denotes the set of all $n$-tuples from $\{1, \dots, d\}$), there exists a set $Y \subseteq [c, d]$ satisfying:

- $|Y| \ge m$, and
- For all $f_i$, the function is **min-homogeneous** on $[Y]^n$.

A **compressed mapping** is a function $f$ such that $f(\vec{a}) < \min(\vec{a})$. This constraint ensures that function values always lie below the smallest input element.

#### Deriving RPP from PHPP

The implication **PHPP ⇒ RPP** is established through two key lemmas:

- **Lifting Lemma**: Given any coloring $g: [d]^n \to k$, we can construct a function $h$ such that applying PHPP to $h$ yields a large enough homogeneous set $Y$ for $g$ within the interval $(c, d)$, satisfying the required size condition.

- **Translation Lemma**: By encoding the behavior of compressed mappings as colorings, we translate the assumptions of RPP into the setting of PHPP. The homogeneous set given by PHPP can then be shown to satisfy the “min-homogeneity” condition of RPP.

As this implication is **formally provable within PA**, we conclude: if PA could prove PHPP, then it could also prove RPP.

#### RPP is True in the Standard Model

Since PHPP is known to be true in the standard model of arithmetic $\mathbb{N}$, and since it implies RPP, it follows that RPP also holds in $\mathbb{N}$. This sets the stage for proving independence: the next step is to find a **nonstandard model of PA** in which RPP fails, leading to the conclusion that PHPP is not provable in PA.

#### Ramsey's Theorem vs. Paris–Harrington Principle

It is important to distinguish between **Ramsey’s finite partition theorem** and PHPP. While PHPP strengthens Ramsey’s theorem by imposing an additional cardinality constraint on the homogeneous set (specifically, that the size of the homogeneous set must be at least as large as its smallest element), this enhancement makes it **unprovable in PA**. By contrast, the classical finite **Ramsey theorem is provable in PA**, using formal induction.

This contrast highlights the delicate boundary between what can and cannot be captured by formal arithmetic systems such as PA. The independence of PHPP from PA provides a vivid example of how **arithmetically natural statements** may escape the formal reach of even very robust systems of arithmetic.
