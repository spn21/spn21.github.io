+++
title = "非 EVM 智能合约漏洞检测：Prompt 与 Fine-Tuning 对比"
date = 2026-02-14

[extra]
cover_image = "/covers/third.png"
cover_sentence = "rust相关漏洞感觉更可以适合llm检测？"
tags = ["Security", "LLM", "paper"]
+++

核心方向：过去十年，Ethereum / Solidity 几乎定义了智能合约安全研究的全部语境：
SWC Registry、Slither、Mythril、SmartCheck……

- Solana、Algorand 等非 EVM 公链正在快速增长
- 使用 Rust、TEAL / PyTeal 等完全不同的语言与执行模型
- 现有安全工具 无法直接迁移

本文围绕大语言模型能否成为非 EVM 智能合约的通用漏洞检测工具展开。

RQ1: Ethereum 生态中的 OWASP Top 10 漏洞，在 Algorand 和 Solana 中是否仍然成立？如何映射？

RQ2: LLM 在 Rust（Solana）和 PyTeal（Algorand） 这类少研究语言中，是否仍具备漏洞检测能力？

RQ3: Prompt Engineering、Fine-Tuning 及其组合，对检测效果的提升有多大？

### 0x01 非 EVM 与 EVM对比:

Solana vs Ethereum

Solana 使用 Rust + 显式账户模型 + 并行执行(Sealevel):

- 没有 storage slot
- 所有账户必须显式传入
- Cross-Program Invocation(CPI) ≈ 非 EVM 风格"重入风险"
- PDA / bump seed 引入全新攻击面

Algorand vs Ethereum

Algorand 使用 TEAL(栈式语言)+ 原子交易组:

- 无重入（架构级消除）
- 无 fallback / delegatecall
- 漏洞集中在：
            任意删除 / 更新
            RekeyTo
            Unchecked Receiver / Fee

[Smart Contract Vulnerabilities according to OWASP Top 10 (2025)]

|ID	| Vulnerability                  |
|---|--------------------------------|
|V1 | Access Control Vulnerabilities |
|V2 | Price Oracle Manipulation      |
|V3 | Logic Errors                   |
|V4 | Lack of Input Validation       |
|V5 | Reentrancy Attacks             |
|V6 | Unchecked External Calls       |
|V7 | Flash Loan Attacks             |
|V8 | Integer Overflow and Underflow |
|V9 | Insecure Randomness            |
|V10| Denial of Service (DoS) Attacks|

### 0x02 RQ1: OWASP TOP 10 在非 EVM 的 mapping

结论: 非 EVM 公链必须采用**平台特定的威胁模型**

跨平台通用的漏洞:
- V1:Access Control
- V6:Unchecked External Calls

Solana 全部"理论可行":
Solana 在架构上允许所有 10 类漏洞存在，但:
- 实际攻击成本更高
- 很多漏洞需要极强的系统理解(如 leader DoS)

Algorand 天然消除的漏洞:
- Reentrancy(V5)
- Flash Loan 攻击(V7)
- Insecure Randomness(V9)

原因：原子交易 + VRF + 无回调模型

### 0x03 RQ2: LLM 对 Rust / PyTeal 的理解

结论: LLM 能理解 Rust 合约的"语义结构"，但面对 TEAL 这种低表达、栈式语言，能力明显下降

作者手工构建了一个:

- Rust(Solana) & PyTeal(Algorand)
- 每类漏洞 ≥ 5 个正样本 + 5 个负样本
- OWASP 启发式标注
- 二分类(是否存在指定漏洞)

使用模型:
- LLaMA-3-8B
- DeepSeek-R1-Distill-Qwen-14B

零 Prompt / 零 Fine-tuning 结果
| 平台       | 模型    | 平均准确率 |
| -------- | -------- | --------  |
| Solana   | DeepSeek | 0.63      |
| Solana   | LLaMA    | 0.53      |
| Algorand | DeepSeek | 0.60 |
| Algorand | LLaMA    | 0.50 |

- Rust > PyTeal
- DeepSeek 明显强于 LLaMA
- LLaMA 在 PyTeal 上几乎无法识别

### 0x04 Prompt Engineering vs Fine-Tuning

结论: 

- DeepSeek:
        稳定、泛化强
        对 prompt 不敏感

- LLaMA:
        对训练上下文高度敏感
        混合策略效果最好

- Prompt Engineering 是性价比最高的 baseline

三种配置：

- Fine-Tuning(LoRA)
- Prompt Engineering(角色提示)

  prompt示例:
  ```
     You are a smart contract security analyzer.
     The vulnerabilities are classified according to OWASP Top 10.
  ```

