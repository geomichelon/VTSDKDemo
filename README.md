# VT-SDK

SDK multiplataforma em Rust para validação visual (Visual Testing).
Suporta modo real e modo mockado, com bindings para iOS (Swift) e Android (Kotlin/Java) via FFI.

---

## Guia de Integração (UI Tests)

Para instruções passo a passo de como instalar e usar no seu projeto (Xcode/XCUITest e Android/Espresso/Instrumentation), veja:

- `docs/GUIA_UI_TESTES.md`

---

## Estrutura

- Workspace: `Cargo.toml` (raiz) declara `core`, `mock` e `ffi`.
- Core: `core/` contém a implementação real (ex.: `compare`).
- Mock: `mock/` contém a implementação mock para testes/integração.
- FFI: `ffi/` expõe a API C (`vt_compare`) como biblioteca compartilhada e estática.

## Features/Modos

- `real` (padrão): usa `vt-sdk-core`.
- `mock`: usa `vt-sdk-mock`.
- São exclusivos entre si; pelo menos um deve estar ativo (o default ativa `real`).

## Build Desktop

- Real: `cargo build -p vt-sdk-ffi --release`
- Mock: `cargo build -p vt-sdk-ffi --release --no-default-features --features mock`

Saídas típicas (macOS/Linux):
- `target/release/libvt_sdk_ffi.dylib` ou `.so` (dinâmica)
- `target/release/libvt_sdk_ffi.a` (estática)

## Android

1) Configure o NDK/toolchains e targets desejados (ex.: `aarch64-linux-android`, `armv7-linux-androideabi`, `x86_64-linux-android`).
2) Compile a FFI (real):
   - `cargo build -p vt-sdk-ffi --release --target aarch64-linux-android`
3) Para mock: adicione `--no-default-features --features mock`.
4) Linke `libvt_sdk_ffi.so` no app e carregue com `System.loadLibrary("vt_sdk_ffi")`.

## iOS

O crate `ffi` agora gera `staticlib` e `cdylib` (ver `ffi/Cargo.toml`). Para iOS, use a estática (`.a`). Targets comuns:

- Dispositivo: `aarch64-apple-ios`
- Simulador: `aarch64-apple-ios-sim` (ou `x86_64-apple-ios` para simuladores Intel)

Comandos exemplo (ajuste conforme seu toolchain):

- `rustup target add aarch64-apple-ios aarch64-apple-ios-sim`
- Real: `cargo build -p vt-sdk-ffi --release --target aarch64-apple-ios`
- Mock: `cargo build -p vt-sdk-ffi --release --no-default-features --features mock --target aarch64-apple-ios`

Para distribuir universal/xcframework, combine artefatos de device + simulator.

## API FFI

Assinatura exposta (C ABI):

- `float vt_compare(const char* baseline, const char* input, int32_t min_similarity);`

Regras de uso:
- `baseline` e `input` devem ser strings C válidas e terminadas em `\0`.
- Retorna `0.0` em caso de ponteiro nulo ou string inválida.

## Binding iOS (Swift)

- Linke a lib estática `libvt_sdk_ffi.a` no projeto e exponha o símbolo:

```swift
@_silgen_name("vt_compare")
func vt_compare(_ baseline: UnsafePointer<CChar>!, _ input: UnsafePointer<CChar>!, _ minSimilarity: Int32) -> Float

func compare(baseline: String, input: String, min: Int32) -> Float {
    let b = (baseline as NSString).utf8String!
    let i = (input as NSString).utf8String!
    return vt_compare(b, i, min)
}
```

## Binding Android (Kotlin/Java)

- Carregue a lib e use um wrapper JNI que chama `vt_compare`.
- Exemplo de assinatura em Kotlin:

```kotlin
object VtSdkFFI {
    init { System.loadLibrary("vt_sdk_ffi") }
    external fun vt_compare(baseline: String, input: String, minSimilarity: Int): Float
}
```

Implemente o `JNIEXPORT jfloat JNICALL Java_pkg_VtSdkFFI_vt_1compare(...)` em C/C++, convertendo `jstring` para `const char*` e delegando para `vt_compare`.

---

## Notas

- iOS: apps em produção preferem estática; evitar `.dylib` custom em iOS.
- Sem rede, extensões de IDE não sugerem nomes/versões de crates; apenas chaves/estrutura TOML.
- A função real em `core` é stub e retorna valor fixo; ajuste conforme a lógica de Visual Testing.
