+++
title = "Rust Quiz"
date = 2026-03-10

[extra]
cover_image = "/covers/7.jpg"
cover_sentence = ""
tags = ["learing", "rust", "note"]
draft = true
+++


### quiz1

```rust
macro_rules! m {
    ($($s:stmt)*) => {
        $(
            { stringify!($s); 1 }
        )<<*
    };
}

fn main() {
    print!(
        "{}{}{}",
        m! { return || true },
        m! { (return) || true },
        m! { {return} || true },
    );
}

output 112

macro:	$( )类似正则表达式
	    $($S: stmt)* 代表匹配零个或多个语句tokens
main:	m!{return || true} <=> return (|| true) 返回一个闭包 
		{"return || true"; 1}
		m!{(return) || true} 或操作 rerturn作为一个表达式，产生的值为NeverType(!)，右边表达式为true		
		{"return || true; 1"} value = 1
		m!{{return} || true} return用{}隔开，代表为一个独立语句，true 可以被看做一个独立闭包，
		{"{true}"; 1} 1<<1 value = 2
```

### quiz2

```rust
struct S(i32);

impl std::ops::BitAnd<S> for () {
    type Output = ();

    fn bitand(self, rhs: S) {
        print!("{}", rhs.0);//省略返回类型
    }
}

fn main() {
    let f = || ( () & S(1) );
    let g = || { () & S(2) };
    let h = || ( {} & S(3) );
    let i = || { {} & S(4) };
    f();
    g();
    h();
    i();
}

output 123

let f = || ( () & S(1) ); 
() & S(1) 输出为rhs.0
let g = || { () & S(2) };
let h = || ( {} & S(3) );
这里闭包是用圆括号来表示，这代表，{} & S(3)按一个完整独立的表达式来处理。其中{}表达式求值结果是()，再和S(3)进行与操作，得到最终值：3
let i = || { {} & S(4) };
{}代表一个块，默认按多条表达式进行解析 {}&S(4)可以看为{}和 &S(4) 返回结果看作为对S(4)的引用
```

### quiz3

```rust
struct S {
    x: i32,
}

const S: S = S { x: 2 };

fn main() {
    let v = &mut S;
    v.x += 1;
    S.x += 1;
    print!("{}{}", v.x, S.x);
}

output: 32
```

### quiz4

```rust
fn main() {
    let (.., x, y) = (0, 1, ..);//RangeFull 类型
    print!("{}", b"066"[y][x]);
}

output 54

let range_full = std::ops::RangeFull;
<=>
let range_full = ..;

模式匹配：
.. 匹配 0
x 匹配 1
y 匹配 ..
打印"066"中的"6",而6的ASCII = 54

### quiz5 （//已经不在quiz中）

trait Trait {
    fn f(self);
}

impl<T> Trait for fn(T) {
    fn f(self) {
        print!("1");
    }
}

impl<T> Trait for fn(&T) {
    fn f(self) {
        print!("2");
    }
}

fn main() {
    let a: fn(_) = |_: u8| {};
    let b: fn(_) = |_: &u8| {};
    let c: fn(&_) = |_: &u8| {};
    a.f();
    b.f();
    c.f();
}

output 112
```

