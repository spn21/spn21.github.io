+++
title = "一个sol的\"旅程\""
date = 2026-05-23

[extra]
cover_image = "/covers/12/0.jpg"
cover_sentence = " "
tags =  ["Solana","Security", "Note"]
draft = false
+++


当我们用 Phantom 给另一个地址发送 SOL，本质上不是"发送一枚代币对象"，而是在 Solana 上提交了一笔包含 System Program transfer 指令的交易，本质是让 Solana 网络把账户里的 lamports 数字减少、对方账户里的 lamports 数字增加

(lamport 是 Solana 网络的最小交易单元)
1 SOL == 1,000,000,000 lamports

（Phantom 中的普通钱包地址一般由 Ed25519 keypair 生成的 publickey 的 base58 编码, 生成路径一般为:
    
    强随机数 entropy
        -> BIP39 助记词(mnemonic words)
        -> seed (mnemonic + optional passphrase)
        -> 派生路径 (如 m/44'/501'/0'/0' => seed/ BIP44 / Solana coin type / 第一个账户 / change/index 类字段)
        -> Ed25519 private key
        -> Ed25519 publick key
        -> Solana address (base58 public key)

在 Phantom 转账，大致流程为:

```
Phantom 构建并签名交易
  ↓
RPC 转发给 leader validator
  ↓
validator / Banking Stage 验证、锁账户、加载账户
  ↓
SVM 执行交易里的 instruction
  ↓
调用内建 System Program 的 transfer
  ↓
修改 from/to 两个账户的 lamports
  ↓
post-condition 检查
  ↓
commit 或 rollback
```

## 0x01 Phantom 构建交易


Phantom 会创建一个 Solana transaction, 其顶层字段为:
```rust
pub struct Transaction {
    pub signatures: Vec<Signature>, // 签名数组
    pub message: Message, //交易信息（包括待处理的指令列表)
}

```

每个 ``` signature ``` 都是对序列化的 ```message ```进行 Ed25519 算法签名，由前面账户私钥签署，``` signature ```签名数组中第一个签名属于手续费支付者，同时也作为交易ID，用于在网络上查询该次交易 

Solana 支持两种交易格式: 传统 / 版本化(v0)

validator 通过检查 ```message``` 字段的第一个字节判断格式:

第一个字节设置了版本前缀位，则为版本化

```rust 
    pub const MESSAGE_VERSION_PREFIX: u8 = 0x80;
```
否则位传统消息
(当交易引用了大量账户并接近1232字节数据包限制(PACKET_DATA_SIZE)，最好使用 v0 / 使用地址表查找表时必须采用 v0, 如果所有账户都内联且交易大小未超限，更推荐传统版本)

在签名完成之后， Phantom 会把交易发送给一个 Solana RPC节点，在检查后，会把交易广播道当前或者即将负责出块的 validator，方才进入到 Solana 网络 的交易处理流程

## 0x02 Validator 检查交易

Solana validator 收到交易请求后，会进行:

1. Receive packet (UDP/QUIC)
  
2. Deserialize into VersionedTransaction

3. Sigverify (parallel Ed25519 verification)
    验证 Ed25519(signatures[i], account_keys[i]，message_bytes)

4. Sanitize (structural validation, metadata extraction)
    此阶段验证:
    - 签名数量与 head 中的 num_required_signature 是否匹配
    - program_id_index 和 account_indices 在有效范围中
    - gas fee支付者(账户索引0) 是可写签名者

5. Parse compute budget, calculate fees
    由预算上限和优先级费用计算得出费用细节

6. Check blockhash age (or verify nonce account)
    有效期在 150 个 slot 内

7. Check status cache (dedup)
    避免同一笔签名交易重放

8. Validate nonce authority and advanceability (if nonce transaction)

9. Validate fee payer (load, check balance, deduct fee)

10. Load all accounts (with data size limits)
    同一批次账户对后续交易可见

11. Load programs (verify loaders)
    验证 program account 是否存在且属于有效 loader

12. Execute instructions sequentially
    按照 message 顺序执行

13. Verify post-conditions (lamport balance, rent state)
    所有账户 lamports总和没有发生变化
    没有账户从免租变为需付租金

14. Commit account changes (or rollback on failure)

## 0x03 SVM 执行

普通 SOL transfer 调用的是内建 System Program；如果是用户自定义程序，才会走 sBPF VM / JIT / bytecode 执行这条路径。

SVM 区别于 EVM 的最大特点是并行处理

交易本身提前列出账户，同时锁定交易账户
(如 Alice 需要同时给 Bob 和 Carol 转账，同时执行的话会造成两个线程同时读 Alice 的余额)

SVM 执行，大致流程包括:

- Locate the Program: 从指令的程序 ID 查找程序账户(0x02.11)

{

  以下步骤为调用用户部署的 Solana/ Anchor 以及调用 SPL Token 这类链上程序:

- Load from Cache: 检查程序是否已在程序缓存中进行 JIT 编译

- Provision the VM: 创建具有特定内存区域和计算预算的隔离 sBPF VM 实例

- Execute Bytecode: 使用指令的输入运行程序的入口点(0x02.13 中 执行 System Program 的 transfer 逻辑)
}

