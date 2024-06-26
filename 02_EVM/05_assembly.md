# ETAAcademy-Adudit: 5. Assembly

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>05. Assembly</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>EVM</th>
          <td>assembly</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

## 1.[Medium] The colRedeemed variable is wrongly retrieved in LibBytes::readProposalData function

### Add v.s. And

- Summary: In the LibBytes::readProposalData function, the colRedeemed variable is incorrectly retrieved due to a mistake in the inline assembly code. The current implementation uses the add operation instead of and, leading to incorrect values for colRedeemed.

- Impact & Recommendation: Replace the add operation with and to correctly apply the mask and retrieve the accurate value for colRedeemed.
  <br> 🐬: [Source](https://code4rena.com/reports/2024-03-dittoeth#m-03-the-colredeemed-variable-is-wrongly-retrieved-in-libbytesreadproposaldata-function) & [Report](https://code4rena.com/reports/2024-03-dittoeth)

<details><summary>POC</summary>

```solidity
contract TestAssembly is Test {
    Assembly asm = new Assembly();
    uint256 testFullWord = 0x1234599990abcdef1234567890abcdef1234567890abcdef1234567890abcdef;
    function test_assembly() external view {
        assert(asm.incorrectColRedeemed(testFullWord) < asm.correctColRedeemed(testFullWord));
        console.log('Incorrect:', asm.incorrectColRedeemed(testFullWord));
        console.log('Correct: ', asm.correctColRedeemed(testFullWord));
    }

```

</details>
