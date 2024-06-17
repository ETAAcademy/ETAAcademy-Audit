# ETAAcademy-Adudit: 3. Access

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>03. Access</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>dos</th>
          <td>access</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [High] Dual transaction nature of composed message transfer allows anyone to steal user funds

### Restricts the lzCompose(…) method

- Summary: Sending OFTs via `lzCompose(…)` is permissionless, allowing anyone to invoke it. Adversaries can monitor for incoming OFTs and use them before legitimate processing, stealing user funds. If `lzCompose(…)` fails, adversaries can exploit retries before users, misusing the OFTs.

- Impact & Recommendation: An immediate but temporary fix restricts the lzCompose(…) method to trusted or whitelisted executors. Alternatively, a design change is proposed, involving directly implementing `lzReceive(…)` to eliminate the need for a composed message altogether.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-canto#h-02-dual-transaction-nature-of-composed-message-transfer-allows-anyone-to-steal-user-funds) & [Report](https://code4rena.com/reports/2024-03-canto)

  <details><summary>POC</summary>

  ```solidity
    it("lzCompose: successful deposit and send on canto", async () => {
        // update whitelist
        await ASDUSDC.updateWhitelist(USDCOFT.target, true);
        // call lzCompose with valid payload
        await expect(
            ASDRouter.lzCompose(
                USDCOFT.target,
                guid,
                generatedComposeMsg(
                    refundAddress,
                    amountUSDCSent,
                    generatedRouterPayload(cantoLzEndpoint.id, refundAddress, TESTASD.target, TESTASD.target, "0", refundAddress, "0")
                ),
                executorAddress,
                "0x"
            )
        )
            .to.emit(ASDRouter, "ASDSent")
            .withArgs(guid, refundAddress, TESTASD.target, amountUSDCSent, cantoLzEndpoint.id, false);
        // expect ASD to be sent to canto
        expect(await TESTASD.balanceOf(refundAddress)).to.equal(amountUSDCSent);
    });

  ```

  </details>

## 2. [Medium] ASDRouter didn’t call ASDUSDC.approve() to to grant permission for crocSwapAddress to spend their ASDUSDC

### Lack of allowance

- Summary: The function `ASDRouter#lzCompose()` fails because the necessary allowance is not set for `crocSwapAddres**s**` to spend ASDUSDC tokens. This prevents the successful swap of ASDUSDC for NOTE, resulting in a failed execution of `_swapOFTForNote()` and refunding all ASDUSDC to the designated receiver.

- Impact & Recommendation: Set `allowance` to the maximum value.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-canto#m-02-asdrouter-didnt-call-asdusdcapprove-to-to-grant-permission-for-crocswapaddress-to-spend-their-asdusdc) & [Report](https://code4rena.com/reports/2024-03-canto)

  <details><summary>POC</summary>

  ```solidity
    const { expect } = require("chai");
    const { ethers } = require("hardhat");
    const hre = require("hardhat");
    describe("Dex", function () {
        const dexAddress = "0x9290C893ce949FE13EF3355660d07dE0FB793618";
        const usdcAddress = "0x80b5a32E4F032B2a058b4F29EC95EEfEEB87aDcd";
        const cNoteAddress = "0xEe602429Ef7eCe0a13e4FfE8dBC16e101049504C";
        const usdcWhaleAddress = "0xfAc5EBD2b1b830806189FCcD0255DC9B174decbc";
        let dex;
        let usdc;
        let cNote;
        this.beforeEach(async () => {
            dex  = await hre.ethers.getContractAt("ICrocSwapDex", dexAddress);
            usdc = await hre.ethers.getContractAt("IERC20", usdcAddress);
            cNote = await hre.ethers.getContractAt("CErc20Interface", cNoteAddress);
        });
        it("User can only call DEX.swap() for cNote after call USDC.approve(dex)", async () => {
            await hre.network.provider.request({
                method: "hardhat_impersonateAccount",
                params: [usdcWhaleAddress],
            });
            whale = await ethers.getSigner(usdcWhaleAddress);
            let usdcBalanceBeforeSwap = await usdc.balanceOf(usdcWhaleAddress);
            await usdc.connect(whale).approve(dexAddress, 0);
            expect(await usdc.allowance(usdcWhaleAddress,dexAddress)).to.be.equal(0);
            //@audit-info swap failed due to zero allowance
            expect(dex.connect(whale).swap(
                usdcAddress,
                cNoteAddress,
                36000,
                true,
                true,
                2000000,
                0,
                0,
                0,
                0
            )).to.be.reverted;
            //@audit-info user set the allowance for dex to 2000000
            await usdc.connect(whale).approve(dexAddress, 2000000);
            expect(await usdc.allowance(usdcWhaleAddress,dexAddress)).to.be.equal(2000000);
            swapTx = await dex.connect(whale).swap(
                usdcAddress,
                cNoteAddress,
                36000,
                true,
                true,
                2000000,
                0,
                ethers.parseEther("10000000000"),
                0,
                0
            );
            await swapTx.wait();
            let usdcBalanceAfterSwap = await usdc.balanceOf(usdcWhaleAddress);
            let usedUSDC = usdcBalanceBeforeSwap - usdcBalanceAfterSwap;
            //@audit-info swap succeeded and 2000000 of USDC was tranferred from user
            expect(usedUSDC).to.be.equal(2000000);
        });
    });

  ```

  </details>

