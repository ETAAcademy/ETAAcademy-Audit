# ETAAcademy-Adudit: 1. Openzeppelin

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. Openzeppelin</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>library</th>
          <td>openzeppelin</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] `maxYieldVaultWithdraw()` uses `yieldVault.convertToAssets()`

### ERC4626: `convertToAssets` and `convertToShares` replaced with `preview`

- Summary: convertToAssets and convertToShares functions could be replaced with yield vault's preview functions for accurate accounting based on current conditions. However, since preview functions may revert, they must be used carefully in prize vault functions like maxDeposit, maxWithdraw, ensuring they don't revert.

- Impact & Recommendation: Use yieldVault.previewRedeem(yieldVault.maxRedeem(address(this))).
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-pooltogether#m-01-the-winner-can-steal-claimer-fees-and-force-him-to-pay-for-the-gas) & [Report](https://code4rena.com/reports/2024-03-pooltogether)

  <details><summary>POC</summary>

  ```solidity
    function _maxYieldVaultWithdraw() internal view returns (uint256) {
        return yieldVault.convertToAssets(yieldVault.maxRedeem(address(this)));
    }

  ```

  </details>
