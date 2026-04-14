+++
title = "LLM审计智能合约"
date = 2026-04-04

[extra]
cover_image = "/covers/9/0.jpg"
cover_sentence = " "
tags =  ["Paper","Security", "LLM"]
draft = true
+++



## EVMbench

made by openai, paradigm,ottersec

该篇论文中提到了三种评估模式：

![评估模式](/covers/10/1.png)

- detect: 测试recall

- patch: agent尝试修补合约，测试修补成功率

- exploit: 在隔离环境中重放交易

EVMbench 的数据集主要来自 Code4rena (智能合约审计平台)，从每次审计竞赛结束的报告中筛选出的高危漏洞，测试环境设置在一个独立的 Ubuntu Docker容器中，与真实人工的审核情况相同，在测试时 agent 被禁止访问 web工具与互联网。 agent完成测试后，评分过程会在 agent无法访问的单独容器中进行

测试方法与评分:

- detect: 最终得分为发现漏洞的百分比(recall)

- patch: agent 直接在代码中修改，确保现有测试在智能合约被修改会仍能通过，上传在测试样例中没有使用过的攻击测试补丁修复后是否有用

- exploit: agent 端到端地攻击部署在本地链上的智能合约

![](/covers/10/2.png)

"OC"表示 OpenCode 框架；没有前缀的模型使用其开发者的 CLI 代理运行，例如 GPT-5.3-Codex 使用 Codex CLI 运行，Claude 模型通过 Claude 运行，Gemini 3 Pro 通过 Gemini CLI 运行





