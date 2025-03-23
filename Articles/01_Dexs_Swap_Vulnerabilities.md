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
          <th>article</th>
          <td>DEXs_Swap_Vulnerabilities</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

A Decentralized Crypto Exchange (DEX) allows users to trade cryptocurrencies directly without a central intermediary, using smart contracts for trustless operations and letting users retain control of their funds and private keys. Unlike centralized exchanges, which hold user funds and face higher security risks, DEXs distribute trading orders across the network, presenting unique security challenges and navigating a more complex regulatory landscape.

DEXs swap attacks primarily revolve around exploiting vulnerabilities in decentralized exchanges (DEXs), particularly those related to liquidity management, transaction execution, and the inherent trustlessness of these platforms. These attacks can range from sophisticated exploits targeting smart contract vulnerabilities to more straightforward manipulations of market conditions, such as slippage attacks, pool closures, Money Laundering, transaction deadlines, price manipulation, superior swap strategies, sandwich attacks, inaccuracies in calculating amountIn and amountOut, and fee calculations. Here's a detailed look at the types of attacks and how they exploit the unique characteristics of DEXs:

### Attack #1. Slippage Attacks (two issues of slippage protection )

**Definition:** Slippage attacks exploit the difference between the expected price of a trade and the actual executed price due to market volatility or liquidity issues. In DEXs, slippage can occur because of rapid changes in the available liquidity for a particular token pair, especially during periods of high volatility or low liquidity.

**Impact:** Traders may end up receiving fewer tokens than expected when buying or selling, or they may pay a higher price than expected when buying. This can be particularly problematic for larger trades, where the size of the trade order can significantly impact the available liquidity in the market.

**Example:** BakerFi, Lybra Finance, Dopex

**No slippage protection**

**BakerFi:** The protocol's `_swap()` method lacks slippage protection by not setting the `amountOutMinimum` parameter, posing risks of front-running and price manipulation. This issue affects several functions, including `_convertFromWETH()` and `_convertToWETH()`, leading to potential vulnerabilities in multiple swap operations. The recommended fix involves setting `amountOutMinimum` to `params.amountOut` to enforce slippage protection. This issue was confirmed and addressed in a subsequent update, as detailed in [GitHub Pull Request #41](https://github.com/baker-fi/bakerfi-contracts/pull/41).

```solidity
    // amountOutMinium = 0

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
    // minAmount Received

    function rigidRedemption(address provider, uint256 eusdAmount,uint256 minAmountReceived) external virtual {
        depositedAsset[provider] -= collateralAmount;
        totalDepositedAsset -= collateralAmount;
+       require(minAmountReceived <= collateralAmount);
        collateralAsset.transfer(msg.sender, collateralAmount);
        emit RigidRedemption(msg.sender, provider, eusdAmount, collateralAmount, block.timestamp);
    }
```

**Invalid slippage protection**

**Dopex:** In the \_curveSwap function, there is an issue with the order of calling getDpxEthPrice() and getEthPrice(). This mistake results in incorrect slippage calculations when performing swaps between ETH and DPXETH. Specifically, when \_ethToDpxEth is true, indicating a swap from ETH to DPXETH, getDpxEthPrice() is erroneously used, which actually returns the price of DPXETH in terms of ETH (DPXETH/ETH). Conversely, when swapping from DPXETH to ETH, getEthPrice() is incorrectly employed, as it provides the price of ETH in terms of DPXETH (ETH/DPXETH).

```solidity
    // wrong amount and price

    uint256 minOut = _ethToDpxEth
      ? (((_amount * getEthPrice()) / 1e8) -
        (((_amount * getEthPrice) * slippageTolerance) / 1e16))
      : (((_amount * getDpxEthPrice()) / 1e8) -
        (((_amount * getDpxEthPrice()) * slippageTolerance) / 1e16));
```

**Dopex:** The reLP() function in the contract has an incorrect formula for calculating mintokenAAmount, which affects slippage protection during liquidity provision. The formula for calculating mintokenAAmount in the above code is `((amountB / 2) * tokenAInfo.tokenAPrice) / 1e8`. `amountB` is the amount of tokenB, but tokenAInfo.tokenAPrice is the price of tokenA, which shouldn’t be multiplied together.

```solidity
    // wrong amount and price

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

### Attack #2. Price manipulation(Sandwich attacks & front-running)

**Definition:** Price manipulation in the context of decentralized exchanges (DEXs) involves activities where a trader or a group of traders artificially influence the price of assets during swap transactions. This can be achieved through tactics such as front-running, where a manipulator observes a pending large transaction and places their own transactions to benefit from the expected price change, or sandwich attacks, where the manipulator places a buy order before and a sell order after a large transaction to profit from the price movement caused by the large transaction.

**Impact:** Legitimate users may face higher slippage, meaning they receive less favorable prices for their swaps. This can lead to significant financial losses, especially for large transactions. Manipulators can profit at the expense of regular traders, creating an uneven playing field and disadvantaging smaller or less-experienced traders.

**Example:** Dopex, Malt Finance

**Dopex:** The vulnerability in reLPContract.reLP() is identified through its indirect invocation by RdpxV2Core.bond(), allowing potential exploitation via a sandwich attack. This occurs because users can manipulate the bonding amount in RdpxV2Core.bond() to control the amount of liquidity removed and subsequently using ETH from flash loan to inflate rDPX price via UniswapV2 within reLPContract.reLP() without requiring mempool access. reLPContract.reLP() performs swaps and liquidity additions, further impacting prices. Profit realization by swapping rDPX back to ETH and repaying the flash loan. Despite slippage protection measures, the attacker can adjust transaction sizes to profitably exploit price discrepancies.

```solidity
// inflate price by sandwich attack

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

