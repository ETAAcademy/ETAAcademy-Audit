# ETAAcademy-Adudit: 6. ZK Security

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>06. ZK Security</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>ZK_Security</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# ZK Security: 10 Vulnerabilities and Mitigation Approaches

Zero-Knowledge (ZK) systems are designed to provide privacy and security by enabling a prover to convince a verifier that they know a secret without revealing the secret itself. However, vulnerabilities can arise at various layers of a ZK system, which can compromise its integrity, soundness, and zero-knowledge properties. These layers include the **circuit arithmetization**, **compilation transformation**, **proof and verification**, and **system integration**.

Vulnerabilities in the circuit arithmetization layer usually come from coding mistakes or weak constraints, which can impact system reliability and security. At the compilation layer, issues arise when errors during compilation lead to incorrect constraints. In the proof and verification layer, attackers might forge proofs, compromising the verification process. System integration vulnerabilities often come from external components interacting with the ZK proof system, especially when implicit constraints aren't properly enforced. Weaknesses in any of these layers can put the entire system at risk.

These vulnerabilities mainly impact three key areas: **soundness**, where false proofs get wrongly accepted; **completeness**, where valid proofs fail verification; and **zero-knowledge**, where sensitive data like private keys or secret inputs might leak. Keeping a ZK system secure depends on getting every layer right—any weakness at any point can put the whole system’s privacy and security at risk.

---

## 1. **Range Validation**

One common vulnerability in ZK systems is **range validation**. From low-level field arithmetic to high-level protocol design, a lack of proper range checks can lead to errors in computation or security flaws.

### Issue

In the **o1js** library used for zero-knowledge application development on the **Mina blockchain**, data is often split into smaller components for processing. For example, the number `7` can be split into `[1, 1, 1]` (binary). Within **o1js**, the data may be represented in terms of powers of $2^{88}$:

$1 + 1 \times 2^{88} + 1 \times (2^{88})^2 = 1 + 1 \times 2^{88} + 1 \times 2^{176}$

Without proper constraints, some values can be split in multiple ways, leading to ambiguity in the computation. For example:

$$
1 + 1 \times 2 + 1 \times 4
== 1 + (1 + 2) \times 2 + (2 - 2) \times 4
== 1 + 3 \times 2
$$

This means the same number can be split in different combinations, leading to incorrect calculations or vulnerabilities.

In the **o1js** implementation, a custom type uses 3 chunks of size $2^{88}$ (limbs)to represent a value. However, the code does not check whether these chunks are within the range of $[0, 2^{88} - 1]$.

### Impact

Due to the lack of proper range checks, attackers can craft specific input values that overflow in subsequent computations, leading to:

- **Corrupted computation results**
- **Compromised cryptographic protocol integrity**
- **Possible bypassing of security validations**

### Solution

The core issue is the **missing range check**, so the solution is to **strictly validate the range of data**:

1. **Check each limb's range** to ensure it falls within $[0, 2^{88}-1]$.
2. **Add constraints** wherever data decomposition occurs to prevent overflow-based attacks.
3. **Perform additional verification** at the circuit level to ensure that computed values do not exceed the valid range.

<details><summary>Code</summary>

```JavaScript

static check(signature: EcdsaSignature) {
	// more efficient than the automatic check, which would do this for each scalar
	separately
	this.Curve.Scalar.assertAlmostReduced(signature.r, signature.s);
}

static assertAlmostReduced<T extends Tuple<ForeignField>>(
	...xs: T
): TupleMap<T, AlmostForeignField> {
	Gadgets.ForeignField.assertAlmostReduced(
		xs.map((x) => x.value),
		this.modulus,
		{ skipMrc: true }
);
return Tuple.map(xs, this.AlmostReduced.unsafeFrom);
}

class Ecdsa extends createEcdsa(CurveParams.Secp256k1) {}

let gx: Field3 = Ecdsa.Curve.generator.x.value;
let gy: Field3 = Ecdsa.Curve.generator.y.value;

gx[0] = gx[0].add(gx[1].mul(2n**88n));
gx[1] = new Field(0n);

gy[0] = gy[0].add(gy[1].mul(2n**88n));
gy[1] = new Field(0n);

console.log(‘pt: {
	x: ${gx},
	y: ${gy}
}‘);

// Not really a signature, just some non-mrc’d points to illustrate
// the EcdsaSignature check() function does not check properly
let sig = new Ecdsa({
	r: gx,
	s: gy,
});
Ecdsa.check(sig); // assertion passed

```

</details>

---

## 2. **Non-Deterministic Circuits**

**Non-deterministic circuits** arise when constraints are insufficiently defined, leading to multiple valid proof options for a given outcome. This presents a security risk, as attackers could potentially exploit multiple valid proof pathways to bypass the circuit's verification and execute malicious actions.

For example, a vulnerability in **ZK Email** was caused by **non-deterministic circuits**, allowing an attacker to forge an email address. By injecting invalid characters (like `\xff`) into the email header, an attacker could bypass the ZK regular expression circuit's verification and spoof the sender's email address. The **ZK Email team** addressed this by implementing proper **input range checks**, ensuring the validity of input characters.

### Issue

**ZK Email** is a technology that enables on-chain transactions to be initiated via email, combining **DKIM signatures** with **zero-knowledge proofs**. The key components of the system include:

- **ZK Regular Expression Compiler**: Translates regular expressions into equivalent **deterministic finite automata (DFA)** for input validation.
- **Circom Circuit**: Verifies the email's DKIM signature, processes email data, and extracts public information while keeping the contents private.
- **DKIM Oracle**: Retrieves the DKIM public key from email providers and generates signature data to submit to the DKIM registration smart contract.
- **Relayer**: Coordinates off-chain components, receives emails, generates ZK proofs, and submits them on-chain.

The issue arose in the **ZK Regular Expression Compiler**, which allowed invalid UTF-8 characters (such as `\xff`) to be injected into the input string. These characters were not correctly processed, causing the circuit logic to fail.

### Impact

Many email providers send invalid UTF-8 characters, such as `\xff`, in email headers (e.g., the **subject**). Attackers can exploit this flaw by injecting the `\xff` character into the **subject** header, bypassing the ZK circuit's validation logic.

For example, an attacker could send a DKIM-signed email with the `\xff` character in the subject, causing the **EmailAddrRegex** circuit to incorrectly recognize the sender's address as `victim@anydomain` while the actual address should be different. This allows attackers to **forge email addresses**, sending fraudulent emails while impersonating the victim.

### Solution

