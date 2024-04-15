# ETAAcademy-Adudit: 4. Pool

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>04. Pool</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>DeFi</th>
          <td>pool</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] Lack of Slippage Protection in withdraw/redeem Functions of the Vault

### Slippage Protection

- Summary: Users in the PrizeVault expect a 1:1 exchange ratio between assets and shares when withdrawing. However, if the underlying yield vault incurs losses, this ratio can decrease. If total assets drop below total debt while a user's withdrawal is pending, they may receive fewer assets than expected, potentially causing losses.

- Impact & Recommendation: The withdraw and redeem functions in the PrizeVault lack slippage protection, potentially leading to user losses if the underlying yield vault experiences losses. To address this, users should be able to specify slippage protection parameters, such as a minimum amount for redemption or a maximum shares input for withdrawal.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-pooltogether#m-04-lack-of-slippage-protection-in-withdrawredeem-functions-of-the-vault) & [Report](https://code4rena.com/reports/2024-03-pooltogether)

  <details><summary>POC</summary>

  ```solidity
    function previewWithdraw(uint256 _assets) public view returns (uint256) {
        uint256 _totalAssets = totalAssets();
        // No withdrawals can occur if the vault controls no assets.
        if (_totalAssets == 0) revert ZeroTotalAssets();
        uint256 totalDebt_ = totalDebt();
        if (_totalAssets >= totalDebt_) {
            return _assets;
        } else {
            // Follows the inverse conversion of `convertToAssets`
            return _assets.mulDiv(totalDebt_, _totalAssets, Math.Rounding.Up);
        }
    }
    function convertToAssets(uint256 _shares) public view returns (uint256) {
        uint256 totalDebt_ = totalDebt();
        uint256 _totalAssets = totalAssets();
        if (_totalAssets >= totalDebt_) {
            return _shares;
        } else {
            // If the vault controls fewer assets than what has been deposited, a share will be worth a
            // proportional amount of the total assets. This can happen due to fees, slippage, or loss
            // of funds in the underlying yield vault.
            return _shares.mulDiv(_totalAssets, totalDebt_, Math.Rounding.Down);
        }
    }
    function totalAssets() public view returns (uint256) {
        return yieldVault.convertToAssets(yieldVault.balanceOf(address(this))) + _asset.balanceOf(address(this));
    }

  ```

  </details>
