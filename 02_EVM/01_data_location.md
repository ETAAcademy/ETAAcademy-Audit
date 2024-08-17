# ETAAcademy-Adudit: 1. Data Location

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. Data Location</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>EVM</th>
          <td>data_location</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[High] Attacker can manipulate the sorted queue in log sorter to emit reverted logs and events

## Manipulate the Sorted Queue in Log Sorter

- Summary: Enforce that the first popped element is write(only a write log, or a write log and a rollback log) and there are no two consecutive rollbacks in the sorted queue.
- Impact & Recommendation: Two adjacent letters share the same timestamp and the same written value. if someone submit `wr rw wr rw` as the sorted queue, All the four logs here are reverted, so no log should be added to the result queue. However, this sorted queue satisfy all the constraints, and it will add the second and the fourth log to the result queue.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#h-02-attacker-can-manipulate-the-sorted-queue-in-log-sorter-to-emit-reverted-logs-and-events) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```rust

        // We compare timestamps, and then resolve logic over rollbacks, so the only way when
        // keys are equal can be when we do rollback
        let sorting_key = sorted_item.timestamp;
        // ensure sorting for uniqueness timestamp and rollback flag
        // We know that timestamps are unique across logs, and are also the same between write and rollback
        let (keys_are_equal, new_key_is_smaller) =
            unpacked_long_comparison(cs, &[previous_key], &[sorting_key]);
        // keys are always ordered no matter what, and are never equal unless it's padding
        new_key_is_smaller.conditionally_enforce_false(cs, should_pop);

        // there are only two cases when keys are equal:
        // - it's a padding element
        // - it's a rollback
        // it's enough to compare timestamps as VM circuit guarantees uniqueness of the if it's not a padding
        let previous_is_not_rollback = previous_item.rollback.negated(cs);
        let enforce_sequential_rollback = Boolean::multi_and(
            cs,
            &[previous_is_not_rollback, sorted_item.rollback, should_pop],
        );
        keys_are_equal.conditionally_enforce_true(cs, enforce_sequential_rollback);

        let same_log = UInt32::equals(cs, &sorted_item.timestamp, &previous_item.timestamp);
        let values_are_equal =
            UInt256::equals(cs, &sorted_item.written_value, &previous_item.written_value);
        let negate_previous_is_trivial = previous_is_trivial.negated(cs);
        let should_enforce = Boolean::multi_and(cs, &[same_log, negate_previous_is_trivial]);
        values_are_equal.conditionally_enforce_true(cs, should_enforce);

        let this_item_is_non_trivial_rollback =
            Boolean::multi_and(cs, &[sorted_item.rollback, should_pop]);
        let negate_previous_item_rollback = previous_item.rollback.negated(cs);
        let prevous_item_is_non_trivial_write = Boolean::multi_and(
            cs,
            &[negate_previous_item_rollback, negate_previous_is_trivial],
        );
        let is_sequential_rollback = Boolean::multi_and(
            cs,
            &[
                this_item_is_non_trivial_rollback,
                prevous_item_is_non_trivial_write,
            ],
        );
        same_log.conditionally_enforce_true(cs, is_sequential_rollback);

        // decide if we should add the PREVIOUS into the queue
        // We add only if previous one is not trivial,
        // and it had a different key, and it wasn't rolled back
        let negate_same_log = same_log.and(cs, should_pop).negated(cs);
        let add_to_the_queue = Boolean::multi_and(
            cs,
            &[
                negate_previous_is_trivial,
                negate_same_log,
                negate_previous_item_rollback,
            ],
        );

  ```

  <details>

## 2. [High] Attacker can forge arbitary read value from memory in case skip_if_legitimate_fat_ptr

### Forge Arbitrary Read Value from Memory

- Summary: When overflow or offset `>=` length, the memory access should be skipped and return zeros to prevent potential manipulation of the read result by attackers.
- Impact & Recommendation: without performing memory reads and activating relevant memory access mechanisms, attackers could potentially manipulate the variables used in calculations.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#h-03-attacker-can-forge-arbitrary-read-value-from-memory-in-case-skip_if_legitimate_fat_ptr) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```rust

    let (_, offset_is_strictly_in_slice) = offset.overflowing_sub(cs, length);
    let offset_is_beyond_the_slice = offset_is_strictly_in_slice.negated(cs);
    let skip_if_legitimate_fat_ptr =
        Boolean::multi_and(cs, &[offset_is_beyond_the_slice, is_fat_ptr]);
    ......
    let skip_memory_access = Boolean::multi_or(
        cs,
        &[
            already_panicked,
            skip_if_legitimate_fat_ptr,
            is_non_addressable,
        ],
    );

    bytes_out_of_bound = bytes_out_of_bound.mask_negated(cs, skip_memory_access);
    bytes_out_of_bound = bytes_out_of_bound.mask_negated(cs, uf);
    let (_, bytes_out_of_bound) = bytes_out_of_bound.div_by_constant(cs, 32);
    // remainder fits into 8 bits too
    let bytes_to_cleanup_out_of_bounds =
        unsafe { UInt8::from_variable_unchecked(bytes_out_of_bound.get_variable()) };
    let new = Self {
        absolute_address,
        page_candidate: page,
        incremented_offset,
        heap_deref_out_of_bounds: is_non_addressable,
        skip_memory_access: skip_memory_access,
        should_set_panic,
        bytes_to_cleanup_out_of_bounds,
    };

    let apply_any = Boolean::multi_and(cs, &[should_apply, no_panic]);
    let update_dst0 = Boolean::multi_or(cs, &[is_read_access, is_write_access_and_increment]);
    let should_update_dst0 = Boolean::multi_and(cs, &[apply_any, update_dst0]);
    diffs_accumulator
        .dst_0_values
        .push((can_write_into_memory, should_update_dst0, dst0_value));
    This case is not treated specially and will not panic, so finally we will push it to dst0. (We should push zeros!)

    // implement shift register
    let zero_u8 = UInt8::zero(cs);
    let mut bytes_array = [zero_u8; 64];
    let memory_value_a_bytes = memory_value_a.value.to_be_bytes(cs);
    bytes_array[..32].copy_from_slice(&memory_value_a_bytes);
    let memory_value_b_bytes = memory_value_b.value.to_be_bytes(cs);
    bytes_array[32..].copy_from_slice(&memory_value_b_bytes);
    // now mask-shift
    let mut selected_word = [zero_u8; 32];
    // idx 0 is unalignment of 0 (aligned), idx 31 is unalignment of 31
    for (idx, mask_bit) in unalignment_bit_mask.iter().enumerate() {
        let src = &bytes_array[idx..(idx + 32)]; // source
        debug_assert_eq!(src.len(), selected_word.len());
        for (dst, src) in selected_word
            .array_chunks_mut::<4>()
            .zip(src.array_chunks::<4>())
        {
            *dst = UInt8::parallel_select(cs, *mask_bit, src, &*dst);
        }
    use crate::tables::uma_ptr_read_cleanup::UMAPtrReadCleanupTable;
    let table_id = cs
        .get_table_id_for_marker::<UMAPtrReadCleanupTable>()
        .expect("table must exist");
    let bytes_to_cleanup_out_of_bound = quasi_fat_ptr.bytes_to_cleanup_out_of_bounds;
    let bytes_to_cleanup_out_of_bound_if_ptr_read =
        bytes_to_cleanup_out_of_bound.mask(cs, is_uma_fat_ptr_read);
    let [uma_cleanup_bitspread, _] = cs.perform_lookup::<1, 2>(
        table_id,
        &[bytes_to_cleanup_out_of_bound_if_ptr_read.get_variable()],
    );
    let uma_ptr_read_cleanup_mask =
        Num::from_variable(uma_cleanup_bitspread).spread_into_bits::<_, 32>(cs);
    for (dst, masking_bit) in selected_word
        .iter_mut()
        .zip(uma_ptr_read_cleanup_mask.iter().rev())
    {
        *dst = dst.mask(cs, *masking_bit);
    }
    .......
    let dst0_value = VMRegister::conditionally_select(
        cs,
        is_write_access_and_increment,
        &incremented_src0_register,
        &read_value_as_register,
    );

    let should_read_a_cell = Boolean::multi_and(cs, &[should_apply, do_not_skip_memory_access]);
    let should_read_b_cell = is_unaligned_read;

    let table_id = cs
        .get_table_id_for_marker::<UMAPtrReadCleanupTable>()
        .expect("table must exist");
    let bytes_to_cleanup_out_of_bound = quasi_fat_ptr.bytes_to_cleanup_out_of_bounds;
    let bytes_to_cleanup_out_of_bound_if_ptr_read =
        bytes_to_cleanup_out_of_bound.mask(cs, is_uma_fat_ptr_read);
    let [uma_cleanup_bitspread, _] = cs.perform_lookup::<1, 2>(
        table_id,
        &[bytes_to_cleanup_out_of_bound_if_ptr_read.get_variable()],
    );
    let uma_ptr_read_cleanup_mask =
        Num::from_variable(uma_cleanup_bitspread).spread_into_bits::<_, 32>(cs);
    We don’t mask neither, since bytes_to_cleanup_out_of_b

  ```

  </details>

## 3. [Medium] Incorrect max precompile address

### ECADD and ECMUL Unrecognized as Precompiles

- Summary: The updated revision of ZkSync Era still refers to the old maximum precompile address, making the new precompiles **`ECADD`** and **`ECMUL`** unrecognized as precompiles due to their higher addresses, thus breaking the system's invariant.
- Impact & Recommendation: It causes unexpected behavior in the system where **`getCodeHash()`** returns zero instead of the expected hash value for these precompiles.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-04-incorrect-max-precompile-address) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

    describe('AccountCodeStorage', function() {
        it('fails to return correct hash for ECADD precompile', async () => {
            expect(await accountCodeStorage.getCodeHash('0x0000000000000000000000000000000000000006')).to.be.eq(
                EMPTY_STRING_KECCAK
            );
        });

        it('fails to return correct hash for ECMUL precompile', async () => {
            expect(await accountCodeStorage.getCodeHash('0x0000000000000000000000000000000000000007')).to.be.eq(
                EMPTY_STRING_KECCAK
            );
        });
    });

  ```

  </details>

## 4. [Medium] Wrong encoding of the data in the sendCompressedBytecode function

### Unsafe Arithmetic -> Incorrect Calldata

- Summary : The absence of checks on unsafe arithmetic operations opens the door for operators to manipulate data, enabling them to pass incorrect compressed calldata and manipulate gas costs, potentially resulting in end users being overcharged.
- Impact & Recommendation: This manipulation could lead to the insertion of incorrect or vulnerable data.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-07-wrong-encoding-of-the-data-in-the-sendcompressedbytecode-function) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

    4                               bytes : `publishCompressedBytecode` selector
    32                              bytes : offset for `_bytecode` parameter                                                  = V
    32                              bytes : offset for `_rawCompressedData` parameter                                         = V + 32 + rounded_len(_bytecode)
    (V - 64)                        bytes : any bytes that will be ignored in the `publishCompressedBytecode` function
    32                              bytes : length of `_bytecode` parameter                                                   = len(_bytecode)
    rounded_len(_bytecode)          bytes : `_bytecode` parameter                                                             = _bytecode
    32                              bytes : length of `_rawCompressedData` parameter                                          = len(_rawCompressedData)
    rounded_len(_rawCompressedData) bytes : `_rawCompressedData` parameter                                                    = _rawCompressedData

  ```

  </details>

## 5. [Medium] Version hash is not correctly enforced in code unpacker

### Constraint system

- Summary: In the code unpacker where the enforcement of the version hash is not correctly implemented by the constraint system.
- Impact & Recommendation: Any changes or updates to the version hash would not pass the validation process, rendering the system unable to accommodate future hash versions effectively.

  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-10-version-hash-is-not-correctly-enforced-in-code-unpacker) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```rust
    pub fn conditionally_enforce_true<CS: ConstraintSystem<F>>(
        &self,
        cs: &mut CS,
        should_enforce: Self,
    ) {
        // this is equal to having !self && should_enforce == false;
        // so (1 - self) * should_enforce == 0
        if cs.gate_is_allowed::<FmaGateInBaseFieldWithoutConstant<F>>() {
            let zero_var = cs.allocate_constant(F::ZERO);
            let gate = FmaGateInBaseFieldWithoutConstant {
                params: FmaGateInBaseWithoutConstantParams {
                    coeff_for_quadtaric_part: F::MINUS_ONE,
                    linear_term_coeff: F::ONE,
                },
                quadratic_part: (self.variable, should_enforce.variable),
                linear_part: should_enforce.variable,
                rhs_part: zero_var,
            };
            gate.add_to_cs(cs);
        } else {
            unimplemented!()
        }
    }

  ```

  </details>

## 6. [Medium] A cache that times out can be recovered

### Recover cache

- Summary: The `LocalCache#set_expire` function doesn't check if a key has expired before setting a timeout, allowing expired values to be reactivated. This could be exploited by malicious users to deceive others or cause inconsistencies in cached data. Similarly, the `LocalCache#get` function inconsistently handles expired keys, potentially allowing ttackers to trick victims into believing expired values no longer exist. This vulnerability could lead to various attack scenarios, including indefinite expiration of values deemed expired by developers.

