# ETAAcademy-Adudit: 1. Bond

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>02. Block</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>DeFi</th>
          <td>bond</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Low] MinBorrow must be based on the market token

## MinBorrow

- Summary: In LendingTerm.sol, initializing minBorrow to 100e18 upon deployment poses an issue, especially with expensive assets like ETH or BTC.

- Impact & Recommendation: Set the minBorrow from the ProfitManager constructor to enhance contract versatility and eliminate the wait period for executing the setMinBorrow() function.
  🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
    constructor(address _core, uint minBorrow) CoreRef(_core) {
        emit MinBorrowUpdate(block.timestamp, 100e18);
    +       _minBorrow = minBorrow //should be carefully chosen by the contract deployer considering the price of collateral token
    }

    uint256 internal _minBorrow = 100e18;
    function minBorrow() external view returns (uint256) {
        return (_minBorrow * 1e18) / creditMultiplier;
    }

  ```

  </details>
