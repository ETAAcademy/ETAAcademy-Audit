# ETAAcademy-Adudit: 3. Solana Security

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>03. Solana Security</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>Solana Security</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# Common Security Vulnerabilities in Solana Development

Solana smart contracts, typically written in Rust and developed with the Anchor framework, prioritize security through automated safety checks, account validation, and efficient data handling. Anchor simplifies CPI, oracles, and scalability, while Solana's features like versioned transactions, LUTs, cNFTs, and Solana Pay optimize security, performance, and integration.

---

Solana On-Chain Programs (Smart Contracts) are typically written in Rust and use the Anchor framework to simplify development. Anchor provides automated safety checks, data handling, and account validation, making it easier to build secure and maintainable smart contracts. Each Solana program has a unique Program ID (or program address), and the program handles client transaction instructions via an instruction processor. Data is stored at Program Derived Addresses (PDAs), which are unique and deterministic, ensuring that both the program and client can consistently locate the data storage.

Solana programs written and deployed in native Rust have an entry point defined using the `entrypoint!` macro. The program processes instructions using `program_id`, `accounts`, and `instruction_data`. Solana program accounts are stateless, meaning the program logic resides in the program account, while the state is stored in separate data accounts.

**Anchor Framework**

Anchor simplifies the development process by using **macros** and **traits**. The **`declare_id!`** macro declares the program's ID, and the **`#[program]`** macro defines instruction handlers. **Context types** provide context for instructions, including necessary accounts and metadata for execution. The **`#[account]`** macro is used for account declaration and initialization, while **`#[derive(Accounts)]`** generates account validation structures. Anchor offers a clear program structure and automated security checks, ensuring the program's safety and maintainability.

Anchor also provides a client-side tool to interact with programs. Using IDL (Interface Definition Language) files, it defines the structure of the program, including instructions, accounts, and types, helping clients interact seamlessly with Solana programs. By creating a `Provider` object, developers can connect their wallet to the Solana network and use the `Program` object to invoke instructions and send transactions. The `MethodsBuilder` simplifies the process of invoking program instructions, and Anchor offers features for transaction construction and account querying, making interactions with Solana programs more efficient and convenient.

**Cross-Program Invocation (CPI)**

Solana allows programs to invoke instructions from other programs via **Cross-Program Invocation (CPI)**. Anchor simplifies this process using the `CpiContext`, which automatically generates helper functions for invoking instructions from other Anchor programs. Developers can also manually invoke other programs using `invoke` and `invoke_signed` or define custom error handling using the `error_code` macro for clearer error management and reporting within the program.

**Oracles and Data Feeds**

Solana supports several decentralized oracle providers, such as Pyth, Switchboard, Chainlink, and DIA, which offer various types of data feeds with varying degrees of decentralization. Switchboard, for instance, enhances security using Trusted Execution Environments (TEEs), which execute sensitive code in isolated secure environments, ensuring the integrity and accuracy of data feeds. These oracles aggregate data from external sources via multiple tasks (Jobs) and use weighted medians to produce the final result, stored in an Aggregator account. Switchboard also uses a staking mechanism to incentivize accurate data updates and penalize oracle nodes for inaccuracies. Additionally, Switchboard provides Verifiable Random Functions (VRFs) for generating secure and transparent random numbers, which can be used in applications requiring randomness.

**Token Program and Extensions**

The native Token program in Solana supports most fungible and non-fungible tokens (NFTs). However, to address scalability issues with the original token program, Solana introduced the Token Extensions Program (also known as Token-2022). This extension provides additional functionalities, such as immutable ownership, transaction fees, and non-transferable tokens. The extension is compatible with the original Token program but operates with its own address and features, preventing fragmentation within the ecosystem. Developers can choose the appropriate extension for their needs, allowing for more customized token management. Client-side applications must distinguish between the two programs, as they offer different functionalities.

**Optimization in Solana Development**