- Verify Invariants: 检查执行是否违反任何运行时规则(0x02.14)

- Collect Results: 打包执行结果

## 0x04 交易进入区块并等待确认


当 SVM 成功执行 System Program 的 transfer 指令后，这笔交易并不是立刻就"最终完成"了

对于 Phantom 用户来说，点击 Confirm 之后, Phantom 会先把已经签名的 transaction 发送给 RPC, RPC 接收到交易后，通常会很快返回一个 transaction signature

这个 signature 就是我们在 Solana Explorer 里查询交易的那串 hash

此时 Phantom 看到的是：

- 交易已经发出
- 网络已经接收
- 但还没有最终确认

所以钱包里通常会先显示 pending / processing 一类的状态。

随后，其他 validators 会接收这个区块，重新验证其中的交易，并对这个区块进行投票。

在 Phantom 或 Solana Explorer 里，这几个常见状态大致可以理解为：

processed: 某个节点已经处理了这笔交易
confirmed: 交易已经获得足够多 stake 的确认
finalized: 交易已经达到最终确认，基本不可逆
所以当 Phantom 显示交易成功时，通常表示这笔交易已经被网络处理，并达到某个确认级别。



note:
errors:

| 错误 | 阶段 | 原因 |
| --- | --- | --- |
| AccountInUse | 调度 | 该账户已被同一批次中的其他交易锁定 |
| AccountLoadedTwice | 调度 | 交易中的 pubkey 出现了两次 |
| AccountNotFound | 费用支付人验证 | 费用支付人账户不存在 |
| ProgramAccountNotFound | 账户加载 | 被调用的程序不存在 |
| InsufficientFundsForFee | 费用支付人验证 | 费用支付人无法支付手续费和 rent 免租金最低额 |
| InvalidAccountForFee | 费用支付人验证 | 费用支付人不是系统账户或 nonce 账户 |
| AlreadyProcessed | 状态缓存 | 该交易已被处理 |
| BlockhashNotFound | 有效期检查 | 区块哈希不在队列中且不是有效的 nonce |
| InstructionError | 执行 | 处理指令时发生错误（包含指令索引和具体的 InstructionError） |
| CallChainTooDeep | 账户加载 | Loader 调用链过深 |
| MissingSignatureForFee | 校验 | 交易需要手续费但未包含签名 |
| InvalidAccountIndex | 校验 | 交易包含无效的账户引用 |
| SignatureFailure | 签名验证 | Ed25519 签名验证失败（数据包被丢弃） |
| InvalidProgramForExecution | 账户加载 | 程序不属于有效的 Loader 所有 |
| SanitizeFailure | 校验 | 交易未能正确校验账户偏移 |
| ClusterMaintenance | 调度 | 由于集群维护，当前已禁用交易 |
| AccountBorrowOutstanding | 执行 | 交易处理后账户存在未清偿的借用引用 |
| WouldExceedMaxBlockCostLimit | 调度 | 交易将超出区块最大成本限制 |
| UnsupportedVersion | 校验 | 交易版本不受支持 |
| InvalidWritableAccount | 账户加载 | 交易加载了不可写的可写账户 |
| WouldExceedMaxAccountCostLimit | 调度 | 交易将超出区块内最大账户成本限制 |
| WouldExceedAccountDataBlockLimit | 调度 | 交易将超出区块内账户数据限制 |
| TooManyAccountLocks | 调度 | 交易锁定了过多账户 |
| AddressLookupTableNotFound | 账户加载 | 地址查找表账户不存在 |
| InvalidAddressLookupTableOwner | 账户加载 | 地址查找表归属于错误的程序 |
| InvalidAddressLookupTableData | 账户加载 | 地址查找表包含无效数据 |
| InvalidAddressLookupTableIndex | 账户加载 | 地址查找表查找使用了无效索引 |
| InvalidRentPayingAccount | 执行后检查 | 账户从 rent 免租金状态变为需支付 rent |
| WouldExceedMaxVoteCostLimit | 调度 | 交易将超出最大投票成本限制 |
| WouldExceedAccountDataTotalLimit | 调度 | 交易将超出总账户数据限制 |
| DuplicateInstruction | 计算预算解析 | 同一交易中存在重复的计算预算指令变体 |
| InsufficientFundsForRent | 执行后检查 | 账户 lamport 不足以支付其数据大小所需的 rent |
| MaxLoadedAccountsDataSizeExceeded | 账户加载 | 加载的数据总量超过 64 MiB 限制 |
| InvalidLoadedAccountsDataSizeLimit | 计算预算解析 | SetLoadedAccountsDataSizeLimit 被设置为 0 |
| ResanitizationNeeded | 校验 | 交易在功能激活前后存在差异，需要重新校验 |
| ProgramExecutionTemporarilyRestricted | 账户加载 | 参考账户上的程序执行被临时限制 |
| UnbalancedTransaction | 执行后检查 | 交易前后的 lamport 总余额不一致 |
| ProgramCacheHitMaxLimit | 账户加载 | 程序缓存达到最大限制 |
| CommitCancelled | 提交 | 提交已在内部取消 |
