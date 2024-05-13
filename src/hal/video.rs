// SPDX-License-Identifier: MIT

use nxdk_sys::hal::video::*;

#[derive(Debug, Default)]
pub enum RefreshRate {
    #[default]
    Default = REFRESH_DEFAULT as isize,
    Hz50 = REFRESH_50HZ as isize,
    Hz60 = REFRESH_60HZ as isize,
}

pub fn xvideo_set_mode(width: i32, height: i32, bpp: i32, refresh_rate: RefreshRate) -> bool {
    let ret;

    unsafe {
        ret = nxdk_sys::hal::video::XVideoSetMode(width, height, bpp, refresh_rate as i32);
    }

    ret != 0
}
