pub const EM_ABUND_CUTOFF: f64 = 0.01;
pub const PAIR_REGEX: &str = r"(.+)(_?1|_?2)(\..+)";
pub const CUTOFF_PVALUE:f64 = 0.9999999999;
pub const SAMPLE_SIZE_CUTOFF: usize = 25;
pub const MEDIAN_ANI_THRESHOLD: f64 = 2.;
pub const QUERY_FILE_SUFFIX: &str = ".syldb";
pub const SAMPLE_FILE_SUFFIX: &str = ".sylsp";
pub const QUERY_FILE_SUFFIX_VALID : [&str;2] = [QUERY_FILE_SUFFIX, ".sylqueries"];
pub const SAMPLE_FILE_SUFFIX_VALID : [&str;2] = [SAMPLE_FILE_SUFFIX, ".sylsample"];
pub const MIN_ANI_DEF: f64 = 0.9;
pub const MIN_ANI_P_DEF: f64 = 0.95;
pub const MAX_MEDIAN_FOR_MEAN_FINAL_EST: f64 = 15.;
pub const DEREP_PROFILE_ANI: f64 = 0.975;
pub const MAX_DEDUP_COUNT: u32 = 4;
