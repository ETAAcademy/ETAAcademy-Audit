# ETAAcademy-Adudit: 13. Mathematical Logic

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>13 Mathematical Logic</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>Mathematical Logic</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# Mathematical Logic: Foundations of Formal Verification

Mathematical logic is a discipline that studies formal reasoning and argumentation. It establishes a rigorous framework for mathematical deduction through two main systems: **propositional logic** and **predicate logic**.

**Propositional logic** focuses on statements that are either true or false and uses logical operators such as ¬ (negation), ∧ (conjunction), ∨ (disjunction), ⊕ (exclusive or), → (implication), and ↔ (biconditional) to form compound statements. Reasoning is carried out using **truth tables** and logical equivalences, such as **De Morgan's laws**, to analyze and manipulate these statements.

**Predicate logic** extends this framework by introducing **predicates** and **quantifiers** (∀ for "for all", ∃ for "there exists", and ∃! for "there exists exactly one") to describe properties of objects and relationships between them. It allows for the expression of more complex mathematical statements by restricting the domain of discourse and nesting quantifiers.

In mathematical reasoning, the process typically begins by formalizing a mathematical statement into a logical expression. Based on the definition of a valid argument, we apply various **inference rules** (such as **modus ponens** and **universal generalization**) and **proof techniques** (including **direct proof**, **proof by contraposition**, and **proof by contradiction**), along with **axioms** and already proven theorems, to derive the conclusion.

## 1. Formalization in Propositional Logic: Logical Equivalence and Compound Statements

Mathematical logic, especially **propositional logic**, is concerned with formal methods of reasoning and argumentation. At its core lies the concept of a **proposition**—a declarative sentence that is either **true** or **false**. These statements are represented by **propositional variables** (such as p, q, r, etc.), which are Boolean variables that can take on the value **true (T)** or **false (F)**.

To build more complex logical expressions, we combine propositions using **logical operators**, including:

- **Negation (¬)**: "not"
- **Conjunction (∧)**: "and"
- **Disjunction (∨)**: "or"
- **Exclusive or (⊕)**: "either...or" (but not both)

These combinations are known as **compound propositions**.

### Conditional Statements and Logical Equivalence

One of the most important compound forms in propositional logic is the **conditional statement**, also called an **implication**, typically expressed as:

> **"If p, then q"**
> Notation: _p → q_

This implication is considered **false only when p is true and q is false**; in all other cases, it is true. In the statement _p → q_, _p_ is called the **antecedent** and _q_ the **consequent**.

Conditional statements have several related forms:

- **Converse**: _q → p_
- **Inverse**: ¬*p* → ¬*q*
- **Contrapositive**: ¬*q* → ¬*p*

Notably, a conditional statement is **logically equivalent** to its contrapositive, meaning they always share the same truth values.

Two compound propositions are said to be **logically equivalent** if they have the same truth value under every possible truth assignment to their variables.

> We write _p ≡ q_ (or sometimes _p ⇔ q_) to denote that _p_ and _q_ are logically equivalent.

Logical equivalence is foundational in mathematical proofs, as it allows substitution of propositions with their equivalent forms to simplify or manipulate expressions without changing their logical meaning.

**Truth Table Example**

Consider the compound proposition:
**(_p_ ∨ ¬*q*) → (_p_ ∧ _q_)**

We analyze its behavior using a **truth table**:

| p   | q   | ¬q  | p ∨ ¬q | p ∧ q | (p ∨ ¬q) → (p ∧ q) |
| --- | --- | --- | ------ | ----- | ------------------ |
| T   | T   | F   | T      | T     | T                  |
| T   | F   | T   | T      | F     | F                  |
| F   | T   | F   | F      | F     | T                  |
| F   | F   | T   | T      | F     | F                  |

The final column shows the truth values of the compound implication. This table can be used to determine whether the proposition is a tautology, contradiction, or contingency.

