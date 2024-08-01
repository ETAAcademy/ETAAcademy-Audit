# ETAAcademy-Adudit: 2. Block

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
          <th>EVM</th>
          <td>block</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] Timestamp Constraints Leading to Number of Blocks Creation Limitations

### Different timestamp constraints between batches and blocks

- Summary : The constraints on timestamp differences between batches and their respective blocks in zkSync lead to smaller batch sizes, and prohibits the simultaneous commitment of two batches on L1 within the same Ethereum block, causing bottlenecks during high transaction volumes and block space utilization.
- Impact & Recommendation: The current timestamp verification process on L1 and L2 exacerbates these issues, necessitating stricter constraints to prevent batches with future timestamps. Mitigation steps should involve applying stricter timestamp constraints on both L1 and L2.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-12-timestamp-constraints-leading-to-number-of-blocks-creation-limitations) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

    Batch 1000:



    Batch timestamp: X + COMMIT_TIMESTAMP_APPROXIMATION_DELTA - 1.

    Timestamp of the last block (fictive block) in this batch: X + COMMIT_TIMESTAMP_APPROXIMATION_DELTA.

    The time this batch is committed on L1: blockTimestamp1000.

    X <= blockTimestamp1000.



    Batch 1001:



    Batch timestamp: X + COMMIT_TIMESTAMP_APPROXIMATION_DELTA + Y.

    Timestamp of the last block (fictive block) in this batch: X + COMMIT_TIMESTAMP_APPROXIMATION_DELTA + Y + K.

    The time this batch is committed on L1: blockTimestamp1001.

  ```

  </details>

## 2. [High] Validity and contests bond ca be incorrectly burned for the correct and ultimately verified transition

### Verify transition

- Summary: Both validity and contest bonds can be erroneously slashed even if the transition is ultimately correct and verified. This occurs because the history of the final verified transition is not considered, leading to situations where participants lose their bonds unjustly. In such scenarios, ts.prover acts as the guardian and is responsible for the final proof of the block.

- Impact & Recommendation: It suggests to enable guardians to refund validity and contest bonds similar to liveness bonds, ensuring bond recovery if a prover or contester is proven correct. Additionally, rewards sent to guardians during proof verification should not be recovered to avoid locking funds in TaikoL1.

<br> 🐬: [Source](https://code4rena.com/reports/2024-03-taiko#h-02-validity-and-contests-bond-ca-be-incorrectly-burned-for-the-correct-and-ultimately-verified-transition) & [Report](https://code4rena.com/reports/2024-03-taiko)

  <details><summary>POC</summary>
 
  ```solidity
      function testProverLoss() external{
        giveEthAndTko(Alice, 1e7 ether, 1000 ether);
        giveEthAndTko(Carol, 1e7 ether, 1000 ether);
        giveEthAndTko(Bob, 1e6 ether, 100 ether);
        console2.log("Bob balance:", tko.balanceOf(Bob));
        uint256 bobBalanceBefore = tko.balanceOf(Bob);
        vm.prank(Bob, Bob);
        bytes32 parentHash = GENESIS_BLOCK_HASH;
        uint256 blockId = 1;
        
        (TaikoData.BlockMetadata memory meta,) = proposeBlock(Alice, Bob, 1_000_000, 1024);
        console2.log("Bob balance After propose:", tko.balanceOf(Bob));
        mine(1);
        bytes32 blockHash = bytes32(1e10 + blockId);
        bytes32 stateRoot = bytes32(1e9 + blockId);
        (, TaikoData.SlotB memory b) = L1.getStateVariables();
        uint64 lastVerifiedBlockBefore = b.lastVerifiedBlockId;
        // Bob proves transition T1 for parent P1
        proveBlock(Bob, Bob, meta, parentHash, blockHash, stateRoot, meta.minTier, "");
        console2.log("Bob balance After proof:", tko.balanceOf(Bob));
        uint16 minTier = meta.minTier;
        // Higher Tier contests by proving transition T2 for same parent P1
        proveHigherTierProof(meta, parentHash, bytes32(uint256(1)), bytes32(uint256(1)), minTier);
        // Guardian steps in to prove T1 is correct transition for parent P1
        proveBlock(
            David, David, meta, parentHash, blockHash, stateRoot, LibTiers.TIER_GUARDIAN, ""
        );
        vm.roll(block.number + 15 * 12);
        vm.warp(
            block.timestamp + tierProvider().getTier(LibTiers.TIER_GUARDIAN).cooldownWindow * 60
                + 1
        );
        vm.roll(block.number + 15 * 12);
        vm.warp(
            block.timestamp + tierProvider().getTier(LibTiers.TIER_GUARDIAN).cooldownWindow * 60
                + 1
        );
        // When the correct transition T1 is verified Bob does permantley loses his validitybond
        // even though it is the correct transition for the verified parent P1.
        verifyBlock(Carol, 1);
        parentHash = blockHash;
        (, b) = L1.getStateVariables();
        uint64 lastVerifiedBlockAfter = b.lastVerifiedBlockId;
        assertEq(lastVerifiedBlockAfter, lastVerifiedBlockBefore + 1 ); // Verification completed
        uint256 bobBalanceAfter = tko.balanceOf(Bob);
        assertLt(bobBalanceAfter, bobBalanceBefore);
        console2.log("Bob Loss:", bobBalanceBefore - bobBalanceAfter);
        console2.log("Bob Loss without couting livenessbond:", bobBalanceBefore - bobBalanceAfter - 1e18); // Liveness bond is 1 ETH in tests
    }
  ```
  </details>

## 3. [High] Taiko L1 - Proposer can maliciously cause loss of funds by forcing someone else to pay prover’s fee

### Pay prover’s fee

- Summary: The libProposing library lets a proposer set the person for assigned prover fees. Malicious actors can exploit this by setting the person to another user's address, forcing them to pay fees for block proposals made by the malicious actor. This can happen if a user's approval allowance for spending tokens exceeds the actual fee they intend to pay.

- Impact & Recommendation: To prevent malicious actors from forcing others to pay fees for block proposals, a simple fix is to ensure that the block proposer always remains the msg.sender.

<br> 🐬: [Source](https://code4rena.com/reports/2024-03-taiko#h-04-taiko-l1---proposer-can-maliciously-cause-loss-of-funds-by-forcing-someone-else-to-pay-provers-fee) & [Report](https://code4rena.com/reports/2024-03-taiko)

<details><summary>POC</summary> 
  
    ```solidity
  
            if (params.coinbase == address(0)) {
        params.coinbase = msg.sender;
    }

    // When a hook is called, all ether in this contract will be send to the hook.
    // If the ether sent to the hook is not used entirely, the hook shall send the Ether
    // back to this contract for the next hook to use.
    // Proposers shall choose use extra hooks wisely.
    IHook(params.hookCalls[i].hook).onBlockProposed{ value: address(this).balance }(
        blk, meta_, params.hookCalls[i].data
    );

    // The proposer irrevocably pays a fee to the assigned prover, either in
    // Ether or ERC20 tokens.
    if (assignment.feeToken == address(0)) {
        // Paying Ether
        _blk.assignedProver.sendEther(proverFee, MAX_GAS_PAYING_PROVER);
    } else {
        // Paying ERC20 tokens
        IERC20(assignment.feeToken).safeTransferFrom(
            _meta.coinbase, _blk.assignedProver, proverFee
        );
    }

    ```

</details>

## 3. [Medium] Taiko L1 - Proposer can maliciously cause loss of funds by forcing someone else to pay prover’s fee

### Proving tier

- Summary: The `getMinTier()` function determines the minimum proving tier required for proposing a block based on a random number `_rand`. If this number meets a specific condition, a more expensive tier is required; otherwise, a cheaper one suffices. However, since the random number can be predicted in advance, proposers may choose to wait for a cheaper tier, causing delays in transaction finalization.

- Impact & Recommendation: Consider using VRF like solutions to make `_rand` truly random.

<br> 🐬: [Source](https://code4rena.com/reports/2024-03-taiko#m-11-proposers-would-choose-to-avoid-higher-tier-by-exploiting-non-randomness-of-parameter-used-in-getmintier) & [Report](https://code4rena.com/reports/2024-03-taiko)

<details><summary>POC</summary> 
  
    ```solidity

        File: contracts/L1/tiers/MainnetTierProvider.sol
    66:               function getMinTier(uint256 _rand) public pure override returns (uint16) {
    67:                   // 0.1% require SGX + ZKVM; all others require SGX
    68: @--->             if (_rand % 1000 == 0) return LibTiers.TIER_SGX_ZKVM;
    69:                   else return LibTiers.TIER_SGX;
    70:               }

        File: contracts/L1/libs/LibProposing.sol
    199:                  // Following the Merge, the L1 mixHash incorporates the
    200:                  // prevrandao value from the beacon chain. Given the possibility
    201:                  // of multiple Taiko blocks being proposed within a single
    202:                  // Ethereum block, we choose to introduce a salt to this random
    203:                  // number as the L2 mixHash.
    204: @--->            meta_.difficulty = keccak256(abi.encodePacked(block.prevrandao, b.numBlocks, block.number));
    205:
    206:                  // Use the difficulty as a random number
    207:                  meta_.minTier = ITierProvider(_resolver.resolve("tier_provider", false)).getMinTier(
    208: @--->                uint256(meta_.difficulty)
    209:                  );


    ```

</details>

## 4. [Medium] Taiko SGX Attestation - Improper validation in certchain decoding

### Improper validation

- Summary: In Taiko's ZK proof setup, SGX provers leverage remote attestation via Automata's modular attestation layer. The process involves decoding the certificate chain, validating notBefore and notAfter tags. However, a flaw in the validation logic allows an attestor to pass any value for the notBefore tag, compromising the integrity of the attestation process. This oversight poses a significant risk to the security and trustworthiness of SGX provers within Taiko's setup, highlighting the importance of rectifying the validation issue promptly.

- Impact & Recommendation: Updating the condition to `(notBeforeTag != 0x17 && notBeforeTag != 0x18)` will prevent improper validation and mitigate unforeseen consequences.

<br> 🐬: [Source](https://code4rena.com/reports/2024-03-taiko#m-13-taiko-sgx-attestation---improper-validation-in-certchain-decoding) & [Report](https://code4rena.com/reports/2024-03-taiko)

<details><summary>POC</summary> 
  
    ```solidity
                {
            uint256 notBeforePtr = der.firstChildOf(tbsPtr);
            uint256 notAfterPtr = der.nextSiblingOf(notBeforePtr);
            bytes1 notBeforeTag = der[notBeforePtr.ixs()];
            bytes1 notAfterTag = der[notAfterPtr.ixs()];
            if (
                (notBeforeTag != 0x17 && notBeforeTag == 0x18)
                    || (notAfterTag != 0x17 && notAfterTag != 0x18)
            ) {
                return (false, cert);
            }
            cert.notBefore = X509DateUtils.toTimestamp(der.bytesAt(notBeforePtr));
            cert.notAfter = X509DateUtils.toTimestamp(der.bytesAt(notAfterPtr));
        }
                if (
                (notBeforeTag != 0x17 && notBeforeTag != 0x18)
                    || (notAfterTag != 0x17 && notAfterTag != 0x18)
            ) {
                return (false, cert);

    ```

</details>

## 5. [Medium] Improper adjustment of Lending Ledger configuration

### Improper configuration for both rewards and block time parameters

- Summary: The LendingLedger's method for adjusting time-related parameters and rewards is insecure, potentially leading to retroactive application of adjustments to markets not yet updated. While warnings exist in the code, they are deemed insufficient to prevent over- or under-estimations of rewards attributed to markets.

- Impact & Recommendation: Track and update all markets together when adjusting parameters or rewards in LendingLedger. This can be done by iterating through all markets to ensure simultaneous updates, either by tracking the total number of markets or by providing an input array of markets. Additionally, for setRewards function, restrict mutations to future epochs to prevent retroactive adjustments.

<br> 🐬: [Source](https://code4rena.com/reports/2024-03-neobase#m-04-improper-adjustment-of-lending-ledger-configuration) & [Report](https://code4rena.com/reports/2024-03-neobase)

<details><summary>POC</summary> 
  
    ```solidity
    function testInsecureRewardUpdate() public {
        setupStateBeforeClaim();
        // Based on `LendingLedger.t.sol::testClaimValidLenderOneBlock`, the reward of the `lender` should be `amountPerBlock - 1` at this time point
        vm.roll(BLOCK_EPOCH * 5 + 1);
        // We update the rewards of the epochs without updating the markets
        vm.prank(governance);
        uint256 newRewardPerBlock = amountPerBlock * 2;
        ledger.setRewards(fromEpoch, toEpoch, newRewardPerBlock);
        // We claim the `lender` rewards, should be `amountPerBlock` based on `LendingLedger.t.sol::testClaimValidLenderOneBlock`
        uint256 balanceBefore = address(lender).balance;
        vm.prank(lender);
        vm.roll(BLOCK_EPOCH * 5 + 1);
        ledger.claim(lendingMarket);
        uint256 balanceAfter = address(lender).balance;
        // Assertion will fail
        assertEq(balanceAfter - balanceBefore, amountPerBlock - 1);
    }
    ```
</details>

## 6. [Medium] Ineffective swap deadline for swapRCH()

### Deadline

- Summary: The `swapRCH()` function in the `FeeCollector` contract sets the swap deadline to `block.timestamp + 10 minutes`, making it ineffective because `block.timestamp` is only determined during transaction execution.

- Impact & Recommendation: Pass the swap deadline as a parameter to the `swapRCH()` function for an absolute deadline.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-sofa-pro-league#m-03-ineffective-swap-deadline-for-swaprch) & [Report](https://code4rena.com/reports/2024-05-sofa-pro-league)

  <details><summary>POC</summary>

  ```solidity
      function swapRCH(
        address token,
        uint256 minPrice,
        address[] calldata path
    ) external onlyOwner {
        // last element of path should be rch
        require(path.length <= 4, "Collector: path too long");
        require(path[path.length - 1] == rch, "Collector: invalid path");
        uint256 amountIn = IERC20(token).balanceOf(address(this));
        IUniswapV2Router(routerV2).swapExactTokensForTokens(
            amountIn,
            amountIn * minPrice / 1e18,
            path,
            address(this),
            block.timestamp + 10 minutes
        );
    }

  ```

  </details>

## 7. [Medium] Inconsistent sequencer unexpected delay in DelayBuffer may harm users calling forceInclusion()

### DelayBuffer of sequencer outage

- Summary: When the sequencer is down, users can call `SequencerInbox::forceInclusion()` to add messages, but the delay buffer reduction is inconsistent. If multiple messages are included at once, the buffer may not decrease, causing longer wait times. However, if messages are included sequentially, the buffer decreases correctly. The provided proof of concept demonstrates this issue.

- Impact & Recommendation: To avoid double counting in the delay buffer when the sequencer is offline, track total unexpected delay separately. Calculate it as block.number minus the maximum of the previous sequenced block number and the oldest delayed message not yet included.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-sofa-pro-league#m-03-ineffective-swap-deadline-for-swaprch) & [Report](https://code4rena.com/reports/2024-05-sofa-pro-league)

  <details><summary>POC</summary>

  ```solidity
    function test_POC_InconsistentBuffer_Decrease() public {
        bool fix = false;
        maxTimeVariation.delayBlocks = 2000;
        BufferConfig memory configBufferable = BufferConfig({
            threshold: 600, //60 * 60 * 2 / 12
            max: 14400, //24 * 60 * 60 / 12 * 2
            replenishRateInBasis: 714
        });
        (SequencerInbox seqInbox, Bridge bridge) = deployRollup(false, true, configBufferable);
        address delayedInboxSender = address(140);
        uint8 delayedInboxKind = 3;
        bytes32 messageDataHash = RAND.Bytes32();
        for (uint i = 0; i < 7; i++) {
            vm.startPrank(dummyInbox);
            bridge.enqueueDelayedMessage(delayedInboxKind, delayedInboxSender, messageDataHash);
            vm.roll(block.number + 1100);
            bridge.enqueueDelayedMessage(delayedInboxKind, delayedInboxSender, messageDataHash);
            vm.stopPrank();
            vm.roll(block.number + 2001);
            uint256 delayedMessagesRead = bridge.delayedMessageCount();
            if (fix) {
                seqInbox.forceInclusion(
                        delayedMessagesRead - 1,
                        delayedInboxKind,
                        [uint64(block.number - 3101), uint64(block.timestamp)],
                        0,
                        delayedInboxSender,
                        messageDataHash
                );
            }
            seqInbox.forceInclusion(
                    delayedMessagesRead,
                    delayedInboxKind,
                    [uint64(block.number - 2001), uint64(block.timestamp)],
                    0,
                    delayedInboxSender,
                    messageDataHash
            );
        }
        (uint256 bufferBlocks, ,,,,) = seqInbox.buffer();
        assertEq(bufferBlocks, fix ? 600 : 7320);
        vm.startPrank(dummyInbox);
        bridge.enqueueDelayedMessage(delayedInboxKind, delayedInboxSender, messageDataHash);
        vm.stopPrank();
        vm.roll(block.number + 601);
        uint256 delayedMessagesRead = bridge.delayedMessageCount();
        if (!fix) vm.expectRevert(ForceIncludeBlockTooSoon.selector);
        seqInbox.forceInclusion(
                delayedMessagesRead,
                delayedInboxKind,
                [uint64(block.number - 601), uint64(block.timestamp)],
                0,
                delayedInboxSender,
                messageDataHash
        );
    }

  ```

  </details>

## 8. [Medium] Reorgs may cause licenses to be sold at 0 price

### Polygon reorgs

- Summary: The `PlayFiLicenseSale` contract had a vulnerability where reorgs on the Polygon network could result in licenses being sold at a price of 0. This issue occurred because the price of the licenses (`tiers[1].price`) could be set in a later block than the user's claim transaction, which relies on different variables.

- Impact & Recommendation: Ensure a sufficient time interval between the price setting transaction and the status update transaction, or to add a check to revert if `tiers[1].price` is zero.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-playfi-proleague#m-01-Reorgs-may-cause-licenses-to-be-sold-at-0-price) & [Report](https://code4rena.com/reports/2024-06-playfi-proleague)

  <details><summary>POC</summary>

  ```solidity

      function claimLicenseEarlyAccess(uint256 amount, bytes calldata data, bytes32[] calldata merkleProof) public payable {
        if(!earlyAccessSaleActive) revert EarlyAccessSaleNotActive();
        (uint256 index, uint256 claimCap) = abi.decode(data,(uint256,uint256));
        uint256 claimedLicenses = earlyAccessClaimsPerAddress[msg.sender];
        if(amount + claimedLicenses > claimCap) revert IndividualClaimCapExceeded();
        bytes32 node = keccak256(abi.encodePacked(index, msg.sender, claimCap));
        if (!MerkleProof.verify(merkleProof, earlyAccessMerkleRoot, node)) revert InvalidProof();
        uint256 toPay = tiers[1].price * amount;

  ```

  </details>
