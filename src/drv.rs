use crate::diverged::ExtiDiverged;
use core::num::NonZeroUsize;
use displaydoc::Display;
use drone_cortexm::{fib, fib::Fiber, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::periph::exti::{
    ExtiFtsrFt, ExtiMap, ExtiPeriph, ExtiPrPif, ExtiRtsrRt, ExtiSwierSwi, SyscfgExticrExti,
};
use futures::prelude::*;

/// EXTI stream overflow
#[derive(Display, Debug)]
pub struct ExtiOverflow;

/// EXTI setup.
pub struct ExtiSetup<
    Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
    ExtiInt: IntToken,
> {
    /// EXTI peripheral.
    pub exti: ExtiPeriph<Exti>,
    /// EXTI interrupt.
    pub exti_int: ExtiInt,
    /// EXTI configuration.
    ///
    /// This will be written to SYSCFG_EXTICRx.EXTIy field. See the reference
    /// manual for details.
    pub config: u32,
    /// Falling trigger selection.
    pub falling: bool,
    /// Rising trigger selection.
    pub rising: bool,
}

/// EXTI driver.
pub struct ExtiDrv<
    Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
    ExtiInt: IntToken,
> {
    exti: ExtiDiverged<Exti>,
    exti_int: ExtiInt,
}

impl<
    Exti: ExtiMap + SyscfgExticrExti + ExtiRtsrRt + ExtiFtsrFt + ExtiSwierSwi + ExtiPrPif,
    ExtiInt: IntToken,
> ExtiDrv<Exti, ExtiInt>
{
    /// Sets up a new [`ExtiDrv`] from `setup` values.
    pub fn init(setup: ExtiSetup<Exti, ExtiInt>) -> Self {
        let ExtiSetup { exti, exti_int, config, falling, rising } = setup;
        let drv = Self { exti: exti.into(), exti_int };
        drv.init_exti(config, falling, rising);
        drv
    }

    /// Creates a new saturating stream of external events.
    pub fn create_saturating_stream(&self) -> impl Stream<Item = NonZeroUsize> + Send + Sync {
        self.exti_int.add_saturating_pulse_stream(self.new_fib())
    }

    /// Creates a new fallible stream of external events.
    pub fn create_try_stream(
        &self,
    ) -> impl Stream<Item = Result<NonZeroUsize, ExtiOverflow>> + Send + Sync {
        self.exti_int.add_pulse_try_stream(|| Err(ExtiOverflow), self.new_fib())
    }

    fn new_fib<R>(&self) -> impl Fiber<Input = (), Yield = Option<usize>, Return = R> {
        let exti_pr_pif = self.exti.exti_pr_pif;
        fib::new_fn(move || {
            if exti_pr_pif.read_bit() {
                // selected trigger request occurred
                exti_pr_pif.set_bit();
                fib::Yielded(Some(1))
            } else {
                fib::Yielded(None)
            }
        })
    }

    fn init_exti(&self, config: u32, falling: bool, rising: bool) {
        self.exti.syscfg_exticr_exti.write_bits(config); // configuration
        self.exti.exti_imr_im.set_bit(); // interrupt request from line 4 is not masked
        if falling {
            self.exti.exti_ftsr_ft.set_bit(); // falling trigger enabled
        }
        if rising {
            self.exti.exti_rtsr_rt.set_bit(); // rising trigger enabled
        }
    }
}
