# ETAAcademy-Adudit: 5. 2024 Contract Security

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>05. 2024 Contract Security</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>2024_Contract_Security</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# 2024 Common Smart Contract Vulnerabilities and Case Studies

The blockchain ecosystem has seen rapid growth, but it remains vulnerable to various smart contract flaws. This article explores some of the most common vulnerabilities identified in 2024, along with real-world cases that highlight their devastating effects.

These vulnerabilities emphasize the importance of rigorous code audits, robust testing, and implementing safeguards against invalid access control, input validation, misconfiguration, price manipulation, rounding errors, reentrancy, storage collision, backdoor, cross-chain interaction bug, faulty contract logic and ecosystem-specific issues.

---

## **1. Lack of Access Control**

**Issue**: Critical functions that should only be accessible to authorized entities are sometimes mistakenly left public.  
**Impact**: Attackers exploit unrestricted access to perform unauthorized actions like minting, burning tokens, or transferring assets.

### **1) Ark Token – Unrestricted LP Token Burn**

**Vulnerability**: The Ark Token contract had a **public** function for burning LP tokens, which should have been restricted using mechanisms like `onlyOwner`. This lack of access control allowed anyone to burn LP tokens arbitrarily.

**Impact**: Attackers exploited this flaw to manipulate the liquidity pool's swap ratio by burning LP tokens. This drained the pool of 348 WBNB, worth approximately $200,000.

**Funds Flow**:  
The attackers employed sophisticated methods to launder the stolen funds:

- **Transfers**: Part of the funds (108 BNB) was sent to SwiftSwap, while the rest (240 BNB) was deposited into Binance accounts.
- **Withdrawals and Bridging**: 210 BNB was withdrawn and bridged to Tron as 158,000 USDT.
- **Laundering**: Funds were further cleaned using multiple exchanges and cross-chain bridges, obscuring their trail.

### **2) Galaxy Fox Token – Forged Merkle Root for Token Claim**

**Vulnerability**: The `setMerkleRoot` function was publicly accessible, allowing attackers to replace the Merkle root without restriction.

**Impact**: By forging the Merkle root, the attackers manipulated the `claim` function to extract all available funds, resulting in a loss of $330,000.

**Funds Flow**:

- **Ethereum Chain**: Part of the stolen funds (8.1 ETH) was laundered via Tornado Cash.
- **BSC Chain**: Assets were bridged to BSC (33.4 ETH) and exchanged into 179.2 BNB using XY Finance.
- **Further Laundering**: 399.2 BNB was processed through Tornado Cash and deposited into centralized exchanges.

#### **3）Predy Finance – Exploiting Callback to Withdraw Funds**

**Vulnerability**: The `trade` function's callback mechanism lacked access control, allowing attackers to assign themselves the **Locker** role, which governs fund withdrawals.

**Impact**: Attackers used the callback to set their address as the **Locker** role, enabling unauthorized use of the `take` function to withdraw all funds.

**Funds Flow**:

- Stolen funds (101 ETH) were bridged from Arbitrum to Ethereum using Across Protocol.
- Tornado Cash was used to launder the assets, hiding the funds' origin.

<details><summary>Code</summary>

```solidity

    function trade(
        GlobalDataLibrary.GlobalData storage globalData,
        IPredyPool.TradeParams memory tradeParams,
        bytes memory settlementData
    ) external returns (IPredyPool.TradeResult memory tradeResult) {
        DataType.PairStatus storage pairStatus = globalData.pairs[tradeParams.pairId];

        // update interest growth
        ApplyInterestLib.applyInterestForToken(globalData.pairs, tradeParams.pairId);

        // update rebalance interest growth
        Perp.updateRebalanceInterestGrowth(pairStatus, pairStatus.sqrtAssetStatus);

        tradeResult = Trade.trade(globalData, tradeParams, settlementData);

        globalData.vaults[tradeParams.vaultId].margin +=
            tradeResult.fee + tradeResult.payoff.perpPayoff + tradeResult.payoff.sqrtPayoff;

        (tradeResult.minMargin,,, tradeResult.sqrtTwap) = PositionCalculator.calculateMinDeposit(
            pairStatus, globalData.vaults[tradeParams.vaultId], DataType.FeeAmount(0, 0)
        );

        // The caller deposits or withdraws margin from the callback that is called below.
        callTradeAfterCallback(globalData, tradeParams, tradeResult);

        // check vault safety
        tradeResult.minMargin =
            PositionCalculator.checkSafe(pairStatus, globalData.vaults[tradeParams.vaultId], DataType.FeeAmount(0, 0));

        emit PositionUpdated(
            tradeParams.vaultId,
            tradeParams.pairId,
            tradeParams.tradeAmount,
            tradeParams.tradeAmountSqrt,
            tradeResult.payoff,
            tradeResult.fee
        );
    }

    function callTradeAfterCallback(
        GlobalDataLibrary.GlobalData storage globalData,
        IPredyPool.TradeParams memory tradeParams,
        IPredyPool.TradeResult memory tradeResult
    ) internal {
        globalData.initializeLock(tradeParams.pairId);

        IHooks(msg.sender).predyTradeAfterCallback(tradeParams, tradeResult);

        (int256 marginAmountUpdate, int256 settledBaseAmount) = globalData.finalizeLock();

        if (settledBaseAmount != 0) {
            revert IPredyPool.BaseTokenNotSettled();
        }

        globalData.vaults[tradeParams.vaultId].margin += marginAmountUpdate;

        emit MarginUpdated(tradeParams.vaultId, marginAmountUpdate);
    }

```

</details>

---

## **2. Improper Input Validation**

**Issue**: Insufficient validation of user input can allow attackers to bypass logic checks or manipulate contracts.  
**Impact**: Attackers can exploit these flaws to steal funds or bypass security mechanisms.

### **1) Socket.Tech – Exploiting an Unreviewed Module**