The **ZK Email team** fixed this vulnerability by introducing **input range checks** in the circuit. Now, all **Circom circuits** generated by the ZK regular expression compiler include constraints to ensure that each input character is within the valid range, preventing attackers from bypassing the validation logic using invalid characters like `\xff`.

<details><summary>Code</summary>

```Json

{
  "parts": [
    {
      "is_public": false,
      "regex_def": "(\r\n|^)dkim-signature:"
    },
    {
      "is_public": false,
      "regex_def": "([a-z]+=[^;]+; )+t="
    },
    {
      "is_public": true,
      "regex_def": "[0-9]+"
    },
    {
      "is_public": false,
      "regex_def": ";"
    }
  ]
}

```

```circom

var num_bytes = msg_bytes+1;
signal in[num_bytes];
// -->
in[0]<==255;
// <--
for (var i = 0; i < msg_bytes; i++) {
	in[i+1] <== msg[i];
}

```

```circom
for (var i = 0; i < num_bytes; i++) {
	state_changed[i] = MultiOR(2);
	eq[0][i] = IsEqual();
	// -->
	eq[0][i].in[0] <== in[i];
	eq[0][i].in[1] <== 255;
	// <--
	and[0][i] = AND();
	and[0][i].a <== states[i][0];
	and[0][i].b <== eq[0][i].out;
        // -->
	states[i+1][1] <== and[0][i].out;
        // <--
	state_changed[i].in[0] <== states[i+1][1];
	eq[1][i] = IsEqual();
	eq[1][i].in[0] <== in[i];
	eq[1][i].in[1] <== 97;
	and[1][i] = AND();
	and[1][i].a <== states[i][1];
	and[1][i].b <== eq[1][i].out;
	states[i+1][2] <== and[1][i].out;
	state_changed[i].in[1] <== states[i+1][2];
	states[i+1][0] <== 1 - state_changed[i].out;
}
```

```circom
for (var i = 0; i < num_bytes; i++) {
	state_changed[i] = MultiOR(2);
	eq[0][i] = IsEqual();
	eq[0][i].in[0] <== in[i];
	eq[0][i].in[1] <== 255;
	and[0][i] = AND();
	and[0][i].a <== states[i][0];
	and[0][i].b <== eq[0][i].out;
	states[i+1][1] <== and[0][i].out;
	state_changed[i].in[0] <== states[i+1][1];
	eq[1][i] = IsEqual();
	// -->
	eq[1][i].in[0] <== in[i];
	eq[1][i].in[1] <== 97;
	// <--
	and[1][i] = AND();
	and[1][i].a <== states[i][1];
	and[1][i].b <== eq[1][i].out;
	states[i+1][2] <== and[1][i].out;
	state_changed[i].in[1] <== states[i+1][2];
	// -->
	states[i+1][0] <== 1 - state_changed[i].out;
 	// <--
}
```

</details>

---

## 3. Memory Operations

To keep ZK systems secure, you need to make sure constraints are properly applied at every step. Otherwise, attackers could tweak the data and sneak past verification. This is especially important in circuits that handle complex memory operations—if the constraints aren’t solid and consistent, bad actors might find a way to generate fake proofs and bypass the checks.

### Issue

In **zkSync Era**, memory can be written by other circuits (such as code decommitment and Keccak hashing). Therefore, the main VM circuit itself cannot independently enforce constraints on memory read operations. Every time the VM circuit reads from or writes to memory, it adds the operation to the memory queue.

The memory queue logs the sequence of memory operations, including the timestamp of the operation, the accessed memory location (memory page and memory index), whether it is a read or write operation, and the value being read or written.

**Constraints on memory operations**:

- During a **write** operation, the Main VM circuit enforces that the value of the source register is placed into a "write" memory query, which is then appended to the queue.
- When processing **read** operations, since the Main VM circuit cannot know the value at a specific address (because other circuits may modify memory), it loads the read value into a register and appends a "read" memory query to the queue.

After all memory operations are submitted, the **RAM Permutation circuit** checks their consistency. This circuit takes two memory queues as input: one from the Main VM circuit (and other memory-accessing circuits) and another from the **witness**, which has sorted the queries by memory page, memory index, and timestamp.

After sorting, the RAM Permutation circuit verifies the memory consistency.

1. **Memory Query Structure**: In the ZK circuit of zkSync Era, a memory query is represented by the `MemoryQuery` structure. In the code, `MemoryQuery` is transformed into a `RawMemoryQuery`. The `value` field of `RawMemoryQuery` is split into two fields: `Num<E>` and `UInt64<E>`. This is because the `bn254` curve used in zkSync can only store 253-bit field elements, so a 256-bit `UInt256<E>` is split into two fields for storage.
2. **Data Packing**: The `RawMemoryQuery` is packed into two `Num<E>` values, `el0` and `el1`. `el0` stores the lower 192 bits, and `el1` uses `LinearCombination` to pack the remaining fields (such as timestamp, memory page, etc.) into one value.
3. **MemoryWriteQuery Construction**: During a memory write operation, the `MemoryWriteQuery` structure is split into multiple parts (such as `lowest_128`, `u64_word_2`, `u64_word_3`), which are then combined using `LinearCombination` into a new `RawMemoryQuery`.

The issue arises during the construction of the `MemoryWriteQuery`, particularly within the `from_key_and_value_witness` function. Here, `LinearCombination` is used to generate constraints, but these constraints are not actually applied. Specifically, the code aims to constrain the higher 128 bits to zero, but due to the lack of operations like `enforce_zero(cs)` or `into_num(cs)`, these constraints do not take effect. As a result, the higher 128 bits of the memory value are unconstrained within the ZK circuit, allowing malicious provers to set these bits to arbitrary values.

Through this vulnerability, attackers can manipulate the higher 128 bits of the `MemoryWriteQuery` value. For example, when the `_amount` parameter is written to memory, attackers can manipulate the value field, replacing a small ETH amount (e.g., 0.00002 ETH) with a large value (e.g., 100K ETH), thereby executing a theft in the L2 to L1 withdrawal process.

**Steps of the Attack:**

1. **Modify the `write_query`**: When the `_amount` is set to a specific value (e.g., `0x1337133713370000000000000000`), the attacker changes the higher 128 bits to a larger value (e.g., `0x152d0000133713371337`), creating a fake large withdrawal.
2. **Modify the Code**: The attacker alters zkSync's backend code (e.g., in `uma.rs` and `write_query.rs`) to include the forged values in the ZK proof. These fake values do not change the circuit's constraints, so they can still pass the verification.
3. **Execute the Attack**: Using the manipulated ZK proof, the attacker can initiate a withdrawal request, successfully transferring large amounts of funds to their own account.

