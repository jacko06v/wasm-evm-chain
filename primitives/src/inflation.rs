// This file is part of Astar.

// Copyright (C) Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// Astar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Astar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Astar. If not, see <http://www.gnu.org/licenses/>.

use super::BlockNumber;

/// Era number type
pub type EraNumber = u32;
/// Period number type
pub type PeriodNumber = u32;
/// Dapp Id type
pub type DAppId = u16;
/// Tier Id type
pub type TierId = u8;
// Tier Rank type
pub type Rank = u8;

/// Configuration for cycles, periods, subperiods & eras.
///
/// * `cycle` - Time unit similar to 'year' in the real world. Consists of one or more periods. At the beginning of each cycle, inflation is recalculated.
/// * `period` - Period consists of two distinct subperiods: `Voting` & `Build&Earn`. They are integral parts of dApp staking.
///              Length is expressed in standard eras or just _eras_.
/// * `era` - Era is the basic time unit in the dApp staking protocol. At the end of each era, reward pools for stakers & dApps are calculated.
///           Era length is expressed in blocks.
pub trait CycleConfiguration {
    /// How many different periods are there in a cycle (a 'year').
    ///
    /// This value has to be at least 1.
    fn periods_per_cycle() -> PeriodNumber;

    /// For how many standard era lengths does the voting subperiod last.
    ///
    /// This value has to be at least 1.
    fn eras_per_voting_subperiod() -> EraNumber;

    /// How many standard eras are there in the build&earn subperiod.
    ///
    /// This value has to be at least 1.
    fn eras_per_build_and_earn_subperiod() -> EraNumber;

    /// How many blocks are there per standard era.
    ///
    /// This value has to be at least 1.
    fn blocks_per_era() -> BlockNumber;

    /// For how many standard era lengths does the period last.
    fn period_in_era_lengths() -> EraNumber {
        Self::eras_per_voting_subperiod().saturating_add(Self::eras_per_build_and_earn_subperiod())
    }

    /// For how many standard era lengths does the cycle (a 'year') last.
    fn cycle_in_era_lengths() -> EraNumber {
        Self::period_in_era_lengths().saturating_mul(Self::periods_per_cycle())
    }

    /// How many blocks are there per cycle (a 'year').
    fn blocks_per_cycle() -> BlockNumber {
        Self::blocks_per_era().saturating_mul(Self::cycle_in_era_lengths())
    }

    /// For how many standard era lengths do all the build&earn subperiods in a cycle last.
    fn build_and_earn_eras_per_cycle() -> EraNumber {
        Self::eras_per_build_and_earn_subperiod().saturating_mul(Self::periods_per_cycle())
    }

    /// How many distinct eras are there in a single period.
    fn eras_per_period() -> EraNumber {
        Self::eras_per_build_and_earn_subperiod().saturating_add(1)
    }

    /// How many distinct eras are there in a cycle.
    fn eras_per_cycle() -> EraNumber {
        Self::eras_per_period().saturating_mul(Self::periods_per_cycle())
    }
}