**Vulnerability**: A new, unreviewed `WrappedTokenSwapperImpl` contract was deployed and integrated into the system. The flawed contract allowed attackers to execute arbitrary calls using previously approved tokens.

**Impact**: Roughly 200 users who interacted with Socket API and Bungee Exchange were affected, resulting in a loss of $1 million.

**Funds Flow**: The attackers returned 80% of the stolen funds, and 1,027 ETH was recovered by the team after negotiations.

<details><summary>Code</summary>

```solidity

// File 68 of 70 : swapWrappedlmpl.sol

    // Send weth to user
    ERC20(toToken).transfer(receiverAddress, amount);
 } else {
    _initialBalanceTokenOut = address(socketGateway).balance;

    // Swap Wrapped Token To Native Token
    ERC20(fromToken).safeTransferFrom(
        msg.sender,
        socketGateway,
        amount
    );

    (bool success, ) = fromToken.call(swapExtraData);

    if (!success) {
        revert SwapFailed();
    }

    _finalBalanceTokenOut = address(socketGateway).balance;

    require(
        (_finalBalanceTokenOut - _initialBalanceTokenOut) == amount.
        "Invalid wrapper contract"

```

</details>

### **2) LI.FI – Vulnerable `swap` Function**

**Vulnerability**: The `swap` function failed to properly validate the target address and calldata. This flaw enabled attackers to drain assets from users who had approved the contract to spend their tokens.

**Impact**: Over $10 million worth of assets was stolen during the attack.

**Funds Flow**: The stolen funds were laundered through Tornado Cash over 69 days. The attacker dispersed assets across multiple addresses and further concealed them through centralized exchanges.

<details><summary>Code</summary>

```solidity

function depositToGasZipERC20(
    LibSwap.SwapData calldata _swapData,
    uint256 _destinationChains,
    address _recipient
)public {
    // get the current native balance
    uint256 currentNativeBalance = address(this).balance;

    // execute the swapData that swaps the ERC20 token into native
    LibSwap.swap(0, _swapData);

    // calculate the swap output amount using the initial native balance
    uint256 swapOutputAmount = address(this).balance - currentNativeBalance;

    // call the gas zip router and deposit tokens
    gasZipRouter.deposit{ value: swapOutputAmount }(
        _destinationChains,
        _recipient
    );
}

```

```solidity

// solhint-disable-next-line avoid-low-level-calls
(bool success, bytes memory res) = _swap.callTo.call{
    value: nativeValue
}(_swap.callData);
if (!success) {
    LibUtil.revertWith(res);
}

```

</details>

---

## **3. Misconfiguration**

**Issue**: Misconfigured smart contracts, such as incorrect initialization or erroneous variable settings, can lead to logical errors and financial losses.  
**Impact**: Improper configurations can disrupt tokenomics or allow attackers to exploit flaws in the contract's design.

### **1) Bedrock – Mint Function Inflation Bug**

**Vulnerability**: The `mint` function for Bedrock's `uniBTC` token allowed other assets to be minted at a 1:1 ratio, bypassing its intended restrictions.

**Impact**: Attackers exploited this flaw to inflate the token supply by 30x, causing a financial loss of $1.7 million.

**Funds Flow**: The attackers laundered 650.1 ETH through Tornado Cash and dispersed the funds to multiple addresses.

<details><summary>Code</summary>

```solidity
/**
 * @dev mint uniBTC with native BTC tokens
 */
function _mint(address _sender, uint256 _amount) internal {
    (,uint256 uniBTCAmount) = _amounts(_amount);
    require(uniBTCAmount > 0, "USR010");

    require(address(this).balance <= caps[NATIVE.BTC], MUSR003M);

    IMintableContract(uniBTC).mint(_sender, uniBTCAmount);

    emit Minted(NATIVE_BTC, _amount);
}
/**
 * @dev mint uniBTC with wrapped BTC tokens
 */
function _mint(address _sender, address _token, uint256 _amount) internal {
    (, uint256 uniBTCAmount) = _amounts(_token, _amount);
    require(uniBTCAmount > 0, "USR010");

    require(IERC20(_token).balanceOf(address(this)) + _amount <= caps[_token], "USR003");

    IERC20(_token).safeTransferFrom(_sender, address(this), _amount);
    IMintableContract(uniBTC).mint(_sender, uniBTCAmount);

    emit Minted(_token, _amount);
}

```

</details>

### **2) Spectral – Price Manipulation via AgentBalances**

**Vulnerability**: The `AgentBalances` contract mishandled the maximum approval for `AgentToken`, allowing attackers to manipulate its price.

**Impact**: By artificially inflating the token's price, attackers drained $250,000 worth of `SPEC` tokens from the bonding curve.

**Funds Flow**: The stolen funds were bridged to Ethereum through Orbiter Finance and laundered using Railgun, involving 41.74 ETH.

<details><summary>Code</summary>

```solidity

function deposit(address from, address token, address agent, uint256 amount)
public {
    if(agentWallets[agent] != address(0)){
        //Pipe it straight through if the agent's wallet is set
        IERC20Upgradeable(token).safeTransferFrom(from,
        agentWallets[agent],
        amount);
    }
    else
    {
        //Otherwise store it here for later
        IERC20Upgradeable(token).safeTransferFrom(from, address(this),
        amount);
        agentBalance[agent][token] += amount;
    }

    emit Deposit(agent, token, amount);

```

</details>

---

## **4. Price Dependency**

**Issue**: When token price calculations depend on a single or small number of liquidity pools, attackers can manipulate prices through flash loans or large trades.  
**Impact**: This results in distorted token prices, allowing attackers to profit via arbitrage.  
**Example**: An attacker manipulates the price in one pool, then exploits the distorted price in another contract that relies on this price logic.

### **1) HFLH Token**

- **Vulnerability**: The price calculation function for FLH Token was flawed. It incorrectly calculated the token price by dividing the balance of one token by another, leading to an exploitable vulnerability.

- **Impact**: Attackers donated tokens to the contract to manipulate the exchange rate, causing a price collapse and stealing funds from the pool. The total loss was approximately **$5,300**.

