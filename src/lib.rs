#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// 显示设备信息
pub unsafe fn PrintDeviceInfo(pstMVDevInfo: *mut MV_CC_DEVICE_INFO) -> bool {
    if pstMVDevInfo.is_null() {
        println!("[INFO]Device info is null!");
        return false;
    }

    if (*pstMVDevInfo).nTLayerType == MV_GIGE_DEVICE {
        let nIp1 = ((*pstMVDevInfo).SpecialInfo.stGigEInfo.nCurrentIp & 0xFF000000) >> 24;
        let nIp2 = ((*pstMVDevInfo).SpecialInfo.stGigEInfo.nCurrentIp & 0x00FF0000) >> 16;
        let nIp3 = ((*pstMVDevInfo).SpecialInfo.stGigEInfo.nCurrentIp & 0x0000FF00) >> 8;
        let nIp4 = (*pstMVDevInfo).SpecialInfo.stGigEInfo.nCurrentIp & 0x000000FF;

        println!("[INFO]CurrentIp:{}.{}.{}.{}", nIp1, nIp2, nIp3, nIp4);
        println!("[INFO]UserDefinedName:{:?}", (*pstMVDevInfo).SpecialInfo.stGigEInfo.chUserDefinedName);

    } else if (*pstMVDevInfo).nTLayerType == MV_USB_DEVICE {
        println!("[INFO]UserDefinedName:{:?}", (*pstMVDevInfo).SpecialInfo.stUsb3VInfo.chUserDefinedName);
        println!("[INFO]Serial Number:{:?}", (*pstMVDevInfo).SpecialInfo.stUsb3VInfo.chSerialNumber);
        println!("[INFO]Device Number:{}", (*pstMVDevInfo).SpecialInfo.stUsb3VInfo.nDeviceNumber);

    } else if (*pstMVDevInfo).nTLayerType == MV_GENTL_GIGE_DEVICE {
        println!("[INFO]UserDefinedName:{:?}", (*pstMVDevInfo).SpecialInfo.stGigEInfo.chUserDefinedName);
        println!("[INFO]Serial Number:{:?}", (*pstMVDevInfo).SpecialInfo.stGigEInfo.chSerialNumber);
        println!("[INFO]Device Number:{:?}", (*pstMVDevInfo).SpecialInfo.stGigEInfo.chModelName);

    } else if (*pstMVDevInfo).nTLayerType == MV_GENTL_CAMERALINK_DEVICE {
        println!("[INFO]UserDefinedName:{:?}", (*pstMVDevInfo).SpecialInfo.stCMLInfo.chUserDefinedName);
        println!("[INFO]Serial Number:{:?}", (*pstMVDevInfo).SpecialInfo.stCMLInfo.chSerialNumber);
        println!("[INFO]Device Number:{:?}", (*pstMVDevInfo).SpecialInfo.stCMLInfo.chModelName);

    } else if (*pstMVDevInfo).nTLayerType == MV_GENTL_CXP_DEVICE {
        println!("[INFO]UserDefinedName:{:?}", (*pstMVDevInfo).SpecialInfo.stCXPInfo.chUserDefinedName);
        println!("[INFO]Serial Number:{:?}", (*pstMVDevInfo).SpecialInfo.stCXPInfo.chSerialNumber);
        println!("[INFO]Device Number:{:?}", (*pstMVDevInfo).SpecialInfo.stCXPInfo.chModelName);

    } else if (*pstMVDevInfo).nTLayerType == MV_GENTL_XOF_DEVICE {
        println!("[INFO]UserDefinedName:{:?}", (*pstMVDevInfo).SpecialInfo.stXoFInfo.chUserDefinedName);
        println!("[INFO]Serial Number:{:?}", (*pstMVDevInfo).SpecialInfo.stXoFInfo.chSerialNumber);
        println!("[INFO]Device Number:{:?}", (*pstMVDevInfo).SpecialInfo.stXoFInfo.chModelName);

    } else {
        println!("[INFO]Device type is unknown! Not Support!");

    }

    return true;
}