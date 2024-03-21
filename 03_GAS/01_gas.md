# ETAAcademy-Adudit: 1. Divisor

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. Gas</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>gas</th>
          <td>gas</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [Medium] An L1->L2 Transaction to be Sent that Does not Cover the Total Gas Limit Required

- Summary: An incorrect check allows an **`L1->L2`** transaction to be sent without covering the total gas limit required, including both overhead and intrinsic costs for the operator. **`{totalGasLimit} = {overhead + actualGasLimit} = {overhead + (intrinsicCosts + executionCosts)}`**
- Impact: This leads to situations where transactions may not have enough gas to be executed on L2, despite incurring overhead and intrinsic costs.
  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/1108) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

    require(
        getMinimalPriorityTransactionGasLimit(
            _encoded.length,
            _transaction.factoryDeps.length,
            _transaction.gasPerPubdataByteLimit
        ) <= _transaction.gasLimit,
        "up"
    );

    function getTransactionBodyGasLimit(
        uint256 _totalGasLimit,
        uint256 _gasPricePerPubdata,
        uint256 _encodingLength
    ) internal pure returns (uint256 txBodyGasLimit) {
        uint256 overhead = getOverheadForTransaction(_totalGasLimit, _gasPricePerPubdata, _encodingLength);
        require(_totalGasLimit >= overhead, "my"); // provided gas limit doesn't cover transaction overhead
        unchecked {
            txBodyGasLimit = _totalGasLimit - overhead;
        }
    }

    function processL1Tx(...){
        ...
        //gasLimitForTx is total - overhead (and some other intrinsic costs)
        let gasLimitForTx, reservedGas := getGasLimitForTx(...)
        ...
        canonicalL1TxHash, gasUsedOnPreparation := l1TxPreparation(txDataOffset)
        ...
    }   if gt(gasLimitForTx, gasUsedOnPreparation) {
            ...
            potentialRefund, success := getExecuteL1TxAndGetRefund(txDataOffset, sub(gasLimitForTx, gasUsedOnPreparation))


  ```

  </details>

## 2. [Medium] A Discrepancy in Units Used

- Summary: A discrepancy in units used for calculating transaction leads to the overhead being 32 times larger than it should be.
- Impact: Users may be charged significantly more than they should for certain transactions, causing potential financial implications and inaccuracies in cost assessments.
  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/1105) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

    //TransactionValidator.getOverheadForTransaction
    uint256 overheadForLength = Math.ceilDiv(_encodingLength * batchOverheadGas, BOOTLOADER_TX_ENCODING_SPACE);
    //bootloader.getTransactionUpfrontOverhead
    let overheadForLength := ceilDiv(
        safeMul(txEncodeLen, totalBatchOverhead, "ad"),
        BOOTLOADER_MEMORY_FOR_TXS()
    )

  ```

  </details>
