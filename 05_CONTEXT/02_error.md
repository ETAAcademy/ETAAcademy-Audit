# ETAAcademy-Adudit: 2. Error

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>02. Error</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>context</th>
          <td>error</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] BOLDUpgradeAction.sol will fail to upgrade contracts due to error in the perform function

### Upgrade contracts

- The vulnerability in `checkClaimIdLink` allows an edge to inherit timers from its rival's children due to inadequate checks. This flaw can be exploited to inflate an edge's timer, enabling near-instant confirmation of any level 0 edge by repeatedly using a proved proof node and its ancestors or rivals. This occurs because only the originId and mutualId match is checked, allowing edges to inherit timers they shouldn't.

- Impact & Recommendation: Allow child edges to inherit the claimId of their parent and ensure the claiming edge's claimId matches the edgeId of the inheriting edge.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-arbitrum-foundation#m-02-boldupgradeactionsol-will-fail-to-upgrade-contracts-due-to-error-in-the-perform-function) & [Report](https://code4rena.com/reports/2024-05-arbitrum-foundation)

<details><summary>POC</summary>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;
import {Test} from "forge-std/Test.sol";
import "forge-std/console.sol";
struct OldStaker {
    uint256 amountStaked;
    uint64 index;
    uint64 latestStakedNode;
    // currentChallenge is 0 if staker is not in a challenge
    uint64 currentChallenge; // 1. cannot have current challenge
    bool isStaked; // 2. must be staked
}
interface IOldRollup {
    function pause() external;
    function forceRefundStaker(address[] memory stacker) external;
    function getStakerAddress(uint64 stakerNum) external view returns (address);
    function stakerCount() external view returns (uint64);
    function getStaker(address staker) external view returns (OldStaker memory);
}
contract C4 is Test {
    IOldRollup oldRollup;
    address admin;
    function setUp() public {
        uint256 forkId = vm.createFork("https://rpc.ankr.com/eth");
        vm.selectFork(forkId);
        oldRollup = IOldRollup(0x5eF0D09d1E6204141B4d37530808eD19f60FBa35);
        admin = 0x3ffFbAdAF827559da092217e474760E2b2c3CeDd;
    }
    function test_Cleanup() public {
        vm.startPrank(admin);
        oldRollup.pause();
        uint64 stakerCount = oldRollup.stakerCount();
        // since we for-loop these stakers we set an arbitrary limit - we dont
        // expect any instances to have close to this number of stakers
        if (stakerCount > 50) {
            stakerCount = 50;
        }
        for (uint64 i = 0; i < stakerCount; i++) {
            // FAILS with panic: array out-of-bounds access
            address stakerAddr = oldRollup.getStakerAddress(i);
            OldStaker memory staker = oldRollup.getStaker(stakerAddr);
            if (staker.isStaked && staker.currentChallenge == 0) {
                address[] memory stakersToRefund = new address[](1);
                stakersToRefund[0] = stakerAddr;
                oldRollup.forceRefundStaker(stakersToRefund);
            }
        }
    }
}
```

</details>