- **Funds Flow**: The stolen funds were transferred to a **Binance address**.

<details><summary>Code</summary>

```solidity

function getPrice() public view returns (uint256) {
    uint256 usdtBalance = IERC20(usdtAddress).balanceOf(lpAddress);
    uint256 HFLHBalance = IERC20(HFLHAddress).baLanceOf(lpAddress);

    require(usdtBalance > 0, “USDT balance is zero11);

    uint256 price = lel8*usdtBalance/ HFLHBalance;
    return price;
}

```

</details>

### **2) WOOFi**

- **Vulnerability**: WOOFi’s sPMM algorithm relied on its proprietary oracle to adjust slippage based on current token prices. However, the exchange rate calculation mechanism had a design flaw that failed to prevent flash loan attacks. This enabled attackers to manipulate the price of the WOO token, dropping it to an extremely low value.

- **Impact**: The `_calcQuoteAmountSellBase` function, responsible for calculating the exchange rate between BaseToken and QuoteToken, lacked proper slippage controls. Attackers exploited this by repeatedly swapping tokens, manipulating the price, and withdrawing approximately **$8.7 million** from the WOOFi contract, leaving it with substantial bad debt.

- **Funds Flow**: The stolen funds were distributed across multiple external accounts (EOAs), then bridged from Arbitrum to Ethereum via **LayerZero Stargate** and **Synapse**. Ultimately, the assets were laundered through **Tornado Cash**, totaling **2,120 ETH**.

<details><summary>Code</summary>

```solidity

function _calcQuoteAmountSellBase(
    address baseToken, uint256 baseAmount, IWooracleV2.State memory state) private view returns (uint256 quoteAmount, uint256 newPrice
) {
    require(state.woFeasible, "WooPPV2: !ORACLE_FEASIBLE");

    DecimalInfo memory decs = decimalInfo[baseToken]);
    // quoteAmount = baseAmount * oracle.price * (1 - oracle.k * baseAmount * oracle.price - oracle.spread)
    {
        uint256 coef = uint256(lel8) - ((uint256(state.coeff) * baseAmount * state.price) / decs.baseDec / decs.priceDec) - state.spread;
        quoteAmount = (((baseAmount * decs.quoteDec * state.price) / decs.priceDec) * coef) / 1e18 / decs.baseDec;
    }

    // newPrice = (1 - 2 * k * oracle.price * baseAmount) * oracle.price
    newPrice = ((uint256(1e18) - (uint256(2) * state.coeff * state.price * baseAmount) / decs.priceDec / decs.baseDec) * state.price) / 1e18;

```

</details>

---

## **5. Misimplemented Transfer Function**

**Issue**: Vulnerabilities in the `transfer` function of ERC20 tokens.  
**Impact**: Attackers could exploit these vulnerabilities, such as self-transfers, to double their token balances or cause unintended losses.

### **1) FIRE Token**

- **Vulnerability**: Shortly after the FIRE Token ($FIRE) was launched on Ethereum, attackers exploited a flaw in the token’s burn functionality embedded in its `transfer()` function.

- **Impact**: When FIRE tokens were sent to a Uniswap trading pair, the pool’s balance and reserves were automatically reduced due to the flawed "sync" mechanism. By sending large amounts of FIRE tokens to the pool, attackers reduced the reserves, swapped FIRE tokens for ETH at distorted rates, and drained the pool. The attacker repeated this process, ultimately stealing **$20,000** worth of ETH.

- **Funds Flow**: The stolen **9.1 ETH** was laundered through **Tornado Cash**.

<details><summary>Code</summary>

```solidity

// Deduct tokens from the liquidity pair and transfer to the dead address
uint256 sellAmount = amount.sub(taxAmount);
if (sellAmount > 0) {
    uint256 LiquidityPairBalance = balanceOf(uniswapV2Pair);
    if (liquidityPairBalance >= sellAmount) {
        _balances[uniswapV2Pair] = _balances[uniswapV2Pair].sub(sellAmount);
        _balances[DEAD_ADDRESS] = _balances[DEAD_ADDRESS].add(sellAmount);
        emit Transfer(uniswapV2Pair, DEAD_ADDRESS, sellAmount);

        // Call sync to update the pair
        IUniswapV2Pair(uniswapV2Pair).sync();
    }
}

```

</details>

### **2) Miner ERCX**

- **Vulnerability**: The $MINER token, created using the experimental **ERC404** standard, suffered from a critical flaw in its `_update` function. The caching mechanism for balances allowed attackers to double their token balances by sending tokens to their own address.

- **Impact**: This vulnerability caused the token’s price to crash by over **99%**, resulting in total losses of approximately **$435,000**.

- **Funds Flow**: After negotiations with the project team failed (a bounty of **$120,000** was offered but declined), the attacker laundered the stolen funds via **Tornado Cash**.

<details><summary>Code</summary>

```solidity

function _update(address from, address to, uint256 value, bool mint) internal virtual {
    uint256 fromBalance = _balances[from];
    uint256 toBalance = _balances[to];
    if (fromBalance < value) {
        revert ERC20InsufficientBalance(from, fromBalance, value);
    }

    unchecked {
        // Overflow not possible: value <= fromBalance <= totalsupply.
        _balances[from] = fromBalance - value;

        // Overflow not possible: balance + value is at most totalSupply, which we know fits into a uint256.
        _balances[to] = toBalance + value;

```

</details>

---

## **6. Precision Loss**

**Issue**: Rounding errors in certain variables were not properly handled.  
**Impact**: Precision loss could be exploited in DeFi projects like Compound V2 and Aave V2 forks, leading to unfair borrowing or trading opportunities.

### **1) Radiant Capital**

