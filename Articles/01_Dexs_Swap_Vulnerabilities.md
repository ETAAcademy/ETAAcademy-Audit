# ETAAcademy-Adudit: 1. DEXs Swap Vulnerabilities

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. DEXs Swap Vulnerabilities</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>library</th>
          <td>DEXs_Swap_Vulnerabilities</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

A Decentralized Crypto Exchange (DEX) allows users to trade cryptocurrencies directly without a central intermediary, using smart contracts for trustless operations and letting users retain control of their funds and private keys. Unlike centralized exchanges, which hold user funds and face higher security risks, DEXs distribute trading orders across the network, presenting unique security challenges and navigating a more complex regulatory landscape.

DEXs swap attacks primarily revolve around exploiting vulnerabilities in decentralized exchanges (DEXs), particularly those related to liquidity management, transaction execution, and the inherent trustlessness of these platforms. These attacks can range from sophisticated exploits targeting smart contract vulnerabilities to more straightforward manipulations of market conditions, such as slippage attacks, pool closures, Money Laundering, transaction deadlines, price manipulation, superior swap strategies, sandwich attacks, inaccuracies in calculating amountIn and amountOut, and fee calculations. Here's a detailed look at the types of attacks and how they exploit the unique characteristics of DEXs:

### Attack #1. Slippage Attacks

**Definition:** Slippage attacks exploit the difference between the expected price of a trade and the actual executed price due to market volatility or liquidity issues. In DEXs, slippage can occur because of rapid changes in the available liquidity for a particular token pair, especially during periods of high volatility or low liquidity.

**Impact:** Traders may end up receiving fewer tokens than expected when buying or selling, or they may pay a higher price than expected when buying. This can be particularly problematic for larger trades, where the size of the trade order can significantly impact the available liquidity in the market.

**Example:** BakerFi, Lybra Finance, Dopex

**BakerFi:** The protocol's `_swap()` method lacks slippage protection by not setting the `amountOutMinimum` parameter, posing risks of front-running and price manipulation. This issue affects several functions, including `_convertFromWETH()` and `_convertToWETH()`, leading to potential vulnerabilities in multiple swap operations. The recommended fix involves setting `amountOutMinimum` to `params.amountOut` to enforce slippage protection. This issue was confirmed and addressed in a subsequent update, as detailed in [GitHub Pull Request #41](https://github.com/baker-fi/bakerfi-contracts/pull/41).

```solidity
    function _swap(
        ISwapHandler.SwapParams memory params
    ) internal override returns (uint256 amountOut) {
        if (params.underlyingIn == address(0)) revert InvalidInputToken();
        if (params.underlyingOut == address(0)) revert InvalidOutputToken();
        uint24 fee = params.feeTier;
        if (fee == 0) revert InvalidFeeTier();
        // Exact Input
        if (params.mode == ISwapHandler.SwapType.EXACT_INPUT) {
            amountOut = _uniRouter.exactInputSingle(
                IV3SwapRouter.ExactInputSingleParams({
                    tokenIn: params.underlyingIn,
                    tokenOut: params.underlyingOut,
                    amountIn: params.amountIn,
-                     amountOutMinimum: 0,   //@audit miss set params.amountOut
+                     amountOutMinimum: params.amountOut,
                    fee: fee,
                    recipient: address(this),
                    sqrtPriceLimitX96: 0
                })
            );
            if (amountOut == 0) {
                revert SwapFailed();
            }
            emit Swap(params.underlyingIn, params.underlyingOut, params.amountIn, amountOut);
            // Exact Output
        } else if (params.mode == ISwapHandler.SwapType.EXACT_OUTPUT) {
            uint256 amountIn = _uniRouter.exactOutputSingle(
                IV3SwapRouter.ExactOutputSingleParams({
                    tokenIn: params.underlyingIn,
                    tokenOut: params.underlyingOut,
                    fee: fee,
                    recipient: address(this),
                    amountOut: params.amountOut,
                    amountInMaximum: params.amountIn,
                    sqrtPriceLimitX96: 0
                })
            );
            if (amountIn < params.amountIn) {
                IERC20(params.underlyingIn).safeTransfer(address(this), params.amountIn - amountIn);
            }
            emit Swap(params.underlyingIn, params.underlyingOut, amountIn, params.amountOut);
            amountOut = params.amountOut;
        }
    }
```

