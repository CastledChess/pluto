use std::fmt::{self};

#[derive(Debug)]
pub enum OptionKind {
    Spin,
    String,
}

impl OptionKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Spin => "spin",
            Self::String => "string",
        }
    }
}

impl fmt::Display for OptionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub struct OptionDescriptor<T> {
    pub name: &'static str,
    pub kind: OptionKind,
    pub value: T,
    pub min: T,
    pub max: T,
}

impl OptionDescriptor<i32> {
    pub fn fmt_spsa(&self) -> String {
        format!(
            "{}, int, {}, {}, {}, {}, {}",
            self.name, self.value, self.min, self.max, 2.25, 0.002
        )
    }
}
impl OptionDescriptor<u8> {
    pub fn fmt_spsa(&self) -> String {
        format!(
            "{}, int, {}, {}, {}, {}, {}",
            self.name, self.value, self.min, self.max, 2.25, 0.002
        )
    }
}
impl OptionDescriptor<usize> {
    pub fn fmt_spsa(&self) -> String {
        format!(
            "{}, int, {}, {}, {}, {}, {}",
            self.name, self.value, self.min, self.max, 2.25, 0.002
        )
    }
}
impl OptionDescriptor<i64> {
    pub fn fmt_spsa(&self) -> String {
        format!(
            "{}, int, {}, {}, {}, {}, {}",
            self.name, self.value, self.min, self.max, 2.25, 0.002
        )
    }
}
impl OptionDescriptor<u64> {
    pub fn fmt_spsa(&self) -> String {
        format!(
            "{}, int, {}, {}, {}, {}, {}",
            self.name, self.value, self.min, self.max, 2.25, 0.002
        )
    }
}
impl OptionDescriptor<f64> {
    pub fn fmt_spsa(&self) -> String {
        format!(
            "{}, float, {}, {}, {}, {}, {}",
            self.name, self.value, self.min, self.max, 2.25, 0.002
        )
    }
}

impl fmt::Display for OptionDescriptor<i32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "option name {} type {} default {} min {} max {}",
            self.name, self.kind, self.value, self.min, self.max
        )
    }
}
impl fmt::Display for OptionDescriptor<u8> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "option name {} type {} default {} min {} max {}",
            self.name, self.kind, self.value, self.min, self.max
        )
    }
}
impl fmt::Display for OptionDescriptor<usize> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "option name {} type {} default {} min {} max {}",
            self.name, self.kind, self.value, self.min, self.max
        )
    }
}
impl fmt::Display for OptionDescriptor<i64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "option name {} type {} default {} min {} max {}",
            self.name, self.kind, self.value, self.min, self.max
        )
    }
}
impl fmt::Display for OptionDescriptor<u64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "option name {} type {} default {} min {} max {}",
            self.name, self.kind, self.value, self.min, self.max
        )
    }
}
impl fmt::Display for OptionDescriptor<f64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "option name {} type {} default {} min {} max {}",
            self.name, self.kind, self.value, self.min, self.max
        )
    }
}

pub struct Config {
    pub move_overhead: OptionDescriptor<usize>,
    pub threads: OptionDescriptor<u8>,
    pub hash: OptionDescriptor<usize>,
    pub qsearch_depth: OptionDescriptor<u8>,
    pub rfp_depth: OptionDescriptor<u8>,
    pub rfp_base_margin: OptionDescriptor<i32>,
    pub rfp_reduction_improving: OptionDescriptor<i32>,
    pub fp_depth_margin: OptionDescriptor<u8>,
    pub fp_base_margin: OptionDescriptor<i32>,
    pub fp_margin_depth_factor: OptionDescriptor<i32>,
    pub nmp_depth: OptionDescriptor<u8>,
    pub nmp_margin: OptionDescriptor<u8>,
    pub nmp_divisor: OptionDescriptor<u8>,
    pub nmp_divisor_improving: OptionDescriptor<u8>,
    pub lmp_move_margin: OptionDescriptor<usize>,
    pub lmp_depth_factor: OptionDescriptor<u8>,
    pub lmr_depth: OptionDescriptor<u8>,
    pub lmr_move_margin: OptionDescriptor<usize>,
    pub lmr_quiet_margin: OptionDescriptor<f64>,
    pub lmr_quiet_divisor: OptionDescriptor<f64>,
    pub lmr_base_margin: OptionDescriptor<f64>,
    pub lmr_base_divisor: OptionDescriptor<f64>,
    pub mo_tt_entry_value: OptionDescriptor<i32>,
    pub mo_capture_value: OptionDescriptor<i32>,
    pub mo_killer_value: OptionDescriptor<i32>,
    pub tc_time_divisor: OptionDescriptor<u64>,
    pub tc_elapsed_factor: OptionDescriptor<i64>,
}