- **Vulnerability**: Radiant Capital, a multi-chain lending protocol based on an **Aave V2** fork, suffered from a precision error in its liquidity pool. When newly deployed pools were initialized with zero liquidity, the `aToken` (LP token) contract’s `totalSupply` was not properly initialized and defaulted to **0**. If an attacker was the first liquidity provider, they could artificially inflate the `liquidityIndex`. Additionally, the `rayDiv` function had rounding errors that amplified the precision loss when `aTokens` were minted or burned.

- **Impact**: Attackers exploited this within **5 seconds** of a new pool’s activation, manipulating the `aToken` supply and precision loss mechanisms to inflate their balances. This led to **$4.5 million** in bad debt for Radiant Capital.

- **Funds Flow**: The stolen funds were split across more than **50 EOA addresses** to obfuscate the trail. Using Hop Protocol, Synapse, and LayerZero Stargate, the funds were bridged from **Arbitrum to Ethereum** and laundered via **Tornado Cash**.

<details><summary>Code</summary>

```solidity

/**
* @dev Divides two ray, rounding half up to the nearest ray
* @param a Ray
* @param b Ray
* @return The result of a/b, in ray
**/

function rayDiv(uint256 a, uint256 b) internal pure returns (uint256) {
    require(b != 0, Errors.MATH_DIVISION_BY_ZERO);
    uint256 halfB = b / 2;

    require(a <= (type(uint256).max - halfB) / RAY, Errors.MATH_MULTIPLICATION_OVERFLOW);

    return (a * RAY + halfB) / b;
}

/**
* @dev Mints 'amount' aTokens to 'user'
* - Only callable by the LendingPool, as extra state updates there need to be managed
* @param user The address receiving the minted tokens
* @param amount The amount of tokens getting minted
* @param index The new liquidity index of the reserve
* @return 'true' if the the previous balance of the user was 0
*/
function mint(address user, uint256 amount, uint256 index) external override onlyLendingPool returns (bool) {
    uint256 previousBalance = super.balanceOf(user);

    uint256 amountScaled = amount.rayDiv( index);
    require(amountScaled != 0, Errors.CT_INVALID_MINT_AMOUNT);
    _mint(user, amountScaled);

    emit Transfer(address(0), user, amount);
    emit Mint(user, amount, index);

    return previousBalance == 0;
}

```

</details>

### **2) Abracadabra.money**

- **Vulnerability**: Abracadabra.money, a well-known DeFi protocol handling $MIM and $SPELL tokens, had a rounding error vulnerability in its **CauldronV4** contract. This issue caused incorrect debt calculations, enabling attackers to exploit two specific pools—**magicAPE** and **yvCrv3Crypto cauldrons**.

  - Attackers initiated a flash loan via **DegenBox** to borrow $MIM, deposited it into **BentoBox**, and used the `repayForAll` function to exploit the rounding error in the debt calculation.
  - The function set `totalBorrow.elastic` to 0, but left a non-zero value in `totalBorrow.base`, causing incorrect share price calculations. Attackers leveraged this to borrow more assets than they were entitled to.

- **Impact**: Attackers repeatedly borrowed and repaid small amounts, artificially inflating share prices. This allowed them to borrow excessive amounts of $MIM, leading to a **$6.4 million** loss for the protocol.

- **Funds Flow**: Over the course of **50 days**, the stolen assets were laundered through **Tornado Cash**, amounting to **2,737.5 ETH**.

<details><summary>Code</summary>

```solidity
/// @notice Used to auto repay everyone 'liabilities'.
/// Transfer MIM deposit to DegenBox for this Cauldron and increase the totalBorrow base or skim
/// all aim inside this contract
function repayForAll(uint128 amount, bool skim) public returns (uint256) {
    accrue();

    if (skim) {
        // ignore amount and take every mim in this contract since it could be taken by anyone, the next block.
        amount = uintl28(magicInternetMoney.balanceOf(address(this)));
        bentoBox.deposit(magicInternetMoney, address(this), address(this), amount
        0);
    } else {
        bentoBox.transfer(magicInternetMoney, msg.sender, address(this), bentoBox.toShare(magicInternetMoney, amount, true));
    }
    uint128 previousElastic = totalBorrow.elastic;

    require(previousElastic - amount > 1000 * 1e18, "Total Elastic too small");

    totalBorrow.elastic = previousElastic - amount;

    emit LogRepayForAll(amount, previousElastic, totalBorrow.elastic);
    return amount;

/// @notice Calculates the base value in relationship to 'elastic' and 'total'.
function toBase(Rebase memory total, uint256 elastic, bool roundUp) internal pure returns (uint256 base) {
    if (total.elastic == 0) {
        base = elastic;
    } else {
        base = (elastic * total.base) / total.elastic;
        if (roundUp && (base * total.elastic) / total.base < elastic) {
        base++;
        }
    }
}

/// @notice Calculates the elastic value in relationship to 'base' and 'total' .
function toElastic(Rebase memory total, uint256 base, bool roundUp) internal pure returns (uint256 elastic) {
    if (total.base = 0) {
        elastic = base;
    } else {
        elastic = (base * total.elastic) / total.base;
        if (roundUp && (elastic * total.base) / total.elastic < base) {
        elastic++;
        }
    }
}

```

</details>

---

## **7. Reentrancy Vulnerabilities**

**Issue**: Reentrancy occurs when a function is repeatedly called before its initial execution is complete, leading to unintended behaviors due to improper sequence or state updates.  
**Impact**: Attackers exploit this to repeatedly access and drain funds before the system can update its internal state.  
**Example**: The infamous DAO hack is a classic case of reentrancy exploitation.

### **1) Clober**

**Vulnerability**: Clober, a decentralized exchange (DEX) on Base, suffered from a reentrancy vulnerability in its **Rebalancer contract's `burn()` function**, which attackers leveraged to execute the exploit.

**Impact**: The attacker created a custom trading pool with their own tokens and exploited the reentrancy flaw in the `burn()` function to manipulate token prices. They extracted $501,000 through this process.

**Funds Flow**: The stolen funds were bridged to Ethereum via **Across Protocol** over 19 days and later laundered using **Tornado Cash**, with 154 ETH being cleaned.