**Lybra Finance:** The function rigidRedemption() in the LybraFinance smart contract is vulnerable to issues caused by volatile prices. When users try to redeem their PeUSD (stable coin) for WstETH/stETH (variable price), they may suffer slippage losses if the transaction is delayed and executed at an unfavorable price. Introduce slippage protection by requiring a minimum amount received parameter in the rigidRedemption() function. This change ensures users can specify the minimum acceptable amount of WstETH/stETH to receive, preventing significant losses due to price volatility.

```solidity
    function rigidRedemption(address provider, uint256 eusdAmount,uint256 minAmountReceived) external virtual {
        depositedAsset[provider] -= collateralAmount;
        totalDepositedAsset -= collateralAmount;
+       require(minAmountReceived <= collateralAmount);
        collateralAsset.transfer(msg.sender, collateralAmount);
        emit RigidRedemption(msg.sender, provider, eusdAmount, collateralAmount, block.timestamp);
    }
```

**Dopex:** In the \_curveSwap function, there is an issue with the order of calling getDpxEthPrice() and getEthPrice(). This mistake results in incorrect slippage calculations when performing swaps between ETH and DPXETH. Specifically, when \_ethToDpxEth is true, indicating a swap from ETH to DPXETH, getDpxEthPrice() is erroneously used, which actually returns the price of DPXETH in terms of ETH (DPXETH/ETH). Conversely, when swapping from DPXETH to ETH, getEthPrice() is incorrectly employed, as it provides the price of ETH in terms of DPXETH (ETH/DPXETH).

```solidity
    uint256 minOut = _ethToDpxEth
      ? (((_amount * getEthPrice()) / 1e8) -
        (((_amount * getEthPrice) * slippageTolerance) / 1e16))
      : (((_amount * getDpxEthPrice()) / 1e8) -
        (((_amount * getDpxEthPrice()) * slippageTolerance) / 1e16));
```

**Dopex:** The reLP() function in the contract has an incorrect formula for calculating mintokenAAmount, which affects slippage protection during liquidity provision. The formula for calculating mintokenAAmount in the above code is `((amountB / 2) * tokenAInfo.tokenAPrice) / 1e8`. `amountB` is the amount of tokenB, but tokenAInfo.tokenAPrice is the price of tokenA, which shouldn’t be multiplied together.

```solidity
  function reLP(uint256 _amount) external onlyRole(RDPXV2CORE_ROLE) {
...
-    mintokenAAmount =
-     (((amountB / 2) * tokenAInfo.tokenAPrice) / 1e8) -
-     (((amountB / 2) * tokenAInfo.tokenAPrice * slippageTolerance) / 1e16);
+    mintokenAAmount =  ((amountB / 2) * 1e8 / tokenAInfo.tokenAPrice) * (1e8 - slippageTolerance) / 1e8;
    uint256 tokenAAmountOut = IUniswapV2Router(addresses.ammRouter)
      .swapExactTokensForTokens(
        amountB / 2,
        mintokenAAmount,
        path,
        address(this),
        block.timestamp + 10
      )[path.length - 1];
    (, , uint256 lp) = IUniswapV2Router(addresses.ammRouter).addLiquidity(
      addresses.tokenA,
      addresses.tokenB,
      tokenAAmountOut,
      amountB / 2,
      0,
      0,
      address(this),
      block.timestamp + 10
    );
    // transfer the lp to the amo
    IERC20WithBurn(addresses.pair).safeTransfer(addresses.amo, lp);
    IUniV2LiquidityAmo(addresses.amo).sync();
    // transfer rdpx to rdpxV2Core
    IERC20WithBurn(addresses.tokenA).safeTransfer(
      addresses.rdpxV2Core,
      IERC20WithBurn(addresses.tokenA).balanceOf(address(this))
    );
    IRdpxV2Core(addresses.rdpxV2Core).sync();
  }
```

### Attack #2. Transaction deadlines

