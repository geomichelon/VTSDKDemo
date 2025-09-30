# Guia: Instalação e Uso em UI Tests (iOS e Android)

Este guia explica como gerar as bibliotecas do SDK (Rust FFI) e integrá‑las em projetos iOS (Xcode/XCUITest) e Android (Instrumented/Espresso), com exemplos práticos de chamada.

## Visão geral

- Artefatos gerados pelo crate `ffi`:
  - iOS: `staticlib` (recomendado) → `libvt_sdk_ffi.a`, empacotado como `dist/VTSDK.xcframework`.
  - Android: `cdylib` → `.so` por ABI (`arm64-v8a`, `armeabi-v7a`, `x86_64`).
- Header C público: `ffi/include/vt_sdk.h`.
- Funções expostas (retornam JSON `const char*` que deve ser liberado com `vt_free_string`):
  - `vt_compare_images(...)`
  - `vt_flex_search(...)`
  - `vt_flex_locate(...)`

## Gerando os binários

Pré‑requisitos: `rustup`, toolchains iOS/Android, Xcode (para iOS) e Android NDK (para Android).

### iOS (device + simulador)

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
cargo build -p vt-sdk-ffi --release --target aarch64-apple-ios
cargo build -p vt-sdk-ffi --release --target aarch64-apple-ios-sim
# Opcional: empacotar em XCFramework
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/libvt_sdk_ffi.a -headers ffi/include \
  -library target/aarch64-apple-ios-sim/release/libvt_sdk_ffi.a -headers ffi/include \
  -output dist/VTSDK.xcframework
```

Arquivos resultantes:

- `dist/VTSDK.xcframework`
- `ffi/include/vt_sdk.h`

### Android (várias ABIs)

Defina o caminho do NDK (ex.: `~/Library/Android/sdk/ndk/25.2.9519653`) em `ANDROID_NDK_HOME`.

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
cargo build -p vt-sdk-ffi --release --target aarch64-linux-android   # arm64-v8a
cargo build -p vt-sdk-ffi --release --target armv7-linux-androideabi # armeabi-v7a
cargo build -p vt-sdk-ffi --release --target x86_64-linux-android    # x86_64
```

Arquivos resultantes:

- `target/<triple>/release/libvt_sdk_ffi.so`

Observação (modo mock): adicione `--no-default-features --features mock` para gerar bibliotecas de mock.

---

## Integração no iOS (Xcode + XCUITest)

Há duas formas comuns:

1) Interagir via UI (recomendado para este sample): a UI Test abre o app, toca botões e valida o JSON exibido.
2) Invocar o FFI diretamente no target de UI Tests (via bridging header) — útil para validações diretas.

Este guia mostra a opção 2 (direta em UI Tests). Adapte conforme sua arquitetura.

### Passo a passo (via UI)

1. Use o sample pronto: projeto `ios-demo/VTSDKDemo.xcodeproj` e scheme `VTSDKDemo-UI`.
2. Adicione um target de UI Tests (se ainda não existir): Xcode → File → New → Target… → iOS → UI Testing Bundle → nome `VTSDKDemoUITests`.
3. Adicione o arquivo do teste de exemplo em `ios-demo/VTSDKDemoUITests/VTSDKDemoUITests.swift` ao target criado.
4. Selecione um simulador e rode a scheme `VTSDKDemo-UI` com o destino de UI Tests.

O teste abre o app, toca “Preparar imagens de exemplo” e “vt_compare_images”, e espera até que o JSON exibido contenha `obtainedSimilarity`.

Notas:

- O SDK retorna JSON (status, obtainedSimilarity, etc.). Faça o parse e valide conforme seu critério.
- Para distribuição, prefira usar o `XCFramework` no app; testes podem apenas exercitar a API.

---

## Integração no Android (Instrumented/Espresso)

Como a API é C, você chama via JNI. A forma mais simples é criar um pequeno “shim” JNI em C/C++ que delega para `vt_compare_images` e converte strings.

### Passo a passo

1. Copie os `.so` gerados para o app:
   - `app/src/main/jniLibs/arm64-v8a/libvt_sdk_ffi.so`
   - `app/src/main/jniLibs/armeabi-v7a/libvt_sdk_ffi.so`
   - `app/src/main/jniLibs/x86_64/libvt_sdk_ffi.so`

2. Adicione o header ao projeto (opcional, para referência): copie `ffi/include/vt_sdk.h` para `app/src/main/cpp/include/`.

3. Crie um shim JNI (ex.: `app/src/main/cpp/vtsdk_jni.cpp`):