<details><summary>Code</summary>

```solidity

function _burn(bytes32 key, address user, uint256 burnAmount)
    public
    selfOnly
    returns (uint256 withdrawalA, uint256 withdrawalB)
{
    Pool storage pool = _pools[key];
    uint256 supply = totalSupply[uint256(key)];

    (uint256 canceledAmountA, uint256 canceledAmountB, uint256 claimedAmountA, uint256 claimedAmountB)= _clearPool(key, pool, burnAnount, supply);

    uint256 reserveA = pool.reserveA;
    uint256 reserveB = pool.reserveB;

    withdrawalA = (reserveA + claimedAmountA) * burnAmount / supply + canceledAmountA;
    withdrawalB = (reserveB + claimedAmountB) * burnAmount / supply + canceledAmountB;

    _burn(user, uint256(key), burnAmount);
    pool.strategy.burnHook(msg.sender, key, burnAmount, supply);
    emit Burn(user, key, withdrawalA, withdrawalB, burnAmount);

    IBookManager.BookKey memory bookKeyA = bookManager.getBookKey(pool.bookIdA);

    pool.reserveA = _settleCurrency(bookKeyA.quote, reserveA) - withdrawalA;
    pool.reserveB = _settleCurrency(bookKeyA.base, reserveB) - withdrawalB;

    if (withdrawalA > 0) {
        bookKeyA.quote.transfer(user, withdrawalA);
    }
    if (withdrawalB > 0) {
        bookKeyA.base.transfer(user, withdrawalB);
    }
}

```

</details>

### **2) GemPad**

**Vulnerability**: GemPad, a BSC-based project, encountered a reentrancy issue in its **Lock Contract's `collectFee` function**, enabling attackers to steal project funds.

**Impact**: The attacker minted a fake token, locked it in the contract along with LP tokens, and used a callback mechanism to repeatedly invoke `collectFee`, ultimately draining the project's vault of $1.9 million.

**Funds Flow**: Within two days, the stolen funds were bridged to Ethereum via **LayerZero Stargate** and laundered through Tornado Cash, resulting in a total of 410.2 ETH being cleaned.

<details><summary>Code</summary>

```solidity

function collectFees(
    uint256 lockId
)external isLockOwner(lockId) validLockLPv3(lockId) returns (uint256 amount0, uint256 amount1) {
    Lock storage userLock = _locks[lockId];
    // set amount0Max and amount1Max to uint256.max to collect all fees
    // alternatively can set recipient to msg.sender and avoid another transaction in 'sendToOwner'
    INonfungiblePositionManager.CollectParams
        memory params = INonfungiblePositionManager.CollectParams({
            tokenId: userLock.nftId,
            recipient: address(this),
            amount0Max: type(uint128).max,
            amount1Max: type(uint128).max
        });
    // send collected feed back to owner
    (
        ,
        ,
        address token0
        address token1,
        ,
        ,
        ,
        ,
        ,
        ,
        ,
    ) = INonfungiblePositionManager(userLock.nftManager).positions(
            userLock.nftId
        );
    uint256 originalAmount0 = IERC20(token0).balanceOf(address(this));
    uint256 originalAmount1 = IERC20(token1).balanceOf(address(this));
    INonfungibLePositionManager(userLock.nftManager).collect(params);
    amount0 = IERC20(token0).balanceOf(address(this)) - originalAmount0;
    amount1 = IERC20(token1).balanceOf(address(this)) - originalAmount1;
    IERC20(token0).safeTransfer(userLock.owner, amount0);
    IERC20(token1).safeTransfer(userLock.owner, amount1);
}

```

</details>

---

## **8. Storage Collision**

**Issue**: Storage collision arises during contract upgrades when new contract variables accidentally overwrite the storage slots of the old contract, leading to logical inconsistencies.  
**Impact**: This could result in disrupted operations, misallocated funds, or exploitable vulnerabilities.

### **1) Pike Finance**

**Vulnerability**: Pike Finance's CCTP integration for USDC cross-chain functionality had critical logical flaws. While transferring USDC between chains, the attacker exploited insufficient validation to manipulate the recipient address and amount. A subsequent patch introduced another vulnerability, a **storage layout mismatch** due to the addition of `pause()` and `unpause()` functionalities. This allowed attackers to reinitialize the contract and transfer ownership to themselves.

**Impact**: The project suffered two consecutive attacks in April, resulting in nearly $2 million in losses.

**Funds Flow**: The stolen funds were laundered using **Tornado Cash** and **Railgun**.

### **2) DeltaPrime**

**Vulnerability**: DeltaPrime, which utilized the **Diamond Beacon Proxy** design, separated its logic (Implementation Contract) from its storage (PrimeAccount Contract). To prevent storage conflicts, it relied on **custom storage slots** initialized via the `init()` and `initialize()` functions. However, the `DiamondBeacon` incorrectly checked its own storage slot for initialization status rather than the PrimeAccount’s. Attackers exploited this flaw to reinitialize the contract and gain control over the PrimeAccount.

**Impact**: The attacker reinitialized the contract, bypassed safeguards like collateral repayment requirements, and ultimately stole $1 million.  
**Funds Flow**: The funds were bridged to Ethereum via **Orbiter Finance** and processed further through **CoinsPaid** and **Revolut**.

---

## **9. Logic Bugs**

**Issue**: Logic bugs occur when a contract’s business logic is implemented incorrectly, allowing unintended actions.  
**Impact**: This can lead to price calculation errors, unauthorized transactions, or bypassing of restrictions.

### **1) OpenLeverage**

**Vulnerability**: OpenLeverage’s **`liquidate` function** erroneously included collateral assets in liquidation calculations, which should have been based solely on borrowed amounts.

**Impact**: Attackers exploited this flaw by repaying minimal amounts to liquidate large positions, profiting excessively. This resulted in $236,000 in losses.

**Funds Flow**: The attacker stole **420.4 BNB** and laundered it through Tornado Cash.

