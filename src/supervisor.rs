use crate::DynRustSBI;
use core::mem::size_of;
use memoffset::offset_of;

pub enum Operation {
    Stop,
    SystemReset,
}

// Hardware thread
pub struct Hart {
    context: SupervisorContext,
    // sbi: DynRustSBI,
    // penglai: DynPenglai,
    // raven: DynRaven,
    // dram_hypervisor: DynDramHypervisor,
}

impl Hart {
    pub fn new(a0: usize, a1: usize, pc: usize) -> Self {
        use riscv::register::mstatus;
        let mstatus = unsafe {
            let mstatus: usize;
            mstatus::set_mpp(mstatus::MPP::Supervisor);
            mstatus::set_mpie();
            core::arch::asm!("csrr {}, mstatus", out(reg) mstatus);
            mstatus
        };
        let context = SupervisorContext {
            mepc: pc,
            mstatus,
            a0,
            a1,
            ..Default::default()
        };
        Self { context }
    }
}

#[derive(Default)]
pub struct SupervisorContext {
    machine_sp: usize,
    ra: usize,
    sp: usize,
    gp: usize,
    tp: usize,
    t0: usize,
    t1: usize,
    t2: usize,
    s0: usize,
    s1: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
    s2: usize,
    s3: usize,
    s4: usize,
    s5: usize,
    s6: usize,
    s7: usize,
    s8: usize,
    s9: usize,
    s10: usize,
    s11: usize,
    t3: usize,
    t4: usize,
    t5: usize,
    t6: usize,
    mstatus: usize,
    mepc: usize,
}

// no need to fix memory representation here; offset_of! takes care of it

struct MachineContext {
    ra: usize,
    gp: usize,
    tp: usize,
    s0: usize,
    s1: usize,
    s2: usize,
    s3: usize,
    s4: usize,
    s5: usize,
    s6: usize,
    s7: usize,
    s8: usize,
    s9: usize,
    s10: usize,
    s11: usize,
}

// todo: save/restore context can have macros like offset_of!(mstatus, SupervisorContext) etc.

