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

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

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

## 4.[High] Malicious User can call lockOnBehalf repeatedly extend a users unlockTime, removing their ability to withdraw previously locked tokens

### Extend unlockTime

- Summary: The lockOnBehalf function allows a user to repeatedly lock tokens on behalf of another user, extending the recipient's unlock time indefinitely without minimum deposit required. This can prevent the original user from withdrawing their tokens and block them from reducing their lock duration.

- Impact & Recommendation: Ensure tokens locked via lockOnBehalf do not alter the recipient's lock duration, while preventing abuse such as during lockdrop NFT minting periods; or making it a two-step process where the recipient must accept or deny the lock before any changes are made to their current token lock state.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-munchables#h-01-malicious-user-can-call-lockonbehalf-repeatedly-extend-a-users-unlocktime-removing-their-ability-to-withdraw-previously-locked-tokens) & [Report](https://code4rena.com/reports/2024-05-munchables)

<details><summary>POC</summary>

```solidity
    address alice = makeAddr("alice");
    address bob = makeAddr("bob");
    function test_lockOnBehalfExtendsRecipientsUnlockTime() public {
        // Alice locks 10 ether in the protocol
        vm.deal(alice, 10 ether);
        vm.startPrank(alice);
        amp.register(MunchablesCommonLib.Realm.Everfrost, address(0));
        lm.lock{value: 10 ether}(address(0), 10 ether);
        vm.stopPrank();
        ILockManager.LockedTokenWithMetadata[] memory locked = lm.getLocked(address(alice));
        uint256 firstUnlockTime = locked[0].lockedToken.unlockTime;
        console.log("Unlock Time Start:", firstUnlockTime);
        // Some time passes
        vm.warp(block.timestamp + 5 hours);
        // Bob makes a zero deposit "on behalf" of alice
        vm.startPrank(bob);
        lm.lockOnBehalf(address(0), 0, alice);
        ILockManager.LockedTokenWithMetadata[] memory lockedNow = lm.getLocked(address(alice));
        uint256 endUnlockTime = lockedNow[0].lockedToken.unlockTime;
        // Confirm Alice's unlock time has been pushed back by bobs deposit
        console.log("Unlock Time End  :", endUnlockTime);
        assert(endUnlockTime > firstUnlockTime);
    }

    function test_lockOnBehalfGriefSetLockDuration() public {
        // Alice plans to call LockManager::setLockDuration to lower her min lock time to 1 hour
        vm.startPrank(alice);
        amp.register(MunchablesCommonLib.Realm.Everfrost, address(0));
        vm.stopPrank();
        // However Bob makes a 1 wei dontation lockOnBehalf beforehand
        vm.startPrank(bob);
        vm.deal(bob, 1);
        lm.lockOnBehalf{value: 1}(address(0), 1, alice);
        vm.stopPrank();
        // Now Alice's setter fails because her new duration is shorter than the `lockdrop.minDuration` set during bob's lockOnBehalf
        vm.startPrank(alice);
        vm.expectRevert();
        lm.setLockDuration(1 hours);
    }

```

</details>

## 5.[Medium] When LockManager.lockOnBehalf is called from MigrationManager, the user’s reminder will be set to 0, resulting in fewer received MunchableNFTs

### Reset remainder to 0

- Summary: The `LockManager.lockOnBehalf` function, used for migrating MunchableNFTs, calls `LockManager.lock` to handle the locking process. However, when called from MigrationManager, this function improperly resets the remainder to 0, causing users to receive fewer MunchableNFTs.

- Impact & Recommendation: When `LockManager._lock` is called from MigrationManager, ensure it does not reset the remainder variable. Instead, carry over the remainder to correctly allocate MunchableNFTs based on the total locked amount.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-munchables#m-02-when-lockmanagerlockonbehalf-is-called-from-migrationmanager-the-users-reminder-will-be-set-to-0-resulting-in-fewer-received-munchablenfts) & [Report](https://code4rena.com/reports/2024-05-munchables)

<details><summary>POC</summary>