<details><summary>Code</summary>

```solidity

    function liquidate(uint16 marketId, bool collateralIndex, address borrower) external override nonReentrant {
        controller.collLiquidateAllowed(marketId);
        // check collateral
        uint collateral = activeCollaterals(borrower][marketId][collateralIndex];
        checkCollateral(collateral);

        BorrowVars memory borrowVars = toBorrowVars(marketId, collateralIndex);
        LiquidateVars memory liquidateVars;
        liquidsteVars.borrowing = OPBorrowingLib.borrowCurrent(borrowVars.borrowPool, borrower);
        liquidateVars.collateralAmount = OPBorrowingLib.shareToAmount(collateral, borrowVars.collateralTotalShare, borrowVars.collateralTotalReserve);

        // check liquidable
        require(checkLiquidable(marketId, liquidateVars.collateralAmount, liquidateVars.borrowing, borrowVars.collateralToken, borrowVars.borrowToken), "BIH");
        // check msg.sender xOLE
        require(xOLE.balanceOf(msg.sender) >= liquidationConf.liquidatorXOLEHeId, "XNE") ;
        // compute liquidation collateral
        MarketConf storage marketConf = marketsConf[marketld];
        liquidateVars.liquidationAmount = liquidateVars.collateralAmount;
        liquidateVars.liquidationShare = collateral;

```

</details>

### **2) Hedgey Finance**

**Vulnerability**: Hedgey Finance’s **`cancelCampaign` function** failed to revoke token approvals after users withdrew locked tokens post-campaign. Attackers exploited this oversight to use **`transferFrom`** and steal funds from the lock contract.

**Impact**: Hedgey Finance, operating on Ethereum and Arbitrum, lost $44.7 million across multiple tokens, including $NOBL, $MASA, $USDC, and $BONUS.

**Funds Flow**: An MEV white-hat hacker intercepted part of the attack, recovering some funds. However, attackers managed to launder a significant portion through Tornado Cash and deposited some tokens into exchanges like Bybit.

<details><summary>Code</summary>

```solidity

function cancelCampaign(bytes16 campaignId) external nonReentrant {
    Campaign memory campaign = campaigns[campaignId];
    require(campaign.manager == msg.sender, '!Imanager');
    delete campaigns[campaignId];
    delete claimLockups[campaignId];
    TransferHelper.withdrawTokens(campaign.token, msg.sender, campaign.amount);
    emit CampaignCancelled(campaignId);
}

```

</details>

---

## **10. Backdoor Exploits**

**Issue**: Hidden backdoor functions deliberately introduced by developers or insiders.  
**Impact**: These backdoors allow developers or attackers to manipulate contracts under specific conditions.  
**Case Study**: The 2024 **Munchables** incident is a prime example of exploitation via a backdoor function.

- **Vulnerability**: The project relied on an **Upgradeable Proxy contract**, but the code was not publicly verified, sparking speculation that the exploit may have been an internal act. On-chain analysis revealed that four wallet addresses belonging to the development team were involved in the attack. Their identities were confirmed through associated GitHub accounts. There were also allegations that the attackers had ties to North Korean hacking groups, drawing significant attention to the case. Within the Blast community, discussions ensued about extreme measures, such as **blocking attacker transactions at the Sequencer layer** or even rolling back the chain state to comply with regulatory concerns. Ultimately, the community opted to block attacker transactions at the Sequencer level, avoiding a full chain rollback.

- **Impact**: Munchables, a GameFi project on the Blast blockchain, suffered a breach that resulted in the unauthorized transfer of a large amount of funds. The incident led to intense community debates and controversy. However, the attacker voluntarily returned all stolen assets, potentially to avoid further legal or chain-level retaliation.

- **Funds Flow**: All stolen funds were returned to the project’s treasury on the same day as the attack.

---

## **11. State Management Flaws**

**Issue**: Inconsistent or incomplete state updates.  
**Impact**: Poor state management can lead to discrepancies on-chain, affecting user operations such as staking, unlocking, or governance voting.  
**Case Study**: **Curvance**

- **Vulnerability**: The vulnerability stemmed from the `processExpiredLock` function, which manages user-locked funds after the lock period ends. Users could choose to either relock their funds (`relock = true`) or withdraw them. If a user opted to relock, the function was supposed to replace the expired lock with a new one containing an updated timestamp. However, the function instead **appended a new lock without removing the old one**, leading to an accumulation of locks in the user’s account. This flaw caused the system to overestimate a user’s voting power, as governance votes were calculated based on the total number of locks.

- **Impact**: Exploiting this flaw, malicious actors could artificially inflate their voting power by repeatedly calling the function to generate multiple locks. The discrepancy in lock calculations also disrupted related functionalities like the `combineAllLocks` function. This allowed attackers to manipulate governance decisions and interfere with the protocol’s operations.

<details><summary>Code</summary>

```solidity

function processExpiredLock(
	uint256 lockIndex,
	bool relock,
	bool continuousLock,
	RewardsData calldata rewardsData,
	bytes calldata params,
	uint256 aux
) external nonReentrant {
	Lock[] storage locks = userLocks[msg.sender];
	// Length is index + 1 so has to be less than array length.
	if (lockIndex >= locks.length) {
		_revert(_INVALID_LOCK_SELECTOR);
	}
	if (block.timestamp < locks[lockIndex].unlockTime && isShutdown != 2) {
		_revert(_INVALID_LOCK_SELECTOR);
	}
	// Claim any pending locker rewards.
	_claimRewards(msg.sender, rewardsData, params, aux);
	Lock memory lock = locks[lockIndex];

	uint256 amount = lock.amount;

	// If the locker is shutdown, do not allow them to relock,
	// we'd want them to exit locked positions.
	if (isShutdown == 2) {
		relock = false;
		// Update their points to reflect the removed lock
		_updateDataFromEarlyUnlock(msg.sender, amount, lock.unlockTime);
	}

	if (relock) {
		// Token points will be caught up by _claimRewards call so we can
		// treat this as a fresh lock and increment rewards again.
		_lock(msg.sender, amount, continuousLock);
	} else { // BUG? user points is not changed if wants to relock
		_burn(msg.sender, amount);
		_removeLock(locks, lockIndex);

		// Transfer the user the unlocked CVE
		SafeTransferLib.safeTransfer(cve, msg.sender, amount);

		emit Unlocked(msg.sender, amount);

		// Check whether the user has no remaining locks and reset their
		// index, that way if in the future they create a new lock,
		// they do not need to claim epochs they have no rewards for.
		if (locks.length == 0 && isShutdown != 2) {
			cveLocker.resetUserClaimIndex(msg.sender);
		}
	}
}

```