**Definition:** When making a swap, Automated Market Makers (AMMs) provide users with a deadline parameter, ensuring the swap transaction will only be executed within the specified time frame. Without a deadline, the transaction can remain pending and may be executed long after the user submits it. By that time, the trade may occur at an unfavorable price, negatively impacting the user's position.

**Impact:** Without a deadline, the transaction can be manipulated by validators, resulting in significant slippage and potential losses for users. For example, a user stakes funds, which are converted using Uniswap. A validator could delay the transaction, waiting for a favorable market condition to front-run the original user, causing maximum slippage.

**Example:** Asymmetry

**Asymmetry:** Lack of a deadline parameter in the `ISwapRouter.exactInputSingle` function can lead to front-running vulnerabilities, where a validator might delay a transaction to exploit market conditions, causing significant slippage. The problem arises when users stake and Uniswap is used to convert ETH to WETH without a set deadline, making the transaction susceptible to manipulation. The recommended mitigation is to include a user-input deadline parameter in the `Reth.deposit()` function and pass it to the swap functions to prevent such exploits and to ensure that the transaction can be executed in a short period of time. And, introduce a deadline parameter to the functions withdraw() for WstEth.sol and SfrxEth.sol and deposit() for Reth.sol.

```solidity
        ISwapRouter.ExactInputSingleParams memory params = ISwapRouter
            .ExactInputSingleParams({
                tokenIn: _tokenIn,
                tokenOut: _tokenOut,
                fee: _poolFee,
                recipient: address(this),
+                     deadline: block.timestamp,
                amountIn: _amountIn,
                amountOutMinimum: _minOut,
                sqrtPriceLimitX96: 0
            });
```

```solidity
modifier ensure(uint deadline) {
	require(deadline >= block.timestamp, 'UniswapV2Router: EXPIRED');
	_;
}
```

### Attack #3. Directed Attacks Against Liquidity Provider Pools

**Definition:** IUnlike Uniswap v3, KyberSwap Elastic introduces an innovative feature called the Reinvestment Curve. This is an additional AMM pool that accumulates fees charged from users’ swaps in the pool, with a curve that supports a price range from 0 to infinity. KyberSwap Elastic combines the reinvestment curve with the original price curve (i.e., the curves are separate, but the funds remain in the same pool), allowing LPs to compound their fees and earn returns even when the price exceeds their position range.

**Impact:** Due to the Reinvestment Curve feature of KyberSwap Elastic pool, when both base liquidity and reinvestment liquidity are considered as actual liquidity, it calculates the amount of tokens needed for exchange at the scale boundary using the calcReachAmount function. This calculation resulted in a higher than expected amount, causing the next price sqrtP to exceed the boundary scale’s sqrtP. The pool, using an inequality to check sqrtP, led to the protocol not updating liquidity and crossing the tick as expected through \_updateLiquidityAndCrossTick.

**Example:** KyberSwap

**KyberSwap:** The attack on KyberSwap, where the attacker stole funds mostly in Ether, wrapped ether (wETH), and USDC from multiple cross-chain deployments of KyberSwap. This suggests that the theft targeted the liquidity provider pools themselves, indicating a directed attack against the core infrastructure of the DEX.

