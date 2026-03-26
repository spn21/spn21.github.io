+++
title = "Note of Refactoring UI"
date = 2026-03-25

[extra]
cover_image = "/covers/first.jpg"
cover_sentence = " "
tags = ["Learning", "UI", "frontend"]
draft = true
+++

## 0x01 Starting from Scratch

### Start with a feature, not a layout

对于一个应用程序的设计，应该从它的实际功能开始，而不是界面布局

文档示例：

“搜索航班”功能：

- 出发城市的场地
- 目的地城市
- 出发日期栏
- 返回日期栏
- 返回日期字段
- 搜索按钮

（除此之外其他东西可能就不需要了.jpg）

### Detail comes later

其余的细节（如字体选择、图标等）是可能最后需要解决的问题，而不是开始需要解决的问题

开始时尽量克制想法细化的想法，可以采用 grayscale，运用间距、对比度和尺寸控制画面

low-fidelity 设计模式在于快速迭代，尽快地做出真正的东西，在做出决定后扔掉草图和线框图

### Don't design too much

在最初设计的抽象层面时，不需要考虑每一个功能（包括一些极端情况）

可以采用循环设计：设计一个简化版再考虑下一个要开发的功能，在用户界面中修复问题比提前设想极端情况要容易得多，不断迭代设计

不要一直陷于抽象设计中，今早构建实际的东西

同时，不要在设计中暗示还没有准备好实现的功能，不然会增加实际的工作量，没有总比有却没有实现好，顺利交付最重要

### Choose a personality

设计模式会体现一定的个性

（如衬线字体会营造一种优雅或经典的风格，而无衬线字体会有一种活泼俏皮的风格）

还有颜色：在实践中，只需要关注不同颜色给人的感觉就好（如蓝色——安全又熟悉，黄色——昂贵、精致）

边界半径：较小的圆角半径比较中性，较大的边框半径会让人感觉更加活泼，无边框半径则更显得严肃正式

*** 保持一致性更为重要，多种混合不如坚持其中一个 ***

语言文字也是界面设计的一部分，专业语气和私人语气的差别也很大

*** 取决于真正需要什么 *** 

*** 做目标受众的网站 ***

*** 不要从友商那里“借鉴”太多 ***

### Limit your choices

颜色字体的选择又很多，也很容易发生选择困难症，尽量限制自己的选择，而非微操大师（

尽量提前从有限的范围内去选择，学会在适当的范围内进行排除法

*** 将事情系统化 ***

- 字体大小
- 字体粗细
- 宽高
- 饱和度
- ...

***

## 0x02 Hierarchy is Everything

### Not all elements are equal

明白每个元素的重要性层次，凸显重要信息、弱化次要和第三级

### Size isn't everything

可以通过更改字体粗细颜色体现层级：

尽量使用两到三种颜色：

- 主要内容（文章标题等）使用深色
- 灰色区域用于显示辅助内容（文章日期）
- 辅助/第三级内容（版权声明），在页脚使用较浅的灰色

字体：

- 大多数字体正常粗细（400/500）
- 需要强调的文本使用较粗字体（600/700）
- 使用较细的字体弱化某些文本，不过尽量避免使用低于 400 的字体（小号字体较为难以阅读）

### Don't use grey text on colored backgrounds

文字颜色最好更接近背景色创造氛围感，选择一种色调相同的颜色，调整饱和度和亮度最好

### Emphasize by de-emphasizing

可以通过让一些元素更贴近背景颜色来相对突出更想要突出的元素

### Labels are a last resort

一些显而易见的东西（电话、邮箱）可以不需要添加标签，也可以通过辅助文字说明，而不是单纯直接的标签

可以通过缩小标签尺寸、降低对比度等弱化标签元素，使用不同粗细的字体突出重要的元素

如果用户需要在信息密集的页面上了解特定的信息（如在手机产品说明书中快速了解手机尺寸），可以通过加粗标签提醒用户，但是数据也不能过于次要，可以通过改变间距、颜色等突出数据

### Separate visual hierarchy from docunment hierarchy

可以考虑适当降低类似标签类型的标题的重要性，突出该标题下的内容，极端情况下可以选择隐藏

### Balance weight and contract

字体的粗细、图标的对比度、边框的宽窄

### Semantics are secondary

页面上的每个操作都有一定的占比，大多数页面可能只需要一个主要操作，几个次要操作，以及几个不常用的第三级操作

- 主要操作应该很明显，如纯色、高对比度
- 次要操作应该清晰明了，但不能过于显眼，降低对比度、改变轮廓样式等
- 三级操作应该易于发现且不引人注目，可以通过添加链接来操作

破坏性/严重性（如delete）操作，需要醒目提醒，尤其在与确认步骤结合时

## 0x03 Layout and Spacing

### Start with too much white space

最开始尽量让每个元素显得宽裕一点，但不能每个元素都宽裕显得松散，比较好的方法是给一个东西留出过多的空间，然后慢慢缩小移除，避免留白太多

如果设计目标是密集型用户界面，可能就需要让内容尽量显示在一个屏幕上

### Establish a spacing and sizing system

预先设定一些尺寸架构，而不是慢慢微调

线性刻度（如保证所有间距尺寸是4px的倍数）行不通，12px-16px跟500px-520px的对比也很大

尺寸差距尽量不小于25%

一个简单的方法：设定一个合理的基准值（如16px，可以被整除，也是所有主流网络浏览器的默认字体大小），建立一个体系，使用该值的因子和倍数

### You don't have to fill the whole screen

有多少用多少，而不是填满整个屏幕

也可以先选择设计移动端，再放大到电脑屏幕进行优化

如果设计目标在较窄的宽度下比较好，放在宽大的界面时显得不平衡，可以尝试拆分成专栏，增加列数而不仅仅是加宽（将辅助文本拆分成单独部分）

### Grids are overrated

### Relative sizing doesn't scale

### Avoid ambiguous spacing

## 0x04 Designing Text

### Establish a type scale

### Use good fonts

### Keep your line length in check

### Baseline, not center

### Line-height is proportional

### Not every link needs a color

### Align with readability in mind

### Use letter-spacing effectively

## 0x05 Working with Color

### Ditch hex for HSL

### You need more colors than you think

### Define your shades up front

### Don't let lightness kill your saturation

### Grey don't have to be grey

### Accessible doesn't have to mean ugly

### Don't rely on color alone

## 0X06 Creating Depth

### Emulate a light source

### Use shadows to convey elevation

### Shadows can have two parts

### Even flat designs can have depth

### Overlap elements to create layers

## 0x07 Working with Images

### Use good photos

### Text needs consistent contrast

### Everything has an intended size

### Beware user-uploaded content

## 0x08 Finishing Touches

### Supercharge the defaults

### Add color with accent borders

### Decorate your backgrounds

### Don't overlook empty states

### Use fewer borders

### Think outside the box