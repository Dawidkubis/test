# PURPOSE: finds the maximum number of a set of data items.

# VARIABLES:
# %ebi - index of the data item
# %ebx - largest data item yet
# %eax - current data item

# MEMORY:
# data_items - contains the data items.

.code32
.section .data
.section .text

.globl _start

_start:

	movl _start, %ebx 
	movl $1, %eax # sys call for exit
	int $0x80 # interrupt (quit)
