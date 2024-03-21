# ETAAcademy-Adudit: 2. Under/Overflow

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. Under/Overflow</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>math</th>
          <td>under/overflow</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[High] Lack of Overflow Check

- Summary: This method converts a constraint system variable (representing a value in the prime field) directly into a **`UInt8`** value without performing any overflow checks.

- Impact: This means that if the original value exceeds the range of **`UInt8`** (0 to 255), an attacker could inject unexpected or malicious behavior into the circuit by manipulating the overflowed **`xor_result`**.
  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/679) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```rust

    let mut composite_result = [Variable::placeholder(); 32];
    for ((a, b), dst) in a.iter().zip(b.iter()).zip(composite_result.iter_mut()) {
        let [result] = cs.perform_lookup::<2, 1>(table_id, &[a.get_variable(), b.get_variable()]);
        *dst = result;
    }

    At first, we perform a lookup to get the composite result for and, or and xor.


    for (src, decomposition) in composite_result.iter().zip(all_results.array_chunks::<3>()) {
        if cs.gate_is_allowed::<ReductionGate<F, 4>>() {
            let mut gate = ReductionGate::<F, 4>::empty();
            gate.params = ReductionGateParams {
                reduction_constants: [F::SHIFTS[0], F::SHIFTS[16], F::SHIFTS[32], F::ZERO],
            };
            gate.reduction_result = *src;
            gate.terms = [
                decomposition[0],
                decomposition[1],
                decomposition[2],
                zero_var,
            ];
            gate.add_to_cs(cs);
        }


    for (((and, or), xor), src) in and_results
    .iter_mut()
    .zip(or_results.iter_mut())
    .zip(xor_results.iter_mut())
    .zip(all_results.array_chunks::<3>())
    {
    *and = src[0];
    *or = src[1];
    \*xor = src[2];
    }
    let and_results = and_results.map(|el| unsafe { UInt8::from_variable_unchecked(el) });
    let or_results = or_results.map(|el| unsafe { UInt8::from_variable_unchecked(el) });
    let xor_results = xor_results.map(|el| unsafe { UInt8::from_variable_unchecked(el) });
    Finally, we get three separate results from all_results.



    for source*set in all_results.array_chunks::<3>() {
    // value is irrelevant, it's just a range check
    let *: [Variable; 1] = cs.perform_lookup::<2, 1>(table_id, &[source_set[0], source_set[1]]);
    }


  ```

</details>