Optimization is crucial in Solana development due to resource constraints, such as transaction fees based on storage usage and limits on stack and heap sizes. Large data structures can be stored on the heap using `Box` to avoid stack overflow issues. For accounts exceeding 10MB, developers can use **Zero-Copy** techniques to directly manipulate raw data, bypassing stack and heap limitations, improving efficiency. The order of fields in data structures is also important for query performance, with variable-length fields best placed at the end of the structure to optimize access. While Solana does not natively support environment variables, developers can mimic this functionality using Rust's feature flags, `cfg` attributes, `cfg!` macros, and administrative program accounts.

**Transaction Versions and Address Lookup Tables (LUTs)**

Solana supports **Versioned Transactions**, which can use both legacy and new formats. The new transaction format (since version 0) introduces Address Lookup Tables (LUTs) to optimize address referencing, reducing transaction size and enabling more complex transactions. Versioned transactions support both the traditional and new formats, with the new format using a 1-byte index to reference account addresses, significantly reducing transaction size. Solana also provides functionalities to create, extend, deactivate, freeze, and close lookup tables, allowing for more efficient management of multiple accounts in transactions. The `@solana/web3.js` library provides tools for working with these tables.

**Compressed NFTs (cNFTs)**

Compressed NFTs (cNFTs) leverage Solana’s state compression technology and concurrent Merkle tree structures to store NFT data on-chain after hashing, significantly reducing storage costs. Compared to traditional NFTs, cNFTs can save over 1000 times in minting and storage fees. Developers can use the Metaplex Bubblegum program and Umi framework to create and manage cNFT collections, including initializing Merkle trees, setting metadata, and minting cNFTs using the `MintV1` instruction. Solana's state compression is not limited to NFTs; it can also be applied to data storage in any Solana program, greatly enhancing storage efficiency and enabling concurrent transactions in a single block time.

**Solana Mobile Wallet Adapter (MWA)**

The Solana Mobile Wallet Adapter (MWA) connects mobile applications and wallets to the Solana blockchain via WebSocket. Using React Native, developers can build Android apps that leverage MWA for wallet connectivity, transaction signing, and submission. The Solana mobile stack simplifies mobile app development and supports use cases such as mobile banking, micro-payments in gaming, and e-commerce. Although challenges such as regulatory constraints and platform fees exist, Solana's app store allows developers to bypass these issues and deploy decentralized apps, opening new opportunities in DeFi, gaming, and e-commerce.

**Durable Transactions**

**Durable Transactions** solve the issue of transaction expiration by using a nonce account instead of the block hash in a typical transaction. This allows transactions to be stored after signing and submitted at a later time. Each durable transaction must include a `nonceAdvance` instruction to update the nonce value and ensure the transaction’s uniqueness. Durable transactions are suitable for scheduled transactions, multi-signature wallets, and cross-chain interactions. However, they also present risks, particularly with malicious transactions. Developers must educate users to only sign transactions from trusted sources and implement security measures, such as using cold wallets for large amounts and regularly auditing transaction records.

**Solana Pay**

Solana Pay is a standard for encoding Solana transactions in URLs, enabling unified payment requests and interactive transaction requests. It allows for Solana's native signature functionality to be used in innovative scenarios, such as transfers, transaction gating, and partial signatures. Transfer requests are initiated via simple URLs supporting both SOL and SPL token transfers. Transaction requests allow users to fetch transaction details and sign with their wallets. Developers can build APIs to create and return serialized transactions, enabling conditional transactions, such as those gated by specific NFTs. Solana Pay also integrates with QR codes, streamlining the payment process.

## Solana Security

Solana development involves various security considerations to ensure the integrity and security of programs (smart contracts) deployed on the network.

### 1. **Signature Checks**

**Signature checks** are critical in ensuring that only authorized accounts can perform certain actions in blockchain development. In Anchor, the `Signer` type or the `#[account(signer)]` constraint can be used to simplify signature verification logic. In more complex cases, developers can manually check the `is_signer` attribute within instruction handlers.

#### 1) Signature Check in Anchor

##### **Using the Signer Type**

In Anchor, you can define an account as a `Signer`, which automatically handles signature verification. For example:

<details><summary>POC</summary>

```rust
pub struct UpdateAuthority<'info> {
    pub authority: Signer<'info>,  // Automatically validates the signature
}
```

</details>

When the account is of the `Signer` type, Anchor automatically checks whether the account has signed the transaction, eliminating the need for manual signature validation.

