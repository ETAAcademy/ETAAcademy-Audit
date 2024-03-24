# ETAAcademy-Adudit: 1. Context

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. Context</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>context</th>
          <td>context</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [Medium] Nonce ordering of EOA can be updated to "arbitary" through an L1 tx

### EOA Nonce Ordering Manipulated via L1 Transaction

- Summary: This vulnerability enables the manipulation of an Externally Owned Account (EOA)'s nonce ordering to an arbitrary state through an L1 priority transaction, leading to the permanent freezing of the user's account.
- Impact: Once the nonce ordering is updated to an arbitrary state, it becomes permanent, rendering the account unable to initiate any new transactions.
  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/861) & [Report](https://code4rena.com/reports/2023-10-zksync)

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

## 2. [Medium] Lack of access to ETH on L2 through L1->L2 transactions

### msg.value

- Summary : Users are unable to access their ETH stored on L2 through L1->L2 transactions, because the msg.value is generated solely from the ETH on Layer 1, not from the active balance of the user's account on Layer 2.
- Impact: Users cannot access their ETH on Layer 2 to withdraw funds from the rollup before a scheduled malicious upgrade, if a malicious operator only processes L1->L2 transactions, effectively trapping their funds.
  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/803) & [Report](https://code4rena.com/reports/2023-10-zksync)

## 3. [Medium] Vulnerabilities in Deposit Limit Enforcement and the Impact on Failed Deposits

### Deposit Limit and Track

- Summary: Users may struggle to claim failed deposits if a deposit limit is later imposed on a token, while malicious actors can exploit the system by intentionally failing deposits before limits are introduced, resetting their total deposited amount and exceeding caps once enforced.
- Impact: To mitigate these risks, the system should be updated to track deposited amounts regardless of existing limits, preventing difficulties in claiming failed deposits and thwarting attempts to bypass deposit restrictions.
  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/425) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity
  function _verifyDepositLimit(address _l1Token, address _depositor, uint256 _amount, bool _claiming) internal {
        IAllowList.Deposit memory limitData = IAllowList(allowList).getTokenDepositLimitData(_l1Token);
        if (_claiming) {
            totalDepositedAmountPerUser[_l1Token][_depositor] -= _amount;
        } else {
            totalDepositedAmountPerUser[_l1Token][_depositor] += _amount;
      if(limitData.depositLimitation){
               require(totalDepositedAmountPerUser[_l1Token][_depositor] <= limitData.depositCap, "d1");
            }
        }
    }

  ```

  </details>