```solidity
function _migrateNFTs(
    address _user,
    address _tokenLocked,
    uint256[] memory tokenIds
) internal {
    // ...
    uint256 quantity = (totalLockAmount * discountFactor) / 10e12;
    if (_tokenLocked == address(0)) {
        _lockManager.lockOnBehalf{value: quantity}(
            _tokenLocked,
            quantity,
            _user
        );
    } else if (_tokenLocked == address(WETH)) {
        WETH.approve(address(_lockManager), quantity);
        _lockManager.lockOnBehalf(_tokenLocked, quantity, _user);
    } else if (_tokenLocked == address(USDB)) {
        USDB.approve(address(_lockManager), quantity);
        _lockManager.lockOnBehalf(_tokenLocked, quantity, _user);
    }
    emit MigrationSucceeded(_user, migratedTokenIds, newTokenIds);
}

function _lock(
    address _tokenContract,
    uint256 _quantity,
    address _tokenOwner,
    address _lockRecipient
) private {
    // ...

    // add remainder from any previous lock
    uint256 quantity = _quantity + lockedToken.remainder;
@>  uint256 remainder;
    uint256 numberNFTs;
    uint32 _lockDuration = playerSettings[_lockRecipient].lockDuration;
    if (_lockDuration == 0) {
        _lockDuration = lockdrop.minLockDuration;
    }
    if (
        lockdrop.start <= uint32(block.timestamp) &&
        lockdrop.end >= uint32(block.timestamp)
    ) {
        if (
            _lockDuration < lockdrop.minLockDuration ||
            _lockDuration >
            uint32(configStorage.getUint(StorageKey.MaxLockDuration))
        ) revert InvalidLockDurationError();
@>      if (msg.sender != address(migrationManager)) {
            // calculate number of nfts
@>          remainder = quantity % configuredToken.nftCost;
            numberNFTs = (quantity - remainder) / configuredToken.nftCost;
            if (numberNFTs > type(uint16).max) revert TooManyNFTsError();
            // Tell nftOverlord that the player has new unopened Munchables
            nftOverlord.addReveal(_lockRecipient, uint16(numberNFTs));
        }
    }

    // ...
@>  lockedToken.remainder = remainder;
    lockedToken.quantity += _quantity;
    lockedToken.lastLockTime = uint32(block.timestamp);
    lockedToken.unlockTime = uint32(block.timestamp) + uint32(_lockDuration);
    // ...
}

```

</details>

## 6.[Medium] Players can gain more NFTs benefiting from that past remainder in subsequent locks

### Update remainder

- Summary: The `LockManager.lockOnBehalf` function, used for migrating MunchableNFTs, calls `LockManager.lock` to handle the locking process. However, when called from MigrationManager, this function improperly resets the remainder to 0, causing users to receive fewer MunchableNFTs.

- Impact & Recommendation: When `LockManager._lock` is called from MigrationManager, ensure it does not reset the remainder variable. Instead, carry over the remainder to correctly allocate MunchableNFTs based on the total locked amount.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-munchables#m-03-players-can-gain-more-nfts-benefiting-from-that-past-remainder-in-subsequent-locks) & [Report](https://code4rena.com/reports/2024-05-munchables)

<details><summary>POC</summary>

```solidity
    uint256 quantity = _quantity + lockedToken.remainder;
    uint256 remainder;
    uint256 numberNFTs;
    uint32 _lockDuration = playerSettings[_lockRecipient].lockDuration;
    // SNIPPED
    if (
        lockdrop.start <= uint32(block.timestamp) &&
        lockdrop.end >= uint32(block.timestamp)
    ) {
        if (
            _lockDuration < lockdrop.minLockDuration ||
            _lockDuration >
            uint32(configStorage.getUint(StorageKey.MaxLockDuration))
        ) revert InvalidLockDurationError();
        if (msg.sender != address(migrationManager)) {
            // calculate number of nfts
-->         remainder = quantity % configuredToken.nftCost;
            numberNFTs = (quantity - remainder) / configuredToken.nftCost;
            if (numberNFTs > type(uint16).max) revert TooManyNFTsError();
            // Tell nftOverlord that the player has new unopened Munchables
            nftOverlord.addReveal(_lockRecipient, uint16(numberNFTs));
        }
    }
  // SNIPPED
->  lockedToken.remainder = remainder;
    lockedToken.quantity += _quantity;

    function unlock(
        address _tokenContract,
        uint256 _quantity
    ) external notPaused nonReentrant {
        LockedToken storage lockedToken = lockedTokens[msg.sender][
            _tokenContract
        ];
        if (lockedToken.quantity < _quantity)
            revert InsufficientLockAmountError();
        if (lockedToken.unlockTime > uint32(block.timestamp))
            revert TokenStillLockedError();
        // force harvest to make sure that they get the schnibbles that they are entitled to
        accountManager.forceHarvest(msg.sender);
        lockedToken.quantity -= _quantity;
        // send token
        if (_tokenContract == address(0)) {
            payable(msg.sender).transfer(_quantity);
        } else {
            IERC20 token = IERC20(_tokenContract);
            token.transfer(msg.sender, _quantity);
        }
        emit Unlocked(msg.sender, _tokenContract, _quantity);
    }

```

