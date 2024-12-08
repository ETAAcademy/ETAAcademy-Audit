# ETAAcademy-Adudit: 2. Admin

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>02. Admin</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>Governance</th>
          <td>admin</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] Governance logic may enter a deadlock

### Key actors manipulate governance mechanism

- Summary: If compromised, key actors like the security council or owner could prevent upgrades or manipulate the governance mechanism, leading to operational challenges.

- Impact & Recommendation: To mitigate this the possibility of a governance deadlock , suggested solutions include restricting the cancel permission to only the owner, and implementing a minimum upgrade delay to prevent instant upgrades, giving users time to react in case of malicious actions.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-13-governance-logic-may-enter-a-deadlock) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

    -   function cancel(bytes32 _id) external onlyOwnerOrSecurityCouncil {
    +   function cancel(bytes32 _id) external onlyOwner {
            require(isOperationPending(_id), "Operation must be pending");
            delete timestamps[_id];
            emit OperationCancelled(_id);
        }

        function updateDelay(uint256 _newDelay) external onlySelf {
    +       require(_newDelay >= MINIMUM_UPGRADE_DELAY, "whatever";)
            emit ChangeMinDelay(minDelay, _newDelay);
            minDelay = _newDelay;
        }

  ```

  </details>

## 2.[Medium] Borrowers can avoid the payment of an interest share fee by setting themselves as a fee_receipient

### fee_receipient

- Summary: Borrowers can avoid the 20% interest share fee by specifying a fee recipient account they control, bypassing the intended fee payment mechanism.

- Impact & Recommendation: Apply restrictions on the fee_receipient by making it a property of the Pool struct, set during the creation of each new trading pool by its operator.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-lavarage#m-02-borrowers-can-avoid-the-payment-of-an-interest-share-fee-by-setting-themselves-as-a-fee_receipient) & [Report](https://code4rena.com/reports/2024-04-lavarage)

  <details><summary>POC</summary>

  ```rust
      let transfer_instruction3 = system_instruction::transfer(
    &ctx.accounts.trader.key(),
    &ctx.accounts.fee_receipient.key(),
    interest_share,
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_instruction3,
        &[
            ctx.accounts.trader.to_account_info(),
            ctx.accounts.fee_receipient.to_account_info(),
        ],
    )?;

  ```

  </details>

## 3.[High] Malicious contracts could become agents

### Agents set when proposed

- Summary: Once the proposal is processed, the malicious contract becomes an agent and can exploit critical functions protected by the `onlyAgent` modifier in the `masterDAO` contract, such as `updateContract`, `addDuty`, and `removeDuty`.

- Impact & Recommendation: To mitigate this issue, agents should only be set after a proposal has been resolved, not when it is proposed.
  <br> 🐬: [Source](https://audit.salusec.io/api/v1/salus/contract/certificate/full/Ink-Finance_audit_report_2023-08-15.pdf) & [Report](https://audit.salusec.io/api/v1/salus/contract/certificate/full/Ink-Finance_audit_report_2023-08-15.pdf)

  <details><summary>POC</summary>

  ```solidity
    function batchSetKV(
    address domain,
    ConfigHelper.KVInfo[] memory keyValueInfos
    ) external override {
        for (uint256 i = 0; i < keyValueInfos.length; i++) {
            if (
                hasRightToSet(
                    domain,
                    keyValueInfos[i].keyPrefix,
                    keyValueInfos[i].keyName
                )
            ) {
                _domainKeyValues.addKeyValue(domain, keyValueInfos[i]);
            }
            ...
        }
    }

  ```

  </details>

## 4.[Medium] Ambiguous PubdataPricingMode Configuration

### PubdataPricingMode between Rollup and Validium

- Summary: The setValidiumMode function in the Admin facet allows toggling PubdataPricingMode between Rollup and Validium freely, contrary to its name. The comment that Validium mode can only be set before the first batch is committed is incorrect because batches can be reverted, and the mode can be changed via changeFeeParams anytime.

- Impact & Recommendation: This inconsistency can lead to unpredictable L1 PubData charges and potential data leaks when switching from Validium to Rollup for privacy reasons. Ensure PubdataPricingMode can only change before the first batch is processed.
  <br> 🐬: [Source](https://blog.openzeppelin.com/zksync-state-transition-diff-audit#ambiguous-pubdatapricingmode-configuration) & [Report](https://blog.openzeppelin.com/zksync-state-transition-diff-audit)

  <details><summary>POC</summary>

  ```solidity

    /// @notice Change the token multiplier for L1->L2 transactions
    function setTokenMultiplier(uint128 _nominator, uint128 _denominator) external;

    /// @notice Change the pubdata pricing mode before the first batch is processed
    /// @param _validiumMode The new pubdata pricing mode
    function setPubdataPricingMode(PubdataPricingMode _pricingMode) external;

    /// @notice Perform the upgrade from the current protocol version with the corresponding upgrade data
    /// @param _protocolVersion The current protocol version from which upgrade is executed
    /// @param _cutData The diamond cut parameters that is executed in the upgrade
    function upgradeChainFromVersion(uint256 _protocolVersion, Diamond.DiamondCutData calldata _cutData) external;

  ```

  </details>