## </details>

## **12. Cross-Chain Logic Bugs**

**Issue**: Cross-chain interactions often involve transferring data or state between different blockchains. This process is prone to vulnerabilities.  
**Impact**: Cross-chain logic flaws typically affect **data consistency**, **message validation**, **permission management**, or **timing delays**, leading to issues like financial loss or governance failures.  
**Case Study**: A cross-chain governance vulnerability involving L1 Guardians and L2 proposal cancellations.

- **Vulnerability**: The L1 **Guardians’ multisig contract** was designed to cancel proposals on L2. However, discrepancies in the way the proposal description’s hash was calculated led to a critical flaw. On L1, the hash was generated using `keccak256(abi.encode(description))`, while on L2, it used `keccak256(bytes(description))`. This mismatch resulted in differing proposal hashes (or proposal IDs) between the two layers.

- **Impact**: When the L1 contract sent a request to cancel a proposal on L2 via the Mailbox contract, the incorrect hash caused the system to attempt to cancel a **nonexistent proposal**. Exploiting this flaw, attackers could submit malicious proposals on L2 and execute them without L1 being able to cancel them in time. This introduced a significant security risk to the protocol.

<details><summary>Code</summary>

```solidity



    /// @return proposalId The unique identifier for the L2 proposal in compatible format with L2 Governors.
    function hashL2Proposal(L2GovernorProposal memory _l2Proposal) public pure returns (uint256 proposalId) {
        proposalId = uint256(
            keccak256(
                abi.encode(
                    _l2Proposal.targets,
                    _l2Proposal.values,
                    _l2Proposal.calldatas,
                    keccak256(abi.encode(_l2Proposal.description))
                )
            )
        );
    }

      /**
     * @dev See {IGovernor-propose}. This function has opt-in frontrunning protection, described in {_isValidDescriptionForProposer}.
     */
    function propose(
        address[] memory targets,
        uint256[] memory values,
        bytes[] memory calldatas,
        string memory description
    ) public virtual override returns (uint256) {
        address proposer = _msgSender();
        require(_isValidDescriptionForProposer(proposer, description), "Governor: proposer restricted");

        uint256 currentTimepoint = clock();
        require(
            getVotes(proposer, currentTimepoint - 1) >= proposalThreshold(),
            "Governor: proposer votes below proposal threshold"
        );

        uint256 proposalId = hashProposal(targets, values, calldatas, keccak256(bytes(description)));

```

</details>

---

### **13. Multiple Entrypoint Contracts for Underlying Assets**

**Issue**: The failure to properly validate token addresses can result in multiple entrypoint contracts being exploited to bypass security checks, allowing manipulation of market exchange rates for profit.
**Impact:** Attackers can exploit this issue to manipulate exchange rates, causing market price fluctuations. Additionally, they may drain all underlying assets from a contract, leading to significant economic losses.

**Case Study: Compound Finance**

- **Vulnerability:** In Compound's implementation, the `sweepToken` function allows users to withdraw tokens accidentally sent to the contract. The function includes a check to ensure that the token being withdrawn is not the underlying asset (`underlying`). However, if the underlying asset has multiple entrypoint contracts (as in early proxy-based implementations), this check can be bypassed.

  - Tokens like TUSD utilize multiple entrypoint contracts that share the same balance information. The main contract is used for primary interactions such as `transfer` and `balanceOf`, while auxiliary entry contracts act as proxies forwarding calls to the main contract. The `sweepToken` function checks whether the token address matches the `underlying` address.
  - Attackers can pass the address of an auxiliary entry contract instead of the main contract, bypassing the check. As a result, the `sweepToken` function processes the transaction and transfers all underlying assets (e.g., TUSD) to the admin's address.

- **Impact:** The removal of underlying assets from the contract causes the exchange rate (e.g., TUSD/cTUSD) to plummet. Attackers can profit by liquidating other users at a reduced cost and repaying loans at a lower expense.

<details><summary>Code</summary>

```solidity

   function sweepToken(EIP20NonStandardInterface token) override external {
        require(address(token) != underlying, "CErc20::sweepToken: can not sweep underlying token");
        uint256 balance = token.balanceOf(address(this));
        token.transfer(admin, balance);
	 }

// Recommendaiton

    function sweepToken(EIP20NonStandardInterface token) override external {
        require(msg.sender == admin, "CErc20::sweepToken: only admin can sweep tokens");
        require(address(token) != underlying, "CErc20::sweepToken: can not sweep underlying token");
        uint256 balance = token.balanceOf(address(this));
        token.transfer(admin, balance);
    }

```

</details>

---

### **14. Cryptography Vulnerabilities**

**Issue**: While zero-knowledge cryptography offers robust privacy protections for blockchain technology, its implementation can introduce various vulnerabilities and attack vectors.
**Impact:** Common vulnerabilities include issues in proof system implementation, binding and hiding properties in commitment schemes, random number generation flaws, circuit design bugs, and key management weaknesses.

**Case Study: Groth16**