### Impact

The practical impact of this vulnerability is that attackers can forge withdrawal requests, allowing them to transfer large amounts of funds from the zkSync Era L2 to the L1 bridging system. Although this attack requires a high level of technical skill and a specific time window (with a 21-hour delay), it may become easier to exploit as zkSync Era decentralizes further.

### Solution

To fix this issue, the constraints generated by `LinearCombination` during the construction of `MemoryWriteQuery` should be enforced, particularly ensuring that the higher 128 bits are properly constrained. Specifically, the functions `enforce_zero(cs)` or `into_num(cs)` should be called within the `from_key_and_value_witness` function to ensure that these fields are correctly validated in the ZK circuit.

<details><summary>Code</summary>

```rust

impl<E: Engine> RawMemoryQuery<E> {
  pub fn pack<CS: ConstraintSystem<E>>(
    &self,
    cs: &mut CS,
  ) -> Result<[Num<E>; 2], SynthesisError> {
    let shifts = compute_shifts::<E::Fr>();
    let el0 = self.value;
    let mut shift = 0;
    let mut lc = LinearCombination::zero();

    lc.add_assign_number_with_coeff(&self.value_residual.inner, shifts[shift]);
    shift += 64;

    // NOTE: we pack is as it would be compatible with PackedMemoryQuery later on
    lc.add_assign_number_with_coeff(&self.memory_index.inner, shifts[shift]);
    shift += 32;
    lc.add_assign_number_with_coeff(&self.memory_page.inner, shifts[shift]);
    shift += 32;

    // ------------
    lc.add_assign_number_with_coeff(&self.timestamp.inner, shifts[shift]);
    shift += 32;
    lc.add_assign_boolean_with_coeff(&self.rw_flag, shifts[shift]);
    shift += 1;
    lc.add_assign_boolean_with_coeff(&self.value_is_ptr, shifts[shift]);
    shift += 1;

    assert!(shift <= E::Fr::CAPACITY as usize);

    let el1 = lc.into_num(cs)?;
    // dbg!(el0.get_value());
    // dbg!(el1.get_value());

    Ok([el0, el1])
  }
}

...

```

</details>

---

## 4. Input/Output

In zero-knowledge proof circuits, some variables may serve as public inputs but may not have any constraints set on them. These unconstrained public inputs might be optimized out of the circuit, meaning they will not be used in the verification process. As a result, when validating the proof, these public input values can be arbitrarily modified without affecting the validity of the proof.

### Issue

The **Jolt validator** relies on memory addresses derived from the proof instead of constructing these memory addresses independently. This allows malicious provers to manipulate memory addresses, making the output value or termination bit identical to the input values, effectively forging the output or controlling the termination bit.

Malicious provers can set the output and termination bit addresses to the same as the input address, forging correct output values and bypassing the validator. This means they can trick the validator into accepting incorrect computational results. In a given test, the attacker first changes the input value to 1, ensuring the termination bit is `true`. They then set the output and termination bit addresses to be the same as the input address, successfully forging the output value.

1. **Test Implementation**:
   In the `truncated_trace` test, the input and output addresses are manipulated, and the attacker tries to generate a proof with these forged input and output values. The proof passes the validator’s check, indicating that the attack was successful.
2. **Example Code**:

   - `program.set_input(&1u8)` – Sets the input value to 1, ensuring the termination bit is `true`.
   - `io_device.memory_layout.output_start = io_device.memory_layout.input_start` – Sets the output address to be the same as the input address.
   - `RV32IJoltVM::verify` – Verifies whether the forged proof can pass the validator’s check.

3. **Attack Reproduction**:
   By altering the memory layout so that both the termination bit and output addresses point to the same location as the input address, the attacker can forge the output value. This attack can be reproduced in the `truncated_trace2_sha3_e2e_hyperkzg` test.

### Impact

**Forgery of Proofs**: The attacker can manipulate memory addresses to forge a proof and make it pass the validator.
**Bypassing the Termination Bit**: By controlling the termination bit’s memory address, the attacker can make the program falsely believe the computation has finished, bypassing the validator’s check.
**Forgery of Output**: The attacker can also forge the output value, making the attack result appear legitimate. In summary, this attack exploits the validator’s trust in memory addresses, bypassing the verification process and successfully forging both the output and termination flags.

### Solution

- **Ensure Memory Address Independence**: The validator should avoid using memory addresses derived from the proof itself. Instead, it should construct or verify these addresses during the verification process to prevent malicious modifications.
- **Strengthen Memory Access Control**: There should be stricter controls on the memory layout to prevent attackers from manipulating addresses to forge outputs or control termination bits.

<details><summary>Code</summary>

```rust

#[test]
fn truncated_trace() {
    let artifact_guard = FIB_FILE_LOCK.lock().unwrap();
    let mut program = host::Program::new("fibonacci-guest");
    program.set_input(&1u8); // change input to 1 so that termination bit equal true
    let (bytecode, memory_init) = program.decode();
    let (mut io_device, mut trace) = program.trace();
    trace.truncate(100);
    // change the output to the same as input to show that we can also forge the output value
    io_device.outputs[0] = 1;
    drop(artifact_guard);

    // change memory address of output & termination bit to the same address as input
    io_device.memory_layout.output_start = io_device.memory_layout.input_start;
    io_device.memory_layout.output_end = io_device.memory_layout.input_end;
    io_device.memory_layout.termination = io_device.memory_layout.input_start;

    let preprocessing =
        RV32IJoltVM::preprocess(bytecode.clone(), memory_init, 1 << 20, 1 << 20, 1 << 20);
    let (proof, commitments, debug_info) =
        <RV32IJoltVM as Jolt<Fr, HyperKZG<Bn254>, C, M>>::prove(
            io_device,
            trace,
            preprocessing.clone(),
        );
    let verification_result =
        RV32IJoltVM::verify(preprocessing, proof, commitments, debug_info);
    assert!(
        verification_result.is_ok(),
        "Verification failed with error: {:?}",
        verification_result.err()
    );
}

```

</details>

---

## 5. Instruction Operations

**Instruction Vulnerabilities** are directly related to missing logic and constraints when the virtual machine executes specific opcodes. For example, the `LOAD` opcode execution may not have proper constraints, leading to register values that do not match the expected values. During the execution of the `LOAD` operation, the virtual machine should load the value from memory into a register (such as `local.a`). However, the current implementation only checks whether the memory value changes after the `LOAD` operation, without ensuring that the value loaded into the register is correctly fetched from memory. This absence of checks allows the register to be overwritten with arbitrary values, creating a security vulnerability.

