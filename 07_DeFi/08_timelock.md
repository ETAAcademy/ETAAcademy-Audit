# ETAAcademy-Adudit: 8. Time Lock

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>08. Time Lock</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>EVM</th>
          <td>time-lock</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [High] Users will never be able to withdraw their claimed airdrop fully in ERC20Airdrop2.sol contract

### Withdraw tokens fully unlocked

- Summary: The ERC20Airdrop2.sol contract manages token airdrops with a withdrawal window where users can claim tokens within a specific period and withdraw them gradually. However, once tokens are fully unlocked, users face difficulty in withdrawing their full allocated amount due to restrictions in the withdraw() function, resulting in potential losses for users who cannot time their withdrawals effectively.

- Impact & Recommendation: The ERC20Airdrop2.sol contract poses timing challenges for users to withdraw their tokens fully, leading to potential losses in claimable amounts. Adding a buffer window to the ongoingWithdrawals() modifier could help users claim their fully unlocked tokens more effectively.

<br> 🐬: [Source](https://code4rena.com/reports/2024-03-taiko#h-03-users-will-never-be-able-to-withdraw-their-claimed-airdrop-fully-in-erc20airdrop2sol-contract) & [Report](https://code4rena.com/reports/2024-03-taiko)

  <details><summary>POC</summary>
 
  ```solidity
    function testAirdropIssue() public {
        vm.warp(uint64(block.timestamp + 11));
        vm.prank(Alice, Alice);
        airdrop2.claim(Alice, 100, merkleProof);
        // Roll 5 days after
        vm.roll(block.number + 200);
        vm.warp(claimEnd + 5 days);
        airdrop2.withdraw(Alice);
        console.log("Alice balance:", token.balanceOf(Alice));
        // Roll 6 days after
        vm.roll(block.number + 200);
        vm.warp(claimEnd + 11 days);
        vm.expectRevert(ERC20Airdrop2.WITHDRAWALS_NOT_ONGOING.selector);
        airdrop2.withdraw(Alice);
    }
  ```
  </details>
