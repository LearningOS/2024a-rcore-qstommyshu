# Lab1

### 总结实现的功能：

1. 在`TaskControlBlock`里加入了`start_time`, 用于计算任务运行时间。
2. 将`TaskControlBlock`里的`TaskStatus`改为了`TaskInfo`, 用于储存任务的所需要的各种信息。
3. 增加了函数`update_current_syscall_times()`用于每次`syscall`时给每个任务的系统调用计数。
4. 增加了函数`get_current_task_info()`用于从外部获取`task_info`。

### 问答题:

1. 环境: rustsbi-qemu 0.1.1, qemu 9.0.0

   1. `ch2b_bad_address.rs`:
   2. `ch2b_bad_instructions.rs`: 使用S态的指令`sret`, 报错: `[kernel] IllegalInstruction in application, kernel killed it`
   3. `ch2b_bad_register.rs`:  使用S态的指令`sstatus`??
2. test
3. test
4. test
5. test
6. test
7. test

### 荣誉准则内容:

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > 代码遇到问题时问了助教🐉Andrew虾仁猪心，以及发到群里问群友帮忙debug
   >
2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > *无*
   >
3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。
4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。