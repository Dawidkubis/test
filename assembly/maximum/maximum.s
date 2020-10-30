# PURPOSE: finds the maximum number of a set of data items.

# VARIABLES:
# %ebi - index of the data item
# %ebx - largest data item yet
# %eax - current data item

# MEMORY:
# data_items - contains the data items.

.code32
.section .data

data_items:
	.long 3, 67, 34, 223, 45, 75, 54, 34, 44, 33, 22, 11, 66

lenght: .long 12

.section .text

.globl _start

_start:

	movl $0, %edi # move 0 into index register
	movl data_items(, %edi, 4), %eax # load first word of data
	movl %eax, %ebx # fist item, therefore biggest yet

	start_loop:
	cmpl lenght, %eax # check for end
	je loop_exit # jump to exit if equal
	incl %edi # load next value
	movl data_items(, %edi, 4), %eax # and move it into eax
	cmpl %ebx, %eax # compare values
	jle start_loop # jump to beginning if isnt bigger

	movl %eax, %ebx # otherwise move value since its larger

	jmp start_loop # jump back (loop)

	loop_exit:

	# since we already have the biggest number found
	# in %ebx there is no need to move anything
	movl $1, %eax # sys call for exit
	int $0x80 # interrupt (quit)