#[naked]
unsafe extern "C" fn machine_to_supervisor(save: *mut SupervisorContext) -> ! {
    core::arch::asm!(
        "addi  sp, sp, -{machine_context}",
        "sd  ra, {machine_ra}(sp)",
        "sd  gp, {machine_gp}(sp)",
        "sd  tp, {machine_tp}(sp)",
        "sd  s0, {machine_s0}(sp)",
        "sd  s1, {machine_s1}(sp)",
        "sd  s2, {machine_s2}(sp)",
        "sd  s3, {machine_s3}(sp)",
        "sd  s4, {machine_s4}(sp)",
        "sd  s5, {machine_s5}(sp)",
        "sd  s6, {machine_s6}(sp)",
        "sd  s7, {machine_s7}(sp)",
        "sd  s8, {machine_s8}(sp)",
        "sd  s9, {machine_s9}(sp)",
        "sd  s10, {machine_s10}(sp)",
        "sd  s11, {machine_s11}(sp)",
        "sd  sp, {machine_sp}(a0)",
        "mv  sp, a0",
        "ld  t0, {supervisor_mstatus}(sp)",
        "ld  t1, {supervisor_mepc}(sp)",
        "csrw  mstatus, t0",
        "csrw  mepc, t1",
        "ld  ra, {supervisor_ra}(sp)",
        "ld  gp, {supervisor_gp}(sp)",
        "ld  tp, {supervisor_tp}(sp)",
        "ld  t0, {supervisor_t0}(sp)",
        "ld  t1, {supervisor_t1}(sp)",
        "ld  t2, {supervisor_t2}(sp)",
        "ld  s0, {supervisor_s0}(sp)",
        "ld  s1, {supervisor_s1}(sp)",
        "ld  a0, {supervisor_a0}(sp)",
        "ld  a1, {supervisor_a1}(sp)",
        "ld  a2, {supervisor_a2}(sp)",
        "ld  a3, {supervisor_a3}(sp)",
        "ld  a4, {supervisor_a4}(sp)",
        "ld  a5, {supervisor_a5}(sp)",
        "ld  a6, {supervisor_a6}(sp)",
        "ld  a7, {supervisor_a7}(sp)",
        "ld  s2, {supervisor_s2}(sp)",
        "ld  s3, {supervisor_s3}(sp)",
        "ld  s4, {supervisor_s4}(sp)",
        "ld  s5, {supervisor_s5}(sp)",
        "ld  s6, {supervisor_s6}(sp)",
        "ld  s7, {supervisor_s7}(sp)",
        "ld  s8, {supervisor_s8}(sp)",
        "ld  s9, {supervisor_s9}(sp)",
        "ld  s10, {supervisor_s10}(sp)",
        "ld  s11, {supervisor_s11}(sp)",
        "ld  t3, {supervisor_t3}(sp)",
        "ld  t4, {supervisor_t4}(sp)",
        "ld  t5, {supervisor_t5}(sp)",
        "ld  t6, {supervisor_t6}(sp)",
        "csrw  mscratch, sp",
        "ld  sp, {supervisor_sp}(sp)",
        "mret",
        machine_context = const size_of::<MachineContext>(),
        machine_ra = const offset_of!(MachineContext, ra),
        machine_gp = const offset_of!(MachineContext, gp),
        machine_tp = const offset_of!(MachineContext, tp),
        machine_s0 = const offset_of!(MachineContext, s0),
        machine_s1 = const offset_of!(MachineContext, s1),
        machine_s2 = const offset_of!(MachineContext, s2),
        machine_s3 = const offset_of!(MachineContext, s3),
        machine_s4 = const offset_of!(MachineContext, s4),
        machine_s5 = const offset_of!(MachineContext, s5),
        machine_s6 = const offset_of!(MachineContext, s6),
        machine_s7 = const offset_of!(MachineContext, s7),
        machine_s8 = const offset_of!(MachineContext, s8),
        machine_s9 = const offset_of!(MachineContext, s9),
        machine_s10 = const offset_of!(MachineContext, s10),
        machine_s11 = const offset_of!(MachineContext, s11),
        machine_sp = const offset_of!(SupervisorContext, machine_sp),
        supervisor_mstatus = const offset_of!(SupervisorContext, mstatus),
        supervisor_mepc = const offset_of!(SupervisorContext, mepc),
        supervisor_ra = const offset_of!(SupervisorContext, ra),
        supervisor_gp = const offset_of!(SupervisorContext, gp),
        supervisor_tp = const offset_of!(SupervisorContext, tp),
        supervisor_t0 = const offset_of!(SupervisorContext, t0),
        supervisor_t1 = const offset_of!(SupervisorContext, t1),
        supervisor_t2 = const offset_of!(SupervisorContext, t2),
        supervisor_s0 = const offset_of!(SupervisorContext, s0),
        supervisor_s1 = const offset_of!(SupervisorContext, s1),
        supervisor_a0 = const offset_of!(SupervisorContext, a0),
        supervisor_a1 = const offset_of!(SupervisorContext, a1),
        supervisor_a2 = const offset_of!(SupervisorContext, a2),
        supervisor_a3 = const offset_of!(SupervisorContext, a3),
        supervisor_a4 = const offset_of!(SupervisorContext, a4),
        supervisor_a5 = const offset_of!(SupervisorContext, a5),
        supervisor_a6 = const offset_of!(SupervisorContext, a6),
        supervisor_a7 = const offset_of!(SupervisorContext, a7),
        supervisor_s2 = const offset_of!(SupervisorContext, s2),
        supervisor_s3 = const offset_of!(SupervisorContext, s3),
        supervisor_s4 = const offset_of!(SupervisorContext, s4),
        supervisor_s5 = const offset_of!(SupervisorContext, s5),
        supervisor_s6 = const offset_of!(SupervisorContext, s6),
        supervisor_s7 = const offset_of!(SupervisorContext, s7),
        supervisor_s8 = const offset_of!(SupervisorContext, s8),
        supervisor_s9 = const offset_of!(SupervisorContext, s9),
        supervisor_s10 = const offset_of!(SupervisorContext, s10),
        supervisor_s11 = const offset_of!(SupervisorContext, s11),
        supervisor_t3 = const offset_of!(SupervisorContext, t3),
        supervisor_t4 = const offset_of!(SupervisorContext, t4),
        supervisor_t5 = const offset_of!(SupervisorContext, t5),
        supervisor_t6 = const offset_of!(SupervisorContext, t6),
        supervisor_sp = const offset_of!(SupervisorContext, sp),
        options(noreturn)
    )
}

