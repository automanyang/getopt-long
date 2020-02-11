
## 命令行参数解析

### 命令行示例

```c
  +------------------------------------------------- Command name
  |      +------------------------------------------ Option
  |      |      +----------------------------------- Option argument
  |      |      |             +--------------------- Operands
  |      |      |             |
--+--   -+-  ---+---   -------+---------
fgrep   -f   patfile   foo.c bar.c baz.c
```

非常清楚，命令行参数由Command name，Option，Option argument 以及Operands组成。

Command name不用多说，就是程序的名称。
操作对象Operands又称nonoption argument。
重点就是这里的选项Option了，它是用来用来决定程序的行为，而选项参数Option argument是选项
Option所需要的信息。

### getopt()函数

getopt()函数将传递给mian()函数的argc, argv作为参数，同时接受字符串参数optstring，
optstring是由选项Option字母组成的字符串。

关于optstring的格式规范简单总结如下：
1. 单个字符，表示该选项Option不需要参数。
2. 单个字符后接一个冒号":"，表示该选项Option需要一个选项参数Option argument。选项参
   数Option argument可以紧跟在选项Option之后，或者以空格隔开。选项参数Option argument
   的首地址赋给optarg。
3. 单个字符后接两个冒号"::"，表示该选项Option的选项参数Option argument是可选的。当提供
   了Option argument时，必须紧跟Option之后，不能以空格隔开，否则getopt()会认为该选项
   Option没有选项参数Option argument，optarg赋值为NULL。相反，提供了选项参数
   Option argument，则optarg指向Option argument。

为了使用getopt()，我们需要在while循环中不断地调用直到其返回-1为止。每一次调用，当getopt()
找到一个有效的Option的时候就会返回这个Option字符，并设置几个全局变量。

getopt()设置的全局变量包括：
1. char *optarg：当匹配一个选项后，如果该选项带选项参数，则optarg指向选项参数字符串；若该
   选项不带选项参数，则optarg为NULL；若该选项的选项参数为可选时，optarg为NULL表明无选项参
   数，optarg不为NULL时则指向选项参数字符串。
2. int optind：下一个待处理元素在argv中的索引值。即下一次调用getopt的时候，从optind存储
   的位置处开始扫描选项。当getopt()返回-1后，optind是argv中第一个Operands的索引值。
   optind的初始值为1。
3. int opterr：opterr的值非0时，在getopt()遇到无法识别的选项，或者某个选项丢失选项参数的
   时候，getopt()会打印错误信息到标准错误输出。opterr值为0时，则不打印错误信息。
4. int optopt：在上述两种错误之一发生时，一般情况下getopt()会返回'?'，并且将optopt赋值为
   发生错误的选项。

### 说明：

使用getopt()时，会犯的错误无外乎有两个：
1. 无法识别的选项（Invalid option）
2. 和丢失选项参数（Missing option argument）

通常情况下，getopt()在发现这两个错误时，会打印相应的错误信息，并且返回字符"?" 。例如，遇见
无法识别的选项时会打印"invalid option"，发现丢失参数时打印"option requires an argument"。
但是当设置opterr为0时，则不会打印这些信息，因此为了便于发现错误，默认情况下，opterr都是非零值。

如果你想亲自处理这两种错误的话，应该怎么做呢？ 首先你要知道什么时候发生的错误是无法识别的选项，
什么时候发生的错误是丢失选项参数。如果像上面描述的那样，都是返回字符"?"的话，肯定是无法分辨出
的。有什么办法吗？

有！getopt()允许我们设置optstring的首字符为冒号":"，在这种情况下，当发生无法识别的选项错误
时getopt()返回字符"?"，当发生丢失选项参数错误时返回字符":"。这样我们就可以很轻松地分辨出错误
类型了，不过代价是getopt()不会再打印错误信息了，一切事物都由我们自己来处理了。

### getopt()扫描模式

关于getopt()的扫描模式，man文档说的不是很清楚，这里通过实例来阐释，尽量做到简单明了，一看即懂。

getopt()的默认模式扫描模式是这样的：getopt()从左到右按顺序扫描argv[1]到argv[argc-1]，并将
选项Option和选项参数Option argument按它们在命令行上出现的次序放到argv数组的左边，而那些
Operands则按它们出现的次序放在argv的后边。也就是说，getopt()在默认扫描模式下，会重新排序
argv数组。

来看个实例，假设我们调用`getopt(argc, argv, "ab:c:de::");`

从前面的知识点，我们很容易得出结论：选项a，d是不需要选项参数的，选项b，c需要选项参数，而选项e
的选项参数是可选的，即如果提供选项e的选项参数的话，那么选项参数必须紧跟选项e之后，不能以空格隔
开。

