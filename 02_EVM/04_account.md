# ETAAcademy-Adudit: 4. Account

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>04. Account</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>EVM</th>
          <td>account</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [Medium] Nonce ordering of EOA can be updated to "arbitary" through an L1 tx

### EOA nonce ordering manipulated via L1 transaction

- Summary: This vulnerability enables the manipulation of an Externally Owned Account (EOA)'s nonce ordering to an arbitrary state through an L1 priority transaction, leading to the permanent freezing of the user's account.
- Impact & Recommendation: Once the nonce ordering is updated to an arbitrary state, it becomes permanent, rendering the account unable to initiate any new transactions.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-06-nonce-ordering-of-eoa-can-be-updated-to-arbitrary-through-an-l1-tx) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

    function _validateTransaction(
        bytes32 _suggestedSignedHash,
        Transaction calldata _transaction
    ) internal returns (bytes4 magic) {
        // Note, that nonce holder can only be called with "isSystem" flag.
        SystemContractsCaller.systemCallWithPropagatedRevert(
            uint32(gasleft()),
            address(NONCE_HOLDER_SYSTEM_CONTRACT),
            0,
            abi.encodeCall(INonceHolder.incrementMinNonceIfEquals, (_transaction.nonce))
        );
    function incrementMinNonceIfEquals(uint256 _expectedNonce) external onlySystemCall {
        uint256 addressAsKey = uint256(uint160(msg.sender));
        uint256 oldRawNonce = rawNonces[addressAsKey];
        (, uint256 oldMinNonce) = _splitRawNonce(oldRawNonce);
        require(oldMinNonce == _expectedNonce, "Incorrect nonce");
        unchecked {
            rawNonces[addressAsKey] = oldRawNonce + 1;
        }
    }

     // Checks whether the nonce `nonce` have been already used for
        // account `from`. Reverts if the nonce has not been used properly.
        function ensureNonceUsage(from, nonce, shouldNonceBeUsed) {
            // INonceHolder.validateNonceUsage selector
            mstore(0, {{RIGHT_PADDED_VALIDATE_NONCE_USAGE_SELECTOR}})
            mstore(4, from)
            mstore(36, nonce)
            mstore(68, shouldNonceBeUsed)
            let success := call(
                gas(),
                NONCE_HOLDER_ADDR(),
                0,
                0,
                100,
                0,
                0
            )
            if iszero(success) {
                revertWithReason(
                    ACCOUNT_TX_VALIDATION_ERR_CODE(),
                    1
                )
            }
        }
    function validateNonceUsage(address _address, uint256 _key, bool _shouldBeUsed) external view {
        bool isUsed = isNonceUsed(_address, _key);
        if (isUsed && !_shouldBeUsed) {
            revert("Reusing the same nonce twice");
        } else if (!isUsed && _shouldBeUsed) {
            revert("The nonce was not set as used");
        }
    }
    function isNonceUsed(address _address, uint256 _nonce) public view returns (bool) {
        uint256 addressAsKey = uint256(uint160(_address));
        return (_nonce < getMinNonce(_address) || nonceValues[addressAsKey][_nonce] > 0);
    }

    function _execute(Transaction calldata _transaction) internal {
        address to = address(uint160(_transaction.to));
        uint128 value = Utils.safeCastToU128(_transaction.value);
        bytes calldata data = _transaction.data;
        uint32 gas = Utils.safeCastToU32(gasleft());
        // Note, that the deployment method from the deployer contract can only be called with a "systemCall" flag.
        bool isSystemCall;
        if (to == address(DEPLOYER_SYSTEM_CONTRACT) && data.length >= 4) {
            bytes4 selector = bytes4(data[:4]);
            // Check that called function is the deployment method,
            // the others deployer method is not supposed to be called from the default account.
            isSystemCall =
                selector == DEPLOYER_SYSTEM_CONTRACT.create.selector ||
                selector == DEPLOYER_SYSTEM_CONTRACT.create2.selector ||
                selector == DEPLOYER_SYSTEM_CONTRACT.createAccount.selector ||
                selector == DEPLOYER_SYSTEM_CONTRACT.create2Account.selector;
        }

            function msgValueSimulatorMimicCall(to, from, value, dataPtr) -> success {
                // Only calls to the deployer system contract are allowed to be system
                let isSystem := eq(to, CONTRACT_DEPLOYER_ADDR())
                success := mimicCallOnlyResult(
                    MSG_VALUE_SIMULATOR_ADDR(),
                    from,
                    dataPtr,
                    0,
                    1,
                    value,
                    to,
                    isSystem
                )
            }

    function updateNonceOrdering(AccountNonceOrdering _nonceOrdering) external onlySystemCall {
        AccountInfo memory currentInfo = accountInfo[msg.sender];
        require(
            _nonceOrdering == AccountNonceOrdering.Arbitrary &&
                currentInfo.nonceOrdering == AccountNonceOrdering.Sequential,
            "It is only possible to change from sequential to arbitrary ordering"
        );
        currentInfo.nonceOrdering = _nonceOrdering;
        _storeAccountInfo(msg.sender, currentInfo);
        emit AccountNonceOrderingUpdated(msg.sender, _nonceOrdering);
    }


  ```

  </details>

## 2. [Medium] Divergences in the Simulation of the extcodehash EVM Opcode

### `keccak256("")` v.s. `bytes32(0)`

- Summary: In the zkSync Era, adherence to EIP-161 criteria determines whether an account is considered "empty" `bytes32(0)`, with no code, zero nonce, and zero balance. However, regardless of the account's balance, zkSync returns bytes32(0) for extcodehash, only considering the nonce and code presence, which diverges from keccak256("") for such accounts with no code in EVM.
- Impact & Recommendation: It accurately emulates the extcodehash EVM opcode as specified by EIP-1051. To mitigate this issue, a recommended solution is provided to precisely simulate the extcodehash EVM opcode based on EIP-1052.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-19-divergences-in-the-simulation-of-the-extcodehash-evm-opcode) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

  function getCodeHash(uint256 _input) external view override returns (bytes32) {
        address account = address(uint160(_input));
        if (uint160(account) <= CURRENT_MAX_PRECOMPILE_ADDRESS && account.balance != 0) {
            return EMPTY_STRING_KECCAK;
        } else if (uint160(account) <= CURRENT_MAX_PRECOMPILE_ADDRESS && address(account).balance == 0) {
            return bytes32(0);
        }
        bytes32 codeHash = getRawCodeHash(account);
        if (codeHash == 0x00 && NONCE_HOLDER_SYSTEM_CONTRACT.getRawNonce(account) > 0) {
            codeHash = EMPTY_STRING_KECCAK;
        }
        else if (Utils.isContractConstructing(codeHash)) {
            codeHash = EMPTY_STRING_KECCAK;
        } else if (codeHash == 0x00 && NONCE_HOLDER_SYSTEM_CONTRACT.getRawNonce(account) == 0 && address(account).balance != 0) {
            codeHash = EMPTY_STRING_KECCAK;
        }
        return codeHash;
    }

  ```

  </details>
