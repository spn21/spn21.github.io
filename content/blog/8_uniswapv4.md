+++
title = "UniswapV4 Learning"
date = 2026-03-12

[extra]
cover_image = "/covers/8/0.png"
cover_sentence = ""
tags = ["Solidity", "Learning", "Note"]
draft = false
+++


![v4与v3的区别](/covers/8/1.png)

v4版本相较于v3,主要引入了4个功能:

## 1.Hooks

在uniswapv4中,每个流动池不再对应一个合约，而是所有池子都在一个合约下(PoolKey作为Pool的唯一ID),以下是官方给出创建流动池的例子

``` solidity

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {IPoolManager} from "v4-core/src/interfaces/IPoolManager.sol";
import {IHooks} from "v4-core/src/interfaces/IHooks.sol";
import {PoolKey} from "v4-core/src/types/PoolKey.sol";
import {CurrencyLibrary, Currency} from "v4-core/src/types/Currency.sol";

contract PoolInitialize {
    using CurrencyLibrary for Currency;

    // set the initialize router
    IPoolManager manager = IPoolManager(address(0x01));

    function init(
        address token0,
        address token1,
        uint24 swapFee,
        int24 tickSpacing,
        address hook,
        uint160 sqrtPriceX96,
        bytes calldata hookData
    ) external {
        // sort your tokens! v4 requires token0 < token1
        if (token0 > token1) {
            (token0, token1) = (token1, token0);
        }

        PoolKey memory pool = PoolKey({
            currency0: Currency.wrap(token0),
            currency1: Currency.wrap(token1),
            fee: swapFee,
            tickSpacing: tickSpacing,
            hooks: IHooks(hook)
        });
        manager.initialize(pool, sqrtPriceX96, hookData);
    }
}

```

可见即使token,swapFee等相同,但是如果hook的address不同，也会被看成不同的流动池.

hook也许是uniswapv4中最重要的内容(在javascript中,hook作为拦截和修改func,使开发者可以拓展代码而不需要修改原始代码),在uniswapv4中,hook的作用也差不多.

在v4中,hooks可以发挥5个主要功能:

  1.beforeInitialize/afterInitialize

  2.beforeAddLiquidity/afterAddLiquidity

  3.beforeRemoveLiquidity/afterRemoveLiquidity

  4.beforeSwap/afterSwap

  5.beforeDonate/afterDonate

v4中允许hook在特定操作(如上面功能名称)前后进行调用,从而实现特定操作,同时,hook可以在swap中收取费用，可以静态也可以动态管理.也可以进行特定比例分配给任意地址.

下图是官方白皮书中hook的工作流程,描述了```beforeSwap```和```afterSwap中hook```是如何工作的.

![官方白皮书中的hook示例](/covers/8/2.png)



官方给出的hook示例:

``` solidity

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {BaseHook} from "v4-periphery/src/base/hooks/BaseHook.sol";
import {PoolKey} from "v4-core/types/PoolKey.sol";
import {IPoolManager} from "v4-core/interfaces/IPoolManager.sol";
import {Hooks} from "v4-core/libraries/Hooks.sol";

contract SampleHook is BaseHook  {
    constructor(IPoolManager _manager, string memory _name, string memory _symbol) BaseHook(_manager){
// do something
    }

    function getHookPermissions() public pure override returns (Hooks.Permissions memory) {
        return Hooks.Permissions({
            beforeInitialize: false,
            afterInitialize: false,
            beforeAddLiquidity: true,
            beforeRemoveLiquidity: false,
            afterAddLiquidity: true,
            afterRemoveLiquidity: false,
            beforeSwap: false,
            afterSwap: false,
            beforeDonate: false,
            afterDonate: false,
            beforeSwapReturnDelta: false,
            afterSwapReturnDelta: false,
            afterAddLiquidityReturnDelta: false,
            afterRemoveLiquidityReturnDelta: false
        });
    }
    function beforeAddLiquidity(
        address sender,
        PoolKey calldata key,
        IPoolManager.ModifyLiquidityParams calldata params,
        bytes override calldata hookData
    ) external returns (bytes4){
    	// do something
    }
    function afterAddLiquidity(
        address sender,
        PoolKey calldata key,
        IPoolManager.ModifyLiquidityParams calldata params,
        BalanceDelta delta,
        bytes calldata hookData
    ) external override returns (bytes4, BalanceDelta); {
		// do something
    }
}

```

可以看到,hook作为一个合约,继承于```BaseHook```,

在构造一个hook中,必须包含``` IPoolManager```参数，其为管理接口,同时也有如``` onlyByPoolManager```修饰符,作为管理员权限func.

```getHookPermission```func必须重载,可以实现上面所说的五个功能，把对应的功能(如```beforeAddLiquidity```)设置为true即可.

## 2.Singleton

先前的Uniswap版本使用factor/pool模式进行代币部署,Uniswapv4使用singleton设计模式,所有资金池由一个合约管理,使池子到部署成本降低99%.

同时singleton与flash accounting互补,v4中,每个操作(如兑换或向池中添加流动性),只更新一个内部净余额 delta,仅在锁定结束时进行外部转账.新的```take()```和```settle()```分别借入或存入,不过d elta 值在执行前后必须保持不变.它表示用户和资金池之间的未结余额，并且在交易开始时和结束时应始终返回零.

对于 delta 这种值,以太坊为此类临时数据提供了一种特定的存储类型：瞬态存储。这是通过 EIP-1153 在 Dencun 升级（2024 年）中引入的。瞬态存储的行为类似于标准以太坊存储，但仅在单个交易中有效。交易结束后，所有瞬态存储值都会自动重置.由于瞬态存储是临时的，不会增加以太坊的永久存储负载，因此使用瞬态存储的作比常规存储便宜得多。例如，写入空存储槽需要 20000 gas，而写入瞬态存储只需 100 gas.

![Mechanism of Transient Storage](/covers/8/3.png)

``` By requiring that no tokens are owed to the pool manager or to the caller by the end of the call, the pool’s solvency is enforced. ```


**仅执行一次余额更新，而不是为每个池单独修改余额**

## 3.Flash accounting


两者加上hook,可以增加pool的数量

## 4.Native ETH

Uniswapv4允许同时支持WETH和ETH配对.原生ETH转账的gas成本大约是ERC-20转账的一半(ETH的gas成本为21k,而ERC-20为约40k).

同时,引入hook之后,```PoolManager```不需要Price Oracle进行数据存储和逻辑处理,Oracle可以通过hook合约.

## 5.Custom Accounting

Hooks without Custom Accounting:

大多数流程中(如beforeswap)会造成revert;

potential attack:


```solidity

removing liquidity: beforeSwap

sandwich attack: beforeSwap+afterSwap and before donate+after donate

permanently lock: before removeLiquidity, after removeLiquidity

```
总体来说,v4算法还是集中流动性,hook实习pool可定制性,singleton,flash accounting以及native ETH降低gas成本.

参考资料：

[audit by OpenZeppelin](https://github.com/Uniswap/v4-core/blob/main/docs/security/audits/OpenZeppelin_audit_core.pdf)

[coset](https://foresightnews.pro/article/detail/69506)

[2077research](https://research.2077.xyz/a-complete-guide-to-uniswap-v4-1#general-behavior-of-hooks)