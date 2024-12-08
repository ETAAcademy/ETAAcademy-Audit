# ETAAcademy-Adudit: 1. Context

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. Context</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>context</th>
          <td>context</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [Medium] Lack of access to ETH on L2 through L1->L2 transactions

### msg.value

- Summary : Users are unable to access their ETH stored on L2 through L1->L2 transactions, because the msg.value is generated solely from the ETH on Layer 1, not from the active balance of the user's account on Layer 2.
- Impact & Recommendation: Users cannot access their ETH on Layer 2 to withdraw funds from the rollup before a scheduled malicious upgrade, if a malicious operator only processes L1->L2 transactions, effectively trapping their funds.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-09-lack-of-access-to-eth-on-l2-through-l1-l2-transactions) & [Report](https://code4rena.com/reports/2023-10-zksync)

## 2. [Medium] Vulnerabilities in Deposit Limit Enforcement and the Impact on Failed Deposits

### Deposit limit and track

- Summary: Users may struggle to claim failed deposits if a deposit limit is later imposed on a token, while malicious actors can exploit the system by intentionally failing deposits before limits are introduced, resetting their total deposited amount and exceeding caps once enforced.
- Impact & Recommendation: To mitigate these risks, the system should be updated to track deposited amounts regardless of existing limits, preventing difficulties in claiming failed deposits and thwarting attempts to bypass deposit restrictions.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-11-vulnerabilities-in-deposit-limit-enforcement-and-the-impact-on-failed-deposits) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity
  function _verifyDepositLimit(address _l1Token, address _depositor, uint256 _amount, bool _claiming) internal {
        IAllowList.Deposit memory limitData = IAllowList(allowList).getTokenDepositLimitData(_l1Token);
        if (_claiming) {
            totalDepositedAmountPerUser[_l1Token][_depositor] -= _amount;
        } else {
            totalDepositedAmountPerUser[_l1Token][_depositor] += _amount;
      if(limitData.depositLimitation){
               require(totalDepositedAmountPerUser[_l1Token][_depositor] <= limitData.depositCap, "d1");
            }
        }
    }

  ```

  </details>

## 3. [Medium] Synchronization Issue Between L1 and L2 Upgrades

### Protocol Version Discrepancy

- Summary: When an L2 upgrade fails but is executed on L1 without verifying its outcome, the protocol version advances despite the L2 system remaining unchanged, because the protocol mandates unique transaction hashes for L2 upgrades, with the nonce matching the new protocol version, causing a disparity between recorded and actual states.
- Impact & Recommendation:A potential solution involves integrating L2 upgrade outcomes into batch executions, allowing for a rollback of the protocol version if an upgrade fails. However, in cases involving both L1 and L2 components, directly reverting to a previous protocol version is challenging, as the L1 upgrade succeeds while the L2 counterpart encounters issues.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-16-synchronization-issue-between-l1-and-l2-upgrades) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

  function executeBatches(StoredBatchInfo[] calldata _batchesData) external nonReentrant onlyValidator {
        //...
        uint256 batchWhenUpgradeHappened = s.l2SystemContractsUpgradeBatchNumber;
        if (batchWhenUpgradeHappened != 0 && batchWhenUpgradeHappened <= newTotalBatchesExecuted) {
            delete s.l2SystemContractsUpgradeTxHash;
            delete s.l2SystemContractsUpgradeBatchNumber;
            if (!proveL1ToL2TransactionStatus(...)){ // checking the L2 upgrade tx was successful or not
               s.protocolVersion = s.OldProtocolVersion; // assuming the old protocol version is stored
            }
        }
    }

  ```

  </details>

## 4. [Medium] Repayers using EOA accounts can be affected if bad debt is generated when they are repaying loans

### Mint to repay loans in the same transaction

- Summary: The current protocol design forces EOA repayers to mint CreditTokens before repaying loans, causing issues if bad debt is generated and the creditMultiplier decreases between minting and repayment. This unfairly burdens repayers, who are already paying interest and fees. Bad debt should be covered by other mechanisms, not by repayers forced to mint additional tokens due to protocol design flaws.

