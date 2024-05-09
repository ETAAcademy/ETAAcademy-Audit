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
```
