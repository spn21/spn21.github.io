+++
title = "Agent ? OS !"
date = 2026-05-31

[extra]
cover_image = "/covers/third.png"
cover_sentence = " "
tags = ["Agent", "OS", "Note"]
draft = true
+++


最近流水线看了一遍 agent, 又顺路把 jyy 老师的 26 版本的 os 课听了一部分, 在抖音上刷到有人的帖子, 感同身受

现在 agent 提出的很多看起来的新概念: harness, skill, RAG, memory, tool, multi-agent, context window

但如果浅浅了解一下, 会发现它们背后并不陌生, 还是 os 那一套的逻辑

操作系统当年要解决的问题是:

- 怎么隔离程序
- 怎么分配资源
- 怎么调度执行
- 怎么让用户程序安全访问外部能力
- 怎么在有限内存里放下更大的工作集

Agent 系统今天要解决的问题其实也差不多，只是资源从 CPU / memory / disk 变成了 token / reasoning time / tool

```text
OS:
user program -> syscall -> kernel -> hardware / file / network

Agent:
model -> function calling -> harness -> web / code / database / external system
```

所以 Agent 并不是凭空出现的一种新魔法，更像是自然语言时代的"用户态程序"

## 0x01 Process / Thread -> Agent / sub-Agent

**"用通信来共享内存, 而不是通过共享内存来通信"**

在操作系统里，process 是资源边界，thread 是执行单元。

同一个进程里的线程可以共享地址空间，读写同一块内存；不同进程之间默认隔离，如果要交流，就要通过 IPC、socket、pipe、shared memory 这类机制

Agent 世界也有类似结构:

```text
main Agent
    -> sub-Agent A
    -> sub-Agent B
    -> sub-Agent C
```

多个 Agent 并行协作, 第一眼看起来是效率提升

但只要它们开始共享状态，老问题就回来了:

- 谁能看到 shared context?
- 谁能修改 task state?
- 多个 Agent 同时写同一份计划怎么办?
- 一个 Agent 等另一个 Agent 的结果，另一个 Agent 又在等它，是否会 deadlock?
- sub-Agent 的输出如果互相矛盾，谁来做 consistency check?

这和多线程编程很像。

共享上下文像共享内存，速度快，但也容易出现 race condition。

消息传递像 IPC，边界清楚，但会增加通信成本。

```text
shared memory:
fast, but dangerous

message passing:
clean, but expensive
```

所以 Multi-Agent 的难点不只是"让很多 Agent 同时跑"，而是要定义清楚:

谁拥有状态，谁能修改状态，谁负责最终合并结果。

不然并行越多，系统越像一个没有锁的多线程程序。

## 0x02 System Call -> Tool Use


用户程序不能直接访问硬件，不能随便读磁盘、发网络包、改内核数据结构。

它必须通过系统调用陷入内核:

```c
read(fd, buf, size);
write(fd, buf, size);
open(path, flags);
```

内核检查权限、执行操作、返回结果。

Agent 也是一样。

模型本身不会真的上网，不会真的运行代码，也不会真的访问数据库。

它只能生成一个结构化请求:

```json
{
  "tool": "search",
  "arguments": {
    "query": "latest OpenAI API docs"
  }
}
```

然后交给 harness 执行。

这里的本质是:

> 在权限边界上打一个受控的洞。

能力从这个洞流进来，风险也被这个洞隔住。

OS 里系统调用需要 syscall table、permission check、capability model。

Agent 里 tool use 也需要:

- tool schema
- argument validation
- sandbox
- permission boundary
- audit log
- retry / timeout

如果没有 harness 这层，model 就像一个能生成指令但没有内核保护的程序。

它可能说"删除所有文件"，也可能说"访问某个私有数据库"。

真正决定能不能做的，不应该是模型的欲望，而是 tool boundary。

## 0x03 Cache / Virtual Memory -> Context Window

Context window 是 Agent 最贵的资源。

它不像普通内存，可以随便塞；每个 token 都会影响成本、速度和注意力分配。

这和 CPU cache / virtual memory 的感觉很像。

```text
register:
    当前正在推理的少量变量

cache:
    最近几轮对话 / 当前文件 / 当前任务计划

memory:
    更长的项目上下文 / 文档 / 历史状态

disk:
    压缩摘要 / RAG 知识库 / log / repo
```

当上下文放不下时，Agent 框架就要做 context compression。

这其实很像 paging / swapping。

不同的是，操作系统换出去的是 page，Agent 换出去的是语义。

```text
memory page:
    bytes with address

context summary:
    meaning with loss
```

这里有一个很重要的不同:

操作系统分页通常希望语义无损，换出去再换回来，程序看到的 byte 应该还是原来的 byte。

但 context compression 很难做到无损。

摘要一旦丢掉细节，后面再取回来时，Agent 可能只记得"大概发生过什么"，不记得"具体哪一行为什么这么改"。

所以 Agent 的 memory management 更像:

缓存 + 摘要 + 日志检索的混合体。

它不只是容量问题，也是信息保真度问题。

## 0x04 File System Mount -> RAG

RAG 很像把外部知识库挂载进系统。

操作系统里，文件系统不一定都在本地磁盘。

它可以 mount:

- local disk
- network storage
- external device
- virtual filesystem

程序看到的是一个统一的路径树。

Agent 也类似。

模型自己的 context window 放不下所有资料，所以把外部文档、代码库、数据库、网页内容挂到 RAG 后面。