**Tautologies, Contradictions, and Contingencies**

- **tautology** is a compound proposition that is always true regardless of the truth values of its components.
- **contradiction** is always false.
- **contingency** is neither always true nor always false—it depends on the truth values of the component propositions.

Understanding these classifications helps in proof techniques such as **proof by contradiction**, where a contradiction is derived to refute a hypothesis.

**Common Logical Equivalences**

Logical equivalences can be proven using truth tables or derived using known equivalences. These equivalences are essential tools for simplifying expressions and constructing logical proofs. Below are some of the most important ones:

- De Morgan’s Laws

  - ¬(p ∧ q) ≡ ¬p ∨ ¬q
  - ¬(p ∨ q) ≡ ¬p ∧ ¬q

- Conditional Equivalences

  - p → q ≡ ¬p ∨ q
  - ¬(p → q) ≡ p ∧ ¬q
  - p ∨ q ≡ ¬p → q
  - p ∧ q ≡ ¬(p → ¬q)

These are particularly useful in transforming conditional statements during logical derivations.

- Biconditional (Double Implication) Equivalences

  - p ↔ q ≡ (p → q) ∧ (q → p)
  - p ↔ q ≡ (p ∧ q) ∨ (¬p ∧ ¬q)
  - ¬(p ↔ q) ≡ p ↔ ¬q

These equivalences are useful when expressing "if and only if" (i.e., necessary and sufficient conditions) in logical or mathematical statements.

### From Propositional Logic to Predicate Logic: Expressing Complex Mathematical Statements

**Propositional logic** is limited to handling statements that are either completely true or false. However, it cannot express or reason about more complex statements involving **properties of objects** or **relationships between them**. For instance, propositional logic cannot capture the meaning of statements such as:

- "Every computer connected to the university network is functioning properly."
- "There is a computer that is currently under attack."

These statements go beyond simple true-or-false propositions by involving **objects**, **their attributes**, and **quantified conditions**. To address these expressive limitations, we turn to **predicate logic** (also known as **first-order logic**), which provides a more powerful framework for representing and reasoning about such statements with greater precision.

**Predicates and Variables**

At the core of predicate logic is the **predicate**, a logical structure that represents either a **property of an object** or a **relation between objects**. Predicates are always used with **variables**, and together they form the fundamental unit of predicate logic.

Consider the statement:

- "x is greater than 3"

This is not a proposition in itself, since we cannot determine its truth value unless we know the value of x. Such a statement is called a **predicate expression**, typically written as **P(x)**, where:

- **P** is the predicate (e.g., “greater than 3”),
- **x** is the variable (the **subject** of the sentence).

Only after assigning a specific value to x does **P(x)** become a **proposition**—a statement with a definite truth value.

Predicates can also involve **multiple variables** to express relationships. For example:

- **Q(x, y)** could represent "x is greater than y".

Again, the truth value of this statement depends on the values of both x and y.

#### Quantifiers: Extending Predicate Logic

To form complete logical statements from predicate expressions, we need **quantifiers**, which express the extent to which a predicate holds over a given **domain of discourse** (the set of values variables may take). The three most common quantifiers are:

**Universal Quantifier (∀)**

Indicates that a predicate holds **for all** values in the domain.

- Notation: **∀x P(x)** means "for all x, P(x) is true".
- Example: Many mathematical theorems assert that a property is true for all values of a variable within a specific domain.

**Existential Quantifier (∃)**

Indicates that there exists **at least one** value in the domain for which the predicate is true.

- Notation: **∃x P(x)** means "there exists an x such that P(x) is true".

**Uniqueness Quantifier (∃!)**

Indicates that **exactly one** value in the domain satisfies the predicate.

- Notation: **∃!x P(x)** means "there exists exactly one x such that P(x) is true".

Quantifiers are essential for turning predicate expressions into complete propositions that can be evaluated as true or false. They form the foundation of **predicate calculus**.