- Impact & Recommendation: Allow repayers to mint the exact amount of CreditTokens needed to repay loans in the same transaction, protecting against bad debt. Alternatively, they can follow the current method. The LendingTerm will pull the required PeggedTokens for minting CreditTokens, handled by the PSM module.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild#m-08-repayers-using-eoa-accounts-can-be-affected-if-bad-debt-is-generated-when-they-are-repaying-loans) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity
    //@audit-issue => A repayer could compute how much CreditTokens are required to repay a loan by calling this function, the computed value will be based on the current value of the creditMultiplier
    //@audit-issue => The repayer would then go and mint the amount returned by this function before calling the `repay()` to finally repay his loan
    /// @notice outstanding borrowed amount of a loan, including interests
    function getLoanDebt(bytes32 loanId) public view returns (uint256) {
        ...
        // compute interest owed
        uint256 borrowAmount = loan.borrowAmount;
        uint256 interest = (borrowAmount *
            params.interestRate *
            (block.timestamp - borrowTime)) /
            YEAR /
            1e18;
        uint256 loanDebt = borrowAmount + interest;
        uint256 _openingFee = params.openingFee;
        if (_openingFee != 0) {
            loanDebt += (borrowAmount * _openingFee) / 1e18;
        }
        uint256 creditMultiplier = ProfitManager(refs.profitManager)
            .creditMultiplier();

        //@audit-info => The loanDebt is normalized using the current value of the `creditMultiplier`. loanDebt includes interests and fees accrued by the original borrowAmount
        loanDebt = (loanDebt * loan.borrowCreditMultiplier) / creditMultiplier;
        return loanDebt;
    }
    //@audit-issue => The problem when repaying the loan is if bad debt was generated in the system, now, the value of the `creditMultiplier` will be slightly lower than when the user queried the total amount of CreditTokens to be repaid by calling the `getLoanDebt()`
    function _repay(address repayer, bytes32 loanId) internal {
        ...
        ...
        ...
        // compute interest owed
        //@audit-issue => Now, when repaying the loan, the creditMultiplier will be different, thus, the computed value of the loanDebt will be greater than before, thus, more CreditTokens will be required to repay the same loan
        uint256 loanDebt = getLoanDebt(loanId);
        uint256 borrowAmount = loan.borrowAmount;
        uint256 creditMultiplier = ProfitManager(refs.profitManager)
            .creditMultiplier();
        uint256 principal = (borrowAmount * loan.borrowCreditMultiplier) /
            creditMultiplier;
        uint256 interest = loanDebt - principal;
        //@audit-issue => The amount of `loanDebt` CreditTokens are pulled from the repayer, this means, the repayer must have already minted the CreditTokens and it also granted enough allowance to the LendingTerm contract to spend on his behalf!
        /// pull debt from the borrower and replenish the buffer of available debt that can be minted.
        CreditToken(refs.creditToken).transferFrom(
            repayer,
            address(this),
            loanDebt
        );
        ...
        ...
        ...
    }


  ```

  </details>

## 5. [Medium] Missing unwrap configuration when withdrawing cross-chain in the depositYBLendSGLLockXchainTOLP() function of MagnetarAssetXChainModule results in being unable to lock and participate on the destination chain

### wrap & unwrap

- Summary: The depositYBLendSGLLockXchainTOLP() function deposits into Singularity and withdraws its tokens cross-chain, wrapping them as TOFT tokens. But it doesn't unwrap these tokens on the destination chain, causing issues for subsequent actions like acquiring YieldBox shares.

- Impact & Recommendation: Unwrapping is necessary for these actions to proceed. Update depositYBLendSGLLockXchainTOLP() to call `_withdrawToChain()` with the unwrap parameter set to true.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-tapioca#m-01-missing-unwrap-configuration-when-withdrawing-cross-chain-in-the-deposityblendsgllockxchaintolp-function-of-magnetarassetxchainmodule-results-in-being-unable-to-lock-and-participate-on-the-destination-chain) & [Report](https://code4rena.com/reports/2024-02-tapioca)

  <details><summary>POC</summary>

  ```solidity
    uint256 fraction =
        _depositYBLendSGL(data.depositData, data.singularity, IYieldBox(yieldBox), data.user, data.lendAmount);
    // wrap SGL receipt into tReceipt
    // ! User should approve `address(this)` for `IERC20(data.singularity)` !
    uint256 toftAmount = _wrapSglReceipt(IYieldBox(yieldBox), data.singularity, data.user, fraction, data.assetId);

    _withdrawToChain(
        MagnetarWithdrawData({
            yieldBox: yieldBox,
            assetId: data.assetId,
            unwrap: false,
            lzSendParams: data.lockAndParticipateSendParams.lzParams,
            sendGas: data.lockAndParticipateSendParams.lzSendGas,
            composeGas: data.lockAndParticipateSendParams.lzComposeGas,
            sendVal: data.lockAndParticipateSendParams.lzSendVal,
            composeVal: data.lockAndParticipateSendParams.lzComposeVal,
            composeMsg: data.lockAndParticipateSendParams.lzParams.sendParam.composeMsg,
            composeMsgType: data.lockAndParticipateSendParams.lzComposeMsgType,
            withdraw: true
        })
    );

  ```

  </details>

## 6. [Medium] Freezed Chain will never be unfreeze since StateTransitionManager::unfreezeChain is calling freezeDiamond instead of unfreezeDiamond

### FreezeChain and unfreezeChain

- Summary : In the StateTransitionManager.sol contract, both freezeChain and unfreezeChain functions mistakenly call freezeDiamond, effectively freezing the chain regardless of the function called. The comment in unfreezeChain inaccurately describes its action. Furthermore, unfreezeChain fails to properly unfreeze the chain by not invoking unfreezeDiamond, leaving the chain frozen even after calling unfreezeChain.

- Impact & Recommendation: In unfreezeChain, replace freezeDiamond with unfreezeDiamond for the IZkSyncStateTransition instance. Also, correct the comment to accurately describe the function's action.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-zksync#m-01-freezed-chain-will-never-be-unfreeze-since-statetransitionmanagerunfreezechain-is-calling-freezediamond-instead-of-unfreezediamond) & [Report](https://code4rena.com/reports/2024-03-zksync)

<details><summary>POC</summary>

```solidity
File:  code/contracts/ethereum/contracts/state-transition/StateTransitionManager.sol
- 164:    /// @dev freezes the specified chain
+ 164:    /// @dev unfreezes the specified chain
165:    function unfreezeChain(uint256 _chainId) external onlyOwner {
- 166:        IZkSyncStateTransition(stateTransition[_chainId]).freezeDiamond();
+ 166:        IZkSyncStateTransition(stateTransition[_chainId]).unfreezeDiamond();
          }

