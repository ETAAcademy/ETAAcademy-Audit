# ETAAcademy-Adudit: 3. Transaction

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>03. Transaction</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>EVM</th>
          <td>transaction</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] Discrepancy in ECRECOVER Precompile when Using Delegatecall

### Delegatecall to precompile

- Summary: When the ECRECOVER precompile contract is invoked using delegatecall, it does not behave consistently with other zkSync Era operations like call and staticcall.
- Impact & Recommendation: This divergence from expected EVM behavior can lead to incorrect signature validation, potentially compromising data integrity and user funds. If the **`_address`** matches the ECRECOVER precompile contract (0x01), it's recommended to perform a static call (**`rawStaticCall`**) instead of a delegate call to ensure consistent behavior with the ECRECOVER contract.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-17-discrepancy-in-ecrecover-precompile-when-using-delegatecall) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

    // SPDX-License-Identifier: MIT
    pragma solidity >=0.8.20;
    contract PoC {
        bytes32 h = keccak256("");
        uint8 v = 27;
        bytes32 r = bytes32(uint256(1));
        bytes32 s = bytes32(uint256(2));
        function ecrecoverStaticcall() public returns (bytes32) {
            bytes memory data = abi.encode(h, v, r, s);
            assembly {
                pop(staticcall(gas(), 0x01, add(data, 0x20), mload(data), 0, 0x20))
                return(0, 0x20)
            }
        }
        function ecrecoverCall() public returns (bytes32) {
            bytes memory data = abi.encode(h, v, r, s);
            assembly {
                pop(call(gas(), 0x01, 0x00, add(data, 0x20), mload(data), 0, 0x20))
                return(0, 0x20)
            }
        }
        function ecrecoverDelegatecall() public returns (bytes32) {
            bytes memory data = abi.encode(h, v, r, s);
            assembly {
                pop(
                    delegatecall(gas(), 0x01, add(data, 0x20), mload(data), 0, 0x20)
                )
                return(0, 0x20)
            }
        }
    }

  ```

## 2.[Medium] Discrepancy in Default Account Behavior

### Reverts in fallback function

- Summary: Default accounts mimic externally owned accounts (EOAs) on Ethereum, but a discrepancy arises when custom accounts delegate-calls to them, triggering a revert due to an `assert(msg.sender != BOOTLOADER_FORMAL_ADDRESS)` in the default account's fallback function.
- Impact & Recommendation: One proposed solution is to add a modifier called **`ignoreInDelegateCall`** to the fallback function of default accounts, which prevents the assertion check from executing when the fallback function is invoked via a delegate call.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-18-discrepancy-in-default-account-behavior) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity
    fallback() external payable {
        // The fallback function of the default account should never be called by the bootloader
        assert(msg.sender != BOOTLOADER_FORMAL_ADDRESS);
        // If the contract is called directly, it should behave like an EOA.
    }
    receive() external payable {
        // If the contract is called directly, it should behave like an EOA.
    }

    function _execute(Transaction calldata _transaction) internal {
        address to = address(uint160(_transaction.to));
        (bool success,) = address(to).delegatecall("0x1234");
        require(success, "call was not successful");
    }

  ```

  </details>

## 3.[Medium] Nonce Behavior Discrepancy Between zkSync Era and EIP-161

### Create & nonce

