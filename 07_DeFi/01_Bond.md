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

## 1.[Low] MinBorrow must be based on the market token

### MinBorrow

- Summary: In LendingTerm.sol, initializing minBorrow to 100e18 upon deployment poses an issue, especially with expensive assets like ETH or BTC.

- Impact & Recommendation: Set the minBorrow from the ProfitManager constructor to enhance contract versatility and eliminate the wait period for executing the setMinBorrow() function.
  🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
    constructor(address _core, uint minBorrow) CoreRef(_core) {
        emit MinBorrowUpdate(block.timestamp, 100e18);
    +       _minBorrow = minBorrow //should be carefully chosen by the contract deployer considering the price of collateral token
    }

    uint256 internal _minBorrow = 100e18;
    function minBorrow() external view returns (uint256) {
        return (_minBorrow * 1e18) / creditMultiplier;
    }

  ```

  </details>

## 2.[Medium] LendingTerm.sol `_partialRepay()` A user cannot partial repay a loan with 0 interest

### Partial repay zero interest

- Summary: The problem arises from a requirement in the code that checks if `interestRepaid != 0`. This condition, meant to prevent small repayments, creates an issue when the loan has zero interest, making partial repayment impossible despite being feasible through `_repay()`.

- Impact & Recommendation: A possible solution would be to remove the `interestRepaid != 0` from the require in `_partialRepay()` .
  🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/756) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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

## 3.[Low] If borrower becomes blacklisted for his collateral his loan needs to be forgiven in case to receive it

### Blacklisted collateral

- Summary: If a user's collateral is blacklisted, repay() will revert, requiring the Governor to initiate forgive() and manually transfer the tokens to the user, which may not be ideal if the user needs the tokens urgently.

- Impact & Recommendation: Consider adding a parameter to enable borrowers to designate another recipient for collateral when making a full repayment.
  🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

## 4.[Medium] Over 90% of the Guild staked in a gauge can be unstaked, despite the gauge utilizing its full debt allocation

### Manipulate the gauge's debt allocation by tolerance

- Summary: The mentioned protocol utilizes a tolerance factor to extend a gauge's debt ceiling by 20%. By exploiting this tolerance, it becomes possible to manipulate the gauge's debt allocation. Specifically, if a gauge's debt allocation is at 100%, it's feasible to decrease the gaugeWeight by a specific amount. After applying the tolerance, the gauge's debt allocation remains unchanged. This manipulation allows unstaking approximately 16.6666% of the totalWeight at a time, given the current operational state.

- Impact: By repetitively exploiting this vulnerability, it's possible to unstake over 90% of the total staked Guild from the gauge, effectively evading potential slashing penalties. Before adjusting the gaugeWeight, an initial check can be implemented to determine if the gauge is already utilizing its full debt allocation. If it is, any attempt to unstake Guild should be prevented to avoid potential issues.
  🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/475) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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

## 5.[Low] Credit rewards accrue for slashed users

### Set creditReward to 0

- Summary: Slashed stakers lose their GUILD tokens but still receive CREDIT tokens, leading to potential bad debt in the system.

- Impact & Recommendation: Set creditReward to 0 for staked users to prevent transfer of CREDIT tokens upon slashing.
  🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
    function getRewards(address user, address term)
            public
            returns (
                uint256 lastGaugeLoss, // GuildToken.lastGaugeLoss(term)
                UserStake memory userStake, // stake state after execution of getRewards()
                bool slashed // true if the user has been slashed
            )
        {
    ... More code
            uint256 _profitIndex = ProfitManager(profitManager).userGaugeProfitIndex(address(this), term);
            uint256 _userProfitIndex = uint256(userStake.profitIndex);
            if (_profitIndex == 0) _profitIndex = 1e18;
            if (_userProfitIndex == 0) _userProfitIndex = 1e18;
            uint256 deltaIndex = _profitIndex - _userProfitIndex;
            if (deltaIndex != 0) {
                uint256 creditReward = (uint256(userStake.guild) * deltaIndex) / 1e18;
                uint256 guildReward = (creditReward * rewardRatio) / 1e18;
                if (slashed) {
                    guildReward = 0;
    +               creditReward = 0
                }
                // forward rewards to user
                if (guildReward != 0) {
                    RateLimitedMinter(rlgm).mint(user, guildReward);
                    emit GuildReward(block.timestamp, user, guildReward);
                }
                if (creditReward != 0) {
                    //@audit will receive them despite being slashed
                    CreditToken(credit).transfer(user, creditReward);
                }
    ... More code
        }

  ```

  </details>

## 6.[Medium] LendingTerm inconsistency between debt ceiling as calculated in borrow() and debtCeiling()

### DebtCeiling calculation

- Summary: There's a discrepancy in debtCeiling calculation between the borrow() and debtCeiling() functions in the LendingTerm contract. This inconsistency not only causes operational differences but also affects liquidity utilization. The borrow() function calculates a more restrictive debtCeiling, leading to underutilized liquidity compared to the debtCeiling() function.

- Impact & Recommendation: Unify the debtCeiling calculation method across the protocol to avoid lost income opportunities for lenders due to unused liquidity not generating interest
  🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/308) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  - `borrow()` function calculates the `debtCeiling` using a simpler formula:

    - debtCeiling = $\frac{(Gauge Weight × (Total Borrowed Credit + Borrow Amount))}{Total Weight} × Gauge Weight Tolerance$

  - `debtCeiling()` function's calculation method is more complex:
    - debtCeiling = $(((\frac{Total BorrowedCredit × (Gauge Weight × 1.2e18)}{Total Weight}) - Issuance) × \frac{Total Weight}{Other Cauges Weight}) + Issuance$

  </details>

## 7.[Medium] ProfitManager’s creditMultiplier calculation does not count undistributed rewards; this can cause value losses to users

### CreditMultiplier calculation consider undistributed rewards

- Summary: The ProfitManager's creditMultiplier calculation doesn't consider undistributed rewards, leading to potential losses for users. When losses occur, excess amounts are attributed to credit token holders by slashing the creditMultiplier, `newCreditMultiplier = (creditMultiplier *  (creditTotalSupply - loss)) / creditTotalSupply;` . However, using totalSupply() can be problematic if a significant portion of the supply is in undistributed rewards, resulting in higher-than-necessary creditMultiplier slashing.

- Impact & Recommendation: CreditMultiplier slashing is higher than necessary due to incorrect accounting, penalizing credit token holders and locking value in the protocol. Consider using targetTotalSupply() instead of totalSupply() to rectify this issue.
  🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/292) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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