- **Vulnerability:** Groth16 is a zero-knowledge proof protocol often used to prove the correctness of certain statements without revealing details. A commitment scheme is used to bind proof variables and generate random challenges. In the gnark extension of Groth16, the commitment scheme is defined as:

  $D*i = \sum*{j \in J_i} a_j \cdot L_j$,

  where $a_j$ are private witness variables and $L_j$ are elliptic curve points assumed to satisfy the discrete logarithm problem (DLP).

  - **Binding Property:** The binding property ensures that a commitment cannot be re-bound to a different value once generated. Whether the commitment is binding depends on whether $L_j$ are independent and uniformly random elements of the elliptic curve. If linear dependencies exist between $L_j$ coefficients due to specific constraints, this dependency could compromise the binding property.

  - **Hiding Property:** The hiding property ensures that the commitment does not reveal private witness information. The gnark implementation lacks a blinding factor, which is common in schemes like Pedersen commitments. For example, Pedersen commitments use the form:

    $D*i = \sum*{j \in J_i} a_j \cdot L_j + d \cdot h$,

    where $d$ is a random blinding factor and $h$ is an independent base point. Without the blinding factor, attackers can guess $a_j$ values and verify their correctness through brute force.

- **Impact:** In early versions of gnark (e.g., prior to 0.11.0), the lack of hiding property could potentially lead to the leakage of private witness variables. While the practical risk is mitigated by the difficulty of the discrete logarithm problem, the theoretical weakness could compromise the protocol's integrity in certain scenarios.

<details><summary>Code</summary>

```solidity

diff --git a/backend/groth16/bn254/prove.go b/backend/groth16/bn254/prove.go
index 100f30e8..1cf93b96 100644
--- a/backend/groth16/bn254/prove.go
+++ b/backend/groth16/bn254/prove.go
@@ -60,7 +60,7 @@ func (proof *Proof) CurveID() ecc.ID {
 }

 // Prove generates the proof of knowledge of a r1cs with full witness (secret + public part).
-func Prove(r1cs *cs.R1CS, pk *ProvingKey, fullWitness witness.Witness, opts ...backend.ProverOption) (*Proof, error) {
+func Prove(r1cs *cs.R1CS, pk *ProvingKey, fullWitness witness.Witness, fakeCommitments []curve.G1Affine, opts ...backend.ProverOption) (*Proof, error) {
 	opt, err := backend.NewProverConfig(opts...)
 	if err != nil {
 		return nil, fmt.Errorf("new prover config: %w", err)
@@ -91,10 +91,7 @@ func Prove(r1cs *cs.R1CS, pk *ProvingKey, fullWitness witness.Witness, opts ...b
 			privateCommittedValues[i][j].SetBigInt(inJ)
 		}

-		var err error
-		if proof.Commitments[i], err = pk.CommitmentKeys[i].Commit(privateCommittedValues[i]); err != nil {
-			return err
-		}
+		proof.Commitments[i] = fakeCommitments[i]

 		opt.HashToFieldFn.Write(constraint.SerializeCommitment(proof.Commitments[i].Marshal(), hashed, (fr.Bits-1)/8+1))
 		hashBts := opt.HashToFieldFn.Sum(nil)
diff --git a/backend/groth16/groth16.go b/backend/groth16/groth16.go
index ca5b8bdc..36f3d459 100644
--- a/backend/groth16/groth16.go
+++ b/backend/groth16/groth16.go
@@ -179,7 +179,8 @@ func Prove(r1cs constraint.ConstraintSystem, pk ProvingKey, fullWitness witness.
 		if icicle_bn254.HasIcicle {
 			return icicle_bn254.Prove(_r1cs, pk.(*icicle_bn254.ProvingKey), fullWitness, opts...)
 		}
-		return groth16_bn254.Prove(_r1cs, pk.(*groth16_bn254.ProvingKey), fullWitness, opts...)
+		panic("changed stuff for poc")
+		//return groth16_bn254.Prove(_r1cs, pk.(*groth16_bn254.ProvingKey), fullWitness, opts...)

 	case *cs_bw6761.R1CS:
 		return groth16_bw6761.Prove(_r1cs, pk.(*groth16_bw6761.ProvingKey), fullWitness, opts...)

```

</details>

---

## **15. Ecosystem-Specific Vulnerabilities**

Different ecosystems exhibit vulnerabilities tied to their programming languages or frameworks. For example, Tact—a statically-typed language for the TON blockchain—requires developers to pay close attention to issues like **data serialization**, **concurrent processing**, **message rollback**, and **gas management**.

### **Data Serialization in Tact**

- **Issue**: Tact allows developers to explicitly define the serialization format of fields. Incorrect serialization can result in misinterpreted return data and potential issues during contract interaction.

- **Case Study:**  
  In an NFT project, the `index` field was serialized as `int257` (signed integer) by default, whereas the standard required `uint256` (unsigned integer). Additionally:

  - `totalAmount` was serialized as `coins`, ensuring it remained a positive value compatible with TON's format.
  - `releasedAmount` was serialized as `int257`, allowing it to be negative and consuming 257 bits of storage.

  This type mismatch led to failures in correctly parsing return values, disrupting contract interactions. For example, fields serialized differently than expected could cause calling contracts to roll back transactions.

- **Impact:** Serialization issues in Tact can lead to state inconsistencies, communication errors, and failed contract calls. Developers must explicitly define serialization types, such as `coins`, `int257`, or `uint256`, to ensure compatibility with TON's ecosystem. If no serialization type is specified, default values (like `int257`) are used, which can lead to unintended consequences.

<details><summary>Code</summary>

```tact

contract VestingWallet {
    totalAmount: Int as coins;
    releasedAmount: Int;
    owner: Address;
    master: Address;

```

```tact

message(0x8b771735) ReportStaticData {
    query_id: Int as uint64; // should be equal with request's
    index_id: Int; // numerical index of this NFT in the collection
    collection: Address; // collection to which this NFT belongs
    }

```

```tact

report_static_data#8b771735 query_id:uint64 index:uint256
collection:MsgAddress = InternalMsgBody

```

</details>

---

<div  align="center">
<img src="img/05_2024_contract_security.gif" width="50%" />
</div>
