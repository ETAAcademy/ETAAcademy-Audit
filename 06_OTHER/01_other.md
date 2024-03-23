# ETAAcademy-Adudit: 1. Context

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. Other</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>Other</th>
          <td>Other</td>
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
- Impact: Users cannot access their ETH on Layer 2 to withdraw funds from the rollup before a scheduled malicious upgrade, if a malicious operator only processes L1->L2 transactions, effectively trapping their funds.
  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/803) & [Report](https://code4rena.com/reports/2023-10-zksync)