**Malt Finance:** The vulnerability in UniswapHandler involves its interaction with UniswapV2Router during token swaps and liquidity removal, where it fails to set minimum output thresholds, exposing it to frontrunning attacks. This allows malicious actors to manipulate transaction order in the mempool, inflating token prices before executing advantageous trades, thereby profiting at the expense of UniswapHandler and related contracts. Mitigation involves implementing proper price slippage checks and restricting direct access to UniswapV2Router to prevent unauthorized manipulation, crucial for safeguarding against potential financial losses across affected functionalities.

```solidity
// frontrun without minimum output

  function buyMalt()
    external
    onlyRole(BUYER_ROLE, "Must have buyer privs")
    returns (uint256 purchased)
  {
    uint256 rewardBalance = rewardToken.balanceOf(address(this));

    if (rewardBalance == 0) {
      return 0;
    }

    rewardToken.approve(address(router), rewardBalance);

    address[] memory path = new address[](2);
    path[0] = address(rewardToken);
    path[1] = address(malt);

    router.swapExactTokensForTokens(
      rewardBalance,
      0, // amountOutMin
      path,
      address(this),
      now
    );

    purchased = malt.balanceOf(address(this));
    malt.safeTransfer(msg.sender, purchased);
  }

  function sellMalt() external returns (uint256 rewards) {
    uint256 maltBalance = malt.balanceOf(address(this));

    if (maltBalance == 0) {
      return 0;
    }

    malt.approve(address(router), maltBalance);

    address[] memory path = new address[](2);
    path[0] = address(malt);
    path[1] = address(rewardToken);

    router.swapExactTokensForTokens(
      maltBalance,
      0,
      path,
      address(this),
      now
    );

    rewards = rewardToken.balanceOf(address(this));
    rewardToken.safeTransfer(msg.sender, rewards);
  }

    function removeLiquidity() external returns (uint256 amountMalt, uint256 amountReward) {
    uint256 liquidityBalance = lpToken.balanceOf(address(this));

    if (liquidityBalance == 0) {
      return (0, 0);
    }

```

### Attack #3. Transaction deadlines

**Definition:** When making a swap, Automated Market Makers (AMMs) provide users with a deadline parameter, ensuring the swap transaction will only be executed within the specified time frame. Without a deadline, the transaction can remain pending and may be executed long after the user submits it. By that time, the trade may occur at an unfavorable price, negatively impacting the user's position.

**Impact:** Without a deadline, the transaction can be manipulated by validators, resulting in significant slippage and potential losses for users. For example, a user stakes funds, which are converted using Uniswap. A validator could delay the transaction, waiting for a favorable market condition to front-run the original user, causing maximum slippage.

**Example:** Asymmetry, Dexe

**Asymmetry:** Lack of a deadline parameter in the `ISwapRouter.exactInputSingle` function can lead to front-running vulnerabilities, where a validator might delay a transaction to exploit market conditions, causing significant slippage. The problem arises when users stake and Uniswap is used to convert ETH to WETH without a set deadline, making the transaction susceptible to manipulation. The recommended mitigation is to include a user-input deadline parameter in the `Reth.deposit()` function and pass it to the swap functions to prevent such exploits and to ensure that the transaction can be executed in a short period of time. And, introduce a deadline parameter to the functions withdraw() for WstEth.sol and SfrxEth.sol and deposit() for Reth.sol.

```solidity
    // check deadline： outdated maximum slippage value

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

**Dexe:** Using `block.timestamp` as a swap deadline in smart contracts fails to protect against manipulation in proof-of-stake (PoS) systems, where validators can delay transactions to exploit more favorable block timestamps. This undermines the security of time-sensitive operations like swaps. To mitigate this risk, smart contracts should allow users to specify swap deadlines as input parameters rather than relying solely on `block.timestamp`. This approach prevents manipulation by validators and enhances transaction security. Due to these vulnerabilities, functionality dependent on `block.timestamp` for swap deadlines was removed in Dexe to safeguard against potential exploits and ensure fair and secure transactions.

```solidity

// caller specify swap deadline input parameter instead of block.timestamp

        uint256[] memory outs = uniswapV2Router.swapExactTokensForTokens(
            amountIn,
            minAmountOut,
            foundPath.path,
            msg.sender,
            block.timestamp
        );