T和&T的区别： T是一个泛型参数，代表一个具体类型，fn(T)为一个函数指针 &T等价于&'a T，代表某个引用类型，fn(&T)等价于fn(&'a T) ('a也是一个泛型参数并不指具体类型)


### quiz6

```rust
use std::mem;

fn main() {
    let a;
    let a =  a = true;
    print!("{}", mem::size_of_val(&a));
}

output 0

a = true => 编译器判断a的类型为bool
(a = (a = true) ) 等价于 (a = ()) 类型不匹配会报错
let a = a = true时有变量屏蔽(重新定义了一个a)
//rust编译器会通过编译，但会爆warning

warning: variable `a` is assigned to, but never used
 --> src/main.rs:4:9
  |
4 |     let a;
  |         ^
  |
  = note: consider using `_a` instead
  = note: `#[warn(unused_variables)]` on by default

warning: value assigned to `a` is never read
 --> src/main.rs:5:13
  |
5 |     let a = a = true;
  |             ^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` on by default
```

### quiz7

``` rust
//This question is no longer part of the Rust Quiz.

#[repr(u8)]
enum Enum {
    First,
    Second,
}

impl Enum {
    fn p(self) {
        match self {
            First => print!("1"),
            Second => print!("2"),
        }
    }
}

fn main() {
    Enum::p(unsafe {
        std::mem::transmute(1u8)
    });
}

第一次执行时，会报错
error[E0170]: pattern binding `First` is named the same as one of the variants of the type `Enum`
  --> src/main.rs:10:13
   |
10 |             First => print!("1"),
   |             ^^^^^ help: to match on the variant, qualify the path: `Enum::First`
   |
   = note: `#[deny(bindings_with_variant_name)]` on by default
```

First和Second和Enum定义的枚举值同名，提示需要加上Enum前缀

加上前缀后
``` rust
output: 2

//修改之前原output为1
```

### quiz8

```rust
macro_rules! m {
    (==>) => { print!("1"); };
    (= = >) => { print!("2"); };
    (== >) => { print!("3"); };
    (= =>) => { print!("4"); };
}

fn main() {
    m!(==>);
    m!(= = >);
    m!(== >);
    m!(= =>);
}

output: 1214

Rust用 macro_rules!来定义宏匹配规则
声明宏的语法规则为
macro_rules! $name {
	$rule0;
	$rule1;
	//...
	$ruleN;
}

宏也可以在函数内部声明和使用

fn foo() {
	//m! ();
	macro_rules! m {
		() => {};
	}
	m!();
}
// m!(); // Error: m 未在作用域内.
```

对于quiz8，定义声明宏m! 包含了四个匹配模式

(==>) 表示要直接和分词得到的token流进行匹配（且rust中 == 优先级高于 =）

(= = >) 同理，匹配=、 = 和 >（匹配模式中包含的空格忽略不计）

(== >)其实与(==>)一样，但是不一样的是它们在宏定义中出现的位置

(= =>) 同理，匹配 = 和 => 

//q 为什么 m!(== >) 输出为1

修改一下代码

```rust
macro_rules! m {
    (==>) => { print!("3"); };
    (= = >) => { print!("2"); };
    (== >) => { print!("1"); };
    (= =>) => { print!("4"); };
}

fn main() {
    m!(==>);
    m!(= = >);
    m!(== >);
    m!(= =>);
}
```

output为 3234
说明宏的另一个特性：优先匹配最前面的匹配分支
因为m!(==>);默认分词匹配为 == 和 >，所以当碰到第一个匹配分支 (== >)时，匹配到了，后面不会继续寻找匹配分支

### quiz9

```rust
macro_rules! m {
    (1) => { print!("1") };
    ($tt:tt) => { print!("2") };
}

macro_rules! e {
    ($e:expr) => { m!($e) };
}

macro_rules! t {
    ($tt:tt) => { e!($tt); m!($tt); };
}

fn main() {
    t!(1);
}

output: 21
```

代码依次定义了三个声明宏：m!、e!、t!，并且一次为包含关系 在main函数中，调用t!(1)时，根据宏定义，其参数1经过词法分析得到词条树，也就是宏元变量$tt:tt的类型tt所指示。然后由e!($tt);和m!($tt);继续匹配。

e!($tt)在匹配过程中，因为e!定义左边元变量$e:expr，表明是一个表达式类型。对于Rust编译器来说，经过e!宏处理的词条，将会变成一个不透明的词条（opaque token tree）。也就是说，后续的宏都会认为它是一个表达式token，而不是其他。所以，在e!宏内匹配右侧m!($e);的时候，只能匹配m!宏的第二条分支($tt:tt)，所以输出： 2

如果把e!的匹配模式修改为

```rust
macro_rules! e {
    ($e:tt) => { m!($e) };
}

output: 11
```

### quiz10

```rust
trait Trait {
    fn f(&self);
}

impl<'a> dyn Trait + 'a {
    fn f(&self) {
        print!("1");
    }
}

impl Trait for bool {
    fn f(&self) {
        print!("2");
    }
}

fn main() {
    Trait::f(&true);
    Trait::f(&true as &dyn Trait);
    <_ as Trait>::f(&true);
    <_ as Trait>::f(&true as &dyn Trait);
    <bool as Trait>::f(&true);
}

output: 222222
```

关于trait： trait关键字一般用来声明一个特征，特征只定义行为看起来是什么样，而不定义行为具体是什么样

如果从函数中返回trait返回指向堆的trait指针，则需要使用dyn关键字编写返回类型

  Trait::f(&true);
//通过指定Trait::前缀，传入具体类型实例引用&true来调用对应的f方法，上述代码中明确为bool类型实现trait，故输出2
  Trait::f(&true as &dyn Trait);
//通过as关键字将&true 转为为 trait对象 &dyn Trait类型，但是对于编译器来说在&dyn Trait和bool之间选择更精确的类型，输出2
  <_ as Trait>::f(&true);
//为无歧义完全限定语法，通过<_ as Trait>，来指定实现了Trait的类型，调用的是Trait中实现的方法。_此处泛指实现了Trait的类型，Rust会根据上下文进行自动推断。所以，必然会去使用impl Trait for bool中定义的f实现。所以，这行代码依旧输出： 2。
  <_ as Trait>::f(&true as &dyn Trait);
//转为trait对象，也还是去寻找具体类型bool实现方法
  <bool as Trait>::f(&true);
//指定bool类型

### quiz11

```rust
fn f<'a>() {}
fn g<'a: 'a>() {}

fn main() {
    let pf = f::<'static> as fn();
    let pg = g::<'static> as fn();
    print!("{}", pf == pg);
}
```

会报编译错误

生命周期，rust中声明周期参数要求被调用时的生命周期比其定义时长，这里面'static'生命周期超出了'a'生命周期的范围

修改代码如下

```rust
fn g() {}

fn main() {
    let pf = f::<'static> as fn();
    let pg = g as fn(); // 没有生命周期参数
    print!("{}", pf == pg);
}
```

### quiz12

```rust
struct D(u8);

impl Drop for D {
    fn drop(&mut self) {
        print!("{}", self.0);
    }
}

struct S {
    d: D,
    x: u8,
}

fn main() {
    let S { x, .. } = S {
        d: D(1),
        x: 2,		//let x=2
    };
    print!("{}", x);

    let S { ref x, .. } = S {
        d: D(3),
        x: 4,
    };
    print!("{}", x);
}

output:1243
```

当我们对S实现一个drop

```rust
impl Drop for S {
	fn drop(&mut self) {
		print!("S");
	}
}

output:S124S3
```

可以发现最先析构的是S的第一个实例。其实是d:D(1)，第三个是结构体S的另一个实例，最后是d:D(3)，两个结构体实例因为是先声明的，先于D(1)和D(3)析构

修改quiz

```rust
let S {x, ..} = S {
	d:D(3),
	x:4,
};

output: S12S34
```

so

1.先drop S;

2.D(1)被析构，未绑定任何变量，drop 1;

3.结构体x中绑定x变量，2;

4.let绑定中使用了模式匹配操作ref，相当于let x = &S{x: 4},x 若此时抛弃该结构体实例会造成悬垂指针，4;

5.x被使用完，析构，drop S;

6.对D(3)析构，3

### quiz13

```rust
struct S;

fn main() {
    let [x, y] = &mut [S, S];
    let eq = x as *mut S == y as *mut S;
    print!("{}", eq as u8);
}

output: 1	
```

在main函数中，使用let绑定模式匹配&mut[S,S]数组，x和y借用的单元结构体S可看作是两个独立的结构体实例，所以可以同时被借用

x,y本身是可变借用，通过as转换为原生可变指针类型(*mut S),然后比较他们地址是否相同，将最后的bool类型结果赋给eq，最后通过as将bool类型转换为u8，0对应false，1对应true`

```rust
struct Empty;
fn main() {
    let x = &mut Empty;
    println!("x {:p}", x);
    let y = &mut Empty;
    println!("y {:p}", y);
}
```

此代码在debug模式下为

```rust
x 0x7fff5b8c2058
y 0x7fff5b8c20c0
```
Release模式下为

```rust
x 0x7ffe9ae101d8
y 0x7ffe9ae101d8
```

可见，debug模式下编译不同的单元结构体实例地址不同，release模式下不同单元结构体实例会被优化为同一个地址

```rust
struct S;
struct E;

struct A {
    s: S,
    e: E,
}

fn main() {
    let (x, y) = &mut (S, E);
    println!("{:p}", x as *mut S);
    println!("{:p}", y as *mut E);

    let A{s, e} = &mut A{s: S, e: E};
    println!("{:p}", s as *mut S);
    println!("{:p}", e as *mut E);
}
```
debug模式下

```rust
0x7ffe370bd7b8
0x7ffe370bd7b8
0x7ffe370bd890
0x7ffe370bd890
```

release模式下
```rust
0x7ffcde64f608
0x7ffcde64f608
0x7ffcde64f608
0x7ffcde64f608
```
关于sizedness（rust概念中可能不起眼的那个，但经常会搞事情x）

这里显示1的原因就因为ZST，所有{}计算结果为()，因为()为零字节，所以()的实例都是相同的，编译器会去优化和ZST相关的操作

### quiz14

```rust
trait Trait: Sized {
    fn is_reference(self) -> bool;
}

impl<'a, T> Trait for &'a T {
    fn is_reference(self) -> bool {
        true
    }
}

fn main() {
    match 0.is_reference() {
        true => print!("1"),
        false => print!("0"),
    }

    match '?'.is_reference() {
        true => print!("1"),
        false => {
            impl Trait for char {
                fn is_reference(self) -> bool {
                    false
                }
            }
            print!("0")
        }
    }
}

output:10
```

quiz中定义并限定了Sized trait，这意味着该trait无法当作trait对象来用

& 'a T实现了Trait

在main中，调用0.is_reference()的时候，会为0自动添加引用，等价于(&0).isrefence()。因为上下文中只有为&'a T实现了trait，此时is_reference方法中self等价于&'a i32，会返回true，并输出1

如果为T实现trait，就不会存在引用

match '?'.is_reference(),会调用?的is_reference()的方法，这里Impl Trait for char {...}对整段代码可见，会输出false

将impl去掉的话

```rust
match '?'.is_reference() {
    true => print!("1"),
    false => print!("0")
}
```
会输出11

### quiz15

```rust
trait Trait {
    fn f(&self);
}

impl Trait for u32 {
    fn f(&self) {
        print!("1");
    }
}

impl<'a> Trait for &'a i32 {	//'a也是一个泛型参数，并不是指具体的类型
    fn f(&self) {
        print!("2");
    }
}

fn main() {
    let x = &0;
    x.f();
}

output: 1
```

```rust
main函数直接使用了 let x = &0;，&0实际上是一个包含了具体生命周期参数实例的具体类型&'a 0，&0实际上会被推断为一个具体的T类型实例u32

将impl Trait for u32{}注释掉，则x.f()会输出2
```

### quiz16

```rust
fn main() {
    let mut x = 4;
    --x;
    print!("{}{}", --x, --x);
}

output:44
```

rust不存在自增/自减运算符，编译器会解析成-(-x)，按照负号而非减号进行解析

### quiz17

```rust
fn main() {
    let mut a = 5;
    let mut b = 3;
    print!("{}", a-- - --b);
}

output:2

a-- - --b<=> a - (-(-(-(-b)))) ,5 - 3 = 2
```

### quiz18

```rust
struct S {
    f: fn(),
}

impl S {
    fn f(&self) {
        print!("1");
    }
}

fn main() {
    let print2 = || print!("2");
    S { f: print2 }.f();
}

output:1
```

quiz代码中实现了结构体S，包含一个类型为fn()的函数指针字段，然后为S实现了同样名为f的调用方法

main函数最后一行代码：

```rust
s { f:print2 }.f();
```

看上去像是调用s的结构体实例的f方法？

不过print2闭包也可以作为一个函数指针，想要调用闭包，最好显示地加上括号告诉编译器你要调用闭包

```rust
struct S {
    f: fn(),
}

impl S {
    fn f(&self) {
        print!("1");
    }
}

fn main() {
    let print2 = || print!("2");
    (S { f: print2 }.f)();
}

output: 2
```

### quiz19

```rust
struct S;

impl Drop for S {
    fn drop(&mut self) {
        print!("1");
    }
}

fn main() {
    let s = S;
    let _ = s;
    print!("2");
}

output: 21

#[derive(Debug)]
struct S;

impl Drop for S {
    fn drop(&mut self) {
        print!("1");
    }
}

fn main() {
    let s = S;
    let _ = s;
    print!("{:?}", s)
}
```

输出结果为S1，在执行了let _= s;之后还是可以使用s，说明s的所有权并未被转移

所以先打印S，在main函数执行完之后，S的drop方法被自动调用，然后打印1

继续修改

```rust
fn main() {
    let s = S;
    let _a = s;
    print!("{:?}", s)
}
```

将_ 变成_a，执行代码时候报错：

error[E0382]: borrow of moved value: `s`

说明s的所有权已经被转移了（这里_a多了一次move操作）

再修改一次

```rust
struct S;

impl Drop for S {
    fn drop(&mut self) {
        print!("1");
    }
}

fn main() {
    let s = S;
    drop(s);
    print!("2");
}

output: 12
```

这里使用drop(s)，未转移所有权，也未绑定任何变量，直接drop，所以会打印语句之前调用drop方法，先输出1

### quiz20

```rust
fn return1() {
    if (return { print!("1") }) {
    }
}

fn return2() {
    if return { print!("2") } {
    }
}

fn break1() {
    loop {
        if (break { print!("1") }) {
        }
    }
}

fn break2() {
    loop {
        if break { print!("2") } {
        }
    }
}

fn main() {
    return1();
    return2();
    break1();
    break2();
}

output:121
```

rust在1.19更新之后加入了break in loop功能，可以从loop循环中使用break返回一个值

break2中因为没有()表达优先级，会解析成

```rust
fn break2() {
    loop {
        if (break) { 
            print!("2") 
        }
        { }
    }
}
```

会先break

在编译时会warning

```rust
quiz21

trait Trait {
    fn f(&self);
}

impl<F: FnOnce() -> bool> Trait for F {
    fn f(&self) {
        print!("1");
    }
}

impl Trait for () {
    fn f(&self) {
        print!("2");
    }
}

fn main() {
    let x = || { (return) || true; };
    x().f();
    //x绑定右侧，整体是一个闭包，当该闭包调用的时候，其闭包体内部的{(return) || true; }是一个完整的或操作表达式。不过，return首先会返回()，当调用x()时，会返回()，x().f()，实际上就是调用闭包的返回值()的f()方法，输出结果为2
    let x = loop { (break) || true; };
    x.f();
	//这两行代码中的x，是绑定了从loop循环中通过break关键字返回的单元值()。因为这里是break，通过括号将break做为了一个独立的表达式，和上面的return类似。
    let x = || { return (|| true); };
    x().f();
	//当闭包x被调用之后，返回|| true 闭包，这个闭包实现了FnOnce() -> bool闭包
    let x = loop { break (|| true); };
    x.f();
	//与上面相似
    let x = || { return || true; };
    x().f();
	//对于return来说加不加括号都一样
    let x = loop { break || true; };
    x.f();
    //break这里与return作用相同
}

output:221111
```

定义了一个trait和泛型实现，该泛型实现中包含了限定```rust <F:FnOnce() - > bool> ```，意思是只有FnOnce() -> bool闭包才可以调用f函数

修改一下

```rust
// 修改1
let x = loop { break { || true }; };
x.f();

// 修改2
let x = loop { break { || true } };
x.f();

// 修改3
let x = loop { break { || true; } };
x.f();
```

在修改3中，||true加了分号，对于块表达式来说，它的求值结果必然是().在rust中，加分号的表达式都会返回(),所以，修改3会输出2，修改1、2等价，都会返回1

### quiz22

```rust
macro_rules! m {
    ($a:tt) => { print!("1") };
    ($a:tt $b:tt) => { print!("2") };
    ($a:tt $b:tt $c:tt) => { print!("3") };
    ($a:tt $b:tt $c:tt $d:tt) => { print!("4") };
    ($a:tt $b:tt $c:tt $d:tt $e:tt) => { print!("5") };
    ($a:tt $b:tt $c:tt $d:tt $e:tt $f:tt) => { print!("6") };
    ($a:tt $b:tt $c:tt $d:tt $e:tt $f:tt $g:tt) => { print!("7") };
}

fn main() {
    m!(-1);
    m!(-1.);
    m!(-1.0);
    m!(-1.0e1);
    m!(-1.0e-1);
}

output:22222
```

在main函数中,

m!(-1)，其中-1，会被Rust分词为两个独立词条（Token）：-和1。所以，宏调用会匹配到第二个模式条件分支，最终输出：2。

Rust对于数字1、1.、1.0、1.0e1和1.0e-1均会识别为一个词条。所以，剩余的4个宏调用，也同样会输出： 2。

-永远都会被识别为独立的词条，比如下面这段代码：

```rust
fn main() {
    let n = -3i32.pow(4);
    println!("{}", n);
}

output：-81。

```

前面22个quiz从张汉东的文章里面学习到，文章比较古老，后面新更的就更快速（才不是因为懒bushi）去解决

```rust
trait Trait {
    fn f(&self);
    fn g(&self);
}

struct S;

impl S {
    fn f(&self) {
        print!("1");
    }

    fn g(&mut self) {
        print!("1");
    }
}

impl Trait for S {
    fn f(&self) {
        print!("2");
    }

    fn g(&self) {
        print!("2");
    }
}

fn main() {
    S.f();
    S.g();
}

output:12
```

不特殊标注调用语法始终优先选择固有方法，Trait::f(&S)或<S as Trairt>::f(&S)才能调用特征方法

### quiz24

```rust
fn main() {
    let x: u8 = 1;
    const K: u8 = 2;

    macro_rules! m {
        () => {
            print!("{}{}", x, K);
        };
    }

    {
        let x: u8 = 3;
        const K: u8 = 4;

        m!();
    }
}

output: 14
```

Hygiene in macro_rules! only applies to local variables.

have a question here: how to recompose to print"3"?

### quiz25

```rust
use std::fmt::{self, Display};

struct S;

impl Display for S {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("1")
    }
}

impl Drop for S {
    fn drop(&mut self) {
        print!("2");
    }
}

fn f() -> S {
    S
}

fn main() {
    let S = f();
    print!("{}", S);
}

output:212
```

main第一行调用f执行不绑定新变量的可靠匹配，无变量可以是f返回s的所有者，因此s在此时被删除

第二行生成一个新的s打印生成

### quiz26

```rust
fn main() {
    let input = vec![1, 2, 3];

    let parity = input
        .iter()
        .map(|x| {
            print!("{}", x);
            x % 2
        });

    for p in parity {
        print!("{}", p);
    }
}
```
map操作延迟执行

output will alternate between numbers printed by the closure and numbers printed by the loop body.

### quiz27

```rust
trait Base {
    fn method(&self) {
        print!("1");
    }
}

trait Derived: Base {
    fn method(&self) {
        print!("2");
    }
}

struct BothTraits;
impl Base for BothTraits {}
impl Derived for BothTraits {}

fn dynamic_dispatch(x: &dyn Base) {
    x.method();
}

fn static_dispatch<T: Base>(x: T) {
    x.method();
}

fn main() {
    dynamic_dispatch(&BothTraits);
    static_dispatch(BothTraits);
}

output:11
```

两个traitBase和Derived各自提供了一个method的方法，都提供了特征方法的默认实现。默认实现在概念上被复制到每个没有显示定义的特征实现中，BothTraits的Base实现将进行trait的默认行为，即print!("1");

在main函数中，x.method()是对自动生成的方法的调用，x通过BothTraits转换为dyn Base获得，最终转发到<BothTraits as Base>::method

特别注意.method()不能调用Derived::method，因为x的类型dyn Base并且没有Derived

这里相似的c++代码的output为22

Rust的Traits和superTraits于c++继承的区别

``` c++
#include <iostream>

struct Base {
    virtual void method() const {
        std::cout << "1";
    }
};

struct Derived: Base {
    void method() const {
        std::cout << "2";
    }
};

void dynamic_dispatch(const Base &x) {
    x.method();
}

template <typename T>
void static_dispatch(const T x) {
    x.method();
}

int main() {
    dynamic_dispatch(Derived{});
    static_dispatch(Derived{});
}
```

### quiz28

```rust
struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        print!("1");
    }
}

fn main() {
    let _guard = Guard;
    print!("3");
    let _ = Guard;
    print!("2");
}
```

_tmp 变量在出作用域时drop

let _ drop立即执行

_ 不是一个变量，而是一个不绑定任何内容的通配符模式

### quiz29

```rust
trait Trait {
    fn p(&self);
}

impl Trait for (u32) {
    fn p(&self) { print!("1"); }
}

impl Trait for (i32,) {
    fn p(&self) { print!("2"); }
}

impl Trait for (u32, u32) {
    fn p(&self) { print!("3"); }
}

impl Trait for (i32, i32,) {
    fn p(&self) { print!("4"); }
}

fn main() {
    (0).p();
    (0,).p();
    (0, 0).p();
    (0, 0,).p();
}

output:1244
```

(0)等同于0

(0,)是含有一个元素的元组

(0,0)等同于(0,0,)

### quiz30

```rust
use std::rc::Rc;

struct A;

fn p<X>(x: X) {
    match std::mem::size_of::<X>() {
        0 => print!("0"),
        _ => print!("1"),
    }
}

fn main() {
    let a = &A;
    p(a);
    p(a.clone());
    
    let b = &();
    p(b);
    p(b.clone());
    
    let c = Rc::new(());
    p(Rc::clone(&c));
    p(c.clone());
}

output:111011
```

()和A都是ZST，Rc中队p的调用都是使用非零大小的X= Rc<()>，使得Rc使用Rc::clone(&c)而不是c.clone

To call the clone method of a value inside a Rc, you would need to dereference it first: (*c).clone().

### quiz31

```rust
trait Or {
    fn f(self);
}

struct T;

impl Or for &T {
    fn f(self) {
        print!("1");
    }
}

impl Or for &&&&T {
    fn f(self) {
        print!("2");
    }
}

fn main() {
    let t = T;
    let wt = &T;
    let wwt = &&T;
    let wwwt = &&&T;
    let wwwwt = &&&&T;
    let wwwwwt = &&&&&T;
    t.f();
    //搜索类型T上定义的函数f->搜索类型&T并找到Or特征的第一个实现
    wt.f();
    wwt.f();
    //&&T -> &&&T -> &mut &&T -> &T
    wwwt.f();
    //&&&T -> &&&&T
    wwwwt.f();
    //&&&&T
    wwwwwt.f();
    //&&&&&T -> &&&&&&T -> &mut &&&&&T -> &&&&T
}

output:111222
```

Obtain [the candidate receiver type] by repeatedly dereferencing the receiver expression's type, adding each type encountered to the list, then finally attempting an unsized coercion at the end, and adding the result type if that is successful. Then, for each candidate T, add &T and &mut T to the list immediately after T.

### quiz32

```rust
fn check(x: i32) -> bool {
    print!("{}", x);
    false
}

fn main() {
    match (1, 2) {
        (x, _) | (_, x) if check(x) => {
            print!("3")
        }
        _ => print!("4"),
    }
}

output:124
```

once per |-separated alternative in the match-arm

### quiz33

```rust
use std::ops::RangeFull;

trait Trait {
    fn method(&self) -> fn();
}

impl Trait for RangeFull {
    fn method(&self) -> fn() {
        print!("1");
        || print!("3")
    }
}

impl<F: FnOnce() -> T, T> Trait for F {
    fn method(&self) -> fn() {
        print!("2");
        || print!("4")
    }
}

fn main() {
    (|| .. .method())();
}
```

(|| .. .method())();等价于(|| ..),method()

### quiz34

```rust
fn d<T>(_f: T) {
    match std::mem::size_of::<T>() {
        0 => print!("0"),
        1 => print!("1"),
        _ => print!("2"),
    }
}

fn a<T>(f: fn(T)) {
    d(f);
}

fn main() {
    a(a::<u8>);
    d(a::<u8>);
}

output : 20
```

a::<u8>为ZST（quiz13）

Currently in Rust there is no syntax to express the type of a specific function, so they are always passed as a generic type parameter with a FnOnce, FnorFnMutbound. In error messages you might see function types appear in the formfn(T) -> U {fn_name}, but you can't use this syntax in code.

main第一次调用在64位系统中大小为8，第二次不涉及函数指针大小为0

### quiz35

```rust
macro_rules! x {
    ($n:expr) => {
        let a = X($n);
    };
}

struct X(u64);

impl Drop for X {
    fn drop(&mut self) {
        print!("{}", self.0);
    }
}

fn main() {
    let a = X(1);
    x!(2);
    print!("{}", a.0);
}

output:121
```

instead imagine hygiene as a way of assigning a color to each mention of the name of a local variable

以quiz代码为例

```rust
fn main() { let <font color = "Blue" >a</font> = X(1); let <font color = "Orange" >a</font> = X(2); print!("{}", <font color = "Blue" >a</font>.0); }
```

### quiz36

```rust
fn call(mut f: impl FnMut() + Copy) {
    f();
}

fn g(mut f: impl FnMut() + Copy) {
    f();
    call(f);
    f();
    call(f);
}

fn main() {
    let mut i = 0i32;
    g(move || {
        i += 1;
        print!("{}", i);
    });
}

output:1223
```
rust 1.26之后，所有捕获实现clone/copy，闭包会自动实现clone/copy

如果省略掉move关键字，编译器会报错

error[E0277]: the trait bound `&mut i32: Copy` is not satisfied in `{closure@src/main.rs:14:7: 14:9}`

并且编译器生成的闭包将通过可变引用而不是值捕获i，也不再有Copy impl

the relationship between move and non-move closures vs Fn and FnMut and FnOnce closures. These are two nearly-orthogonal things

move 与非 move 是关于编译器生成的闭包结构体的字段是否与原始类型相同的问题捕获变量的类型

Fn vs FnMut vs FnOnce 是关于编译器生成的闭包结构体的call方法是否有一个接收者。