#### Quantifiers with Domain Constraints

Predicate logic often uses **restricted (or constrained) quantifiers**, which allow us to specify conditions directly after the quantifier. This simplifies logical statements by clearly expressing the subset of the domain under consideration.

For example:

- **∀x > 0 P(x)** means "for all x greater than 0, P(x) is true".
  This is logically equivalent to **∀x(x > 0 → P(x))**.
- **∃x > 0 P(x)** means "there exists an x greater than 0 such that P(x) is true".
  This is equivalent to **∃x(x > 0 ∧ P(x))**.

Using constrained quantifiers makes expressions more concise and readable, which is especially helpful in mathematical writing and formal proofs.

#### Nested Quantifiers

**Nested quantifiers** occur when one quantifier appears within the scope of another. For instance:

- **∀x ∃y (x + y = 0)**

This can also be seen as **∀x Q(x)**, where **Q(x) ≡ ∃y P(x, y)** and **P(x, y) ≡ x + y = 0**.

In nested quantifiers, the **outer quantifier** applies before the **inner quantifier**. For example:

- **∀x ∀y (x + y = y + x)** expresses the **commutative law** of addition for real numbers.
- **∀x ∃y (x + y = 0)** asserts that every real number has an **additive inverse**.
- **∀x ∀y ∀z (x + (y + z) = (x + y) + z)** expresses the **associative law** of addition.

To correctly interpret nested quantifiers, one must read them **from left to right**, understanding the scope and implications of each.

For example:

- **∀x ∀y ((x > 0 ∧ y < 0) → xy < 0)**
  Means: "For any real numbers x and y, if x is positive and y is negative, then their product is negative."

#### The Importance of Quantifier Order

The **order** of quantifiers is **crucial** in predicate logic, especially when **different types** of quantifiers are mixed.

When **all quantifiers are the same type**, changing their order does **not** affect the meaning:

- **∀x ∀y (x + y = y + x)** and **∀y ∀x (x + y = y + x)** are equivalent.

However, **mixing existential and universal quantifiers** changes the meaning drastically:

- **∃x ∀y (x + y = 0)** means "there exists an x such that for all y, x + y = 0" (which is **false**).
- **∀y ∃x (x + y = 0)** means "for every y, there exists an x such that x + y = 0" (which is **true**, since _x = -y_ works).

Another example:

- **∀x ∀y ∃z (x + y = z)** is **true**, because for any _x_ and _y_, their sum _z_ exists.
- **∃z ∀x ∀y (x + y = z)** is **false**, because there is **no single z** that equals the sum of all possible pairs _x + y_.

#### Translating Mathematical Statements

Accurately converting mathematical statements into **predicate logic** involves identifying variables, their domains, and arranging quantifiers correctly.

Examples:

- **"Every nonzero real number has a multiplicative inverse"**
  Translates to:
  **∀x (x ≠ 0 → ∃y (xy = 1))**

- **Definition of a function limit**:
  "The limit of f(x) as x approaches a is L" becomes:
  **∀ε > 0 ∃δ > 0 ∀x (0 < |x - a| < δ → |f(x) - L| < ε)**
  This statement precisely captures the notion that for any margin of error ε, there exists a threshold δ such that within δ of _a_, the function's output remains within ε of _L_.

---

## 2. Argumentation and Methods of Proof

**Arguments** are structured collections of propositions where the final statement is the **conclusion**, and all preceding statements serve as **premises**. The **validity** of an argument refers to whether the conclusion must be true whenever all the premises are true—that is, a valid argument cannot have all true premises and a false conclusion. For example, given the premises “If you have the current password, then you can log into the network” and “You have the current password,” we can validly infer the conclusion “You can log into the network.” In symbolic form, this argument is represented as:

**p → q, p ⊢ q**

This argument is valid because the implication **(p → q) ∧ p → q** is a tautology—it is always true. Particularly, when both **p → q** and **p** are true, the conclusion **q** must also be true.