```

### Attack #4. Loss of accuracy

**Definition:** IUnlike Uniswap v3, KyberSwap Elastic introduces an innovative feature called the Reinvestment Curve. This is an additional AMM pool that accumulates fees charged from users’ swaps in the pool, with a curve that supports a price range from 0 to infinity. KyberSwap Elastic combines the reinvestment curve with the original price curve (i.e., the curves are separate, but the funds remain in the same pool), allowing LPs to compound their fees and earn returns even when the price exceeds their position range.

**Impact:** Due to the Reinvestment Curve feature of KyberSwap Elastic pool, when both base liquidity and reinvestment liquidity are considered as actual liquidity, it calculates the amount of tokens needed for exchange at the scale boundary using the calcReachAmount function. This calculation resulted in a higher than expected amount, causing the next price sqrtP to exceed the boundary scale’s sqrtP. The pool, using an inequality to check sqrtP, led to the protocol not updating liquidity and crossing the tick as expected through \_updateLiquidityAndCrossTick.

**Example:** [KyberSwap](https://slowmist.medium.com/a-deep-dive-into-the-kyberswap-hack-3e13f3305d3a)

**KyberSwap:** The attack on KyberSwap, where the attacker stole funds mostly in Ether, wrapped ether (wETH), and USDC from multiple cross-chain deployments of KyberSwap. This suggests that the theft targeted the liquidity provider pools themselves, indicating a directed attack against the core infrastructure of the DEX. Due to the Reinvestment Curve feature of KyberSwap Elastic pool, when both base liquidity and reinvestment liquidity are considered as actual liquidity, it calculates the amount of tokens needed for exchange at the scale boundary using the calcReachAmount function. This calculation resulted in a higher than expected amount, causing the next price sqrtP to exceed the boundary scale’s sqrtP. The pool, using an inequality to check sqrtP, led to the protocol not updating liquidity and crossing the tick as expected through \_updateLiquidityAndCrossTick.

<div  align="center">
<img src="https://github.com/ETAAcademy/ETAAcademy-Images/blob/main/ETAAcademy-Audit/01_swap.webp?raw=true" width="50%" />
</div>

```solidity
    // fee reinvestment => the next price sqrtP to exceed the boundary scale’s sqrtP. The pool, but not updating liquidity and crossing the tick as expected by Invalid validation
    // flashloan attack => final reverse swap resulted in more funds than anticipated
    // swapData.reinvestL,
    // if swapData.sqrtP != swapData.nexSqrtP

    (usedAmount, returnedAmount, deltaL, swapData.sqrtP) = SwapMath.computeSwapStep(
        swapData.baseL + swapData.reinvestL,
        swapData.sqrtP,
        targetSqrtP,
        swapFeeUnits,
        swapData.specifiedAmount,
        swapData.isExactInput,
        swapData.isToken0
    );

    swapData.specifiedAmount -= usedAmount;
    swapData.returnedAmount += returnedAmount;
    swapData.reinvestL += deltaL.toUint128();
}

// if price has not reached the next sqrt price
if swapData.sqrtP != swapData.nextSqrtP {
    if (swapData.sqrtP != swapData.startSqrtP) {
        //update the current tick data in case the sqrtP has changed
        swapData.currentTick = TickMath.getTickAtSqrtRatio(swapData.sqrtP);
    }
    break;
}
swapData.currentTick = willUpTick ? tempNextTick : tempNextTick - 1;

```

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

### Attack #5. Price Arbitrage

**Definition:** In the context of a decentralized exchange (DEX) swap, the price refers to the exchange rate between two different cryptocurrencies or tokens. This rate determines how much of one token you will receive in exchange for a certain amount of another token. Arbitrage in the context of DEX swaps involves exploiting price discrepancies of the same asset across different markets or liquidity pools to generate a profit.

**Impact:** Legitimate users may face wrong price resources, meaning they receive less favorable prices for their swaps. This can lead to significant financial losses, especially for large transactions.

**Example:** Asymmetry, Dopex

**Asymmetry:** The poolPrice function in the Reth derivative contract risks overflow when calculating the spot price of the derivative asset using a Uniswap V3 pool. This can cause inaccurate price calculations and potential loss of funds or erroneous contract behavior. Example code line that may overflow: sqrtPriceX96 _ (uint(sqrtPriceX96)) _ (1e18). Replace the current calculation with a safer implementation using the OracleLibrary.getQuoteAtTick function from the Uniswap V3 periphery, which includes overflow checks and handles different numerical issues. Asymmetry confirmed using Chainlink to obtain prices instead of relying on the poolPrice function, effectively mitigating the risk of overflow.

```solidity
// pool price

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
// pool price

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

### Attack #6. Fee calculation & operation

**Definition:** A swap fee, also known as a trading fee or transaction fee, is a small percentage of the transaction value that is charged by a decentralized exchange (DEX) like Uniswap for facilitating trades between different token pairs. This fee is typically paid by the trader and is distributed to liquidity providers (LPs) as an incentive for supplying liquidity to the pool.

**Impact:** The primary impact of this bug is the loss of swap commission fees that should be collected on ITM positions. This affects the profitability of the protocol since it reduces the overall revenue generated from swap commissions.

**Example:** Panoptic, Tally, Vader Protocol, OpenLeverage, Goat

**Panoptic:** The main invariant of Panoptic, "Fees paid to a given user should not exceed the amount of fees earned by the liquidity owned by that user," can be broken due to a slight difference in fee computation methods. Panoptic calculates fees as (currFeeGrowth _ liquidity / Q128) - (prevFeeGrowth _ liquidity / Q128), whereas Uniswap V3 calculates it as (currFeeGrowth - prevFeeGrowth) \* liquidity / Q128. This difference can result in a user collecting more fees than they should, potentially reducing the fees available for other users.

```solidity
// difference between equation

int256 amountToCollect = _getFeesBase(univ3pool, startingLiquidity, liquidityChunk).sub(
            s_accountFeesBase[positionKey]
        );
    feesBase = int256(0)
                .toRightSlot(int128(int256(Math.mulDiv128(feeGrowthInside0LastX128, liquidity))))
                .toLeftSlot(int128(int256(Math.mulDiv128(feeGrowthInside1LastX128, liquidity))));
```

**Panoptic:** The Panoptic protocol's CollateralTracker contract has a potential issue where the swap commission paid on the intrinsic value could be zero if the Uniswap pool fee is set below 0.01%. This situation arises because of the way the \_poolFee and s_ITMSpreadFee are calculated. To address this issue, it's recommended to use Uniswap's DECIMALS (1e6) instead of 10,000 and update all related code accordingly. This change ensures that even lower fee levels are appropriately handled by the Panoptic protocol.

