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

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

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