### Issue

The issue revolves around CPU memory checks in the Recursion VM when executing the `LOAD` opcode. Specifically, the `LOAD` opcode should load the value from memory into the corresponding register (e.g., `local.a`), but the current implementation only checks whether the memory value changes after the operation, without ensuring the value loaded into the register is correct. As a result, the register’s value may be overwritten by any value, creating a potential security flaw.

- For the `LOAD` operation: The current implementation only ensures that the memory value hasn't changed.

- For the `STORE` operation: The register value is written to memory.

### Impact

In the `LOAD` operation, the system does not enforce a strict check to ensure that the value in register `a` matches the value in memory. As long as the memory value hasn't changed, the operation passes the check, without verifying that the loaded value is actually correct.

### Solution

It is recommended to modify the code to include a check during the `LOAD` operation, ensuring that the value loaded into register `a` matches the value in memory. This can be achieved by modifying the selector for the `LOAD` operation to implement this check.

<details><summary>Code</summary>

```rust

// Constraints on the memory column depending on load or store.
// We read from memory when it is a load.
builder.when(local.selectors.is_load).assert_block_eq(
    *memory_cols.memory.prev_value(),
    *memory_cols.memory.value(),
);
// When there is a store, we ensure that we are writing the value of the a operand to the memory.
builder
    .when(local.selectors.is_store)
    .assert_block_eq(*local.a.value(), *memory_cols.memory.value());

```

</details>

---

## 6. Compilation Conversion

Frontend vulnerabilities occur during the process of compiling high-level source code into specific arithmetic representations. These vulnerabilities often stem from compilation errors or incorrect representations generated during the conversion process, which can lead to issues with the circuit’s constraints, affecting the correctness and validation of the circuit.

For example, a frontend circuit might fail to properly handle the polynomial blinding step, exposing sensitive data when using zk-SNARKs. The design and implementation of frontend circuits must ensure that all operations follow proper security protocols, particularly when dealing with polynomials and secret witnesses. If polynomial blinding is not appropriately applied, attackers may be able to deduce polynomial coefficients from evaluation points, thereby revealing secret information.

### Issue

In certain Zero-Knowledge (ZK) protocols, particularly those using zk-SNARKs, the prover demonstrates knowledge of a secret by encoding the secret witness as part of a polynomial. To verify correctness, the prover typically reveals evaluation points of the polynomial, which are determined by the protocol setup. However, without adequate protections, revealing enough evaluation points could allow an attacker to reconstruct the polynomial, thereby learning the secret witness and breaking the zero-knowledge property.

For instance, a polynomial of degree `n` (with `n+1` coefficients) can be fully reconstructed from `n+1` evaluation points.

To prevent this, **polynomial blinding** is employed, which adds randomness to mask the polynomial, often by adding random coefficients to prevent reconstruction. For example, a degree `n` polynomial can be blinded into a degree `n+r` polynomial, with `r` additional evaluation points, making it impossible to deduce the polynomial’s information from the evaluation points. This ensures the privacy of the secret witness and maintains the zero-knowledge guarantee of the protocol.

However, in the implementation of the Linea PLONK Go prover, polynomial segments were not appropriately blinded, which could potentially allow attackers to recover the witness through statistical analysis.

This issue specifically concerns the **blinding of quotient polynomials**, where the quotient polynomial `h(X)` is divided into multiple sub-polynomials (e.g., $h_1, h_2, h_3$), and their KZG commitments are directly included in the proof without blinding. This violates the latest PLONK specification, as it enables attackers to analyze these commitments and recover the original quotient polynomial, thus breaking the zero-knowledge property.

### Impact

To fix this issue, the sub-polynomials of `h(X)` (i.e., $h_1, h_2, h_3$) should now be blinded, as per the latest PLONK specification (February 23, 2024 version). This update also introduces a global option `StatisticalZK`, which controls whether blinding is enabled. When this option is disabled, the code will revert to the previous version, where sub-polynomials are not blinded.

However, the fix introduces some performance overhead, as each prover will need to allocate extra memory to store the three blinded polynomial arrays. These arrays are of size `n`, where `n` is the size of the circuit, and previously these polynomials were simply sliced from the allocated `h(X)` polynomial without additional memory overhead.

### Solution

1. **Extend the `StatisticalZK` option** to control the blinding of all polynomials (e.g., `a`, `b`, `c`, `z`), not just the sub-polynomials of the quotient polynomial (`t shards`). This will reduce memory consumption when blinding is unnecessary and simplify the implementation.
2. Ensure that the size of the $h_3$ polynomial matches the size of $h_1$ and $h_2$ by setting its top coefficients to zero. This will eliminate some conditional logic and improve the readability of the code.

<details><summary>Code</summary>

```go

// computeQuotient computes H
func (s *instance) computeQuotient() (err error) {
	s.x[id_Ql] = s.trace.Ql
	s.x[id_Qr] = s.trace.Qr
	s.x[id_Qm] = s.trace.Qm
	s.x[id_Qo] = s.trace.Qo
	s.x[id_S1] = s.trace.S1
	s.x[id_S2] = s.trace.S2
	s.x[id_S3] = s.trace.S3

	for i := 0; i < len(s.commitmentInfo); i++ {
		s.x[id_Qci+2*i] = s.trace.Qcp[i]
	}

	n := s.domain0.Cardinality
	lone := make([]fr.Element, n)
	lone[0].SetOne()

	// wait for solver to be done
	select {
	case <-s.ctx.Done():
		return errContextDone
	case <-s.chLRO:
	}

	for i := 0; i < len(s.commitmentInfo); i++ {
		s.x[id_Qci+2*i+1] = s.cCommitments[i]
	}

	// wait for Z to be committed or context done
	select {
	case <-s.ctx.Done():
		return errContextDone
	case <-s.chZ:
	}

	// derive alpha
	if err = s.deriveAlpha(); err != nil {
		return err
	}

	// TODO complete waste of memory find another way to do that
	identity := make([]fr.Element, n)
	identity[1].Set(&s.beta)

	s.x[id_ID] = iop.NewPolynomial(&identity, iop.Form{Basis: iop.Canonical, Layout: iop.Regular})
	s.x[id_LOne] = iop.NewPolynomial(&lone, iop.Form{Basis: iop.Lagrange, Layout: iop.Regular})
	s.x[id_ZS] = s.x[id_Z].ShallowClone().Shift(1)

	numerator, err := s.computeNumerator()
	if err != nil {
		return err
	}

	s.h, err = divideByXMinusOne(numerator, [2]*fft.Domain{s.domain0, s.domain1})
	if err != nil {
		return err
	}

	// commit to h
	if err := commitToQuotient(s.h1(), s.h2(), s.h3(), s.proof, s.pk.Kzg); err != nil {
		return err
	}

	if err := s.deriveZeta(); err != nil {
		return err
	}

	// wait for clean up tasks to be done
	select {
	case <-s.ctx.Done():
		return errContextDone
	case <-s.chRestoreLRO:
	}

	close(s.chH)

	return nil
}

```