```solidity
// decimals

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

**Tally:** The use of `transfer()` in `Swap.sol` can cause ETH to be irretrievable or undelivered if the recipient is a smart contract that either lacks a payable fallback function or requires more than 2300 gas units to execute its fallback function. This can lead to potential loss of funds, particularly if gas costs change. To mitigate this, it is recommended to replace `transfer()` with `msg.sender.call.value(amount)` or use the OpenZeppelin `Address.sendValue` library, as re-entrancy has already been accounted for in the contract.

```solidity
    // transfer() swap fee

    /// @notice Sweeps accrued ETH and ERC20 swap fees to the pre-established
    ///         fee recipient address.
    /// @dev Fees are tracked based on the contract's balances, rather than
    ///      using any additional bookkeeping. If there are bugs in swap
    ///      accounting, this function could jeopardize funds.
    /// @param tokens An array of ERC20 contracts to withdraw token fees
    function sweepFees(
        address[] calldata tokens
    ) external nonReentrant {
        require(
            feeRecipient != address(0),
            "Swap::withdrawAccruedFees: feeRecipient is not initialized"
        );
        for (uint8 i = 0; i<tokens.length; i++) {
            uint256 balance = IERC20(tokens[i]).balanceOf(address(this));
            if (balance > 0) {
                IERC20(tokens[i]).safeTransfer(feeRecipient, balance);
                emit FeesSwept(tokens[i], balance, feeRecipient);
            }
        }
@=>audit        feeRecipient.transfer(address(this).balance);
        emit FeesSwept(address(0), address(this).balance, feeRecipient);
    }

```

**Tally:** Users can exploit the `Swap.swapByQuote()` function to execute an ETH swap without paying the required swap fees by tricking the system into believing an ERC20 swap is occurring. This is achieved by setting `zrxBuyTokenAddress` to a malicious contract, which manipulates the balance checks to make the system treat the gained ETH as a refund rather than part of the swap, thus bypassing the fee. The recommended fix is to ensure swap fees are charged on refunded ETH in such scenarios. Charge swap fees for the “refunded ETH” on ERC20 swaps (when boughtERC20Amount > 0), or require boughtETHAmount == 0.

```solidity
// not charge fee

    function swapByQuote(
        address zrxSellTokenAddress,
        uint256 amountToSell,
        address zrxBuyTokenAddress,
        uint256 minimumAmountReceived,
        address zrxAllowanceTarget,
        address payable zrxTo,
        bytes calldata zrxData,
        uint256 deadline
    ) external payable whenNotPaused nonReentrant {
...
        if (boughtERC20Amount > 0) {
            // take the swap fee from the ERC20 proceeds and return the rest
            uint256 toTransfer = SWAP_FEE_DIVISOR.sub(swapFee).mul(boughtERC20Amount).div(SWAP_FEE_DIVISOR);
            IERC20(zrxBuyTokenAddress).safeTransfer(msg.sender, toTransfer);
            // return any refunded ETH
            payable(msg.sender).transfer(boughtETHAmount);

            emit SwappedTokens(
                zrxSellTokenAddress,
                zrxBuyTokenAddress,
                amountToSell,
                boughtERC20Amount,
                boughtERC20Amount.sub(toTransfer)
            );
        } else {

            // take the swap fee from the ETH proceeds and return the rest. Note
            // that if any 0x protocol fee is refunded in ETH, it also suffers
            // the swap fee tax
            uint256 toTransfer = SWAP_FEE_DIVISOR.sub(swapFee).mul(boughtETHAmount).div(SWAP_FEE_DIVISOR);
            payable(msg.sender).transfer(toTransfer);
            emit SwappedTokens(
                zrxSellTokenAddress,
                zrxBuyTokenAddress,
                amountToSell,
                boughtETHAmount,
                boughtETHAmount.sub(toTransfer)
            );
        }
        if (zrxAllowanceTarget != address(0)) {
            // remove any dangling token allowance
            IERC20(zrxSellTokenAddress).safeApprove(zrxAllowanceTarget, 0);
        }
    }

```

**Vader Protocol**: The issue centers around the BasePoolV2.sol contract's mint() function, which assumes that transferred amounts equal received amounts, leading to potential discrepancies when handling ERC20 tokens like Vader that charge fees on transfers. This function transfers assets and calculates liquidity units based on these amounts, potentially ignoring fees incurred during transfers.

```solidity
// not charge fee

function mint(
    IERC20 foreignAsset,
    uint256 nativeDeposit,
    uint256 foreignDeposit,
    address from,
    address to
)
    external
    override
    nonReentrant
    onlyRouter
    supportedToken(foreignAsset)
    returns (uint256 liquidity)
{
    (uint112 reserveNative, uint112 reserveForeign, ) = getReserves(
        foreignAsset
    ); // gas savings

    nativeAsset.safeTransferFrom(from, address(this), nativeDeposit);
    foreignAsset.safeTransferFrom(from, address(this), foreignDeposit);

    PairInfo storage pair = pairInfo[foreignAsset];
    uint256 totalLiquidityUnits = pair.totalSupply;
    if (totalLiquidityUnits == 0) liquidity = nativeDeposit;
    else
        liquidity = VaderMath.calculateLiquidityUnits(
            nativeDeposit,
            reserveNative,
            foreignDeposit,
            reserveForeign,
            totalLiquidityUnits
        );

    require(
        liquidity > 0,
        "BasePoolV2::mint: Insufficient Liquidity Provided"
    );

    uint256 id = positionId++;

    pair.totalSupply = totalLiquidityUnits + liquidity;
    _mint(to, id);

    positions[id] = Position(
        foreignAsset,
        block.timestamp,
        liquidity,
        nativeDeposit,
        foreignDeposit
    );

    _update(
        foreignAsset,
        reserveNative + nativeDeposit,
        reserveForeign + foreignDeposit,
        reserveNative,
        reserveForeign
    );

    emit Mint(from, to, nativeDeposit, foreignDeposit);
    emit PositionOpened(from, to, id, liquidity);
}

