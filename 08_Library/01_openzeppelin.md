# ETAAcademy-Adudit: 1. Openzeppelin

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. Openzeppelin</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>library</th>
          <td>openzeppelin</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] `maxYieldVaultWithdraw()` uses `yieldVault.convertToAssets()`

### ERC4626: `convertToAssets` and `convertToShares` replaced with `preview`

- Summary: convertToAssets and convertToShares functions could be replaced with yield vault's preview functions for accurate accounting based on current conditions. However, since preview functions may revert, they must be used carefully in prize vault functions like maxDeposit, maxWithdraw, ensuring they don't revert.

- Impact & Recommendation: Use yieldVault.previewRedeem(yieldVault.maxRedeem(address(this))).
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-pooltogether#m-01-the-winner-can-steal-claimer-fees-and-force-him-to-pay-for-the-gas) & [Report](https://code4rena.com/reports/2024-03-pooltogether)

  <details><summary>POC</summary>

  ```solidity
    function _maxYieldVaultWithdraw() internal view returns (uint256) {
        return yieldVault.convertToAssets(yieldVault.maxRedeem(address(this)));
    }

  ```

  </details>

## 2.[Medium] Funds locked due to missing transfer check

### Two corner cases of ERC-20 and ERC-4246

- Summary: ERC-4626 does not guarantee transferring a specific amount of assets during redemption; only the specified shares are burned. When the contract tries to redeem shares for assets but receives feer shares than expected, leading to an insufficient asset balance. Despite the failed asset transfer, the ERC-20 compliant token only returns **`false`** instead of reverting the transaction. As a result, users’ assets become locked in the PrizeVault contract.

- Impact & Recommendation: To fix the problems, use OpenZeppelin's SafeERC20 safeTransfer for ERC-20 transfers and withdraw the necessary shares from the yield vault, either directly or after previewWithdraw/redeem.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-pooltogether#m-06-funds-locked-due-to-missing-transfer-check) & [Report](https://code4rena.com/reports/2024-03-pooltogether)

  <details><summary>POC</summary>

  ```solidity
    // Place in test/unit/PrizeVault/PoCLockedFunds.t.sol
    pragma solidity ^0.8.24;
    import { UnitBaseSetup } from "./UnitBaseSetup.t.sol";
    import { IERC20, IERC4626 } from "openzeppelin/token/ERC20/extensions/ERC4626.sol";
    import { ERC20PermitMock } from "../../contracts/mock/ERC20PermitMock.sol";
    import { ERC4626Mock } from "openzeppelin/mocks/ERC4626Mock.sol";
    import { Math } from "openzeppelin/utils/math/Math.sol";
    // An ERC20-compliant token that does not throw on insufficient balance.
    contract NoRevertToken is IERC20 {
        uint8   public decimals = 18;
        uint256 public totalSupply;
        mapping (address => uint)                      public balanceOf;
        mapping (address => mapping (address => uint)) public allowance;
        constructor(uint _totalSupply) {
            totalSupply = _totalSupply;
            balanceOf[msg.sender] = _totalSupply;
            emit Transfer(address(0), msg.sender, _totalSupply);
        }
        function transfer(address dst, uint wad) external returns (bool) {
            return transferFrom(msg.sender, dst, wad);
        }
        function transferFrom(address src, address dst, uint wad) virtual public returns (bool) {
            if (balanceOf[src] < wad) return false;                        // insufficient src bal
            if (src != msg.sender && allowance[src][msg.sender] != type(uint).max) {
                if (allowance[src][msg.sender] < wad) return false;        // insufficient allowance
                allowance[src][msg.sender] = allowance[src][msg.sender] - wad;
            }
            balanceOf[src] -= wad;
            balanceOf[dst] += wad;
            emit Transfer(src, dst, wad);
            return true;
        }
        function approve(address usr, uint wad) virtual external returns (bool) {
            allowance[msg.sender][usr] = wad;
            emit Approval(msg.sender, usr, wad);
            return true;
        }
    }
    // An ERC4626-compliant (yield) vault.
    // `withdraw(assets)` burns `assets * totalSupply / (totalAssets + 1)` shares.
    // `redeem(shares)` transfers `shares * (totalAssets + 1) / (totalSupply + 1)` assets.
    contract YieldVault is ERC4626Mock {
        using Math for uint256;
        constructor(address _asset) ERC4626Mock(_asset) {}
        function previewWithdraw(uint256 assets) public view virtual override returns (uint256) {
            return assets.mulDiv(totalSupply(), totalAssets() + 1);
        }
    }
    // Demonstrate that all of Alice's funds are locked in the PrizeVault,
    // with all corresponding shares burned.
    contract PoCLockedFunds is UnitBaseSetup {
        NoRevertToken asset;
        function setUpUnderlyingAsset() public view override returns (ERC20PermitMock) {
            return ERC20PermitMock(address(asset));
        }
        function setUpYieldVault() public override returns (IERC4626) {
            return new YieldVault(address(underlyingAsset));
        }
        function setUp() public override {
            return;
        }
        function test_poc_lockedFundsOnLossyWithdrawal() public {
            uint256 deposited = 1e18;
            // Mint 10^18 tokens and transfer them to Alice.
            asset = new NoRevertToken(deposited);
            super.setUp();
            asset.transfer(alice, deposited);
            // Alice holds all tokens, the yield vault and the price vaults are empty.
            assertEq(underlyingAsset.balanceOf(alice), deposited);
            assertEq(underlyingAsset.balanceOf(address(vault)), 0);
            assertEq(underlyingAsset.balanceOf(address(yieldVault)), 0);
            assertEq(yieldVault.totalSupply(), 0);
            assertEq(yieldVault.balanceOf(address(vault)), 0);
            assertEq(vault.totalSupply(), 0);
            assertEq(vault.balanceOf(alice), 0);
            // Alice enters the vault.
            vm.startPrank(alice);
            underlyingAsset.approve(address(vault), deposited);
            vault.deposit(deposited, alice);
            // All assets were transferred into the yield vault,
            // as many yield vault shares were minted to the prize vault, and
            // as many prize vault shares were minted to Alice.
            assertEq(underlyingAsset.balanceOf(alice), 0);
            assertEq(underlyingAsset.balanceOf(address(vault)), 0);
            assertEq(underlyingAsset.balanceOf(address(yieldVault)), deposited);
            assertEq(yieldVault.totalSupply(), deposited);
            assertEq(yieldVault.balanceOf(address(vault)), deposited);
            assertEq(vault.totalSupply(), deposited);
            assertEq(vault.balanceOf(alice), deposited);
            // Perform the lossy withdraw.
            vault.withdraw(deposited, alice, alice);
            // At this point Alice should've received all her assets back,
            // and all prize/yield vault shares should've been burned.
            // In contrast, no assets were transferred to Alice,
            // but (almost) all shares have been burned.
            assertEq(underlyingAsset.balanceOf(alice), 0);
            assertEq(underlyingAsset.balanceOf(address(vault)), 999999999999999999);
            assertEq(underlyingAsset.balanceOf(address(yieldVault)), 1);
            assertEq(yieldVault.totalSupply(), 1);
            assertEq(yieldVault.balanceOf(address(vault)), 1);
            assertEq(vault.totalSupply(), 0);
            assertEq(vault.balanceOf(alice), 0);
            // As a result, Alice's funds are locked in the vault;
            // she cannot even withdraw a single asset.
            vm.expectRevert();
            vault.withdraw(1, alice, alice);
            vm.expectRevert();
            vault.redeem(1, alice, alice);
        }
    }


  ```

  </details>
