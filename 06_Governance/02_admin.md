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
  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/260) & [Report](https://code4rena.com/reports/2023-10-zksync)

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