## 3.[High] DOS of completeQueuedWithdrawal when ERC20 buffer is filled

### Restricted access

- Summary: The OperatorDelegator::completeQueuedWithdrawal function in the code tries to fill the ERC20 withdrawal buffer but fails because it calls depositQueue::fillERC20withdrawBuffer, which is restricted to be accessed only by the RestakeManager contract. This restriction causes the function call to revert, leading to a Denial of Service (DoS) that prevents the completion of withdrawals and the retrieval of funds from EigenLayer (EL), resulting in a potential loss of funds for the protocol and its users.

- Impact & Recommendation: The simplest solution is to remove the onlyRestakeManager modifier from the depositQueue::fillERC20withdrawBuffer function, allowing anyone to call it.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-renzo#h-07-dos-of-completequeuedwithdrawal-when-erc20-buffer-is-filled) & [Report](https://code4rena.com/reports/2024-04-renzo)

  <details><summary>POC</summary>

  ```solidity
    function completeQueuedWithdrawal(
        IDelegationManager.Withdrawal calldata withdrawal,
        IERC20[] calldata tokens,
        uint256 middlewareTimesIndex
    ) external nonReentrant onlyNativeEthRestakeAdmin {
        uint256 gasBefore = gasleft();
        if (tokens.length != withdrawal.strategies.length) revert MismatchedArrayLengths();
        // Complete the queued withdrawal from EigenLayer with receiveAsToken set to true
        delegationManager.completeQueuedWithdrawal(withdrawal, tokens, middlewareTimesIndex, true);
        IWithdrawQueue withdrawQueue = restakeManager.depositQueue().withdrawQueue();
        for (uint256 i; i < tokens.length; ) {
            if (address(tokens[i]) == address(0)) revert InvalidZeroInput();
            // Deduct queued shares for tracking TVL
            queuedShares[address(tokens[i])] -= withdrawal.shares[i];
            // Check if the token is not Native ETH
            if (address(tokens[i]) != IS_NATIVE) {
                // Check the withdrawal buffer and fill if below buffer target
                uint256 bufferToFill = withdrawQueue.getBufferDeficit(address(tokens[i]));
                // Get the balance of this contract
                uint256 balanceOfToken = tokens[i].balanceOf(address(this));
                if (bufferToFill > 0) {
                    bufferToFill = (balanceOfToken <= bufferToFill) ? balanceOfToken : bufferToFill;
                    // Update the amount to send to the operator Delegator
                    balanceOfToken -= bufferToFill;
                    // Safely approve for depositQueue
                    tokens[i].safeApprove(address(restakeManager.depositQueue()), bufferToFill);
                    // Fill the Withdraw Buffer via depositQueue
                    restakeManager.depositQueue().fillERC20withdrawBuffer(
                        address(tokens[i]),
                        bufferToFill
                    );
                }
                // Deposit remaining tokens back to EigenLayer
                if (balanceOfToken > 0) {
                    _deposit(tokens[i], balanceOfToken);
                }
            }
            unchecked {
                ++i;
            }
        }
        // Emit the Withdraw Completed event with withdrawalRoot
        emit WithdrawCompleted(
            delegationManager.calculateWithdrawalRoot(withdrawal),
            withdrawal.strategies,
            withdrawal.shares
        );
        // Record the current spent gas
        _recordGas(gasBefore);
    }

  ```

  </details>

## 4.[High] Availability of deposit invariant can be bypassed

### ETH deposits influence lpETH minting

- Summary: The issue in the PrelaunchPoints.sol contract arises during the claiming of lpETH, where non-ETH/WETH LRT tokens are swapped to ETH using \_fillQuote(), and this ETH is subsequently used to mint lpETH. The vulnerability occurs because any ETH transferred directly to the contract before calling claim() is also included in the minting calculation. This allows users to bypass the intended lock-up period and artificially inflate their lpETH holdings beyond their initial LRT stake.

- Impact & Recommendation: To mitigate this, it is crucial to modify the contract so that only ETH obtained from LRT swaps is used for minting lpETH, and to prevent any additional ETH deposits from influencing the lpETH minting process.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-loop#h-01-availability-of-deposit-invariant-can-be-bypassed) & [Report](https://code4rena.com/reports/2024-05-loop)

  <details><summary>POC</summary>

  ```solidity
      function testClaimLRT() public {
        // user only needs to lock 1 wei LRT, then he could cliam any amount he want
        uint256 lockAmount = 1;
        lrt.approve(address(prelaunchPoints), lockAmount);
        prelaunchPoints.lock(address(lrt), lockAmount, referral);

        prelaunchPoints.setLoopAddresses(address(lpETH), address(lpETHVault));
        vm.warp(prelaunchPoints.loopActivation() + prelaunchPoints.TIMELOCK() + 1);
        prelaunchPoints.convertAllETH();

        vm.warp(prelaunchPoints.startClaimDate() + 1);
        bytes4 y =  bytes4(0x415565b0);
        bytes memory da = abi.encodeWithSelector(y, address(lrt), (ETH), 0);
        // user deposit eth to this and call claim to get lp
        address(prelaunchPoints).call{value: 1 ether}("");
        prelaunchPoints.claim(address(lrt), 0, PrelaunchPoints.Exchange.TransformERC20, da);

        console.log("lp get : ",lpETH.balanceOf(address(this)));
    }

  ```

  </details>
