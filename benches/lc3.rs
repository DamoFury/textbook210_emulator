#![allow(clippy::unusual_byte_groupings)] // so we can group bits by instruction parts

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tools_for_210::emulator::*;

fn criterion_benchmark(c: &mut Criterion) {
    // Test program that covers all LC-3 instructions
    let test_program = r#"
        .ORIG x3000
        ; Arithmetic operations
        ADD R1, R1, #5      ; ADD immediate
        ADD R2, R1, R1      ; ADD register
        AND R3, R1, #3      ; AND immediate
        AND R4, R2, R1      ; AND register
        NOT R5, R1          ; NOT

        ; Data movement
        LD R6, DATA1        ; Load
        LDR R7, R0, #5      ; Load register
        LDI R1, PTRDATA     ; Load indirect
        LEA R2, LOOP        ; Load effective address
        ST R1, RESULT       ; Store
        STR R2, R0, #6      ; Store register
        STI R3, PTRRESULT   ; Store indirect

        ; Control flow
        LOOP:
            ADD R4, R4, #1
            BRz SKIP        ; Branch if zero
            BRnp CONTINUE   ; Branch if not zero
        SKIP:
            JSR SUBROUTINE  ; Jump to subroutine
            BR NEXT         ; Unconditional branch
        CONTINUE:
            JSRR R2         ; Jump to subroutine register
        NEXT:
            JMP R7          ; Jump

        SUBROUTINE:
            RET             ; Return

        ; System operations
        TRAP x25          ; HALT trap

        ; Data
        DATA1: .FILL x1234
        RESULT: .BLKW 1
        PTRDATA: .FILL DATA1
        PTRRESULT: .FILL RESULT
        .END
    "#;

    // Create a group for measuring instruction execution speed
    let mut group = c.benchmark_group("LC3_Instructions");

    // Parse the program
    let parse_result = Emulator::parse_program(test_program).unwrap();
    let (instructions, _, orig_address) = parse_result;

    // Benchmark instruction execution
    group.bench_function("instruction_execution", |b| {
        b.iter(|| {
            let mut emulator = Emulator::new();
            emulator.flash_memory(
                black_box(instructions.iter().map(|(_, instr)| *instr).collect()),
                black_box(orig_address),
            );

            // Run the program
            emulator.running = true;
            emulator.run(Some(100)).unwrap();
        });
    });

    // Benchmark individual instruction types
    group.bench_function("add_execution", |b| {
        b.iter(|| {
            let mut emulator = Emulator::new();
            emulator.r[1].set(5);
            emulator.r[2].set(10);
            // ADD R3, R1, R2
            emulator.ir.set(0b0001_011_001_000_010);
            AddOp.execute(&mut emulator);
            black_box(());
        });
    });

    group.bench_function("and_execution", |b| {
        b.iter(|| {
            let mut emulator = Emulator::new();
            emulator.r[1].set(0x00FF);
            emulator.r[2].set(0x0F0F);
            // AND R3, R1, R2
            emulator.ir.set(0b0101_011_001_000_010);
            AndOp.execute(&mut emulator);
            black_box(());
        });
    });

    group.bench_function("branch_execution", |b| {
        b.iter(|| {
            let mut emulator = Emulator::new();
            emulator.pc.set(0x3000);
            emulator.n.set(1);
            // BRn #10 (branch if negative)
            emulator.ir.set(0b0000_100_000001010);
            BrOp.execute(&mut emulator);
            black_box(());
        });
    });

    group.bench_function("load_store_execution", |b| {
        b.iter(|| {
            let mut emulator = Emulator::new();
            emulator.pc.set(0x3000);
            emulator.memory[0x3005].set(0x1234);
            // LD R3, #5
            emulator.ir.set(0b0010_011_000000101);
            emulator.mar.set(0x3005);
            emulator.mdr.set(0x1234);
            LdOp.execute(&mut emulator);
            black_box(());
        });
    });

    group.bench_function("jsr_execution", |b| {
        b.iter(|| {
            let mut emulator = Emulator::new();
            emulator.pc.set(0x3000);
            // JSR #20
            emulator.ir.set(0b0100_1_00000010100);
            JsrOp.execute(&mut emulator);
            black_box(());
        });
    });

    group.bench_function("trap_execution", |b| {
        b.iter(|| {
            let mut emulator = Emulator::new();
            emulator.pc.set(0x3000);
            // TRAP x25 (HALT)
            emulator.ir.set(0b1111_0000_00100101);
            TrapOp.execute(&mut emulator);
            black_box(());
        });
    });

    group.bench_function("parse_program", |b| {
        b.iter(|| {
            black_box(Emulator::parse_program(black_box(test_program)).unwrap());
        });
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