```

### Attack #7. Fee on transfer

**Definition:** Some tokens take a transfer fee (e.g. STA, PAXG), some do not currently charge a fee but may do so in the future (e.g. USDT, USDC).

**Impact:** A vulnerability fails to account for fee-on-transfer tokens, resulting in potential system funds deficits and user fund freezes, where the actual amount of tokens received is less than expected due to transfer fees.

**OpenLeverage:** The OpenLevV1 contract has a vulnerability when interacting with V3 DEX for closing trades, where it fails to account for fee-on-transfer tokens, resulting in potential system funds deficits and user fund freezes. Malicious users can exploit this by repeatedly opening and closing positions with taxed tokens, draining contract funds. The issue primarily arises when the received funds are less than indicated by the DEX, leading to incorrect repayment accounting and failures in the `closeTrade` function. The recommended mitigation is to implement balance checks before and after DEX operations to ensure accurate accounting.

```solidity
// receive less amount to drain the funds

    function closeTrade(uint16 marketId, bool longToken, uint closeHeld, uint minOrMaxAmount, bytes memory dexData) external override nonReentrant onlySupportDex(dexData) {
        // In the trade.depositToken != longToken case when flashSell is used this can imply inability to send remainder funds to a user and the failure of the whole closeTrade function, the end result is a freezing of user’s funds within the system.

        if (trade.depositToken != longToken) {
            minOrMaxAmount = Utils.maxOf(closeTradeVars.repayAmount, minOrMaxAmount);
            closeTradeVars.receiveAmount = flashSell(marketId, address(marketVars.buyToken), address(marketVars.sellToken), closeTradeVars.closeAmountAfterFees, minOrMaxAmount, dexData);
            require(closeTradeVars.receiveAmount >= closeTradeVars.repayAmount, "ISR");

            closeTradeVars.sellAmount = closeTradeVars.closeAmountAfterFees;
            marketVars.buyPool.repayBorrowBehalf(msg.sender, closeTradeVars.repayAmount);

            closeTradeVars.depositReturn = closeTradeVars.receiveAmount.sub(closeTradeVars.repayAmount);
            doTransferOut(msg.sender, marketVars.buyToken, closeTradeVars.depositReturn);
        } else {
            uint balance = marketVars.buyToken.balanceOf(address(this));
            minOrMaxAmount = Utils.minOf(closeTradeVars.closeAmountAfterFees, minOrMaxAmount);
            closeTradeVars.sellAmount = flashBuy(marketId, address(marketVars.buyToken), address(marketVars.sellToken), closeTradeVars.repayAmount, minOrMaxAmount, dexData);
            closeTradeVars.receiveAmount = marketVars.buyToken.balanceOf(address(this)).sub(balance);
            require(closeTradeVars.receiveAmount >= closeTradeVars.repayAmount, "ISR");

            marketVars.buyPool.repayBorrowBehalf(msg.sender, closeTradeVars.repayAmount);
            closeTradeVars.depositReturn = closeTradeVars.closeAmountAfterFees.sub(closeTradeVars.sellAmount);
            require(marketVars.sellToken.balanceOf(address(this)) >= closeTradeVars.depositReturn, "ISB");
            doTransferOut(msg.sender, marketVars.sellToken, closeTradeVars.depositReturn);
        }

    }

```

**OpenLeverage:** The uniClassSell() function relies on getAmountOut() to determine the buy amount, which does not account for tokens with fees on transfer. This can result in receiving fewer tokens than expected, potentially falling below minBuyAmount. Update uniClassSell() to use the actual balance received rather than the output from getAmountOut() to ensure the minimum buy amount requirement is met.

```solidity
// not support fee on transfer
// check operates on getAmountOut() and not the bought output.

function uniClassSell(DexInfo memory dexInfo,
    address buyToken,
    address sellToken,
    uint sellAmount,
    uint minBuyAmount,
    address payer,
    address payee
) internal returns (uint bought){
    address pair = getUniClassPair(buyToken, sellToken, dexInfo.factory);
    IUniswapV2Pair(pair).sync();
    (uint256 token0Reserves, uint256 token1Reserves,) = IUniswapV2Pair(pair).getReserves();
    sellAmount = transferOut(IERC20(sellToken), payer, pair, sellAmount);
    uint balanceBefore = IERC20(buyToken).balanceOf(payee);
    dexInfo.fees = getPairFees(dexInfo, pair);
    if (buyToken < sellToken) {
        buyAmount = getAmountOut(sellAmount, token1Reserves, token0Reserves, dexInfo.fees);
        IUniswapV2Pair(pair).swap(buyAmount, 0, payee, "");
    } else {
        buyAmount = getAmountOut(sellAmount, token0Reserves, token1Reserves, dexInfo.fees);
        IUniswapV2Pair(pair).swap(0, buyAmount, payee, "");
    }
    uint bought = IERC20(buyToken).balanceOf(payee).sub(balanceBefore);
    require(bought >= minBuyAmount, 'buy amount less than min');
}

