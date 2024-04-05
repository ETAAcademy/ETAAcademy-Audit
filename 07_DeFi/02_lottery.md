# ETAAcademy-Adudit: 2. Lottery

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>02. Lottery</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>DeFi</th>
          <td>lottery</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] Tickets can be entered after prizes for current round have partially been distributed

### The clear end of a round

- Summary: The ThrusterTreasure contract is meant for a lottery game where users can enter tickets to win prizes. However, there's a significant flaw in the enterTickets() function. It checks if winning tickets for prize index 0 have been set but overlooks distributed prizes for higher indices. This flaw lets users enter tickets after some prizes have been distributed, giving them an unfair advantage.

- Impact & Recommendation: The flaw in the contract cannot be fixed by setting all prizes simultaneously because early-entered tickets would still miss out on rewards. To address this, there must be a clear distinction between the end of a round and the distribution of prizes to prevent users from spending funds with no chance of return.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-thruster#m-01-tickets-can-be-entered-after-prizes-for-current-round-have-partially-been-distributed) & [Report](https://code4rena.com/reports/2024-02-thruster)

  <details><summary>POC</summary>

  ```solidity
    function enterTickets(uint256 _amount, bytes32[] calldata _proof) external {
        ...
        require(winningTickets[currentRound_][0].length == 0, "ET");
        ...
    }

  ```

  <details>

## 2.[Medium] claimPrizesForRound transfers the entire amount deposited for a prize regardless of the number of winners

### Distribution of prizes

- Summary: The **`claimPrizesForRound()`** function transfers the entire prize amount to the first caller, ignoring the possibility of multiple winners. This denies other winners their share of the prize.

- Impact & Recommendation: It's uncertain if **`setPrize()`** amounts are for all winners or each winner individually. Scaling the owner's amount by the number of winners would be a cleaner solution.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-02-thruster#m-02-claimprizesforround-transfers-the-entire-amount-deposited-for-a-prize-regardless-of-the-number-of-winners) & [Report](https://code4rena.com/reports/2024-02-thruster)

  <details><summary>POC</summary>

  ```solidity
    function setPrize(uint256 _round, uint64 _prizeIndex, uint256 _amountWETH, uint256 _amountUSDB, uint64 _numWinners)
        external
        onlyOwner
    {
        require(_round >= currentRound, "ICR");
        require(_prizeIndex < maxPrizeCount, "IPC");
        depositPrize(msg.sender, _amountWETH, _amountUSDB);
        prizes[_round][_prizeIndex] = Prize(_amountWETH, _amountUSDB, _numWinners, _prizeIndex, uint64(_round));
    }
    function depositPrize(address _from, uint256 _amountWETH, uint256 _amountUSDB) internal {
        WETH.transferFrom(_from, address(this), _amountWETH);
        USDB.transferFrom(_from, address(this), _amountUSDB);
        emit DepositedPrizes(_amountWETH, _amountUSDB);
    }

    function claimPrizesForRound(uint256 roundToClaim) external {
        ...
        for (uint256 i = 0; i < maxPrizeCount_; i++) {
            Prize memory prize = prizes[roundToClaim][i];
            uint256[] memory winningTicketsRoundPrize = winningTickets[roundToClaim][i];
            for (uint256 j = 0; j < winningTicketsRoundPrize.length; j++) {
                uint256 winningTicket = winningTicketsRoundPrize[j];
                if (round.ticketStart <= winningTicket && round.ticketEnd > winningTicket) {
                    _claimPrize(prize, msg.sender, winningTicket);
                }
            }
        }
        ...
    }
    function _claimPrize(Prize memory _prize, address _receiver, uint256 _winningTicket) internal {
        uint256 amountETH = _prize.amountWETH;
        uint256 amountUSDB = _prize.amountUSDB;
        WETH.transfer(_receiver, amountETH);
        USDB.transfer(_receiver, amountUSDB);
        emit ClaimedPrize(_receiver, _prize.round, _prize.prizeIndex, amountETH, amountUSDB, _winningTicket);
    }

  ```

  </details>
