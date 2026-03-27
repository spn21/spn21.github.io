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

不需要强求占满整个屏幕，也不必强求所有东西塞进一个小空间里面

### Grids are overrated

不一定需要网格去辅助区域宽度，尝试将一些区域的宽度固定

### Relative sizing doesn't scale

放弃所有东西按照比例放大的想法

### Avoid ambiguous spacing

可以通过调整垂直、水平边距，将边界分隔开

用户不应该在理解用户界面花费更多精力

## 0x04 Designing Text

### Establish a type scale

没有体系的字体大小，会导致：

- 设计时候的不一致
- 减慢工作效率

选择尺寸：不需要把时间浪费在46px和48px之间

按照一个基准值（16px）以此类推（3：1、4：5、1：0.618）不过尽量避免使用小数尺寸使得调整变得困难

避免使用 em 单位来定义尺寸

``` markdown
If you give an element a font size of 1.25em (20px by default), inside of that
element 1em is now equal to 20px. That means that if you give one of the
nested elements a font size of .875em, the actual computed font size is
17.5px, not a value from your type scale
```

坚持使用 px 和 rem

### Use good fonts

对用户界面设计而言，最稳健的中性无衬线字体（例如 Helvetica），或者其他系统字体（用户比较熟悉）

优化易读性（标题的字体一般字间距较窄，避免使用高度较短的窄体字体用于主要用户界面文本）

选大众性的字体也不失为一种选择

平日里偷点理解、培养直觉

### Keep your line length in check

一搬来说字符数一行在45 - 75之间的范围更好，限制整体的内容区域

### Baseline, not center

设定一个假想线，让不同大小的字体在同一行，更干净

### Line-height is proportional

文本行间距过小，很同意让人注意力不集中，窄内容可以使用较短的行高（1.5），宽内容可以使用较宽的行高（可能到2）

字体较小时增加行间距很重要，文本换行后眼睛更容易找到下一行

行高与字体大小成反比

### Not every link needs a color

有些链接可以仅在鼠标悬停时改变颜色，也可以添加下划线，本质辅助工具，不应该让变颜色的链接突出注意力

### Align with readability in mind

长篇文本不需要居中对齐，会显得参差，短文本如标题、简短独立文本块居中比较好

数字可以选择右对齐，对于一组带小数点的数字，可以选择小数点对齐

采用断字避免文本两端对齐时可能会出现的间距问题

### Use letter-spacing effectively

设置文本样式时，文本粗细、颜色和线条粗细、高度，字母间距都是需要调整的，字母的大小写之间的间距在视觉上也需要注意

```
Lowercase letters have a lot of variety visually. Letters like n, v, and e fit
entirely within a typeface’s x-height, other letters like y, g, and p have
descenders that poke out below the baseline, and letters like b, f, and t have
ascenders that extend above.
```

全大写字母可以增加字母间距提高文本可读性，因为字母之间特征区别相对较少

## 0x05 Working with Color

### Ditch hex for HSL

HSL 0' 为红 120'为绿 240'为蓝

HSB 0% 为黑 100% 为白

HSB 饱和度为 100% 时，HSB亮度为 100%，相当于HSL的 100% 饱和度和 50% 亮度

### You need more colors than you think

灰色：文本、背景、面板、表单控件——界面之中的所有内容，需要的灰色可能比预料的更多

主色：大多数网站只需要一到两种颜色用于主要操作，导航栏元素之类，这种颜色也需要5-10种深浅不同的色调

强调色：使用强调色向用户传达不同的信息：醒目的颜色（黄色、粉色、青色）突出显示新功能，（红色）这种破坏性质强调状态，（黄色）表示警告信息

### Define your shades up front

提前预先定义一组固定的颜色供你选择，也是选择一种底色，颜色最深表示文字，浅色渲染背景

（选择9种由深到浅）

### Don't let lightness kill your saturation

同样的饱和度值在50% 时亮度调高会看起来比 90% 亮度时色彩更鲜艳

小技巧：通过旋转色调来改变亮度（120' 或 240'）

### Grey don't have to be grey

灰色可以通过调节色温来改变作用，灰色+ 蓝色看起来更冷，灰色+ 黄色看起来更温暖

为了保持色温恒定，需要提高饱和度，对于较浅或者较深的色调，不建议这样做

### Accessible doesn't have to mean ugly

通过增加对比度，保留彩色背景的白色文字

为了解决正文部分和辅助文本，可以通过增加对比度（将色调向某个方向旋转），更容易让文本便于阅读

### Don't rely on color alone

考虑色盲用户，通过其他方式传达信息（如添加图标变化来体现积极还是消极）

不同的颜色代表不同的部分对于色盲患者来说更好分辨，对于他们来说明区分暗难度比分辨明暗难度要大很多

*** 颜色是强化设计表达的内容，而不能作为唯一的方式 ***

## 0X06 Creating Depth

### Emulate a light source

模拟光源在现实中的位置，让平面显得更加立体，朝向天空颜色较浅，远离天空颜色较深

在用户界面想设计一个按钮让它感觉是凸起的，可以尝试上面的做法，添加一个略微垂直偏移的小型深色阴影框

内嵌显示的元素（文本输入框）可以添加一个略带阴影的深色小内嵌框，正向垂直偏移

这种可能会很耗时，不要刻意去求

### Use shadows to convey elevation

阴影体现深度

下拉菜单其位置略高于用户界面其他部分

大阴影适合对话框

阴影也需要一套系统，设计最大最小，然后填充颜色，中间部分阴影大小呈随线性变化

阴影也是提供视觉提示的一种方式，如在一个列表中选定时选中目标可以增加阴影，方便用户拖动

也可以做成一种按钮看着像是被按进界面一样

### Shadows can have two parts

更大、更柔和、垂直偏移量大，模糊半径较大，模拟物体在直射光照下投射的阴影（光源型）

更集中、更深、垂直偏移量更小，模糊半径较小，模拟物体下方的阴影区域

物体远离表面，阴影会越来越小、越来越暗，使用双阴影需要确保代表事物海拔较高

### Even flat designs can have depth

用色彩营造深度，较浅的物体感觉更近

另外一种传达深度的方法是使用短小、垂直的线条，不加模糊半径，不过牺牲了扁平化的美感

### Overlap elements to create layers

景深的效果之一是不同的物体相互重叠，彰显层次感

重叠元素也可以为较小的组件增加深度，对图像也同样适用

## 0x07 Working with Images

### Use good photos

糟糕的图片会毁掉其他所有的设计，好的照片是必须的

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