To formalize and simplify logical reasoning, propositional logic relies on **inference rules**, which allow us to move from premises to conclusions based on valid argument structures. By substituting propositional variables for concrete statements, we abstract an **argument** into an **argument form**. One of the most fundamental inference rules is **Modus Ponens**: if **p → q** and **p** are both true, then **q** must be true.

Other essential inference rules include:

- **Addition**: From **p**, infer **p ∨ q**
- **Simplification**: From **p ∧ q**, infer **p**
- **Hypothetical Syllogism**: From **p → q** and **q → r**, infer **p → r**

These rules serve as logical building blocks for constructing more complex and valid arguments. For instance, from the premises “If it rains today, then we will not have a barbecue” and “If we don’t have a barbecue, then we will go for a canoe trip tomorrow,” we can use **Hypothetical Syllogism** to conclude: “If it rains today, then we will go for a canoe trip tomorrow.”

In more elaborate arguments, multiple inference rules are often used in combination. Consider the following example:

Premises:

1. It is not sunny this afternoon, and it is colder than yesterday.
2. We will go swimming only if it is sunny this afternoon.
3. If we do not go swimming, then we will go canoeing.
4. If we go canoeing, then we will return home before dusk.

We aim to conclude: “We will return home before dusk.”

Here’s the logical chain of reasoning:

- From (1), apply **Simplification** to get ¬p (It is not sunny).
- From (2) and ¬p, use **Modus Tollens** to get ¬q (We will not go swimming).
- From (3) and ¬q, apply **Modus Ponens** to get **r** (We will go canoeing).
- From (4) and **r**, apply **Modus Ponens** again to get **s** (We will return home before dusk).

This multi-step deduction demonstrates how complex arguments can be built systematically using basic rules of inference.

However, not all forms of reasoning are valid. There are common **fallacies** in propositional logic, especially when misapplying inference patterns:

- **Affirming the Consequent**
  Form: If **p → q**, and **q** is true, conclude **p**
  Symbolically: (p → q) ∧ q ⊢ p (Invalid)
  This is a fallacy because **q** may be true for reasons unrelated to **p**.

- **Denying the Antecedent**
  Form: If **p → q**, and **p** is false, conclude **q** is false
  Symbolically: (p → q) ∧ ¬p ⊢ ¬q (Invalid)
  This is also a fallacy because **q** might still be true through other causes.

For example, in the conditional statement “If you complete all the exercises, then you’ve studied calculus,” it is incorrect to reason that having studied calculus necessarily implies completing all exercises (**affirming the consequent**), or that not completing exercises implies not having studied calculus (**denying the antecedent**).

Understanding valid inference rules and recognizing fallacies is fundamental to constructing sound logical proofs and ensuring the rigor of deductive reasoning.

### Rules of Inference for Quantified Statements

In mathematical reasoning, **rules of inference for quantified statements** are essential tools that are frequently used—often implicitly—in formal proofs. These rules allow us to move logically between statements involving quantifiers such as "for all" (∀) and "there exists" (∃). The four fundamental inference rules involving quantifiers are:

- **Universal Instantiation (UI)**:
  From a universal statement **∀x P(x)**, we can deduce that **P(c)** holds for any specific element **c** in the domain.
  _Example_: From "All students passed the test" (∀x Passed(x)), we can infer "Alice passed the test" (Passed(Alice)).

- **Universal Generalization (UG)**:
  If **P(c)** is true for every element **c** in the domain (and **c** is arbitrary), then we can conclude **∀x P(x)**.
  _Note_: Care must be taken that **c** is not a special or fixed element but represents any arbitrary object in the domain.

- **Existential Instantiation (EI)**:
  From an existential statement **∃x P(x)**, we can assume there exists a specific (but unknown) element **c** such that **P(c)** holds.
  _Example_: From "There exists a student who read the book" (∃x Read(x)), we can infer "Some student, say c, read the book" (Read(c)).

