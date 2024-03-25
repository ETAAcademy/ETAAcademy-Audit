# ETAAcademy-Adudit: 1. Transaction

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>02. Transaction</td>
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

## Delegatecall to precompile

- Summary: When the ECRECOVER precompile contract is invoked using delegatecall, it does not behave consistently with other zkSync Era operations like call and staticcall.
- Impact & Recommendation: This divergence from expected EVM behavior can lead to incorrect signature validation, potentially compromising data integrity and user funds. If the **`_address`** matches the ECRECOVER precompile contract (0x01), it's recommended to perform a static call (**`rawStaticCall`**) instead of a delegate call to ensure consistent behavior with the ECRECOVER contract.
  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/175) & [Report](https://code4rena.com/reports/2023-10-zksync)

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

## Reverts in fallback function

- Summary: Default accounts mimic externally owned accounts (EOAs) on Ethereum, but a discrepancy arises when custom accounts delegate-calls to them, triggering a revert due to an `assert(msg.sender != BOOTLOADER_FORMAL_ADDRESS)` in the default account's fallback function.
- Impact & Recommendation: One proposed solution is to add a modifier called **`ignoreInDelegateCall`** to the fallback function of default accounts, which prevents the assertion check from executing when the fallback function is invoked via a delegate call.
  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/168) & [Report](https://code4rena.com/reports/2023-10-zksync)

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