- Impact & Recommendation: Ensures that expired values are promptly removed, reducing the risk of malicious exploitation and data inconsistency.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-phala-network#m-03-a-cache-that-times-out-can-be-recovered) & [Report](https://code4rena.com/reports/2024-03-phala-network)

  <details><summary>POC</summary>

  ```rust
        pub fn set_expire(&mut self, id: Cow<[u8]>, key: Cow<[u8]>, expire: u64) {
    -       self.maybe_clear_expired();
    +       self.clear_expired();
            if expire == 0 {
                let _ = self.remove(id.as_ref(), key.as_ref());
            } else if let Some(v) = self
                .storages
                .get_mut(id.as_ref())
                .and_then(|storage| storage.kvs.get_mut(key.as_ref()))
            {
                v.expire_at = now().saturating_add(expire)
            }
        }

  ```

  </details>

## 7. IMarket.execute.selector, `_checkSender` bypass allows to execute arbitrary operations

### Forge calldata by length

- Summary: An incorrect interpretation of calldata for the execute signature allows bypassing the `_checkSender` function, so that an attacker can forge it to match the value of a whitelisted or specially generated address by manipulating the length of calldata.

- Impact & Recommendation: Address the incorrect interpretation of calldata to prevent bypassing security checks and unauthorized execution in the market contract.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-tapioca#m-08-incorrect-return-value-of-function-basetapiocaomnichainengine_paynative) & [Report](https://code4rena.com/reports/2024-02-tapioca)

  <details><summary>POC</summary>

  ```solidity
    // SPDX-License-Identifier: MIT
    pragma solidity 0.8.22;
    import {Test} from "forge-std/Test.sol";
    import {console2} from "forge-std/console2.sol";
    contract MockCallerChecker {
    function doTheCheck(bytes calldata _actionCalldata) external {
        console2.log("Calldata Length", _actionCalldata.length);
        _checkSender(abi.decode(_actionCalldata[4:36], (address)));
    }
    function _checkSender(address entry) internal {
        console2.log("msg.sender", msg.sender);
        console2.log("entry", entry);
        require(msg.sender == entry);
    }
    }
    contract BasicTest is Test {
        // 4 bytes is funsig 0xaaaaaaaa
        // 32 bytes are the address (since abi.encoding uses a full word)
        // 0000000000000000000000000000000000000000111111111111111111111111
        bytes data = hex"aaaaaaaa0000000000000000000000000000000000000000111111111111111111111111";
        function testDemo() public {
        MockCallerChecker checker = new MockCallerChecker();
        console2.log(data.length);
        // Same address as the length
        vm.prank(address(0x111111111111111111111111));
        checker.doTheCheck(data);
        // For a real exploit, all we have to do is find the cheapest between available addresses and one we can mine
        }
    }

  ```

  </details>