</details>

---

## 7. Recursive Verification

Recursive verification is a widely used technique in Zero-Knowledge Proofs (ZKPs), especially in scenarios involving multi-layer nested proofs. The core idea of recursive verification is to leverage recursion, where smaller proofs are verified to enable the validation of larger computations. This technique effectively reduces the size of proofs and computational complexity, making it especially useful for handling large-scale tasks such as validating smart contract execution on blockchains or ensuring the integrity of vast datasets.

However, vulnerabilities in recursive verification often stem from the complexity of validation, proof compression and construction, execution path control, and boundary checks. To mitigate the risk of these vulnerabilities, developers need to ensure that each layer of the recursive verification process is thoroughly validated and constrained, avoiding over-optimizations that could lead to security issues.

### Issue

Two key vulnerabilities, **unconstrained `committed_value_digest`** and the **missing `next_pc` check on the first layer of the recursive tree**, can be exploited together to allow an attacker to forge a proof:

1. **Unconstrained `committed_value_digest`**: In SP1 executors, the `COMMIT` system call (0x00_00_00_10) is typically issued at the end of program execution. Since the `COMMIT` call is the only event that constrains the `committed_value_digest` in the `ExecutionRecord`, if the program is aborted before the main function returns, the `COMMIT` call will not be triggered, leaving the `committed_value_digest` unconstrained. By default, it is set to zero, so attackers can modify the `committed_value_digest` to any arbitrary value during proof generation.
2. **Missing `next_pc` check on the first layer of the recursive tree**: In the recursive tree’s first layer verification code, if a shard is marked as "complete", the `next_pc == 0` condition is not checked. While this check exists in other parts of the recursive tree, it is absent in the first layer, potentially compromising proof validity. Although this vulnerability does not directly impact the verification process on its own, it can be exploited in combination with other flaws.

These two vulnerabilities enable an attacker to forge a proof as follows:

1. **SP1-2.1**: By aborting the program execution before the `COMMIT` system call is triggered, the `committed_value_digest` remains unconstrained. The attacker can replace it with any arbitrary value during proof generation.
2. **SP1-2.2**: By marking the `is_complete` flag as `true`, the attacker can cause the proof to pass validation, even if the `next_pc` value is inconsistent.

As a result, the attacker can generate a fake proof for an invalid execution and submit it through the validation program, which will erroneously accept the proof, even though it is not valid.

### Impact

- **Forged Proofs**: Attackers can generate counterfeit proofs to claim ownership of any blockchain address or private key, or even to falsify ownership of the "Bitcoin Genesis Block" address.
- **Potential Risks**: Since exploiting these vulnerabilities does not require altering the program itself—only modifying the proof client—this attack method can be widely applied to any guest program.

### Solution

1. **SP1-2.1**: Ensure that the `committed_value_digest` remains constrained, even if the `COMMIT` system call is not triggered. This may require redesigning the current implementation to ensure that the `committed_value_digest` cannot be arbitrarily modified when the program does not finish normally.

2. **SP1-2.2**: Fix the recursive program to apply the `next_pc == 0` constraint in the first layer of the recursive tree as well. As a temporary fix, the validation program should also check if `next_pc` equals 0, even if it is not constrained in the proof system.

<details><summary>Code</summary>

```rust

//! A program that takes a number `n` as input, and writes if `n` is prime as an output.
use sp1_sdk::{utils, ProverClient, SP1ProofWithPublicValues};

// Generated with `cargo prove build --docker --elf-name is-prime-write --output-directory elf`
// in the program directory
const ELF: &[u8] = include_bytes!("../../../program/elf/is-prime");
const FILENAME: &'static str = "42-is-prime.proof";

fn main() {
    // Setup a tracer for logging.
    utils::setup_logger();

    // Generate the verifying key from the ELF
    let client = ProverClient::new();
    let (_, vk) = client.setup(ELF);

    // Deserialize the proof
    let mut deserialized_proof =
        SP1ProofWithPublicValues::load(FILENAME).expect("loading proof failed");
    eprintln!("{deserialized_proof:?}");

    // Verify the deserialized proof.
    client
        .verify(&deserialized_proof, &vk)
        .expect("verification failed");

    // Now that it's accepted, read the primality boolean and the number from
    // the proof's public values and display them.
    let is_prime: bool = deserialized_proof.public_values.read();
    let n: u64 = deserialized_proof.public_values.read();
    println!("Verifier: Is {n} prime? {is_prime}");
}

...

//! A program that takes a number `n` as input, and writes if `n` is prime as an output.
use sp1_sdk::{utils, ProverClient, SP1ProofWithPublicValues};

// Generated with `cargo prove build --docker --elf-name is-prime-write --output-directory elf`
// in the program directory
const ELF: &[u8] = include_bytes!("../../program/elf/proof-of-address");
const FILENAME: &'static str = "genesis.proof";

fn main() {
    // Setup a tracer for logging.
    utils::setup_logger();

    // Generate the verifying key from the ELF
    let client = ProverClient::new();
    let (_, vk) = client.setup(ELF);

    // Deserialize the proof
    let mut deserialized_proof =
        SP1ProofWithPublicValues::load(FILENAME).expect("loading proof failed");

    // Verify the deserialized proof.
    client
        .verify(&deserialized_proof, &vk)
        .expect("verification failed");

    // Now that it's accepted, read the address from the proof's public values
    // and display it.
    let address: String = deserialized_proof.public_values.read();
    println!("Received proof of address for {address}!");
}

```

</details>

---

## 8. Forging ZK Proofs

The core function of ZK systems is to provide a short cryptographic proof that ensures an off-chain or private computation (such as a blockchain transaction) can be accepted and verified by a ZK proof verifier. **Forged ZK proofs** refer to vulnerabilities that allow attackers to submit counterfeit ZK proofs, deceiving the proof verifier into accepting a fraudulent transaction.

#### Issue

