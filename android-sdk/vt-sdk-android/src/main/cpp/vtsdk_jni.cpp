#include <jni.h>
#include <string>
#include <dlfcn.h>

typedef const char* (*vt_compare_images_t)(const char*, const char*, int32_t, int32_t, const char*, const char*);
typedef void (*vt_free_string_t)(const char*);

static vt_compare_images_t p_vt_compare_images = nullptr;
static vt_free_string_t p_vt_free_string = nullptr;

static void ensure_symbols_loaded() {
    if (p_vt_compare_images && p_vt_free_string) return;
    void* handle = dlopen("libvt_sdk_ffi.so", RTLD_LAZY);
    if (!handle) return;
    p_vt_compare_images = (vt_compare_images_t)dlsym(handle, "vt_compare_images");
    p_vt_free_string = (vt_free_string_t)dlsym(handle, "vt_free_string");
}

extern "C" JNIEXPORT jstring JNICALL
Java_com_geomichelon_vtsdk_VtSdkFFI_vtCompareImages(
        JNIEnv* env, jclass,
        jstring jbaseline, jstring jinput, jint jminSim, jint jnoise,
        jstring jexcluded, jstring jmeta) {
    ensure_symbols_loaded();
    if (!p_vt_compare_images) {
        const char* err = "{}";
        return env->NewStringUTF(err);
    }

    const char* b = env->GetStringUTFChars(jbaseline, nullptr);
    const char* i = env->GetStringUTFChars(jinput, nullptr);
    const char* ex = env->GetStringUTFChars(jexcluded, nullptr);
    const char* me = env->GetStringUTFChars(jmeta, nullptr);

    const char* out = p_vt_compare_images(b, i, (int32_t)jminSim, (int32_t)jnoise, ex, me);

    env->ReleaseStringUTFChars(jbaseline, b);
    env->ReleaseStringUTFChars(jinput, i);
    env->ReleaseStringUTFChars(jexcluded, ex);
    env->ReleaseStringUTFChars(jmeta, me);

    jstring result = env->NewStringUTF(out ? out : "{}");
    if (p_vt_free_string && out) p_vt_free_string(out);
    return result;
}