- Summary: The **`CREATE3`** library facilitates EVM contract creation similar to **`CREATE2`**, but it excludes the contract **`initCode`** from the address derivation formula. It involves deploying a new proxy contract using the **`CREATE2`** method, which then deploys the child contract using **`CREATE`**. The child contract's address is computed based on the proxy contract's address and its hardcoded nonce `**hex"01”**` ,which aligns with EIP-161. However, in the zkSync Era, where the nonce does not increment by one as expected, this mechanism unexpectedly fails compared to the EVM.
- Impact & Recommendation: It is recommended to increase the deployment nonce of a contract by one before calling its constructor.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-20-nonce-behavior-discrepancy-between-zksync-era-and-eip-161) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <br> 🐬: Others

  - [Medium] Deployment Nonce Does not Increment For a Reverted Child Contract: [Source](https://code4rena.com/reports/2023-10-zksync#m-21-deployment-nonce-does-not-increment-for-a-reverted-child-contract) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

  function _constructContract(
        address _sender,
        address _newAddress,
        bytes32 _bytecodeHash,
        bytes calldata _input,
        bool _isSystem,
        bool _callConstructor
    ) internal {
        NONCE_HOLDER_SYSTEM_CONTRACT.incrementDeploymentNonce(_newAddress);
        //...
    }

  ```

  </details>

## 4.[Medium] Permit doesn’t work with DAI

### DAI permit

- Summary: The issue arises from using the depositWithPermit function in PrizeVault.sol with permit options, intending to utilize sDAI but encountering discrepancies with DAI's permit signature. The problem stems from the missing nonce field in DAI's permit function, causing permit transactions to revert due to incorrect parameters.

- Impact & Recommendation: For the special case of DAI token, allow a different implementation of the permit function which allows a nonce variable.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-pooltogether#m-08-permit-doesnt-work-with-dai) & [Report](https://code4rena.com/reports/2024-03-pooltogether)

  <details><summary>POC</summary>

  ```solidity
    IERC20Permit(address(_asset)).permit(_owner, address(this), _assets, _deadline, _v, _r, _s);

    function permit(address holder, address spender, uint256 nonce, uint256 expiry,
                        bool allowed, uint8 v, bytes32 r, bytes32 s) external


  ```

  </details>

## 5.[Medium] Reorg attack on user’s Vault deployment and deposit may lead to theft of funds

### Create & chain reorgs

- Summary: Attacks exploiting chain reorganizations can steal deployed Vaults and deposits. By front-running a user's Vault deployment, including their deposit, and then withdrawing the funds, attackers can exploit vulnerabilities in the deployment process. This medium-severity issue poses a high impact but is less likely to occur.

- Impact & Recommendation: Vault instances should use create2 with a salt based on id, minter, and msg.sender for deployment.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-07-amphora#m-02-reorg-attack-on-users-vault-deployment-and-deposit-may-lead-to-theft-of-funds) & [Report](https://code4rena.com/reports/2023-07-amphora)

  <details><summary>POC</summary>

  ```solidity
  /// @notice Deploys a new Vault
  /// @param _id The id of the vault
  /// @param _minter The address of the minter of the vault
  /// @return _vault The vault that was created
  function deployVault(uint96 _id, address _minter) external returns (IVault _vault) {
    _vault = IVault(new Vault(_id, _minter, msg.sender, CVX, CRV));
  }

  ```

  </details>

## 6.[Medium] Reorg attack on user’s Vault deployment and deposit may lead to theft of funds

### src == msg.sender

- Summary: In mimswap's Router.sol file, the createPoolETH method wraps native tokens to their "wrapped" counterpart before sending them to a new pool. However, on chains like Blast, Wrapped Arbitrum, and Wrapped Fantom, using address(weth).safeTransferFrom causes approval issues due to differences in WETH implementations that lacks this src == msg.sender handling.

- Impact & Recommendation: To fix the issue preventing the creation of native token pools on multiple chains like Blast due to Router contract's failure to approve spending WETH tokens, modify Router.sol by replacing `safeTransferFrom` with `safeTransfer`.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-abracadabra-money#m-01-pool-creation-failure-due-to-weth-transfer-compatibility-issue-on-some-chains) & [Report](https://code4rena.com/reports/2024-03-abracadabra-money)

<details><summary>POC</summary>

```solidity
  pragma solidity ^0.8.0;
  import "forge-std/Test.sol";
  import "forge-std/console.sol";
  import {IERC20} from "forge-std/interfaces/IERC20.sol";
  contract PairTest is Test {
      address alice = address(0xf683Ce59521AA464066783d78e40CD9412f33D21);
      address bob = address(0x2);
      // WETH address on Blast network
      IERC20 public constant WETH = IERC20(0x4300000000000000000000000000000000000004);
      error InsufficientAllowance();
      function testPoC_TransferFromRevert() public {
          // stdstore write for packed slot is complex so we use a real address that has tokens in blaset main net weth
          // if this fails we need to update alice address to an address that has more than 1 ether balance in weth blast main net
          assert(WETH.balanceOf(alice) > 1 ether);
          vm.startPrank(alice);
          vm.expectRevert(InsufficientAllowance.selector);
          WETH.transferFrom(alice, bob, 1 ether);
          vm.stopPrank();
      }
}

```

</details>

## 7.[High] Native gas tokens can become stuck in ASDRouter contract

### msg.value stucked

- Summary: Excess gas sent to the ASDRouter contract gets stuck there after successful ASD token transfers, which is not refunded to the sender's address, but held in the ASDRouter contract. This violates the rule that the ASDRouter's native balance should always be zero.

- Impact & Recommendation: The suggested changes to the `_sendASD()` method in the ASDRouter contract will refund any leftover ether (gas) back to the specified refund address, preventing it from getting stuck in the contract after successful ASD token transfers.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-canto#h-01-native-gas-tokens-can-become-stuck-in-asdrouter-contract) & [Report](https://code4rena.com/reports/2024-03-canto)

<details><summary>POC</summary>

```solidity
    diff --git a/test/ASDRouter.js b/test/ASDRouter.js
    index 2a36337..eccedc0 100644
    --- a/test/ASDRouter.js
    +++ b/test/ASDRouter.js
    @@ -276,6 +276,7 @@ describe("ASDRouter", function () {
        it("lzCompose: successful deposit and send on canto", async () => {
            // update whitelist
            await ASDUSDC.updateWhitelist(USDCOFT.target, true);
    +        const gas = ethers.parseEther("1");
            // call lzCompose with valid payload
            await expect(
                ASDRouter.lzCompose(
    @@ -287,12 +288,18 @@ describe("ASDRouter", function () {
                        generatedRouterPayload(cantoLzEndpoint.id, refundAddress, TESTASD.target, TESTASD.target, "0", refundAddress, "0")
                    ),
                    executorAddress,
    -                "0x"
    +                "0x",
    +                { value: gas }
                )
            )
                .to.emit(ASDRouter, "ASDSent")
                .withArgs(guid, refundAddress, TESTASD.target, amountUSDCSent, cantoLzEndpoint.id, false);
            // expect ASD to be sent to canto
            expect(await TESTASD.balanceOf(refundAddress)).to.equal(amountUSDCSent);
    +
    +        // expect gas to be refunded and not held in ASDRouter
    +        expect(await ethers.provider.getBalance(ASDRouter.target)).to.equal(0);
    +        expect(await ethers.provider.getBalance(refundAddress)).to.equal(gas);
    +
        });
    });

```

</details>

## 8. Same contract multi permits fundamentally cannot be solved via the chosen standards

### MEV by permit signatures

- Summary: Cross-chain USDO and TOFT flows using approvals may be vulnerable to DoS attacks through permit-based griefing. Attackers can exploit front-run exploits by monitoring permit signatures in the mempool and executing them before intended transactions, rendering transactions ineffective. This limits Tapioca's architecture to single signature without revokes.
- Impact & Recommendation: Use Permits for granting approvals(with try-catch), avoiding their use for revoking approvals to prevent front-run exploits. Suggest granting higher allowances and implementing renounceAllowance for TOFT tokens to enforce a secure allowance pattern.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-tapioca#m-25-same-contract-multi-permits-fundamentally-cannot-be-solved-via-the-chosen-standards) & [Report](https://code4rena.com/reports/2024-02-tapioca)

## 9.[Medium] The signatures are replayable

### No Nonce and check

- Summary: User-signed orders lack a nonce, making signatures replayable and allowing the same order to be reused multiple times. The system only verifies the user's signature without ensuring the signed order matches the parameters used in execution, which can lead to unauthorized actions and potential fund loss.

- Impact & Recommendation: The recommended fix is to add a nonce and verify that operator parameters align with the user's signed order.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-krystal-defi#m-02-the-signatures-are-replayable) & [Report](https://code4rena.com/reports/2024-06-krystal-defi)

<details><summary>POC</summary>

```solidity

@>  StructHash.Order emptyUserConfig; // todo: remove this when we fill user configuration
    function setUp() external {
        _setupBase();
    }
    function testAutoAdjustRange() external {
        // add liquidity to existing (empty) position (add 1 DAI / 0 USDC)
        _increaseLiquidity();
        (address userAddress, uint256 privateKey) = makeAddrAndKey("positionOwnerAddress");
        vm.startPrank(TEST_NFT_ACCOUNT);
        NPM.safeTransferFrom(TEST_NFT_ACCOUNT, userAddress, TEST_NFT);
        vm.stopPrank();
@>      bytes memory signature = _signOrder(emptyUserConfig, privateKey);
        uint256 countBefore = NPM.balanceOf(userAddress);
        (, , , , , , , uint128 liquidityBefore, , , , ) = NPM.positions(
            TEST_NFT
        );
        V3Automation.ExecuteParams memory params = V3Automation.ExecuteParams(
            V3Automation.Action.AUTO_ADJUST,
            Common.Protocol.UNI_V3,
            NPM,
            TEST_NFT,
            liquidityBefore,
            address(USDC),
            500000000000000000,
            400000,
            _get05DAIToUSDCSwapData(),
            0,
            0,
            "",
            0,
            0,
            block.timestamp,
            184467440737095520, // 0.01 * 2^64
            0,
            MIN_TICK_500,
            -MIN_TICK_500,
            true,
            0,
            0,
            emptyUserConfig,
            signature
        );
        // using approve / execute pattern
        vm.prank(userAddress);
        NPM.setApprovalForAll(address(v3automation), true);
        vm.prank(TEST_OWNER_ACCOUNT);
        v3automation.execute(params);
        // now we have 2 NFTs (1 empty)
        uint256 countAfter = NPM.balanceOf(userAddress);
        assertGt(countAfter, countBefore);
        (, , , , , , , uint128 liquidityAfter, , , , ) = NPM.positions(
            TEST_NFT
        );
        assertEq(liquidityAfter, 0);
    }

```

</details>

## 10.[High] refinanceFull/addNewTranche reusing a lender’s signature leads to unintended behavior

### Reuse signature

- Summary: The `refinanceFull` and `addNewTranche` functions in the `MultiSourceLoan` contract use the same signature from `RenegotiationOffer`. This allows a malicious user to reuse the signature intended for `refinanceFull` to execute `addNewTranche`, leading to unintended and risky behavior.

- Impact & Recommendation: Introduce a `type` field in `RenegotiationOffer` to differentiate between the types of operations (`refinanceFull` and `addNewTranche`). This would prevent the misuse of signatures intended for one operation being used in another.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-gondi#h-17-refinanceFull/addNewTranche-reusing-a-lender’s-signature-leads-to-unintended-behavior) & [Report](https://code4rena.com/reports/2024-04-gondi)

<details><summary>POC</summary>

```solidity

    function refinanceFull(
        RenegotiationOffer calldata _renegotiationOffer,
        Loan memory _loan,
        bytes calldata _renegotiationOfferSignature
    ) external nonReentrant returns (uint256, Loan memory) {
...
        if (lenderInitiated) {
            if (_isLoanLocked(_loan.startTime, _loan.startTime + _loan.duration)) {
                revert LoanLockedError();
            }
            _checkStrictlyBetter(
                _renegotiationOffer.principalAmount,
                _loan.principalAmount,
                _renegotiationOffer.duration + block.timestamp,
                _loan.duration + _loan.startTime,
                _renegotiationOffer.aprBps,
                totalAnnualInterest / _loan.principalAmount,
                _renegotiationOffer.fee
            );
        } else if (msg.sender != _loan.borrower) {
            revert InvalidCallerError();
        } else {
            /// @notice Borrowers clears interest
@>          _checkSignature(_renegotiationOffer.lender, _renegotiationOffer.hash(), _renegotiationOfferSignature);
            netNewLender -= totalAccruedInterest;
            totalAccruedInterest = 0;
        }
    function addNewTranche(
        RenegotiationOffer calldata _renegotiationOffer,
        Loan memory _loan,
        bytes calldata _renegotiationOfferSignature
    ) external nonReentrant returns (uint256, Loan memory) {
...
        uint256 loanId = _renegotiationOffer.loanId;
        _baseLoanChecks(loanId, _loan);
        _baseRenegotiationChecks(_renegotiationOffer, _loan);
@>      _checkSignature(_renegotiationOffer.lender, _renegotiationOffer.hash(), _renegotiationOfferSignature);
        if (_loan.tranche.length == getMaxTranches) {
            revert TooManyTranchesError();
        }

```

</details>

## 11.[Medium] A malicious user can frontrun permit transaction to make it revert due to invalid signature

### Front-run a permit

- Summary: In the StrategOperatorProxy and StrategUserInteractions smart contracts, a vulnerability was identified where a malicious user could front-run a permit transaction, causing the signature to become invalid and resulting in transaction reversion. The issue stemmed from the executePermit() function, which directly called asset.permit() with the permit signature, allowing attackers to exploit this by using the signature before it was properly validated.

- Impact & Recommendation: Wrap the asset.permit() call in a try-catch block.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-strateg-proleague#m-01-a-malicious-user-can-frontrun-permit-transaction-to-make-it-revert-due-to-invalid-signature) & [Report](https://code4rena.com/reports/2024-06-strateg-proleague)

<details><summary>POC</summary>

```solidity
    function executePermit(address _asset, address _from, address _to, uint256 _amount, bytes memory _permitParams) internal {
        DataTypes.PermitParams memory p = abi.decode(_permitParams, (DataTypes.PermitParams));
        ERC20Permit(_asset).permit(_from, _to, _amount, p.deadline, p.v, p.r, p.s);
    }

```

</details>

## 12.[Medium] signedHash should contain \_vault in vaultWithdrawalRebalance()

### `_vault` parameter

- summary: In the `StrategOperatorProxy` smart contract, the `vaultWithdrawalRebalance()` function allowed users to provide a signature for authority actions without including the `_vault` parameter in the `signedHash`. This omission allowed users to potentially use `dynParamsExit` in any vault, which could have unintended side effects across different vaults with varying strategies.

- Impact & Recommendation: The `_vault` parameter has been added to the `signedHash` calculation to ensure that signatures are specific to each vault, preventing misuse.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-strateg-proleague#m-03-signedhash-should-contain-_vault-in-vaultwithdrawalrebalance) & [Report](https://code4rena.com/reports/2024-06-strateg-proleague)

<details><summary>POC</summary>

```solidity

    function vaultWithdrawalRebalance(
        address _user,
        address _vault,
        uint256 _deadline,
        uint256 _amount,
        bytes memory _signature,
        bytes memory _portalPayload,
        bytes memory _permitParams,
        uint256[] memory _dynParamsIndexExit,
        bytes[] memory _dynParamsExit
    ) external payable returns (uint256 returnedAssets) {
        if(msg.sender != addressProvider.userInteractions()) revert OnlyUserInteraction();
        if(_deadline < block.timestamp) revert DeadlineExceeded();
        bool isProtected = _dynParamsIndexExit.length > 0;
        if(isProtected) {
            bytes32 signedHash = keccak256(
                abi.encode(
                    _user,
                    userWithdrawalRebalancedNonce[_user],
                    _deadline,
                    _dynParamsIndexExit,
                    _dynParamsExit
                )
            );

```

</details>

## 13.[Medium] Users can bypass limits and purchase more licenses than allowed by re-entering functions

### Re-entrancy via call

- Summary: The `PlayFiLicenseSale` contract has a vulnerability in the `claimLicensePartner()` and `claimLicensePublic()` functions where the commission is sent via a low-level call before state variables are updated. This allows for re-entrancy, enabling users to bypass purchase limits and acquire more licenses than allowed. An attacker can exploit this by creating a malicious contract to re-enter the purchase function.

- Impact & Recommendation: Use `nonReentrant` modifiers on affected functions and transfer commissions after updating state variables.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-playfi-proleague#h-02-Users-can-bypass-limits-and-purchase-more-licenses-than-allowed-by-re-entering-functions) & [Report](https://code4rena.com/reports/2024-06-playfi-proleague)

<details><summary>POC</summary>

```solidity

function claimLicensePublic(uint256 amount, uint256 tier, string memory referral) public payable {
    if(!publicSaleActive) revert PublicSaleNotActive();
    if(tiers[tier].totalClaimed + amount > tiers[tier].totalCap) revert TotalTierCapExceeded();
    if(claimsPerTierPerAddress[tier][msg.sender] + amount > tiers[tier].individualCap) revert IndividualTierCapExceeded();
    (uint256 toPay, uint256 commission,) = paymentDetailsForReferral(amount, tier, referral, false);
    if(msg.value < toPay) revert InsufficientPayment();
    if(commission > 0) {
        (bool sent, ) = payable(referrals[referral].receiver).call{ value: commission }("");
        if (!sent) revert CommissionPayoutFailed();
        emit CommissionPaid(referral, referrals[referral].receiver, commission);
    }
    tiers[tier].totalClaimed += amount;
    publicClaimsPerAddress[msg.sender] += amount;
    totalLicenses += amount;
    referrals[referral].totalClaims += amount;
    emit PublicLicensesClaimed(msg.sender, amount, tier, toPay, referral);
}

```

</details>