In zkWasm, memory heap retrieval is done through a set of `LoadN` instructions, where `N` represents the size of the data being loaded. For instance, `Load64` should retrieve 64 bits of data from the memory address, and `Load8` retrieves 8 bits (1 byte), filling the remaining 56 bits with leading zeros to create a 64-bit value. The internal memory representation in zkWasm is as an array of 64-bit words, requiring a segment of this array to be selected and using four intermediate variables (u16_cells) to form a complete 64-bit load value.

For example:

- `Load8(addr)` should read 1 byte (e.g., `0xAB`) → returns `0x00000000000000AB`
- `Load16(addr)` should read 2 bytes → upper 6 bytes are zeroed out to form a 64-bit value

The issue lies in the constraint code, which should enforce the following logic:

- **Load8**: The upper 56 bits (7 bytes) must be zeroed out → ensures only the 1 valid byte remains.
- **Load16**: The upper 48 bits (6 bytes) must be zeroed out → ensures only the 2 valid bytes remain.
- **Load32**: The upper 32 bits (4 bytes) must be zeroed out → ensures only the 4 valid bytes remain.

However, the constraints are incorrectly implemented. For example:

- The **Load8** constraint only ensures that bits 9–16 are zero, whereas the upper 56 bits (7 bytes) should have been zeroed out. This flaw allows attackers to alter ZK proofs by loading unexpected data, potentially leading to data corruption. These forged transactions will pass through zkWasm's verifier and be accepted on-chain.

For instance, if a 64-bit word in memory stores `0x1122334455667788`, a valid `Load8` should return `0x0000000000000088`. However, an attacker could manipulate unconstrained data to form a value like `0xAAAAAAAABBBB0088`. The system would incorrectly treat this as a valid result.

### Impact

- **Data Forgery**: Attackers can forge data by carefully crafting bitwise combinations, creating illegal transactions that are accepted by the verifier.
- **Logic Bypass**: The attacker can alter program execution flow, potentially triggering unauthorized operations.
- **Asset Theft**: In DeFi scenarios, attackers can forge transaction amounts, facilitating fraudulent transfers of assets.

Suppose the 64-bit word in memory is `0x1122334455667788`, and a legitimate `Load8` should return `0x0000000000000088`. An attacker, by manipulating the unverified high bits, can craft a payload like `0xAAAAAAAABBBB0088`, which the system incorrectly accepts.

### Solution

- **Fix Constraint Logic**: The constraints should be fixed to ensure that the upper bits are properly zeroed out. This is a typical **ZK circuit implementation flaw** that arises from **incomplete constraints** or **logical errors in constraint coding**. A localized fix can be implemented by updating the constraint code to enforce the correct bitwise conditions.

<details><summary>Code</summary>

```rust

// The constraints for these LoadN instructions are defined as follows:
constraint_builder.push(
    "op_load pick value size check",
    Box::new(move |meta| {
        vec![
            is_four_bytes.expr(meta)
                * (load_picked.u16_cells_le[2].expr(meta)
                    + load_picked.u16_cells_le[3].expr(meta),
            is_two_bytes.expr(meta)
                *(load_picked.expr(meta) - load_picked_leading_u16.expr(meta)),
            is_one_byte.expr(meta) *(load_picked_leading_u16_u8_high.expr(meta)),
        ]
    }),
);

```

</details>

---

## 9. Algebraic Attacks

In recent years, with the development of symmetric encryption techniques—particularly the emergence of arithmetic permutation (AOP) operations integrated with Zero-Knowledge Proof (ZKP) systems—new symmetric encryption algorithms have adopted larger field sizes and low multiplication depth designs, aiming to improve performance. However, these new designs have also introduced the risk of algebraic attacks. Algebraic attacks model encryption problems as systems of polynomial equations and utilize techniques like Gröbner basis methods to solve these equations, enabling attackers to break encryption keys or data. By optimizing the modeling and solving process of the equations, attackers can significantly reduce the difficulty of the attack, posing a serious threat to the security of modern symmetric encryption.

### Issue

One of the most powerful algebraic attacks currently known is the **FreeLunch method**. Its success demonstrates that even in multi-round encryption schemes, potential vulnerabilities exist that can be exploited effectively. The **FreeLunch method** is a novel algebraic attack that significantly improves the efficiency of solving these encryption problems. It applies to many recent permutation families that adopt arithmetic designs, such as Griffin, Anemoi, ArionHash, and XHash8, which rely on the difficulty of solving the Constrained Input-Constrained Output (CICO) problem for their security. The FreeLunch attack customizes the monomial ordering and equation generation process in such a way that solving the CICO problem using **Gröbner bases** incurs almost no additional cost. By carefully selecting the monomial ordering, the naturally occurring polynomial system (encoding the CICO problem) already forms a **Gröbner basis**—an algebraic tool.

Key innovations of the **FreeLunch method** include:

1. **Free Gröbner Basis**: Through clever monomial ordering and equation generation methods, the computation cost of calculating the Gröbner basis when solving the CICO problem is almost zero.
2. **Efficient Algorithm**: A new, more efficient algorithm has been designed to improve the **FGLM** algorithm (a commonly used reordering algorithm), making the FreeLunch attack significantly more practical compared to existing methods.

In experimental verification, the FreeLunch method has been successfully applied to multiple encryption primitives with reduced rounds. The results show that the complexity of the attack can be reduced to levels as low as 264 (for Griffin), 298 (for Arion), and 2118 (for Anemoi), despite these primitives claiming to offer 128-bit security.

### Impact

The FreeLunch attack is an algebraic attack strategy, outlined in Algorithm 1. The core of the attack is to generate a **FreeLunch system** and solve it through several steps:

1. **sysGen**: Generate a FreeLunch system. This process involves creating a polynomial system associated with the target encryption primitive (e.g., Griffin, Anemoi, etc.).
2. **matGen**: Compute the multiplication matrix $T_0$. In this step, the generated multiplication matrix is used in subsequent polynomial solving.
3. **polyDet**: Calculate the determinant of the polynomial $f(x_0)$, with the expression:

   $f(x_0) = \det(x_0^\alpha I_{DH} + \sum_{i=0}^{\alpha-1} x_0^i M_i)$

   This step involves computing a determinant, which is one of the most computationally intensive steps in the attack process.

4. **uniSol**: Solve the equation $f(x_0) = 0$. This is the final solving step, aiming to find the roots of the univariate polynomial. Compared to the previous steps, the complexity of the **uniSol** step is relatively low, estimated at $O(DI)$.

The overall complexity of the FreeLunch attack primarily depends on the **matGen** and **polyDet** steps. Experimental results show that in larger instances, **matGen** is typically the dominant step.