##### **Using the `#[account(signer)]` Constraint**

The `#[account(signer)]` constraint is a more flexible approach, allowing the signature check and access to the account’s data simultaneously. This is useful for both verifying signatures and securely interacting with account data. For example:

<details><summary>POC</summary>

```rust
#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(signer)]
    pub authority: Account<'info, AuthState>,  // Verifies the signature and allows access to the AuthState data
}
```

</details>

#### 2) Signature Check in Native Rust

In native Rust (without Anchor), developers need to manually check the `is_signer` attribute of an account. If the account hasn't signed the transaction, the program should return the `MissingRequiredSignature` error to prevent further action:

<details><summary>POC</summary>

```rust
if !ctx.accounts.authority.is_signer {
    return Err(ProgramError::MissingRequiredSignature.into());
}
```

</details>

### 2. **Owner Checks**

**Owner checks** are crucial to ensure that the `owner` field of an account matches the program’s `program_id`. Without proper owner validation, attackers could exploit accounts owned by other programs to forge or tamper with operations, leading to significant security vulnerabilities.

#### **Manual Owner Check**

A simple manual check can be implemented within the program to verify the account's owner matches the program's `program_id`:

<details><summary>POC</summary>

```rust
if ctx.accounts.account.owner != ctx.program_id {
    return Err(ProgramError::IncorrectProgramId.into());
}
```

</details>

#### **Anchor Automatic Owner Check**

Anchor provides automatic owner validation using the `Account` type, ensuring that the account's owner matches the current program:

<details><summary>POC</summary>

```rust
use anchor_lang::prelude::*;

declare_id!("Cft4eTTrt4sJU4Ar35rUQHx6PSXfJju3dixmvApzhWws"); // Program ID

#[derive(Accounts)]
pub struct Checked<'info> {
    #[account(
        has_one = admin, // Ensures the admin field in the admin_config account matches
    )]
    admin_config: Account<'info, AdminConfig>, // Automatically checks if the owner of admin_config matches the current program
    admin: Signer<'info>,  // Admin account performing the instruction
}
```

</details>

#### **Using `#[account(owner = <expr>)]`**

Sometimes, an account’s owner may not be the current program but another program’s derived address (PDA). In such cases, the `owner` constraint can be used to validate ownership:

<details><summary>POC</summary>

```rust
#[derive(Accounts)]
pub struct Checked<'info> {
    #[account(
        has_one = admin, // Checks that the admin field in the admin_config matches the provided admin account
    )]
    admin_config: Account<'info, AdminConfig>,
    admin: Signer<'info>,
    #[account(
        seeds = b"test-seed", // Uses a seed to derive the PDA
        bump, // Verifies the validity of the PDA using bump
        owner = token_program.key() // Ensures the account is controlled by token_program
    )]
    pda_derived_from_another_program: AccountInfo<'info>,  // PDA controlled by another program (Token program)
    token_program: Program<'info, Token>,  // Token program
}
```

</details>

### 3. **Account Data Matching**

Account data matching ensures that malicious users cannot alter account data or perform unauthorized operations, such as updating admin information or withdrawing funds without authorization.

#### **1) Manual Account Data Verification (Rust)**

In Rust, you can explicitly verify account data by checking whether the caller is the current admin. For example:

<details><summary>POC</summary>

```rust
if ctx.accounts.admin.key() != ctx.accounts.admin_config.admin {
    return Err(ProgramError::InvalidAccountData.into());
}
```

</details>

#### **2) Anchor Account Data Verification**

Anchor simplifies account data verification using the `has_one` or `constraint` attributes. This approach eliminates the need for additional manual code, making the logic more concise and clear.

##### **Using `has_one`**

The `has_one` constraint ensures that the field in `admin_config` (e.g., the `admin` field) matches the provided `admin` account’s public key:

<details><summary>POC</summary>

```rust
#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    #[account(
        mut,
        has_one = admin // Verifies that the admin field in admin_config matches the admin account
    )]
    pub admin_config: Account<'info, AdminConfig>, // The account being updated
    #[account(mut)]
    pub admin: Signer<'info>, // Admin account performing the instruction
    /// CHECK: This account will not be checked by Anchor
    pub new_admin: UncheckedAccount<'info>, // New admin account
}
```