</details>

## 7.[High] Lido’s apr can be maliciously updated to 0 value due to missing getLidoUpdateTolerance check, DOS pool lending

### Timestamp validation check

- Summary: The function `updateLidoValues()` in `LidoEthBaseInterestAllocator.sol` is permissionless and lacks a timestamp validation check. This function allows anyone to update Lido's APR (`aprBps`) without restrictions. Lido’s stETH rebases daily, and a malicious actor can exploit this by calling `updateLidoValues()` between rebases, setting `aprBps` to 0. This causes the `getBaseAprWithUpdate()` function to revert with an `InvalidAprError`, leading to a Denial-of-Service (DoS) that prevents the pool from validating new offers and effectively halts lending.

- Impact & Recommendation: It's recommended to add a check ensuring `_updateLidoValues()` only executes when the time since the last update exceeds a certain tolerance, i.e., `(block.timestamp - lidoData.lastTs > getLidoUpdateTolerance)`.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-gondi#h-01-lido’s-apr-can-be-maliciously-updated-to-0-value-due-to-missing-getLidoUpdateTolerance-check,-DOS-pool-lending) & [Report](https://code4rena.com/reports/2024-06-gondi)

<details><summary>POC</summary>

```solidity
//src/lib/pools/LidoEthBaseInterestAllocator.sol
    //@audit anyone can call updateLidoValues at any time, risks of _lidoData.aprBps being set to 0.
    function updateLidoValues() external {
        _updateLidoValues(getLidoData);
    }
    function _updateLidoValues(LidoData memory _lidoData) private {
        uint256 shareRate = _currentShareRate();
        _lidoData.aprBps = uint16(
            (_BPS * _SECONDS_PER_YEAR * (shareRate - _lidoData.shareRate)) /
                _lidoData.shareRate /
                (block.timestamp - _lidoData.lastTs)
        );
        _lidoData.shareRate = uint144(shareRate);
        _lidoData.lastTs = uint96(block.timestamp);
        getLidoData = _lidoData;
        emit LidoValuesUpdated(_lidoData);
    }

    //test/pools/LidoEthBaseInterestAllocator.t.sol
...
    function testMaliciousUpdateLidoValues() public {
        assertEq(_baseAllocator.getBaseAprWithUpdate(), 1000);
        (uint96 lastTs, , ) = _baseAllocator.getLidoData();
        vm.warp(uint256(lastTs) + 12);
        _baseAllocator.updateLidoValues();
        (, , uint16 newAprBps) = _baseAllocator.getLidoData();
        assertEq(newAprBps, 0);
        vm.expectRevert(abi.encodeWithSignature("InvalidAprError()"));
        _baseAllocator.getBaseAprWithUpdate();
    }
...

```

</details>

## 8.[Medium] Users can evade the yDUSD vault’s withdrawal timelock mechanism

### Bypass timestamp check

- Summary: A vulnerability in the yDUSD vault’s withdrawal timelock mechanism allows attackers to bypass the 7-day withdrawal period by using another account's proposed withdrawal information. By leveraging the withdraw() function, an attacker can withdraw DUSD assets from their own account using the request details of another account, thereby evading the timelock. This breaks the vault’s core security invariant, enabling immediate withdrawals without waiting for the timelock period.