```

**Goat Trading:** Some tokens take a transfer fee (e.g. STA, PAXG), some do not currently charge a fee but may do so in the future (e.g. USDT, USDC). The router is not designed to handle tokens that charge a fee on transfers. This causes issues in various functions, such as removeLiquidity, where the actual amount of tokens received is less than expected due to transfer fees. Add functionality to the router to support fee on transfer tokens, a good example of where this is correctly implememented is the [Uniswap Router02](https://etherscan.io/address/0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D).

```solidity

address pair = GoatV1Factory(FACTORY).getPool(token);

IERC20(pair).safeTransferFrom(msg.sender, pair, liquidity); //-> 1. Transfers liquidity tokens to the pair
(amountWeth, amountToken) = GoatV1Pair(pair).burn(to); //-> 2. Burns the liquidity tokens and sends WETH and TOKEN to the recipient
if (amountWeth < wethMin) { //-> 3. Ensures enough WETH has been transferred
    revert GoatErrors.InsufficientWethAmount();
}
if (amountToken < tokenMin) { //4. Ensures enough TOKEN has been transferred
    revert GoatErrors.InsufficientTokenAmount();
}


```

### Attack #8. Ignore liquidity

**Definition:** Pool liquidity refers to the availability of assets in a liquidity pool on a decentralized exchange (DEX) like Uniswap or Sushiswap. Liquidity pools are collections of funds locked in a smart contract, provided by liquidity providers (LPs) to facilitate trading. The liquidity in a pool determines how easily assets can be swapped without significant price impact. High liquidity means trades can be executed smoothly with minimal slippage, while low liquidity can lead to higher slippage and difficulty in executing large trades.

**Impact:** liquidity mismatch leads to reduced market efficiency and potential financial losses for users relying on the adapter for accurate liquidity assessment.

**Example:** Mochi

**Mochi:** The `UniswapV2TokenAdapter` does not support assets exclusive to Sushiswap due to its `supports` function only checking UniswapV2 liquidity. If the liquidity is below the minimum threshold, it returns false without considering Sushiswap's liquidity. This could allow an attacker to create an empty UniswapV2 pool, causing the adapter to ignore a potentially liquid Sushiswap pool. The recommended solution is to compare Sushiswap liquidity if UniswapV2 liquidity is insufficient. This issue is confirmed by ryuheimat (Mochi).

```solidity
// ignore other pool's liquidity

try uniswapCSSR.getLiquidity(_asset, _pairedWith) returns (
            uint256 liq
        ) {
    float memory price = cssrRouter.getPrice(_pairedWith);
    // @audit this returns early. if it's false it should check sushiswap first
    return convertToValue(liq, price) >= minimumLiquidity;
} catch {
    try sushiCSSR.getLiquidity(_asset, _pairedWith) returns (
        uint256 liq
    ) {
        float memory price = cssrRouter.getPrice(_pairedWith);
        return convertToValue(liq, price) >= minimumLiquidity;
    } catch {
        return false;
    }
}

```

### Attack #9. Access control

**Definition:** DEXs allow users to trade cryptocurrencies directly without relying on a central authority. However, Users are not supposed to interact with the pool contracts directly.

**Impact:** Without proper validation, users might inject malicious code or manipulate swap parameters for personal gain. Bypassing checks could lead to unintended consequences during the swap process, potentially causing financial losses or disrupting DEX operations.

**Example:** Vader Protocol

**Vader Protocol:** The `BasePool.swap()` function in the Vader protocol lacks an `onlyRouter` modifier, unlike `BasePoolV2.swap()`. This allows users to directly call `BasePool.swap()` without necessary input validation, potentially bypassing checks performed by `VaderRouter._swap()`. The impact is that users can exploit this function directly, leading to potential misuse. The recommended fix is to add an `onlyRouter` modifier to `BasePool.swap()`.

```solidity
// lacks `onlyRouter` modifier, although same to UniswapV2

    function swap(
        uint256 nativeAmountIn,
        uint256 foreignAmountIn,
        address to
    ) public override nonReentrant validateGas returns (uint256) {
        require(
            (nativeAmountIn > 0 && foreignAmountIn == 0) ||
                (nativeAmountIn == 0 && foreignAmountIn > 0),
            "BasePool::swap: Only One-Sided Swaps Supported"
        );
        (uint112 nativeReserve, uint112 foreignReserve, ) = getReserves(); // gas savings

        uint256 nativeBalance;
        uint256 foreignBalance;
        uint256 nativeAmountOut;
        uint256 foreignAmountOut;
        {
            // scope for _token{0,1}, avoids stack too deep errors
            IERC20 _nativeAsset = nativeAsset;
            IERC20 _foreignAsset = foreignAsset;
            nativeBalance = _nativeAsset.balanceOf(address(this));
            foreignBalance = _foreignAsset.balanceOf(address(this));

            require(
                to != address(_nativeAsset) && to != address(_foreignAsset),
                "BasePool::swap: Invalid Receiver"
            );

            if (foreignAmountIn > 0) {
                require(
                    foreignAmountIn <= foreignBalance - foreignReserve,
                    "BasePool::swap: Insufficient Tokens Provided"
                );
                require(
                    foreignAmountIn <= foreignReserve,
                    "BasePool::swap: Unfavourable Trade"
                );

                nativeAmountOut = VaderMath.calculateSwap(
                    foreignAmountIn,
                    foreignReserve,
                    nativeReserve
                );

                require(
                    nativeAmountOut > 0 && nativeAmountOut <= nativeReserve,
                    "BasePool::swap: Swap Impossible"
                );

                _nativeAsset.safeTransfer(to, nativeAmountOut); // optimistically transfer tokens
            } else {
                require(
                    nativeAmountIn <= nativeBalance - nativeReserve,
                    "BasePool::swap: Insufficient Tokens Provided"
                );
                require(
                    nativeAmountIn <= nativeReserve,
                    "BasePool::swap: Unfavourable Trade"
                );

                foreignAmountOut = VaderMath.calculateSwap(
                    nativeAmountIn,
                    nativeReserve,
                    foreignReserve
                );

                require(
                    foreignAmountOut > 0 && foreignAmountOut <= foreignReserve,
                    "BasePool::swap: Swap Impossible"
                );

                _foreignAsset.safeTransfer(to, foreignAmountOut); // optimistically transfer tokens
            }

            nativeBalance = _nativeAsset.balanceOf(address(this));
            foreignBalance = _foreignAsset.balanceOf(address(this));
        }

        _update(nativeBalance, foreignBalance, nativeReserve, foreignReserve);

        emit Swap(
            msg.sender,
            nativeAmountIn,
            foreignAmountIn,
            nativeAmountOut,
            foreignAmountOut,
            to
        );

        return nativeAmountOut > 0 ? nativeAmountOut : foreignAmountOut;
    }