#[naked]
unsafe extern "C" fn supervisor_to_machine() -> ! {
    core::arch::asm!(
        ".p2align 2",
        "csrrw  sp, mscratch, sp",
        "sd  ra, {supervisor_ra}(sp)",
        "sd  gp, {supervisor_gp}(sp)",
        "sd  tp, {supervisor_tp}(sp)",
        "sd  t0, {supervisor_t0}(sp)",
        "sd  t1, {supervisor_t1}(sp)",
        "sd  t2, {supervisor_t2}(sp)",
        "sd  s0, {supervisor_s0}(sp)",
        "sd  s1, {supervisor_s1}(sp)",
        "sd  a0, {supervisor_a0}(sp)",
        "sd  a1, {supervisor_a1}(sp)",
        "sd  a2, {supervisor_a2}(sp)",
        "sd  a3, {supervisor_a3}(sp)",
        "sd  a4, {supervisor_a4}(sp)",
        "sd  a5, {supervisor_a5}(sp)",
        "sd  a6, {supervisor_a6}(sp)",
        "sd  a7, {supervisor_a7}(sp)",
        "sd  s2, {supervisor_s2}(sp)",
        "sd  s3, {supervisor_s3}(sp)",
        "sd  s4, {supervisor_s4}(sp)",
        "sd  s5, {supervisor_s5}(sp)",
        "sd  s6, {supervisor_s6}(sp)",
        "sd  s7, {supervisor_s7}(sp)",
        "sd  s8, {supervisor_s8}(sp)",
        "sd  s9, {supervisor_s9}(sp)",
        "sd  s10, {supervisor_s10}(sp)",
        "sd  s11, {supervisor_s11}(sp)",
        "sd  t3, {supervisor_t3}(sp)",
        "sd  t4, {supervisor_t4}(sp)",
        "sd  t5, {supervisor_t5}(sp)",
        "sd  t6, {supervisor_t6}(sp)",
        "csrr  t0, mstatus",
        "csrr  t1, mepc",
        "sd  t0, {supervisor_mstatus}(sp)",
        "sd  t1, {supervisor_mepc}(sp)",
        "csrr  t2, mscratch",
        "sd  t2, {supervisor_sp}(sp)",
        "ld  sp, {machine_sp}(sp)",
        "ld  ra, {machine_ra}(sp)",
        "ld  gp, {machine_gp}(sp)",
        "ld  tp, {machine_tp}(sp)",
        "ld  s0, {machine_s0}(sp)",
        "ld  s1, {machine_s1}(sp)",
        "ld  s2, {machine_s2}(sp)",
        "ld  s3, {machine_s3}(sp)",
        "ld  s4, {machine_s4}(sp)",
        "ld  s5, {machine_s5}(sp)",
        "ld  s6, {machine_s6}(sp)",
        "ld  s7, {machine_s7}(sp)",
        "ld  s8, {machine_s8}(sp)",
        "ld  s9, {machine_s9}(sp)",
        "ld  s10, {machine_s10}(sp)",
        "ld  s11, {machine_s11}(sp)",
        "addi  sp, sp, {machine_context}",
        "jr  ra",
        supervisor_ra = const offset_of!(SupervisorContext, ra),
        supervisor_gp = const offset_of!(SupervisorContext, gp),
        supervisor_tp = const offset_of!(SupervisorContext, tp),
        supervisor_t0 = const offset_of!(SupervisorContext, t0),
        supervisor_t1 = const offset_of!(SupervisorContext, t1),
        supervisor_t2 = const offset_of!(SupervisorContext, t2),
        supervisor_s0 = const offset_of!(SupervisorContext, s0),
        supervisor_s1 = const offset_of!(SupervisorContext, s1),
        supervisor_a0 = const offset_of!(SupervisorContext, a0),
        supervisor_a1 = const offset_of!(SupervisorContext, a1),
        supervisor_a2 = const offset_of!(SupervisorContext, a2),
        supervisor_a3 = const offset_of!(SupervisorContext, a3),
        supervisor_a4 = const offset_of!(SupervisorContext, a4),
        supervisor_a5 = const offset_of!(SupervisorContext, a5),
        supervisor_a6 = const offset_of!(SupervisorContext, a6),
        supervisor_a7 = const offset_of!(SupervisorContext, a7),
        supervisor_s2 = const offset_of!(SupervisorContext, s2),
        supervisor_s3 = const offset_of!(SupervisorContext, s3),
        supervisor_s4 = const offset_of!(SupervisorContext, s4),
        supervisor_s5 = const offset_of!(SupervisorContext, s5),
        supervisor_s6 = const offset_of!(SupervisorContext, s6),
        supervisor_s7 = const offset_of!(SupervisorContext, s7),
        supervisor_s8 = const offset_of!(SupervisorContext, s8),
        supervisor_s9 = const offset_of!(SupervisorContext, s9),
        supervisor_s10 = const offset_of!(SupervisorContext, s10),
        supervisor_s11 = const offset_of!(SupervisorContext, s11),
        supervisor_t3 = const offset_of!(SupervisorContext, t3),
        supervisor_t4 = const offset_of!(SupervisorContext, t4),
        supervisor_t5 = const offset_of!(SupervisorContext, t5),
        supervisor_t6 = const offset_of!(SupervisorContext, t6),
        supervisor_mstatus = const offset_of!(SupervisorContext, mstatus),
        supervisor_mepc = const offset_of!(SupervisorContext, mepc),
        supervisor_sp = const offset_of!(SupervisorContext, sp),
        machine_sp = const offset_of!(SupervisorContext, machine_sp),
        machine_ra = const offset_of!(MachineContext, ra),
        machine_gp = const offset_of!(MachineContext, gp),
        machine_tp = const offset_of!(MachineContext, tp),
        machine_s0 = const offset_of!(MachineContext, s0),
        machine_s1 = const offset_of!(MachineContext, s1),
        machine_s2 = const offset_of!(MachineContext, s2),
        machine_s3 = const offset_of!(MachineContext, s3),
        machine_s4 = const offset_of!(MachineContext, s4),
        machine_s5 = const offset_of!(MachineContext, s5),
        machine_s6 = const offset_of!(MachineContext, s6),
        machine_s7 = const offset_of!(MachineContext, s7),
        machine_s8 = const offset_of!(MachineContext, s8),
        machine_s9 = const offset_of!(MachineContext, s9),
        machine_s10 = const offset_of!(MachineContext, s10),
        machine_s11 = const offset_of!(MachineContext, s11),
        machine_context = const size_of::<MachineContext>(),
        options(noreturn)
    )
}