</details>

##### **Using `constraint` for Complex Logic**

Anchor allows developers to use the `constraint` attribute for more complex validation logic. For example, verifying that the `admin` field in `admin_config` matches the public key of the provided `admin` account:

<details><summary>POC</summary>

```rust
#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    #[account(
        mut,
        constraint = admin_config.admin == admin.key() // Manual validation of the admin field
    )]
    pub admin_config: Account<'info, AdminConfig>, // The account being updated
    #[account(mut)]
    pub admin: Signer<'info>, // Admin account performing the instruction
    /// CHECK: This account will not be checked by Anchor
    pub new_admin: UncheckedAccount<'info>, // New admin account
}
```

</details>

### 4. **Initialization Checks**

In Solana programs, account initialization is crucial for allocating space and data for accounts. If proper checks are not performed, an attacker might be able to reinitialize an existing account, tampering with critical account fields (like `authority`).

1. **Add Initialization Checks**

By adding an `is_initialized` field in the account structure, you can check whether the account has already been initialized. If it has, reinitialization is prevented:

<details><summary>POC</summary>

```rust
pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let user = &mut ctx.accounts.user;

    if user.is_initialized {
        return Err(ProgramError::AccountAlreadyInitialized.into()); // Throw an error if already initialized
    }

    user.is_initialized = true;
    user.authority = ctx.accounts.authority.key();
    Ok(())
}
```

</details>

2. **Using Anchor’s `init` Constraint**

In Anchor, the `#[account(init)]` constraint is used to initialize an account and allocate space for it. The discriminator ensures that the account can only be initialized once, thus avoiding overwriting existing data. The constraint requires both the **payer** and **space** to be specified:

<details><summary>POC</summary>

```rust
const DISCRIMINATOR_SIZE: usize = 8;  // Size of the discriminator

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,                             // Initialize the account
        payer = authority,                // Payer of the initialization fees
        space = DISCRIMINATOR_SIZE + User::INIT_SPACE   // Space allocated for the account
    )]
    pub user: Account<'info, User>,     // Account to be initialized
    #[account(mut)]
    pub authority: Signer<'info>,        // Signing account
    pub system_program: Program<'info, System>,  // System program
}
```

</details>

3. **Caution with `init_if_needed`**

The `init_if_needed` constraint automatically initializes an account if it hasn’t been initialized yet. While useful for multiple scenarios, it can be risky if not handled carefully, as it may unintentionally reset account data. Always perform additional checks to prevent data overwriting:

<details><summary>POC</summary>

```rust
#[derive(Accounts)]
pub struct InitializeWithCheck<'info> {
    #[account(
        init_if_needed,
        payer = authority,
        space = DISCRIMINATOR_SIZE + User::INIT_SPACE
    )]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

</details>

### 5. **Duplicate Mutable Accounts**

In Solana programs, when a single instruction requires two mutable accounts of the same type, attackers can potentially pass the same account twice, leading to unauthorized changes in account states. This type of vulnerability can be exploited to maliciously manipulate data or cause security breaches.

To prevent attackers from passing the same mutable account twice, the program should explicitly check the public keys of the accounts involved. If the public keys of the two accounts are identical, an error should be thrown to stop the execution of the program. This check ensures that the program is not manipulated by attackers trying to use the same account for malicious actions.

#### 1) Checking Duplicate Accounts in Rust

You can add a simple check in the program logic to compare the public keys of two accounts. If they match, return an error.

<details><summary>POC</summary>

```rust
if ctx.accounts.user_a.key() == ctx.accounts.user_b.key() {
    return Err(ProgramError::InvalidArgument);
}
```

</details>

#### 2) Using Anchor's `constraint` Attribute

In Anchor, you can enforce account validation using the `#[account(...)]` attribute with a `constraint`. For example, you can ensure that two accounts are distinct by checking their keys:

<details><summary>POC</summary>

```rust
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        mut, // Both accounts are mutable
        constraint = user_a.key() != user_b.key()
    )]
    pub user_a: Account<'info, User>,
    #[account(mut)]
    pub user_b: Account<'info, User>,
}
```