```

### Attack #10. Swap type & calculation

**Definition:** In decentralized finance (DeFi), liquidity pools enable the trading of cryptocurrencies by maintaining a constant product formula, which is crucial for determining the price of tokens within the pool. swap calculation ensures that the product of the amounts of the two tokens in the pool remains constant before and after a swap, allowing for dynamic pricing based on the ratio of the tokens in the pool.

**Impact:** Any bugs or inaccuracies in swap calculations can lead to incorrect pricing information, impacting both liquidity providers and traders.

**Example:** Vader Protocol

**Vader Protocol:** The issue in `VaderRouter._swap` involves incorrect handling of swap arguments within a 3-path swap sequence. The function is designed to perform a series of swaps where foreign assets are first exchanged for native assets, and then the received native assets are swapped for different foreign assets. However, the implementation mistakenly uses the `amountIn` parameter as `nativeAmountIn` when calling the swap functions on `pool0` and `pool1`, leading to incorrect behavior. This results in failed swap operations due to mismatches in expected native and foreign amounts, impacting the functionality of 3-path swaps through `VaderRouter`. The recommended mitigation is to correct the argument order to ensure proper execution of the swap sequence, specifically using `pool1.swap(pool0.swap(0, amountIn, address(pool1)), 0, to);` to correctly handle foreign and native asset swaps in sequence.

```solidity

// wrong swap order from native to foreign assets

function _swap(
    uint256 amountIn,
    address[] calldata path,
    address to
) private returns (uint256 amountOut) {
    if (path.length == 3) {
      // ...
      // @audit calls this with nativeAmountIn = amountIn. but should be foreignAmountIn (second arg)
      return pool1.swap(0, pool0.swap(amountIn, 0, address(pool1)), to);
    }
}
// @audit should be this instead
return pool1.swap(pool0.swap(0, amountIn, address(pool1)), 0, to);

```

**Vader Protocol:** The issue in `VaderRouter.calculateOutGivenIn` involves incorrect sequence handling of swap operations within a 3-path swap scenario. The function is intended to execute a sequence where foreign assets are first swapped to native assets in `pool0`, followed by exchanging the received native assets for different foreign assets in `pool1`. However, the code mistakenly performs the initial swap in `pool1` instead of `pool0`, resulting in incorrect calculations of asset swaps. This error affects all computations for 3-path swaps, potentially causing transaction failures or financial losses for smart contracts or frontend applications relying on accurate swap calculations. The recommended fix involves correcting the sequence to ensure swaps begin correctly in `pool0` before proceeding to `pool1`, thereby ensuring accurate swap outcomes and preventing potential financial risks.

```solidity
// wrong calculation by order