```solidity
function computeSwapStep(
  uint256 liquidity,
  uint160 currentSqrtP,
  uint160 targetSqrtP,
  uint256 feeInFeeUnits,
  int256 specifiedAmount,
  bool isExactInput,
  bool isToken0
)
  internal
  pure
  returns (
    int256 usedAmount,
    int256 returnedAmount,
    uint256 deltaL,
    uint160 nextSqrtP
  )
{
  // in the event currentSqrtP == targetSqrtP because of tick movements, return
  // eg. swapped up tick where specified price limit is on an initialised tick
  // then swapping down tick will cause next tick to be the same as the current tick
  if (currentSqrtP == targetSqrtP) return (0, 0, 0, currentSqrtP);
  usedAmount = calcReachAmount(
    liquidity,
    currentSqrtP,
    targetSqrtP,
    feeInFeeUnits,
    isExactInput,
    isToken0
  );

  if (
    (isExactInput && usedAmount > specifiedAmount) ||
    (!isExactInput && usedAmount <= specifiedAmount)
  ) {
    usedAmount = specifiedAmount;
  } else {
    nextSqrtP = targetSqrtP;
  }

  uint256 absDelta = usedAmount >= 0 ? uint256(usedAmount) : usedAmount.revToUint256();
  if (nextSqrtP == 0) {
    deltaL = estimateIncrementalLiquidity(
      absDelta,
      liquidity,
      currentSqrtP,
      feeInFeeUnits,
      isExactInput,
      isToken0
    );
    nextSqrtP = calcFinalPrice(absDelta, liquidity, deltaL, currentSqrtP, isExactInput, isToken0)
    .toUint160();
  } else {
    deltaL = calcIncrementalLiquidity(
      absDelta,
      liquidity,
      currentSqrtP,
      nextSqrtP,
      isExactInput,
      isToken0
    );
  }
  returnedAmount = calcReturnedAmount(
    liquidity,
    currentSqrtP,
    nextSqrtP,
    deltaL,
    isExactInput,
    isToken0
  );
}
```

### Attack #4. Money Laundering Through DEXs

**Definition:** Criminals can use DEXs for crypto-to-crypto swaps to launder criminal proceeds. This involves obtaining Ether or Ethereum-based tokens (for example, by hacking an exchange), swapping them at a DEX for new tokens, and then depositing these new tokens at a legitimate exchange to cash out for fiat.

**Impact:** While DEXs offer advantages in terms of bypassing compliance controls and lacking a central administrator, they also present challenges for money laundering. All DEX crypto-to-crypto swaps are recorded in smart contracts on the blockchain, allowing for visibility into these transactions and the potential tracing of illicit funds.

### Attack #5. Price Arbitrage

**Definition:** In the context of a decentralized exchange (DEX) swap, the price refers to the exchange rate between two different cryptocurrencies or tokens. This rate determines how much of one token you will receive in exchange for a certain amount of another token. Arbitrage in the context of DEX swaps involves exploiting price discrepancies of the same asset across different markets or liquidity pools to generate a profit.

**Impact:** Legitimate users may face wrong price resources, meaning they receive less favorable prices for their swaps. This can lead to significant financial losses, especially for large transactions.

**Example:** Asymmetry, Dopex

**Asymmetry:** The poolPrice function in the Reth derivative contract risks overflow when calculating the spot price of the derivative asset using a Uniswap V3 pool. This can cause inaccurate price calculations and potential loss of funds or erroneous contract behavior. Example code line that may overflow: sqrtPriceX96 _ (uint(sqrtPriceX96)) _ (1e18). Replace the current calculation with a safer implementation using the OracleLibrary.getQuoteAtTick function from the Uniswap V3 periphery, which includes overflow checks and handles different numerical issues. Asymmetry confirmed using Chainlink to obtain prices instead of relying on the poolPrice function, effectively mitigating the risk of overflow.

```solidity
function poolPrice() private view returns (uint256) {
    address rocketTokenRETHAddress = RocketStorageInterface(
        ROCKET_STORAGE_ADDRESS
    ).getAddress(
            keccak256(
                abi.encodePacked("contract.address", "rocketTokenRETH")
            )
        );
    IUniswapV3Factory factory = IUniswapV3Factory(UNI_V3_FACTORY);
    IUniswapV3Pool pool = IUniswapV3Pool(
        factory.getPool(rocketTokenRETHAddress, W_ETH_ADDRESS, 500)
    );
    (, int24 tick, , , , , ) = pool.slot0();
    return OracleLibrary.getQuoteAtTick(tick, 1e18, rocketTokenRETHAddress, W_ETH_ADDRESS);
}
```

**Dopex:** The PerpetualAtlanticVaultLP contract enables users to deposit WETH and receive shares in return, which can later be redeemed for a proportional amount of WETH and rdpx. This mechanism can be exploited to swap WETH for rdpx without incurring swap fees from the V2 Permissioned AMM. The issue arises because the vault calculates the rdpx return based on its proportion in the vault at the time of the initial deposit rather than its current market value. This discrepancy allows for potential arbitrage opportunities, as users can effectively swap WETH for rdpx at a discount, thereby reducing the value for depositors and causing the AMM to lose out on swap fees. To mitigate this, it is recommended to use up-to-date rdpx prices when calculating redemption amounts and consider mechanisms to prevent immediate deposit and redeem actions.