</details>

This method performs the account validation at compile-time, automatically checking if the accounts are identical and throwing an error if validation fails.

### 6. **Type Cosplay**

**Type Cosplay Vulnerability** occurs when a program mistakenly uses an account's data in an unintended manner due to the lack of proper type differentiation. In Solana, accounts store data as byte arrays, which are deserialized into custom account types. If these types are not properly verified, a program could mistakenly treat an ordinary user account as an administrator account, leading to security risks such as privilege escalation or data corruption.

1. **Account Discriminant**: Anchor provides an 8-byte discriminator that uniquely identifies different account types. Anchor automatically checks this discriminator when deserializing account data to ensure it matches the expected type, improving security and reducing the chances of type errors.
2. **Anchor's Automatic Handling**: Anchor automatically inserts and verifies account discriminators, reducing the developer's burden and increasing security by ensuring that the account type is always correct.

3. **Using `#[account]`**: This attribute automatically adds a discriminator to accounts and ensures that the correct account type is used, reducing the risk of type mismatches.

#### Rust Enum Discriminant and Anchor Account Discriminant

- **Rust Enum Discriminant**: In Rust, an enum discriminant is used to represent the internal value that identifies the specific variant of an enum. You can add a `discriminant` field in the account structure to signify the account type. Every time account data is deserialized, the program can check the value of this field to ensure it matches the expected type.

<details><summary>POC</summary>

```rust
#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
    discriminant: AccountDiscriminant, // A field to identify account type
    user: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AccountDiscriminant {
    User, // User type
    Admin, // Admin type
}
```

</details>

When the program reads and deserializes account data, it checks the `discriminant` field to verify that the account data matches the expected type. For example, if the `discriminant` is not the expected `User` variant, the program can return an error.

<details><summary>POC</summary>

```rust
if user.discriminant != AccountDiscriminant::User {
    return Err(ProgramError::InvalidAccountData.into());
}
```

</details>

- **Anchor Account Discriminant**: In Anchor, when you declare an account as `Account<'info, T>`, Anchor automatically handles the discriminator for you. This simplifies the process, as you no longer need to manually manage or verify discriminators.

<details><summary>POC</summary>

```rust
#[derive(Accounts)]
pub struct AdminInstruction<'info> {
    #[account(has_one = admin)]
    admin_config: Account<'info, AdminConfig>, // Account type is automatically verified
    admin: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct AdminConfig {
    admin: Pubkey, // Field for the account
}

#[account]
#[derive(InitSpace)]
pub struct UserConfig {
    user: Pubkey, // User account field
}
```

</details>

### 7. **Cross-Program Invocation (CPI)**

**Cross-Program Invocation (CPI)** allows one program to call the instructions of another program, leveraging its functionality. However, **arbitrary CPI vulnerabilities** arise when the target program's ID is not properly validated, allowing an attacker to pass the ID of a malicious program, which may execute unsafe operations. This could lead to severe security issues, including unauthorized access and manipulation of funds.

<details><summary>POC</summary>

```rust
#[derive(Accounts)]
pub struct Cpi<'info> {
    source: UncheckedAccount<'info>,
    destination: UncheckedAccount<'info>,
    authority: UncheckedAccount<'info>,
    token_program: UncheckedAccount<'info>, // No check for the token program ID
}
```

</details>

To prevent this vulnerability, always verify that the target program ID is the expected one before performing the CPI. For example, ensure that the `token_program` matches the expected SPL Token program ID:

<details><summary>POC</summary>

```rust
pub fn cpi_secure(ctx: Context<Cpi>, amount: u64) -> ProgramResult {
    // Ensure token_program is the SPL Token program
    if &spl_token::ID != ctx.accounts.token_program.key {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Proceed with CPI if the program ID is correct
    solana_program::program::invoke(
        &spl_token::instruction::transfer(
            ctx.accounts.token_program.key,
            ctx.accounts.source.key,
            ctx.accounts.destination.key,
            ctx.accounts.authority.key,
            &[],
            amount,
        )?,
        &[
            ctx.accounts.source.clone(),
            ctx.accounts.destination.clone(),
            ctx.accounts.authority.clone(),
        ],
    )
}
```

