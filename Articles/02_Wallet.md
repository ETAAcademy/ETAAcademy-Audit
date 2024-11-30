# ETAAcademy-Adudit: 2. Wallet

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>02. Wallet</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>Wallet</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Eta](https://twitter.com/pwhattie), looking forward to your joining

# Web3 Wallet Security: Top Vulnerabilities and Fixes

Wallet vulnerabilities range from improper data storage and domain validation flaws to supply chain threats and novel challenges introduced by advanced standards like EIP-4337. Mitigating these risks requires adopting best practices, rigorous validation mechanisms, and user education to safeguard digital assets effectively.

## 1. **Sensitive Data Storage**

**Overview**  
It poses significant security risks to storing sensitive information such as wallet passwords and private keys insecurely in plain text or unsafe locations (e.g., Android external storage, iOS UserDefaults, or local storage) . Using HTML5 Local Storage for session data, including critical key data, is particularly vulnerable to Cross-Site Scripting (XSS) attacks. Data in Local Storage can be freely read and modified through JavaScript without access control. If a user's device is compromised by malware, plaintext private keys could be directly exposed to attackers, endangering their digital assets.

**Example**  
Through an XSS vulnerability, attackers can extract a dApp's key data and issue malicious transaction requests to connected wallets. This could lead to unauthorized manipulation of a user's assets.

**Short-Term Measures**

1. Consider using cookies to store and transmit tokens instead of Local Storage.
2. Enable Cross-Site Request Forgery (CSRF) protection to mitigate these risks.
3. Configure cookies with the `httpOnly` flag to prevent JavaScript access and the `secure` flag to restrict transmission over HTTPS.

**Insecure Storage Example**

Unencrypted file:

```json
{
  "keystore": {
    "derivation": "m/0h",
    "pw_hash_version": 1,
    "root_fingerprint": "8dce399b",
    "seed": "2s60b+ylh6UjagB7Le9xEA+c7Bv1QPP7mXw...",
    "seed_type": "segwit",
    "type": "bip32",
    "xprv": "/b79KEOPszLEpSrFdMD637SkuMKYrXd5...",
    "xpub": "zpub6nbUj438iUpLF4e9xSGiq43PMiJrcoI..."
  }
}
```

Encrypted file:

```bash
+ wallets cat ./default_wallet

Qk1 FMQT007s8sDCTvcaBd jiVRRTNTONRupdopwilb+QNNv+1TLbn8AdCdPCD2Vqtw8.1m0QqBBq+p61orkXbAYr2wum’ d4962uHIMa+kY6M6nuHe
...

PAQuxDvQIk4tSY/WyEt/VI7yqrMT8/+CxOCjySmIt 1Mkrb3dg5Ot7b1 1ThSHgbxDPGiTKRTUCck1ecZ5zQNCaj eSZZySUTnnMe8NFa4vIMFBR3KL
...
bb8i xf QpZhLFMcBnSSLHhEwbS8chggPX4B2Gtk7T7VERTEQVCVwCFAr3XPKXL j sBKRHKLLHIJkmCnUn7i9Ro7kBk7AFL tukNXNoy2xUXci
...

```

**Solutions**

1. **Avoid Local Storage for Sensitive Data**: Local Storage is vulnerable to XSS attacks. Transition to safer methods like `httpOnly` cookies and implement CSRF protection.
2. **Do Not Store Seed Phrases on Devices**: Device compromises could lead to seed phrase theft and, consequently, loss of funds.

---

## 2. **Domain Validation Flaws in Wallet and DApp Communication**

**Overview**  
When connecting wallets (e.g., MetaMask, WalletConnect) to decentralized applications (dApps), verifying the requesting domain's authenticity is critical. However, in cross-platform or cross-application communication scenarios, messages often pass through relay servers, making domain validation challenging. This opens the door to phishing attacks where malicious dApps impersonate legitimate ones by manipulating domain information.

**Example**  
In SDKs like TonConnect, the `dappMetadata` configuration holds key dApp information, including the `manifestUrl`. If this URL is tampered with, malicious actors can trick wallets into connecting to fake dApps.

Example of a tampered configuration:

```json
{
  "url": "https://ton.org",
  "name": "Fake DApp",
  "iconUrl": "https://example.com/icon.png",
  "termsOfUseUrl": "https://example.com/terms.txt",
  "privacyPolicyUrl": "https://example.com/privacy.txt"
}
```

**Solutions**

1. **Enhance `dappMetadata` Validation**: Ensure URLs are verified and fetched securely to prevent tampering.
2. **Domain Validation Mechanism**: Add domain validation in cross-platform communications to ensure the source cannot be spoofed.
3. **Improve User Alerts**: Provide warnings about unverified or suspicious dApps to reduce phishing risks.

---

### 3. **Supply Chain Attacks and Fake Wallets**

**Overview**

Supply chain attacks occur when software installation packages, even if downloaded from official websites, are compromised during the supply chain process and injected with malicious code. This is not just a theoretical risk but a real-world threat. Hash verification is a reliable method to determine whether an installation package has been tampered with. By comparing the hash value provided on the official website with the actual hash of the downloaded file, users can verify the file's integrity and authenticity. However, not all users are equipped to perform hash verification, making them more susceptible to trusting misleading explanations, such as dismissing antivirus warnings as false positives, which increases their vulnerability to attacks.

Fake wallets exploit phishing schemes by designing websites that closely mimic legitimate wallet platforms, often with nearly identical layouts and branding to deceive users. For example, phishing sites imitating Trust Wallet often have only slight differences in domain names, making them difficult to distinguish from the real website. These fake wallets often contain backdoors capable of hijacking the mnemonic phrase generation function or injecting malicious code to steal private keys or seed phrases.

**Example**

On the VirusTotal platform, a common desktop wallet installation package was scanned by 65 antivirus programs, with 19 identifying the file as malicious. This highlights the importance of improving user verification capabilities.

The BombFlower group, known for distributing fake wallets, uses a unique backdoor strategy. Unlike traditional malware-laden fake wallets, BombFlower embeds another application's binary file within the malicious application.

Initially, the BombFlower malware extracts an APK file (e.g., "bitkeep.apk") from its memory and installs it within a virtual client environment. Once the user installs and launches the BombFlower application, they are actually interacting with this embedded trojanized wallet. User private keys or seed phrases are then extracted directly from the device memory.

Another notable feature of the BombFlower group is their use of a forensic countermeasure called "ZipBomb." A ZipBomb is a compressed file designed to generate a massive number of junk files upon extraction, overwhelming automated analysis tools. This technique effectively prevents many antivirus programs from identifying the malicious software.

**Solutions**

1. **Detailed Hash Verification Tutorials**: Wallet providers should offer comprehensive tutorials on how to perform hash verification, empowering users to independently verify the integrity of downloaded files.
2. **Enhanced File Integrity Protections**: Wallet providers should improve product designs to ensure robust file integrity, bolstering user trust in their offerings.

---

## 4. **Vulnerabilities in EIP-4337-Based Account Abstraction Wallets**

**Overview**  
EIP-4337 defines a process where users sign data (`UserOperation`) and submit it to a specialized mempool (`Alt Mempool`) for further processing. A `Bundler` validates these operations before passing them to the `EntryPoint` for execution. While this system enhances flexibility, it introduces unique vulnerabilities, such as improper validation of the standard interfaces or potential bypass of security checks.

**Examples**

1. **Compliance with EIP-4337 Standard Interfaces**  
   Ensure that core interfaces are implemented correctly and return values adhere to the standard. Key interfaces include `validateUserOp` and `validatePaymasterUserOp`. The return values must include `authorizer`, `validUntil`, and `validAfter`. For signature validation, failure should return `SIG_VALIDATION_FAILED` (value 1), while success should return `SIG_VALIDATION_SUCCESS` (value 0). Similarly, Paymasters must implement `validatePaymasterUserOp` to validate Paymaster-related data.

   Example Implementation:

   ```solidity
   function validateUserOp(
       PackedUserOperation calldata userOp,
       bytes32 userOpHash,
       uint256 missingAccountFunds
   ) external returns (uint256 validationData);

   function validatePaymasterUserOp(
       PackedUserOperation calldata userOp,
       bytes32 userOpHash,
       uint256 maxCost
   ) external returns (bytes memory context, uint256 validationData);

   function postOp(
       PostOpMode mode,
       bytes calldata context,
       uint256 actualGasCost,
       uint256 actualUserOpFeePerGas
   ) external;
   ```

2. **Caller Trustworthiness Validation**  
   Verify that wallet and Paymaster interfaces can only be invoked by a trusted EntryPoint contract. For example, enforce access control checks using functions like `_requireFromEntryPointOrOwner`.

   Example Implementation:

   ```solidity
   function entryPoint() public view virtual override returns (IEntryPoint) {
       return _entryPoint;
   }

   function execute(address dest, uint256 value, bytes calldata func) external {
       _requireFromEntryPointOrOwner();
       _call(dest, value, func);
   }

   function executeBatch(address[] calldata dest, uint256[] calldata value, bytes[] calldata func) external {
       _requireFromEntryPointOrOwner();
       ...
   }
   ```

3. **Fee Payment Support**  
   Ensure the wallet supports direct payment of transaction fees by verifying that the `validateUserOp` function handles transferring `missingAccountFunds` to the EntryPoint contract.

   Example Implementation:

   ```solidity
   function validateUserOp(
       PackedUserOperation calldata userOp,
       bytes32 userOpHash,
       uint256 missingAccountFunds
   ) external virtual override returns (uint256 validationData) {
       ...
       _payPrefund(missingAccountFunds);
   }
   ```

4. **ERC1271 Standard Compliance**  
   Ensure the wallet correctly implements the ERC1271 standard for signature validation. Check that the `isValidSignature` function is secure and follows the specification.

   Example Implementation:

   ```solidity
   function isValidSignature(bytes32 _dataHash, bytes calldata _signature) public view override returns (bytes4) {
       // Caller should be a Safe
       ISafe safe = ISafe(payable(msg.sender));
       bytes memory messageData = encodeMessageDataForSafe(safe, abi.encode(_dataHash));
       bytes32 messageHash = keccak256(messageData);
       if (_signature.length == 0) {
           require(safe.signedMessages(messageHash) != 0, "Hash not approved");
       } else {
           safe.checkSignatures(messageHash, _signature);
       }
       return EIP1271_MAGIC_VALUE;
   }
   ```

**Solutions**

1. **Standards Compliance**:  
   Ensure all interfaces and return values strictly adhere to EIP-4337, ERC1271, and other relevant standards.

2. **Validation Enhancements**:  
   Strengthen checks on access control, signature verification logic, storage access, and extension module security.

3. **Deployment Best Practices**:

   - Support multi-chain compatibility.
   - Use `CREATE2` for deterministic wallet creation.
   - Ensure consistency across repeated wallet creations.
   - Use Solidity versions 0.8.20 or lower, and ensure compatibility with the `paris` hard fork.

4. **Fee Handling**:
   - Allow direct payment of transaction fees.
   - Prevent permanent locking of staked funds.
   - Implement robust handling for failed transactions to ensure proper fee deduction.

By addressing these key areas, EIP-4337-based account abstraction wallets can achieve enhanced security and usability.

---

## 5. Strong Password Policy, Two-Factor Authentication (2FA), or PIN Code to Access Sensitive Information

**Overview**

Many desktop wallets use flawed file encryption methods, making it easier for attackers to access and decrypt users' encrypted data. Specifically, these wallets do not bind the file encryption process to the hardware of the device. As a result, even if a file is encrypted, an attacker can transfer it to another device for offline decryption, bypassing the device's built-in security measures.

**Example**

Some wallets that use PIN codes have weaknesses in their brute-force protection algorithms. The encryption algorithms used by these wallets have significantly fewer hash iterations than industry standards. For example, OWASP recommends 600,000 iterations for password protection, Apple’s backup keychain uses 10 million iterations, and services like 1Password and LastPass use 650,000 and 600,000 iterations, respectively. However, some desktop wallets use only 5,000 iterations, far below these security benchmarks. Even relatively complex PINs can be easily cracked by attackers using brute-force techniques, allowing them to access the user’s sensitive data.

Furthermore, some wallet software’s password protection mechanisms fail to withstand complex attacks. Malicious attackers often first steal encrypted data and then use powerful computational resources to decrypt it offline. Due to flaws in the chosen encryption algorithms, these wallets’ encrypted data is more vulnerable to cracking. This not only puts users' digital assets at risk but also introduces new security challenges for the entire Web3 ecosystem.

**Solutions**

When using desktop wallets, it is recommended to choose **MPC wallets** or **hardware wallets** instead. Compared to mobile devices, desktop systems have inherent security vulnerabilities. Although desktop wallets are convenient, their constant internet connectivity makes them more susceptible to hacking and malware. Therefore, for users with high security requirements, choosing alternative wallet types is a more cautious approach.

---

## 6. Weak Private Key Vulnerability Due to Insufficient Randomness During Key Generation

**Overview**

This issue is a **cryptographic vulnerability** specifically caused by **insufficient randomness during private key generation**, leading to **weak private keys**. The root cause lies in the pseudo-random number generator (PRNG) used in the key generation algorithm, which fails to provide sufficient randomness, making the resulting private keys predictable. This is a **cryptographic algorithm implementation flaw** resulting from a faulty or insecure random number generation mechanism, which compromises the integrity of private key generation and significantly reduces account security.

**Example**

When using the **Profanity tool** to generate wallet addresses, the private key generation process relies on a non-fully random seed. Profanity uses `random_device` and `mt19937_64`, but `mt19937_64` is a deterministic pseudo-random number generator that depends on the initial seed, leading to a reduced key space. This makes it easier for attackers to brute-force the private key.

Due to the insufficient randomness in key generation, attackers can speed up private key recovery by calculating `SeedPrivateKey*G = PublicKey - Iterator*G`. This method reduces the search space, allowing attackers to recover the private key after only about 2³² computations, a relatively low amount of effort, especially with modern computational resources like GPUs. In a matter of minutes, attackers could potentially recover the private key.

**Solutions**

Private keys should be generated using high-quality random number generators, such as hardware-based RNGs, to ensure the integrity and security of the key space. Unfortunately, the Profanity tool’s method does not meet this standard, resulting in the **unsafe generation of private keys**.

---

## 7. Replay Attacks

**Overview**

The lack of protection against replay attacks is primarily due to the following reasons:

1. Missing randomness verification mechanisms.
2. Key context data (such as pairing topics) is absent in the signed payload.

Short-term suggestions include adding timestamps and pairing topics, while long-term improvements should focus on enhancing signature design, with an emphasis on using randomness verification for increased security.

**Example**

The **WalletConnect v2** protocol allows applications and wallets to exchange encrypted and authenticated messages through a public WebSocket relay server. Although the relay server cannot access the encryption keys, dynamic testing during audits revealed that the protocol lacks protection against **replay attacks**.

The WalletConnect authentication protocol is a challenge-response protocol between the user and server, with the user signing the message with their private key. The message includes a **random number (nonce)** generated by the server, which should theoretically prevent attackers from replaying old signed messages to spoof authentication.

However, testing showed:

1. **Lack of nonce validation**: Besides checking for the presence of a nonce, the protocol does not verify its correctness, meaning repeated signatures are accepted.
2. **No pairing topic included in the signature**: The signed message payload does not include the pairing topic between the user and server. If an attacker steals a signature, they can apply it to a new pairing, impersonating the user.

Example of a message sent over WalletConnect:

```json
{
  "id": 1680643717702847,
  "jsonrpc": "2.0",
  "method": "irn_publish",
  "params": {
    "topic": "42507dee006fe8(...)2d797cccf8c71fa9de4",
    "message": "AFv70BclFEn6MteTRFemaxD7Q7(...)y/eAPv3ETRHL0x86cJ6iflkIww",
    "ttl": 300,
    "prompt": true,
    "tag": 1108
  }
}
```

**Solutions**

**Short-term**: Include **timestamps** in the signed payload and verify that the timestamp aligns with the current time (within a reasonable window). Additionally, include the **pairing topic** in the signed message payload to bind the signature to the specific pairing relationship, preventing cross-pairing replay attacks.

**Long-term**: Include all relevant pairing and authentication data (such as sender and receiver public keys) in the signed payload. If possible, prioritize using **random numbers (nonces)** instead of timestamps. Nonces are better suited to prevent replay attacks, but given WalletConnect's distributed architecture, implementing nonce verification may be more complex.

---

## 8. Key Generation Vulnerability Due to Lack of `rejectZero` Option

**Overview**

The key derivation implementation uses the **x25519** library but does not enable the `rejectZero` option. If the counterparty is controlled by an attacker, this could lead to the generation of a shared key that is all zeroes. In such a case, the attacker can observe or tamper with the communication.

**Example**

An attacker can compromise a hosted dApp's Web server and inject malicious code to always provide low-order points during key exchanges. When a wallet connected via WalletConnect initiates the connection, the derived key will be zero, allowing the attacker to passively capture and read the exchanged messages.

Example code:

```jsx
export function deriveSymKey(privateKeyA: string, publicKeyB: string): string {
  const sharedKey = x25519.sharedKey(
    fromString(privateKeyA, BASE16),
    fromString(publicKeyB, BASE16)
  );
  const hkdf = new HKDF(SHA256, sharedKey);
  const symKey = hkdf.expand(KEY_LENGTH);
  return toString(symKey, BASE16);
}
```

**Solutions**

- **Short-term**: Enable the `rejectZero` flag in the `deriveSymKey` function to prevent the generation of all-zero keys.
- **Long-term**: When using cryptographic primitives, study possible edge cases and always review implementation details. Follow best practices and include deep defense mechanisms to ensure protocols work as expected.

---

## 9. Leakage of Wallet Private Keys and Mnemonic Phrases

**Overview**

Creating wallets on public platforms presents a significant **security risk**. Due to the public nature of these platforms, anyone can easily access files containing sensitive information. Users who create wallets on public platforms should immediately transfer their assets and delete any sensitive files. When using unfamiliar network platforms to generate wallets or mnemonics, users must be highly cautious. To mitigate the risk of data leakage, it is recommended to use wallets that have undergone strict security audits and come from reputable sources.

**Example**

When creating wallets using the **Atomicals protocol** on **Replit**, there is a risk of **mnemonic leakage**, which could lead to stolen crypto assets.

Replit is a popular online programming platform that allows users to write code in various programming languages directly in the browser. When creating wallets with the **atomicals-js** library (a JavaScript tool for the Atomicals protocol) on Replit, the platform generates a file, `wallet.json`, containing sensitive information such as mnemonics, private keys, and addresses. Since Replit is a public platform, anyone can access these files, potentially leading to a leak of sensitive data.

Attackers can use **Google Hacking** or simple search techniques to locate publicly deployed instances of the atomicals-js project and obtain these sensitive `wallet.json` files, allowing them to access and steal users' assets.

**Solutions**

Users should avoid using public and easily accessible platforms to create cryptocurrency wallets. Especially when generating private keys or mnemonic phrases, it is crucial to use secure and reliable environments.

---

## 10. Incorrect Data Type Conversion Leading to Excessive Gas Fees

**Overview**

The `ethjs-util` library’s `intToBuffer` function does not support floating-point numbers. When building transactions, if the fee is a floating-point number, an error occurs during conversion to an integer. This data type conversion issue results in incorrect fee calculations and can lead to extremely high gas fees

.

**Example**

In **ethereumjs**, the fee parameters `maxPriorityFeePerGas` and `maxFeePerGas` are processed through the `toBuffer` function, which internally calls `ethjs-util`'s `intToBuffer`. Here’s what happens:

1. The integer is converted to hexadecimal.
2. It checks if padding is necessary to make the length even.

If the input is a floating-point number (e.g., `33974229950.550003`), the decimal portion is discarded during processing, leading to an error in the calculated transaction fees.

In browsers, when handling this floating-point data, the decimal is “eaten” by the `parseInt` function, causing an incorrect conversion. In Node.js, the data is truncated, resulting in smaller (but still incorrect) values for the fee.

**Solutions**

The `ethjs-util` library's `intToBuffer` function does not check for data types. Floating-point data should first be type-checked to ensure it’s an expected integer, and an exception should be thrown if the data type is incorrect, instead of continuing with the conversion.

## Conclusion

In the rapidly evolving world of Web3, wallet security remains one of the most critical aspects for protecting digital assets.

Vulnerabilities such as improper data storage, domain validation flaws, EIP-4337 failures, supply chain threats, weak PIN code hashing, insufficient private key protection, and high gas fees can expose users to significant risks.

To address these issues, implementing stronger encryption and hashing algorithms, ensuring secure private key generation, improving signature protocols, using private platforms for wallet creation, and conducting regular security audits are essential steps toward enhancing wallet security and safeguarding digital assets.

<div  align="center"> 
<img src="img/02_audit_wallet" width="50%" />
</div>
