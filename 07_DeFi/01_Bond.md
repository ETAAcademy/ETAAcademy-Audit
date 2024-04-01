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

## 2.[Medium] LendingTerm.sol `_partialRepay()` A user cannot partial repay a loan with 0 interest

## Partial repay zero interest

- Summary: The problem arises from a requirement in the code that checks if `interestRepaid != 0`. This condition, meant to prevent small repayments, creates an issue when the loan has zero interest, making partial repayment impossible despite being feasible through `_repay()`.

- Impact & Recommendation: A possible solution would be to remove the `interestRepaid != 0` from the require in `_partialRepay()` .
  🐬: [Source](https://github.com/code-423n4/2023-12-ethereumcreditguild-findings/issues/756) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
      function testPartialRepayWithZeroInterestFail() public {
        LendingTerm term2 = LendingTerm(
            Clones.clone(address(new LendingTerm()))
        );
        term2.initialize(
            address(core),
            term.getReferences(),
            LendingTerm.LendingTermParams({
                collateralToken: address(collateral),
                maxDebtPerCollateralToken: _CREDIT_PER_COLLATERAL_TOKEN,
                interestRate: 0,
                maxDelayBetweenPartialRepay: _MAX_DELAY_BETWEEN_PARTIAL_REPAY,
                minPartialRepayPercent: _MIN_PARTIAL_REPAY_PERCENT,
                openingFee: 0,
                hardCap: _HARDCAP
            })
        );
        vm.label(address(term2), "term2");
        guild.addGauge(1, address(term2));
        guild.decrementGauge(address(term), _HARDCAP);
        guild.incrementGauge(address(term2), _HARDCAP);
        vm.startPrank(governor);
        core.grantRole(CoreRoles.RATE_LIMITED_CREDIT_MINTER, address(term2));
        core.grantRole(CoreRoles.GAUGE_PNL_NOTIFIER, address(term2));
        vm.stopPrank();
        // prepare & borrow
        uint256 borrowAmount = 20_000e18;
        uint256 collateralAmount = 12e18;
        collateral.mint(address(this), collateralAmount);
        collateral.approve(address(term2), collateralAmount);
        bytes32 loanId = term2.borrow(borrowAmount, collateralAmount);
        assertEq(term2.getLoan(loanId).collateralAmount, collateralAmount);
        vm.warp(block.timestamp + 10);
        vm.roll(block.number + 1);

        // check that the loan amount is the same as the initial borrow amount to ensure there are no accumulated interest
        assertEq(term2.getLoanDebt(loanId), 20_000e18);
        credit.mint(address(this), 10_000e18);
        credit.approve(address(term2), 10_000e18);
        vm.expectRevert("LendingTerm: repay too small");
        term2.partialRepay(loanId, 10_000e18);
    }

  ```

  </details>

## 3.[Low] If borrower becomes blacklisted for his collateral his loan needs to be forgiven in case to receive it

## Blacklisted collateral

- Summary: If a user's collateral is blacklisted, repay() will revert, requiring the Governor to initiate forgive() and manually transfer the tokens to the user, which may not be ideal if the user needs the tokens urgently.

- Impact & Recommendation: Consider adding a parameter to enable borrowers to designate another recipient for collateral when making a full repayment.
  🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)