Regarding the **matGen** step, while no explicit complexity estimation is provided, it can be loosely bounded by the complexity of the **FGLM algorithm** ( $O(nD^3)$ ) as an upper bound. This upper bound is sufficient to break instances of Griffin and α-Arion. For example, for Griffin, the FGLM complexity is $2^{108}$ and $2^{122}$ (for $t \geq 12$ and $\alpha = 3, 5$, respectively). For α-Arion (with $\alpha = 121$ and $e = 3$), the complexity is $2^{117}$ and $2^{127}$ (for $t = 4, 5$). For $e = 5$, the complexity is $2^{114}$ and $2^{124}$ (for $t = 3, 4$).

For complete-round algorithms, the time complexity of the **polyDet** step (in base-2 logarithmic units) typically targets 128-bit security.

In summary, the FreeLunch attack is capable of breaking multiple modern symmetric encryption algorithms at much lower complexity than traditional methods, offering a potent attack vector.

### Solution

To defend against the FreeLunch attack, one approach is to increase the number of rounds in the encryption algorithm to raise the complexity of the attack. This is especially important because the **polyDet** step is usually the most computationally intensive part of the attack. Additionally, it's essential to remain vigilant against classical symmetric cryptographic attacks and improve the overall security of the algorithms.

Regarding the operational mode, the FreeLunch system is multi-variable, but a single variable ($x_0$) plays a unique role in the system. This makes it particularly well-suited to the CICO (Constrained Input-Constrained Output) problem, where a single output word must be set to 0. However, if multiple outputs are required to be zero, this approach becomes ineffective. Therefore, one simple defense against the FreeLunch attack (and single-variable attacks) is to enforce that at least two words in the sponge function's capacity be set to zero, even if it seems sufficient to only set one. While this measure provides some defense, its long-term effectiveness remains uncertain.

Future research could focus on optimizing the time estimation of the polynomial simplification step, reducing attack bottlenecks, and exploring multi-variable FreeLunch attack variants to further strengthen the security of symmetric encryption algorithms.

<details><summary>Code</summary>

```c++

/**
 * @brief build the matrix used in the linear layer of Griffin
*/
mp_limb_t ** build_matrix(int tt)
{
    mp_limb_t **MDS_LAYER = (mp_limb_t **)malloc(tt * sizeof(mp_limb_t *));
    for (int i = 0; i < tt; ++i)
        MDS_LAYER[i] = (mp_limb_t *)malloc(tt * sizeof(mp_limb_t));
    if (tt == 3)
    {
        for (int i = 0; i < 3; ++i)
            for (int j = 0; j < 3; ++j)
                MDS_LAYER[i][j] = M3[i][j];
    }

    if (tt == 4)
    {
        for (int i = 0; i < 4; ++i)
            for (int j = 0; j < 4; ++j)
                MDS_LAYER[i][j] = M4[i][j];
    }

    else
    {
        int tp = tt/4;
        for (int k = 0; k < tp; ++k)
        {
            for (int l = 0; l < tp; ++l)
            {
                mp_limb_t c = k == l ? 2 : 1;
                for (int i = 0; i < 4; ++i)
                    for (int j = 0; j < 4; ++j)
                        MDS_LAYER[4*k+i][4*l+j] = c * M4[i][j];
            }
        }
    }
    return MDS_LAYER;
}

/**
         * @brief build the MDS matrix
         * if t = 3, then M = M3 = [[2,1,1],[1,2,1],[1,1,2]]
         * if t = 4 then M = M4 = [[5,7,1,3],[4,6,1,1],[1,3,5,7],[1,1,4,6]]
         * if t = 4*t' >= 8 then M = [[2*M4  M4  ...  M4], [M4  2*M4  M4  ...  M4], ..., [M4  M4  ...  M4  2*M4]]
         */
        void build_mds()
        {
            this->MDS = Mat<T>(INIT_SIZE, t, t);

            if (t == 3)
            {
                for (int i = 0; i < 3; ++i)
                    for (int j = 0; j < 3; ++j)
                        this->MDS[i][j] = i == j ? 2 : 1;
            }

            else
            {
                // first build the matrix M'
                Mat<T> Mp(INIT_SIZE, t, t);
                SETNULL(Mp);
                int tp = t/4;

                int raw_M4[16] = {5,7,1,3,4,6,1,1,1,3,5,7,1,1,4,6};
                for (int b = 0; b < tp; ++b)
                    for (int i = 0; i < 4; ++i)
                        for (int j = 0; j < 4; ++j)
                            Mp[4*b + i][4*b + j] = raw_M4[4*i+j];

                if (t == 4)
                    this->MDS = Mp;

                else // then t = 4*t' >= 8
                {
                    Mat<T> Mpp(INIT_SIZE, t, t);

                    // build circ([2*I, I, ..., I])
                    for (int b1 = 0; b1 < tp; ++b1)
                        for (int b2 = 0; b2 < tp; ++b2)
                            for (int i = 0; i < 4; ++i)
                                for (int j = 0; j < 4; ++j)
                                    Mpp[4*b1+i][4*b2+j] = b1 == b2 ? (i == j ? 2 : 0) : (i == j ? 1 : 0);

                    this->MDS = Mp * Mpp;
                }
            }
        }

```

</details>

---

## 10. Protocol Vulnerabilities

Cryptographic protocol-level vulnerabilities fall under the category of reliability (soundness) flaws in Zero-Knowledge Proof (ZKP) systems. These are design-level issues rather than implementation errors, meaning they affect any system that uses the compromised protocol design, regardless of the specific implementation.

### Issue

The **Fiat-Shamir (FS) transformation** is a key cryptographic technique that turns interactive protocols into non-interactive ones. While it’s secure in theory under the Random Oracle Model (ROM), real-world implementations can have security risks. Previous research has shown weaknesses in FS, but mostly in artificial scenarios rather than real-world protocols.

There's a vulnerability in how the **Fiat-Shamir (FS) transformation** is used in the **Generalized Knowledge Replication (GKR) protocol**. An attacker can tweak a circuit, $C$, to create a modified version, $C'$, that can predict the verifier’s randomness. This allows them to generate valid proofs for false statements. Essentially, they can adjust the circuit to make it function the same but with a backdoor built in. This issue happens no matter what hash function is used.

In this protocol, the verifier checks whether there exists a witness $w$ such that:

$C(x, w) = y$