二者结合

- Solana（Rust）

Prompt Engineering 单独就很强

DeepSeek 在不微调的情况下表现已接近上限

- Algorand（PyTeal）

Fine-Tuning 明显更重要

LLaMA 在「Prompt + Fine-Tuning」下达到最佳表现（0.65）

### 最终结论


1. OWASP 漏洞在非 EVM 中仍然有意义，但必须重映射

2. LLM 可以作为非 EVM 静态分析的可行工具

3. 语言表达能力决定 LLM 上限

- Rust > PyTeal

4. Prompt Engineering ≠ 玩具

- 在 Solana 上几乎等同于 Fine-Tuning

5. Fine-Tuning 更适合低层 DSL

另:

|Blockchain	| Vulnerability	|Accuracy|	Precision|	F1-score| Recall|
|-----------| --------------| ------| -----|----| ----|
|           |                | DS	LM	|DS	LM	|DS	LM	|DS	LM|
|Solana	|Bump Seed|	0.80	0.60|	1.00	1.00 |	0.75	0.33 |	0.60	0.20|
|       | CPI	|0.53	0.60	|0.60	1.00	|0.30	0.33	|0.20	0.20|
| |Integer Flow	|0.67	0.43|	0.78	0.00|	0.58	0.00|	0.47	0.00|
| |Missing Key Check|	0.57	0.50	|1.00	0.00	|0.24	0.00	|0.13	0.00|
| |Type Confusion|	0.60	0.50	|1.00	0.00|	0.33	0.00	|0.20	0.00|
| |Avg.|	0.63	0.53|  |   |   |  |						
|Algorand|	Arbitrary Deletion|	0.57	0.50	|1.00	0.00	|0.24	0.00	|0.13	0.00|
| |Arbitrary Update|	0.73	0.50|	1.00	0.00|	0.64	0.00|	0.47	0.00|
| |Unchecked Asset Close To	|0.50	0.50	|0.00	0.00	|0.00	0.00	|0.00	0.00|
| |Unchecked Close Remainder To	|0.53	0.50	|1.00	0.00|	0.13	0.00|	0.07	0.00|
| |Unchecked Rekey To|	0.57	0.50|	1.00	0.00|	0.24	0.00|	0.13	0.00|
| |Unchecked Transaction Fee	|0.57	0.50	|1.00	|0.00	|0.24	0.00	|0.13	0.00|
| |Unchecked Asset Receiver	|0.60	0.50	|1.00	0.00	|0.33	0.00	|0.20	0.00|
| |Unchecked Payment Receiver	|0.70	0.50	|1.00	0.00	|0.57	0.00	|0.40	0.00|
| |Avg.|	0.60	0.50|  |  |  |						

TABLE II:Mapping of Vulnerabilities (V1 – V10) in Algorand and Solana 

|ID  | Algorand | Solana|
|--- | -------- | ------|
|V1| Arbitrary Update, Arbitrary Delete, Unchecked Payment Receiver, Unchecked Asset Receiver|Access control vulnerabilities are applicable. Requires Owner Check, Signer Check, Key Check.|
|V2|Vulnerability arises from off-chain oracle integration without authentication. Out of scope.|Oracle manipulation is possible via oracles like Pyth or Switchboard. Out of scope.|
|V3 |Generic logical vulnerabilities. Out of scope.|Generic logical vulnerabilities. Out of scope.|
|V4|N/A.|Type Confusion due to missing type checks when parsing account inputs.|
|V5|N/A.Algorand’s atomic, stateless transaction model prevents this class|Cross-Program Invocation (CPI) introduces risks similar to reentrancy. Requires strict validation.|
|V6|Present as Unchecked RekeyTo (dangerous rekeying without validation)|Tied to Bump Seed; unchecked low-level calls possible if not validated.|
|V7|N/A. Hard to exploit due to atomic groups, no nonce, and no penalties for failed txs.|Flash loan attacks are possible but depend on external protocols. Out of scope.|
|V8|TEAL-specific: Arithmetic overflow/underflow, Unchecked Transaction Fee|Integer overflow/underflow risk if no validation is performed.|
|V9|N/A.|PDA collisions from Bump Seed; similar to V6|
|V10| Partially mitigated; DoS possible via Unchecked Transaction Fee and resource exhaustion |Leader election DoS possible if randomness is predictable; network disruption could occur.|

[paper1:Prompt Engineering vs. Fine-Tuning for LLM-Based Vulnerability Detection in Solana and Algorand Smart Contracts](https://arxiv.org/abs/2511.11250)
