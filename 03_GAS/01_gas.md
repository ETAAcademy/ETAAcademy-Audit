# ETAAcademy-Adudit: 1. Gas

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

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [Medium] TransactionValidator checks intrinsic costs against wrong value

### L1->L2 transaction sent without enough gas

- Summary: An incorrect check allows an **`L1->L2`** transaction to be sent without covering the total gas limit required, including both overhead and intrinsic costs for the operator. **`{totalGasLimit} = {overhead + actualGasLimit} = {overhead + (intrinsicCosts + executionCosts)}`**
- Impact & Recommendation: This leads to situations where transactions may not have enough gas to be executed on L2, despite incurring overhead and intrinsic costs.
  <br> 🐬: [Source]([M-01] TransactionValidator checks intrinsic costs against wrong value) & [Report](https://code4rena.com/reports/2023-10-zksync)

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

## 2. [Medium] Unit difference between transaction encoding and bootloader memory constant

### A discrepancy in units used

- Summary: A discrepancy in units used for calculating transaction leads to the overhead being 32 times larger than it should be.
- Impact & Recommendation: Users may be charged significantly more than they should for certain transactions, causing potential financial implications and inaccuracies in cost assessments.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-02-unit-difference-between-transaction-encoding-and-bootloader-memory-constant) & [Report](https://code4rena.com/reports/2023-10-zksync)

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

## 3. [Medium] Loss of funds for the sender when L1->L2 TX fails in the bootloader on L2

### L1->L2 transaction reverts but consume all gas

- Summary: When an L1->L2 transaction is initiated, zkSync employs a near call opcode to execute the transaction on the L2 network. This opcode is exempt from certain gas usage limitations, such as the 63/64 rule. The discrepancy arises from zkSync's failure to return unspent gas to the caller when a transaction fails due to a REVERT opcode, resembling the behavior of Ethereum's deprecated THROW opcode.
- Impact & Recommendation: L1->L2 transactions that revert will consume all gas, posing inconsistency with the EVM and potential risk for end users.

  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-03-loss-of-funds-for-the-sender-when-l1-l2-tx-fails-in-the-bootloader-on-l2) & [Report](https://code4rena.com/reports/2023-10-zksync)

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

## 4. [Medium] Operator can steal all gas provided by ANY user for L1 → L2 transactions

### Manipulate gas refund calculation

- Summary: Malicious operators abuse the gas refund system to stealing all gas provided by users for L1→L2 transactions, due to inadequate overflow checks in the refund calculation, allowing the operator to inflate the refundGas value.
- Impact & Recommendation: To mitigate this risk, the recommended solution is to replace the **`add`** function with **`safeAdd`** to ensure overflow checks are performed, preventing malicious operators from claiming more gas than provided by users, resulting in a loss of gas funds for them.

  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-14-operator-can-steal-all-gas-provided-by-any-user-for-l1l2-transactions) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```rust

    refundGas := add(refundGas, reservedGas) // overflow, refundGas = 0 while gasLimit != 0
    if gt(refundGas, gasLimit) { // correct, 0 < x for all x iff x != 0
        assertionError("L1: refundGas > gasLimit")
    }
    // gasPrice * (gasLimit - refundGas) == gasPrice * (gasLimit - 0) == gasPrice * gasLimit
    let payToOperator := safeMul(gasPrice, safeSub(gasLimit, refundGas, "lpah"), "mnk")

  ```

## 4. [Medium] Potential Gas Manipulation via Bytecode Compression

### Gas & Compression