impl Config {
    pub fn default() -> Self {
        Self {
            move_overhead: OptionDescriptor {
                name: "MoveOverhead",
                kind: OptionKind::Spin,
                value: 0,
                min: 0,
                max: 10000,
            },
            threads: OptionDescriptor {
                name: "Threads",
                kind: OptionKind::Spin,
                value: 1,
                min: 1,
                max: 1,
            },
            hash: OptionDescriptor {
                name: "Hash",
                kind: OptionKind::Spin,
                value: 255,
                min: 1,
                max: 1024,
            },
            qsearch_depth: OptionDescriptor {
                name: "QSearchDepth",
                kind: OptionKind::Spin,
                value: 15,
                min: 1,
                max: 20,
            },
            rfp_depth: OptionDescriptor {
                name: "RFPDepth",
                kind: OptionKind::Spin,
                value: 11,
                min: 1,
                max: 20,
            },
            rfp_base_margin: OptionDescriptor {
                name: "RFPBaseMargin",
                kind: OptionKind::Spin,
                value: 55,
                min: 1,
                max: 200,
            },
            rfp_reduction_improving: OptionDescriptor {
                name: "RFPReductionImproving",
                kind: OptionKind::Spin,
                value: 23,
                min: 1,
                max: 200,
            },
            fp_depth_margin: OptionDescriptor {
                name: "FPDepthMargin",
                kind: OptionKind::Spin,
                value: 7,
                min: 1,
                max: 20,
            },
            fp_base_margin: OptionDescriptor {
                name: "FPBaseMargin",
                kind: OptionKind::Spin,
                value: 40,
                min: 1,
                max: 200,
            },
            fp_margin_depth_factor: OptionDescriptor {
                name: "FPMarginDepthFactor",
                kind: OptionKind::Spin,
                value: 30,
                min: 1,
                max: 200,
            },
            nmp_depth: OptionDescriptor {
                name: "NMPDepth",
                kind: OptionKind::Spin,
                value: 5,
                min: 1,
                max: 20,
            },
            nmp_margin: OptionDescriptor {
                name: "NMPMargin",
                kind: OptionKind::Spin,
                value: 10,
                min: 1,
                max: 20,
            },
            nmp_divisor: OptionDescriptor {
                name: "NMPDivisor",
                kind: OptionKind::Spin,
                value: 6,
                min: 1,
                max: 20,
            },
            nmp_divisor_improving: OptionDescriptor {
                name: "NMPDivisorImproving",
                kind: OptionKind::Spin,
                value: 5,
                min: 1,
                max: 20,
            },
            lmp_move_margin: OptionDescriptor {
                name: "LMPMoveMargin",
                kind: OptionKind::Spin,
                value: 2,
                min: 1,
                max: 20,
            },
            lmp_depth_factor: OptionDescriptor {
                name: "LMPDepthFactor",
                kind: OptionKind::Spin,
                value: 5,
                min: 1,
                max: 20,
            },
            lmr_depth: OptionDescriptor {
                name: "LMRDepth",
                kind: OptionKind::Spin,
                value: 12,
                min: 1,
                max: 20,
            },
            lmr_move_margin: OptionDescriptor {
                name: "LMRMoveMargin",
                kind: OptionKind::Spin,
                value: 9,
                min: 1,
                max: 20,
            },
            lmr_quiet_margin: OptionDescriptor {
                name: "LMRQuietMargin",
                kind: OptionKind::String,
                value: 2.74,
                min: 0.0,
                max: 10.0,
            },
            lmr_quiet_divisor: OptionDescriptor {
                name: "LMRQuietDivisor",
                kind: OptionKind::String,
                value: 1.65,
                min: 1.0,
                max: 10.0,
            },
            lmr_base_margin: OptionDescriptor {
                name: "LMRBaseMargin",
                kind: OptionKind::String,
                value: 0.25,
                min: 1.0,
                max: 10.0,
            },
            lmr_base_divisor: OptionDescriptor {
                name: "LMRBaseDivisor",
                kind: OptionKind::String,
                value: 1.7,
                min: 1.0,
                max: 10.0,
            },
            mo_tt_entry_value: OptionDescriptor {
                name: "MOTTEntryValue",
                kind: OptionKind::Spin,
                value: 233,
                min: 1,
                max: 500,
            },
            mo_capture_value: OptionDescriptor {
                name: "MOCaptureValue",
                kind: OptionKind::Spin,
                value: 60,
                min: 0,
                max: 500,
            },
            mo_killer_value: OptionDescriptor {
                name: "MOKillerValue",
                kind: OptionKind::Spin,
                value: 76,
                min: 0,
                max: 500,
            },
            tc_time_divisor: OptionDescriptor {
                name: "TCTimeDivisor",
                kind: OptionKind::Spin,
                value: 8,
                min: 2,
                max: 100,
            },
            tc_elapsed_factor: OptionDescriptor {
                name: "TCElapsedFactor",
                kind: OptionKind::Spin,
                value: 5,
                min: 1,
                max: 10,
            },
        }
    }
}