## 8. [Medium] Unvalidated memory access in readMem and writeMem functions

### Validate memory address

- Summary: Unvalidated memory access in the `readMem()` and `writeMem()` functions do not enforce proper bounds checking on memory addresses. This oversight allows unauthorized access to memory outside the intended range, potentially enabling malicious users to read or write sensitive data, manipulate program flow, or disrupt memory allocation. Specifically, the lack of address validation could allow access to the entire 4GB address space. The proof of concept demonstrates how high memory addresses can be used for unauthorized operations.

- Impact & Recommendation: Implement strict bounds checks to limit accessible memory and prevent such exploits.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-07-optimism#m-04-Unvalidated-memory-access-in-readMem-and-writeMem-functions) & [Report](https://code4rena.com/reports/2024-07-optimism)

<details><summary>POC</summary>

```solidity

    function testUnauthorizedMemoryAccess() public {
        // Setup initial state
        bytes32 initialMemRoot = bytes32(uint256(1));
        uint32 initialPC = 0x1000;
        uint32 initialHeap = 0x40000000;
        bytes memory stateData = abi.encodePacked(
            initialMemRoot,    // memRoot
            bytes32(0),        // preimageKey
            uint32(0),         // preimageOffset
            initialPC,         // pc
            initialPC + 4,     // nextPC
            uint32(0),         // lo
            uint32(0),         // hi
            initialHeap,       // heap
            uint8(0),          // exitCode
            false,             // exited
            uint64(0),         // step
            uint32(0x8C020000) // lw $v0, 0($zero) - load word instruction
        );
        // Add register data (32 registers)
        for (uint i = 0; i < 32; i++) {
            stateData = abi.encodePacked(stateData, uint32(0));
        }
        // Create a proof that allows reading from any address
        bytes memory proof = new bytes(28 * 32);
        for (uint i = 0; i < 28; i++) {
            bytes32 proofElement = bytes32(uint256(i + 1));
            assembly {
                mstore(add(proof, mul(add(i, 1), 32)), proofElement)
            }
        }
        // Step 1: Read from a very high address (to out of bounds)
        uint32 highAddress = 0xFFFFFFFC;
        bytes memory maliciousStateData = abi.encodePacked(
            stateData,
            uint32(0x8C020000 | highAddress) // lw $v0, 0($zero) with high address
        );
        vm.expectRevert("Memory address out of bounds");
        mips.step(maliciousStateData, proof, bytes32(0));
        // Step 2: Write to a very high address (should be out of bounds)
        maliciousStateData = abi.encodePacked(
            stateData,
            uint32(0xAC020000 | highAddress) // sw $v0, 0($zero) with high address
        );
        vm.expectRevert("Memory address out of bounds");
        mips.step(maliciousStateData, proof, bytes32(0));
    }

```

</details>
