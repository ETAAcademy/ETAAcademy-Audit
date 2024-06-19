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

## 2. [Medium] Incentive accumulation can be sandwiched with additional shares to gain advantage over long-term depositors

### No enforced unbonding period

- Summary: In this system, rewards accumulate periodically and are distributed among deposited shares. Users can swiftly deposit, claim rewards, and withdraw shares, incentivizing rapid turnover rather than long-term holding. However, this setup allows adversaries to borrow shares and unfairly claim a disproportionate share of rewards during accumulation periods. Unlike in the earnings module, there's no enforced waiting period for share withdrawals.

- Impact & Recommendation: It's recommended to track deposited shares between accumulation intervals and adjust incentive rewards based on the actual deposit duration.

<br> 🐬: [Source](https://code4rena.com/reports/2024-03-acala#m-02-incentive-accumulation-can-be-sandwiched-with-additional-shares-to-gain-advantage-over-long-term-depositors) & [Report](https://code4rena.com/reports/2024-03-acala)

  <details><summary>POC</summary>
 
  ```rust
    diff --git a/src/modules/incentives/src/tests.rs b/src/modules/incentives/src/tests.rs
    index 1370d5b..fa16a08 100644
    --- a/src/modules/incentives/src/tests.rs
    +++ b/src/modules/incentives/src/tests.rs
    @@ -1171,10 +1171,11 @@ fn transfer_reward_and_update_rewards_storage_atomically_when_accumulate_incenti
            assert_eq!(TokensModule::free_balance(AUSD, &VAULT::get()), 0);
    
            RewardsModule::add_share(&ALICE::get(), &PoolId::Loans(LDOT), 1);
    +		RewardsModule::add_share(&BOB::get(), &PoolId::Loans(LDOT), 1);
            assert_eq!(
                RewardsModule::pool_infos(PoolId::Loans(LDOT)),
                PoolInfo {
    -				total_shares: 1,
    +				total_shares: 2,
                    ..Default::default()
                }
            );
    @@ -1188,7 +1189,7 @@ fn transfer_reward_and_update_rewards_storage_atomically_when_accumulate_incenti
            assert_eq!(
                RewardsModule::pool_infos(PoolId::Loans(LDOT)),
                PoolInfo {
    -				total_shares: 1,
    +				total_shares: 2,
                    rewards: vec![(ACA, (30, 0)), (AUSD, (90, 0))].into_iter().collect()
                }
            );
    @@ -1202,7 +1203,7 @@ fn transfer_reward_and_update_rewards_storage_atomically_when_accumulate_incenti
            assert_eq!(
                RewardsModule::pool_infos(PoolId::Loans(LDOT)),
                PoolInfo {
    -				total_shares: 1,
    +				total_shares: 2,
                    rewards: vec![(ACA, (60, 0)), (AUSD, (90, 0))].into_iter().collect()
                }
            );
  ```
  </details>

## 3.[Medium] Fee and profit payments into RevenueSharingVault can be sandwiched

### Rewards without long-term staking

- Summary: The `RevenueSharingVault` contract is vulnerable to attacks where users can deposit TRNDO, trigger buy/sell fees to appreciate their shares, and quickly withdraw to earn disproportionate rewards without long-term staking.

- Impact & Recommendation: Rewards in the `RevenueSharingVault` are sent to a separate `VaultDistributor` contract, which pays out rewards slowly over a configurable timeframe. The `VaultDistributor` snapshots available rewards and distributes them linearly. This requires overriding additional functions in ERC4626.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-tornadoblast-proleague#m-03-fee-and-profit-payments-into-revenueSharingVault-can-be-sandwiched) & [Report](https://code4rena.com/reports/2024-06-tornadoblast-proleague)

<details><summary>POC</summary>

```solidity
--- a/apps/contracts/src/tornadoToken/RevenueSharingVault.sol
+++ b/apps/contracts/src/tornadoToken/RevenueSharingVault.sol
@@ -7,12 +7,18 @@ import { ERC4626 } from "@openzeppelin/contracts/token/ERC20/extensions/ERC4626.
 import { TornadoBlastBotToken } from "./TornadoBlastBotToken.sol";
 import { BlastGasAndYield } from "../commons/BlastGasAndYield.sol";
+import {VaultDistributor} from "../VaultDistributor.sol";
+
 /// @dev send tornado blast tokens to this contract to redistribute them to stakers
 /// @dev treasury MUST stake a significant amount first to avoid future share/tokenAmount slippage
 contract RevenueSharingVault is ERC4626, BlastGasAndYield {
+    VaultDistributor vaultDistributor;
     constructor(
-        TornadoBlastBotToken tornadoBlastToken
-    ) ERC4626(tornadoBlastToken) ERC20("Staked Tornado Blast Token", "stTRNDO") {}
+        TornadoBlastBotToken tornadoBlastToken,
+        VaultDistributor _vaultDistributor
+    ) ERC4626(tornadoBlastToken) ERC20("Staked Tornado Blast Token", "stTRNDO") {
+        vaultDistributor = _vaultDistributor;
+    }
     function _update(address from, address to, uint256 value) internal override {
         // allow mint and burn, disallow transfers
@@ -21,4 +27,32 @@ contract RevenueSharingVault is ERC4626, BlastGasAndYield {
         }
         super._update(from, to, value);
     }
+
+    function setVaultDistributor(VaultDistributor _vaultDistributor) external onlyOwner {
+        vaultDistributor = _vaultDistributor;
+    }
+
+    function totalAssets() public view override returns (uint256) {
+        return _asset.balanceOf(address(this)) + vaultDistributor.pendingRewards();
+    }
+
+    function deposit(uint256 assets, address receiver) public override returns (uint256) {
+        vaultDistributor.processRewards();
+        super.deposit(assets, receiver);
+    }
+
+    function mint(uint256 shares, address receiver) public override returns (uint256) {
+        vaultDistributor.processRewards();
+        super.mint(shares, receiver);
+    }
+
+    function withdraw(uint256 assets, address receiver, address owner) public override returns (uint256) {
+        vaultDistributor.processRewards();
+        super.withdarw(assets, receiver, owner);
+    }
+
+    function redeem(uint256 shares, address receiver, address owner) public override returns (uint256) {
+        vaultDistributor.processRewards();
+        super.redeem(shares, receiver, owner);
+    }
 }

```

</details>