```solidity
  function _convertToAssets(
    uint256 shares
  ) internal view virtual returns (uint256 assets, uint256 rdpxAmount) {
    uint256 supply = totalSupply;
    return
      (supply == 0)
        ? (shares, 0)
        : (
          shares.mulDivDown(totalCollateral(), supply),
          shares.mulDivDown(_rdpxCollateral, supply)
        );
  }
```

### Attack #6. Price manipulation(Sandwich attacks & front-running)

**Definition:** Price manipulation in the context of decentralized exchanges (DEXs) involves activities where a trader or a group of traders artificially influence the price of assets during swap transactions. This can be achieved through tactics such as front-running, where a manipulator observes a pending large transaction and places their own transactions to benefit from the expected price change, or sandwich attacks, where the manipulator places a buy order before and a sell order after a large transaction to profit from the price movement caused by the large transaction.

**Impact:** Legitimate users may face higher slippage, meaning they receive less favorable prices for their swaps. This can lead to significant financial losses, especially for large transactions. Manipulators can profit at the expense of regular traders, creating an uneven playing field and disadvantaging smaller or less-experienced traders.

**Example:** Dopex

**Dopex:** The vulnerability in reLPContract.reLP() is identified through its indirect invocation by RdpxV2Core.bond(), allowing potential exploitation via a sandwich attack. This occurs because users can manipulate the bonding amount in RdpxV2Core.bond() to control the amount of liquidity removed and subsequently using ETH from flash loan to inflate rDPX price via UniswapV2 within reLPContract.reLP() without requiring mempool access. reLPContract.reLP() performs swaps and liquidity additions, further impacting prices. Profit realization by swapping rDPX back to ETH and repaying the flash loan. Despite slippage protection measures, the attacker can adjust transaction sizes to profitably exploit price discrepancies.

```solidity
  function bond(
    uint256 _amount,
    uint256 rdpxBondId,
    address _to
  ) public returns (uint256 receiptTokenAmount) {
    ...
    //@audit an attacker can indirectly trigger this to perform a large trade and sandwich attack it
    // reLP
    if (isReLPActive) IReLP(addresses.reLPContract).reLP(_amount);
```

### Attack #7. Superior swap strategies

**Definition:** Superior swap strategies involve using liquidity pools with greater liquidity to reduce slippage and improve trading efficiency. For example, depositing more ETH than the available in a pool can cause the swap to fail due to slippage protection. Instead of making large deposits in one go, the strategy should involve multiple smaller steps. Implementing a timelock mechanism that can dynamically move funds between different protocols allows for more granular rebalancing, enhanced transparency, and gives users time to react to changes in protocol exposure.

**Impact:** By using liquidity pools with higher liquidity, users can achieve lower slippage and better trade execution. This approach not only optimizes the use of available liquidity but also enhances the overall user experience by providing more reliable and efficient swaps. Additionally, the use of a timelock mechanism ensures better fund management and allows users to make informed decisions based on changes in protocol exposure, thereby increasing trust and transparency in the system.

**Example:** Asymmetry

**Asymmetry:** Uniswap rETH/WETH pool for swaps has higher fees and lower liquidity compared to alternatives like Balancer and Curve pools. The Uniswap pool has $5 million in liquidity with a 0.05% fee, while Balancer has $80 million in liquidity with a 0.04% fee, and Curve has $8 million in liquidity with a 0.037% fee. The use of the Uniswap pool results in higher slippage and unnecessary fees, diminishing user value. The recommended mitigation is to use RocketPool’s RocketSwapRouter.sol contract’s swapTo() function, which optimizes the swap path between Balancer and Uniswap pools. Alternatively, modifying Reth.sol to use the Balancer pool would also reduce swap fees and slippage costs.