</details>

#### Using Anchor's CPI Module

Anchor's CPI module automatically handles program ID validation, reducing the risk of errors or omissions. With Anchor, you can easily perform CPI with built-in safety checks for the target program ID.

<details><summary>POC</summary>

```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod arbitrary_cpi_recommended {
    use super::*;

    pub fn cpi(ctx: Context<Cpi>, amount: u64) -> ProgramResult {
        // Use the token::transfer module to perform CPI
        token::transfer(ctx.accounts.transfer_ctx(), amount)
    }
}

#[derive(Accounts)]
pub struct Cpi<'info> {
    source: Account<'info, TokenAccount>, // Source account
    destination: Account<'info, TokenAccount>, // Destination account
    authority: Signer<'info>, // Authority signer
    token_program: Program<'info, Token>, // SPL Token program
}

impl<'info> Cpi<'info> {
    // Create the transfer_ctx context
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let program = self.token_program.to_account_info(); // Get program account info
        let accounts = token::Transfer {
            from: self.source.to_account_info(),
            to: self.destination.to_account_info(),
            authority: self.authority.to_account_info(),
        };

        // Return the CPI context
        CpiContext::new(program, accounts)
    }
}
```

</details>

By using Anchor's CPI module, you can ensure that program ID validation is handled automatically, which reduces the chances of malicious exploitation and increases the security of your program.

### 8. **Program Derived Addresses (PDA) and Canonical Bump in Solana**

In Solana, **Program Derived Addresses (PDAs)** are deterministically generated addresses derived from seeds, typically used to uniquely identify accounts associated with a program. Solana provides two primary methods for generating PDAs: `create_program_address` and `find_program_address`. Although they both derive addresses from seeds, their handling of **bump values** differs significantly, and understanding these differences is crucial for ensuring program security.

#### `create_program_address` vs `find_program_address`

- **`create_program_address`**: This method allows you to specify any bump value when generating a PDA. While flexible, it can introduce security vulnerabilities. Since the bump value is user-defined, an attacker may generate multiple addresses by changing the bump value, potentially bypassing restrictions such as reward limits or other security checks.
- **`find_program_address`**: This method automatically selects a canonical bump value, ensuring that the derived address is consistent every time it is called. This makes it more secure, as it avoids the risks associated with manually specifying bump values and guarantees the same PDA address for a given seed.

Using `create_program_address` allows users to specify their own bump value. An attacker could exploit this by passing different bump values to generate multiple PDAs, thus bypassing restrictions like reward limits.

<details><summary>POC</summary>

```rust
pub fn set_value(ctx: Context<BumpSeed>, key: u64, new_value: u64, bump: u8) -> Result<()> {
    let address = Pubkey::create_program_address(&[key.to_le_bytes().as_ref(), &[bump]], ctx.program_id)
        .unwrap();
    if address != ctx.accounts.data.key() {
        return Err(ProgramError::InvalidArgument.into());
    }
    ctx.accounts.data.value = new_value;
    Ok(())
}
```

</details>

To ensure consistency and security, it’s recommended to use `find_program_address`. This method guarantees the use of a canonical bump and prevents the use of different bumps to generate multiple addresses. Additionally, it is advisable to store the bump value in the account to avoid recalculating it every time, improving efficiency.

<details><summary>POC</summary>

```rust
pub fn set_value_secure(ctx: Context<BumpSeed>, key: u64, new_value: u64) -> Result<()> {
    let (address, expected_bump) = Pubkey::find_program_address(&[key.to_le_bytes().as_ref()], ctx.program_id);
    if address != ctx.accounts.data.key() {
        return Err(ProgramError::InvalidArgument.into());
    }
    if expected_bump != ctx.accounts.data.bump {
        return Err(ProgramError::InvalidArgument.into());
    }
    ctx.accounts.data.value = new_value;
    Ok(())
}
```

</details>

In **Anchor**, you can manage PDAs using the `seeds` and `bump` attributes, which ensure the correct bump is used without introducing security vulnerabilities. By specifying `bump = <some_bump>`, you can control which bump value to use, but Anchor will always use `find_program_address` to ensure the use of a canonical bump.

