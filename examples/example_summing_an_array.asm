# int offset, val, s;
# int *addr;

main:
    ADDI t0, zero, 1     # int step_size = 1;
    ADDI t1, zero, 0     # int i = 0;
    ADDI t2, zero, array # int ref = &array;
    JAL zero, test       # goto test;

body:
    SLL t3, t1, t0       # offset = i << 1
    SLL t3, t3, t0       # offset = offset << 1; offset in t3 now contains i*4
    ADD t4, t3, t2       # addr = &array[offset]
    LW t4, 0(t4)         # val = *addr;
    LW t6, sum           # s = *sum;
    ADD t6, t6, t4       # s = s + val;
    SW t6, sum           # *sum = s;
    ADDI t1, t1, 1       # i++;

test:
    LW t6, n             # s = n;
    BLT t1, t6, body     # if (i < s) goto body;
    LW t6, sum           # s = sum;
    SW t6, 0x7FC(zero)   #// write s to stdout
    EBREAK               #// return main routine
    
n:     .word 4           # number of elements in n
array: .word 3, 4, 5, 6  # array to sum up
sum:   .word 0           # sum should finally be 18 == 0x12
