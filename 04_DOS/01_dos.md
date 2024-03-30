# ETAAcademy-Adudit: 1. Dos

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. DOS</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>dos</th>
          <td>dos</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [Medium] EIP-155 is not enforced, allowing attackers/malicious operators to profit from replaying transactions

### Absence of enforcement of EIP-155

- Summary: Attackers and malicious operators profit from replaying transactions due to the absence of enforcement of **`EIP-155`**, which prevents replay attacks by including the chain ID in the transaction's signature.

- Impact & Recommendation: Attackers can replay transactions from networks not protected by EIP-155, while operators can replay early user transactions from other EVM networks to collect gas fees or profit directly.
  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/882) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```rust

                  let should_check_chain_id = if matches!(
                    common_data.transaction_type,
                    TransactionType::LegacyTransaction
                ) && common_data.extract_chain_id().is_some()
                {
                    U256([1, 0, 0, 0])
                } else {
                    U256::zero()
                };
    pub fn extract_chain_id(&self) -> Option<u64> {
        let bytes = self.input_data()?;
        let chain_id = match bytes.first() {
            Some(x) if *x >= 0x80 => {
                let rlp = Rlp::new(bytes);
                let v = rlp.val_at(6).ok()?;
                PackedEthSignature::unpack_v(v).ok()?.1?
            }

  ```

  </details>

## 2. [Low] CoreRef::emergencyAction is susceptible to returnbomb attack

### Lack of assembly to handle returned data

- Summary: **`Emergency()`** lack of assembly to handle returned data leaves it vulnerable to returnbomb attacks, especially when interacting with untrusted external contracts.

- Impact & Recommendation: Consider using the ExcessivelySafeCall library or assembly to mitigate the vulnerability.
  🐬: [Source](https://code4rena.com/reports/2023-12-ethereumcreditguild) & [Report](https://code4rena.com/reports/2023-12-ethereumcreditguild)

  <details><summary>POC</summary>

  ```solidity

    /// @notice due to inflexibility of current smart contracts,
    /// add this ability to be able to execute arbitrary calldata
    /// against arbitrary addresses.
    /// callable only by governor
    function emergencyAction(Call[] calldata calls)
        external
        payable
        onlyCoreRole(CoreRoles.GOVERNOR)
        returns (bytes[] memory returnData)
    {
        returnData = new bytes[](calls.length);
        for (uint256 i = 0; i < calls.length; i++) {
            address payable target = payable(calls[i].target);
            uint256 value = calls[i].value;
            bytes calldata callData = calls[i].callData;
            (bool success, bytes memory returned) = target.call{value: value}(callData);
            require(success, "CoreRef: underlying call reverted");
            returnData[i] = returned;
        }
    }

  ```

  </details>
