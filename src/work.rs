//! The furnaces' answer to the Machine Work library's shared capability (`machine_work::MACHINE_WORK`; this port's
//! addition): how far a furnace is with its current smelt and how long a smelt of an item takes it now, for any mod that
//! reads it (AE2's crafting ETA asks it before its own `register_machine`, which `ae2_eta.rs` still answers for AE2
//! without the library). Registered in `lib.rs` for every tier and the Rainbow Furnace when the library is installed.
//!
//! - **pace**: the furnace's own smelt time for the item (`furnace::cook_time_of`: the tier's speed against the recipe's
//!   time, the speed and fuel augments, the red augment's recipe type), and the smelts at once as batch
//!   (`ae2_eta::parallel`: the Rainbow, Allthemodium, Vibranium and Unobtainium mult; in factory mode the rows with Auto
//!   Split). None when it doesn't cook the item.
//! - **progress**: the cook under way, `CookTime` of `CookTimeTotal` with the smelts it works at once; in factory mode
//!   the row closest to done, with the rows cooking as batch. None when idle.
//!
//! A generator, or a furnace switched off by redstone, answers None to both. Read-only: the saved data and slots.

use crate::ae2_eta::working_furnace;
use crate::furnace::Fields;
#[cfg(feature = "machine_work")]
use crate::{
    ae2_eta::parallel_of,
    config,
    furnace::{self, INPUT},
    util::*,
};
use mod_api::*;

/// A furnace's cook under way, as (ticks done, ticks it takes, smelts at once): `cook` of `total`, smelting the input's
/// `count` up to its `mult` at once. None while it doesn't cook.
#[cfg_attr(not(feature = "machine_work"), allow(dead_code))]
pub(crate) fn furnace_progress(
    cook: i32,
    total: i32,
    count: u32,
    mult: i32,
) -> Option<(u32, u32, u32)> {
    if cook <= 0 || total <= 0 {
        return None;
    }
    let batch = count.clamp(1, mult.clamp(1, 64) as u32);
    Some((cook.min(total) as u32, total as u32, batch))
}

/// A factory's cooks under way, from its rows' (timer, total): the row closest to done, with the rows cooking as the
/// batch. None while no row cooks.
#[cfg_attr(not(feature = "machine_work"), allow(dead_code))]
pub(crate) fn factory_progress(rows: &[(i32, i32)]) -> Option<(u32, u32, u32)> {
    let cooking: Vec<(i32, i32)> = rows
        .iter()
        .copied()
        .filter(|&(cook, total)| cook > 0 && total > 0)
        .collect();
    let &(cook, total) = cooking
        .iter()
        .min_by_key(|&&(cook, total)| total - cook.min(total))?;
    Some((cook.min(total) as u32, total as u32, cooking.len() as u32))
}

/// The smelt time of an item, as the capability's pace: (ticks, smelts at once). None when the furnace doesn't cook it.
#[cfg_attr(not(feature = "machine_work"), allow(dead_code))]
pub(crate) fn furnace_pace(cook: i32, parallel: u32) -> Option<(u32, u32)> {
    (cook > 0).then(|| (cook as u32, parallel.max(1)))
}

/// The furnace at `at`, as `working_furnace` reads it, with its block.
#[cfg_attr(not(feature = "machine_work"), allow(dead_code))]
fn read(host: &dyn Host, at: BlockSide) -> Option<(String, Fields, i32)> {
    let block = at.level.block(host, at.pos)?.id(host);
    let (e, mode) = working_furnace(host, at.level, at.pos, &block)?;
    Some((block, e, mode))
}

/// The furnace at a position, answering `machine_work::MACHINE_WORK`.
#[cfg_attr(not(feature = "machine_work"), allow(dead_code))]
pub(crate) struct FurnaceWork(pub BlockSide);

#[cfg(feature = "machine_work")]
impl machine_work::MachineWork for FurnaceWork {
    fn progress(&self, host: &mut dyn ServerHost) -> Option<machine_work::WorkProgress> {
        let (block, e, mode) = read(&*host, self.0)?;
        let (done, total, batch) = if mode == 1 {
            let timers = int_array(&e.nbt, "FactoryCookTime", 6);
            let totals = int_array(&e.nbt, "FactoryTotalCookTime", 6);
            let range = furnace::factory_range(config::tier(&*host, &block));
            let rows: Vec<(i32, i32)> = range
                .filter_map(|j| Some((*timers.get(j)?, *totals.get(j)?)))
                .collect();
            factory_progress(&rows)?
        } else {
            let count = e
                .items
                .get(INPUT)
                .and_then(|s| s.as_ref())
                .map_or(0, |s| s.count());
            furnace_progress(
                get_i(&e.nbt, "CookTime", 0),
                get_i(&e.nbt, "CookTimeTotal", 0),
                count,
                config::smelt_mult(&*host, &block),
            )?
        };
        Some(machine_work::WorkProgress { done, total, batch })
    }

    fn pace(
        &self,
        host: &mut dyn ServerHost,
        input: &ItemResource,
    ) -> Option<machine_work::WorkPace> {
        let (block, e, mode) = read(&*host, self.0)?;
        let cook = furnace::cook_time_of(&*host, self.0.level, &block, &e, input.as_stack());
        let (ticks, batch) = furnace_pace(cook, parallel_of(&*host, &block, &e, mode))?;
        Some(machine_work::WorkPace { ticks, batch })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_furnace_cooks_its_input_up_to_its_mult() {
        // An iron furnace 40 into a 160-tick smelt, one at a time.
        assert_eq!(furnace_progress(40, 160, 12, 1), Some((40, 160, 1)));
        // The Rainbow Furnace (mult 64) with 12 in: 12 at once; with 100 in: 64.
        assert_eq!(furnace_progress(10, 20, 12, 64), Some((10, 20, 12)));
        assert_eq!(furnace_progress(10, 20, 100, 64), Some((10, 20, 64)));
        // Idle: no timer, or no total.
        assert_eq!(furnace_progress(0, 160, 12, 1), None);
        assert_eq!(furnace_progress(40, 0, 12, 1), None);
        // A timer past its total (the total fell with an augment) counts as done.
        assert_eq!(furnace_progress(200, 160, 1, 1), Some((160, 160, 1)));
    }

    #[test]
    fn a_factory_reports_its_row_closest_to_done() {
        // Rows at 30/100, 80/100 and an idle one: 80 of 100, two cooking.
        assert_eq!(
            factory_progress(&[(30, 100), (80, 100), (0, 100)]),
            Some((80, 100, 2))
        );
        // A shorter row with less left wins over a longer one further along.
        assert_eq!(factory_progress(&[(150, 200), (40, 50)]), Some((40, 50, 2)));
        assert_eq!(factory_progress(&[(0, 100), (0, 0)]), None);
        assert_eq!(factory_progress(&[]), None);
    }

    #[test]
    fn the_pace_is_the_smelt_time_and_the_smelts_at_once() {
        assert_eq!(furnace_pace(160, 1), Some((160, 1)));
        // Six factory rows with Auto Split (`ae2_eta::parallel`).
        assert_eq!(
            furnace_pace(100, crate::ae2_eta::parallel(1, 1, 6, true)),
            Some((100, 6))
        );
        // Nothing cooks the item.
        assert_eq!(furnace_pace(0, 1), None);
        assert_eq!(furnace_pace(50, 0), Some((50, 1)));
    }
}
