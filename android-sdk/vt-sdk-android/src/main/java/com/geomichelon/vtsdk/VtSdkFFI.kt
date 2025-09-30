package com.geomichelon.vtsdk

object VtSdkFFI {
    init {
        try { System.loadLibrary("vt_sdk_ffi") } catch (_: Throwable) {}
        System.loadLibrary("vtsdk_shim")
    }

    external fun vtCompareImages(
        baseline: String,
        input: String,
        minSim: Int,
        noise: Int,
        excludedJson: String,
        metaJson: String
    ): String
}