<details><summary>POC</summary>

```rust
#[derive(Accounts)]
pub struct VerifyAddress<'info> {
    #[account(
        seeds = [key.to_le_bytes().as_ref()],
        bump = data.bump  // Use the stored canonical bump
    )]
    pub data: Account<'info, Data>,
}
```

</details>

#### Security Vulnerability Example and Fix

By addressing unsafe PDA derivation issues, programs can prevent attackers from using multiple bump values to derive different PDAs and claim rewards multiple times. By using `find_program_address` and storing the canonical bump, programs ensure that each user can claim rewards only once.

<details><summary>POC</summary>

```rust
pub fn create_user_secure(ctx: Context<CreateUserSecure>) -> Result<()> {
    ctx.accounts.user.set_inner(UserSecure {
        auth: ctx.accounts.payer.key(),
        bump: ctx.bumps.user,
        rewards_claimed: false,
    });
    Ok(())
}

pub fn claim_secure(ctx: Context<SecureClaim>) -> Result<()> {
    // Check if the canonical bump matches
    ctx.accounts.user.rewards_claimed = true;
    Ok(())
}
```

</details>

### 9. **Rebirth Attack on Closed Accounts**

In Solana, when an account is closed, if not handled properly, an attacker could prevent the account from being garbage collected by refunding the rent in Lamports, effectively "reviving" the account and using it again. To prevent such **rebirth attacks**, the program must ensure that the account is fully cleaned before being closed, and a special identifier (such as `CLOSED_ACCOUNT_DISCRIMINATOR`) is set to mark the account as closed.

1. **Properly Closing Accounts**: This involves transferring the Lamports from the account, clearing the account’s data, and setting a close identifier to ensure the account is securely closed.

<details><summary>POC</summary>

```rust
#[program]
pub mod closing_accounts_secure {
    use super::*;

    pub fn close(ctx: Context<Close>) -> ProgramResult {
        let account = ctx.accounts.account.to_account_info();
        let dest_starting_lamports = ctx.accounts.destination.lamports();

        **ctx.accounts.destination.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(account.lamports())
            .unwrap();
        **account.lamports.borrow_mut() = 0;

        let mut data = account.try_borrow_mut_data()?;
        for byte in data.deref_mut().iter_mut() {
            *byte = 0; // Clear data
        }

        let dst: &mut [u8] = &mut data;
        let mut cursor = std::io::Cursor::new(dst);
        cursor
            .write_all(&anchor_lang::__private::CLOSED_ACCOUNT_DISCRIMINATOR)
            .unwrap(); // Set close identifier

        Ok(())
    }
}
```

</details>

2. **Additional Protection with `force_defund` Instruction**: This instruction ensures that if an account has been revived or tampered with, it is forcibly defunded, preventing further use.

<details><summary>POC</summary>

```rust
pub fn force_defund(ctx: Context<ForceDefund>) -> ProgramResult {
    let account = &ctx.accounts.account;

    let data = account.try_borrow_data()?;
    assert!(data.len() > 8);

    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&data[0..8]);
    if discriminator != CLOSED_ACCOUNT_DISCRIMINATOR {
        return Err(ProgramError::InvalidAccountData);
    }

    let dest_starting_lamports = ctx.accounts.destination.lamports();
    **ctx.accounts.destination.lamports.borrow_mut() = dest_starting_lamports
        .checked_add(account.lamports())
        .unwrap();
    **account.lamports.borrow_mut() = 0;

    Ok(())
}
```

</details>

3. **Using Anchor's `close` Constraint**: The Anchor framework simplifies the account closing process by automating the steps of transferring Lamports, clearing data, and setting the closed identifier. This reduces the chances of vulnerabilities due to improper account closure.

<details><summary>POC</summary>

```rust
#[derive(Accounts)]
pub struct CloseAccount {
    #[account(mut, close = receiver)]
    pub data_account: Account<'info, MyData>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>
}

```

</details>

---

[Sealevel-attacks-master](https://github.com/ETAAcademy/ETAAcademy-Audit/tree/main/Articles/Appendix/sealevel-attacks-master)

<div  align="center"> 
<img src="img/03_solana_security.gif" width="50%" />
</div>
