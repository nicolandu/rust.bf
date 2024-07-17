+++++ +++++ +++ // *13*

[
	>
	++
	>
	+++++
	>
	+++++++
	<<<
	-
] // *0* 26 65 91

> goto outer loop counter: a

[ // main loop
	
	
	// b *a* 65 91
	
	[ // move a 3 cells after 91
		>>>>>
		+
		<<<<<
		-
	] // b *0* 65 91 0 0 a
	
	>>>>>
	
	[ // copy a to original position; temp_a; and temp_a2
		<<<<<
		+
		>>>
		+
		>
		+
		>
		-
	] // b a 65 91 temp_a temp_a2 *0*
	
	<<
	
	[ // remove temp_a from 91 (opening square bracket; next char after Z) to get first letter a (start_a)
		<
		-
		>
		-
	] // b a 65 start_a *0* temp_a2
	
	>
	
	[ // output letters before break (temp_b is loop counter); return start_a back to 91 due to sum of (start_a := 91 minus a) and (a)
		<<
		.
		+
		>>
		-
	] // b a 65 91 0 *0*
	
	<<<<<
	
	[ // move b 2 cells after 91
		>>>>>
		+
		<<<<<
		-
	] // *0* a 65 91 0 b
	
	>>>>>
	
	[ // move b to original position and temp_b
		<<<<<
		+
		>>>>
		+
		>
		-
	] // b a 65 91 temp_b *0*
	
	<<<
	
	[ // move 65 2 cells after temp_b
		>>>>
		+
		<<<<
		-
	] // b a *0* 91 temp_b 0 65
	
	>>>>
	
	[ // move 65 (A) to original position and to b_start
		<<<<
		+
		>>>
		+
		>
		-
	] // b a 65 91 temp_b b_start *0*
	
	<<
	
	[ // output letters after break (temp_b is loop counter); b_start (initialized to 65 ASCII for 'A') as letter counter
		>
		.
		+
		<
		-
	] // b a 65 91 *0* (last_letter := a_start plus temp_a)
	
	>
	
	[-] // cleanup last_letter (decrement to 0)
	// b a 65 91 0 *0*
	
	+++++ +++++ // b a 65 91 0 *(newline := ASCII 10)*
	. // output newline
	[-] // cleanup newline (decrement to 0)
	
	<<<<<
	+ inc b
	>
	- dec a (outer loop counter)
]