```c
#include <stdio.h>
#include <stdlib.h>
#include <getopt.h>

static int cnt = 1;

static void print(int optc, int argc, char *argv[], int optind)
{
    int i;

    printf("%02d: optc - '%c', argv: ", cnt++, optc);
    for (i = 0; i < argc; i++) {
        printf("%s ", argv[i]);
    }
    printf("---- optind = %d\n", optind);
}

int main(int argc, char *argv[])
{
    int optc;

    print('0', argc, argv, optind);

    while ((optc = getopt(argc, argv, ":ab:c:de::")) != -1) {
        print(optc, argc, argv, optind);
        switch (optc) {
            default:
                break;
        }
    }

    print('0', argc, argv, optind);

    exit(EXIT_SUCCESS);
}
```

以下是实际的运行情况：
```bash
xt@sf315:~/dev/cpp/gof/bin$ ./getopt operand1 -a operand2 -b barg operand3 -c carg operand4 -d operand5 operand6 -e operand7
01: optc - '=', argv: ./getopt operand1 -a operand2 -b barg operand3 -c carg operand4 -d operand5 operand6 -e operand7 - optind = 1, optarg = (null)
02: optc - 'a', argv: ./getopt operand1 -a operand2 -b barg operand3 -c carg operand4 -d operand5 operand6 -e operand7 - optind = 3, optarg = (null)
03: optc - 'b', argv: ./getopt -a operand1 operand2 -b barg operand3 -c carg operand4 -d operand5 operand6 -e operand7 - optind = 6, optarg = barg
04: optc - 'c', argv: ./getopt -a -b barg operand1 operand2 operand3 -c carg operand4 -d operand5 operand6 -e operand7 - optind = 9, optarg = carg
05: optc - 'd', argv: ./getopt -a -b barg -c carg operand1 operand2 operand3 operand4 -d operand5 operand6 -e operand7 - optind = 11, optarg = (null)
06: optc - 'e', argv: ./getopt -a -b barg -c carg -d operand1 operand2 operand3 operand4 operand5 operand6 -e operand7 - optind = 14, optarg = (null)
07: optc - '=', argv: ./getopt -a -b barg -c carg -d -e operand1 operand2 operand3 operand4 operand5 operand6 operand7 - optind = 8, optarg = (null)
```

### 代码说明

当然在真实情况下，一个程序很少需要这么多的Operands，这里只是为了更清楚地演示getopt()是如何
扫描命令行参数的。

扫描过程中，要时刻铭记optind是下一个待处理元素在argv中的索引，当遇到Operands的时候则跳过，
optind数值增加跳过的Operands个数。

好，现在我们根据这些规则，详细分析下刚刚程序的扫描过程：
- 第一行：即getopt()扫描重排序之前，可以看到optind的值默认被初始化为1。
- 第二行：getopt()首先从operand1开始扫描，发现operand1是Operands，则跳过，optind增加1
  等于2指向-a，继续扫描。扫描到-a时发现是有效选项，则optind增加1等于3指向operand2，然后返
  回选项a。
- 第三行：在继续扫描前，getopt()重新排序argv数组，将`-a`和`operand1`的位置互换。继续扫描，
  发现operand2是Operands，跳过，optind增加1等于4指向-b，继续扫描。发现-b是有效选项，因为
  选项b需要参数，因此把barg的首地址赋给optarg，optind增加2等于6指向operand3，返回选项b。
- 第四行：在继续扫描前，getopt()重新排序argv数组，将`-b barg`和`operand1 operand2`的
  位置互换。继续扫描，发现operand3是Operands，跳过，optind增加1等于7指向-c，继续扫描。扫
  描到-c是发现是有效的选项，因为选项c跟选项b一样，都需要参数，因此处理过程是一样的，把carg的
  首地址赋给optarg，optind增加2等于9指向operand4，返回选项c。
- 第五行：在继续扫描前，getopt()重新排序argv数组，将`-c carg`和`operand1 operand2 operand3`
  的位置互换。继续扫描，发现operand4是Operands，跳过，optind增加1等于10指向-d，
  继续扫描。扫描到-d时发现是有效选项，因为选项d不需要参数，因此直接optind增加1等于11指向
  operand5，返回选项d。
- 第六行：在继续扫描前，getopt()重新排序argv数组，将`-d`和`operand1 operand2 operand3`
 `operand4`的位置互换。继续扫描，发现operand5是Operands，跳过，optind增加1等于12指向
    operand6，继续扫描。扫描到operand6时发现依然是Operands，跳过，optind增加1等于13指向-e，
    继续扫描。扫描到-e时发现是有效选项，因为后面的operand7与-e之间有间隔，因此这里选项e没有参
    数。optind增加1等于14指向operand7，返回选项e。
- 第七行：在继续扫描前，getopt()重新排序argv数组，将`-e`和`operand1 operand2 operand3`
  `operand4 operand5 operand6`的位置互换。继续扫描，发现operand7是Operands，跳过，
  optind增加1等于15指向argv[argc]，即NULL。至此扫描完毕，getopt()重新设置optind为8，是
  其指向第一个Operands，即operand1，最后返回-1停止扫描。
