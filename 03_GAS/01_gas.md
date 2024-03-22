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

## 3. [Medium] L1->L2 Transaction Reverts but Consume all Gas

- Summary: When an L1->L2 transaction is initiated, zkSync employs a near call opcode to execute the transaction on the L2 network. This opcode is exempt from certain gas usage limitations, such as the 63/64 rule. The discrepancy arises from zkSync's failure to return unspent gas to the caller when a transaction fails due to a REVERT opcode, resembling the behavior of Ethereum's deprecated THROW opcode.
- Impact: L1->L2 transactions that revert will consume all gas, posing inconsistency with the EVM and potential risk for end users.

  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/979) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

  // SPDX-License-Identifier: MIT
    pragma solidity ^0.8.0;
    // PoC => No refund for gas on `L1->L2` tx failure, it always burns the gas even if not used
    import {Test} from "forge-std/Test.sol";
    import "forge-std/console.sol";
    import {DSTest} from "ds-test/test.sol";
    uint256 constant OVERHEAD_TX = 100_000; // assume overhead as 100000
    uint256 constant GAS_PREP = 2000; // assume preparation value
    contract ExternalContract {
        uint256 varState;
        function doSomething(uint256 num) external {
            varState = 1;
            //  revert if num is zero to cause nearCallPanic later
            if (num == 0) {
                revert("something wrong happened");
            }
        }
    }
    interface IExternalContract {
        function doSomething(uint256 num) external;
    }
    interface IBooloaderMock {
        function ZKSYNC_NEAR_CALL_SIMULATION_executeL1Tx(
            uint256 callAbi,
            bytes memory txCalldataEncoded
        ) external;
    }
    contract BooloaderMock {
        ExternalContract externalContract;
        constructor() {
            externalContract = new ExternalContract();
        }
        /// @dev The overhead in gas that will be used when checking whether the context has enough gas, i.e.
        /// when checking for X gas, the context should have at least X+CHECK_ENOUGH_GAS_OVERHEAD() gas.
        function CHECK_ENOUGH_GAS_OVERHEAD() internal pure returns (uint256 ret) {
            ret = 1000000;
        }
        function checkEnoughGas(uint256 gasToProvide) internal view {
            // Using margin of CHECK_ENOUGH_GAS_OVERHEAD gas to make sure that the operation will indeed
            // have enough gas
            // CHECK_ENOUGH_GAS_OVERHEAD => 1_000_000
            if (gasleft() < (gasToProvide + CHECK_ENOUGH_GAS_OVERHEAD())) {
                revert("No enough gas");
            }
        }
        function notifyExecutionResult(bool success) internal {}
        function nearCallPanic() internal pure {
            // Here we exhaust all the gas of the current frame.
            // This will cause the execution to panic.
            // Note, that it will cause only the inner call to panic.
            uint256 x = 0;
            while (true) {
                x += 1;
            }
        }
        // simulation of near call
        function ZKSYNC_NEAR_CALL_SIMULATION_executeL1Tx(
            uint256 callAbi,
            bytes memory txCalldataEncoded
        ) public {
            (bool success, ) = address(externalContract).call{gas: callAbi}(
                txCalldataEncoded
            );
            if (!success) {
                // nearCall panic
                nearCallPanic();
            }
        }
        function getExecuteL1TxAndGetRefund(
            uint256 gasForExecution,
            bytes memory txCalldataExternalContract
        ) internal returns (uint256 potentialRefund, bool success) {
            uint256 callAbi = gasForExecution;
            checkEnoughGas(gasForExecution);
            uint256 gasBeforeExecution = gasleft();
            bytes memory txCalldataEncoded = abi.encodeCall(
                IBooloaderMock.ZKSYNC_NEAR_CALL_SIMULATION_executeL1Tx,
                (callAbi, txCalldataExternalContract)
            );
            console.log("Nearcall callAbi: %d", callAbi);
            // pass 64/63 to simulate nearCall that doesn't follow this 63/64 rule
            uint256 fullGas = (callAbi * 64) / 63;
            (success, ) = address(this).call{gas: fullGas}(txCalldataEncoded);
            notifyExecutionResult(success);
            uint256 gasSpentOnExecution = gasBeforeExecution - gasleft();
            console.log("gasSpentOnExecution: %d", gasSpentOnExecution);
            if (gasSpentOnExecution <= gasForExecution) {
                potentialRefund = gasForExecution - gasSpentOnExecution;
            }
        }
        function processL1Tx(
            uint256 l2ValueProvidedByUser,
            uint256 gasLimitProvidedByUser,
            bytes memory txCalldataExternalContract
        ) external payable returns (uint256 potentialRefund, bool success) {
            uint256 overheadTX = OVERHEAD_TX; // assume overhead for simplicity
            uint256 gasLimitForTx = gasLimitProvidedByUser - overheadTX;
            uint256 gasUsedOnPreparation = GAS_PREP; // assume preparation value simplicity
            uint256 gasLimit = gasLimitProvidedByUser;
            uint256 gasPrice = 13e9;
            uint256 txInternalCost = gasPrice * gasLimit;
            require(
                msg.value >= l2ValueProvidedByUser + txInternalCost,
                "deposited eth too low"
            );
            require(gasLimitForTx > gasUsedOnPreparation, "Tx didn't continue");
            (potentialRefund, success) = getExecuteL1TxAndGetRefund(
                (gasLimitForTx - gasUsedOnPreparation),
                txCalldataExternalContract
            );
        }
    }
    contract BootloaderMockTest is DSTest, Test {
        BooloaderMock bootloaderMock;
        function setUp() public {
            bootloaderMock = new BooloaderMock();
            vm.deal(address(this),100 ether);
        }
        function test_no_gas_refund_on_failure() public {
            uint256 gasLimitByUser = 100_000_000 + OVERHEAD_TX + GAS_PREP;
            uint256 l2Value = 0;
            bytes memory txCalldataExternalContract = abi.encodeCall(
                IExternalContract.doSomething,
                (0) // value 0 cause the call to fail
            );
            (uint256 potentialRefund, bool success) = bootloaderMock.processL1Tx{
                value: 10 ether
            }(l2Value, gasLimitByUser, txCalldataExternalContract);
            console.log("success: ", success);
            console.log("potentialRefund: %d", potentialRefund);
        }
        function test_actual_gas_spent_on_success() public {
            uint256 gasLimitByUser = 100_000_000 + OVERHEAD_TX + GAS_PREP;
            uint256 l2Value = 0;
            bytes memory txCalldataExternalContract = abi.encodeCall(
                IExternalContract.doSomething,
                (1) // value 1 makes the call successful
            );
            (uint256 potentialRefund, bool success) = bootloaderMock.processL1Tx{
                value: 10 ether
            }(l2Value, gasLimitByUser, txCalldataExternalContract);
            console.log("success: ", success);
            console.log("potentialRefund: %d", potentialRefund);
        }
    }

  ```

  </details>