- **Existential Generalization (EG)**:
  From a known instance **P(c)**, we can generalize to **∃x P(x)**, meaning that there exists at least one element for which **P(x)** is true.

These quantified inference rules are typically used in conjunction with propositional logic rules to build sound and rigorous mathematical proofs. For instance, consider the task of proving the conclusion:
**“Some student who passed the first exam did not read the book”**
given the premises:

- “There is a student in this class who did not read the book” (∃x ¬Read(x)), and
- “Every student in this class passed the first exam” (∀x Passed(x)).

To complete the proof, we proceed as follows:

- Apply **Existential Instantiation** to the first premise to get a specific student **c** such that ¬Read(c).
- Apply **Universal Instantiation** to the second premise to get Passed(c).
- Use **Conjunction (Simplification)** to combine or extract necessary parts and arrive at the conclusion:
    “There exists a student who passed the first exam and did not read the book” (∃x (Passed(x) ∧ ¬Read(x))).

These steps demonstrate how quantified inference rules provide the foundational structure needed to construct logically valid arguments.

In mathematical logic, quantified and propositional inference rules are often used in combination. Two of the most commonly applied combined rules are:

- **Universal Modus Ponens**:
  This is a combination of **Universal Instantiation** and **Modus Ponens**. If we are given
  **∀x (P(x) → Q(x))** and **P(c)** for a specific element **c**, we can conclude **Q(c)**.
  _Example_: From “For all real numbers x, if x > 4, then x² < 2^x” and the fact that 6 > 4, we can deduce that 6² < 2⁶.

- **Universal Modus Tollens**:
  A combination of **Universal Instantiation** and **Modus Tollens**. If we have
  **∀x (P(x) → Q(x))** and **¬Q(c)**, we can conclude **¬P(c)**.
  _Example_: If “For all x, if x is a cat then x has whiskers” and “x does not have whiskers,” then we can conclude “x is not a cat.”

By mastering these quantified inference rules and their interactions with propositional logic, one gains the tools necessary for precise and logically sound mathematical proof construction.

### Key Terminology and Proof Techniques in Mathematical Logic

Mathematical logic is built upon a precise vocabulary and a variety of proof techniques that form the foundation of rigorous mathematical reasoning. Understanding these terms and methods is essential for constructing and evaluating proofs effectively.

#### Important Terminology in Mathematical Logic

- **Theorem**: A statement that has been rigorously proven to be true using logical reasoning based on axioms, definitions, and previously established theorems.

- **Proposition**: A minor theorem, often less central or complex, but still requiring formal proof.

- **Lemma**: A "helper" theorem used to prove more significant results. Though sometimes overlooked, lemmas can be crucial to the structure of a larger proof.

- **Corollary**: A statement that follows directly and easily from a previously proven theorem, requiring little to no additional proof.

- **Conjecture**: A statement believed to be true based on evidence or intuition but has not yet been formally proven.

- **Proof**: A logical argument that establishes the truth of a mathematical statement using axioms, definitions, and already proven results.

- **Axiom**: A foundational statement assumed to be true without proof. Axioms use undefined primitive terms and serve as the starting point for deducing theorems.

- **Assumption**: A temporary premise taken as true for the sake of argument or proof, often within the context of a specific logical derivation.

Clear definitions are essential for all components of a proof, except for axioms and primitive terms, which are accepted without definition. These elements together form the structure of formal mathematical reasoning.

#### Common Proof Techniques

- **Direct Proof**
  A direct proof aims to establish the truth of a conditional statement **p → q** by assuming **p** is true and logically deducing that **q** must also be true.
  _Example_: To prove "If **n** is odd, then **n²** is odd," assume **n = 2k + 1** for some integer **k**. Then,

  $n^2 = (2k + 1)^2 = 4k^2 + 4k + 1 = 2(2k^2 + 2k) + 1,$

  which is an odd number. Hence, the statement is proven directly.

