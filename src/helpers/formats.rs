use strum_macros::EnumIter;

#[derive(Debug)]
#[derive(Copy,Clone, EnumIter)]
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
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
    MP4_144, // 160
    MP4_360, // 18
    MP4_720, // 22
    MP4_1080, // 37
    MP4_3072, // 38
    MP4_360_3D, // 82
    MP4_480_3D, // 83
    MP4_720_3D, // 84
    MP4_1080_3D, // 85
    MP4_240_VIDEO, // 133
    MP4_360_VIDEO, // 134
    MP4_480_VIDEO, // 135
    MP4_720_VIDEO, // 136
    MP4_720_VIDEO_2, // 298
    MP4_1080_VIDEO, // 137
    MP4_1080_VIDEO_2, // 299
    MP4_1080_VIDEO_3, // 399
    MP4_1440_VIDEO, // 264
    MP4_1440_VIDEO_2, // 400
    MP4_2160_VIDEO, // 138
    MP4_2160_VIDEO_2, // 266
    MP4_2160_VIDEO_3, // 401
    MP4_2880_VIDEO, // 402

    // hls
    HLS_72, //151
    HLS_240, // 92
    HLS_260, // 93
    HLS_480, // 94
    HLS_720, // 95
    HLS_1080, // 96
    HLS_240_2D, // 132
    
    // webm video
    WEBM_360,  // 43
    WEBM_480,  // 44
    WEBM_720,  // 45
    WEBM_1080, // 46
    WEBM_360_3D, // 100
    WEBM_480_3D, // 101
    WEBM_720_3D, // 102

    WEBM_144_VIDEO, // 219
    WEBM_144_VIDEO_2, // 278
    WEBM_144_VIDEO_3, // 330
    WEBM_240_VIDEO, // 242 
    WEBM_360_VIDEO, // 167 
    WEBM_360_VIDEO_2, // 243
    WEBM_480_VIDEO, // 168
    WEBM_480_VIDEO_2, // 218 
    WEBM_480_VIDEO_3, // 244
    WEBM_480_VIDEO_4, // 245
    WEBM_480_VIDEO_5, // 246
    WEBM_720_VIDEO, // 247 
    WEBM_720_VIDEO_2, // 302
    WEBM_1080_VIDEO, // 248
    WEBM_1080_VIDEO_2,// 303
    WEBM_1440_VIDEO, // 271
    WEBM_1440_VIDEO_2, // 308
    WEBM_2160_VIDEO, // 313
    WEBM_2160_VIDEO_2, // 315
    WEBM_4320_VIDEO, // 272

    // webm audio
    WEBM_AUDIO_171, // 171
    WEBM_AUDIO_249, // 249
    WEBM_AUDIO_250, // 250
    WEBM_AUDIO_251, // 251
    
    // webm video hdr
    WEBM_VIDEO_240_HDR, // 331
    WEBM_VIDEO_360_HDR, // 332
    WEBM_VIDEO_480_HDR, // 333
    WEBM_VIDEO_720_HDR, // 334
    WEBM_VIDEO_1080_HDR, // 335
    WEBM_VIDEO_1440_HDR, // 336
    WEBM_VIDEO_2160_HDR, // 337

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