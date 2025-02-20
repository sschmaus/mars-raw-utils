use crate::calibfile;
use crate::enums;
use crate::memcache;
use crate::veprintln;
use crate::vprintln;
use regex::Regex;
use sciimg::path;
use std::convert::TryInto;

use anyhow::anyhow;
use anyhow::Result;

pub const ILT: [u32; 256] = [
    0, 2, 3, 3, 4, 5, 5, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 18, 19, 20, 22, 24, 25, 27, 29, 31,
    33, 35, 37, 39, 41, 43, 46, 48, 50, 53, 55, 58, 61, 63, 66, 69, 72, 75, 78, 81, 84, 87, 90, 94,
    97, 100, 104, 107, 111, 115, 118, 122, 126, 130, 134, 138, 142, 146, 150, 154, 159, 163, 168,
    172, 177, 181, 186, 191, 196, 201, 206, 211, 216, 221, 226, 231, 236, 241, 247, 252, 258, 263,
    269, 274, 280, 286, 292, 298, 304, 310, 316, 322, 328, 334, 341, 347, 354, 360, 367, 373, 380,
    387, 394, 401, 408, 415, 422, 429, 436, 443, 450, 458, 465, 472, 480, 487, 495, 503, 510, 518,
    526, 534, 542, 550, 558, 566, 575, 583, 591, 600, 608, 617, 626, 634, 643, 652, 661, 670, 679,
    688, 697, 706, 715, 724, 733, 743, 752, 761, 771, 781, 790, 800, 810, 819, 829, 839, 849, 859,
    869, 880, 890, 900, 911, 921, 932, 942, 953, 964, 974, 985, 996, 1007, 1018, 1029, 1040, 1051,
    1062, 1074, 1085, 1096, 1108, 1119, 1131, 1142, 1154, 1166, 1177, 1189, 1201, 1213, 1225, 1237,
    1249, 1262, 1274, 1286, 1299, 1311, 1324, 1336, 1349, 1362, 1374, 1387, 1400, 1413, 1426, 1439,
    1452, 1465, 1479, 1492, 1505, 1519, 1532, 1545, 1559, 1573, 1586, 1600, 1614, 1628, 1642, 1656,
    1670, 1684, 1698, 1712, 1727, 1741, 1755, 1770, 1784, 1799, 1814, 1828, 1843, 1858, 1873, 1888,
    1903, 1918, 1933, 1948, 1963, 1979, 1994, 2009, 2025, 2033,
];

// https://pds-imaging.jpl.nasa.gov/data/nsyt/insight_cameras/calibration/ilut/
pub const NSYT_ILT: [u32; 256] = [
    0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 5, 6, 7, 8, 9, 11, 12, 14, 15, 17, 19, 21, 23, 25, 27, 29,
    32, 34, 37, 40, 43, 46, 49, 52, 55, 59, 62, 66, 70, 73, 77, 82, 86, 90, 95, 99, 104, 109, 114,
    119, 124, 129, 135, 140, 146, 152, 158, 164, 170, 176, 182, 189, 196, 202, 209, 216, 224, 231,
    238, 246, 254, 261, 269, 277, 286, 294, 302, 311, 320, 328, 337, 347, 356, 365, 375, 384, 394,
    404, 414, 424, 435, 445, 456, 467, 477, 488, 500, 511, 522, 534, 545, 557, 569, 581, 594, 606,
    619, 631, 644, 657, 670, 683, 697, 710, 724, 738, 752, 766, 780, 794, 809, 823, 838, 853, 868,
    884, 899, 914, 930, 946, 962, 978, 994, 1011, 1027, 1044, 1061, 1078, 1095, 1112, 1130, 1147,
    1165, 1183, 1201, 1219, 1237, 1256, 1274, 1293, 1312, 1331, 1350, 1370, 1389, 1409, 1429, 1449,
    1469, 1489, 1509, 1530, 1551, 1572, 1593, 1614, 1635, 1657, 1678, 1700, 1722, 1744, 1766, 1789,
    1811, 1834, 1857, 1880, 1903, 1926, 1950, 1974, 1997, 2021, 2045, 2070, 2094, 2119, 2144, 2168,
    2193, 2219, 2244, 2270, 2295, 2321, 2347, 2373, 2400, 2426, 2453, 2479, 2506, 2534, 2561, 2588,
    2616, 2644, 2671, 2700, 2728, 2756, 2785, 2813, 2842, 2871, 2900, 2930, 2959, 2989, 3019, 3049,
    3079, 3109, 3140, 3170, 3201, 3232, 3263, 3295, 3326, 3358, 3390, 3421, 3454, 3486, 3518, 3551,
    3584, 3617, 3650, 3683, 3716, 3750, 3784, 3818, 3852, 3886, 3920, 3955, 3990, 4025, 4060, 4095,
];