```

</details>

## 7.[Medium] L2SharedBridge l1LegacyBridge is not set

### `l1LegacyBridge` initialization

- Summary: In the `L2SharedBridge` contract, the `l1LegacyBridge` is not set during initialization, leaving it as `address(0)`. This omission means that any messages from the old `L1ERC20Bridge` sent before the upgrade of `L1ERC20Bridge` will fail during the `finalizeDeposit()` validation.

- Impact & Recommendation: Fix this by setting `l1LegacyBridge` during initialization. Although deposits will fail, users can still call `claimFailedDeposit` to recover funds.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-zksync#m-02-l2sharedbridge-l1legacybridge-is-not-set) & [Report](https://code4rena.com/reports/2024-03-zksync)

  <details><summary>POC</summary>

  ```solidity
        function initialize(
            address _l1Bridge,
            address _l1LegecyBridge,
            bytes32 _l2TokenProxyBytecodeHash,
            address _aliasedOwner
        ) external reinitializer(2) {
    ...
            if (block.chainid != ERA_CHAIN_ID) {
                address l2StandardToken = address(new L2StandardERC20{salt: bytes32(0)}());
                l2TokenBeacon = new UpgradeableBeacon{salt: bytes32(0)}(l2StandardToken);
                l2TokenBeacon.transferOwnership(_aliasedOwner);
            } else {
                require(_l1LegecyBridge != address(0), "bf2");
    +           l1LegacyBridge = _l1LegecyBridge
                // l2StandardToken and l2TokenBeacon are already deployed on ERA, and stored in the proxy
            }
        }

  ```

  </details>

## 8.[Medium] Liquidating positions with bounded Kerosen could be unprofitable for liquidators

### Break liquidation logic

- Summary: Liquidators are not rewarded with `Kerosene` tokens because only assets from the `vaults` mapping are moved to liquidators during liquidation, leaving `Kerosene` tokens in the liquidated Note. This results in liquidators receiving less than expected, potentially incurring losses.

- Impact & Recommendation: To fix this, the `vaultsKerosene` mapping should also be included as a source of assets in the `liquidate` function. The proposed change adds code to transfer assets from `vaultsKerosene` to the liquidator, ensuring they receive the full expected collateral.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-dyad#m-04-liquidating-positions-with-bounded-kerosen-could-be-unprofitable-for-liquidators) & [Report](https://code4rena.com/reports/2024-04-dyad)

  <details><summary>POC</summary>

  ```solidity
    function liquidate(uint id, uint to) external isValidDNft(id) isValidDNft(to) {
      uint cr = collatRatio(id);
      if (cr >= MIN_COLLATERIZATION_RATIO) revert CrTooHigh();
      dyad.burn(id, msg.sender, dyad.mintedDyad(address(this), id));

      uint cappedCr = cr < 1e18 ? 1e18 : cr;
      uint liquidationEquityShare = (cappedCr - 1e18).mulWadDown(LIQUIDATION_REWARD);
      uint liquidationAssetShare = (liquidationEquityShare + 1e18).divWadDown(cappedCr);

      uint numberOfVaults = vaults[id].length();
      for (uint i = 0; i < numberOfVaults; i++) {
          Vault vault = Vault(vaults[id].at(i));
          uint collateral = vault.id2asset(id).mulWadUp(liquidationAssetShare);
          vault.move(id, to, collateral);
      }
      emit Liquidate(id, msg.sender, to);
  }

  ```

  </details>

## 9.[High] Edge from dishonest challenge edge tree can inherit timer from honest tree allowing confirmation of incorrect assertion

### Inadequate checks

- The vulnerability in `checkClaimIdLink` allows an edge to inherit timers from its rival's children due to inadequate checks. This flaw can be exploited to inflate an edge's timer, enabling near-instant confirmation of any level 0 edge by repeatedly using a proved proof node and its ancestors or rivals. This occurs because only the originId and mutualId match is checked, allowing edges to inherit timers they shouldn't.

- Impact & Recommendation: Allow child edges to inherit the claimId of their parent and ensure the claiming edge's claimId matches the edgeId of the inheriting edge.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-arbitrum-foundation#m-01-inconsistent-sequencer-unexpected-delay-in-delaybuffer-may-harm-users-calling-forceinclusion) & [Report](https://code4rena.com/reports/2024-05-arbitrum-foundation)

<details><summary>POC</summary>

```solidity
    function checkClaimIdLink(EdgeStore storage store, bytes32 edgeId, bytes32 claimingEdgeId, uint8 numBigStepLevel)
        private
        view
    {
        // the origin id of an edge should be the mutual id of the edge in the level below
        if (store.edges[edgeId].mutualId() != store.edges[claimingEdgeId].originId) {
            revert OriginIdMutualIdMismatch(store.edges[edgeId].mutualId(), store.edges[claimingEdgeId].originId);
        }
        // the claiming edge must be exactly one level below
        if (nextEdgeLevel(store.edges[edgeId].level, numBigStepLevel) != store.edges[claimingEdgeId].level) {
            revert EdgeLevelInvalid(
                edgeId,
                claimingEdgeId,
                nextEdgeLevel(store.edges[edgeId].level, numBigStepLevel),
                store.edges[claimingEdgeId].level
            );
        }
    }