- Impact & Recommendation: Add a check to ensure the msg.sender and the owner are the same, preventing this exploit.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-07-dittoeth#m-01-Users-can-evade-the-yDUSD-vault’s-withdrawal-timelock-mechanism) & [Report](https://code4rena.com/reports/2024-07-dittoeth)

<details><summary>POC</summary>

```solidity
    function withdraw(uint256 assets, address receiver, address owner) public override returns (uint256) {
+       if (msg.sender != owner) revert Errors.ERC4626InvalidOwner();
        WithdrawStruct storage withdrawal = withdrawals[msg.sender];
        uint256 amountProposed = withdrawal.amountProposed;
        uint256 timeProposed = withdrawal.timeProposed;
        if (timeProposed == 0 && amountProposed <= 1) revert Errors.ERC4626ProposeWithdrawFirst();
        if (timeProposed + C.WITHDRAW_WAIT_TIME > uint40(block.timestamp)) revert Errors.ERC4626WaitLongerBeforeWithdrawing();
        // @dev After 7 days from proposing, a user has 45 days to withdraw
        // @dev User will need to cancelWithdrawProposal() and proposeWithdraw() again
        if (timeProposed + C.WITHDRAW_WAIT_TIME + C.MAX_WITHDRAW_TIME <= uint40(block.timestamp)) {
            revert Errors.ERC4626MaxWithdrawTimeHasElapsed();
        }
        if (amountProposed > maxWithdraw(owner)) revert Errors.ERC4626WithdrawMoreThanMax();
        checkDiscountWindow();
        uint256 shares = previewWithdraw(amountProposed);
        IAsset _dusd = IAsset(dusd);
        uint256 oldBalance = _dusd.balanceOf(receiver);
        _withdraw(_msgSender(), receiver, owner, amountProposed, shares);
        uint256 newBalance = _dusd.balanceOf(receiver);
        // @dev Slippage is likely irrelevant for this. Merely for preventative purposes
        uint256 slippage = 0.01 ether;
        if (newBalance < slippage.mul(amountProposed) + oldBalance) revert Errors.ERC4626WithdrawSlippageExceeded();
        delete withdrawal.timeProposed;
        //reset withdrawal (1 to keep slot warm)
        withdrawal.amountProposed = 1;
        return shares;
    }

```

</details>

## 9.[Medium] Most of the FTC rewards can be taken by single entity

### Order farming

- Summary: Malicious users can quickly earn FTC rewards by opening and immediately closing positions, without actually taking on market risk. This is possible because the current implementation of the onDecreasePosition function does nothing, allowing users to collect rewards without any real position change. The protocol has a configuration (getStepMinProfitDuration) that sets a minimum duration for positions to stay open before rewards are disbursed. However, this mechanism only applies to profitable positions and doesn’t prevent users from exploiting zero-profit or small-loss positions.

- Impact & Recommendation: The team argues that the risk is low because the esFDX token, which is used for rewards, is non-transferable and under the control of the protocol. Therefore, even if users exploit this issue, they cannot immediately extract rewards. However, the document mentions "other incentives" that might be distributed alongside esFDX, which could increase the risk of manipulation. While the specifics of these incentives are unclear, their potential introduction could lead to further exploits. In conclusion, while the Flex Perpetuals team has some safeguards in place, the current design allows for possible manipulation of the reward system, and changes are recommended to mitigate this risk. The onDecreasePosition function should be modified to enforce a time window, requiring users to hold positions for a certain period before they can close them and collect rewards. If they close the position too soon, the reward should be revoked.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-12-flex-perpetuals#m-02-most-of-the-ftc-rewards-can-be-taken-by-single-entity) & [Report](https://code4rena.com/reports/2024-12-flex-perpetuals)

<details><summary>POC</summary>

```solidity
//FTCHook::onDecreasePosition
    function onDecreasePosition(
        address _primaryAccount,
        uint256,
        uint256,
        uint256 _sizeDelta,
        bytes32
    ) external onlyWhitelistedCaller {
        // Do nothing
    }

```

</details>
