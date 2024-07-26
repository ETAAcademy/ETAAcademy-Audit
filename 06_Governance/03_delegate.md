# ETAAcademy-Adudit: 3. Delegate

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>03. Delegate</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>Governance</th>
          <td>delegate</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] Delegations cannot be removed in some cases due to vulnerable

### Empty `bytes32 _rights` parameter

- Summary: In the `MultiSourceLoan` contract, the `revokeDelegate()` function has a vulnerability that prevents the removal of custom delegations due to passing empty rights to `delegateERC721`. This mismatch causes the function to fail in clearing the existing delegation, allowing an old borrower to exploit an old delegation to claim benefits like event tickets.

- Impact & Recommendation: The recommended fix is to modify `revokeDelegate()` to accept and pass the appropriate `bytes32 _rights` parameter to correctly revoke delegations with custom rights.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-gondi#m-03-Delegations-cannot-be-removed-in-some-cases-due-to-vulnerable) & [Report](https://code4rena.com/reports/2024-06-gondi)

<details><summary>POC</summary>

```solidity
//src/lib/loans/MultiSourceLoan.sol
  function delegate(uint256 _loanId, Loan calldata loan, address _delegate, bytes32 _rights, bool _value) external {
      if (loan.hash() != _loans[_loanId]) {
          revert InvalidLoanError(_loanId);
      }
      if (msg.sender != loan.borrower) {
          revert InvalidCallerError();
      }
      //@audit-info a borrower can pass custom rights to delegateERC721
|>      IDelegateRegistry(getDelegateRegistry).delegateERC721(
          _delegate, loan.nftCollateralAddress, loan.nftCollateralTokenId, _rights, _value
      );
      emit Delegated(_loanId, _delegate, _value);
  }

  //src/lib/loans/MultiSourceLoan.sol
  function revokeDelegate(address _delegate, address _collection, uint256 _tokenId) external {
      if (ERC721(_collection).ownerOf(_tokenId) == address(this)) {
          revert InvalidMethodError();
      }
      //@audit revokeDelegate will always pass empty rights.
|>      IDelegateRegistry(getDelegateRegistry).delegateERC721(_delegate, _collection, _tokenId, "", false);
      emit RevokeDelegate(_delegate, _collection, _tokenId);
  }

```

</details>