- Summary: Gas consumption is determined by the length of the transmitted message, but malicious operators could exploit this by inflating gas costs through manipulation of the compression method, potentially leading to increased gas costs for message publication in L1 and undermining the intended efficiency and cost-effectiveness of the compression mechanism.
- Impact & Recommendation: The function **`publishCompressedBytecode`** is updated to include an array called **`usedDictionaryIndex`** to track the usage of dictionary chunks and ensure that all chunks in the dictionary are utilized.

  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-22-potential-gas-manipulation-via-bytecode-compression) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

  function publishCompressedBytecode(
        bytes calldata _bytecode,
        bytes calldata _rawCompressedData
    ) external payable onlyCallFromBootloader returns (bytes32 bytecodeHash) {
        unchecked {
            (bytes calldata dictionary, bytes calldata encodedData) = _decodeRawBytecode(_rawCompressedData);
            require(dictionary.length % 8 == 0, "Dictionary length should be a multiple of 8");
            require(dictionary.length <= 2 ** 16 * 8, "Dictionary is too big");
            require(
                encodedData.length * 4 == _bytecode.length,
                "Encoded data length should be 4 times shorter than the original bytecode"
            );
            // This code is added
            bool[] memory usedDictionaryIndex = new bool[](
                dictionary.length / 8
            );
            //////////////////////
            for (uint256 encodedDataPointer = 0; encodedDataPointer < encodedData.length; encodedDataPointer += 2) {
                uint256 indexOfEncodedChunk = uint256(encodedData.readUint16(encodedDataPointer)) * 8;
                require(indexOfEncodedChunk < dictionary.length, "Encoded chunk index is out of bounds");
                // This code is added
                usedDictionaryIndex[indexOfEncodedChunk] = true;
                //////////////////////
                uint64 encodedChunk = dictionary.readUint64(indexOfEncodedChunk);
                uint64 realChunk = _bytecode.readUint64(encodedDataPointer * 4);
                require(encodedChunk == realChunk, "Encoded chunk does not match the original bytecode");
            }
            // This code is added
            for (uint256 i = 0; i < usedDictionaryIndex.length; ++i) {
                require(
                    usedDictionaryIndex[i],
                    "the dictionary includes chunks that are useless"
                );
            }
            //////////////////////
        }
        bytecodeHash = Utils.hashL2Bytecode(_bytecode);
        L1_MESSENGER_CONTRACT.sendToL1(_rawCompressedData);
        KNOWN_CODE_STORAGE_CONTRACT.markBytecodeAsPublished(bytecodeHash);
    }

  ```

  </details>

## 5. [Medium] Incorrect gas claiming logic in ThrusterPoolDeployer

### Blast gas claim logic

- Summary: The ThrusterPoolDeployer contract has a flaw in its claimGas() implementation. It incorrectly attempts to claim gas for the zero address instead of its own address. This prevents the contract from reclaiming any gas, as the claimMaxGas() call always fails with the zero address, according to the Blast gas claim logic.

- Impact & Recommendation: Use `address(this)` rather than `0`.

  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-thruster#m-04-incorrect-gas-claiming-logic-in-thrusterpooldeployer) & [Report](https://code4rena.com/reports/2024-02-thruster)

  <details><summary>POC</summary>

  ```solidity
  /**
     * @notice Claims gas available to be claimed at max claim rate for a specific contract. Called by an authorized user
    * @param contractAddress The address of the contract for which maximum gas is to be claimed
    * @param recipientOfGas The address of the recipient of the gas
    * @return The amount of gas that was claimed
    */
    function claimMaxGas(address contractAddress, address recipientOfGas) external returns (uint256) {
        require(isAuthorized(contractAddress), "Not allowed to claim max gas");
        return IGas(GAS_CONTRACT).claimMax(contractAddress, recipientOfGas);
    }

  ```

  </details>

## 6. [High] Gas issuance is inflated and will halt the chain or lead to incorrect base fee

### inflated gas calculation

- Summary: The `anchor()` function's base fee calculation is flawed, leading to inflated issuance. If called consecutively on 5 blocks, it erroneously issues 15 times the gas target per L1 block instead of the expected 5 times, potentially causing the chain to halt or suffer from a significantly deflated base fee.

- Impact & Recommendation: Issue `_config.gasTargetPerL1Block` for each L1 block instead of issuing `uint256 issuance = (_l1BlockOd - lastSyncedBlock) * _config.gasTargetPerL1Block`.

<br> 🐬: [Source](https://code4rena.com/reports/2024-03-taiko#h-01-gas-issuance-is-inflated-and-will-halt-the-chain-or-lead-to-incorrect-base-fee) & [Report](https://code4rena.com/reports/2024-03-taiko)

  <details><summary>POC</summary>
 
  ```solidity
      struct Config {
        uint32 gasTargetPerL1Block;
        uint8 basefeeAdjustmentQuotient;
    }
    function getConfig() public view virtual returns (Config memory config_) {
        config_.gasTargetPerL1Block = 15 * 1e6 * 4;
        config_.basefeeAdjustmentQuotient = 8;
    }
    uint256 lastSyncedBlock = 1;
    uint256 gasExcess = 10;
    function _calc1559BaseFee(
        Config memory _config,
        uint64 _l1BlockId,
        uint32 _parentGasUsed
    )
        private
        view
        returns (uint256 issuance, uint64 gasExcess_)
    {
        if (gasExcess > 0) {
            uint256 excess = uint256(gasExcess) + _parentGasUsed;
            uint256 numL1Blocks;
            if (lastSyncedBlock > 0 && _l1BlockId > lastSyncedBlock) {
                numL1Blocks = _l1BlockId - lastSyncedBlock;
            }
            if (numL1Blocks > 0) {
                issuance = numL1Blocks * _config.gasTargetPerL1Block;
                excess = excess > issuance ? excess - issuance : 1;
            }
			// I have commented out the below basefee calculation
			// and return issuance instead to show the actual
			// accumulated issuance over 5 L1 blocks.
			// nothing else is changed
		
            //gasExcess_ = uint64(excess.min(type(uint64).max));
			
            //basefee_ = Lib1559Math.basefee(
            //    gasExcess_, uint256(_config.basefeeAdjustmentQuotient) * _config.gasTargetPerL1Block
            //);
        }
        //if (basefee_ == 0) basefee_ = 1;
    }
        
    function testIssuance() external {
        uint256 issuance;
        uint256 issuanceAdded;
        Config memory config = getConfig();
        for (uint64 x=2; x <= 6 ;x++){
            
            (issuanceAdded ,) = _calc1559BaseFee(config, x, 0);
            issuance += issuanceAdded;
            console2.log("added", issuanceAdded);
        }
        uint256 expectedIssuance = config.gasTargetPerL1Block*5;
        console2.log("Issuance", issuance);
        console2.log("Expected Issuance", expectedIssuance);
        
        assertEq(expectedIssuance*3, issuance);
  
  ```
  </details>

## 7. [High] paymaster will refund spentOnPubdata to user

### less gas calculation

- Summary: A recent update modifies how GAS spent on pubdata is collected at the transaction's final step. However, when a paymaster is involved, the `_maxRefundedGas` value calculated during `paymaster.postTransaction(_maxRefundedGas)` does not subtract the spent gas `spentOnPubdata` on pubdata, which leads to an overestimation of `_maxRefundedGas` refunding more than necessary.

- Impact & Recommendation: Subtract the spentOnPubdata from the total gas calculation during the post-operation refund.

<br> 🐬: [Source](https://code4rena.com/reports/2024-03-zksync#h-01-paymaster-will-refund-spentonpubdata-to-user) & [Report](https://code4rena.com/reports/2024-03-zksync)

<details><summary>POC</summary>

```solidity
            function refundCurrentL2Transaction(
                txDataOffset,
                transactionIndex,
                success,
                gasLeft,
                gasPrice,
                reservedGas,
                basePubdataSpent,
                gasPerPubdata
            ) -> finalRefund {
                setTxOrigin(BOOTLOADER_FORMAL_ADDR())
                finalRefund := 0
                let innerTxDataOffset := add(txDataOffset, 32)
                let paymaster := getPaymaster(innerTxDataOffset)
                let refundRecipient := 0
                switch paymaster
                case 0 {
                    // No paymaster means that the sender should receive the refund
                    refundRecipient := getFrom(innerTxDataOffset)
                }
                default {
                    refundRecipient := paymaster
+                   let expectSpentOnPubdata := getErgsSpentForPubdata(
+                        basePubdataSpent,
+                        gasPerPubdata
+                    )
                    if gt(gasLeft, 0) {
                        checkEnoughGas(gasLeft)
                        let nearCallAbi := getNearCallABI(gasLeft)
                        let gasBeforePostOp := gas()
                        pop(ZKSYNC_NEAR_CALL_callPostOp(
                            // Maximum number of gas that the postOp could spend
                            nearCallAbi,
                            paymaster,
                            txDataOffset,
                            success,
                            // Since the paymaster will be refunded with reservedGas,
                            // it should know about it
-                           safeAdd(gasLeft, reservedGas, "jkl"),
+                           saturatingSub(add(reservedGas, gasLeft), expectSpentOnPubdata),
                            basePubdataSpent,
                            reservedGas,
                            gasPerPubdata
                        ))
                        let gasSpentByPostOp := sub(gasBeforePostOp, gas())
                        gasLeft := saturatingSub(gasLeft, gasSpentByPostOp)
                    }
                }
                // It was expected that before this point various `isNotEnoughGasForPubdata` methods would ensure that the user
                // has enough funds for pubdata. Now, we just subtract the leftovers from the user.
                let spentOnPubdata := getErgsSpentForPubdata(
                    basePubdataSpent,
                    gasPerPubdata
                )
                let totalRefund := saturatingSub(add(reservedGas, gasLeft), spentOnPubdata)
                askOperatorForRefund(
                    totalRefund,
                    spentOnPubdata,
                    gasPerPubdata
                )
                let operatorProvidedRefund := getOperatorRefundForTx(transactionIndex)
                // If the operator provides the value that is lower than the one suggested for
                // the bootloader, we will use the one calculated by the bootloader.
                let refundInGas := max(operatorProvidedRefund, totalRefund)
                // The operator cannot refund more than the gasLimit for the transaction
                if gt(refundInGas, getGasLimit(innerTxDataOffset)) {
                    assertionError("refundInGas > gasLimit")
                }
                if iszero(validateUint32(refundInGas)) {
                    assertionError("refundInGas is not uint32")
                }
                let ethToRefund := safeMul(
                    refundInGas,
                    gasPrice,
                    "fdf"
                )
                directETHTransfer(ethToRefund, refundRecipient)
                finalRefund := refundInGas

```

<details>

## 8.[Medium] calculateTVL may run out of gas for modest number of operators and tokens breaking deposits, withdrawals, and trades

### Gas for nested loops

- Summary: The `calculateTVLs` function in the `RestakeManager` suffers from high gas consumption due to its nested loops, which iterate over each operator delegator (OD) and each token. This results in quadratic gas costs, with each internal loop calling expensive functions and allocating memory. For a small number of ODs and tokens, this function already consumes significant gas, and for larger numbers, it can exceed the block gas limit, making the protocol unusable.
- Impact & Recommendation: Instead of continuously querying each operator delegator (OD) for token balances, a more efficient "push" pattern can be implemented.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-renzo#m-05-calculatetvl-may-run-out-of-gas-for-modest-number-of-operators-and-tokens-breaking-deposits-withdrawals-and-trades) & [Report](https://code4rena.com/reports/2024-04-renzo)

## 9.[Medium] Cross Domain Messengers Can Fail in Relaying a Message

### Insufficient gas

- Summary: The `L1CrossDomainMessenger` contract's `relayMessage` function can fail due to inadequate gas estimates for additional instructions like ERC-20 token approvals. The original gas check function, `hasMinGas`, adds 40,000 units of gas to account for worst-case scenarios, but this buffer is insufficient when extra operations are introduced. These additional instructions, such as token approvals, can consume much more gas, leading to potential message relay failures and opening the door for DoS attacks.

- Impact & Recommendation: It is recommended to revisit the `RELAY_GAS_CHECK_BUFFER` and `RELAY_RESERVED_GAS` values and assess the necessity of repeated approvals to ensure sufficient gas overhead.
  <br> 🐬: [Source](https://blog.openzeppelin.com/mantle-v2-solidity-contracts-audit#cross-domain-messengers-can-fail-in-relaying-a-message) & [Report](https://blog.openzeppelin.com/mantle-v2-solidity-contracts-audit)

<details><summary>POC</summary>

```solidity
    /**
     * @notice Gas reserved for finalizing the execution of `relayMessage` after the safe call.
     */
    uint64 public constant RELAY_RESERVED_GAS = 90_000;

    /**
     * @notice Gas reserved for the execution between the `hasMinGas` check and the external
     *         call in `relayMessage`.
     */
    uint64 public constant RELAY_GAS_CHECK_BUFFER = 55_000;

    /**
     * @notice BASE gas reserved for Hashing.hashCrossDomainMessage

```

</details>

## 10.[High] Malicious operators can bypass checks in DSS Hooks

### Insufficient gas

- Summary: When an operator registers and stakes in the DSS, the function `callHookIfInterfaceImplemented` checks if the DSS supports the required interface and then calls the corresponding hook. The issue arises because a low-level call is used to check for the interface support and execute the hook, which can be manipulated by supplying insufficient gas. Specifically, a malicious operator can provide a very low gas limit for the low-level call, causing it to fail due to out-of-gas (OOG) error. The protocol does not handle this failure correctly and assumes the DSS does not implement the hook, allowing the operator to bypass essential registration and stake update checks.

- Impact & Recommendation: it is recommended to ensure sufficient gas is available before making the low-level call in `callHookIfInterfaceImplemented`.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-karak-pro-league#h-02-malicious-operators-can-bypass-checks-in-dss-hooks) & [Report](https://code4rena.com/reports/2024-06-karak-pro-league)

<details><summary>POC</summary>

```solidity
    function callHookIfInterfaceImplemented(
        IERC165 dss,
        bytes memory data,
        bytes4 interfaceId,
        bool ignoreFailure,
        uint256 gas
    ) internal returns (bool) {
+       if (gasleft() < Constants.SUPPORTS_INTERFACE_GAS_LIMIT ) revert NotEnoughGas();
        (bool success,) = address(dss).call{gas: Math.min(Constants.SUPPORTS_INTERFACE_GAS_LIMIT, gasleft())}(
            abi.encodeWithSelector(IERC165.supportsInterface.selector, interfaceId)
        );
        if (!success) {
            emit InterfaceNotSupported();
            return false;
        }
        return callHook(address(dss), data, ignoreFailure, gas);
    }

        function registerOperatorToDSS(State storage self, IDSS dss, address operator, bytes memory registrationHookData)
        external
    {
        if (self.dssMap.length() == Constants.MAX_DSS_PER_OPERATOR) revert MaxDSSCapacityReached();
        self.dssMap.set(address(dss), 1); // Set a non zero value for dss
        HookLib.callHookIfInterfaceImplemented(
            dss,
            abi.encodeWithSelector(dss.registrationHook.selector, operator, registrationHookData),
            dss.registrationHook.selector,
            false,
            Constants.DEFAULT_HOOK_GAS
        );
    }
```

</details>

## 11.[Medium] Adding staking instance as nominee before it is created

### Unnecessary iteration

- Summary: An attacker could add a staking instance as a nominee before it is created, causing unnecessary iteration over epochs when claiming incentives and leading to excessive gas consumption. By predicting the address of future instances, attackers can add them as nominees early on, forcing the system to process irrelevant epochs when the instance is eventually created and claims rewards.

- Impact & Recommendation: Ensure nominees are already created before being added and incorporating the `initPayload` into the address generation process to prevent such prediction-based attacks.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-olas#m-03-adding-staking-instance-as-nominee-before-it-is-created) & [Report](https://code4rena.com/reports/2024-05-olas)

<details><summary>POC</summary>

```solidity

    function addNomineeEVM(address account, uint256 chainId) external {
        require(IStakingFactory.mapInstanceParams(account).implementation != address(0), " the nominee is not created yet");
        // ....
    }

    function createStakingInstance(
        address implementation,
        bytes memory initPayload
    ) external returns (address payable instance) {
        //.....
        bytes32 salt = keccak256(abi.encodePacked(block.chainid, localNonce, keccak256(initPayload)));
        //.....
    }

```

</details>

## 12.[Medium] Gas griefing/attack via creating the proposals

### Gas attack

- Summary: The core of the problem lies in the protocol’s behavior that requires proposals to be either executed or canceled. When keys are compromised, attackers can send numerous transactions consuming gas, forcing the victims to spend a significant amount of gas to cancel these proposals. The defense mechanisms available—using pause guardians or cold signers to cancel proposals—also require the victims to pay gas fees. If attackers send enough transactions, the victim may not have enough funds to cancel all proposals, allowing the attackers to drain the vault.

- Impact & Recommendation: Implement an epoch system in the timelock contract. Whenever the contract is paused, the epoch should advance to the next variable. Only transactions from the current epoch should be executable, making all previous proposals automatically invalid once the epoch changes.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-10-kleidi#m-01-gas-griefingattack-via-creating-the-proposals) & [Report](https://code4rena.com/reports/2024-10-kleidi)

<details><summary>POC</summary>

```solidity

    function testGasConsumption() public {
        bytes32 scheduleSalt = bytes32("saltxyz");
        uint256 numOfProposals = 100000;
        bytes32[] memory saltArray = new bytes32[](numOfProposals);
        for(uint i; i < numOfProposals; i++) {
            saltArray[i] = keccak256(abi.encodePacked("salt", bytes32(i + 1)));
        }
        bytes memory scheduleData = abi.encode(timelock.updateDelay, MINIMUM_DELAY);
        address timelockAddress = address(timelock);
        // initial call costs more gas
        vm.prank(address(safe));
        timelock.schedule(
            timelockAddress,
            0,
            scheduleData,
            scheduleSalt,
            MINIMUM_DELAY
        );
        vm.startPrank(address(safe));
        uint256 gasBeforeSchedule = gasleft();
        for(uint256 i; i < numOfProposals; i++){
            timelock.schedule(
                timelockAddress,
                0,
                scheduleData,
                saltArray[i],
                MINIMUM_DELAY
            );
        }
        uint256 gasAfterSchedule = gasleft();
        vm.stopPrank();
        bytes32[] memory ids = new bytes32[](numOfProposals);
        for(uint256 i; i < numOfProposals; i++){
            ids[i] = timelock.hashOperation(
                address(timelock),
                0,
                scheduleData,
                saltArray[i]
            );
        }
        vm.startPrank(timelock.pauseGuardian());
        uint256 gasBeforeCancel = gasleft();
        timelock.pause(); // 10000 -> 32,260,154 4.6%
        uint256 gasAfterCancel = gasleft();
        vm.stopPrank();
        // vm.startPrank(address(safe));
        // uint256 gasBeforeCancel = gasleft();
        // for(uint256 i; i < numOfProposals; i++){
        //     timelock.cancel(ids[i]); // 10000 -> 44,890,040  448,900,040 6%
        // }
        // uint256 gasAfterCancel = gasleft();
        // vm.stopPrank();
        // For 100,000 proposals
        // shecdule 7,398,200,040
        // pause guardian pause 340,048,201 ~ 4.6%
        // safe cancel 448,900,040 ~ 6%
        console.log("Gas consumption of schedule: ", gasBeforeSchedule - gasAfterSchedule); // 10000 -> 739,820,040 7,398,200,040
        console.log("Gas consumption of cancel: ", gasBeforeCancel - gasAfterCancel);
    }

```

</details>
