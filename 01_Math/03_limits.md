# ETAAcademy-Adudit: 2. Limits

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. Limits</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>math</th>
          <td>limits</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] Mailbox.requestL2Transaction() checks the deposit limit of msg.sender (L1WethBridge) instead of the real depositor of weth from L1, as a result, after certain time, nobody will be able to deposit weth anymore from L1

### Check the deposit limit of msg.sender not depositor

- Summary : The deposit limit check is based on the **`msg.sender`** (bridge) rather than the actual depositor. Consequently, when the bridge's deposit limit is met, further deposits are blocked, even if individual depositors haven't reached their personal limits.
- Impact & Recommendation: This flaw could prevent anyone from using Zksync to deposit WETH from L1 to L2. To address this issue, the deposit limit check should be based on the real depositor's limit instead of the bridge's.
  🐬: [Source](https://github.com/code-423n4/2023-10-zksync-findings/issues/246) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

    // SPDX-License-Identifier: MIT
    pragma solidity ^0.8.17;
    import "lib/forge-std/src/Test.sol";
    import {L1WethBridgeTest} from "./_L1WethBridge_Shared.t.sol";
    import {IAllowList} from "../../../../../../cache/solpp-generated-contracts/common/interfaces/IAllowList.sol";
    import {REQUIRED_L2_GAS_PRICE_PER_PUBDATA} from "../../../../../../cache/solpp-generated-contracts/zksync/Config.sol";
    contract DepositTest is L1WethBridgeTest {
        function deposit(address user, uint256 amount) private returns (bool) {
            hoax(user);
            l1Weth.deposit{value: amount}();
            hoax(user);
            l1Weth.approve(address(bridgeProxy), amount);
            bytes memory depositCallData = abi.encodeWithSelector(
                bridgeProxy.deposit.selector,
                user,
                bridgeProxy.l1WethAddress(),
                amount,
                1000000,                        // gas limit
                REQUIRED_L2_GAS_PRICE_PER_PUBDATA,
                user
            );
            hoax(user);
            (bool success, ) = address(bridgeProxy).call{value: 0.1 ether}(depositCallData);
            return success;
        }
        function test_DepositExceedLimit() public {
            console.log("\n \n test_DepositExceeLimit is started....$$$$$$$$$$$$$$4");
            address user1 = address(111);
            address user2 = address(222);
            address user3 = address(333);
            vm.prank(owner);
            allowList.setDepositLimit(address(0), true, 10 ether); // deposit at most 10 ether
            IAllowList.Deposit memory limitData = IAllowList(allowList).getTokenDepositLimitData(address(0));
            assertEq(limitData.depositCap, 10 ether);

            bool success = deposit(user1, 3 ether); // send 3 ether weth and 0.1 ether eth
            assertTrue(success);
            success = deposit(user2, 4 ether); // send 4 ether weth and 0.1 ether eth
            assertTrue(success);
            success =  deposit(user3, 2.7 ether + 1); // send 2.7 ether + 1 weth  and 0.1 ether eth, now a total of 10ether + 1, will it exceed?
            assertFalse(success);   // s.totalDepositedAmountPerUser[L1WethBridge] = 10 ether + 1, it exceeds the limit of 10 ether
        }
    }


  ```

  </details>
