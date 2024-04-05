# ETAAcademy-Adudit: 2. Block

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>02. Block</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>EVM</th>
          <td>block</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] Timestamp Constraints Leading to Number of Blocks Creation Limitations

### Different timestamp constraints between batches and blocks

- Summary : The constraints on timestamp differences between batches and their respective blocks in zkSync lead to smaller batch sizes, and prohibits the simultaneous commitment of two batches on L1 within the same Ethereum block, causing bottlenecks during high transaction volumes and block space utilization.
- Impact & Recommendation: The current timestamp verification process on L1 and L2 exacerbates these issues, necessitating stricter constraints to prevent batches with future timestamps. Mitigation steps should involve applying stricter timestamp constraints on both L1 and L2.
  <br> 🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/316) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

    Batch 1000:



    Batch timestamp: X + COMMIT_TIMESTAMP_APPROXIMATION_DELTA - 1.

    Timestamp of the last block (fictive block) in this batch: X + COMMIT_TIMESTAMP_APPROXIMATION_DELTA.

    The time this batch is committed on L1: blockTimestamp1000.

    X <= blockTimestamp1000.



    Batch 1001:



    Batch timestamp: X + COMMIT_TIMESTAMP_APPROXIMATION_DELTA + Y.

    Timestamp of the last block (fictive block) in this batch: X + COMMIT_TIMESTAMP_APPROXIMATION_DELTA + Y + K.

    The time this batch is committed on L1: blockTimestamp1001.

  ```

## 2.[Low] Chaining hashes with the wrong initial value

### `""` v.s. `0`

- Summary: Some contracts in L1 use 0, instead of using an empty string hash as the initial value when chaining hashes together
- Impact & Recommendation: It potentially causes issues or inconsistencies in the hashing process.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

    bytes32 reconstructedChainedLogsHash; // defaults to bytes32(0)

    function _collectOperationsFromPriorityQueue(uint256 _nPriorityOps) internal returns (bytes32 concatHash) {
        concatHash = EMPTY_STRING_KECCAK;
        for (uint256 i = 0; i < _nPriorityOps; i = i.uncheckedInc()) {
            PriorityOperation memory priorityOp = s.priorityQueue.popFront();
            concatHash = keccak256(abi.encode(concatHash, priorityOp.canonicalTxHash));
        }
    }



  ```

  </details>