```cpp
#include <jni.h>
#include <string>
#include "vt_sdk.h" // ajustar includePath no CMake

extern "C" JNIEXPORT jstring JNICALL
Java_com_example_vtsdk_VtSdkFFI_vtCompareImages(
        JNIEnv* env, jclass,
        jstring jbaseline, jstring jinput, jint jminSim, jint jnoise,
        jstring jexcluded, jstring jmeta) {
    const char* b = env->GetStringUTFChars(jbaseline, nullptr);
    const char* i = env->GetStringUTFChars(jinput, nullptr);
    const char* ex = env->GetStringUTFChars(jexcluded, nullptr);
    const char* me = env->GetStringUTFChars(jmeta, nullptr);

    const char* out = vt_compare_images(b, i, (int32_t)jminSim, (int32_t)jnoise, ex, me);

    env->ReleaseStringUTFChars(jbaseline, b);
    env->ReleaseStringUTFChars(jinput, i);
    env->ReleaseStringUTFChars(jexcluded, ex);
    env->ReleaseStringUTFChars(jmeta, me);

    jstring result = env->NewStringUTF(out ? out : "{}");
    vt_free_string(out);
    return result;
}
```

4. Configure o `CMakeLists.txt` para compilar o shim e linkar com `vt_sdk_ffi`:

```cmake
cmake_minimum_required(VERSION 3.18)
project(vtsdk_shim LANGUAGES C CXX)

add_library(vtsdk_shim SHARED src/main/cpp/vtsdk_jni.cpp)
target_include_directories(vtsdk_shim PRIVATE src/main/cpp/include)

# Link dinâmico com a .so do Rust (já empacotada em jniLibs)
add_library(vt_sdk_ffi SHARED IMPORTED)
set_target_properties(vt_sdk_ffi PROPERTIES IMPORTED_NO_SONAME TRUE)

target_link_libraries(vtsdk_shim PRIVATE vt_sdk_ffi log)
```

5. Ative o CMake no `build.gradle` do módulo `app`:

```groovy
android {
  defaultConfig { externalNativeBuild { cmake { cppFlags "-std=c++17" } } }
  externalNativeBuild { cmake { path file("CMakeLists.txt") } }
  // Certifique-se de embutir as .so de jniLibs por ABI
}
```

6. Exponha uma API Kotlin para os testes:

```kotlin
package com.example.vtsdk

object VtSdkFFI {
    init { System.loadLibrary("vtsdk_shim") }
    external fun vtCompareImages(
        baseline: String,
        input: String,
        minSim: Int,
        noise: Int,
        excludedJson: String,
        metaJson: String
    ): String
}
```

7. Use nos Instrumented Tests (Espresso):

```kotlin
@RunWith(AndroidJUnit4::class)
class UiTests {
    @Test fun compareExample() {
        val json = VtSdkFFI.vtCompareImages(
            "baseline.png", "input.png", 50, 20, "[]", "{\"testName\":\"UI-Compare\"}")
        assert(json.isNotEmpty())
    }
}
```

Notas:

- O shim JNI compila para cada ABI e delega a chamada para a `.so` do Rust, já empacotada em `jniLibs`.
- Se preferir, você pode compilar o shim como parte do próprio app (sem módulo separado).

---

## Modo real vs. mock

- `real` (padrão): usa a implementação de `core`.
- `mock`: usa respostas simuladas e é útil para integrar pipeline e UI antes da lógica real.
- Para iOS/Android, gere as libs com o conjunto de features desejado.

Exemplos:

```bash
# iOS mock
cargo build -p vt-sdk-ffi --release --no-default-features --features mock --target aarch64-apple-ios

# Android mock (arm64)
cargo build -p vt-sdk-ffi --release --no-default-features --features mock --target aarch64-linux-android
```

---

## Boas práticas e troubleshooting

- Sempre libere strings retornadas pelo SDK com `vt_free_string` após copiá‑las (
  iOS: depois de `String(cString:)`; Android: após criar o `jstring`).
- Confirme caminhos de imagens/URLs usados nos testes (acessíveis no sandbox do app/teste).
- Xcode Preview requer esquema em `Debug` (otimização `-Onone`).
- Em Android, garanta que os ABIs do app/test combinam com os `.so` incluídos.
- Problemas de link no iOS: verifique se o `XCFramework` está em “Link Binary With Libraries” do target correto e que o Header Search Path inclui `ffi/include`.
- Problemas de `UnsatisfiedLinkError` no Android: confirme `System.loadLibrary("vtsdk_shim")` e que as `.so` estão em `jniLibs/<ABI>/`.

---

## Onde olhar no repositório

- Header C: `ffi/include/vt_sdk.h`
- XCFramework (exemplo gerado): `dist/VTSDK.xcframework`
- Demo iOS pronta: `ios-demo/VTSDKDemo.xcodeproj`
- UI Tests (exemplo): `ios-demo/VTSDKDemoUITests/VTSDKDemoUITests.swift`
