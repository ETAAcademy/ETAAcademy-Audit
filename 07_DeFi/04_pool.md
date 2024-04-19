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

## 2.[High] ReLPContract wrongfully assumes protocol owns all of the liquidity in the UniswapV2 pool

### Not calculations on the all liquidity

- Summary: The ReLPContract assumes it owns all liquidity in the UniswapV2 pool. When RdpxV2Core calls ReLPContract#reLP, it may pass incorrect amounts or trigger a revert if the protocol doesn't own the majority of LP balance. This is because the calculation assumes protocol owns all RDPX reserves, potentially leading to a denial-of-service if lpToRemove exceeds actual LP balance.

- Impact & Recommendation: Change the logic and base all calculations on the pair balance of `UniV2LiquidityAmo`
  <br> 🐬: [Source](https://code4rena.com/reports/2023-08-dopex#h-09-relpcontract-wrongfully-assumes-protocol-owns-all-of-the-liquidity-in-the-uniswapv2-pool) & [Report](https://code4rena.com/reports/2023-08-dopex)

    <details><summary>POC</summary>

  ```solidity
    function testReLpContract() public {
        testV2Amo();
        // set address in reLP contract and grant role
        reLpContract.setAddresses(
            address(rdpx),
            address(weth),
            address(pair),
            address(rdpxV2Core),
            address(rdpxReserveContract),
            address(uniV2LiquidityAMO),
            address(rdpxPriceOracle),
            address(factory),
            address(router)
        );
        reLpContract.grantRole(reLpContract.RDPXV2CORE_ROLE(), address(rdpxV2Core));
        reLpContract.setreLpFactor(9e4);
        // add liquidity
        uniV2LiquidityAMO.addLiquidity(5e18, 1e18, 0, 0);
        uniV2LiquidityAMO.approveContractToSpend(
            address(pair),
            address(reLpContract),
            type(uint256).max
        );
        rdpxV2Core.setIsreLP(true);
        (uint256 reserveA, uint256 reserveB, ) = pair.getReserves();
        weth.mint(address(2), reserveB * 10);
        rdpx.mint(address(2), reserveA * 10);
        vm.startPrank(address(2));
        weth.approve(address(router), reserveB * 10);
        rdpx.approve(address(router), reserveA * 10);
        router.addLiquidity(address(rdpx), address(weth), reserveA * 10, reserveB * 10, 0, 0, address(2), 12731316317831123);
        vm.stopPrank();

        console.log("UniV2Amo balance isn't enough and will underflow");
        uint pairBalance = pair.balanceOf(address(uniV2LiquidityAMO));
        console.log("UniV2Amo LP balance: ", pairBalance);
        vm.expectRevert("ds-math-sub-underflow");
        rdpxV2Core.bond(1 * 1e18, 0, address(this));
    }

  ```

    </details>
