use strum_macros::EnumIter;

#[derive(Debug)]
#[derive(Copy,Clone, EnumIter)]
#[allow(non_camel_case_types)]
pub enum Formats {
    NO_URL, // in case of no format being available
    // 3gp
    _3GP_144, // 17
    _3GP_180, // 36
    // flv 
    FLV_240, // 5
    FLV_270, // 6
    FLV_360, // 34
    FLV_480, // 35
    // mp4
    MP4_360, // 18
    MP4_720, // 22
    MP4_1080, // 37
    MP4_3072, // 38
    MP4_360_3D, // 82
    MP4_480_3D, // 83
    MP4_720_3D, // 84
    MP4_1080_3D, // 85
    // hls
    HLS_240, // 92
    HLS_260, // 93
    HLS_480, // 94
    HLS_720, // 95
    HLS_1080, // 96
    // webm video
    WEBM_360,  // 43
    WEBM_480,  // 44
    WEBM_720,  // 45
    WEBM_1080, // 46
    // webm audio
    WEBM_AUDIO_171, // 171
    WEBM_AUDIO_249, // 249
    WEBM_AUDIO_250, // 250
    WEBM_AUDIO_251, // 251
    // m4a audio
    M4A_139, // 139
    M4A_140, // 140
    M4A_141, // 141 
}

impl std::fmt::Display for Formats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// YT video codes
// https://gist.github.com/sidneys/7095afe4da4ae58694d128b1034e01e2