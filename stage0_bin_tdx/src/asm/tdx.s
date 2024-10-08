.align 16
.section .tdx.bootstrap, "ax"
.code32

.global ap_start
ap_start:
    hlt

.global _begin_of_tdx
_begin_of_tdx:
    # VCPU_INDEX is in esi
    cli

    andl $0x3f, %ebx # [6:0] GPAW
    movl %ebx, %ebp

    movl $gdt_desc_offset, %ebx
    lgdtl (%ebx)

    movl $0x00000023, %eax # SEC_DEFAULT_CR0
    movl %eax, %cr0

    ljmpl $cs32, $_tdx_32bit_long_mode_start

_tdx_32bit_long_mode_start:

    movl $0x640, %eax # SEC_DEFAULT_CR4
    movl %eax, %cr4

    movl $ds, %eax
    movl %eax, %ds
    movl %eax, %es
    movl %eax, %fs
    movl %eax, %gs
    movl %eax, %ss

    # Skip BFV check
    # Skip UEFI SEC setup

    # Note that no matter what the GPAW is, we only use 4-level
    # paging in stage0. Linux will set up its own PTs later.
    movl %cr4, %eax
    bts $0x05, %eax # PAE
    movl %eax, %cr4

    # page tables are set in the linker script
    movl $bios_pml4, %ecx
    movl %ecx, %cr3

    # No need for rdmsr/wrmsr

    # Protected mode + paging
    mov %cr0, %eax
    or $0x80000001, %eax # set PG
    mov %eax, %cr0

    # Reload CS, enter long mode, jump to 64-bit code.
    ljmpl $cs, $_tdx_64bit_start

.align 16
.code64
_tdx_64bit_start:
    # Clean up data segments.
    movw $ds, %ax
    movw %ax, %ds
    movw %ax, %es
    movw %ax, %fs
    movw %ax, %gs
    movw %ax, %ss

    # Park the APs
    test %esi, %esi
    jnz _park_ap_64bit

    # BSP re-creates a set of page table in ram_low
    # Clear BSS: base address goes to EDI, value (0) goes to EAX,
    # count goes into ECX. Page tables will be located in BSS
    movl $bss_start, %edi
    movl $bss_size,  %ecx
    xorl %eax, %eax
    rep stosb

    # Set the first entry of PML4 to point to PDPT (0..512GiB).
    movl ${pdpt}, %esi
    orl $3, %esi              # esi |= 3 (PRESENT and WRITABLE)
    movl %esi, ({pml4})        # set first half of PML4[0]

    # Set the first entry of PDPT to point to PD_0 (0..1GiB).
    movl ${pd_0}, %esi
    orl $3, %esi              # esi |= 3 (PRESENT and WRITABLE)
    movl %esi, ({pdpt})        # set first half of PDPT[0]

    # Set the fourth entry of PDPT to point to PD_3 (3..4GiB).
    movl ${pdpt}, %eax
    movl ${pd_3}, %esi
    orl $3, %esi              # esi |= 3 (PRESENT and WRITABLE)
    movl %esi, 24(%eax)        # set first half of PDPT[3], each entry is 8 bytes

    # Set the first entry of PD_0 to point to and identity mapped huge page (0..2MiB).
    movl $0x83, %esi           # esi = 0x0 | 131 (PRESENT and WRITABLE and HUGE_PAGE)
    movl %esi, ({pd_0})        # set first half of PD_0[0]

    # Set the last entry of PD_3 to point to an identity-mapped 2MiB huge page ((4GiB-2MiB)..4GiB).
    # This is where the firmware ROM image is mapped, so we don't make it writable.
    movl ${pd_3}, %eax
    movl $0xFFE00000, %esi     # address of 4GiB-2MiB
    orl $0x81, %esi           # esi |= 129 (PRESENT and HUGE_PAGE)
    movl %esi, 0xFF8(%eax)     # set first half of PML4[511], each entry is 8 bytes

    # Reload PML4 to use the writable PML4
    movq ${pml4}, %rax
    movq %rax, %cr3

    # Copy DATA from the ROM image (stored just after TEXT) to
    # the expected location. Source address goes to ESI, destination
    # goes to EDI, count goes to ECX.
    movl $text_end,   %esi
    movl $data_start, %edi
    movl $data_size,  %ecx
    rep movsd

    # Set up the stack. Stack now is in ram_low
    movl $stack_start, %esp
    push $0

    # Set GPAW
    movl %ebp, (GPAW)

    # ...and jump to Rust code.
    jmp rust64_start

_park_ap_64bit:
    leaq (AP_IN_64BIT_COUNT), %rcx
    lock incq (%rcx)

    # %esi stores the VCPU_INDEX
    movl %esi, %ebp # save the VCPU_INDEX

    movq $1, %rax # TDCALL_TDINFO
    tdcall

    # R8  [31:0]  NUM_VCPUS
    #     [63:32] MAX_VCPUS
    # R9  [31:0]  VCPU_INDEX

_inner_loop:
    # On entering the inner loop, APs are using the hard-coded page
    # tables from ROM. Before wake up the aps we need to reload cr3
    # for APs.

    # Finally we will need to call ap_wakeup_vector with
    # VCPU_INDEX as the first argument like
    # fn ap_wakeup_vector(vcpu_index: u64);

    # movq $r9, $rdi
    # call ap_wakeup_vector

    pause

    jmp _inner_loop