where $C$ is the circuit, $x$ is the public input, and $y$ is the claimed output. This proof system, or its variations, has been widely adopted in several studies and applied in practical systems such as **Expander**. Many of these studies assume that, when combined with the Fiat-Shamir transformation, the system is secure in the **Random Oracle Model (ROM)**. However, the reality is that **if the Fiat-Shamir hash function and the polynomial commitment scheme have roughly the same computational depth, the system is insecure in practice**. Specifically, attackers can craft **false proofs** that still pass validation, even if $y \neq C(x, w)$. This flaw affects a range of **succinct zero-knowledge proof (SNARK)** systems based on GKR and MLPCS, including actual ZK proof schemes like **Expander**.

#### Protocol Workflow ($\Pi_{comm,d}$) (Assuming Circuit Depth is $d$)

**Preprocessing Phase**:

1. The verifier selects parameters for the polynomial commitment scheme (i.e., generates a key/salt).
2. The prover and verifier agree to use a depth $d$ arithmetic circuit. The verifier only stores a short summary $\langle C \rangle$ (typically the hash of $C$).

**Online Phase**:

1. The prover selects input $x$, witness $w$, and output $y$, then:
   - Sends $x$ and $y$ directly to the verifier.
   - Uses the **MLPCS scheme** to commit to $w$, generating $comm(w)$ and sending it to the verifier.
2. The verifier selects a random point $r$, computes the **multilinear extension (MLE)** of the output $y$ at $r$, and sends $r$ to the prover.
3. **Using the GKR protocol**: The prover and verifier interact through the GKR protocol to reduce the computation about $y$ to the computation about $x$ and $w$. The verifier can check the correctness of $x$, but the correctness of $w$ requires verification via the MLPCS scheme. In practice, $x$ is often the parameterization of the circuit, and sometimes it can be omitted.

#### Fiat-Shamir Transformation to Non-Interactive Protocol

In the **Fiat-Shamir transformation (FS transformation)**, the **random challenge $r$** in the protocol is calculated using a hash function $h$, rather than being chosen randomly by the verifier. For example, in step 4 above:

$r = h(\langle C \rangle, comm(w), x, y)$

The protocol after the Fiat-Shamir transformation is denoted as $FSh(\Pi_{comm,d})$.

### Impact

This vulnerability undermines the basic security guarantees of Zero-Knowledge Proof systems, allowing the system to prove false statements. This issue affects any system that deploys this protocol, especially those involving recursive proof systems. In the worst case, it could cause the entire proof system to lose its trustworthiness.

### Solution

To mitigate this vulnerability, several strategies can be employed:

1. **Limit the depth of the circuit**: Restrict the computational depth of the circuit used in the protocol.
2. **Increase the computational depth of the hash function**: Use a hash function with greater computational depth to make the FS transformation more secure.
3. **Reduce the supported circuit depth**: Limiting the depth of circuits the protocol can handle could reduce the attack surface.
4. **Commit to intermediate values**: Add extra commitments for intermediate values, increasing the complexity of the protocol and improving security.
5. **Redesign the protocol**: Consider alternative methods to the Fiat-Shamir transformation, such as using additional safety checks or different cryptographic constructions.
6. **Evaluate the use case carefully**: In practical applications, carefully assess the security needs of the system and possibly accept some performance trade-offs for enhanced security.

The **Polyhedra team** has already implemented a fix, but this attack highlights a **fundamental security flaw** in the Fiat-Shamir transformation in certain environments.

<details><summary>Code</summary>

```rust

#[inline]
fn append_commitment(&mut self, commitment_bytes: &[u8]) {
    self.append_u8_slice(commitment_bytes);

    #[cfg(not(feature = "recursion"))]
    {
        // When appending the initial commitment, we hash the commitment bytes
        // for sufficient number of times, so that the FS hash has a sufficient circuit depth
        let mut digest = [0u8; 32];
        H::hash(&mut digest, commitment_bytes);
        for _ in 0..PCS_DIGEST_LOOP {
            H::hash_inplace(&mut digest);
        }
        self.append_u8_slice(&digest);
    }
}

#[inline]
/// check that the pcs digest in the proof is correct
fn append_commitment_and_check_digest<R: Read>(
    &mut self,
    commitment_bytes: &[u8],
    _proof_reader: &mut R,
) -> bool {
    self.append_u8_slice(commitment_bytes);

    #[cfg(not(feature = "recursion"))]
    {
        // When appending the initial commitment, we hash the commitment bytes
        // for sufficient number of times, so that the FS hash has a sufficient circuit depth
        let mut digest = [0u8; 32];
        H::hash(&mut digest, commitment_bytes);
        for _ in 0..PCS_DIGEST_LOOP {
            H::hash_inplace(&mut digest);
        }
        self.append_u8_slice(&digest);

        // check that digest matches the proof
        let mut pcs_digest = [0u8; 32];
        _proof_reader.read_exact(&mut pcs_digest).unwrap();

        digest == pcs_digest
    }
    #[cfg(feature = "recursion")]
    true
}

    #[inline]
    fn append_commitment(&mut self, commitment_bytes: &[u8]) {
        self.append_u8_slice(commitment_bytes);

        #[cfg(not(feature = "recursion"))]
        {
            // When appending the initial commitment, we hash the commitment bytes
            // for sufficient number of times, so that the FS hash has a sufficient circuit depth
            let mut challenge = self.generate_challenge_field_element().to_limbs();
            let hasher = H::new();
            for _ in 0..PCS_DIGEST_LOOP {
                challenge = hasher.hash_to_state(&challenge);
            }

            let mut digest_bytes = vec![];
            challenge.serialize_into(&mut digest_bytes).unwrap();
            self.append_u8_slice(&digest_bytes);
        }
    }

    #[inline]
    fn append_commitment_and_check_digest<R: Read>(
        &mut self,
        commitment_bytes: &[u8],
        _proof_reader: &mut R,
    ) -> bool {
        self.append_u8_slice(commitment_bytes);

        #[cfg(not(feature = "recursion"))]
        {
            // When appending the initial commitment, we hash the commitment bytes
            // for sufficient number of times, so that the FS hash has a sufficient circuit depth
            let mut challenge = self.generate_challenge_field_element().to_limbs();
            let hasher = H::new();
            for _ in 0..PCS_DIGEST_LOOP {
                challenge = hasher.hash_to_state(&challenge);
            }
            let mut digest_bytes = vec![];
            challenge.serialize_into(&mut digest_bytes).unwrap();
            self.append_u8_slice(&digest_bytes);

            // check that digest matches the proof
            let challenge_from_proof =
                Vec::<ChallengeF::BaseField>::deserialize_from(_proof_reader).unwrap();

            challenge_from_proof == challenge
        }

        #[cfg(feature = "recursion")]
        true
    }

```

</details>

---

<div  align="center">
<img src="img/06_zk_security.gif" width="50%" />
</div>
