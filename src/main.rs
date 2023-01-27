#![feature(portable_simd)]
#![allow(dead_code)]
use duration_human::DurationHuman;
use itertools::Itertools;
use std::hint::black_box;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Instant;
use tinyvec::ArrayVec;

// #[inline(never)]
// fn mpsc_test_big_chunked_10x_with_iteration(
//     tx: &Sender<[f32; 100]>,
//     rx: &mut Receiver<[f32; 100]>,
// ) {
//     let array = [2f32; 100];
//     for i in black_box(0..100) {
//         let tx_send_data = array.iter().map(|v| black_box(black_box(*v) * 2.));
//         let v: [f32; 100] = tx_send_data.collect_vec().try_into().unwrap();
//         tx.send(black_box(v)).unwrap();
//     }
//     for i in black_box(0..100) {
//         for s in black_box(rx.recv().unwrap()) {
//             black_box(s);
//         }
//     }
// }

// .section .text.exobody_performance_tests::mpsc,"ax",@progbits
//         .p2align        4, 0x90
//         .type   exobody_performance_tests::mpsc,@function
// exobody_performance_tests::mpsc:
//
//         .cfi_startproc
//         .cfi_personality 155, DW.ref.rust_eh_personality
//         .cfi_lsda 27, .Lexception11
//         push rbp
//         .cfi_def_cfa_offset 16
//         .cfi_offset rbp, -16
//         mov rbp, rsp
//         .cfi_def_cfa_register rbp
//
//         push r15
//         push r14
//         push r13
//         push r12
//         push rbx
//         and rsp, -128
//         sub rsp, 1024
//         .cfi_offset rbx, -56
//         .cfi_offset r12, -48
//         .cfi_offset r13, -40
//         .cfi_offset r14, -32
//         .cfi_offset r15, -24
//         xorps xmm0, xmm0
//         movaps xmmword ptr [rsp + 640], xmm0
//         movaps xmmword ptr [rsp + 768], xmm0
//
//         lea rdi, [rsp + 128]
//         lea rsi, [rsp + 640]
//
//         mov edx, 256
//         call qword ptr [rip + memcpy@GOTPCREL]
//         mov dword ptr [rsp + 384], 0
//         mov byte ptr [rsp + 388], 0
//         mov qword ptr [rsp + 392], 0
//         mov qword ptr [rsp + 400], 8
//         xorps xmm0, xmm0
//         movups xmmword ptr [rsp + 408], xmm0
//         mov qword ptr [rsp + 424], 8
//         mov qword ptr [rsp + 432], 0
//         mov byte ptr [rsp + 440], 1
//         movdqa xmm0, xmmword ptr [rip + .LCPI42_0]
//         movdqa xmmword ptr [rsp + 512], xmm0
//         mov byte ptr [rsp + 528], 0
//
//         mov edi, 512
//         mov esi, 128
//         call qword ptr [rip + __rust_alloc@GOTPCREL]
//
//         test rax, rax
//         je .LBB42_318
//
//         mov r13, rax
//         lea rbx, [rsp + 128]
//         mov edx, 512
//         mov rdi, rax
//         mov rsi, rbx
//         call qword ptr [rip + memcpy@GOTPCREL]
//
//         mov qword ptr [rsp + 112], 1
//         mov qword ptr [rsp + 120], r13
//         mov qword ptr [rsp + 80], 1
//         mov qword ptr [rsp + 88], r13
//
//         mov dword ptr [rsp + 128], 1084227584
//         mov qword ptr [rsp + 104], rbx
//         #APP
//         #NO_APP
//         movd xmm0, dword ptr [rsp + 128]
//
//         movd dword ptr [rsp + 24], xmm0
//
//         call qword ptr [rip + <std::sync::mpmc::zero::ZeroToken as core::default::Default>::default@GOTPCREL]
//
//         call qword ptr [rip + std::sync::mpmc::utils::Backoff::new@GOTPCREL]
//
//         mov r12, qword ptr [r13 + 128]
//
//         mov rbx, qword ptr [r13 + 136]
//
//         test r12b, 1
//         jne .LBB42_317
//
//         mov r14d, eax
//         xor eax, eax
//         mov qword ptr [rsp + 8], rax
//         jmp .LBB42_7
//
//         .p2align        4, 0x90
// .LBB42_5:
//         cmp r14d, 7
//         adc r14d, 0
//
// .LBB42_6:
//         mov r12, qword ptr [r13 + 128]
//         mov rbx, qword ptr [r13 + 136]
//
//         test r12b, 1
//         jne .LBB42_36
//
// .LBB42_7:
//         mov r15d, r12d
//         shr r15d
//         and r15d, 31
//
//         cmp r15d, 31
//         jne .LBB42_15
//
//         cmp r14d, 7
//         jae .LBB42_27
//
//         mov ecx, r14d
//         imul ecx, ecx
//
//         test ecx, ecx
//
//         je .LBB42_29
//
//         lea edx, [rcx - 1]
//         mov eax, ecx
//         and eax, 7
//         cmp edx, 7
//         jb .LBB42_13
//
//         and ecx, -8
//
//         .p2align        4, 0x90
// .LBB42_12:
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         add ecx, -8
//         jne .LBB42_12
//
// .LBB42_13:
//         test eax, eax
//         je .LBB42_28
//
//         .p2align        4, 0x90
// .LBB42_14:
//         pause
//
//         dec eax
//         jne .LBB42_14
//         jmp .LBB42_28
//
//         .p2align        4, 0x90
// .LBB42_15:
//         cmp r15d, 30
//         jne .LBB42_19
//
//         cmp qword ptr [rsp + 8], 0
//         jne .LBB42_19
//
//         mov edi, 504
//         mov esi, 8
//         call qword ptr [rip + __rust_alloc@GOTPCREL]
//
//         test rax, rax
//         je .LBB42_315
//
//         mov edx, 504
//         mov qword ptr [rsp + 8], rax
//         mov rdi, rax
//         xor esi, esi
//         call qword ptr [rip + memset@GOTPCREL]
//
// .LBB42_19:
//         test rbx, rbx
//         je .LBB42_30
//
//         lea rcx, [r12 + 2]
//
//         mov rax, r12
//         lock cmpxchg    qword ptr [r13 + 128], rcx
//
//         je .LBB42_190
//
// .LBB42_21:
//         cmp r14d, 6
//         mov ecx, 6
//         cmovb ecx, r14d
//
//         imul ecx, ecx
//
//         test ecx, ecx
//
//         je .LBB42_5
//
//         lea edx, [rcx - 1]
//         mov eax, ecx
//         and eax, 7
//         cmp edx, 7
//         jb .LBB42_25
//
//         and ecx, -8
//
//         .p2align        4, 0x90
// .LBB42_24:
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         add ecx, -8
//         jne .LBB42_24
//
// .LBB42_25:
//         test eax, eax
//         je .LBB42_5
//
//         .p2align        4, 0x90
// .LBB42_26:
//         pause
//
//         dec eax
//         jne .LBB42_26
//         jmp .LBB42_5
//
//         .p2align        4, 0x90
// .LBB42_27:
//
//         call qword ptr [rip + std::thread::yield_now@GOTPCREL]
//
// .LBB42_28:
//         cmp r14d, 10
//         ja .LBB42_6
//
// .LBB42_29:
//         inc r14d
//         jmp .LBB42_6
//
// .LBB42_30:
//         mov edi, 504
//         mov esi, 8
//         call qword ptr [rip + __rust_alloc@GOTPCREL]
//
//         test rax, rax
//         je .LBB42_316
//
//         mov rbx, rax
//
//         mov edx, 504
//         mov rdi, rax
//         xor esi, esi
//         call qword ptr [rip + memset@GOTPCREL]
//
//         xor eax, eax
//         lock cmpxchg    qword ptr [r13 + 136], rbx
//
//         jne .LBB42_33
//
//         mov qword ptr [r13 + 8], rbx
//
//         lea rcx, [r12 + 2]
//
//         mov rax, r12
//         lock cmpxchg    qword ptr [r13 + 128], rcx
//
//         jne .LBB42_21
//         jmp .LBB42_190
//
// .LBB42_33:
//         mov rdi, qword ptr [rsp + 8]
//
//         test rdi, rdi
//         je .LBB42_35
//
//         mov esi, 504
//         mov edx, 8
//         call qword ptr [rip + __rust_dealloc@GOTPCREL]
//
// .LBB42_35:
//         mov qword ptr [rsp + 8], rbx
//         jmp .LBB42_6
//
// .LBB42_36:
//         xor ebx, ebx
//
//         xor r15d, r15d
//
// .LBB42_37:
//         mov rdi, qword ptr [rsp + 8]
//
//         test rdi, rdi
//         je .LBB42_39
//
//         mov esi, 504
//         mov edx, 8
//         call qword ptr [rip + __rust_dealloc@GOTPCREL]
//
// .LBB42_39:
//         test rbx, rbx
//         je .LBB42_317
//
// .LBB42_40:
//         shl r15, 4
//
//         movd xmm0, dword ptr [rsp + 24]
//
//         movd dword ptr [rbx + r15 + 8], xmm0
//
//         lock or qword ptr [rbx + r15], 1
//
//         add r13, 256
//
//         mov rdi, r13
//         call std::sync::mpmc::waker::SyncWaker::notify
//
//         mov rax, qword ptr [rsp + 80]
//         mov rcx, qword ptr [rsp + 88]
//
//         mov qword ptr [rsp + 8], rcx
//
//         test rax, rax
//         je .LBB42_102
//
//         cmp eax, 1
//         jne .LBB42_161
//
//         mov dword ptr [rsp + 56], 1000000000
//
//         call qword ptr [rip + <std::sync::mpmc::zero::ZeroToken as core::default::Default>::default@GOTPCREL]
//
//         pxor xmm0, xmm0
//         movdqa xmmword ptr [rsp + 144], xmm0
//         movdqa xmmword ptr [rsp + 128], xmm0
//         mov qword ptr [rsp + 160], rax
//         mov r13, qword ptr [rip + std::sync::mpmc::context::Context::with::CONTEXT::__getit::__KEY@GOTTPOFF]
//         lea rax, [rsp + 32]
//         mov qword ptr [rsp + 96], rax
//
//         .p2align        4, 0x90
// .LBB42_45:
//
//         call qword ptr [rip + std::sync::mpmc::utils::Backoff::new@GOTPCREL]
//
//         mov r14d, eax
//         jmp .LBB42_48
//
//         .p2align        4, 0x90
// .LBB42_47:
//         inc r14d
//
// .LBB42_48:
//         mov ebx, r14d
//         imul ebx, ebx
//         lea eax, [rbx - 1]
//         mov dword ptr [rsp + 16], eax
//         mov eax, ebx
//         and eax, 7
//         mov dword ptr [rsp + 24], eax
//         mov eax, ebx
//         and eax, -8
//         mov dword ptr [rsp + 76], eax
//         jmp .LBB42_51
//
//         .p2align        4, 0x90
// .LBB42_49:
//
//         call qword ptr [rip + std::thread::yield_now@GOTPCREL]
//
// .LBB42_50:
//         cmp r14d, 10
//         jbe .LBB42_47
//
// .LBB42_51:
//         mov rcx, qword ptr [rsp + 8]
//         mov rax, qword ptr [rcx]
//
//         mov r12, qword ptr [rcx + 8]
//
//         mov rcx, rax
//         shr rcx
//         mov r15d, ecx
//         and r15d, 31
//
//         cmp r15, 31
//         jne .LBB42_60
//
//         cmp r14d, 7
//
//         jae .LBB42_49
//
//         test ebx, ebx
//
//         je .LBB42_47
//
//         cmp dword ptr [rsp + 16], 7
//         jb .LBB42_57
//
//         mov eax, dword ptr [rsp + 76]
//
//         .p2align        4, 0x90
// .LBB42_56:
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         add eax, -8
//         jne .LBB42_56
//
// .LBB42_57:
//         test bl, 7
//         je .LBB42_50
//
//         mov eax, dword ptr [rsp + 24]
//
//         .p2align        4, 0x90
// .LBB42_59:
//         pause
//
//         dec eax
//         jne .LBB42_59
//         jmp .LBB42_50
//
//         .p2align        4, 0x90
// .LBB42_60:
//         mov rdi, r13
//         lea r13, [rax + 2]
//
//         test al, 1
//         jne .LBB42_63
//
//         mfence
//
//         mov rdx, qword ptr [rsp + 8]
//
//         mov rdx, qword ptr [rdx + 128]
//
//         mov rsi, rdx
//         shr rsi
//         cmp rcx, rsi
//         je .LBB42_83
//
//         xor rdx, rax
//
//         xor ecx, ecx
//         cmp rdx, 64
//         setae cl
//         or r13, rcx
//
// .LBB42_63:
//         mov rcx, r12
//         test r12, r12
//         jne .LBB42_75
//
//         cmp r14d, 7
//
//         mov r13, rdi
//
//         jae .LBB42_72
//
//         test ebx, ebx
//
//         je .LBB42_74
//
//         cmp dword ptr [rsp + 16], 7
//         jb .LBB42_69
//
//         mov eax, dword ptr [rsp + 76]
//
//         .p2align        4, 0x90
// .LBB42_68:
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         add eax, -8
//         jne .LBB42_68
//
// .LBB42_69:
//         cmp dword ptr [rsp + 24], 0
//         je .LBB42_73
//
//         mov eax, dword ptr [rsp + 24]
//
//         .p2align        4, 0x90
// .LBB42_71:
//         pause
//
//         dec eax
//         jne .LBB42_71
//         jmp .LBB42_73
//
//         .p2align        4, 0x90
// .LBB42_72:
//
//         call qword ptr [rip + std::thread::yield_now@GOTPCREL]
//
// .LBB42_73:
//         cmp r14d, 10
//         ja .LBB42_51
//
// .LBB42_74:
//         inc r14d
//         jmp .LBB42_48
//
//         .p2align        4, 0x90
// .LBB42_75:
//         mov rcx, qword ptr [rsp + 8]
//
//         lock cmpxchg    qword ptr [rcx], r13
//
//         je .LBB42_198
//
//         cmp r14d, 6
//         mov ecx, 6
//
//         cmovb ecx, r14d
//
//         imul ecx, ecx
//
//         test ecx, ecx
//         mov r13, rdi
//
//         je .LBB42_82
//
//         lea edx, [rcx - 1]
//         mov eax, ecx
//         and eax, 7
//         cmp edx, 7
//         jb .LBB42_80
//
//         and ecx, -8
//
//         .p2align        4, 0x90
// .LBB42_79:
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         add ecx, -8
//         jne .LBB42_79
//
// .LBB42_80:
//         test eax, eax
//         je .LBB42_82
//
//         .p2align        4, 0x90
// .LBB42_81:
//         pause
//
//         dec eax
//         jne .LBB42_81
//
// .LBB42_82:
//         cmp r14d, 7
//         adc r14d, 0
//
//         jmp .LBB42_48
//
//         .p2align        4, 0x90
// .LBB42_83:
//         test dl, 1
//         jne .LBB42_216
//
//         mov r14d, dword ptr [rsp + 56]
//         cmp r14d, 1000000000
//         mov r13, rdi
//         je .LBB42_87
//
//         mov rbx, qword ptr [rsp + 48]
//
//         call qword ptr [rip + std::time::Instant::now@GOTPCREL]
//
//         xor ecx, ecx
//         cmp rax, rbx
//         setne cl
//         mov eax, 255
//         cmovl ecx, eax
//
//         cmp edx, r14d
//         mov eax, 0
//         sbb eax, eax
//         test cl, cl
//         cmovne eax, ecx
//
//         cmp al, 2
//         jb .LBB42_314
//
// .LBB42_87:
//         lea rax, [rsp + 128]
//         mov qword ptr [rsp + 640], rax
//         mov rax, qword ptr [rsp + 8]
//         mov qword ptr [rsp + 648], rax
//         lea rax, [rsp + 48]
//         mov qword ptr [rsp + 656], rax
//
//         cmp qword ptr fs:[r13], 0
//
//         je .LBB42_89
//
//         mov rax, qword ptr fs:[0]
//         lea r14, [rax + r13]
//         add r14, 8
//         jmp .LBB42_91
//
//         .p2align        4, 0x90
// .LBB42_89:
//
//         xor edi, edi
//         call std::thread::local::fast::Key<T>::try_initialize
//
//         mov r14, rax
//
//         test rax, rax
//
//         je .LBB42_98
//
// .LBB42_91:
//         mov rbx, qword ptr [r14]
//
//         mov qword ptr [r14], 0
//
//         test rbx, rbx
//         je .LBB42_96
//
//         mov qword ptr [rsp + 32], rbx
//
//         mov qword ptr [rbx + 16], 0
//
//         mov qword ptr [rbx + 24], 0
//
//         lea rdi, [rsp + 640]
//         lea rsi, [rsp + 32]
//         call std::sync::mpmc::context::Context::with::{{closure}}
//
//         mov rdi, qword ptr [r14]
//
//         mov qword ptr [r14], rbx
//
//         test rdi, rdi
//         je .LBB42_45
//
//         lock dec        qword ptr [rdi]
//
//         jne .LBB42_45
//
//         #MEMBARRIER
//         call alloc::sync::Arc<T>::drop_slow
//
//         jmp .LBB42_45
//
//         .p2align        4, 0x90
// .LBB42_96:
//
//         call qword ptr [rip + std::sync::mpmc::context::Context::new@GOTPCREL]
//
//         mov rbx, rax
//         mov qword ptr [rsp + 32], rax
//
//         lea rdi, [rsp + 640]
//         lea rsi, [rsp + 32]
//         call std::sync::mpmc::context::Context::with::{{closure}}
//
//         jmp .LBB42_100
//
// .LBB42_98:
//         call qword ptr [rip + std::sync::mpmc::context::Context::new@GOTPCREL]
//
//         mov rbx, rax
//         mov qword ptr [rsp + 32], rax
//
//         lea rdi, [rsp + 640]
//         lea rsi, [rsp + 32]
//         call std::sync::mpmc::context::Context::with::{{closure}}
//
// .LBB42_100:
//         lock dec        qword ptr [rbx]
//         jne .LBB42_45
//
//         #MEMBARRIER
//         mov rdi, qword ptr [rsp + 32]
//         call alloc::sync::Arc<T>::drop_slow
//         jmp .LBB42_45
//
// .LBB42_102:
//         mov dword ptr [rsp + 56], 1000000000
//
//         call qword ptr [rip + <std::sync::mpmc::zero::ZeroToken as core::default::Default>::default@GOTPCREL]
//
//         pxor xmm0, xmm0
//         movdqa xmmword ptr [rsp + 144], xmm0
//         movdqa xmmword ptr [rsp + 128], xmm0
//         mov qword ptr [rsp + 160], rax
//         mov rcx, qword ptr [rip + std::sync::mpmc::utils::Backoff::new@GOTPCREL]
//         lea rax, [rsp + 32]
//         mov qword ptr [rsp + 16], rax
//
// .LBB42_104:
//         call rcx
//
//         mov r13d, eax
//         jmp .LBB42_107
//
//         .p2align        4, 0x90
// .LBB42_106:
//         cmp r13d, 7
//         adc r13d, 0
//
// .LBB42_107:
//         mov r14d, r13d
//         imul r14d, r14d
//         lea eax, [r14 - 1]
//         mov dword ptr [rsp + 24], eax
//         mov r15d, r14d
//         and r15d, 7
//         mov r12d, r14d
//         and r12d, -8
//         jmp .LBB42_110
//
//         .p2align        4, 0x90
// .LBB42_108:
//
//         call qword ptr [rip + std::thread::yield_now@GOTPCREL]
//
// .LBB42_109:
//         cmp r13d, 10
//         jbe .LBB42_120
//
// .LBB42_110:
//         mov rax, qword ptr [rsp + 8]
//         mov rcx, qword ptr [rax]
//
//         mov rdi, qword ptr [rax + 416]
//         dec rdi
//         and rdi, rcx
//
//         mov r8, qword ptr [rax + 384]
//
//         mov rax, qword ptr [rax + 408]
//
//         mov rsi, rdi
//         shl rsi, 4
//
//         mov rbx, qword ptr [r8 + rsi]
//
//         lea rdx, [rcx + 1]
//         cmp rdx, rbx
//         je .LBB42_121
//
//         cmp rbx, rcx
//         je .LBB42_130
//
//         cmp r13d, 7
//
//         jae .LBB42_108
//
//         test r14d, r14d
//
//         je .LBB42_120
//
//         cmp dword ptr [rsp + 24], 7
//         jb .LBB42_117
//
//         mov eax, r12d
//
//         .p2align        4, 0x90
// .LBB42_116:
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         add eax, -8
//         jne .LBB42_116
//
// .LBB42_117:
//         test r15d, r15d
//         je .LBB42_109
//
//         mov eax, r15d
//
//         .p2align        4, 0x90
// .LBB42_119:
//         pause
//
//         dec eax
//         jne .LBB42_119
//         jmp .LBB42_109
//
//         .p2align        4, 0x90
// .LBB42_120:
//         inc r13d
//         jmp .LBB42_107
//
//         .p2align        4, 0x90
// .LBB42_121:
//         inc rdi
//
//         mov rdx, qword ptr [rsp + 8]
//         cmp rdi, qword ptr [rdx + 400]
//         jb .LBB42_123
//
//         neg rax
//         and rax, rcx
//
//         add rax, qword ptr [rdx + 408]
//
//         mov rbx, rax
//
// .LBB42_123:
//         mov rax, rcx
//         lock cmpxchg    qword ptr [rdx], rbx
//
//         je .LBB42_188
//
//         cmp r13d, 6
//         mov ecx, 6
//
//         cmovb ecx, r13d
//
//         imul ecx, ecx
//
//         test ecx, ecx
//
//         je .LBB42_106
//
//         lea edx, [rcx - 1]
//
//         mov eax, ecx
//         and eax, 7
//         cmp edx, 7
//         jb .LBB42_128
//
//         and ecx, -8
//
//         .p2align        4, 0x90
// .LBB42_127:
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         add ecx, -8
//         jne .LBB42_127
//
// .LBB42_128:
//         test eax, eax
//         je .LBB42_106
//
//         .p2align        4, 0x90
// .LBB42_129:
//         pause
//
//         dec eax
//         jne .LBB42_129
//         jmp .LBB42_106
//
//         .p2align        4, 0x90
// .LBB42_130:
//         mfence
//
//         mov rdx, qword ptr [rsp + 8]
//
//         mov rax, qword ptr [rdx + 128]
//
//         mov rdx, qword ptr [rdx + 416]
//         mov rsi, rdx
//         not rsi
//         and rsi, rax
//         cmp rsi, rcx
//         je .LBB42_137
//
//         cmp r13d, 6
//         mov ecx, 6
//
//         cmovb ecx, r13d
//
//         imul ecx, ecx
//
//         test ecx, ecx
//
//         je .LBB42_106
//
//         lea edx, [rcx - 1]
//         mov eax, ecx
//
//         and eax, 7
//         cmp edx, 7
//         jb .LBB42_135
//
//         and ecx, -8
//
//         .p2align        4, 0x90
// .LBB42_134:
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         add ecx, -8
//         jne .LBB42_134
//
// .LBB42_135:
//         test eax, eax
//         je .LBB42_106
//
//         .p2align        4, 0x90
// .LBB42_136:
//         pause
//
//         dec eax
//         jne .LBB42_136
//         jmp .LBB42_106
//
//         .p2align        4, 0x90
// .LBB42_137:
//         test rdx, rax
//         jne .LBB42_313
//
//         mov r14d, dword ptr [rsp + 56]
//         cmp r14d, 1000000000
//         mov r15, qword ptr [rip + std::sync::mpmc::context::Context::with::CONTEXT::__getit::__KEY@GOTTPOFF]
//         je .LBB42_141
//
//         mov rbx, qword ptr [rsp + 48]
//
//         call qword ptr [rip + std::time::Instant::now@GOTPCREL]
//
//         xor ecx, ecx
//         cmp rax, rbx
//         setne cl
//         mov eax, 255
//         cmovl ecx, eax
//
//         cmp edx, r14d
//         mov eax, 0
//         sbb eax, eax
//         test cl, cl
//         cmovne eax, ecx
//
//         cmp al, 2
//         jb .LBB42_314
//
// .LBB42_141:
//         lea rax, [rsp + 128]
//         mov qword ptr [rsp + 640], rax
//         mov rax, qword ptr [rsp + 8]
//         mov qword ptr [rsp + 648], rax
//         lea rax, [rsp + 48]
//         mov qword ptr [rsp + 656], rax
//
//         cmp qword ptr fs:[r15], 0
//
//         je .LBB42_143
//
//         mov rax, qword ptr fs:[0]
//         lea r14, [rax + r15]
//         add r14, 8
//         jmp .LBB42_145
//
// .LBB42_143:
//         xor edi, edi
//         call std::thread::local::fast::Key<T>::try_initialize
//
//         mov r14, rax
//
//         test rax, rax
//
//         je .LBB42_156
//
// .LBB42_145:
//         mov rbx, qword ptr [r14]
//
//         mov qword ptr [r14], 0
//
//         test rbx, rbx
//         je .LBB42_151
//
//         mov qword ptr [rsp + 32], rbx
//
//         mov qword ptr [rbx + 16], 0
//
//         mov qword ptr [rbx + 24], 0
//
//         lea rdi, [rsp + 640]
//         lea rsi, [rsp + 32]
//         call std::sync::mpmc::context::Context::with::{{closure}}
//
//         mov rdi, qword ptr [r14]
//
//         mov qword ptr [r14], rbx
//
//         test rdi, rdi
//         je .LBB42_150
//
//         lock dec        qword ptr [rdi]
//
//         jne .LBB42_150
//
//         #MEMBARRIER
//         call alloc::sync::Arc<T>::drop_slow
//
// .LBB42_150:
//         mov rcx, qword ptr [rip + std::sync::mpmc::utils::Backoff::new@GOTPCREL]
//         jmp .LBB42_104
//
// .LBB42_151:
//         call qword ptr [rip + std::sync::mpmc::context::Context::new@GOTPCREL]
//
//         mov rbx, rax
//         mov qword ptr [rsp + 32], rax
//
//         lea rdi, [rsp + 640]
//         lea rsi, [rsp + 32]
//         call std::sync::mpmc::context::Context::with::{{closure}}
//
//         lock dec        qword ptr [rbx]
//
//         jne .LBB42_155
//
//         #MEMBARRIER
//         mov rdi, qword ptr [rsp + 32]
//         call alloc::sync::Arc<T>::drop_slow
//
// .LBB42_155:
//         mov rcx, qword ptr [rip + std::sync::mpmc::utils::Backoff::new@GOTPCREL]
//         jmp .LBB42_104
//
// .LBB42_156:
//         call qword ptr [rip + std::sync::mpmc::context::Context::new@GOTPCREL]
//
//         mov rbx, rax
//         mov qword ptr [rsp + 32], rax
//
//         lea rdi, [rsp + 640]
//         lea rsi, [rsp + 32]
//         call std::sync::mpmc::context::Context::with::{{closure}}
//
//         lock dec        qword ptr [rbx]
//
//         jne .LBB42_160
//
//         #MEMBARRIER
//         mov rdi, qword ptr [rsp + 32]
//         call alloc::sync::Arc<T>::drop_slow
//
// .LBB42_160:
//         mov rcx, qword ptr [rip + std::sync::mpmc::utils::Backoff::new@GOTPCREL]
//         jmp .LBB42_104
// .LBB42_161:
//
//         mov dword ptr [rsp + 40], 1000000000
//
//         call qword ptr [rip + <std::sync::mpmc::zero::ZeroToken as core::default::Default>::default@GOTPCREL]
//
//         pxor xmm0, xmm0
//         movdqa xmmword ptr [rsp + 656], xmm0
//         movdqa xmmword ptr [rsp + 640], xmm0
//         mov qword ptr [rsp + 672], rax
//         mov ecx, 1
//
//         xor eax, eax
//         mov rdx, qword ptr [rsp + 8]
//         lock cmpxchg    dword ptr [rdx], ecx
//
//         je .LBB42_164
//
//         mov rdi, qword ptr [rsp + 8]
//         call qword ptr [rip + std::sys::unix::locks::futex_mutex::Mutex::lock_contended@GOTPCREL]
//
// .LBB42_164:
//         mov rax, qword ptr [rip + std::panicking::panic_count::GLOBAL_PANIC_COUNT@GOTPCREL]
//
//         mov rax, qword ptr [rax]
//
//         shl rax, 1
//         test rax, rax
//         je .LBB42_212
//
//         call qword ptr [rip + std::panicking::panic_count::is_zero_slow_path@GOTPCREL]
//
//         xor al, 1
//         mov dword ptr [rsp + 16], eax
//
//         mov rcx, qword ptr [rsp + 8]
//
//         movzx eax, byte ptr [rcx + 4]
//
//         test al, al
//
//         jne .LBB42_213
//
// .LBB42_167:
//         mov rax, qword ptr [rcx + 24]
//
//         test rax, rax
//         je .LBB42_193
//
//         mov rcx, qword ptr [rsp + 8]
//
//         mov r14, qword ptr [rcx + 16]
//
//         shl rax, 3
//
//         lea rdx, [rax + 2*rax]
//         mov ebx, 1
//         xor r13d, r13d
//         mov rdi, qword ptr [rip + std::sync::mpmc::waker::current_thread_id::DUMMY::__getit::__KEY@GOTTPOFF]
//         mov qword ptr [rsp + 24], rdx
//         jmp .LBB42_170
//
//         .p2align        4, 0x90
// .LBB42_169:
//         add r13, 24
//
//         inc rbx
//         cmp rdx, r13
//         je .LBB42_193
//
// .LBB42_170:
//         mov r12, qword ptr [r14 + r13 + 16]
//
//         mov r15, qword ptr [r12 + 32]
//
//         cmp byte ptr fs:[rdi], 0
//
//         je .LBB42_172
//
//         mov rax, qword ptr fs:[0]
//         add rax, rdi
//         inc rax
//         cmp r15, rax
//         je .LBB42_169
//         jmp .LBB42_173
//
//         .p2align        4, 0x90
// .LBB42_172:
//         xor edi, edi
//         call std::thread::local::fast::Key<T>::try_initialize
//         mov rdi, qword ptr [rip + std::sync::mpmc::waker::current_thread_id::DUMMY::__getit::__KEY@GOTTPOFF]
//         mov rdx, qword ptr [rsp + 24]
//
//         mov rax, qword ptr fs:[0]
//         add rax, rdi
//         inc rax
//         cmp r15, rax
//         je .LBB42_169
//
// .LBB42_173:
//         mov rcx, qword ptr [r14 + r13]
//
//         xor eax, eax
//         lock cmpxchg    qword ptr [r12 + 16], rcx
//
//         jne .LBB42_169
//
//         mov rcx, qword ptr [r14 + r13 + 8]
//
//         mov rax, qword ptr [r14 + r13 + 16]
//
//         test rcx, rcx
//
//         je .LBB42_176
//
//         mov qword ptr [rax + 24], rcx
//
// .LBB42_176:
//         mov rdi, qword ptr [rax + 40]
//
//         add rdi, 16
//
//         call qword ptr [rip + std::thread::Inner::parker@GOTPCREL]
//
//         mov ecx, 1
//
//         xchg dword ptr [rax], ecx
//
//         cmp ecx, -1
//         jne .LBB42_179
//
//         mov rdi, rax
//         call qword ptr [rip + std::sys::unix::futex::futex_wake@GOTPCREL]
//
// .LBB42_179:
//         lea rdi, [rbx - 1]
//
//         mov r12, qword ptr [rsp + 8]
//
//         mov r14, qword ptr [r12 + 24]
//
//         cmp r14, rdi
//         jbe .LBB42_322
//
//         mov rax, qword ptr [r12 + 16]
//
//         lea rdi, [rax + r13]
//         movups xmm0, xmmword ptr [rax + r13]
//         movaps xmmword ptr [rsp + 48], xmm0
//         mov r15, qword ptr [rax + r13 + 16]
//
//         lea rsi, [rax + r13]
//         add rsi, 24
//
//         mov rax, r14
//         sub rax, rbx
//         shl rax, 3
//         lea rdx, [rax + 2*rax]
//         call qword ptr [rip + memmove@GOTPCREL]
//
//         dec r14
//
//         mov qword ptr [r12 + 24], r14
//
//         test r15, r15
//         je .LBB42_193
//
//         movdqa xmm0, xmmword ptr [rsp + 48]
//         movdqa xmmword ptr [rsp + 128], xmm0
//         mov qword ptr [rsp + 144], r15
//         mov rax, qword ptr [rsp + 136]
//         mov qword ptr [rsp + 672], rax
//
//         cmp byte ptr [rsp + 16], 0
//         mov r15, qword ptr [rip + std::thread::yield_now@GOTPCREL]
//         je .LBB42_286
//
// .LBB42_182:
//         xor eax, eax
//         mov rdi, qword ptr [rsp + 8]
//
//         xchg dword ptr [rdi], eax
//
//         cmp eax, 2
//         jne .LBB42_184
//
//         call qword ptr [rip + std::sys::unix::locks::futex_mutex::Mutex::wake@GOTPCREL]
//
// .LBB42_184:
//         mov r14, qword ptr [rsp + 672]
//         test r14, r14
//         je .LBB42_290
//
//         cmp byte ptr [r14 + 9], 0
//         je .LBB42_291
//
//         mov r15d, dword ptr [r14 + 4]
//
//         cmp dword ptr [r14], 0
//
//         mov dword ptr [r14], 0
//
//         je .LBB42_323
//
//         mov byte ptr [r14 + 8], 1
//         jmp .LBB42_306
//
// .LBB42_188:
//         lea rax, [r8 + rsi]
//
//         mov qword ptr [rsp + 128], rax
//
//         mov rdi, qword ptr [rsp + 8]
//
//         add rcx, qword ptr [rdi + 408]
//
//         mov qword ptr [rsp + 136], rcx
//
//         mov ebx, dword ptr [r8 + rsi + 8]
//
//         mov qword ptr [rax], rcx
//
//         add rdi, 256
//
//         call std::sync::mpmc::waker::SyncWaker::notify
//
//         shl rbx, 32
//         or rbx, 256
//
//         test bl, 1
//         je .LBB42_311
//         jmp .LBB42_319
//
// .LBB42_190:
//         cmp r15d, 30
//
//         jne .LBB42_37
//
//         mov rax, qword ptr [rsp + 8]
//
//         test rax, rax
//         je .LBB42_321
//
//         mov qword ptr [r13 + 136], rax
//
//         lock add        qword ptr [r13 + 128], 2
//
//         mov qword ptr [rbx + 496], rax
//         mov r15d, 30
//
//         jmp .LBB42_40
//
// .LBB42_193:
//         mov rcx, qword ptr [rsp + 8]
//
//         cmp byte ptr [rcx + 104], 0
//         je .LBB42_214
//
//         cmp byte ptr [rsp + 16], 0
//         je .LBB42_255
//
// .LBB42_195:
//         xor eax, eax
//         mov rdi, qword ptr [rsp + 8]
//         xchg dword ptr [rdi], eax
//         xor r15d, r15d
//         mov r14d, 1
//
//         cmp eax, 2
//         jne .LBB42_309
//
//         call qword ptr [rip + std::sys::unix::locks::futex_mutex::Mutex::wake@GOTPCREL]
//
//         xor r15d, r15d
//         jmp .LBB42_309
// .LBB42_198:
//
//         mov rax, r15
//
//         cmp r15d, 30
//         mov r14, r12
//         jne .LBB42_218
//
//         call qword ptr [rip + std::sync::mpmc::utils::Backoff::new@GOTPCREL]
//
//         mov ebx, eax
//         jmp .LBB42_202
//
//         .p2align        4, 0x90
// .LBB42_201:
//         inc ebx
//
// .LBB42_202:
//         mov rax, qword ptr [r14 + 496]
//
//         test rax, rax
//
//         jne .LBB42_217
//
//         cmp ebx, 7
//         jae .LBB42_210
//
//         mov ecx, ebx
//         imul ecx, ecx
//
//         test ecx, ecx
//
//         je .LBB42_201
//
//         lea edx, [rcx - 1]
//         mov eax, ecx
//
//         and eax, 7
//         cmp edx, 7
//         jb .LBB42_208
//
//         and ecx, -8
//
//         .p2align        4, 0x90
// .LBB42_207:
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         add ecx, -8
//         jne .LBB42_207
//
// .LBB42_208:
//         test eax, eax
//         je .LBB42_211
//
//         .p2align        4, 0x90
// .LBB42_209:
//         pause
//
//         dec eax
//         jne .LBB42_209
//         jmp .LBB42_211
//
//         .p2align        4, 0x90
// .LBB42_210:
//
//         call qword ptr [rip + std::thread::yield_now@GOTPCREL]
//
// .LBB42_211:
//         cmp ebx, 10
//         ja .LBB42_202
//         jmp .LBB42_201
//
// .LBB42_212:
//         mov dword ptr [rsp + 16], 0
//
//         mov rcx, qword ptr [rsp + 8]
//
//         movzx eax, byte ptr [rcx + 4]
//
//         test al, al
//
//         je .LBB42_167
//
// .LBB42_213:
//         mov qword ptr [rsp + 128], rcx
//         mov eax, dword ptr [rsp + 16]
//         mov byte ptr [rsp + 136], al
//
//         lea rdi, [rip + .L__unnamed_5]
//         lea rcx, [rip + .L__unnamed_2]
//
//         lea r8, [rip + .L__unnamed_29]
//         lea rdx, [rsp + 128]
//         mov esi, 43
//         call qword ptr [rip + core::result::unwrap_failed@GOTPCREL]
//
//         jmp .LBB42_326
//
// .LBB42_214:
//         mov qword ptr [rsp + 128], rcx
//         mov eax, dword ptr [rsp + 16]
//         mov byte ptr [rsp + 136], al
//         lea rax, [rsp + 640]
//         mov qword ptr [rsp + 144], rax
//         lea rax, [rsp + 32]
//         mov qword ptr [rsp + 152], rax
//         mov qword ptr [rsp + 160], rcx
//
//         mov rax, qword ptr [rip + std::sync::mpmc::context::Context::with::CONTEXT::__getit::__KEY@GOTTPOFF]
//
//         cmp qword ptr fs:[rax], 0
//
//         je .LBB42_260
//
//         mov rcx, qword ptr fs:[0]
//         lea r12, [rcx + rax]
//         add r12, 8
//         jmp .LBB42_262
// .LBB42_216:
//         mov ecx, 1
//         jmp .LBB42_254
// .LBB42_217:
//
//         add r13, 2
//
//         and r13, -2
//
//         mov rcx, qword ptr [rax + 496]
//
//         xor edx, edx
//         test rcx, rcx
//         setne dl
//
//         or rdx, r13
//
//         mov rcx, qword ptr [rsp + 8]
//
//         mov qword ptr [rcx + 8], rax
//
//         mov qword ptr [rcx], rdx
//         mov eax, 30
//
// .LBB42_218:
//         mov qword ptr [rsp + 144], r14
//         mov qword ptr [rsp + 152], rax
//
//         call qword ptr [rip + std::sync::mpmc::utils::Backoff::new@GOTPCREL]
//
//         mov ebx, eax
//         mov r13, r15
//         shl r13, 4
//         add r14, r13
//         jmp .LBB42_221
//
//         .p2align        4, 0x90
// .LBB42_220:
//         inc ebx
//
// .LBB42_221:
//         mov rax, qword ptr [r14]
//
//         test al, 1
//         jne .LBB42_231
//
//         cmp ebx, 7
//         jae .LBB42_229
//
//         mov ecx, ebx
//         imul ecx, ecx
//
//         test ecx, ecx
//
//         je .LBB42_220
//
//         lea edx, [rcx - 1]
//         mov eax, ecx
//         and eax, 7
//         cmp edx, 7
//         jb .LBB42_227
//
//         and ecx, -8
//
//         .p2align        4, 0x90
// .LBB42_226:
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         add ecx, -8
//         jne .LBB42_226
//
// .LBB42_227:
//         test eax, eax
//         je .LBB42_230
//
//         .p2align        4, 0x90
// .LBB42_228:
//         pause
//
//         dec eax
//         jne .LBB42_228
//         jmp .LBB42_230
//
//         .p2align        4, 0x90
// .LBB42_229:
//
//         call qword ptr [rip + std::thread::yield_now@GOTPCREL]
//
// .LBB42_230:
//         cmp ebx, 10
//         ja .LBB42_221
//         jmp .LBB42_220
//
// .LBB42_231:
//         movd xmm0, dword ptr [r12 + r13 + 8]
//
//         lea rdx, [r15 + 1]
//         cmp rdx, 31
//         jne .LBB42_242
//
//         xor ecx, ecx
//         xor edx, edx
//         jmp .LBB42_234
//
//         .p2align        4, 0x90
// .LBB42_233:
//         add rdx, 2
//
//         cmp rdx, 30
//
//         je .LBB42_252
//
// .LBB42_234:
//         mov rsi, rdx
//         shl rsi, 4
//
//         mov rax, qword ptr [r12 + rsi]
//
//         test al, 2
//         jne .LBB42_238
//
//         add rsi, r12
//
//         mov rax, qword ptr [rsi]
//
//         .p2align        4, 0x90
// .LBB42_236:
//         mov rbx, rax
//         or rbx, 4
//         lock cmpxchg    qword ptr [rsi], rbx
//         jne .LBB42_236
//
//         test al, 2
//         je .LBB42_254
//
// .LBB42_238:
//         mov rsi, rdx
//         or rsi, 1
//
//         shl rsi, 4
//
//         mov rax, qword ptr [r12 + rsi]
//
//         test al, 2
//         jne .LBB42_233
//
//         add rsi, r12
//
//         mov rax, qword ptr [rsi]
//
//         .p2align        4, 0x90
// .LBB42_240:
//         mov rbx, rax
//         or rbx, 4
//         lock cmpxchg    qword ptr [rsi], rbx
//         jne .LBB42_240
//
//         test al, 2
//         jne .LBB42_233
//         jmp .LBB42_254
//
// .LBB42_242:
//         mov rax, qword ptr [r14]
//
//         .p2align        4, 0x90
// .LBB42_243:
//         mov rcx, rax
//         or rcx, 2
//         lock cmpxchg    qword ptr [r14], rcx
//         jne .LBB42_243
//
//         test al, 4
//         je .LBB42_253
//
//         cmp r15d, 28
//
//         ja .LBB42_252
//
//         xor ecx, ecx
//         jmp .LBB42_248
//
//         .p2align        4, 0x90
// .LBB42_247:
//         inc rdx
//
//         cmp rdx, 30
//
//         je .LBB42_252
//
// .LBB42_248:
//         mov rsi, rdx
//         shl rsi, 4
//
//         mov rax, qword ptr [r12 + rsi]
//
//         test al, 2
//         jne .LBB42_247
//
//         add rsi, r12
//
//         mov rax, qword ptr [rsi]
//
//         .p2align        4, 0x90
// .LBB42_250:
//         mov rbx, rax
//         or rbx, 4
//         lock cmpxchg    qword ptr [rsi], rbx
//         jne .LBB42_250
//
//         test al, 2
//         jne .LBB42_247
//         jmp .LBB42_254
//
// .LBB42_252:
//         mov esi, 504
//         mov edx, 8
//         mov rdi, r12
//         movd dword ptr [rsp + 8], xmm0
//
//         call qword ptr [rip + __rust_dealloc@GOTPCREL]
//
//         movd xmm0, dword ptr [rsp + 8]
//
// .LBB42_253:
//         xor ecx, ecx
//
// .LBB42_254:
//         movd eax, xmm0
//         shl rax, 32
//         lea rbx, [rax + rcx]
//         add rbx, 256
//
//         test bl, 1
//         je .LBB42_311
//         jmp .LBB42_319
//
// .LBB42_255:
//         mov rax, qword ptr [rip + std::panicking::panic_count::GLOBAL_PANIC_COUNT@GOTPCREL]
//         mov rax, qword ptr [rax]
//
//         movabs rcx, 9223372036854775807
//
//         test rax, rcx
//         je .LBB42_195
//
//         call qword ptr [rip + std::panicking::panic_count::is_zero_slow_path@GOTPCREL]
//
//         test al, al
//         jne .LBB42_195
//
//         mov rax, qword ptr [rsp + 8]
//
//         mov byte ptr [rax + 4], 1
//         jmp .LBB42_195
//
// .LBB42_314:
//         mov ebx, 1
//
//         test bl, 1
//         je .LBB42_311
//         jmp .LBB42_319
//
// .LBB42_260:
//         xor edi, edi
//         call std::thread::local::fast::Key<T>::try_initialize
//
//         mov r12, rax
//
//         test rax, rax
//
//         je .LBB42_273
//
// .LBB42_262:
//         mov rbx, qword ptr [r12]
//
//         mov qword ptr [r12], 0
//
//         test rbx, rbx
//         je .LBB42_267
//
//         mov qword ptr [rsp + 48], rbx
//
//         mov qword ptr [rbx + 16], 0
//
//         mov qword ptr [rbx + 24], 0
//
//         lea rdi, [rsp + 128]
//         lea r15, [rsp + 48]
//
//         mov rsi, r15
//         call std::sync::mpmc::context::Context::with::{{closure}}
//
//         mov r14, rax
//
//         mov rdi, qword ptr [r12]
//
//         mov qword ptr [r12], rbx
//
//         test rdi, rdi
//         je .LBB42_272
//
//         lock dec        qword ptr [rdi]
//
//         jne .LBB42_272
//
//         #MEMBARRIER
//         jmp .LBB42_271
//
// .LBB42_267:
//         call qword ptr [rip + std::sync::mpmc::context::Context::new@GOTPCREL]
//
//         mov rbx, rax
//         mov qword ptr [rsp + 48], rax
//
//         lea rdi, [rsp + 128]
//         lea r15, [rsp + 48]
//         mov rsi, r15
//         call std::sync::mpmc::context::Context::with::{{closure}}
//
//         mov r14, rax
//
//         lock dec        qword ptr [rbx]
//
//         jne .LBB42_272
//
//         #MEMBARRIER
//         mov rdi, qword ptr [rsp + 48]
//
// .LBB42_271:
//         call alloc::sync::Arc<T>::drop_slow
//
// .LBB42_272:
//         cmp r14b, 2
//         jne .LBB42_277
//
// .LBB42_273:
//         call qword ptr [rip + std::sync::mpmc::context::Context::new@GOTPCREL]
//
//         mov rbx, rax
//         mov qword ptr [rsp + 48], rax
//
//         lea rdi, [rsp + 128]
//         lea rsi, [rsp + 48]
//         call std::sync::mpmc::context::Context::with::{{closure}}
//
//         mov r14, rax
//
//         lock dec        qword ptr [rbx]
//
//         jne .LBB42_277
//
//         #MEMBARRIER
//         mov rdi, qword ptr [rsp + 48]
//         call alloc::sync::Arc<T>::drop_slow
//
// .LBB42_277:
//         movzx eax, byte ptr [rsp + 136]
//
//         cmp al, 2
//         je .LBB42_281
//
//         mov rbx, qword ptr [rsp + 128]
//
//         test al, al
//         je .LBB42_282
//
// .LBB42_279:
//         xor eax, eax
//
//         xchg dword ptr [rbx], eax
//
//         cmp eax, 2
//         jne .LBB42_281
//
//         mov rdi, rbx
//         call qword ptr [rip + std::sys::unix::locks::futex_mutex::Mutex::wake@GOTPCREL]
//
// .LBB42_281:
//         mov r15, r14
//         and r15, -65536
//         jmp .LBB42_309
//
// .LBB42_282:
//         mov rax, qword ptr [rip + std::panicking::panic_count::GLOBAL_PANIC_COUNT@GOTPCREL]
//         mov rax, qword ptr [rax]
//
//         movabs rcx, 9223372036854775807
//         test rax, rcx
//         je .LBB42_279
//
//         call qword ptr [rip + std::panicking::panic_count::is_zero_slow_path@GOTPCREL]
//
//         test al, al
//         jne .LBB42_279
//
//         mov byte ptr [rbx + 4], 1
//         jmp .LBB42_279
//
// .LBB42_286:
//         mov rax, qword ptr [rip + std::panicking::panic_count::GLOBAL_PANIC_COUNT@GOTPCREL]
//         mov rax, qword ptr [rax]
//
//         movabs rcx, 9223372036854775807
//         test rax, rcx
//         je .LBB42_182
//
//         call qword ptr [rip + std::panicking::panic_count::is_zero_slow_path@GOTPCREL]
//
//         test al, al
//         jne .LBB42_182
//
//         mov rax, qword ptr [rsp + 8]
//
//         mov byte ptr [rax + 4], 1
//         jmp .LBB42_182
//
// .LBB42_290:
//         jmp .LBB42_306
//
// .LBB42_291:
//         call qword ptr [rip + std::sync::mpmc::utils::Backoff::new@GOTPCREL]
//
//         mov ebx, eax
//         jmp .LBB42_294
//
// .LBB42_293:
//         inc ebx
//
// .LBB42_294:
//         movzx eax, byte ptr [r14 + 8]
//
//         test al, al
//
//         jne .LBB42_304
//
//         cmp ebx, 7
//         jae .LBB42_302
//
//         mov ecx, ebx
//         imul ecx, ecx
//
//         test ecx, ecx
//
//         je .LBB42_293
//
//         lea edx, [rcx - 1]
//         mov eax, ecx
//         and eax, 7
//         cmp edx, 7
//         jb .LBB42_300
//
//         and ecx, -8
//
//         .p2align        4, 0x90
// .LBB42_299:
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         pause
//
//         add ecx, -8
//         jne .LBB42_299
//
// .LBB42_300:
//         test eax, eax
//         je .LBB42_303
//
//         .p2align        4, 0x90
// .LBB42_301:
//         pause
//
//         dec eax
//         jne .LBB42_301
//         jmp .LBB42_303
//
// .LBB42_302:
//         call r15
//
// .LBB42_303:
//         cmp ebx, 10
//         ja .LBB42_294
//         jmp .LBB42_293
//
// .LBB42_304:
//         mov r15d, dword ptr [r14 + 4]
//
//         cmp dword ptr [r14], 0
//
//         mov dword ptr [r14], 0
//
//         je .LBB42_324
//
//         mov rdi, qword ptr [rsp + 672]
//
//         mov esi, 12
//         mov edx, 4
//         call qword ptr [rip + __rust_dealloc@GOTPCREL]
//
// .LBB42_306:
//         xor ebx, ebx
//         test r14, r14
//         sete bl
//
//         shl r15, 32
//
//         mov rax, qword ptr [rsp + 144]
//
//         lock dec        qword ptr [rax]
//
//         jne .LBB42_308
//
//         #MEMBARRIER
//         mov rdi, qword ptr [rsp + 144]
//         call alloc::sync::Arc<T>::drop_slow
//
// .LBB42_308:
//         lea r14, [r15 + rbx]
//         add r14, 256
//
// .LBB42_309:
//         movzx ebx, r14b
//         or rbx, r15
//
//         test bl, 1
//         jne .LBB42_319
//
// .LBB42_311:
//         shr rbx, 32
//
//         mov dword ptr [rsp + 128], ebx
//         lea rax, [rsp + 128]
//         #APP
//         #NO_APP
//
//         lea rdi, [rsp + 80]
//
//         call core::ptr::drop_in_place<std::sync::mpmc::Receiver<f32>>
//
//         lea rdi, [rsp + 112]
//
//         call core::ptr::drop_in_place<std::sync::mpmc::Sender<f32>>
//
//         lea rsp, [rbp - 40]
//
//         pop rbx
//         pop r12
//         pop r13
//         pop r14
//         pop r15
//         pop rbp
//         .cfi_def_cfa rsp, 8
//         ret
// .LBB42_313:
//         .cfi_def_cfa rbp, 16
//         mov ebx, 257
//
//         test bl, 1
//         je .LBB42_311
//
// .LBB42_319:
//         lea r8, [rip + .L__unnamed_30]
//         lea rcx, [rip + .L__unnamed_4]
//         jmp .LBB42_320
// .LBB42_315:
//
//         xor eax, eax
//         mov qword ptr [rsp + 8], rax
//
//         mov edi, 504
//         mov esi, 8
//         call qword ptr [rip + alloc::alloc::handle_alloc_error@GOTPCREL]
//
//         jmp .LBB42_326
//
// .LBB42_316:
//         mov edi, 504
//         mov esi, 8
//         call qword ptr [rip + alloc::alloc::handle_alloc_error@GOTPCREL]
//
//         jmp .LBB42_326
//
// .LBB42_317:
//         movd xmm0, dword ptr [rsp + 24]
//
//         movd dword ptr [rsp + 128], xmm0
//         lea r8, [rip + .L__unnamed_31]
//         lea rcx, [rip + .L__unnamed_3]
//
// .LBB42_320:
//         lea rdi, [rip + .L__unnamed_5]
//         mov esi, 43
//         mov rdx, qword ptr [rsp + 104]
//         call qword ptr [rip + core::result::unwrap_failed@GOTPCREL]
//
//         jmp .LBB42_326
//
// .LBB42_318:
//         mov edi, 512
//         mov esi, 128
//         call qword ptr [rip + alloc::alloc::handle_alloc_error@GOTPCREL]
//
//         jmp .LBB42_326
//
// .LBB42_321:
//         lea rdi, [rip + .L__unnamed_14]
//         lea rdx, [rip + .L__unnamed_32]
//         mov esi, 43
//         call qword ptr [rip + core::panicking::panic@GOTPCREL]
//
//         jmp .LBB42_326
//
// .LBB42_322:
//         lea rdx, [rip + .L__unnamed_12]
//         mov rsi, r14
//         call qword ptr [rip + alloc::vec::Vec<T,A>::remove::assert_failed@GOTPCREL]
//
//         jmp .LBB42_326
//
// .LBB42_323:
//         lea rdx, [rip + .L__unnamed_33]
//         jmp .LBB42_325
//
// .LBB42_324:
//         lea rdx, [rip + .L__unnamed_34]
//
// .LBB42_325:
//         lea rdi, [rip + .L__unnamed_14]
//         mov esi, 43
//         call qword ptr [rip + core::panicking::panic@GOTPCREL]
//
// .LBB42_326:
//         ud2
//
//         jmp .LBB42_330
//
//         jmp .LBB42_340
//
// .LBB42_330:
//         mov r14, rax
//
//         mov rdi, r15
//         call core::ptr::drop_in_place<std::sync::mpmc::context::Context>
//         jmp .LBB42_337
//
//         jmp .LBB42_334
//
//         mov r14, rax
//         lea rdi, [rsp + 48]
//
//         call core::ptr::drop_in_place<std::sync::mpmc::context::Context>
//         jmp .LBB42_337
//
// .LBB42_334:
//         mov r14, rax
//         lea rdi, [rsp + 128]
//         call core::ptr::drop_in_place<std::sync::mpmc::waker::Entry>
//         jmp .LBB42_363
//
//         jmp .LBB42_340
//
//         mov r14, rax
//
// .LBB42_337:
//         mov rdi, qword ptr [rsp + 128]
//         mov esi, dword ptr [rsp + 136]
//
//         call core::ptr::drop_in_place<core::option::Option<std::sync::mpmc::zero::Channel<f32>::recv::{{closure}}>>
//
//         jmp .LBB42_363
//
//         call qword ptr [rip + core::panicking::panic_cannot_unwind@GOTPCREL]
//         ud2
//
// .LBB42_340:
//         mov r14, rax
//         mov rdi, qword ptr [rsp + 16]
//
//         call core::ptr::drop_in_place<std::sync::mpmc::context::Context>
//         jmp .LBB42_363
//
//         mov r14, rax
//
//         movzx esi, byte ptr [rsp + 16]
//         mov rdi, qword ptr [rsp + 8]
//         call core::ptr::drop_in_place<std::sync::mutex::MutexGuard<std::sync::mpmc::zero::Inner>>
//
//         jmp .LBB42_363
//
//         call qword ptr [rip + core::panicking::panic_cannot_unwind@GOTPCREL]
//         ud2
//
//         mov r14, rax
//
//         lea rdi, [rsp + 128]
//
//         call core::ptr::drop_in_place<std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<std::sync::mpmc::zero::Inner>>>
//
//         jmp .LBB42_363
//
//         call qword ptr [rip + core::panicking::panic_cannot_unwind@GOTPCREL]
//         ud2
//
//         jmp .LBB42_354
//
//         jmp .LBB42_362
//
//         jmp .LBB42_354
//
//         mov r14, rax
//         jmp .LBB42_364
//
//         mov r14, rax
//
//         lea rdi, [rsp + 128]
//
//         call core::ptr::drop_in_place<std::sync::mpmc::counter::Counter<std::sync::mpmc::list::Channel<f32>>>
//
//         jmp .LBB42_365
//
//         call qword ptr [rip + core::panicking::panic_cannot_unwind@GOTPCREL]
//         ud2
//
//         jmp .LBB42_362
//
//         jmp .LBB42_362
//
// .LBB42_354:
//         mov r14, rax
//         mov rdi, qword ptr [rsp + 96]
//
//         call core::ptr::drop_in_place<std::sync::mpmc::context::Context>
//         jmp .LBB42_363
//
//         jmp .LBB42_359
//
//         jmp .LBB42_362
//
//         jmp .LBB42_362
//
// .LBB42_359:
//         mov r14, rax
//         mov rdi, qword ptr [rsp + 8]
//
//         call core::ptr::drop_in_place<core::option::Option<alloc::boxed::Box<std::sync::mpmc::list::Block<f32>>>>
//         jmp .LBB42_363
//
//         jmp .LBB42_362
//
// .LBB42_362:
//         mov r14, rax
// .LBB42_363:
//
//         lea rdi, [rsp + 80]
//         call core::ptr::drop_in_place<std::sync::mpsc::Receiver<f32>>
//
// .LBB42_364:
//         lea rdi, [rsp + 112]
//         call core::ptr::drop_in_place<std::sync::mpsc::Sender<f32>>
//
// .LBB42_365:
//         mov rdi, r14
//         call _Unwind_Resume@PLT
//         ud2
//
//         call qword ptr [rip + core::panicking::panic_cannot_unwind@GOTPCREL]
//         ud2
#[inline(never)]
fn mpsc() {
    let (tx, mut rx) = mpsc::channel::<f32>();
    tx.send(black_box(5.)).unwrap();

    black_box(rx.recv().unwrap());
}

// .section .text.exobody_performance_tests::five_plus_six,"ax",@progbits
//         .p2align        4, 0x90
//         .type   exobody_performance_tests::five_plus_six,@function
// exobody_performance_tests::five_plus_six:
//
//         .cfi_startproc
//         sub rsp, 12
//         .cfi_def_cfa_offset 20
//
//         mov dword ptr [rsp], 5
//         mov rax, rsp
//         #APP
//         #NO_APP
//
//         mov dword ptr [rsp + 4], 6
//         lea rax, [rsp + 4]
//         #APP
//         #NO_APP
//
//         mov dword ptr [rsp + 8], 11
//         lea rax, [rsp + 8]
//         #APP
//         #NO_APP
//
//         add rsp, 12
//         .cfi_def_cfa_offset 8
//         ret

#[inline(never)]
fn five_plus_six() {
    let a = black_box(5);
    let b = black_box(6);
    // Hardcodes 11 instead of doing calculation during runtime
    black_box(5 + 6);
}

#[inline(never)]
fn main() {
    mpsc();
    five_plus_six();
    //let mut a = 5;

    //let (tx, mut rx) = mpsc::channel::<[f32; 100]>();

    //mpsc_test_big_chunked_10x_with_iteration(&tx, &mut rx);
}
