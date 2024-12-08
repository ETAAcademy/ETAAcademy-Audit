# ETAAcademy-Adudit: 9. GameFi

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>09. GameFi</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>EVM</th>
          <td>GameFi</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [High] The functions claimLicensePartner() and claimLicensePublic() dont update state variables allowing users to purchase more licenses than expected

### Bypass licenses purchase limits

- Summary: The functions `claimLicensePartner()` and `claimLicensePublic()` in `PlayFiLicenseSale.sol` do not update the state variables `partnerClaimsPerTierPerAddress` and `claimsPerTierPerAddress` after a user purchases licenses, causing these variables to remain at 0. This flaw allows users to bypass purchase limits and acquire more licenses than permitted.

- Impact & Recommendation: Increment these variables by the number of licenses purchased within the respective functions.

<br> 🐬: [Source](<https://code4rena.com/reports/2024-06-playfi-proleague#h-01-The-functions-claimLicensePartner()-and-claimLicensePublic()-dont-update-state-variables-allowing-users-to-purchase-more-licenses-than-expected>) & [Report](https://code4rena.com/reports/2024-06-playfi-proleague)

<details><summary>POC</summary>

```solidity

  /// @notice Claims licenses for partners + make sure they do not exceed their personal claim cap and that
  /// they paid enough.
  /// @param amount The amount of licenses to claim
  /// @param partnerCode The code of the partner sale
  function claimLicensePartner(uint256 amount,  uint256 tier, string memory partnerCode, string memory referral) public payable {
      if(!partnerSaleActive[partnerCode]) revert PartnerSaleNotActive();
      if(partnerTiers[partnerCode][tier].totalClaimed + amount > partnerTiers[partnerCode][tier].totalCap) revert TotalTierCapExceeded();
      if(partnerClaimsPerTierPerAddress[partnerCode][tier][msg.sender] + amount > partnerTiers[partnerCode][tier].individualCap) revert IndividualTierCapExceeded();
      (uint256 toPay, uint256 commission,) = paymentDetailsForPartnerReferral(amount, tier, partnerCode, referral);
      if(msg.value < toPay) revert InsufficientPayment();
      if(partnerReferrals[partnerCode].receiver != address(0)) {
          if(commission > 0) {
              (bool sent, ) = payable(partnerReferrals[partnerCode].receiver).call{ value: commission }("");
              if (!sent) revert CommissionPayoutFailed();
              emit CommissionPaid(partnerCode, partnerReferrals[partnerCode].receiver, commission);
          }
      } else {
          if(commission > 0) {
              (bool sent, ) = payable(referrals[referral].receiver).call{ value: commission }("");
              if (!sent) revert CommissionPayoutFailed();
              emit CommissionPaid(referral, referrals[referral].receiver, commission);
          }
          referrals[referral].totalClaims += amount;
      }
      partnerReferrals[partnerCode].totalClaims += amount;
      partnerTiers[partnerCode][tier].totalClaimed += amount;
      partnerClaimsPerAddress[partnerCode][msg.sender] += amount;
      totalLicenses += amount;
      emit PartnerLicensesClaimed(msg.sender, amount, tier, toPay, partnerCode, referral);
  }

  /// @notice Claims licenses for the public in a specific tier + make sure they do not exceed their personal claim
  /// cap and total tier cap. Additionally also make sure that they paid enough.
  /// @param amount The amount of licenses to claim
  /// @param tier The tier to buy the licenses from
  /// @param referral A referral code that can give discounts.
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

## 2. [High] Excess payment is not reimbursed causing users to spend more than necessary for the license

### Excess payment made by referral discounts

- Summary: The `PlayFiLicenseSale` contract had a flaw in the `claimLicensePublic()` and similar functions where any excess payment made by users was not refunded. Since the payment amount depends on referral discounts and claim counts, the actual price could end up being lower at the time of execution, leading to overpayment.

- Impact & Recommendation: Refund any excess payment, including an edge case for `claimLicenseFriendsFamily` and `claimLicenseEarlyAccess` functions.

<br> 🐬: [Source](https://code4rena.com/reports/2024-06-playfi-proleague#h-03-Excess-payment-is-not-reimbursed-causing-users-to-spend-more-than-necessary-for-the-license) & [Report](https://code4rena.com/reports/2024-06-playfi-proleague)

<details><summary>POC</summary>

```solidity

    function claimLicensePublic(uint256 amount, uint256 tier, string memory referral) public payable {
        ......
        (uint256 toPay, uint256 commission,) = paymentDetailsForReferral(amount, tier, referral, false);
        if(msg.value < toPay) revert InsufficientPayment();

    function paymentDetailsForReferral(uint256 amount, uint256 tier, string memory referral, bool isWhitelist) public view returns (uint256 toPay, uint256 commission, uint256 discount) {
        .......
        if(referrals[referral].receiver != address(0)) {
=>       uint256 totalClaims = referrals[referral].totalClaims;
            if(totalClaims < 20) {
                commission = fullPrice * 10 / 100;
            } else if (totalClaims < 40) {
                commission = fullPrice * 11 / 100;

```

</details>