- **Proof by Contraposition**
  This is an indirect method that leverages the logical equivalence between a statement **p → q** and its contrapositive **¬q → ¬p**. When proving **p → q** is difficult, one can instead prove **¬q → ¬p**.
  _Example_: To prove "If **n² > m²** (for positive integers **n** and **m**), then **n > m** or **n = –m**", assume the conclusion is false: **n ≤ m** and **n ≠ –m**. Then **n² ≤ m²**, contradicting the assumption. Thus, the original statement is true.

- **Proof by Contradiction**
  To prove a statement **p**, assume **¬p** is true. If this assumption leads to a logical contradiction, then **¬p** must be false, and **p** must be true.
  _Example_: To prove "√2 is irrational," assume the opposite—that it is rational—and show that this leads to a contradiction in the form of both numerator and denominator being even, which violates the assumption of reduced form.

- **Biconditional Proof (Proof of Equivalence)**
  To prove **p ↔ q**, both **p → q** and **q → p** must be proven. This method is commonly used to demonstrate the equivalence of multiple statements.
  For multiple statements **p₁, p₂, ..., pₙ**, proving their equivalence requires establishing the chain:

  $p_1 ↔ p_2, \quad p_2 ↔ p_3, \quad ..., \quad p_{n−1} ↔ p_n.$

  This shows that all statements imply and are implied by one another.

- **Proof by Exhaustion (Proof by Cases)**
  This method involves checking all possible cases individually, often used when the domain is small and finite.
  _Example_: To prove "For all positive integers **n ≤ 4**, we have **n² < 2ⁿ**", simply verify:

  - n=1: 1² = 1 < 2
  - n=2: 4 = 4
  - n=3: 9 > 8 (violates condition)
  - n=4: 16 = 16
    (Note: This example reveals a contradiction, so it actually disproves the statement rather than proves it, which highlights how proof by exhaustion can also disprove claims.)

- **Case Analysis (Proof by Cases)**
  This technique divides the problem into multiple mutually exclusive and collectively exhaustive scenarios, proving the statement for each case.
  _Example_: To prove "For all integers **n**, |n| ≥ 0", consider:

  - Case 1: n > 0 → |n| = n ≥ 0
  - Case 2: n = 0 → |n| = 0
  - Case 3: n < 0 → |n| = –n > 0
    In each case, the result holds.
    The phrase **“without loss of generality” (WLOG)** is often used to reduce the number of cases by focusing on representative scenarios when symmetry or similar reasoning applies.

- **Existence Proofs**
  These are used to prove that at least one object with a specific property exists, often expressed as **∃x P(x)**.

  - **Constructive Proof**: Provides an explicit example.
    _Example_: 1729 = 9³ + 10³ = 1³ + 12³ shows that a number can be expressed as the sum of cubes in two different ways.
  - **Non-Constructive Proof**: Shows existence indirectly, without providing a specific instance.
    _Example_: To prove there exist irrational numbers **x** and **y** such that **xʸ** is rational, consider **x = √2**. Either **√2^√2** is rational (done), or if it's irrational, then let **y = √2** and **x = √2^√2**; in this case, **xʸ = 2**, which is rational.

- **Uniqueness Proofs**
  These are used to show that a solution or object not only exists but is also the only one of its kind.
  The proof involves two steps:

  - **Existence**: Prove that such an object exists.
  - **Uniqueness**: Show that if two such objects exist, they must be equal.
    _Example_: Prove that the equation **ax + b = 0** (with **a ≠ 0**) has a unique solution.
  - Existence: x = –b/a satisfies the equation.
  - Uniqueness: Suppose y is another solution. Then **ay + b = 0**, which implies **y = –b/a = x**.

Together, these terminologies and proof techniques form the backbone of mathematical logic and reasoning. Mastery of these tools enables one to build and understand rigorous arguments across all areas of mathematics.