```solidity
function swapExactInputSingleHop(
    address _tokenIn,
    address _tokenOut,
    uint24 _poolFee,
    uint256 _amountIn,
    uint256 _minOut
) private returns (uint256 amountOut) {
    IERC20(_tokenIn).approve(UNISWAP_ROUTER, _amountIn);
    ISwapRouter.ExactInputSingleParams memory params = ISwapRouter
        .ExactInputSingleParams({
            tokenIn: _tokenIn,
            tokenOut: _tokenOut,
            fee: _poolFee,
            recipient: address(this),
            amountIn: _amountIn,
            amountOutMinimum: _minOut,
            sqrtPriceLimitX96: 0
        });
    amountOut = ISwapRouter(UNISWAP_ROUTER).exactInputSingle(params);
}
```

### Attack #8. Fee

**Definition:** A swap fee, also known as a trading fee or transaction fee, is a small percentage of the transaction value that is charged by a decentralized exchange (DEX) like Uniswap for facilitating trades between different token pairs. This fee is typically paid by the trader and is distributed to liquidity providers (LPs) as an incentive for supplying liquidity to the pool.

**Impact:** The primary impact of this bug is the loss of swap commission fees that should be collected on ITM positions. This affects the profitability of the protocol since it reduces the overall revenue generated from swap commissions.

**Example:** Panoptic

**Panoptic:** The main invariant of Panoptic, "Fees paid to a given user should not exceed the amount of fees earned by the liquidity owned by that user," can be broken due to a slight difference in fee computation methods. Panoptic calculates fees as (currFeeGrowth _ liquidity / Q128) - (prevFeeGrowth _ liquidity / Q128), whereas Uniswap V3 calculates it as (currFeeGrowth - prevFeeGrowth) \* liquidity / Q128. This difference can result in a user collecting more fees than they should, potentially reducing the fees available for other users.

```solidity
int256 amountToCollect = _getFeesBase(univ3pool, startingLiquidity, liquidityChunk).sub(
            s_accountFeesBase[positionKey]
        );
    feesBase = int256(0)
                .toRightSlot(int128(int256(Math.mulDiv128(feeGrowthInside0LastX128, liquidity))))
                .toLeftSlot(int128(int256(Math.mulDiv128(feeGrowthInside1LastX128, liquidity))));
```

**Panoptic:** The Panoptic protocol's CollateralTracker contract has a potential issue where the swap commission paid on the intrinsic value could be zero if the Uniswap pool fee is set below 0.01%. This situation arises because of the way the \_poolFee and s_ITMSpreadFee are calculated. To address this issue, it's recommended to use Uniswap's DECIMALS (1e6) instead of 10,000 and update all related code accordingly. This change ensures that even lower fee levels are appropriately handled by the Panoptic protocol.

```solidity
    function startToken(
        bool underlyingIsToken0,
        address token0,
        address token1,
        uint24 fee,
        PanopticPool panopticPool
    ) external {

        __SNIP__
        // cache the pool fee in basis points
        uint24 _poolFee;
        unchecked {
            _poolFee = fee / 100; // @audit below fee 0.01%, then _poolFee = 0
        }
        s_poolFee = _poolFee;
        ...
        __SNIP__
        // Additional risk premium charged on intrinsic value of ITM positions
        unchecked {
            s_ITMSpreadFee = uint128((ITM_SPREAD_MULTIPLIER * _poolFee) / DECIMALS);
        }
    }
```

### Remediation Summary

To mitigate the risk of dex swap attacks, several strategies can be employed:
**Improved Liquidity Management:** Optimizing liquidity management algorithms to better handle large trade orders without causing significant price movements.
**Dynamic Fee Structure:** Adjusting trading fees based on the size of the order to discourage large orders that could cause slippage.
**Anti-Front Running Measures:** Implementing measures to prevent or mitigate front-running, which could indirectly address slippage attacks.
**Order Execution Delays:** Introducing a slight delay in executing trades after they are placed to allow the market to adjust to changes in liquidity.
**Price Oracles and Decentralized Data Feeds:** Ensuring that the price used for executing trades is accurate and not easily manipulated.
**Layer 2 Scaling Solutions:** Improving the speed and efficiency of DEX transactions, reducing the likelihood of slippage during times of high network congestion.
