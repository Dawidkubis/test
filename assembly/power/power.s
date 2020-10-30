# PURPOSE: creates a function for computing powers
# will compute:
# 2^3 + 5^2

.code32
.section .data
.section .text

.globl _start

_start:
	# 2^3
	pushl $2 # push second argument
	pushl $3 # push first argument
	call power # call the function
	addl $8, %esp # move stack pointer back
	# in order to overwrite arguments

	pushl %eax # save first answer before calling again

	# 5^2
	pushl $2 # push second argument
	pushl $5 # push first argument
	call power # call the function
	addl $8, %esp # move the stack pointer back

	popl %ebx # pop first answer into %ebx

	addl %eax, %ebx # add together
	# result is now in %ebx

	movl $1, %eax # exit system call (%ebx is returned)
	int $0x80

# PURPOSE: function for computing the value of a number
# raised to a power

# INPUT: 
# 	first argument - base number
# 	second argument - power

# NOTES: the power must be 1 or greater

# VARIABLES: 
#	%ebx - holds base number
#	%ecx - holds power
#	-4(%ebp) - holds current result
#	%eax - temporary storage
.type power, @function

power:
	pushl %ebp # save the old base pointer
	movl %esp, %ebp # make stack pointer the base pointer
	subl $4, %esp # get room for local storage

	movl 8(%ebp), %ebx # put first argument in %ebx
	movl 12(%ebp), %ecx # put second argument in %ecx

	movl %ebx, -4(%ebp) #store current result

	power_loop_start:
		cmpl $1, %ecx # if the power is 1, we are done
		je end_power
		movl -4(%ebp), %eax # move the current result into %eax
		imull %ebx, %eax # multiply current result by base number

		movl %eax, -4(%ebp) # store current result

		decl %ecx # decrease the power
		jmp power_loop_start # run for the next power
	
	end_power:
		movl -4(%ebp), %eax # return value goes into %eax
		movl %ebp, %esp # restore the stack pointer
		popl %ebp # restore the base pointer
		ret # return
