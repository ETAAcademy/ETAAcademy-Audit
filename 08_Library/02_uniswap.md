# ETAAcademy-Adudit: 2. Uniswap

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>02. Uniswap</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>library</th>
          <td>uniswap</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[High] `_getReferencePoolPriceX96()` will show incorrect price for negative tick deltas in current implementation cause it doesn’t round up for them

### Negative tickCumulativesDelta

- Summary: while the protocol accurately calculates tick deltas using `tickCumulativesDelta =  tickCumulatives[0] - tickCumulatives[1]`, it fails to round down ticks when the delta is negative, as it’s done in the [uniswap library](https://github.com/Uniswap/v3-periphery/blob/main/contracts/libraries/OracleLibrary.sol#L36). This oversight could lead to incorrect prices, reverting, unavailable `[_checkLoanIsHealthy()](https://github.com/code-423n4/2024-03-revert-lend/blob/457230945a49878eefdc1001796b10638c1e7584/src/V3Vault.sol#L702-L703)` , wrong tick and potential arbitrage opportunities.

- Impact & Recommendation: When tick deltas are negative, the protocol should rectify the rounding issue by adding `if (tickCumulatives[0] - tickCumulatives[1] < 0 && (tickCumulatives[0] - tickCumulatives[1]) % secondsAgo != 0) timeWeightedTick --;`**.**
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-revert-lend#h-05--_getreferencepoolpricex96-will-show-incorrect-price-for-negative-tick-deltas-in-current-implementation-cause-it-doesnt-round-up-for-them) & [Report](https://code4rena.com/reports/2024-03-revert-lend)

  <details><summary>POC</summary>

  ```solidity
      function _getReferencePoolPriceX96(IUniswapV3Pool pool, uint32 twapSeconds) internal view returns (uint256) {
        uint160 sqrtPriceX96;
        // if twap seconds set to 0 just use pool price
        if (twapSeconds == 0) {
            (sqrtPriceX96,,,,,,) = pool.slot0();
        } else {
            uint32[] memory secondsAgos = new uint32[](2);
            secondsAgos[0] = 0; // from (before)
            secondsAgos[1] = twapSeconds; // from (before)
            (int56[] memory tickCumulatives,) = pool.observe(secondsAgos); // pool observe may fail when there is not enough history available (only use pool with enough history!)
            //@audit
            int24 tick = int24((tickCumulatives[0] - tickCumulatives[1]) / int56(uint56(twapSeconds)));
            sqrtPriceX96 = TickMath.getSqrtRatioAtTick(tick);
        }
        return FullMath.mulDiv(sqrtPriceX96, sqrtPriceX96, Q96);
    }

  ```

  </details>
