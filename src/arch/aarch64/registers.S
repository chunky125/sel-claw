
.global _sel_claw_write_tpidr_el0
_sel_claw_write_tpidr_el0:
	msr	tpidr_el0, x0
	ret

.global _sel_claw_read_tpidr_el0
_sel_claw_read_tpidr_el0:
	mrs 	x0, tpidr_el0
	ret