function calculateOutGivenIn(uint256 amountIn, address[] calldata path)
    external
    view
    returns (uint256 amountOut)
{
  if(...) {
  } else {
    return
        VaderMath.calculateSwap(
            VaderMath.calculateSwap(
                // @audit the inner trade should not be in pool1 for a forward swap. amountIn foreign => next param should be foreignReserve0
                amountIn,
                nativeReserve1,
                foreignReserve1
            ),
            foreignReserve0,
            nativeReserve0
        );
  }
 /** @audit instead should first be trading in pool0!
    VaderMath.calculateSwap(
        VaderMath.calculateSwap(
            amountIn,
            foreignReserve0,
            nativeReserve0
        ),
        nativeReserve1,
        foreignReserve1
    );
  */

```

### Attack #11. Superior swap strategies

**Definition:** Superior swap strategies involve using liquidity pools with greater liquidity to reduce slippage and improve trading efficiency. For example, depositing more ETH than the available in a pool can cause the swap to fail due to slippage protection. Instead of making large deposits in one go, the strategy should involve multiple smaller steps. Implementing a timelock mechanism that can dynamically move funds between different protocols allows for more granular rebalancing, enhanced transparency, and gives users time to react to changes in protocol exposure.

**Impact:** By using liquidity pools with higher liquidity, users can achieve lower slippage and better trade execution. This approach not only optimizes the use of available liquidity but also enhances the overall user experience by providing more reliable and efficient swaps. Additionally, the use of a timelock mechanism ensures better fund management and allows users to make informed decisions based on changes in protocol exposure, thereby increasing trust and transparency in the system.

**Example:** Asymmetry

**Asymmetry:** Uniswap rETH/WETH pool for swaps has higher fees and lower liquidity compared to alternatives like Balancer and Curve pools. The Uniswap pool has $5 million in liquidity with a 0.05% fee, while Balancer has $80 million in liquidity with a 0.04% fee, and Curve has $8 million in liquidity with a 0.037% fee. The use of the Uniswap pool results in higher slippage and unnecessary fees, diminishing user value. The recommended mitigation is to use RocketPool’s RocketSwapRouter.sol contract’s swapTo() function, which optimizes the swap path between Balancer and Uniswap pools. Alternatively, modifying Reth.sol to use the Balancer pool would also reduce swap fees and slippage costs.

```solidity
// lower fees and higher liquidity

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

### Attack #12. Governance check & manipulation

**Definition:** Governance in decentralized finance (DeFi) plays a crucial role in shaping the future of protocols, including those focused on swapping tokens. Governance mechanisms allow the community to make collective decisions regarding the protocol's direction, including changes to fee structures, addition of new tokens, or adjustments to liquidity incentives.

**Impact:** Swap governance attacks in decentralized finance (DeFi) target the governance mechanisms of protocols that facilitate token swaps. These attacks aim to exploit vulnerabilities in the governance process, potentially leading to unauthorized changes in the protocol's parameters, such as fee structures, liquidity incentives, or even the introduction of new tokens.

**Example:** PoolTogether，SolidlyV3AMM

**PoolTogether:** The vulnerability identified in `SwappableYieldSource.sol` involves the `swapYieldSource` function, which allows the owner or asset manager to switch the yield source contract at any time. This capability could potentially be exploited maliciously to immediately withdraw all funds from the current yield source to a new, possibly malicious contract that implements a compatible `depositToken()` function. This could lead to a rug pull scenario where funds are effectively stolen. Recommendations to mitigate this risk include implementing checks to ensure the new yield source is from a trusted registry, or enforcing a timelock mechanism for governance approval before executing such swaps.

```solidity
// not check _newYieldSource(yield receiver) contract

  /// @notice Swap current yield source for new yield source.
  /// @dev This function is only callable by the owner or asset manager.
  /// @dev We set a new yield source and then transfer funds from the now previous yield source to the new current yield source.
  /// @param _newYieldSource New yield source address to set and transfer funds to.
  /// @return true if operation is successful.
  function swapYieldSource(IYieldSource _newYieldSource) external onlyOwnerOrAssetManager returns (bool) {
    IYieldSource _currentYieldSource = yieldSource;
    uint256 balance = _currentYieldSource.balanceOfToken(address(this));

    _setYieldSource(_newYieldSource);
    _transferFunds(_currentYieldSource, balance);

    return true;
  }

```

**SolidlyV3AMM:** The Solidly V3 AMM protocol allows the owner to control swap fees through the feeCollector role assigned by the SolidlyV3Factory. This role, initially held by a RewardsDistributor contract, lets the owner set a Merkle root determining who can claim fees. Risks include the owner directing fees to themselves, setting a Merkle root that benefits only them, or leaving fees unclaimable indefinitely. Despite using a multi-sig and Timelock contract, the 24-hour delay may not suffice for LPs to claim their yield, suggesting a need to reconsider the fee claiming mechanism(owner direct fees to themselves).

### Appendix. Money Laundering Through DEXs

**Definition:** Criminals can use DEXs for crypto-to-crypto swaps to launder criminal proceeds. This involves obtaining Ether or Ethereum-based tokens (for example, by hacking an exchange), swapping them at a DEX for new tokens, and then depositing these new tokens at a legitimate exchange to cash out for fiat.

**Impact:** While DEXs offer advantages in terms of bypassing compliance controls and lacking a central administrator, they also present challenges for money laundering. All DEX crypto-to-crypto swaps are recorded in smart contracts on the blockchain, allowing for visibility into these transactions and the potential tracing of illicit funds.

**Example:** Tornado cash, Blender.io

**Tornado cash:** By sending illicit funds to Tornado Cash, criminals can obfuscate the funds trail – making it more difficult to decipher their activity. In the recent Ronin Bridge hack attributed to North Korea’s Lazarus Group, the hackers made extensive use of Tornado Cash to launder some of the stolen cryptoassets from the heist, which at the time of the theft totalled $540 million.

A money laundering typology involving DEXs works as follows:
1)a criminal obtains Ether or Ethereum-based tokens, for example by hacking a DeFi lending platform;
2)the criminal sends the stolen funds to a Tornado Cash address;
3)the criminal receives new “clean” tokens from Tornado cash; and
4)the new tokens are deposited at a centralized exchange platform, and cashed out for fiat.

**Blender.io:** As an aside, Bitcoin mixer Blender.io recently became the first virtual asset mixer to be targeted by sanctions from the US Office of Foreign Assets Control (OFAC). However, it is a mixer that handles Bitcoin exclusively.

### Remediation Summary

To mitigate the risk of dex swap attacks, several strategies can be employed:

**Improved Liquidity Management:** Optimizing liquidity management algorithms to better handle large trade orders without causing significant price movements.

**Dynamic Fee Structure:** Adjusting trading fees based on the size of the order to discourage large orders that could cause slippage.

**Anti-Front Running Measures:** Implementing measures to prevent or mitigate front-running, which could indirectly address slippage attacks.

**Order Execution Delays:** Introducing a slight delay in executing trades after they are placed to allow the market to adjust to changes in liquidity.

**Price Oracles and Decentralized Data Feeds:** Ensuring that the price used for executing trades is accurate and not easily manipulated.

**Layer 2 Scaling Solutions:** Improving the speed and efficiency of DEX transactions, reducing the likelihood of slippage during times of high network congestion.
