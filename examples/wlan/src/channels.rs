const CHANNELS: [(u32, f64); 67] = [
    // ### 11g
    (1, 2412e6),
    (2, 2417e6),
    (3, 2422e6),
    (4, 2427e6),
    (5, 2432e6),
    (6, 2437e6),
    (7, 2442e6),
    (8, 2447e6),
    (9, 2452e6),
    (10, 2457e6),
    (11, 2462e6),
    (12, 2467e6),
    (13, 2472e6),
    (14, 2484e6),
    // ### 11a
    (34, 5170e6),
    (36, 5180e6),
    (38, 5190e6),
    (40, 5200e6),
    (42, 5210e6),
    (44, 5220e6),
    (46, 5230e6),
    (48, 5240e6),
    (50, 5250e6),
    (52, 5260e6),
    (54, 5270e6),
    (56, 5280e6),
    (58, 5290e6),
    (60, 5300e6),
    (62, 5310e6),
    (64, 5320e6),
    (100, 5500e6),
    (102, 5510e6),
    (104, 5520e6),
    (106, 5530e6),
    (108, 5540e6),
    (110, 5550e6),
    (112, 5560e6),
    (114, 5570e6),
    (116, 5580e6),
    (118, 5590e6),
    (120, 5600e6),
    (122, 5610e6),
    (124, 5620e6),
    (126, 5630e6),
    (128, 5640e6),
    (132, 5660e6),
    (134, 5670e6),
    (136, 5680e6),
    (138, 5690e6),
    (140, 5700e6),
    (142, 5710e6),
    (144, 5720e6),
    (149, 5745e6),
    (151, 5755e6),
    (153, 5765e6),
    (155, 5775e6),
    (157, 5785e6),
    (159, 5795e6),
    (161, 5805e6),
    (165, 5825e6),
    // ### 11p
    (172, 5860e6),
    (174, 5870e6),
    (176, 5880e6),
    (178, 5890e6),
    (180, 5900e6),
    (182, 5910e6),
    (184, 5920e6),
];

pub fn channel_to_freq(chan: u32) -> Option<f64> {
    CHANNELS
        .iter()
        .find_map(|(c, f)| if chan == *c { Some(*f) } else { None })
}

pub fn parse_channel(s: &str) -> Result<f64, String> {
    let channel: u32 = s
        .parse()
        .map_err(|_| format!("`{}` isn't a WLAN channel number", s))?;

    channel_to_freq(channel).ok_or_else(|| format!("`{}` isn't a WLAN channel number", s))
}