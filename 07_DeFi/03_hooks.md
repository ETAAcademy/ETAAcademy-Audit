# ETAAcademy-Adudit: 3. Hooks

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>03. Hooks</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>DeFi</th>
          <td>hooks</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] The winner can steal claimer fees, and force him to pay for the gas

### Use hooks to claim prize without fees

- Summary: To avoid paying transaction fees, a user can exploit a vulnerability in the Claimer contract. They can create a hook function, beforeClaimPrize, which allows them to claim their prize without paying fees and return their address. When the Claimer contract attempts to claim the prize, it fails as it has already been claimed. Using a MEV searcher, the user can then claim multiple prizes, including their own, without paying fees.

- Impact & Recommendation: One of the defense against the described attack is to check the gas cost of calling the beforeClaimPrize hook. By verifying if the prize state changes from unclaimed to claimed after the hook is called, transactions can be reverted if such a change occurs, thus preventing the attack from succeeding.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-pooltogether#m-01-the-winner-can-steal-claimer-fees-and-force-him-to-pay-for-the-gas) & [Report](https://code4rena.com/reports/2024-03-pooltogether)

  <details><summary>POC</summary>

  ```solidity
    import { console2 } from "forge-std/console2.sol";
    import { PrizePoolMock } from "../contracts/mock/PrizePoolMock.sol";
    contract Auditor_MockPrizeToken {
        mapping(address user => uint256 balance) public balanceOf;
        function mint(address user, uint256 amount) public {
            balanceOf[user] += amount;
        }
        function burn(address user, uint256 amount) public {
            balanceOf[user] -= amount;
        }
    }
    contract Auditor_PrizePoolMock {
        Auditor_MockPrizeToken public immutable prizeToken;
        constructor(address _prizeToken) {
            prizeToken = Auditor_MockPrizeToken(_prizeToken);
        }
        // The reward is fixed to 100 tokens
        function claimPrize(
            address winner,
            uint8 /* _tier */,
            uint32 /* _prizeIndex */,
            address /* recipient */,
            uint96 reward,
            address rewardRecipient
        ) public returns (uint256) {
            // Distribute rewards if the PrizePool earns a reward
            if (prizeToken.balanceOf(address(this)) >= 100e18) {
                prizeToken.mint(winner, 100e18 - uint256(reward)); // Transfer reward tokens to the winner
                // Transfer fees to the claimer Receipent.
                // Instead of adding balance to the PrizePool contract and then the claimerRecipent
                // Can withdraw it, we will transfer it to the claimerRecipent directly in our simulation
                prizeToken.mint(rewardRecipient, reward);
                // Simulating Token transfereing by minting and burning
                prizeToken.burn(address(this), 100e18);
            } else {
                return 0;
            }
            return uint256(100e18);
        }
    }
    contract Auditor_Claimer {
        ClaimableWrapper public immutable prizeVault;
        constructor(address _prizeVault) {
            prizeVault = ClaimableWrapper(_prizeVault);
        }
        function claimPrizes(
            address[] calldata _winners,
            uint8 _tier,
            uint256 _claimerFees,
            address _feeRecipient
        ) external {
            for (uint i = 0; i < _winners.length; i++) {
                prizeVault.claimPrize(_winners[i], _tier, 0, uint96(_claimerFees), _feeRecipient);
            }
        }
    }

  ```

  </details>