```

</details>

## 10. [High] Single plot can be occupied by multiple renters

### Plot management and reward attribution

- Summary: LandManager contract allows a token to be transferred to a new plot without updating the `plotId` in the `ToilerState` struct, causing the contract to incorrectly assume the token is still in the original plot. This flaw can lead to issues such as plots being misidentified as occupied or unoccupied, incorrect reward calculations, and potential staking of tokens in plots that should be vacant, especially if the total number of plots is reduced.

- Impact & Recommendation: Update the `plotId` field in the `ToilerState` to reflect the new plot to ensure accurate plot management and reward attribution.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-07-munchables#h-01-Single-plot-can-be-occupied-by-multiple-renters) & [Report](https://code4rena.com/reports/2024-07-munchables)

<details><summary>POC</summary>

```solidity
it("successful path with logging plotId", async () => {
  //@audit-note Log plotId before transfer
  const beforeState =
    await testContracts.landManagerProxy.contract.read.getToilerState([1]);
  console.log("PlotId before transfer: " + beforeState.plotId);
  const { request } =
    await testContracts.landManagerProxy.contract.simulate.transferToUnoccupiedPlot(
      [1, 10],
      {
        account: bob,
      },
    );
  const txHash = await testClient.writeContract(request);
  await assertTxSuccess({ txHash: txHash });
  //@audit-note Log plotId after transfer
  const afterState =
    await testContracts.landManagerProxy.contract.read.getToilerState([1]);
  console.log("PlotId after transfer: " + afterState.plotId);
  //@audit-note Assert plotId hasn't changed
  assert.equal(beforeState.plotId, afterState.plotId, "PlotId did not change");
});

```

</details>

## 11. [Medium] MsgSwapOrder will never work for Canto nodes

### Identify the signer

- Summary: In the `MsgSwapOrder` for Canto nodes, the message lacks the required `cosmos.msg.v1.signer` to identify the signer, making the message ineffective. This issue prevents the `MsgSwapOrder` from functioning correctly.

- Impact & Recommendation: Add a `DefineCustomGetSigners` call in `app.go`, similar to what was done for `MsgConvertERC20`.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-canto#m-02-MsgSwapOrder-will-never-work-for-Canto-nodes) & [Report](https://code4rena.com/reports/2024-05-canto)

<details><summary>POC</summary>

```go

signingOptions.DefineCustomGetSigners(protov2.MessageName(&erc20v1.MsgConvertERC20{}), erc20types.GetSignersFromMsgConvertERC20V2)

```

</details>
