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
  <br> 🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/246) & [Report](https://code4rena.com/reports/2023-10-zksync)

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
  <br> 🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/1253) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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
  <br> 🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/1194) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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
  <br> 🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/1032) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <br> 🐬: Others

  - [Low] ProfitManager::donateToTermSurplusBuffer() does not check if the term is from the same market: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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
  <br> 🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/1057) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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
  <br> 🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/868) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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
  <br> 🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/708) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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

## 8.[Low] IncrementGauge can be called with 0 weight

### 0 weight result in infinite loops

- Summary: The code doesn't check if the passed weight is greater than 0, leading to potential infinite loops, especially in \_decrementWeightUntilFree. This allows users to avoid slashing and grief those calling applyGaugeLoss for them. Additionally, operations like transfer, transferFrom, and burn may cause infinite loops if the user lacks sufficient balance and weight to be freed but has a weight of 0 in the first iteration of the loop.

- Impact & Recommendation: Implementing a verification check to ensure that the passed weight is greater than 0 would mitigate the potential for infinite loops.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
    function incrementGauge(address gauge, uint256 weight) public virtual returns (uint256 newUserWeight) {
            require(isGauge(gauge), "ERC20Gauges: invalid gauge");
            _incrementGaugeWeight(msg.sender, gauge, weight);
            return _incrementUserAndGlobalWeights(msg.sender, weight);
        }

    function _decrementWeightUntilFree(address user, uint256 weight) internal {
        uint256 userFreeWeight = balanceOf(user) - getUserWeight[user];
            // early return if already free
        if (userFreeWeight >= weight) return;
        // cache totals for batch updates
        uint256 userFreed;
        uint256 totalFreed;
        // Loop through all user gauges, live and deprecated
        address[] memory gaugeList = _userGauges[user].values();
        // Free gauges until through entire list or underweight
        uint256 size = gaugeList.length;
        for (
            uint256 i = 0;
            i < size && (userFreeWeight + userFreed) < weight;
        ) {
            address gauge = gaugeList[i];
            uint256 userGaugeWeight = getUserGaugeWeight[user][gauge];
            if (userGaugeWeight != 0) {
                userFreed += userGaugeWeight;
                _decrementGaugeWeight(user, gauge, userGaugeWeight);
                // If the gauge is live (not deprecated), include its weight in the total to remove
                if (!_deprecatedGauges.contains(gauge)) {
                    totalTypeWeight[gaugeType[gauge]] -= userGaugeWeight;
                    totalFreed += userGaugeWeight;
                }
                unchecked {
                    ++i; //@audit only in case userGaugeWeight != 0
                }
            }
        }
        totalWeight -= totalFreed;
    }

  ```

  </details>

## 9.[Medium] Inability to offboard term twice in a 7-day period may lead to bad debt to the market

### Offboard term twice in duration

- Summary: The system restricts proposing the offboarding of a lending term more than once within a 7-day period to prevent abuse. However, if a term is offboarded and re-onboarded quickly due to market conditions, voters won't be able to offboard it again if needed, potentially leading to the creation of bad debt and market impact.

- Impact & Recommendation: Currently, voters cannot offboard the same term twice within a 7-day window, potentially leading to bad debt and market impact if loans default. To address this, it's suggested to modify proposeOffboard() to allow a second offboarding if the previous one is completed.
  <br> 🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/370) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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

## 10.[Medium] SurplusGuildMinter.getReward() is susceptible to DoS due to unbounded loop

### No limit set on the length of loop

- Summary: SurplusGuildMinter's `getReward()` function invokes ProfitManager's `claimRewards()` that in a loop for all gauges/terms. With no limit set on the number of gauges and terms by `GuildToken.setMaxGauges(max)`, excessive gas consumption or Out-Of-Gas reverts may occur.
- Impact & Recommendation: In `SurplusGuildMinter's getReward(user, term)` call, use `ProfitManager(profitManager).claimRewards(address(this), term)` to ensure specific updating of the profit index for the given term instead of updating all available terms.
  <br> 🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/69) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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

## 11.[Medium] Dynamic modification of maxPrizeCount affects prize claims

### Change the length of loop

- Summary: There's a variable called maxPrizeCount, set by the owner, defining the maximum number of prize winners for a round. The issue arises when maxPrizeCount is decreased after setting prizes but before they're claimed. This causes winners of prizes with indices higher than the new maxPrizeCount to be unable to claim their winnings.

- Impact & Recommendation: To address this issue, it's advisable to implement a require check ensuring that maxPrizeCount cannot be decreased, as that aligns with the intended functionality.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-thruster#proof-of-concept-2) & [Report](https://code4rena.com/reports/2024-02-thruster)

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
