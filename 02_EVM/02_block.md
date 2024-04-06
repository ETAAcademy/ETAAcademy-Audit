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
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-12-timestamp-constraints-leading-to-number-of-blocks-creation-limitations) & [Report](https://code4rena.com/reports/2023-10-zksync)

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

  </details>