// https://pds-geosciences.wustl.edu/m2020/urn-nasa-pds-mars2020_mission/calibration_camera/ilut/M20_LUT2_v2a.txt
pub const LUT2: [u32; 256] = [
    0, 1, 127, 131, 135, 140, 144, 149, 153, 158, 163, 168, 173, 178, 183, 188, 193, 199, 204, 210,
    215, 221, 227, 233, 239, 245, 251, 257, 263, 269, 276, 282, 289, 296, 302, 309, 316, 323, 330,
    337, 345, 352, 359, 367, 374, 382, 390, 398, 406, 414, 422, 430, 438, 447, 455, 464, 472, 481,
    490, 499, 508, 517, 526, 535, 544, 554, 563, 573, 582, 592, 602, 612, 622, 632, 642, 653, 663,
    674, 684, 695, 705, 716, 727, 738, 749, 761, 772, 783, 795, 806, 818, 830, 841, 853, 865, 877,
    890, 902, 914, 927, 939, 952, 965, 978, 990, 1004, 1017, 1030, 1043, 1057, 1070, 1084, 1097,
    1111, 1125, 1139, 1153, 1167, 1181, 1196, 1210, 1225, 1239, 1254, 1269, 1284, 1299, 1314, 1329,
    1344, 1360, 1375, 1391, 1406, 1422, 1438, 1454, 1470, 1486, 1502, 1519, 1535, 1552, 1568, 1585,
    1602, 1619, 1636, 1653, 1670, 1687, 1705, 1722, 1740, 1758, 1775, 1793, 1811, 1829, 1848, 1866,
    1884, 1903, 1921, 1940, 1959, 1978, 1997, 2016, 2035, 2054, 2074, 2093, 2113, 2132, 2152, 2172,
    2192, 2212, 2232, 2252, 2273, 2293, 2314, 2335, 2355, 2376, 2397, 2418, 2439, 2461, 2482, 2504,
    2525, 2547, 2569, 2590, 2612, 2634, 2657, 2679, 2701, 2724, 2746, 2769, 2792, 2815, 2838, 2861,
    2884, 2907, 2931, 2954, 2978, 3002, 3025, 3049, 3073, 3098, 3122, 3146, 3171, 3195, 3220, 3244,
    3269, 3294, 3319, 3345, 3370, 3395, 3421, 3446, 3472, 3498, 3524, 3550, 3576, 3602, 3628, 3655,
    3681, 3708, 3734, 3761, 3788, 3815, 3842, 3870, 3897, 3924, 3952, 3980, 4007, 4035, 4095,
];

lazy_static! {
    static ref LUT_SPEC_PAIR: Regex = Regex::new(r"([0-9]+) ([0-9]+)").unwrap();
}

#[derive(Debug, Clone)]
pub struct LookUpTable {
    pub lut: Vec<u32>,
}

impl LookUpTable {
    pub fn new(lut: &[u32; 256]) -> LookUpTable {
        LookUpTable { lut: lut.to_vec() }
    }
    pub fn new_from_vec(lut: &Vec<u32>) -> Result<LookUpTable> {
        if lut.len() != 256 {
            Err(anyhow!("Invalid LUT specification length"))
        } else {
            Ok(LookUpTable { lut: lut.clone() })
        }
    }
    pub fn max(&self) -> u32 {
        self.lut[255]
    }

    pub fn to_array(&self) -> [u32; 256] {
        self.lut
            .clone()
            .try_into()
            .unwrap_or_else(|_: Vec<u32>| panic!("LUT array is of invalid length"))
    }
}

pub fn get_ilt_for_instrument(instrument: enums::Instrument) -> Result<LookUpTable> {
    let lut_file_path =
        calibfile::get_calibration_file_for_instrument(instrument, enums::CalFileType::Lut)
            .unwrap_or("".to_string());

    if lut_file_path.is_empty() {
        Ok(LookUpTable::new(&ILT))
    } else {
        load_ilut_spec_file(&lut_file_path)
    }
}

pub fn load_ilut_spec_file(file_path: &String) -> Result<LookUpTable> {
    vprintln!("Loading LUT file: {}", file_path);

    if !path::file_exists(file_path) {
        veprintln!("ERROR: LUT file not found: {}", file_path);
        return Err(anyhow!("Lookup table file not found"));
    }

    let mut lut_vec: Vec<u32> = vec![];
    memcache::load_text_file(file_path)
        .unwrap()
        .split('\n')
        .for_each(|line| {
            // This regex capture will validate if the line is in the format "<number><space><number>"
            // which ignores any embedded VICAR label information
            if let Some(caps) = LUT_SPEC_PAIR.captures(line) {
                let s_lut_value = caps.get(2).unwrap().as_str().parse::<u32>().unwrap_or(0);
                lut_vec.push(s_lut_value);
            }
        });
    LookUpTable::new_from_vec(&lut_vec)
}
