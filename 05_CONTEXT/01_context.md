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

## 1. [Medium] Lack of access to ETH on L2 through L1->L2 transactions

### msg.value

- Summary : Users are unable to access their ETH stored on L2 through L1->L2 transactions, because the msg.value is generated solely from the ETH on Layer 1, not from the active balance of the user's account on Layer 2.
- Impact & Recommendation: Users cannot access their ETH on Layer 2 to withdraw funds from the rollup before a scheduled malicious upgrade, if a malicious operator only processes L1->L2 transactions, effectively trapping their funds.
  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/803) & [Report](https://code4rena.com/reports/2023-10-zksync)

## 2. [Medium] Vulnerabilities in Deposit Limit Enforcement and the Impact on Failed Deposits

### Deposit limit and track

- Summary: Users may struggle to claim failed deposits if a deposit limit is later imposed on a token, while malicious actors can exploit the system by intentionally failing deposits before limits are introduced, resetting their total deposited amount and exceeding caps once enforced.
- Impact & Recommendation: To mitigate these risks, the system should be updated to track deposited amounts regardless of existing limits, preventing difficulties in claiming failed deposits and thwarting attempts to bypass deposit restrictions.
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

## 3. [Medium] Synchronization Issue Between L1 and L2 Upgrades

### Protocol Version Discrepancy

- Summary: When an L2 upgrade fails but is executed on L1 without verifying its outcome, the protocol version advances despite the L2 system remaining unchanged, because the protocol mandates unique transaction hashes for L2 upgrades, with the nonce matching the new protocol version, causing a disparity between recorded and actual states.
- Impact & Recommendation:A potential solution involves integrating L2 upgrade outcomes into batch executions, allowing for a rollback of the protocol version if an upgrade fails. However, in cases involving both L1 and L2 components, directly reverting to a previous protocol version is challenging, as the L1 upgrade succeeds while the L2 counterpart encounters issues.
  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/214) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

  function executeBatches(StoredBatchInfo[] calldata _batchesData) external nonReentrant onlyValidator {
        //...
        uint256 batchWhenUpgradeHappened = s.l2SystemContractsUpgradeBatchNumber;
        if (batchWhenUpgradeHappened != 0 && batchWhenUpgradeHappened <= newTotalBatchesExecuted) {
            delete s.l2SystemContractsUpgradeTxHash;
            delete s.l2SystemContractsUpgradeBatchNumber;
            if (!proveL1ToL2TransactionStatus(...)){ // checking the L2 upgrade tx was successful or not
               s.protocolVersion = s.OldProtocolVersion; // assuming the old protocol version is stored
            }
        }
    }

  ```

  </details>