需要的时候检索，不需要的时候不占上下文。

```text
expensive fast memory:
    context window

cheap large storage:
    vector db / documents / repo / logs
```

这和 OS 用低速大容量存储补偿昂贵内存很像。

但是 RAG 和文件系统也有关键不同。

文件系统读文件，路径通常是确定的:

```text
/home/user/project/main.rs
```

RAG 检索更像"语义路径":

```text
"和权限边界相关的历史设计文档"
```

它不是按地址取数据，而是按相似度取数据。

所以 RAG 的问题不是文件找不到，而是:

- 取到了看起来相关但其实不相关的 chunk
- 没取到真正关键的上下文
- 取到了过期文档
- 多个片段之间互相冲突

这就像一个文件系统会根据"感觉像"来返回文件。

很强，也很危险。

## 0x05 Kernel / Scheduler -> Harness / Orchestrator

Agent = Model + Harness。

Model 是计算本身，Harness 更像操作系统内核。

它负责:

- 管 tool permission
- 管 sandbox
- 管上下文注入
- 管任务分解
- 管 tool call 的返回
- 管错误恢复
- 管多 Agent 调度

在多 Agent 系统里，Orchestrator 就更像 scheduler。

它要决定:

```text
which Agent runs?
how long does it run?
what context does it receive?
where should its result go?
should it be retried?
should it be killed?
```

这和操作系统里的调度问题很像。

CPU 时间有限，所以 OS 要做 time slice。

推理时间和 token 也有限，所以 Agent 系统要决定哪个子任务值得继续推，哪个任务应该提前停止。

一个成熟的 Agent framework，最后大概率会长出很多 OS 味道的东西:

- permission model
- scheduler
- memory manager
- process table
- tool registry
- logging system
- sandbox
- recovery mechanism

只是它们服务的不是二进制程序，而是自然语言程序。

## 0x06 相同点: 都是在有限资源上构造秩序

OS 和 Agent 最像的地方，不是某个 API 的表面对应，而是它们都在处理同一种系统问题:

> 怎么让一个不完全可信、能力有限、需要外部资源的执行体，稳定地完成任务。

OS 面对的是 user program。

Agent harness 面对的是 model。

两者都不能假设执行体永远正确。

程序会 crash，model 会 hallucinate。

程序会越界访问，model 会越权请求。

程序会死循环，model 会陷入无意义的推理循环。

程序需要 I/O，model 需要 tool。

所以核心都不是"给它更多能力"，而是:

- 能力如何授权
- 状态如何隔离
- 资源如何计量
- 错误如何恢复
- 输出如何验证

从这个角度看，Agent engineering 很像系统工程。

不是 prompt 写得更华丽，而是边界、调度、记忆、工具、验证这些东西是否真的可控。

## 0x07 不同点: Agent 的状态是语义的

但 Agent 和 OS 也不能完全等同。

最大的不同是:

OS 管理的是确定性的机器状态，Agent 管理的是不稳定的语义状态。

操作系统里的内存地址、page table、file descriptor 都是明确的。

但 Agent 的 context 不是这样。

一句话放进上下文后，它产生什么影响，不只是由字面内容决定，还取决于:

- 它和其它上下文的关系
- 模型当前注意到什么
- prompt 的优先级
- 工具返回的结构
- 历史摘要有没有丢细节

所以 OS 里的 bug 通常可以追到某个状态转移。

Agent 里的 bug 经常是:

"它好像理解错了"

"它漏看了某个约束"

"它把旧信息当成新的"

"它把类似概念混在一起了"

这类问题更难 debug。

因为 byte-level state 可以 dump，semantic-level state 很难完整 dump。

## 0x08 不同点: OS 的程序被执行，Agent 的程序在生成

另一个区别是:

传统程序通常先写好，再交给 OS 执行。

Agent 的"程序"很多时候是在运行时生成的。

Prompt、tool call、intermediate reasoning、subtask plan，都是边执行边生成。

```text
traditional:
source code -> compile -> execute

agent:
goal -> plan -> tool use -> observe -> re-plan -> execute
```

这让 Agent 更灵活，也更难验证。

OS 可以假设程序二进制已经存在。

Agent harness 却要面对一个不断改写自己执行路径的模型。

所以 Agent 系统里，verification 可能比 generation 更重要。

能生成答案只是第一步。

更重要的是:

- 这个答案有没有证据
- tool call 是否真的成功
- 文件是否真的被修改
- 测试是否真的通过
- 结论是否来自当前上下文，而不是模型记忆

## 0x09 小结

学习 Agent 时重新看 OS，会发现很多问题并没有消失，只是换了皮肤。

过去我们问:

```text
process 如何隔离?
thread 如何同步?
syscall 如何授权?
cache 如何命中?
memory 如何换页?
scheduler 如何公平?
```

今天我们问:

```text
Agent 如何隔离?
sub-Agent 如何协作?
tool use 如何授权?
context 如何管理?
RAG 如何取回正确知识?
orchestrator 如何调度?
```

资源从 CPU / memory 变成了 token / inference time。

程序从 machine instruction 变成了 natural language。

系统调用从 syscall 变成了 function calling。

文件系统从 path lookup 变成了 semantic retrieval。

但底层问题还是:

隔离、权限、调度、记忆、一致性。

操作系统花了几十年才把这些问题想清楚。

Agent 时代正在把同样的问题重新答一遍。

历史不会重复，但真的会押韵。


原帖:
![/covers/13/1.jpg]

